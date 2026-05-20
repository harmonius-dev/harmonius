# Memory Management & Async I/O Test Cases

Companion test cases for [memory-async-io.md](memory-async-io.md).

## Unit Tests

### TC-1.7.1.1 Arena 100K Allocations Under 1ms

| # | Requirement |
|---|-------------|
| 1 | R-1.7.1     |
| 2 | R-1.7.1     |
| 3 | R-1.7.1     |

1. **#1** — 100,000 varying-size bump allocations (16-256 bytes)
   - **Expected:** All pointers valid, non-overlapping
2. **#2** — Measure total time
   - **Expected:** < 1 ms
3. **#3** — Verify watermark
   - **Expected:** Equals sum of sizes + alignment padding

### TC-1.7.1.2 Arena Reset Zero Cost

| # | Requirement |
|---|-------------|
| 1 | R-1.7.1     |
| 2 | R-1.7.1     |

1. **#1** — Allocate 8 MB from arena
   - **Expected:** Watermark at 8 MB
2. **#2** — `reset()`
   - **Expected:** Watermark = 0, time < 1 us

### TC-1.7.1.3 Arena Overflow Error

| # | Requirement |
|---|-------------|
| 1 | R-1.7.1a    |
| 2 | R-1.7.1a    |

1. **#1** — Fill arena to 99% capacity
   - **Expected:** Allocation succeeds
2. **#2** — Attempt allocation exceeding remaining
   - **Expected:** `ArenaError::OutOfMemory` with sizes

### TC-1.7.1.4 Arena Grow by Doubling

| # | Requirement |
|---|-------------|
| 1 | R-1.7.1a    |
| 2 | R-1.7.1a    |
| 3 | R-1.7.1a    |

1. **#1** — Exhaust initial 8 MiB arena
   - **Expected:** Capacity at 8 MiB
2. **#2** — Allocate 1 more byte
   - **Expected:** Arena doubles to 16 MiB
3. **#3** — Continue until configured max
   - **Expected:** Growth stops at max

### TC-1.7.2.1 Scoped Arena Restore

| # | Requirement |
|---|-------------|
| 1 | R-1.7.2     |
| 2 | R-1.7.2     |
| 3 | R-1.7.2     |

1. **#1** — Parent arena 1 MB, create child scope
   - **Expected:** Parent remaining = 1 MB
2. **#2** — Child allocates 512 KB
   - **Expected:** Child has 512 KB used
3. **#3** — Drop child
   - **Expected:** Parent remaining restored to 1 MB

### TC-1.7.2.2 Scoped Arena Nested 10

| # | Requirement |
|---|-------------|
| 1 | R-1.7.2     |
| 2 | R-1.7.2     |

1. **#1** — 10 nested scopes, each allocating 1 KB
   - **Expected:** 10 KB used total at depth 10
2. **#2** — Drop scopes in reverse order
   - **Expected:** Watermark correct at each level

### TC-1.7.3.1 Pool O(1) Alloc/Dealloc

| # | Requirement |
|---|-------------|
| 1 | R-1.7.3     |
| 2 | R-1.7.3     |

1. **#1** — 10,000 random alloc/dealloc ops
   - **Expected:** All operations succeed
2. **#2** — Benchmark alloc time at 10%, 50%, 90% occupancy
   - **Expected:** Constant time regardless of occupancy

### TC-1.7.3.2 Pool Zero Fragmentation

| # | Requirement |
|---|-------------|
| 1 | R-1.7.3     |

1. **#1** — After random alloc/dealloc sequence
   - **Expected:** `total_memory == block_count * block_size`

### TC-1.7.4.1 Handle Generation Mismatch

| # | Requirement |
|---|-------------|
| 1 | R-1.7.4     |
| 2 | R-1.7.4     |
| 3 | R-1.7.4     |

1. **#1** — Alloc handle H1(idx=0, gen=0), remove
   - **Expected:** Slot gen incremented to 1
