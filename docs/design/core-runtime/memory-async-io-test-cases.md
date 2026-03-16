# Memory Management & Async I/O Test Cases

Companion test cases for [memory-async-io.md](memory-async-io.md).

## Unit Tests

### TC-1.7.1.1 Arena 100K Allocations Under 1ms

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 100,000 varying-size bump allocations (16-256 bytes) | All pointers valid, non-overlapping | R-1.7.1 |
| 2 | Measure total time | < 1 ms | R-1.7.1 |
| 3 | Verify watermark | Equals sum of sizes + alignment padding | R-1.7.1 |

### TC-1.7.1.2 Arena Reset Zero Cost

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Allocate 8 MB from arena | Watermark at 8 MB | R-1.7.1 |
| 2 | `reset()` | Watermark = 0, time < 1 us | R-1.7.1 |

### TC-1.7.1.3 Arena Overflow Error

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fill arena to 99% capacity | Allocation succeeds | R-1.7.1a |
| 2 | Attempt allocation exceeding remaining | `ArenaError::OutOfMemory` with sizes | R-1.7.1a |

### TC-1.7.1.4 Arena Grow by Doubling

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Exhaust initial 8 MiB arena | Capacity at 8 MiB | R-1.7.1a |
| 2 | Allocate 1 more byte | Arena doubles to 16 MiB | R-1.7.1a |
| 3 | Continue until configured max | Growth stops at max | R-1.7.1a |

### TC-1.7.2.1 Scoped Arena Restore

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Parent arena 1 MB, create child scope | Parent remaining = 1 MB | R-1.7.2 |
| 2 | Child allocates 512 KB | Child has 512 KB used | R-1.7.2 |
| 3 | Drop child | Parent remaining restored to 1 MB | R-1.7.2 |

### TC-1.7.2.2 Scoped Arena Nested 10

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10 nested scopes, each allocating 1 KB | 10 KB used total at depth 10 | R-1.7.2 |
| 2 | Drop scopes in reverse order | Watermark correct at each level | R-1.7.2 |

### TC-1.7.3.1 Pool O(1) Alloc/Dealloc

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10,000 random alloc/dealloc ops | All operations succeed | R-1.7.3 |
| 2 | Benchmark alloc time at 10%, 50%, 90% occupancy | Constant time regardless of occupancy | R-1.7.3 |

### TC-1.7.3.2 Pool Zero Fragmentation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | After random alloc/dealloc sequence | `total_memory == block_count * block_size` | R-1.7.3 |

### TC-1.7.4.1 Handle Generation Mismatch

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Alloc handle H1(idx=0, gen=0), remove | Slot gen incremented to 1 | R-1.7.4 |
| 2 | Alloc new H2 at same index | H2(idx=0, gen=1) returned | R-1.7.4 |
| 3 | Validate H1 | Returns `GenerationMismatch` | R-1.7.4 |

### TC-1.7.4.2 Handle Validate 1M

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Validate 1,000,000 handles | All validations complete | R-1.7.4 |
| 2 | Measure per-validation time | O(1) per validation | R-1.7.4 |

### TC-1.7.5.1 SlotMap Dense Iteration

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Insert 10,000 entries, remove 5,000 random | 5,000 remain | R-1.7.5 |
| 2 | Dense iteration | Visits exactly 5,000 entries | R-1.7.5 |

### TC-1.7.5.2 SlotMap 4M Entries

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Insert 4,000,000 entries | All inserted | R-1.7.5a |
| 2 | Lookup each by handle | All lookups succeed | R-1.7.5a |

### TC-1.7.5.3 SlotMap Stale Error

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Remove entry, lookup with old handle | `GenerationMismatch(expected=1, actual=0)` | R-1.7.5a |

### TC-1.7.6.1 Budget Eviction

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Set 100 MB budget, allocate to limit | All allocations succeed | R-1.7.6 |
| 2 | Attempt next allocation | `BudgetExceeded` error | R-1.7.6 |

### TC-1.7.7.1 Profiling Hooks Dev Build

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1,000 allocations across 3 allocators (dev build) | Correct alloc counts per allocator | R-1.7.7 |
| 2 | Query byte totals and peak | Match expected values | R-1.7.7 |

### TC-1.7.7.2 Profiling Compiled Out Release

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Build release binary | No profiling symbols in binary | R-1.7.7 |

### TC-1.7.8.1 Tag Propagation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Parent arena tagged "physics" | Tag = "physics" | R-1.7.8 |
| 2 | Create child scope | Child inherits "physics" tag | R-1.7.8 |
| 3 | Query per-tag stats | "physics" stats sum parent + child | R-1.7.8 |

