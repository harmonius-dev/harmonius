# Rendering ↔ Grids/Volumes Integration Test Cases

> **Scope note.** This file covers the 3D fog-of-war, 3D voxel GI, and 3D SDF-shadow integration
> tests only. 2D/2.5D fog, 2D tilemap overlays, and 2D lighting tests are intentionally out of scope
> and live in the separate 2D grids/overlays integration test file.

## Integration Tests

| ID | Test | Req |
|----|------|-----|
| TC-IR-3.3.1.1 | Fog texture uploads from grid | IR-3.3.1 |
| TC-IR-3.3.1.2 | Fog three-state R8 values correct | IR-3.3.1 |
| TC-IR-3.3.1.3 | Fog shader darkens hidden area | IR-3.3.1 |
| TC-IR-3.3.1.4 | Fog grid reads `GridTransform` ECS component | IR-3.3.1 |
| TC-IR-3.3.2.1 | Voxel GI volume uploads as 3D texture | IR-3.3.2 |
| TC-IR-3.3.2.2 | Voxel GI lights scene with indirect light | IR-3.3.2 |
| TC-IR-3.3.3.1 | SDF shadow ray march produces soft shadow | IR-3.3.3 |
| TC-IR-3.3.3.2 | SDF shadow has no NaN output | IR-3.3.3 |
| TC-IR-3.3.4.1 | Dirty tile partial upload covers only dirty cells | IR-3.3.4 |
| TC-IR-3.3.4.2 | No upload when grid is clean | IR-3.3.4 |
| TC-IR-3.3.4.3 | `DirtyRegionSet::drain_sorted` is deterministic | IR-3.3.4 |
| TC-IR-3.3.5.1 | Tactical overlay renders color-coded decal | IR-3.3.5 |
| TC-IR-3.3.5.2 | Tactical overlay updates with grid change | IR-3.3.5 |
| TC-IR-3.3.X.1 | MPSC worker->render upload boundary | IR-3.3.1 |
| TC-IR-3.3.X.2 | Generational handle stale read returns `None` | IR-3.3.4 |

### Integration Test Details

1. **TC-IR-3.3.1.1** -- Input: 256x256 `UniformGrid<FogCell>` with 10 dirty tiles marked. Expected:
   device texture updated at each dirty tile's world-space coordinates.
2. **TC-IR-3.3.1.2** -- Input: cells with values hidden, explored, visible. Expected: R8 texture
   samples read 0, 128, 255 respectively.
3. **TC-IR-3.3.1.3** -- Input: camera positioned over a hidden region. Expected: rendered pixels in
   the hidden region are darkened per the fog palette.
4. **TC-IR-3.3.1.4** -- Input: fog grid entity with a `GridTransform` component. Expected: shader
   samples at `GridTransform::world_origin`; the GPU struct carries no spatial metadata.
5. **TC-IR-3.3.2.1** -- Input: 64^3 `VoxelVolume<VoxelGiCell>` with GI data. Expected: 3D device
   texture contents exactly match the volume cells.
6. **TC-IR-3.3.2.2** -- Input: emissive voxel in the volume. Expected: nearby surfaces receive
   indirect light from the voxel cone-trace pass.
7. **TC-IR-3.3.3.1** -- Input: SDF volume containing a unit sphere at the center. Expected: a soft
   shadow is rendered behind the sphere by the SDF ray-march pass.
8. **TC-IR-3.3.3.2** -- Input: SDF containing edge-case zero-distance cells. Expected: shadow buffer
   contains no NaN pixels.
9. **TC-IR-3.3.4.1** -- Input: 256x256 grid with exactly one 16x16 dirty tile. Expected: exactly 256
   cells uploaded; all other cells skipped.
10. **TC-IR-3.3.4.2** -- Input: grid untouched for 10 frames. Expected: zero texture uploads across
    all 10 frames.
11. **TC-IR-3.3.4.3** -- Input: tile indices marked in random order. Expected: `drain_sorted`
    returns indices in ascending order for every call.
12. **TC-IR-3.3.5.1** -- Input: tactical grid with cover values set. Expected: a color-coded decal
    appears on the terrain matching the cover palette.
13. **TC-IR-3.3.5.2** -- Input: cell changes from open to cover state. Expected: decal color updates
    on the next rendered frame.
14. **TC-IR-3.3.X.1** -- Input: worker sends a `GridUploadCommand` on `grid_upload_commands`.
    Expected: render thread drains the command on its next Phase 7 pull.
