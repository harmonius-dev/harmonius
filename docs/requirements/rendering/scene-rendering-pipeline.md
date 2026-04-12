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

## Render Layers

7. **R-2.10.7** — The engine **SHALL** filter renderable entities, cameras, and lights using a
   32-bit render layer bitmask where an entity is visible to a camera only when their masks overlap.
   - **Rationale:** Render layers enable selective visibility for split-screen, minimap, editor
     overlays, and debug views without duplicating scene data.
   - **Verification:** Set entity layer to bit 2 and camera layer to bit 2. Assert entity is
     visible. Change camera layer to bit 3. Assert entity is invisible. Verify light-entity
     filtering follows the same bitmask rule.

## Transform Interpolation

8. **R-2.10.8** — The engine **SHALL** interpolate extracted transforms using an alpha factor for
   smooth rendering when simulation and render rates differ.
   - **Rationale:** Fixed-timestep simulation produces discrete positions; interpolation eliminates
     visual stutter at non-matching frame rates.
   - **Verification:** Run simulation at 30 Hz and rendering at 60 Hz. Assert interpolated positions
     produce smooth motion with no visible stutter.

## Per-Frame Ring Buffering

9. **R-2.10.9** — The engine **SHALL** ring-buffer per-frame GPU resources (instance, uniform,
   indirect argument buffers) indexed by frame-in-flight to prevent CPU/GPU synchronization stalls.
   - **Rationale:** Ring buffering allows the CPU to write the next frame's data while the GPU
     consumes the current frame's data.
   - **Verification:** Profile with 3 frames in flight. Assert zero CPU wait on GPU fence per frame.
     Assert buffer index advances modulo frame count.

## Probes

10. **R-2.4.24** — The engine **SHALL** provide light probes (SH irradiance) and reflection probes
    (cubemap with influence and blend volumes) as ECS entities with configurable refresh modes
    (baked, on-load, periodic, on-change).
    - **Rationale:** Discrete probe entities provide localized indirect lighting and reflections
      without global GI, essential for interiors and enclosed spaces.
    - **Verification:** Place a light probe and reflection probe. Assert SH irradiance is sampled by
      nearby surfaces. Assert cubemap reflections appear within the influence volume. Change refresh
      mode to periodic and assert updates.

## Sort Key

11. **R-2.10.4a** — The engine **SHALL** encode draw list sort keys in a 64-bit integer with fields
    for translucency, render phase, pipeline, material, and quantized depth, enabling single-pass
    radix sort.
    - **Rationale:** A compact 64-bit sort key enables cache-friendly radix sorting of draw lists
      without multi-key comparisons.
    - **Verification:** Render 10000 draws. Assert sort key layout matches the documented bit
      allocation. Assert radix sort produces correct draw order minimizing state changes.

## Mesh Shader Dispatch

12. **R-2.3.14** — The engine **SHALL** dispatch surviving meshlets via mesh shader groups alongside
    an indirect draw fallback path for non-mesh-shader hardware.
    - **Rationale:** Mesh shaders eliminate vertex/index buffer overhead on modern hardware while
      the fallback ensures compatibility.
    - **Verification:** Render a meshlet scene on mesh-shader hardware. Assert mesh shader dispatch
      is used. Render on non-mesh-shader hardware. Assert indirect draw fallback produces identical
      output.