### TC-1.7.9.1 BigInt Determinism

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Compute distance at 10M light-years using BigInt | Bit-identical on x86_64 and aarch64 | R-1.7.9 |

### TC-1.7.9.2 BigFloat Conversion

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | BigFloat -> f32 -> BigFloat round-trip | Deterministic across architectures | R-1.7.9 |
| 2 | BigFloat -> f64 -> BigFloat round-trip | Deterministic across architectures | R-1.7.9 |

### TC-1.8.3.1 Async Read Data Integrity

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Write 4 MB at explicit offsets from 4 async tasks | Writes complete | R-1.8.3 |
| 2 | Read back all 4 regions | Data matches written bytes exactly | R-1.8.3 |

### TC-1.8.3.2 No Stdlib File I/O

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Static analysis of codebase | Zero uses of `std::fs` or `std::io::Read/Write` | R-1.8.3 |

### TC-1.8.2.1 Completion Typed Result

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Submit 1 MB async write | Completion received | R-1.8.2 |
| 2 | Inspect completion | Correct byte count and context attached | R-1.8.2 |

### TC-1.8.2.2 Completion Latency P99

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10,000 concurrent 4 KB reads | All complete | R-1.8.2a |
| 2 | Measure p99 delivery latency | < 100 us | R-1.8.2a |

### TC-1.8.6.1 Vectored Write Integrity

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Write 3 non-contiguous buffers via single vectored write | Write completes | R-1.8.6 |
| 2 | Read back | Data equals concatenation of 3 buffers | R-1.8.6 |

### TC-1.8.6.2 Vectored Syscall Reduction

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Benchmark vectored vs individual writes (same data) | Vectored uses >= 30% fewer syscalls | R-1.8.6 |

### TC-1.8.7.1 Priority Ordering

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Submit 100 background reads, then 1 critical read | All complete | R-1.8.7 |
| 2 | Verify completion order | Critical completes before 90% of background | R-1.8.7 |

### TC-1.8.8.1 Cancel Fires Completion

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Submit 100 MB read, cancel within 1 ms | `IoError::Cancelled` completion fires | R-1.8.8 |
| 2 | Verify resources | No buffer leak | R-1.8.8 |

### TC-1.8.8.2 Cancel 1000 No Leaks

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Submit 1,000 ops, cancel all | All complete within 10 ms | R-1.8.8a |
| 2 | Verify resources | No buffer or handle leaks | R-1.8.8a |

### TC-1.8.9.1 Buffer Pool Backpressure

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Register 64 buffers, submit 128 reads | 64 execute, 64 queued | R-1.8.9 |
| 2 | Complete first 64 | Queued 64 begin executing | R-1.8.9 |

### TC-1.8.9.2 Buffer Pool Reclaim

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | All 128 completions finish | All 64 buffers returned to free list | R-1.8.9 |

### TC-1.8.1.1 Reactor Nothing Before Poll

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Submit async I/O, do not call `poll()` | No future wakes fire | R-1.8.1 |
| 2 | Call `poll()` | Completions delivered | R-1.8.1 |

## Integration Tests

### TC-1.8.1.I1 Backend Per Platform

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Run full test suite on Windows | All tests pass (IOCP backend) | R-1.8.1 |
| 2 | Run full test suite on macOS | All tests pass (GCD backend) | R-1.8.1 |
| 3 | Run full test suite on Linux | All tests pass (io_uring backend) | R-1.8.1 |

### TC-1.8.4.I1 TCP Connect Accept 1MB

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Async TCP connect/accept | Connection established | R-1.8.4 |
| 2 | Send 1 MB, read on other end | 1 MB received, data intact | R-1.8.4 |

### TC-1.8.4.I2 UDP 1000 Datagrams

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Send 1,000 UDP datagrams | All sent | R-1.8.4 |
| 2 | Count received | Delivery count matches expected (minus loss) | R-1.8.4 |

### TC-1.8.4.I3 Concurrent TCP 500

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Open 500 concurrent TCP connections | All established | R-1.8.4 |
| 2 | Close all | No handle leaks | R-1.8.4 |

### TC-1.8.5.I1 Audio Latency Under 10ms

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Audio writes concurrent with 100 MB background reads | Audio operations complete | R-1.8.5 |
| 2 | Measure p99 audio latency over 60 s | < 10 ms, zero underruns | R-1.8.5 |

### TC-1.7.6.I1 Budget 24H Server

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 24-hour sustained server load | No OOM | R-1.7.6 |
| 2 | Monitor budget at end | Budget never exceeded | R-1.7.6 |

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
