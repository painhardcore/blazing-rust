# Blazing Rust

A Rust performance skill for Claude Code, Codex, and other agents that support [Agent Skills](https://agentskills.io/specification).

Use it to investigate slow Rust code and test optimizations against a baseline. It covers allocations, data structures, memory layout, async code, SIMD, parallelism, and compiler settings. The workflow requires measurements before keeping a change.

The [CarryFS example](#example-carryfs-catalog-keys) includes before-and-after code and recorded benchmark results.

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

The skill uses your project's Rust toolchain and measurement tools. It does not require Criterion or a profiler. When a tool is unavailable, the agent must state what it could not measure.

The [technique guide](references/techniques.md) has detailed optimization advice. The agent reads the sections relevant to the workload.

## Example: CarryFS catalog keys

A first run on [CarryFS](https://github.com/carry-fs/carryfs/pull/33) replaced heap-allocated catalog keys with fixed-size arrays. An inode key contains an eight-byte object ID and a one-byte tag, so it needs exactly nine bytes.

Before:

```rust
fn inode_key(object: u64) -> Vec<u8> {
    let mut key = object.to_be_bytes().to_vec();
    key.push(1);
    key
}
```

After:

```rust
fn inode_key(object: u64) -> [u8; 9] {
    let mut key = [0; 9];
    key[..8].copy_from_slice(&object.to_be_bytes());
    key[8] = 1;
    key
}
```

The [same commit](https://github.com/carry-fs/carryfs/commit/ebb6e9a815ab2fbdbf0aa63a463e97083e300770) changed extent and group keys to arrays. Variable-length directory keys still use `Vec`, with capacity reserved once. Some callers convert arrays to `Vec` to satisfy existing APIs.

The commit reports these lookup times for 10,000 files in read-write mode:

| Version | Median | Reported range |
| --- | --- | --- |
| Before the commit | 2.69 ms | 2.60 to 2.99 ms |
| After the commit | 1.94 ms | 1.88 to 2.11 ms |

The commit lists release builds and "CPU 2". It reports a median of five alternating processes with five rounds each. The [core benchmark](https://github.com/carry-fs/carryfs/blob/ebb6e9a815ab2fbdbf0aa63a463e97083e300770/benches/core.rs) uses a RAM-backed device, so the figures measure library lookups rather than mounted-filesystem or disk throughput.

From a CarryFS checkout, run the checked-in workload with:

```sh
CARRYFS_BENCH_ROUNDS=5 cargo bench --bench core
```

Compare the linked commit with its parent. The command runs the workload. You must alternate the versions yourself. The commit omits the CPU model, operating system, Rust version, and raw samples. The figures above come from the commit notes and have not been independently reproduced here.

Another [commit](https://github.com/carry-fs/carryfs/commit/f968140ec85bf54bbb2c21ce80a7db37b367de83) merged allocation runs in place. It reported 14,338 fewer allocator calls per core round. The latency results were inconclusive, so this change has no demonstrated speedup.

The PR also reports two larger improvements for mixed-Unicode directory reads and file metadata lookups:

| Change | Before | After | Latency reduction |
| --- | --- | --- | --- |
| [Skip unnecessary Unicode lookups](https://github.com/carry-fs/carryfs/commit/3122eb405437fed0ecfca28bf75799652390185b) | 60.79 ms | 33.74 ms | 44.5% |
| [Fast-path ASCII processing](https://github.com/carry-fs/carryfs/commit/a6f69ef703ee1ecea25c61c449a92e6b40b24c25) | 34.31 ms | 21.59 ms | 37.1% |

Each row compares one commit with its parent. The figures come from the commit notes. The repository does not include the mixed-Unicode benchmark workload.

## License

[MIT](LICENSE). Copyright 2026 Andrei Iurchenkov.
