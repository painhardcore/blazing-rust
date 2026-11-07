# Rust performance techniques

Read the section matching the measured cost. Check each API against the project's minimum supported Rust version. Compare candidates against the same baseline and preserve behavior.

## Allocations and layout

- `clone_from` can reuse destination allocations when replacing a `Vec` or `String`. Moving or borrowing can avoid the copy entirely. Buffer reuse retains capacity, so check retained memory after unusually large inputs. [Allocation reuse](https://nnethercote.github.io/perf-book/heap-allocations.html#clone)
- `with_capacity` reserves storage without initializing elements or changing length. Size reservations from real inputs. Excess capacity increases memory usage. [Vec guarantees](https://doc.rust-lang.org/std/vec/struct.Vec.html#guarantees)
- Inline strings and `SmallVec` can avoid allocations for short values. Measure the length distribution, element size, and copy costs. Inline storage lives inside its containing value, which may itself be on the heap. String limits count bytes. Preserve UTF-8 and define how oversized inputs spill or fail. Prefer existing implementations over a custom string type. [Short collections](https://nnethercote.github.io/perf-book/heap-allocations.html#short-vecs)
- A rare large enum variant can widen every element. Boxing its payload trades a smaller enum for allocation and indirection. Compare arrays of structures with structures of arrays when loops touch only selected fields. Verify layout claims with generated code and cache measurements. [Type sizes](https://nnethercote.github.io/perf-book/type-sizes.html)
- Direct ownership or `Box` can replace unnecessary shared ownership. When sharing stays within one thread, `Rc` avoids `Arc`'s atomic reference counting. Preserve required `Send` and `Sync` behavior. [Arc thread safety](https://doc.rust-lang.org/std/sync/struct.Arc.html#thread-safety)

## Collections and loops

- `select_nth_unstable` partitions in linear time without fully sorting. Check the index and empty-input cases. Sort the selected partition only if the output needs ordering. `sort_unstable` avoids allocation but changes the order of equal elements. `sort_by_cached_key` computes expensive keys at most once per element, using temporary storage. [Slice APIs](https://doc.rust-lang.org/std/primitive.slice.html#method.select_nth_unstable)
- `swap_remove` removes a vector element in constant time by moving the last element into its place. Use `retain` for bulk filtering when appropriate. Preserve ordering and any stored indices. [Vector operations](https://nnethercote.github.io/perf-book/standard-library-types.html#vec)
- Current `HashMap` iteration scans capacity rather than just length. After large deletions, shrinking or rebuilding may help repeated scans. Include maintenance and future growth costs. Keep collision-attack resistance for untrusted keys. [HashMap iteration](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.iter)
- Iteration, slicing before a loop, and range assertions can expose enough information to eliminate bounds checks safely. Inspect assembly before considering unchecked indexing. Preserve behavior on short inputs. [Bounds checks](https://nnethercote.github.io/perf-book/bounds-checks.html)
- For custom iterators, accurate `size_hint` or `ExactSizeIterator` implementations can reduce allocation during collection. Try `extend` instead of collecting and appending. Loop fusion can remove passes, but must preserve side effects, ordering, and numerical behavior. [Iterators](https://nnethercote.github.io/perf-book/iterators.html)

## I/O and text

- `BufRead::lines` allocates owned strings. A reusable `String` with `read_line` and `clear` can reduce allocations when each line is consumed before the next read. `read_line` retains the line terminator, unlike `lines`, so preserve the original newline handling. [Line buffers](https://nnethercote.github.io/perf-book/heap-allocations.html#reading-lines-from-a-file)
- Lock stdout once for repeated output. Buffer many small reads or writes where latency permits. Explicitly flush writers when errors must reach the caller. Flushing a buffer does not by itself guarantee durable storage. [Buffered I/O](https://nnethercote.github.io/perf-book/io.html), [File synchronization](https://doc.rust-lang.org/std/fs/struct.File.html#method.sync_all)
- For hot byte or substring searches, compare `memchr` and `memmem` with existing code. Reuse `memmem::Finder` for a repeated needle. Test short and long inputs. Preserve encoding semantics and validate UTF-8 boundaries before slicing strings. [Byte search](https://docs.rs/memchr/latest/memchr/)
- Compile reusable regexes once. Use `is_match` for existence, `find` for a match location, and captures only when required. Unicode-to-ASCII changes need matching input contracts. If profiling shows contention on a shared regex, compare a regex clone per worker. [Regex performance](https://docs.rs/regex/latest/regex/#performance)

## Parallel and async work

- Rayon chunk sizes and `with_min_len` can reduce task overhead. Larger jobs can reduce load balancing. Prefer task-local accumulation followed by a reduction over shared updates when semantics permit. Measure wall time, CPU use, and thread count. [Rayon task size](https://docs.rs/rayon/latest/rayon/iter/trait.IndexedParallelIterator.html#method.with_min_len), [Rayon reductions](https://docs.rs/rayon/latest/rayon/iter/trait.ParallelIterator.html#method.fold)
- Heavy CPU work can starve async workers. Bound CPU-heavy `spawn_blocking` work or use the project's CPU pool. Started blocking tasks cannot be aborted, so preserve cancellation and shutdown requirements. [Tokio blocking tasks](https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html)
- If independent writes interfere across cores, investigate false sharing. `CachePadded` separates values at a memory cost. Check the target hardware and cache-coherence evidence before padding data. [Cache padding](https://docs.rs/crossbeam-utils/latest/crossbeam_utils/struct.CachePadded.html)

## Compiler tuning

- Configure profiles in the workspace-root `Cargo.toml`. Compare `lto = "thin"` with existing settings. Test `lto = "fat"` and `codegen-units = 1` independently, then together if useful. Keep settings only when runtime or size gains justify build costs. Use `target-cpu=native` only when deployment hardware permits it. [Cargo profiles](https://doc.rust-lang.org/cargo/reference/profiles.html)
- Moving rare work into a separate `#[cold]` function can improve hot-path code generation. The attribute is a hint. Check branch frequency, assembly, and code size. [Outlining](https://nnethercote.github.io/perf-book/inlining.html#outlining)
- Profile-guided optimization uses execution data to guide compiler decisions. Build with instrumentation, exercise representative workloads, merge profiles, then rebuild using them. Keep compiler settings consistent and avoid stale profiles. Validate on workloads outside the training set. [Rust PGO workflow](https://doc.rust-lang.org/rustc/profile-guided-optimization.html)