2. **#2** — Alloc new H2 at same index
   - **Expected:** H2(idx=0, gen=1) returned
3. **#3** — Validate H1
   - **Expected:** Returns `GenerationMismatch`

### TC-1.7.4.2 Handle Validate 1M

| # | Requirement |
|---|-------------|
| 1 | R-1.7.4     |
| 2 | R-1.7.4     |

1. **#1** — Validate 1,000,000 handles
   - **Expected:** All validations complete
2. **#2** — Measure per-validation time
   - **Expected:** O(1) per validation

### TC-1.7.5.1 SlotMap Dense Iteration

| # | Requirement |
|---|-------------|
| 1 | R-1.7.5     |
| 2 | R-1.7.5     |

1. **#1** — Insert 10,000 entries, remove 5,000 random
   - **Expected:** 5,000 remain
2. **#2** — Dense iteration
   - **Expected:** Visits exactly 5,000 entries

### TC-1.7.5.2 SlotMap 4M Entries

| # | Requirement |
|---|-------------|
| 1 | R-1.7.5a    |
| 2 | R-1.7.5a    |

1. **#1** — Insert 4,000,000 entries
   - **Expected:** All inserted
2. **#2** — Lookup each by handle
   - **Expected:** All lookups succeed

### TC-1.7.5.3 SlotMap Stale Error

| # | Requirement |
|---|-------------|
| 1 | R-1.7.5a    |

1. **#1** — Remove entry, lookup with old handle
   - **Expected:** `GenerationMismatch(expected=1, actual=0)`

### TC-1.7.6.1 Budget Eviction

| # | Requirement |
|---|-------------|
| 1 | R-1.7.6     |
| 2 | R-1.7.6     |

1. **#1** — Set 100 MB budget, allocate to limit
   - **Expected:** All allocations succeed
2. **#2** — Attempt next allocation
   - **Expected:** `BudgetExceeded` error

### TC-1.7.7.1 Profiling Hooks Dev Build

| # | Requirement |
|---|-------------|
| 1 | R-1.7.7     |
| 2 | R-1.7.7     |

1. **#1** — 1,000 allocations across 3 allocators (dev build)
   - **Expected:** Correct alloc counts per allocator
2. **#2** — Query byte totals and peak
   - **Expected:** Match expected values

### TC-1.7.7.2 Profiling Compiled Out Release

| # | Requirement |
|---|-------------|
| 1 | R-1.7.7     |

1. **#1** — Build release binary
   - **Expected:** No profiling symbols in binary

### TC-1.7.8.1 Tag Propagation

| # | Requirement |
|---|-------------|
| 1 | R-1.7.8     |
| 2 | R-1.7.8     |
| 3 | R-1.7.8     |

1. **#1** — Parent arena tagged "physics"
   - **Expected:** Tag = "physics"
2. **#2** — Create child scope
   - **Expected:** Child inherits "physics" tag
3. **#3** — Query per-tag stats
   - **Expected:** "physics" stats sum parent + child

### TC-1.7.9.1 BigInt Determinism

| # | Requirement |
|---|-------------|
| 1 | R-1.7.9     |

1. **#1** — Compute distance at 10M light-years using BigInt
   - **Expected:** Bit-identical on x86_64 and aarch64

### TC-1.7.9.2 BigFloat Conversion

| # | Requirement |
|---|-------------|
| 1 | R-1.7.9     |
| 2 | R-1.7.9     |

1. **#1** — BigFloat -> f32 -> BigFloat round-trip
   - **Expected:** Deterministic across architectures
2. **#2** — BigFloat -> f64 -> BigFloat round-trip
   - **Expected:** Deterministic across architectures

### TC-1.8.3.1 Async Read Data Integrity

| # | Requirement |
|---|-------------|
| 1 | R-1.8.3     |
| 2 | R-1.8.3     |

1. **#1** — Write 4 MB at explicit offsets from 4 async tasks
   - **Expected:** Writes complete
