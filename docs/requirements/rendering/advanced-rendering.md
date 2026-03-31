# R-2.5 -- Advanced Rendering Requirements

## Acceleration Structures

1. **R-2.5.1** — The engine **SHALL** build BLAS from meshlet geometry with post-build compaction
   and rebuild or refit the TLAS each frame for dynamic scenes.
   - **Rationale:** Compacted BLAS minimizes VRAM; per-frame TLAS updates ensure dynamic objects are
     traced correctly.
   - **Verification:** Build BLAS for a test mesh. Verify compaction reduces size by at least 30%.
     Move objects and verify TLAS refit produces correct intersections.

## Reflections and GI

2. **R-2.5.2** — The engine **SHALL** provide hybrid reflections combining SSR with RT rays
   controlled by a roughness threshold, with temporal denoising.
   - **Rationale:** Hybrid reflections balance quality and cost across hardware tiers.
   - **Verification:** Render a mirror surface. Verify SSR handles near surfaces and RT fills
     off-screen gaps. Verify denoised output is temporally stable.

3. **R-2.5.3** — The engine **SHALL** provide DDGI probe grids with octahedral atlas textures and
   temporal hysteresis for dynamic diffuse GI, plus voxel-based GI as a rasterization-only fallback.
   - **Rationale:** Multiple GI techniques ensure indirect lighting is available across all hardware
     tiers.
   - **Verification:** Move a colored light near a wall. Verify color bleeding appears within 1
     second. Verify voxel GI produces similar results without RT.

4. **R-2.5.4** — The engine **SHALL** provide surfel-based GI requiring no precomputation, with
   clipmap probes and ray guiding scaling to arbitrary environment sizes.
   - **Rationale:** Surfel GI enables instant iteration on level layout without rebaking.
   - **Verification:** Add geometry to a scene. Verify GI updates within the next frame without
     manual rebake.

## Path Tracing

5. **R-2.5.5** — The engine **SHALL** provide a progressive path tracer supporting all material
   types, volumetrics, and sky atmosphere for ground-truth reference rendering.
   - **Rationale:** A reference renderer validates real-time approximations and enables cinematic
     output.
   - **Verification:** Render a test scene to convergence. Compare against an external reference
     renderer within 1 dB PSNR.

6. **R-2.5.6** — The engine **SHALL** provide tiered real-time path tracing from direct-RT through
   multi-bounce with rasterized GI fallback after the final bounce.
   - **Rationale:** Tiered path tracing allows quality/perf selection without terminating to black.
   - **Verification:** Render with direct-RT tier. Verify GI fallback produces continuous lighting.
     Render with multi-bounce tier. Verify additional bounces are visible.

## RT Optimization

7. **R-2.5.7** — The engine **SHALL** support opacity micromaps, shader execution reordering, and
   neural denoising with conventional NRD fallback.
   - **Rationale:** RT optimizations improve performance and quality on supported hardware while
     falling back gracefully.
   - **Verification:** Enable OMM on vegetation. Verify any-hit invocations are reduced. Enable SER.
     Verify GPU occupancy improves. Enable neural denoiser. Verify output is temporally stable.

## Stochastic Sampling

8. **R-2.5.8** — The engine **SHALL** provide a ReSTIR sampling framework for direct and indirect
   illumination with per-pixel reservoir reuse across frames.
   - **Rationale:** ReSTIR enables efficient sampling of thousands of lights with rapid convergence.
   - **Verification:** Render 5,000 lights via ReSTIR. Verify convergence within 4 frames. Compare
     against brute-force reference within 2 dB PSNR.

## Screen-Space Techniques

9. **R-2.5.9** — The engine **SHALL** provide stochastic SSR importance-sampled from the microfacet
   BRDF with spatiotemporal filtering at half resolution.
   - **Rationale:** Stochastic SSR reproduces specular elongation and roughness variation without
     RT.
   - **Verification:** Render a surface with varying roughness. Verify specular elongation matches
     RT reflections within 5 dB PSNR.

## Neural Techniques

10. **R-2.5.10** — The engine **SHALL** support a neural radiance cache trained at runtime to
    terminate path- traced paths early and RT lens flare simulation.
    - **Rationale:** Neural caching reduces noise without extra ray budget; RT flares replace
      approximations.
    - **Verification:** Enable radiance cache. Verify noise reduction vs uncached. Enable RT flares.
      Verify ghost and halo match a real lens model.
