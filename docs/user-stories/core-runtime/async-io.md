# Async I/O User Stories

## Platform I/O Backends

## US-1.8.1 Abstract Platform I/O With Zero Dynamic Dispatch

**As an** engine developer (P-26), **I want** a trait-based abstraction over IOCP, GCD, and io_uring
resolved at compile time with static dispatch, **so that** every I/O operation uses platform-native
async primitives without any subsystem bypassing the unified layer.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Compile-time backend selection via conditional compilation | F-1.8.1 | R-1.8.1 |
| Zero dynamic dispatch, no trait objects | F-1.8.1 | R-1.8.1 |
| All I/O routes through this abstraction | F-1.8.1 | R-1.8.1 |

## US-1.8.2 Implement IOCP Backend for Windows

**As an** engine developer (P-26), **I want** a Windows I/O backend wrapping CreateIoCompletionPort
and GetQueuedCompletionStatusEx via bindgen, **so that** Windows I/O uses native IOCP for maximum
async performance.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| CreateIoCompletionPort and GetQueuedCompletionStatusEx wrapped | F-1.8.1 | R-1.8.1 |
| C COM wrappers through bindgen | F-1.8.1 | R-1.8.1 |
| All file and socket operations use IOCP | F-1.8.1 | R-1.8.1 |

## US-1.8.3 Implement GCD Backend for macOS

**As an** engine developer (P-26), **I want** a macOS I/O backend wrapping dispatch_io_create,
dispatch_io_read, and dispatch_io_write via cxx.rs Objective-C++ wrappers, **so that** macOS I/O
uses native Grand Central Dispatch.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| dispatch_io APIs wrapped via cxx.rs | F-1.8.1 | R-1.8.1 |
| Objective-C++ wrappers for GCD integration | F-1.8.1 | R-1.8.1 |
| All file and socket operations use GCD | F-1.8.1 | R-1.8.1 |

## US-1.8.4 Verify Backend Abstraction Passes Same Tests on All Platforms

**As an** engine tester (P-27), **I want** to run the same I/O test suite against IOCP, GCD, and
io_uring backends, **so that** I can verify behavioral equivalence across all platforms.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Same test suite passes on Windows, macOS, Linux | F-1.8.1 | R-1.8.1 |
| Completion semantics identical across backends | F-1.8.1 | R-1.8.1 |
| Error codes mapped to platform-agnostic types | F-1.8.1 | R-1.8.1 |

## Completion Model

## US-1.8.5 Receive Typed Completion Notifications From the OS Kernel

**As an** engine developer (P-26), **I want** a completion-based proactor model where the OS
notifies on I/O completion with typed results and context tokens, **so that** I avoid the extra
copies and retry loops of readiness-based models.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Completion notifications carry typed results | F-1.8.2 | R-1.8.2 |
| Context token for caller correlation | F-1.8.2 | R-1.8.2 |
| No readiness polling or retry loops | F-1.8.2 | R-1.8.2 |
| Platform-appropriate in-flight operation limits | F-1.8.2 | R-1.8.2 |

## US-1.8.6 Stress-Test In-Flight Operation Limits

**As an** engine tester (P-27), **I want** to stress-test the async I/O system at maximum in-flight
operation counts per platform, **so that** I can verify the system handles saturation without
deadlock or dropped completions.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Mobile: 32 in-flight operations without deadlock | F-1.8.2 | R-1.8.2 |
| Desktop: 256 in-flight operations sustained | F-1.8.2 | R-1.8.2 |
| No dropped completions under saturation | F-1.8.2 | R-1.8.2 |

## File I/O

## US-1.8.7 Load Assets and Save Games via Async File I/O

**As a** game developer (P-15), **I want** async file read, write, seek, and flush with explicit
byte offsets routed through the platform backend, **so that** asset loading and save game
persistence never block worker threads.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Async read, write, seek, flush operations | F-1.8.3 | R-1.8.3 |
| Explicit byte offsets for positional I/O | F-1.8.3 | R-1.8.3 |
| No Rust standard library file I/O used | F-1.8.3 | R-1.8.3 |
| Concurrent access to different file regions | F-1.8.3 | R-1.8.3 |

## US-1.8.8 Experience Smooth Loading Without Hitches

**As a** player (P-23), **I want** asset loading and save operations to happen in the background
without causing frame rate drops, **so that** gameplay is never interrupted by disk I/O.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| No frame drops during background asset loading | F-1.8.3 | R-1.8.3 |
| Save operations complete without gameplay stutter | F-1.8.3 | R-1.8.3 |
| Loading progress reported accurately | F-1.8.3 | R-1.8.3 |

## Network Socket I/O

## US-1.8.9 Use Async TCP and UDP Sockets for Networking

**As a** game developer (P-15), **I want** async TCP and UDP socket operations registered with the
completion mechanism on creation, **so that** the transport layer, server infrastructure, and
telemetry have a single non-blocking network I/O path.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Async accept, connect, send, receive, sendto, recvfrom | F-1.8.4 | R-1.8.4 |
| Sockets registered with completion mechanism on creation | F-1.8.4 | R-1.8.4 |
| Single network I/O path for all subsystems | F-1.8.4 | R-1.8.4 |

## US-1.8.10 Verify Network Socket I/O Under Concurrent Connections

**As an** engine tester (P-27), **I want** to verify async socket I/O handles hundreds of concurrent
connections without resource leaks or dropped packets, **so that** MMO server infrastructure
operates reliably at scale.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| 500+ concurrent TCP connections sustained | F-1.8.4 | R-1.8.4 |
| No socket handle leaks after disconnect | F-1.8.4 | R-1.8.4 |
| UDP send/receive at high packet rates without drops | F-1.8.4 | R-1.8.4 |

