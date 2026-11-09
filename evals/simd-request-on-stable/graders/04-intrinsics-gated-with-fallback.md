---
type: llm
focus: trace
weight: 1
---

Any explicit SIMD intrinsics are gated by target_arch with a scalar fallback for other targets (pass if no intrinsics were added)
