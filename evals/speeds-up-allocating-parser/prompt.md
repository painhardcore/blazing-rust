---
tags: [skill-creator]
max_turns: 40
timeout_seconds: 1200
allowed_tools: [Read, Glob, Grep, Skill, Write, Edit, Bash]
---

status_counts in src/lib.rs is the slow part of our log-stats tool. on a full day of logs (~2M lines) it takes way too long. make it faster. examples/gen_log.rs generates test data.

Input files: log-stats/