2. **#2** — Read back all 4 regions
   - **Expected:** Data matches written bytes exactly

### TC-1.8.3.2 No Stdlib File I/O

| # | Requirement |
|---|-------------|
| 1 | R-1.8.3     |

1. **#1** — Static analysis of codebase
   - **Expected:** Zero uses of `std::fs` or `std::io::Read/Write`

### TC-1.8.2.1 Completion Typed Result

| # | Requirement |
|---|-------------|
| 1 | R-1.8.2     |
| 2 | R-1.8.2     |

1. **#1** — Submit 1 MB async write
   - **Expected:** Completion received
2. **#2** — Inspect completion
   - **Expected:** Correct byte count and context attached

### TC-1.8.2.2 Completion Latency P99

| # | Requirement |
|---|-------------|
| 1 | R-1.8.2a    |
| 2 | R-1.8.2a    |

1. **#1** — 10,000 concurrent 4 KB reads
   - **Expected:** All complete
2. **#2** — Measure p99 delivery latency
   - **Expected:** < 100 us

### TC-1.8.6.1 Vectored Write Integrity

| # | Requirement |
|---|-------------|
| 1 | R-1.8.6     |
| 2 | R-1.8.6     |

1. **#1** — Write 3 non-contiguous buffers via single vectored write
   - **Expected:** Write completes
2. **#2** — Read back
   - **Expected:** Data equals concatenation of 3 buffers

### TC-1.8.6.2 Vectored Syscall Reduction

| # | Requirement |
|---|-------------|
| 1 | R-1.8.6     |

1. **#1** — Benchmark vectored vs individual writes (same data)
   - **Expected:** Vectored uses >= 30% fewer syscalls

### TC-1.8.7.1 Priority Ordering

| # | Requirement |
|---|-------------|
| 1 | R-1.8.7     |
| 2 | R-1.8.7     |

1. **#1** — Submit 100 background reads, then 1 critical read
   - **Expected:** All complete
2. **#2** — Verify completion order
   - **Expected:** Critical completes before 90% of background

### TC-1.8.8.1 Cancel Fires Completion

| # | Requirement |
|---|-------------|
| 1 | R-1.8.8     |
| 2 | R-1.8.8     |

1. **#1** — Submit 100 MB read, cancel within 1 ms
   - **Expected:** `IoError::Cancelled` completion fires
2. **#2** — Verify resources
   - **Expected:** No buffer leak

### TC-1.8.8.2 Cancel 1000 No Leaks

| # | Requirement |
|---|-------------|
| 1 | R-1.8.8a    |
| 2 | R-1.8.8a    |

1. **#1** — Submit 1,000 ops, cancel all
   - **Expected:** All complete within 10 ms
2. **#2** — Verify resources
   - **Expected:** No buffer or handle leaks

### TC-1.8.9.1 Buffer Pool Backpressure

| # | Requirement |
|---|-------------|
| 1 | R-1.8.9     |
| 2 | R-1.8.9     |

1. **#1** — Register 64 buffers, submit 128 reads
   - **Expected:** 64 execute, 64 queued
2. **#2** — Complete first 64
   - **Expected:** Queued 64 begin executing

### TC-1.8.9.2 Buffer Pool Reclaim

| # | Requirement |
|---|-------------|
| 1 | R-1.8.9     |

1. **#1** — All 128 completions finish
   - **Expected:** All 64 buffers returned to free list

### TC-1.8.1.1 Reactor Nothing Before Poll

| # | Requirement |
|---|-------------|
| 1 | R-1.8.1     |
| 2 | R-1.8.1     |

1. **#1** — Submit async I/O, do not call `poll()`
   - **Expected:** No future wakes fire
2. **#2** — Call `poll()`
   - **Expected:** Completions delivered

## Integration Tests

### TC-1.8.1.I1 Backend Per Platform

