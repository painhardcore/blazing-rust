---
type: llm
focus: trace
weight: 1
---

The new status_counts no longer allocates a String per line or per field (it borrows &str slices)
