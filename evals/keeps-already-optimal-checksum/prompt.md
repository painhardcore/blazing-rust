---
tags: [skill-creator]
max_turns: 40
timeout_seconds: 1200
allowed_tools: [Read, Glob, Grep, Skill, Write, Edit, Bash]
---

page_checksum in src/lib.rs runs on every page our storage engine reads or writes, so it's really hot. can you make it faster?

Input files: page-sum/
