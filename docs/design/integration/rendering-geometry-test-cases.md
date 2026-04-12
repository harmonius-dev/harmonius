# Rendering ↔ World Geometry Integration Test Cases

All test cases are CI-runnable via `cargo test` (integration and negative) and `cargo bench`
(benchmarks). Every test maps to an IR from [rendering-geometry.md](rendering-geometry.md). Negative
tests are listed explicitly.

## Integration Tests

| ID | Test | Req |
|----|------|-----|
| TC-IR-3.2.1.1 | Meshlet clusters upload to GPU | IR-3.2.1 |
| TC-IR-3.2.1.2 | Frustum cull rejects off-screen clusters | IR-3.2.1 |
| TC-IR-3.2.1.3 | HZB occlusion cull rejects hidden clusters | IR-3.2.1 |
| TC-IR-3.2.1.4 | Backface cull uses baked normal cone | IR-3.2.1 |
| TC-IR-3.2.1.5 | Two-phase HZB handles late-visible clusters | IR-3.2.1 |
| TC-IR-3.2.2.1 | LOD selects coarsest valid cut | IR-3.2.2 |
| TC-IR-3.2.2.2 | LOD transitions smoothly with hysteresis | IR-3.2.2 |
| TC-IR-3.2.3.1 | V-buffer writes correct 64-bit IDs | IR-3.2.3 |
| TC-IR-3.2.3.2 | Material eval reads V-buffer | IR-3.2.3 |
| TC-IR-3.2.4.1 | Terrain clipmap renders all LOD levels | IR-3.2.4 |
| TC-IR-3.2.4.2 | Virtual texture feedback requests pages | IR-3.2.4 |
| TC-IR-3.2.5.1 | Foliage GPU cull produces indirect draws | IR-3.2.5 |
| TC-IR-3.2.6.1 | Ocean FFT pass executes in render graph | IR-3.2.6 |
| TC-IR-3.2.6.2 | Sky atmosphere LUT renders | IR-3.2.6 |
| TC-IR-3.2.6.3 | Volumetric cloud pass registers with deps | IR-3.2.6 |
| TC-IR-3.2.7.1 | Missing page uses lowest-LOD fallback | IR-3.2.7 |
| TC-IR-3.2.7.2 | Page load updates GPU buffer next frame | IR-3.2.7 |
| TC-IR-3.2.7.3 | rkyv mmap loads without deserialization | IR-3.2.7 |
| TC-IR-3.2.8.1 | RenderExtractor snapshots into ProxyStore | IR-3.2.8 |
| TC-IR-3.2.8.2 | ProxyStore feeds GPU instance buffer | IR-3.2.8 |
| TC-IR-3.2.9.1 | HZB shared by meshlet and foliage cull | IR-3.2.9 |
| TC-IR-3.2.9.2 | HZB built after depth pass, read-only | IR-3.2.9 |
| TC-IR-3.2.10.1 | Mobile indirect-draw path produces draws | IR-3.2.10 |
| TC-IR-3.2.10.2 | Mobile 32-bit V-buffer packs IDs correctly | IR-3.2.10 |

### Test case details

1. **TC-IR-3.2.1.1** -- Input: `MeshletDAG` with 1000 clusters. Expected: GPU buffer contains all
   1000 `GpuMeshletCluster` entries byte-identical to source.
2. **TC-IR-3.2.1.2** -- Input: 100 clusters, 50 outside frustum. Expected: 50 clusters survive
   frustum cull; 50 rejected.
3. **TC-IR-3.2.1.3** -- Input: 100 clusters, 30 fully occluded behind a wall. Expected: only 70
   survive HZB cull; 30 rejected as hidden.
4. **TC-IR-3.2.1.4** -- Input: 1 cluster facing +Z, camera looking +Z (cone faces camera). Expected:
   cluster rejected by normal-cone backface cull.
5. **TC-IR-3.2.1.5** -- Input: cluster that fails phase-1 HZB (old depth) but passes phase-2 (new
   depth). Expected: cluster is drawn exactly once in phase 2, not dropped.