## Audio Stream I/O

## US-1.8.11 Stream Audio Without Buffer Underruns

**As a** game developer (P-15), **I want** low-latency async audio buffer I/O with deadline hints
that prioritize audio over bulk transfers, **so that** audio playback maintains sub-10ms latency
without underruns during asset streaming.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Audio I/O with deadline hints for priority scheduling | F-1.8.5 | R-1.8.5 |
| Sub-10ms round-trip latency | F-1.8.5 | R-1.8.5 |
| No buffer underruns during concurrent asset streaming | F-1.8.5 | R-1.8.5 |

## US-1.8.12 Hear Smooth Audio During Loading and Level Transitions

**As a** player (P-23), **I want** audio playback that never stutters or glitches during loading
screens or level transitions, **so that** the audio experience remains smooth and immersive.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| No audio glitches during level load | F-1.8.5 | R-1.8.5 |
| Background music continues smoothly during transitions | F-1.8.5 | R-1.8.5 |
| Microphone input uninterrupted during loading | F-1.8.5 | R-1.8.5 |

## Vectored I/O

## US-1.8.13 Assemble Composite Writes Without Staging Copies

**As an** engine developer (P-26), **I want** scatter-gather operations that read into or write from
multiple non-contiguous buffers in a single system call, **so that** composite writes and protocol
framing avoid staging buffer copies.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Multiple buffers in a single read/write call | F-1.8.6 | R-1.8.6 |
| Header + metadata + payload assembled without copy | F-1.8.6 | R-1.8.6 |
| Network protocol headers prepended at zero copy | F-1.8.6 | R-1.8.6 |

## US-1.8.14 Verify Vectored I/O Data Integrity

**As an** engine tester (P-27), **I want** to verify that scatter-gather operations produce correct
output when assembling data from multiple non-contiguous buffers, **so that** no bytes are lost,
reordered, or corrupted during composite writes.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Assembled output matches expected byte sequence | F-1.8.6 | R-1.8.6 |
| Multiple buffer chains of varying sizes correct | F-1.8.6 | R-1.8.6 |
| Works correctly on all three platform backends | F-1.8.6 | R-1.8.6 |

## Priority and Scheduling

## US-1.8.15 Prioritize Audio and Frame-Critical I/O Over Background Work

**As an** engine developer (P-26), **I want** I/O submissions with priority levels (critical,
normal, background) and deadline hints that reorder the submission queue, **so that** audio buffers
and frame-critical assets are serviced before prefetch and log flushing.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Three priority levels: critical, normal, background | F-1.8.7 | R-1.8.7 |
| Deadline hints for latency-sensitive I/O | F-1.8.7 | R-1.8.7 |
| Critical I/O serviced ahead of background | F-1.8.7 | R-1.8.7 |

## US-1.8.16 Verify Priority Scheduling Under Load

**As an** engine tester (P-27), **I want** to verify that critical I/O operations complete before
background operations when the I/O system is saturated, **so that** audio buffers and frame-critical
assets are never starved by bulk transfers.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Critical I/O latency unaffected by background saturation | F-1.8.7 | R-1.8.7 |
| Background I/O completes eventually (no starvation) | F-1.8.7 | R-1.8.7 |
| Priority ordering verified with latency measurements | F-1.8.7 | R-1.8.7 |

## Cancellation

## US-1.8.17 Cancel Stale I/O Operations During Zone Transitions

**As a** game developer (P-15), **I want** to cancel in-flight I/O operations via cancellation
tokens with guaranteed completion callbacks, **so that** the asset system cancels stale loads when
the player moves away and resources are cleaned up deterministically.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Cancellation token passed at submission time | F-1.8.8 | R-1.8.8 |
| Completion callback fires with cancellation status | F-1.8.8 | R-1.8.8 |
| Resources cleaned up deterministically on cancel | F-1.8.8 | R-1.8.8 |

## US-1.8.18 Verify Cancellation Callback Fires Deterministically

**As an** engine tester (P-27), **I want** to verify that cancelling an in-flight I/O operation
always fires the completion callback with cancellation status, **so that** no resource leaks occur
from cancelled operations.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Completion callback always fires (success or cancel) | F-1.8.8 | R-1.8.8 |
| Cancelled status distinguishable from success/error | F-1.8.8 | R-1.8.8 |
| No resource leaks from cancelled operations | F-1.8.8 | R-1.8.8 |

## Buffer Management

## US-1.8.19 Register Buffer Pools for Zero-Copy I/O

**As an** engine developer (P-26), **I want** pre-allocated page-aligned I/O buffer pools registered
with the platform backend for zero-copy transfers, **so that** high-throughput asset streaming
avoids per-operation buffer setup costs and memory copies.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Pre-allocated, page-aligned buffer pools | F-1.8.9 | R-1.8.9 |
| Registered with platform backend for kernel pinning | F-1.8.9 | R-1.8.9 |
| Generational handle tracking for buffer lifetime | F-1.8.9 | R-1.8.9 |
| Buffers reclaimed to free list on completion | F-1.8.9 | R-1.8.9 |

## US-1.8.20 Verify Buffer Pool Lifetime Management

**As an** engine tester (P-27), **I want** to verify that buffer pool handles are correctly
reclaimed after I/O completion and that stale handles are rejected, **so that** no buffer leaks or
double-use occurs during sustained streaming.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Buffers returned to pool after completion | F-1.8.9 | R-1.8.9 |
| Stale generational handles rejected | F-1.8.9 | R-1.8.9 |
| No buffer leaks after 10,000 I/O operations | F-1.8.9 | R-1.8.9 |