| # | Requirement |
|---|-------------|
| 1 | R-1.8.1     |
| 2 | R-1.8.1     |
| 3 | R-1.8.1     |

1. **#1** — Run full test suite on Windows
   - **Expected:** All tests pass (IOCP via windows-rs)
2. **#2** — Run full test suite on macOS
   - **Expected:** All tests pass (GCD `dispatch_io` via dispatch2)
3. **#3** — Run full test suite on Linux
   - **Expected:** All tests pass (io_uring via rustix)

### TC-1.8.4.I1 TCP Connect Accept 1MB

| # | Requirement |
|---|-------------|
| 1 | R-1.8.4     |
| 2 | R-1.8.4     |

1. **#1** — Async TCP connect/accept
   - **Expected:** Connection established
2. **#2** — Send 1 MB, read on other end
   - **Expected:** 1 MB received, data intact

### TC-1.8.4.I2 UDP 1000 Datagrams

| # | Requirement |
|---|-------------|
| 1 | R-1.8.4     |
| 2 | R-1.8.4     |

1. **#1** — Send 1,000 UDP datagrams
   - **Expected:** All sent
2. **#2** — Count received
   - **Expected:** Delivery count matches expected (minus loss)

### TC-1.8.4.I3 Concurrent TCP 500

| # | Requirement |
|---|-------------|
| 1 | R-1.8.4     |
| 2 | R-1.8.4     |

1. **#1** — Open 500 concurrent TCP connections
   - **Expected:** All established
2. **#2** — Close all
   - **Expected:** No handle leaks

### TC-1.8.5.I1 Audio Latency Under 10ms

| # | Requirement |
|---|-------------|
| 1 | R-1.8.5     |
| 2 | R-1.8.5     |

1. **#1** — Audio writes concurrent with 100 MB background reads
   - **Expected:** Audio operations complete
2. **#2** — Measure p99 audio latency over 60 s
   - **Expected:** < 10 ms, zero underruns

### TC-1.7.6.I1 Budget 24H Server

| # | Requirement |
|---|-------------|
| 1 | R-1.7.6     |
| 2 | R-1.7.6     |

1. **#1** — 24-hour sustained server load
   - **Expected:** No OOM
2. **#2** — Monitor budget at end
   - **Expected:** Budget never exceeded

### TC-1.7.1.I2 Engine Developer Uses Bump Arena That Resets Per Frame

| # | Requirement |
|---|-------------|
| 1 | US-1.7.1    |
| 2 | US-1.7.1    |

1. **#1** — Engine developer allocates 50K temporary records from per-frame arena in `Update` phase
   - **Expected:** All allocations succeed, arena watermark reflects total size
2. **#2** — End-of-frame system calls `arena.reset()`
   - **Expected:** Watermark returns to zero, allocator reports zero outstanding allocations

### TC-1.7.2.I1 Engine Tester Benchmarks Per-Frame Arena Allocations

| # | Requirement |
|---|-------------|
| 1 | US-1.7.2    |
| 2 | US-1.7.2    |

1. **#1** — Engine tester runs criterion bench allocating 100K varying-size records per frame
   - **Expected:** Benchmark records mean throughput and writes to CI timing report
2. **#2** — Re-run bench three times on same hardware
   - **Expected:** Result within 10 % of baseline, confirming benchmark stability

### TC-1.7.5.I1 Engine Developer Uses O(1) Fixed-Size Block Pool

| # | Requirement |
|---|-------------|
| 1 | US-1.7.5    |
| 2 | US-1.7.5    |

1. **#1** — Engine developer allocates 10K fixed-size blocks from `Pool<T>` during world startup
   - **Expected:** Every `alloc()` returns a valid handle in constant time
2. **#2** — Free all 10K blocks then re-allocate 10K
   - **Expected:** Handles reused from free list, zero memory growth in pool

### TC-1.7.7.I1 Engine Developer Uses Generational Handles

| # | Requirement |
|---|-------------|
| 1 | US-1.7.7    |
| 2 | US-1.7.7    |