6. **TC-IR-3.2.2.1** -- Input: object at 100 m, 1-pixel error threshold. Expected: coarsest DAG
   level whose error is under threshold is selected.
7. **TC-IR-3.2.2.2** -- Input: object moving 50 m -> 200 m over 60 frames. Expected: no visible pop;
   hysteresis threshold prevents oscillation.
8. **TC-IR-3.2.3.1** -- Input: 10 instances, 100 triangles each. Expected: every pixel has a valid
   `(instance_id, triangle_id)` pair decoded from 64-bit V-buffer.
9. **TC-IR-3.2.3.2** -- Input: V-buffer with 3 distinct materials. Expected: correct material
   applied per pixel by deferred material compute pass.
10. **TC-IR-3.2.4.1** -- Input: 4 clipmap levels, camera at center. Expected: all 4 levels rendered;
    no seams between levels.
11. **TC-IR-3.2.4.2** -- Input: camera moves into a new terrain tile. Expected: virtual texture
    feedback buffer lists the new page; streamer loads it.
12. **TC-IR-3.2.5.1** -- Input: 10k foliage instances, half inside frustum+HZB. Expected: ~5k in
    `DrawIndexedIndirect` args.
13. **TC-IR-3.2.6.1** -- Input: `OceanFFT` with 3 cascades. Expected: FFT compute pass completes
    within budget; displacement texture populated.
14. **TC-IR-3.2.6.2** -- Input: default `AtmosphereLut` params. Expected: LUT texture has no NaN and
    matches reference Hillaire LUT within tolerance.
15. **TC-IR-3.2.6.3** -- Input: `VolumetricCloud` pass registered with depth + atmosphere deps.
    Expected: render graph executes pass after deps, never before.
16. **TC-IR-3.2.7.1** -- Input: request for page not yet streamed. Expected: lowest resident LOD
    rendered; no stall, no crash.
17. **TC-IR-3.2.7.2** -- Input: stream 64 KiB page from disk via non-blocking I/O. Expected: GPU
    buffer updated on the next frame after completion is polled.
18. **TC-IR-3.2.7.3** -- Input: rkyv-archived `MeshletDAG` file. Expected: `mmap` opens file;
    `&ArchivedMeshletDAG` view is used without any copy/deserialize step.
19. **TC-IR-3.2.8.1** -- Input: 1000 entities with `Transform` + `MaterialId`. Expected:
    `ProxyStore` contains 1000 transforms and material IDs after Phase 7.
20. **TC-IR-3.2.8.2** -- Input: `ProxyStore` with 1000 proxies. Expected: GPU instance buffer
    matches `ProxyStore` layout 1:1.
21. **TC-IR-3.2.9.1** -- Input: HZB built at start of frame. Expected: meshlet cull and foliage cull
    both read the same `HzbResource` handle within the same frame.
22. **TC-IR-3.2.9.2** -- Input: depth pass followed by HZB build. Expected: HZB mip chain is
    read-only for the rest of the frame; write barrier inserted once.
23. **TC-IR-3.2.10.1** -- Input: device reports no mesh-shader support; 100 clusters. Expected:
    `IndirectExpander` produces `DrawIndexedIndirect` args for all 100.
24. **TC-IR-3.2.10.2** -- Input: 20-bit instance ID + 12-bit triangle ID packed into 32 bits.
    Expected: unpacked IDs match source; no bit collision up to 2^20 instances.

## Negative Tests

