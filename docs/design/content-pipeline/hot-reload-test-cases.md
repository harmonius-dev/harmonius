# Hot Reload Test Cases

Companion test cases for [hot-reload.md](hot-reload.md).

## Unit Tests

### TC-12.4.1.1 Change Detector Filters False Positive

| # | Requirement |
|---|-------------|
| 1 | R-12.4.1    |

1. **#1** — Touch file without changing content (same BLAKE3 hash)
   - **Expected:** No reload request generated

### TC-12.4.2.1 Change Detector Resolves Dependents

| # | Requirement |
|---|-------------|
| 1 | R-12.4.2    |

1. **#1** — Change texture T1 referenced by materials M1, M2, M3
   - **Expected:** Reload request dependents list = {M1, M2, M3}

### TC-12.4.2.2 Handle Table Swap Preserves Handle

| # | Requirement |
|---|-------------|
| 1 | R-12.4.2    |
| 2 | R-12.4.2    |

1. **#1** — Allocate handle H1 pointing to data D1
   - **Expected:** H1 resolves to D1
2. **#2** — Swap H1 pointer to data D2
   - **Expected:** H1 resolves to D2

### TC-12.4.2.3 Handle Table Stale Generation

| # | Requirement |
|---|-------------|
| 1 | R-12.4.2    |
| 2 | R-12.4.2    |

1. **#1** — Allocate handle H1 (gen=1), retire H1
   - **Expected:** H1 is retired
2. **#2** — Allocate new handle H2 at same index (gen=2)
   - **Expected:** H1 resolves to None; H2 resolves to new data

### TC-12.4.2.4 Swap Scheduler Applies at Boundary

| # | Requirement |
|---|-------------|
| 1 | R-12.4.2    |
| 2 | R-12.4.2    |

1. **#1** — Schedule swap S1
   - **Expected:** S1 not applied (still pending)
2. **#2** — Call apply_pending_swaps()
   - **Expected:** S1 applied; handle resolves to new data

### TC-12.4.2.5 Swap Scheduler Retires After Fence

| # | Requirement |
|---|-------------|
| 1 | R-12.4.2    |
| 2 | R-12.4.2    |
| 3 | R-12.4.2    |

1. **#1** — Schedule swap at frame 10, apply it
   - **Expected:** Old data not freed
2. **#2** — Call retire_old_assets(fence=9)
   - **Expected:** Old data not freed
3. **#3** — Call retire_old_assets(fence=10)
   - **Expected:** Old data freed

### TC-12.4.3.1 Shader Reloader Identifies Permutations

| # | Requirement |
|---|-------------|
| 1 | R-12.4.3    |
| 2 | R-12.4.3    |

1. **#1** — Register 8 permutations for shader S1
   - **Expected:** Ok
2. **#2** — Query affected_permutations(S1)
   - **Expected:** Returns all 8 permutations

### TC-12.4.3.2 Shader Reloader Error Preserves Old

| # | Requirement |
|---|-------------|
| 1 | R-12.4.3    |

1. **#1** — Compile shader with syntax error
   - **Expected:** Old PSO remains active; error list non-empty

### TC-12.4.4.1 Logic Graph Compatible Layout

| # | Requirement |
|---|-------------|
| 1 | R-12.4.4    |

1. **#1** — Reload graph with same variable layout
   - **Expected:** State preserved; patched_count > 0; restarted_count == 0

### TC-12.4.4.2 Logic Graph Incompatible Layout

| # | Requirement |
|---|-------------|
| 1 | R-12.4.4    |

1. **#1** — Reload graph with changed variable layout
   - **Expected:** Instances restarted; restarted_count > 0

### TC-12.4.5.1 UI Reloader Preserves Scroll

| # | Requirement |
|---|-------------|
| 1 | R-12.4.5    |

1. **#1** — Set scroll position to (0, 250); reload UI
   - **Expected:** Scroll position == (0, 250) after reload

### TC-12.4.5.2 UI Reloader Preserves Focus

| # | Requirement |
|---|-------------|
| 1 | R-12.4.5    |

1. **#1** — Set focus on widget W3; reload UI
   - **Expected:** Focus restored to W3 after reload

### TC-12.4.6.1 Partial Reimport Single Clip

| # | Requirement |
|---|-------------|
| 1 | R-12.4.6    |

1. **#1** — Modify 1 of 10 animation clips in a file
   - **Expected:** Only 1 sub-asset reimported; 9 unchanged

### TC-12.4.1.2 Debounce Coalesces Rapid Events

| # | Requirement |
|---|-------------|
| 1 | R-12.4.1    |

1. **#1** — Write to same file 5 times in 20 ms
   - **Expected:** Exactly 1 reload request generated

### TC-12.4.7.1 Editor Sync Property Roundtrip

| # | Requirement |
|---|-------------|
| 1 | R-12.4.7    |

1. **#1** — Send PropertyChanged{entity=E1, field="color", value=Red} from editor
   - **Expected:** Runtime entity E1 color == Red

## Integration Tests

