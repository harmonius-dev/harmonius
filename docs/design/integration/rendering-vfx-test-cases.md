# Rendering ↔ VFX Integration Test Cases

All tests below are CI-runnable. GPU tests execute against a software adapter (llvmpipe on Linux,
WARP on Windows) so CI does not require a discrete GPU.

## Upstream Requirements Trace

Each integration requirement maps 1:1 to an upstream VFX requirement of the same numeric suffix.
Regressions in any test case below also constitute regressions against the listed upstream IDs.

| IR-ID    | Upstream R-ID |
|----------|---------------|
| IR-3.7.1 | R-3.7.1       |
| IR-3.7.2 | R-3.7.2       |
| IR-3.7.3 | R-3.7.3       |
| IR-3.7.4 | R-3.7.4       |
| IR-3.7.5 | R-3.7.5       |
| IR-3.7.6 | R-3.7.6       |
| IR-3.7.7 | R-3.7.7       |

## Integration Tests

| ID | Test | Req |
|----|------|-----|
| TC-IR-3.7.1.1 | Particle sim compute registers in render graph | IR-3.7.1 |
| TC-IR-3.7.1.2 | Barrier inserted between sim write and draw read | IR-3.7.1 |
| TC-IR-3.7.2.1 | Sprite particles DrawIndirect | IR-3.7.2 |
| TC-IR-3.7.2.2 | Ribbon particles DrawIndirect | IR-3.7.2 |
| TC-IR-3.7.2.3 | Alpha sort back-to-front by camera distance | IR-3.7.2 |
| TC-IR-3.7.3.1 | Fog volume injects density into froxel | IR-3.7.3 |
| TC-IR-3.7.3.2 | Weather particles add froxel scattering | IR-3.7.3 |
| TC-IR-3.7.4.1 | Decal modifies G-buffer albedo | IR-3.7.4 |
| TC-IR-3.7.4.2 | Decal priority layering | IR-3.7.4 |
| TC-IR-3.7.4.3 | Decal angle-threshold fade | IR-3.7.4 |
| TC-IR-3.7.5.1 | Particle lights in cluster grid | IR-3.7.5 |
| TC-IR-3.7.5.2 | Particle lights capped at budget | IR-3.7.5 |
| TC-IR-3.7.6.1 | Heat haze distortion renders | IR-3.7.6 |
| TC-IR-3.7.6.2 | Shockwave expands correctly | IR-3.7.6 |
| TC-IR-3.7.7.1 | LOD reduces at distance | IR-3.7.7 |
| TC-IR-3.7.7.2 | LOD culls at max distance | IR-3.7.7 |
| TC-IR-3.7.7.3 | LOD hysteresis prevents pop | IR-3.7.7 |
| TC-IR-3.7.1.S1 | Phase 7 snapshot handoff to render thread | IR-3.7.1 |
| TC-IR-3.7.1.S2 | VfxGraphCompiler emits pass descriptors | IR-3.7.1 |
| TC-IR-3.7.3.S1 | Mobile froxel CPU screen-tile fallback | IR-3.7.3 |

1. **TC-IR-3.7.1.1** — Input: 1000-particle emitter. Expected: compute pass dispatches, alive count
   matches.
2. **TC-IR-3.7.1.2** — Input: sim writes and draw reads the same GPU buffer. Expected: no data race,
   pixels validate against reference.
3. **TC-IR-3.7.2.1** — Input: 500 billboard particles. Expected: `DrawIndirect` issued with count ==
   500.
4. **TC-IR-3.7.2.2** — Input: ribbon trail emitter. Expected: connected strip geometry spanning the
   emitter path.
5. **TC-IR-3.7.2.3** — Input: 100 transparent particles. Expected: draw order is back-to-front by
   camera distance.
6. **TC-IR-3.7.3.1** — Input: fog volume AABB in view frustum. Expected: froxel cells inside the
   AABB have `density > 0`.
7. **TC-IR-3.7.3.2** — Input: rain weather at intensity 1.0. Expected: froxel scattering channel
   increases versus baseline.
8. **TC-IR-3.7.4.1** — Input: blood decal projected on wall. Expected: G-buffer albedo at decal rect
   differs from baseline.
9. **TC-IR-3.7.4.2** — Input: two decals with priorities 1 and 2. Expected: priority-2 decal renders
   on top.
10. **TC-IR-3.7.4.3** — Input: decal on 80-degree slope, threshold 60. Expected: decal alpha fades
    to zero.
