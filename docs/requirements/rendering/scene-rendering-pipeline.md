# R-2.10 -- Scene Rendering Pipeline Requirements

## ECS-to-Renderer Bridge

1. **R-2.10.1** — The engine **SHALL** extract visible ECS entities into a renderer-owned snapshot
   each frame on a dedicated thread using immutable queries, so simulation advances concurrently
   with rendering.
   - **Rationale:** Decoupled extraction enables CPU parallelism between simulation and rendering.
   - **Verification:** Profile a frame. Verify extraction runs on a separate thread. Verify
     simulation systems are not blocked during extraction.

2. **R-2.10.2** — The engine **SHALL** store render proxies in a flat SoA layout holding only
   GPU-needed data, with dirty-flag-driven incremental updates reducing per-frame bandwidth from
   O(N) to O(changed).
   - **Rationale:** SoA layout maximizes cache efficiency; incremental updates minimize bus
     bandwidth at scale.
   - **Verification:** Modify 10 of 10,000 entities. Verify only 10 proxies are re-uploaded. Measure
     bandwidth reduction vs full upload.

## View Setup

3. **R-2.10.3** — The engine **SHALL** register each active view with projection, view matrix,
   viewport rect, and quality tier, supporting simultaneous rendering of split-screen, VR stereo,
   shadow, and probe views from a single extracted snapshot.
   - **Rationale:** Multi-view from one snapshot avoids redundant extraction and shares proxy data.
   - **Verification:** Register 4 views. Verify each produces correct output. Verify VR stereo uses
     single-pass instancing where supported.

## Draw List and Batching

4. **R-2.10.4** — The engine **SHALL** assemble per-view draw lists sorted by material, mesh, and
   render state, with GPU batch compaction producing contiguous indirect draw buffers grouped by
   material.
   - **Rationale:** Sorted draw lists minimize state changes; GPU compaction eliminates per-draw CPU
     overhead.
   - **Verification:** Render 100,000 meshlet instances. Verify draw call count is below 200. Verify
     sort order minimizes state changes via GPU capture.

5. **R-2.10.5** — The engine **SHALL** bind per-draw material parameters via bindless descriptor
   indices in a per-instance buffer, eliminating descriptor set switching.
   - **Rationale:** Bindless binding enables material- agnostic batching at scale.
   - **Verification:** Render 1,000 draws with diverse materials. Verify zero descriptor set
     switches in GPU capture. Verify correct parameters per draw.

## Debug Visualization

6. **R-2.10.6** — The engine **SHALL** provide an immediate- mode debug drawing API and diagnostic
   render modes (depth, normals, roughness, overdraw) stripped from shipping builds via compile-time
   gating.
   - **Rationale:** Debug visualization aids development without shipping-build overhead.
   - **Verification:** Enable debug lines. Verify overlay renders. Build in release mode. Verify
     debug code is absent from the binary.