### TC-12.4.2.I1 Texture Hot Reload End-to-End

| # | Requirement |
|---|-------------|
| 1 | R-12.4.2    |

1. **#1** — Load scene with textured mesh; modify source texture on disk
   - **Expected:** Runtime texture updates within 2 s; no visual artifacts between frames

### TC-12.4.2.I2 Mesh Hot Reload End-to-End

| # | Requirement |
|---|-------------|
| 1 | R-12.4.2    |

1. **#1** — Modify mesh source file on disk
   - **Expected:** Runtime mesh updates within 3 s; no visual glitches during swap

### TC-12.4.3.I1 Shader Hot Reload Valid Change

| # | Requirement |
|---|-------------|
| 1 | R-12.4.3    |

1. **#1** — Modify shader with valid change
   - **Expected:** PSO updates within one frame boundary

### TC-12.4.3.I2 Shader Hot Reload Error Overlay

| # | Requirement |
|---|-------------|
| 1 | R-12.4.3    |

1. **#1** — Introduce shader syntax error
   - **Expected:** Error overlay appears; previous shader continues rendering

### TC-12.4.4.I1 Logic Graph Reload Latency

| # | Requirement |
|---|-------------|
| 1 | R-12.4.4    |

1. **#1** — Modify logic graph on disk
   - **Expected:** Hot reload completes within 500 ms; state preserved

### TC-12.4.5.I1 UI Reload Full State Preservation

| # | Requirement |
|---|-------------|
| 1 | R-12.4.5    |

1. **#1** — Reload UI with active scroll, focus, and animation
   - **Expected:** Scroll, focus, and animation all preserved

### TC-12.4.6.I1 Partial Reimport Latency

| # | Requirement |
|---|-------------|
| 1 | R-12.4.6    |

1. **#1** — Partial reimport 1 of 10 clips
   - **Expected:** Latency < 20% of full reimport time

### TC-12.4.7.I1 Editor-Runtime Sync Latency

| # | Requirement |
|---|-------------|
| 1 | R-12.4.7    |

1. **#1** — Change material parameter in editor
   - **Expected:** Runtime reflects change within 100 ms

### TC-12.4.7.I2 Multi-Device Sync

| # | Requirement |
|---|-------------|
| 1 | R-12.4.7    |

1. **#1** — Connect 3 runtime instances; push change from editor
   - **Expected:** All 3 instances receive synchronized change

### TC-12.4.2.I3 Hot Reload No Memory Leak

| # | Requirement |
|---|-------------|
| 1 | R-12.4.2    |

1. **#1** — Hot reload textures, meshes, shaders, logic graphs 100 times each
   - **Expected:** CPU and GPU memory usage does not grow unboundedly

### TC-12.4.1.I1 Buffer Overflow Recovery

| # | Requirement |
|---|-------------|
| 1 | R-12.4.1    |

1. **#1** — Write 10,000 files rapidly to trigger buffer overflow
   - **Expected:** Watcher recovers via full scan; no changes missed

### TC-12.4.2.I4 DCC Live Link Reload

| # | Requirement |
|---|-------------|
| 1 | R-12.4.2    |

1. **#1** — Push change through DCC plugin live link
   - **Expected:** Engine hot reloads the asset correctly

### TC-12.4.1.I2 Platform Watcher Latency

| # | Requirement |
|---|-------------|
| 1 | R-12.4.1    |

1. **#1** — Write file; measure time to event dispatch
   - **Expected:** Under 500 ms on all platforms

## Benchmarks

### TC-12.4.2.B1 Texture Hot Reload Latency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Modify texture on disk to runtime update | Wall time | < 2 s | R-12.4.2 |

### TC-12.4.2.B2 Mesh Hot Reload Latency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Modify mesh on disk to runtime update | Wall time | < 3 s | R-12.4.2 |

### TC-12.4.3.B1 Shader Hot Reload Latency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Modify shader source to PSO update | Wall time | < 5 s | R-12.4.3 |

### TC-12.4.4.B1 Logic Graph Hot Reload Latency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Modify logic graph to state-preserved reload | Wall time | < 500 ms | R-12.4.4 |

### TC-12.4.5.B1 UI Hot Reload Latency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Modify UI definition to state-preserved reload | Wall time | < 500 ms | R-12.4.5 |

### TC-12.4.1.B1 File Change Detection Latency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | File write to debounced event dispatch | Wall time | < 500 ms | R-12.4.1 |

### TC-12.4.7.B1 Editor-Runtime Sync Latency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Editor property change to runtime apply | Wall time | < 100 ms | R-12.4.7 |

### TC-12.4.2.B3 Handle Table Resolve

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Resolve single handle in table | Wall time | < 10 ns | R-12.4.2 |

### TC-12.4.2.B4 Handle Table Swap

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Swap single handle pointer | Wall time | < 50 ns | R-12.4.2 |

### TC-12.4.2.B5 Memory Growth After Repeated Reloads

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1000 hot reload cycles | Net memory growth | 0 bytes | R-12.4.2 |