11. **TC-IR-3.7.5.1** — Input: 10 emissive particles. Expected: 10 point lights appear in
    `GpuLightBuffer` and cluster grid.
12. **TC-IR-3.7.5.2** — Input: 1000 particle lights with budget 100. Expected: only the 100
    brightest are injected (min-heap selection).
13. **TC-IR-3.7.6.1** — Input: heat haze source in view. Expected: screen-space distortion pass
    modifies pixels in affected region.
14. **TC-IR-3.7.6.2** — Input: shockwave at origin, t = 0.5. Expected: ring distortion at half
    radius.
15. **TC-IR-3.7.7.1** — Input: emitter at 200 m, `full_distance` = 50 m. Expected:
    `LodTier::Reduced` active.
16. **TC-IR-3.7.7.2** — Input: emitter at 500 m, `cull_distance` = 400 m. Expected:
    `LodTier::Culled`, no render output.
17. **TC-IR-3.7.7.3** — Input: emitter oscillating at a tier boundary. Expected: no rapid tier
    switching within the hysteresis band.
18. **TC-IR-3.7.1.S1** — Input: simulation workers write pass descriptors. Expected: render thread
    reads only the Phase 7 `RenderFrame` snapshot (no shared mutable state).
19. **TC-IR-3.7.1.S2** — Input: `VfxGraph` with 3 nodes. Expected: `VfxGraphCompiler::compile` emits
    3 pass descriptors matching the node kinds.
20. **TC-IR-3.7.3.S1** — Input: mobile capability set, fog volume present. Expected: the CPU
    screen-tile fallback buffer is populated and the forward shader samples it.

## Negative / Failure-Mode Tests

| ID | Failure | Req |
|----|---------|-----|
| TC-IR-3.7.1.F1 | GPU compute queue stall (fence timeout) | IR-3.7.1 |
| TC-IR-3.7.2.F1 | Sort scratch buffer OOM | IR-3.7.2 |
| TC-IR-3.7.3.F1 | Froxel voxel density overflow | IR-3.7.3 |
| TC-IR-3.7.4.F1 | Decal atlas full | IR-3.7.4 |
| TC-IR-3.7.5.F1 | Particle light inject overflow | IR-3.7.5 |
| TC-IR-3.7.7.F1 | Effect budget exceeded | IR-3.7.7 |

1. **TC-IR-3.7.1.F1** — Force a fence timeout during particle simulation dispatch. Expected: render
   graph reissues the dispatch on the graphics queue and the frame completes.
2. **TC-IR-3.7.2.F1** — Allocate a sort scratch buffer smaller than the particle count. Expected:
   sort is skipped, particles render in emission order, warning logged.
3. **TC-IR-3.7.3.F1** — Write density values above the per-voxel cap. Expected: density is clamped,
   frame renders without NaN or artifacts.
4. **TC-IR-3.7.4.F1** — Allocate decals beyond atlas capacity. Expected: LRU eviction frees the
   oldest slot and the new decal fits.
5. **TC-IR-3.7.5.F1** — Inject 500 particle lights with budget 100. Expected: 100 brightest
   retained, 400 dropped, counter increments.
6. **TC-IR-3.7.7.F1** — Spawn 2x cap particles per frame. Expected: `EffectBudget.spawn_rate_scale`
   ramps down on subsequent frames until under cap.

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-3.7.1.B1 | Particle sim 100K | < 1 ms GPU | IR-3.7.1 |
| TC-IR-3.7.2.B1 | Particle draw 100K sprites | < 2 ms GPU | IR-3.7.2 |
| TC-IR-3.7.2.B2 | GPU radix sort 100K | < 0.5 ms GPU | IR-3.7.2 |
| TC-IR-3.7.3.B1 | Froxel injection 10 volumes | < 0.5 ms GPU | IR-3.7.3 |
| TC-IR-3.7.4.B1 | 100 decals on G-buffer | < 0.5 ms GPU | IR-3.7.4 |
| TC-IR-3.7.5.B1 | 100 particle lights inject | < 0.1 ms CPU | IR-3.7.5 |
| TC-IR-3.7.6.B1 | 4 screen effects post-pass | < 0.4 ms GPU | IR-3.7.6 |
| TC-IR-3.7.7.B1 | LOD eval 1000 emitters | < 0.1 ms CPU | IR-3.7.7 |
