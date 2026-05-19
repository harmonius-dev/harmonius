# R-2.1 -- GPU Abstraction Layer Requirements

## Backend Trait and Dispatch

1. **R-2.1.1** — The engine **SHALL** define a top-level Rust trait with associated types for
   device, command buffer, pipeline state, and resource handles, using generics for static dispatch
   with zero vtable overhead.
   - **Rationale:** Static dispatch eliminates virtual call overhead in the hot rendering path while
     providing a uniform API surface for the Vulkan backend.
   - **Verification:** Compile a test binary instantiating the trait with the Vulkan backend and
     verify the generated assembly contains no indirect call instructions at trait call sites.

2. **R-2.1.2** — The engine **SHALL** provide a command buffer abstraction supporting graphics,
   compute, and copy operations with type-safe resource binding, where mismatched resource types
   produce compile-time errors.
   - **Rationale:** Type-safe binding catches resource misuse at compile time rather than at
     runtime, eliminating a class of GPU crash bugs.
   - **Verification:** Record a command buffer with one graphics, one compute, and one copy
     operation. Verify successful submission via fence signal. Confirm that binding a wrong
     resource type fails at compile time.

3. **R-2.1.3** — The engine **SHALL** provide a unified pipeline state object validated at creation
   time covering shader stages, vertex layout, blend state, depth-stencil, and rasterizer settings,
   so that encoding incurs no validation overhead.
   - **Rationale:** Pre-validation moves error detection to load time and ensures zero-cost encoding
     during frame submission.
   - **Verification:** Create PSOs with valid and invalid configurations. Verify invalid
     combinations return errors at creation time. Measure that PSO bind during encoding adds zero
     conditional branches.

## Vulkan Backend

4. **R-2.1.4** — The engine **SHALL** implement the GPU backend in Rust using the `ash` crate on
   every supported platform (Windows, macOS, Linux, iOS, Android), with no C++ or Swift in the FFI
   boundary.
   - **Rationale:** A single Vulkan backend via `ash` keeps all GPU code in Rust and eliminates
     multi-API maintenance.
   - **Verification:** Build the Vulkan backend on each target OS. Run the GPU conformance suite
     with validation layers enabled in debug builds.

## GPU Memory and State

5. **R-2.1.7** — The engine **SHALL** provide a GPU heap sub-allocator reducing OS-level allocations
   to fewer than 64 per frame, respecting Vulkan alignment requirements
   (`minUniformBufferOffsetAlignment`, `minStorageBufferOffsetAlignment`, and mapped-memory
   alignment).
   - **Rationale:** Sub-allocation from large heaps reduces costly OS-level GPU memory allocations.
   - **Verification:** Render a 10,000+ draw scene. Count OS allocations per frame and verify fewer
     than 64. Verify alignment matches device limits.

6. **R-2.1.8** — The engine **SHALL** maintain CPU-side shadow state per command buffer, filtering
   redundant pipeline and binding transitions to reduce driver overhead by at least 20% in 1,000+
   draw scenes.
   - **Rationale:** Redundant state filtering avoids unnecessary driver validation and GPU pipeline
     flushes.
   - **Verification:** Record a command buffer setting the same pipeline twice. Verify only one bind
     reaches the driver. Measure at least 20% API call reduction.

7. **R-2.1.9** — The engine **SHALL** automatically batch, merge, and reorder resource barriers
   within command buffers, inserting split barriers when transitions can overlap with independent
   work.
   - **Rationale:** Barrier merging and splitting reduce GPU pipeline stalls.
   - **Verification:** Record three consecutive barriers on the same resource. Verify one merged
     `vkCmdPipelineBarrier2` call.

8. **R-2.1.10** — The engine **SHALL** support GPU work graphs for GPU-driven dispatch, using native
   Vulkan extensions when available and compute emulation otherwise.
   - **Rationale:** Work graphs eliminate CPU-GPU sync for GPU-driven pipelines.
   - **Verification:** Execute a work graph natively and via emulation. Verify output matches within
     floating-point tolerance.

## Feature Emulation and Profiling

9. **R-2.1.11** — The engine **SHALL** provide a feature emulation layer selecting emulated paths at
   device creation time with no runtime branching, covering work graphs, mesh shaders, and enhanced
   barriers.
   - **Rationale:** Emulation ensures consistent rendering without per-frame capability checks.
   - **Verification:** Create a device lacking mesh shader support. Verify emulation is selected at
     creation. Render a reference scene and verify output matches native within 40 dB PSNR.

10. **R-2.1.12** — The engine **SHALL** expose GPU timestamp and pipeline-statistics queries through
    the abstraction layer, with results read back one frame later to avoid stalls.
    - **Rationale:** Deferred readback prevents profiling from inserting pipeline bubbles.
    - **Verification:** Bracket a pass with `vkCmdWriteTimestamp`. Verify results are available on
      the following frame without blocking the CPU.

## Handles and Shader Compilation

11. **R-2.1.13** — The engine **SHALL** emulate GPU work graphs on the CPU task graph when native
    Vulkan work graphs are unavailable.
    - **Rationale:** Consistent authoring model across hardware generations.
    - **Verification:** Run the same work graph on native and emulated paths; verify identical draw
      counts.

12. **R-2.1.14** — The engine **SHALL** reference all GPU resources through generational
    `Handle<T>` indices with no raw GPU pointers in public APIs.
    - **Rationale:** Generational handles detect use-after-free without undefined behavior.
    - **Verification:** Free a buffer handle and attempt reuse; verify runtime error.

13. **R-2.1.15** — The engine **SHALL** compile GLSL shader sources to SPIR-V via the `glslc` CLI
    during asset processing, caching artifacts by source hash and never invoking compilers at
    runtime in shipping builds.
    - **Rationale:** Offline compilation eliminates runtime hitches and shader-tool dependencies in
      shipped games.
    - **Verification:** Process a shader graph asset; verify SPIR-V is stored in CAS and loads via
      `vkCreateShaderModule`.
