---
name: blazing-rust
description: Profile and optimize Rust runtime performance with reproducible benchmarks. Use whenever someone wants Rust code faster or leaner (slow functions, latency or throughput regressions, allocation churn, CPU hot spots, async stalls), even if nothing has been measured yet. The skill measures first and rejects changes without evidence.
license: MIT
---

# Blazing Rust

Improve measured performance while preserving behavior. Keep the simplest change that produces a repeatable gain on representative inputs.

## Measure before editing

1. Identify the workload, target metric, and correctness constraints. Read the hot path and its callers. Respect the project's toolchain, minimum supported Rust version, features, and deployment targets. An explanation request authorizes measurement and profiling, not edits.
2. Capture a baseline with the project's optimized production profile, usually `release`. Record the input, exact command, toolchain, flags, CPU, and thread count. Use realistic input sizes and distributions, including small and adverse cases.
3. Find what limits performance. Profile CPU time, allocations, I/O, or lock waits as appropriate. For async stalls, inspect task scheduling and queue growth with runtime-aware tracing. Use existing tools first; [the profiling section](references/techniques.md#profiling) lists common ones and how to keep symbols in release builds. If profiling is unavailable, use targeted measurements and state the limits of the evidence.
4. State a hypothesis and make one measurable change. Run focused correctness tests and repeat the same benchmark. Verify the result in the full workload before claiming a user-visible improvement.
5. Keep supported gains. Undo only your unsuccessful edits, preserving unrelated work. Stop when the target is met or further changes lack evidence.

## Make the benchmark credible

- Separate setup, compilation, input generation, and process startup from the operation unless they belong in the target metric. Consume results. For microbenchmarks, use the benchmark library's optimizer barriers or `std::hint::black_box` on inputs and outputs. These barriers are best effort, so inspect implausible results.
- Compare equivalent work and outputs. Keep toolchain, profile, features, allocator, CPU settings, and thread count fixed unless one is the variable under test. Warm up when measuring steady state. Measure cold starts separately when relevant.
- Repeat runs and report sample count, median, and variability. Alternate baseline and candidate runs when drift matters. A difference within the noise does not establish a win.
- Match metrics to the task. For services, measure p95/p99 latency at a stated load, throughput, errors, and resource use. Check that reduced latency did not come from dropped work or a growing queue.
- Reuse existing benchmarks. Add a small regression case when new logic needs one. Do not install tools, add dependencies, or change build settings without the task's authorization. Never invent measurements.

## Choose changes from evidence

Start with unnecessary work and algorithms. Consider lower-level changes only when the remaining bottleneck calls for them. Read the matching section of [the technique guide](references/techniques.md) when a measured bottleneck needs more detail.

| Observed cost | Candidate change and checks |
| --- | --- |
| Repeated scans or computation | Improve the algorithm or collection choice. Remove duplicate parsing, sorting, serialization, and lookups. Account for cache invalidation and memory if caching helps. |
| Allocation or copying | Borrow `&str` and slices, move owned values, reuse buffers, and reserve credible capacities. Try `clone_from` when replacing an allocated value. Avoid intermediate collections. `clone` may be cheap for shared handles, so measure its cost. |
| Collection lookups | Use borrowed keys and entry APIs where they avoid repeated work. Compare dense vectors or sorted data when the access pattern fits. Preserve collision-attack resistance for untrusted keys when changing hashers. |
| Sorting or removal | Use `select_nth_unstable` for selection, `sort_unstable` when equal-element order is irrelevant, and `sort_by_cached_key` for expensive keys. Use `swap_remove` only when element order is irrelevant. |
| Cache misses or large values | Compare arrays of structures with structures of arrays. Check `size_of`, `align_of`, and cache misses. Inline storage can increase copying. Consider boxing rare large enum payloads, accounting for allocations and indirection. |
| Small hot calls or loops | Prefer ordinary iteration. Help the compiler remove bounds checks with slices, iteration, or range assertions. Inspect optimized assembly before inlining or unrolling. Test empty inputs, tails, overflow, and code size. |
| Text search | Reuse compiled regexes and request only needed match information. Compare specialized byte searches when encoding semantics permit. Avoid rebuilding search state inside hot loops. |
| Vectorizable computation | Check auto-vectorization before explicit SIMD. Inspect generated code. Guard target-specific instructions where needed and retain a scalar fallback. Test tails and representative CPUs. Preserve alignment, bounds, and numerical behavior. |
| Independent CPU work | Compare serial execution with bounded parallelism. Measure scheduling cost, contention, bandwidth, thread count, and total CPU consumption. Tune task size only when overhead matters. Preserve reduction semantics. |
| I/O or async stalls | Reuse line buffers, batch small I/O, and handle flush errors. Preserve latency, newline handling, and durability. Keep blocking or heavy CPU work off async workers. Bound concurrency and queues. Preserve ordering, backpressure, and cancellation. |
| Build or code-generation limits | Compare existing settings with thin LTO. Test fat LTO and `codegen-units = 1` separately before combining. Include build time and binary size. Use CPU-specific flags only when deployment hardware permits them. |

## Preserve correctness

Keep public APIs, error handling, validation, ordering, serialization, and FFI contracts intact. Preserve integer overflow and floating-point behavior. Enum representation or variant order can change layout and discriminants; neither guarantees a smaller or faster enum.

Prefer safe Rust and existing dependencies. Introducing unsafe code requires a measured need, explicit safety invariants, and focused tests. Use Miri or sanitizers where supported and relevant. Never trade validation or soundness for benchmark speed, and do not treat passing tests as proof of soundness.

## Report the result

State the bottleneck, change, and keep-or-revert decision. Include reproducible commands, before/after results with variability, relevant allocation or memory figures, correctness checks, and trade-offs. Distinguish multicore wall-clock gains from single-core efficiency. Name unmeasured claims and unchecked platforms.