15. **TC-IR-3.3.X.2** -- Input: slot freed and reused in `Arena<DeviceTexture>`. Expected: stale
    `GpuTextureHandle::resolve` returns `None`.

## Negative Tests (Failure Modes)

| ID | Test | Req |
|----|------|-----|
| TC-IR-3.3.N.1 | Upload budget exceeded | IR-3.3.4 |
| TC-IR-3.3.N.2 | 3D texture allocation OOM | IR-3.3.2 |
| TC-IR-3.3.N.3 | Stale SDF volume 1-frame skip | IR-3.3.3 |
| TC-IR-3.3.N.4 | Grid resize at runtime | IR-3.3.4 |
| TC-IR-3.3.N.5 | NaN in SDF data clamped | IR-3.3.3 |
| TC-IR-3.3.N.6 | NaN in SDF data rejected in editor | IR-3.3.3 |

### Negative Test Details

1. **TC-IR-3.3.N.1** -- Input: 10 000 dirty tiles with a 1 ms budget. Expected: excess tiles are
   deferred to the next frame; the current frame upload stays under 1 ms.
2. **TC-IR-3.3.N.2** -- Input: attempt to allocate a 4096^3 volume on a 4 GB GPU. Expected: render
   thread sends `GridUploadStatus::Failed`; ECS emits `GridAllocFailedEvent` and falls back to baked
   irradiance probes for that frame.
3. **TC-IR-3.3.N.3** -- Input: worker skips upload for one frame. Expected: SDF pass samples the
   last `Ready` texture; no crash and no artifacts beyond one frame of lag.
4. **TC-IR-3.3.N.4** -- Input: grid dimensions change from 256 to 512. Expected: worker sends
   `grid_alloc_requests`, receives a new `GpuTextureHandle`, and marks all tiles dirty on the new
   handle. Old handle resolves to `None`.
5. **TC-IR-3.3.N.5** -- Input: a SDF cell contains `f32::NAN`; `SdfClampMode::ClampToMaxDistance` is
   active. Expected: the value is clamped to `max_distance`; the uploaded texture has no NaN values.
6. **TC-IR-3.3.N.6** -- Input: same NaN cell with `SdfClampMode::RejectNaN`. Expected: upload is
   rejected and an asset error is surfaced to the editor.

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-3.3.1.B1 | Fog upload 256x256 full | < 1 ms (NFR-SIM.GV5) | IR-3.3.1 |
| TC-IR-3.3.4.B1 | Dirty upload 100 tiles | < 0.5 ms | IR-3.3.4 |
| TC-IR-3.3.4.B2 | Partial upload 1024x1024 @ 5% dirty | < 1 ms (NFR-SIM.GV5) | IR-3.3.4 |
| TC-IR-3.3.4.B3 | `drain_sorted` over 4096 tiles | < 50 us | IR-3.3.4 |
| TC-IR-3.3.2.B1 | Voxel GI compute pass 128^3 | < 2 ms GPU | IR-3.3.2 |
| TC-IR-3.3.3.B1 | SDF shadow ray-march 1080p | < 1 ms GPU | IR-3.3.3 |
| TC-IR-3.3.X.B1 | MPSC drain of 128 upload commands | < 20 us | IR-3.3.1 |

### Benchmark Details

1. **TC-IR-3.3.4.B2** is the worst-case partial upload benchmark. It uploads a 1024x1024
   `UniformGrid<FogCell>` with 5% of its 16x16 tiles marked dirty (approximately 205 tiles, or 52
   480 cells), on the lowest-spec target platform, and must stay under the NFR-SIM.GV5 1 ms budget.
2. **TC-IR-3.3.1.B1** remains the full-upload benchmark (all tiles dirty on a fresh texture) for the
   cold-start path.

## CI Runner Assignment

All integration tests, negative tests, and benchmarks are CI-runnable. GPU-dependent cases run on
the GPU test runners; CPU-only cases run on the standard CI runners.

| Runner | Test cases |
|--------|-----------|
| GPU runner | TC-IR-3.3.1.3, TC-IR-3.3.2.1, TC-IR-3.3.2.2, TC-IR-3.3.3.1, TC-IR-3.3.3.2 |
| GPU runner (cont.) | TC-IR-3.3.5.1, TC-IR-3.3.5.2, TC-IR-3.3.2.B1, TC-IR-3.3.3.B1 |
| CPU runner | All other TC-IR-3.3.\* cases |