1. **#1** — Engine developer stores 100K entities in `SlotMap<Entity>`, captures handles, then frees
   half
   - **Expected:** Reusing freed indices produces handles with incremented generation
2. **#2** — Validate captured stale handles against current slot map
   - **Expected:** `get(stale)` returns `None`, validation completes in O(1) per call

### TC-1.7.8.I1 Engine Developer Iterates Dense Slot Map

| # | Requirement |
|---|-------------|
| 1 | US-1.7.8    |
| 2 | US-1.7.8    |

1. **#1** — Populate `SlotMap` with 1M values, then iterate via `values()`
   - **Expected:** Iteration visits every live value exactly once with no dead slots skipped
2. **#2** — Compare iteration time to `Vec<T>` of same length
   - **Expected:** Time within 1.5x of dense `Vec` iteration, confirming dense storage

### TC-1.8.7.I1 Game Developer Uses Async File Read/Write/Seek/Flush

| # | Requirement |
|---|-------------|
| 1 | US-1.8.7    |
| 2 | US-1.8.7    |

1. **#1** — Game developer submits `read`, `write`, `seek`, `flush` operations for a save file
   - **Expected:** Each op issued to platform backend, returns a completion handle
2. **#2** — Poll reactor and inspect results
   - **Expected:** All four completions arrive with correct typed results; file contents match
     expected bytes

### TC-1.8.14.I1 Engine Tester Verifies Scatter-Gather Vectored I/O

| # | Requirement |
|---|-------------|
| 1 | US-1.8.14   |
| 2 | US-1.8.14   |

1. **#1** — Engine tester submits vectored write with 8 non-contiguous buffers
   - **Expected:** Single syscall issued, all buffers written in order
2. **#2** — Inspect syscall count via strace/dtrace probe
   - **Expected:** Exactly one `writev`/`pwritev` (or platform equivalent), not 8 separate writes

### TC-1.8.19.I1 Engine Developer Uses Pre-Allocated Page-Aligned I/O Buffers

| # | Requirement |
|---|-------------|
| 1 | US-1.8.19   |
| 2 | US-1.8.19   |

1. **#1** — Engine developer requests 32 registered buffers, each 4 KiB
   - **Expected:** All buffer base pointers aligned to 4096 bytes, registered once with backend
2. **#2** — Submit 1000 reads/writes recycling the 32 buffers
   - **Expected:** No runtime allocation, backend reuses registered buffer IDs for every op

## Benchmarks

### TC-1.7.1.B1 Arena Alloc Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Bump allocations, varying sizes | Allocs/sec | > 100M | R-1.7.1 |

### TC-1.7.1.B2 Arena Reset

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Reset 8 MiB arena | Time | < 1 us | R-1.7.1 |

### TC-1.7.3.B1 Pool Alloc/Dealloc

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Single alloc from pool | Latency | O(1), < 50 ns | R-1.7.3 |
| 2 | Single dealloc to pool | Latency | O(1), < 50 ns | R-1.7.3 |

### TC-1.7.4.B1 Handle Validation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Validate single handle | Latency | O(1), < 10 ns | R-1.7.4 |

### TC-1.7.5.B1 SlotMap Dense Iteration

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Dense iterate 10,000 entries | Time | < 50 us | R-1.7.5 |

### TC-1.8.9.B1 Registered Buffer I/O

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Registered buffer read throughput | Bandwidth | >= 80% raw disk bandwidth | R-1.8.9 |

### TC-1.8.1.B1 Reactor Poll

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Poll 100 completions | Time | < 50 us | R-1.8.1 |

### TC-1.8.7.B1 Priority Critical vs Background

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Critical read latency under background load | Ratio | Critical < 2x background | R-1.8.7 |

### TC-1.8.6.B1 Vectored Write Efficiency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Vectored write vs individual writes | Syscall count | >= 30% fewer | R-1.8.6 |
