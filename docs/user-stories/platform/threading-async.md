# User Stories -- 14.3 Threading & Async

## US-14.3.1 Utilize All CPU Cores Efficiently

**As a** game developer (P-15), **I want** a work-stealing thread pool sized to performance
cores with per-thread local queues, **so that** CPU-parallel work (physics, culling, asset
decompression) is distributed evenly across all available cores.

## US-14.3.2 Pin Critical Threads to Performance Cores

**As an** engine developer (P-26), **I want** thread affinity and OS-level priority control
to pin the main and render threads to performance cores and background I/O to efficiency
cores, **so that** game-critical work is never preempted by background tasks on hybrid CPU
architectures.

## US-14.3.3 Express Parallel Work as a Task Graph

**As a** game developer (P-15), **I want** a DAG-based job system with typed data
dependencies, fan-out, fan-in, continuations, and nested sub-graphs, **so that** frame work
(physics substeps, rendering prep) is parallelized automatically based on declared
dependencies.

## US-14.3.4 Yield Long-Running Jobs Without Blocking Threads

**As an** engine developer (P-26), **I want** lightweight fibers with platform-specific
context switching (CreateFiber, ucontext/assembly) that can suspend mid-execution and
resume on any worker thread, **so that** pathfinding, procedural generation, and other
long-running jobs yield without wasting a thread.

## US-14.3.5 Complete I/O Without Blocking Worker Threads

**As an** engine developer (P-26), **I want** platform async I/O completions (IOCP, GCD,
io_uring) dispatched as job-system continuations, **so that** file reads, network
operations, and GPU readbacks integrate with the unified task scheduler without blocking
any worker thread.

## US-14.3.6 Run 40-Player Raids at Consistent Frame Time

**As a** player (P-23), **I want** the engine to fully utilize my CPU during large-scale
content (raids, guild sieges) without frame time spikes, **so that** gameplay stays smooth
even with hundreds of entities updating simultaneously.

## US-14.3.7 Detect Hybrid Core Topologies at Startup

**As an** engine developer (P-26), **I want** automatic detection of performance and
efficiency cores via cpuid (x86) and IOKit/sysctl (Apple Silicon) at startup, **so that**
thread pool sizing and affinity are configured correctly on hybrid architectures without
manual tuning.

## US-14.3.8 Verify Work-Stealing Load Balance Under Imbalanced Workloads

**As an** engine tester (P-27), **I want** benchmarks that submit imbalanced job graphs
(some branches heavy, some light) and measure per-thread utilization, **so that** I can
verify work-stealing achieves at least 80% CPU utilization under realistic workloads.

## US-14.3.9 Verify Fiber Context Switching on All Platforms

**As an** engine tester (P-27), **I want** automated tests that create, suspend, and resume
fibers on all platforms (CreateFiber on Windows, ucontext/assembly on macOS/Linux) and
verify stack integrity, **so that** fiber implementation regressions are caught in CI.

## US-14.3.10 Verify No Worker Thread Blocks on I/O

**As an** engine tester (P-27), **I want** stress tests that issue concurrent file reads and
network operations under full CPU load and verify no worker thread blocks for I/O, **so
that** I can confirm the async I/O integration never causes thread starvation.

## US-14.3.11 Profile Job Scheduling Overhead

**As an** engine developer (P-26), **I want** microbenchmarks that measure per-job dispatch
and completion overhead in the task graph scheduler, **so that** scheduling cost does not
exceed a fixed budget (target < 1 microsecond per job dispatch) on all platforms.

## US-14.3.12 Design Fiber Stack Sizes for Each Workload Category

**As a** game developer (P-15), **I want** configurable fiber stack sizes per workload
category (default 64 KiB with guard pages), **so that** pathfinding and procedural
generation fibers have sufficient stack space without wasting memory for simpler jobs.