| ID | Test | Req |
|----|------|-----|
| TC-IR-3.2.1.N1 | Malformed meshlet DAG rejected at load | IR-3.2.1 |
| TC-IR-3.2.2.N1 | LOD resolve with empty DAG returns empty cut | IR-3.2.2 |
| TC-IR-3.2.3.N1 | V-buffer overflow clamps, does not corrupt | IR-3.2.3 |
| TC-IR-3.2.4.N1 | Missing terrain tile uses neighbor fallback | IR-3.2.4 |
| TC-IR-3.2.5.N1 | Zero foliage instances produces no draws | IR-3.2.5 |
| TC-IR-3.2.6.N1 | Cloud pass with missing depth dep errors | IR-3.2.6 |
| TC-IR-3.2.7.N1 | Corrupt rkyv archive rejected by check_bytes | IR-3.2.7 |
| TC-IR-3.2.7.N2 | I/O error on page load uses fallback LOD | IR-3.2.7 |
| TC-IR-3.2.8.N1 | Phase 7 snapshot with zero entities is safe | IR-3.2.8 |
| TC-IR-3.2.9.N1 | HZB read before build returns clamped zero | IR-3.2.9 |
| TC-IR-3.2.10.N1 | Desktop device rejects force_mobile_path off | IR-3.2.10 |

### Negative test details

1. **TC-IR-3.2.1.N1** -- Input: rkyv file with invalid cluster count. Expected: `check_bytes` fails;
   asset rejected; engine logs error; no panic.
2. **TC-IR-3.2.2.N1** -- Input: `MeshletDAG` with zero clusters. Expected: `LodCut::clusters` is
   empty; GPU cull dispatches no work.
3. **TC-IR-3.2.3.N1** -- Input: 2^20 + 1 instances. Expected: V-buffer write clamps to buffer
   bounds; remaining writes discarded with a profiler counter bump.
4. **TC-IR-3.2.4.N1** -- Input: terrain tile fails to load. Expected: neighbor low-res tile is
   sampled; no hole in ground.
5. **TC-IR-3.2.5.N1** -- Input: empty foliage buffer. Expected: compute dispatch produces zero
   draws; no GPU errors.
6. **TC-IR-3.2.6.N1** -- Input: `VolumetricCloud` pass with unresolved `depth` dep. Expected: render
   graph validation fails at build time with a descriptive error.
7. **TC-IR-3.2.7.N1** -- Input: truncated rkyv archive. Expected: `check_bytes` rejects; asset
   marked failed; fallback placeholder mesh used.
8. **TC-IR-3.2.7.N2** -- Input: disk I/O returns `EIO`. Expected: `MeshletStreamer` marks page
   failed; lowest resident LOD is used; error reported to profiler.
9. **TC-IR-3.2.8.N1** -- Input: zero entities matching extract query. Expected: `ProxyStore` is
   cleared to zero length; no allocation on empty path.
10. **TC-IR-3.2.9.N1** -- Input: meshlet cull scheduled before HZB build. Expected: render graph
    reorders or errors; no read-before-write.
11. **TC-IR-3.2.10.N1** -- Input: desktop device, `debug.force_mobile_path=false`. Expected:
    mesh-shader path used; indirect-draw path not exercised.

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-3.2.1.B1 | GPU cull 100k instances | < 1.0 ms GPU | IR-3.2.1 |
| TC-IR-3.2.1.B2 | HZB build 1080p | < 0.3 ms GPU | IR-3.2.1 |
| TC-IR-3.2.2.B1 | LOD resolve 100k instances | < 0.5 ms CPU | IR-3.2.2 |
| TC-IR-3.2.3.B1 | V-buffer + material eval 1080p | < 2.0 ms GPU | IR-3.2.3 |
| TC-IR-3.2.4.B1 | Terrain clipmap 4 levels draw | < 0.8 ms GPU | IR-3.2.4 |
| TC-IR-3.2.5.B1 | Foliage cull 1M instances | < 1.0 ms GPU | IR-3.2.5 |
| TC-IR-3.2.6.B1 | Ocean FFT 3 cascades | < 1.0 ms GPU | IR-3.2.6 |
| TC-IR-3.2.7.B1 | rkyv mmap page upload 64 KiB | < 0.1 ms CPU | IR-3.2.7 |
| TC-IR-3.2.8.B1 | ProxyStore extract 100k entities | < 0.5 ms CPU | IR-3.2.8 |
| TC-IR-3.2.9.B1 | HZB sample in cull shader | < 0.1 ms GPU | IR-3.2.9 |
| TC-IR-3.2.10.B1 | Mobile indirect draw 50k clusters | < 1.5 ms GPU | IR-3.2.10 |
