# Blazing Rust

A Rust performance skill for Claude Code, Codex, and other agents that support [Agent Skills](https://agentskills.io/specification).

It guides the agent through profiling, benchmarking, and small changes that preserve behavior. It covers allocation costs, data structures, memory layout, async stalls, SIMD, parallelism, and build tuning. Each technique needs evidence from the workload.

See [a first run on CarryFS](#example-carryfs-catalog-keys) for a concrete change and its reported benchmark.

## Install

With Node.js 22.20 or later, run this from the project where you want the skill:

```sh
    npx skills add painhardcore/blazing-rust --skill blazing-rust
```

Choose your agent when prompted. To select one directly:

```sh
# Claude Code
npx skills add painhardcore/blazing-rust --skill blazing-rust -a claude-code

# Codex
npx skills add painhardcore/blazing-rust --skill blazing-rust -a codex
```

Add `-g` to install for all your projects. The [skills CLI](https://github.com/vercel-labs/skills#readme) also supports Cursor, OpenCode, and other agents.

To try an unpublished checkout, replace `painhardcore/blazing-rust` with its absolute local path.

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

For Codex, set `skill_dir="$HOME/.agents/skills/blazing-rust"` instead. For another agent, copy `SKILL.md`, `LICENSE`, and the `references` directory into a folder named `blazing-rust` in its skills directory.

The paths follow the official [Claude Code](https://code.claude.com/docs/en/skills#choose-where-skills-load) and [Codex](https://learn.chatgpt.com/docs/build-skills#where-codex-loads-local-skills) documentation.

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

The agent can also select the skill for relevant Rust performance requests. Include representative inputs, your target metric, and deployment constraints when you have them.

The skill uses your project's Rust toolchain and existing measurement tools. Criterion, profilers, and other tools are optional. It requires the agent to report missing measurements rather than invent results.

The main skill contains the workflow and technique selection table. The agent reads relevant sections of [the technique guide](references/techniques.md) when it needs implementation details.

## Example: CarryFS catalog keys

In [CarryFS PR #33](https://github.com/carry-fs/carryfs/pull/33), one change replaced heap-allocated catalog keys with fixed-size arrays. An inode key always contains an eight-byte object ID followed by a one-byte tag.

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

The [commit](https://github.com/carry-fs/carryfs/commit/ebb6e9a815ab2fbdbf0aa63a463e97083e300770) also changed the fixed-size extent and group keys to arrays. Variable-length directory keys kept their `Vec` representation and reserved capacity once. Callers converted arrays to `Vec` where existing APIs required that type.

The commit reports these results for read-write lookups across 10,000 files:

| Version | Median | Reported range |
| --- | --- | --- |
| Before the commit | 2.69 ms | 2.60 to 2.99 ms |
| After the commit | 1.94 ms | 1.88 to 2.11 ms |

The commit records release builds, "CPU 2", and a median of five alternating processes with five rounds each. The [core benchmark](https://github.com/carry-fs/carryfs/blob/ebb6e9a815ab2fbdbf0aa63a463e97083e300770/benches/core.rs) uses a RAM-backed device. These figures describe library lookups, not mounted-filesystem or disk throughput.

From a CarryFS checkout, run the checked-in workload with:

```sh
CARRYFS_BENCH_ROUNDS=5 cargo bench --bench core
```

Compare the linked commit with its parent. The command runs the workload but does not automate the alternating comparison. The commit does not record the CPU model, operating system, Rust version, or raw samples. These are commit-reported results, not an independently reproduced benchmark.

Another [change in the same PR](https://github.com/carry-fs/carryfs/commit/f968140ec85bf54bbb2c21ce80a7db37b367de83) coalesced allocation runs in place. It reported 14,338 fewer allocator calls per core round, with inconclusive latency results. Fewer allocations alone do not establish a speedup.

## License

[MIT](LICENSE). Copyright 2026 Andrei Iurchenkov.
