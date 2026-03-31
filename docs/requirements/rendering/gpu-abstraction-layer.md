# R-2.1 -- GPU Abstraction Layer Requirements

## Backend Trait and Dispatch

1. **R-2.1.1** — The engine **SHALL** define a top-level Rust trait with associated types for
   device, command buffer, pipeline state, and resource handles, using generics for static dispatch
   with zero vtable overhead.
   - **Rationale:** Static dispatch eliminates virtual call overhead in the hot rendering path while
     providing a uniform API surface across backends.
   - **Verification:** Compile a test binary instantiating the trait with each backend and verify
     the generated assembly contains no indirect call instructions at trait call sites.

2. **R-2.1.2** — The engine **SHALL** provide a command buffer abstraction supporting graphics,
   compute, and copy operations with type-safe resource binding, where mismatched resource types
   produce compile-time errors.
   - **Rationale:** Type-safe binding catches resource misuse at compile time rather than at
     runtime, eliminating a class of GPU crash bugs.
   - **Verification:** Record a command buffer with one graphics, one compute, and one copy
     operation on each backend. Verify successful submission via fence signal. Confirm that binding
     a wrong resource type fails at compile time.

3. **R-2.1.3** — The engine **SHALL** provide a unified pipeline state object validated at creation
   time covering shader stages, vertex layout, blend state, depth-stencil, and rasterizer settings,
   so that encoding incurs no validation overhead.
   - **Rationale:** Pre-validation moves error detection to load time and ensures zero-cost encoding
     during frame submission.
   - **Verification:** Create PSOs with valid and invalid configurations. Verify invalid
     combinations return errors at creation time. Measure that PSO bind during encoding adds zero
     conditional branches.

## Platform Backends

4. **R-2.1.4** — The engine **SHALL** implement the Metal backend as a Swift library with
   swift-bridge FFI, with no Objective-C or C++ in the FFI boundary.
   - **Rationale:** Swift provides first-class Metal API access; swift-bridge produces a stable C
     ABI consumable by Rust directly.
   - **Verification:** Build the Metal backend on macOS and iOS. Verify the FFI boundary contains
     only C-compatible signatures. Run the GPU conformance suite.

5. **R-2.1.5** — The engine **SHALL** implement the D3D12 backend in pure Rust using COM bindings
   with safe wrappers managing reference counting, with no C++ dependency.
   - **Rationale:** Pure Rust COM bindings eliminate all C++ from the Windows backend.
   - **Verification:** Build on Windows. Verify no C++ translation units appear. Run the conformance
     suite. Verify COM reference counts reach zero on shutdown.

6. **R-2.1.6** — The engine **SHALL** implement the Vulkan backend using the ash crate with RAII
   lifetime management and validation layer integration in debug builds.
   - **Rationale:** Ash provides zero-overhead bindings. RAII prevents resource leaks. Validation
     layers catch errors during development.
   - **Verification:** Build on Windows and Linux. Run the conformance suite with validation layers
     enabled and verify zero validation errors.

## GPU Memory and State

7. **R-2.1.7** — The engine **SHALL** provide a GPU heap sub-allocator reducing OS-level allocations
   to fewer than 64 per frame, respecting per-backend alignment (256 B on D3D12, variable on Vulkan,
   page-aligned on Metal).
   - **Rationale:** Sub-allocation from large heaps reduces costly OS-level GPU memory allocations.
   - **Verification:** Render a 10,000+ draw scene. Count OS allocations per frame and verify fewer
     than 64. Verify alignment per backend.

8. **R-2.1.8** — The engine **SHALL** maintain CPU-side shadow state per command buffer, filtering
   redundant pipeline and binding transitions to reduce driver overhead by at least 20% in 1,000+
   draw scenes.
   - **Rationale:** Redundant state filtering avoids unnecessary driver validation and GPU pipeline
     flushes.
   - **Verification:** Record a command buffer setting the same pipeline twice. Verify only one bind
     reaches the driver. Measure at least 20% API call reduction.

9. **R-2.1.9** — The engine **SHALL** automatically batch, merge, and reorder resource barriers
   within command buffers, inserting split barriers when transitions can overlap with independent
   work.
   - **Rationale:** Barrier merging and splitting reduce GPU pipeline stalls.
   - **Verification:** Record three consecutive barriers on the same resource. Verify one merged
     barrier call. On Metal, verify barrier calls are elided.

10. **R-2.1.10** — The engine **SHALL** support GPU work graphs for GPU-driven dispatch, using
    native D3D12 API and compute emulation on Metal and Vulkan.
    - **Rationale:** Work graphs eliminate CPU-GPU sync for GPU-driven pipelines.
    - **Verification:** Execute a work graph on D3D12 and via emulation on Metal/Vulkan. Verify
      output matches within floating-point tolerance.

## Feature Emulation and Profiling

11. **R-2.1.11** — The engine **SHALL** provide a feature emulation layer selecting emulated paths
    at device creation time with no runtime branching, covering work graphs, mesh shaders, and
    enhanced barriers.
    - **Rationale:** Emulation ensures consistent rendering without per-frame capability checks.
    - **Verification:** Create a device lacking mesh shader support. Verify emulation is selected at
      creation. Render a reference scene and verify output matches native within 40 dB PSNR.

12. **R-2.1.12** — The engine **SHALL** expose per-pass GPU timestamps, pipeline statistics, and
    occupancy counters with results read back one frame later to avoid stalls.
    - **Rationale:** Deferred readback provides accurate per-pass data without GPU stalls.
    - **Verification:** Bracket five passes with timestamps. Read results next frame. Verify
      non-zero monotonic timestamps and non-zero invocation counts.

## Resource Safety

13. **R-2.1.13** — The engine **SHALL** reference all GPU resources through generational Handle<T>
    indices that detect use-after-free at runtime, with no raw GPU pointers in any public API type.
    - **Rationale:** Generational handles make memory bugs from stale references impossible in the
      public API.
    - **Verification:** Free a resource and attempt to use its handle. Verify a structured error is
      returned. Attempt to pass Handle<Buffer> as Handle<Texture> and verify compile failure.

## Non-Functional Requirements

14. **R-2.1.14** — The abstraction layer **SHALL** introduce no more than 5% CPU overhead compared
    to raw backend API calls in a 10,000+ draw benchmark.
    - **Rationale:** The abstraction must not be a performance bottleneck.
    - **Verification:** Profile the benchmark with and without the abstraction. Confirm delta below
      5%.

15. **R-2.1.15** — The sub-allocator **SHALL** complete operations in O(1) amortized time and the
    state tracker **SHALL** consume no more than 64 KB per command buffer.
    - **Rationale:** Allocation latency and memory overhead must be bounded for consistent frame
      pacing.
    - **Verification:** Measure sub-allocation latency. Measure state tracker memory per command
      buffer.
