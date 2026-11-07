# Blazing Rust

A Rust performance skill for Claude Code, Codex, and other agents that support [Agent Skills](https://agentskills.io/specification).

It guides the agent through profiling, benchmarking, and small changes that preserve behavior. It covers allocation costs, data structures, memory layout, async stalls, SIMD, parallelism, and build tuning. Each technique needs evidence from the workload.

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

## License

[MIT](LICENSE). Copyright 2026 Andrei Iurchenkov.
