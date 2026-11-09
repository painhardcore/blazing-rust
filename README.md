# Blazing Rust

A Rust performance skill for Claude Code, Codex, and other agents that support [Agent Skills](https://agentskills.io/specification).

The agent measures a baseline, finds the bottleneck, and pushes it toward the hardware limit one measured change at a time. It stops at lines it will not cross on its own: changed behavior, unsound `unsafe`, weaker security, new threads in library code, and new dependencies or build settings. Faster options beyond those lines are reported with their measured gain. It covers allocations, data structures, memory layout, async code, SIMD, parallelism, and compiler settings.

## Results

Four tasks, each run with and without the skill on the same prompt, with no user instructions loaded. 27 checks cover:
- measuring before editing;
- correct results;
- not crossing the lines above;
- pushing past the first win;
- not editing code when only asked why it is slow.

| Model | Checks passed with the skill | Without the skill |
| --- | --- | --- |
| Claude Opus 5.5 | **100%** | 82% |
| Claude Sonnet 5.5, high effort | **96%** | 76% |
| Claude Haiku 5.5 | **97%** | 79% |

What changed with the skill, on Opus:

| Task | With the skill | Without the skill |
| --- | --- | --- |
| Speed up an allocation-heavy log parser | **46x faster** on one thread. It keeps the collision-resistant hasher, adds no threads, and reports that 4 threads would give another 3.7x. | 32x, using a weak hasher on untrusted input and threads inside a library function |
| Rewrite a byte counter with SIMD on stable Rust | **14x faster with no `unsafe`.** It wrote hand-written NEON code, measured it, and did not ship it because the safe version was as fast. | 15x with 10 `unsafe` blocks, never compared to a safe version |
| Explain why an event merge is slow | Confirms both O(n²) loops at small sizes, extrapolates the full case, and edits nothing | Retries the minute-long full-size run until the process is killed |
| Speed up a checksum that is already vectorized | Keeps the original code and shows it is within about 20% of the L1-cache limit and near memory bandwidth beyond it | Also keeps it, but does not say how much headroom is left |

Sonnet with the skill produced safe 12–19x speedups at about a fifth of Opus's cost. With the skill, runs take roughly twice the time and tokens of runs without it, because the agent measures more. These are single runs on an Apple M1, so treat individual numbers as rough. The test cases are in [`evals/`](evals).

## Install

With Node.js 22.20 or later, run this from the project where you want the skill:

```sh
npx skills add painhardcore/blazing-rust --skill blazing-rust
```

Choose your agent when prompted, or select it with `-a`:

```sh
# Claude Code
npx skills add painhardcore/blazing-rust --skill blazing-rust -a claude-code

# Codex
npx skills add painhardcore/blazing-rust --skill blazing-rust -a codex
```

Add `-g` to install for all your projects. The [skills CLI](https://github.com/vercel-labs/skills#readme) also supports Cursor, OpenCode, and other agents.

To install from a local checkout, replace `painhardcore/blazing-rust` with its absolute path.

## Install without Node.js

Clone this repository, or use your existing checkout:

```sh
git clone https://github.com/painhardcore/blazing-rust.git
cd blazing-rust
```

Choose the destination for your agent:

| Agent | All your projects | One project |
| --- | --- | --- |
| Claude Code | `~/.claude/skills/blazing-rust` | `<project>/.claude/skills/blazing-rust` |
| Codex | `~/.agents/skills/blazing-rust` | `<project>/.agents/skills/blazing-rust` |

For example, install for Claude Code:

```sh
skill_dir="$HOME/.claude/skills/blazing-rust"
mkdir -p "$skill_dir"
cp -i SKILL.md LICENSE "$skill_dir/"
mkdir -p "$skill_dir/references"
cp -i references/techniques.md "$skill_dir/references/"
```

For Codex, set `skill_dir="$HOME/.agents/skills/blazing-rust"` instead. For other agents, use their skills directory. Copy `SKILL.md`, `LICENSE`, and `references` into a folder named `blazing-rust`.

See the [Claude Code](https://code.claude.com/docs/en/skills#choose-where-skills-load) and [Codex](https://learn.chatgpt.com/docs/build-skills#where-codex-loads-local-skills) documentation for supported skill locations.

## Use

Start a new agent session after installation. Invoke the skill directly:

Claude Code:

```text
/blazing-rust Investigate why parsing large files is slow. Measure a baseline, optimize the bottleneck, and verify the result.
```

Codex:

```text
$blazing-rust Investigate why parsing large files is slow. Measure a baseline, optimize the bottleneck, and verify the result.
```

The agent can also load the skill when you ask about Rust performance. Give it representative inputs, a target metric, and any deployment constraints.

## Steer it from the prompt

By default the agent keeps pushing until it meets your target, reaches the hardware limit, or runs out of candidates that stay within its lines. Add a sentence to your prompt to move a line:

| Default | To change it, say |
| --- | --- |
| `unsafe` is kept only when it beats the best safe version by more than the noise, with a portable fallback and tests | "Safe Rust only." or "No `unsafe` unless it's at least 2x faster." |
| Library code gets no new threads; the multicore gain is reported | "This is a CLI that owns its threads; parallelism is fine." |
| No new dependencies, build settings, or installed tools | "You may add crates." or "You may enable LTO and `target-cpu=native`; we deploy on one CPU type." |
| Hashers stay collision-resistant for untrusted keys | "Keys are trusted internal IDs." |
| Pushes toward the hardware limit, even if the code grows | "Stop at 2x." or "Target: under 50 ms for 1M lines." or "Keep the code simple." |
| "Why is X slow?" gets measurements and a diagnosis, with no edits | "Explain, then fix it." |

The agent tells you the gain it left on the table, so you can rerun with a looser line if that gain is worth it.

The skill uses your project's Rust toolchain and measurement tools. It does not require Criterion or a profiler. When a tool is unavailable, the agent must state what it could not measure.

The [technique guide](references/techniques.md) has detailed optimization advice. The agent reads the sections relevant to the workload.

## Example: CarryFS

A run on [CarryFS](https://github.com/carry-fs/carryfs/pull/33) produced these changes. The figures come from the commit notes and have not been reproduced independently.

| Change | Before | After |
| --- | --- | --- |
| [Fixed-size arrays for catalog keys](https://github.com/carry-fs/carryfs/commit/ebb6e9a815ab2fbdbf0aa63a463e97083e300770) instead of `Vec<u8>`: lookups for 10,000 files | 2.69 ms | 1.94 ms (−28%) |
| [Skip unnecessary Unicode lookups](https://github.com/carry-fs/carryfs/commit/3122eb405437fed0ecfca28bf75799652390185b) in mixed-Unicode directory reads | 60.79 ms | 33.74 ms (−44.5%) |
| [Fast-path ASCII processing](https://github.com/carry-fs/carryfs/commit/a6f69ef703ee1ecea25c61c449a92e6b40b24c25) | 34.31 ms | 21.59 ms (−37.1%) |

The key change replaced `object.to_be_bytes().to_vec()` plus a pushed tag byte with a `[u8; 9]` array. Another [commit](https://github.com/carry-fs/carryfs/commit/f968140ec85bf54bbb2c21ce80a7db37b367de83) cut 14,338 allocator calls per round, but its latency results were within noise, so it claims no speedup.

## License

[MIT](LICENSE). Copyright 2026 Andrei Iurchenkov.
