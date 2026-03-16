# Hot Reload Test Cases

Companion test cases for [hot-reload.md](hot-reload.md).

## Unit Tests

### TC-12.4.1.1 Change Detector Filters False Positive

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Touch file without changing content (same BLAKE3 hash) | No reload request generated | R-12.4.1 |

### TC-12.4.2.1 Change Detector Resolves Dependents

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Change texture T1 referenced by materials M1, M2, M3 | Reload request dependents list = {M1, M2, M3} | R-12.4.2 |

### TC-12.4.2.2 Handle Table Swap Preserves Handle

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Allocate handle H1 pointing to data D1 | H1 resolves to D1 | R-12.4.2 |
| 2 | Swap H1 pointer to data D2 | H1 resolves to D2 | R-12.4.2 |

### TC-12.4.2.3 Handle Table Stale Generation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Allocate handle H1 (gen=1), retire H1 | H1 is retired | R-12.4.2 |
| 2 | Allocate new handle H2 at same index (gen=2) | H1 resolves to None; H2 resolves to new data | R-12.4.2 |

### TC-12.4.2.4 Swap Scheduler Applies at Boundary

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Schedule swap S1 | S1 not applied (still pending) | R-12.4.2 |
| 2 | Call apply_pending_swaps() | S1 applied; handle resolves to new data | R-12.4.2 |

### TC-12.4.2.5 Swap Scheduler Retires After Fence

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Schedule swap at frame 10, apply it | Old data not freed | R-12.4.2 |
| 2 | Call retire_old_assets(fence=9) | Old data not freed | R-12.4.2 |
| 3 | Call retire_old_assets(fence=10) | Old data freed | R-12.4.2 |

### TC-12.4.3.1 Shader Reloader Identifies Permutations

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Register 8 permutations for shader S1 | Ok | R-12.4.3 |
| 2 | Query affected_permutations(S1) | Returns all 8 permutations | R-12.4.3 |

### TC-12.4.3.2 Shader Reloader Error Preserves Old

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Compile shader with syntax error | Old PSO remains active; error list non-empty | R-12.4.3 |

### TC-12.4.4.1 Logic Graph Compatible Layout

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Reload graph with same variable layout | State preserved; patched_count > 0; restarted_count == 0 | R-12.4.4 |

### TC-12.4.4.2 Logic Graph Incompatible Layout

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Reload graph with changed variable layout | Instances restarted; restarted_count > 0 | R-12.4.4 |

### TC-12.4.5.1 UI Reloader Preserves Scroll

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Set scroll position to (0, 250); reload UI | Scroll position == (0, 250) after reload | R-12.4.5 |

### TC-12.4.5.2 UI Reloader Preserves Focus

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Set focus on widget W3; reload UI | Focus restored to W3 after reload | R-12.4.5 |

### TC-12.4.6.1 Partial Reimport Single Clip

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Modify 1 of 10 animation clips in a file | Only 1 sub-asset reimported; 9 unchanged | R-12.4.6 |

### TC-12.4.1.2 Debounce Coalesces Rapid Events

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Write to same file 5 times in 20 ms | Exactly 1 reload request generated | R-12.4.1 |

### TC-12.4.7.1 Editor Sync Property Roundtrip

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Send PropertyChanged{entity=E1, field="color", value=Red} from editor | Runtime entity E1 color == Red | R-12.4.7 |

## Integration Tests

### TC-12.4.2.I1 Texture Hot Reload End-to-End

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Load scene with textured mesh; modify source texture on disk | Runtime texture updates within 2 s; no visual artifacts between frames | R-12.4.2 |

### TC-12.4.2.I2 Mesh Hot Reload End-to-End

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Modify mesh source file on disk | Runtime mesh updates within 3 s; no visual glitches during swap | R-12.4.2 |

### TC-12.4.3.I1 Shader Hot Reload Valid Change

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Modify shader with valid change | PSO updates within one frame boundary | R-12.4.3 |

### TC-12.4.3.I2 Shader Hot Reload Error Overlay

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Introduce shader syntax error | Error overlay appears; previous shader continues rendering | R-12.4.3 |

### TC-12.4.4.I1 Logic Graph Reload Latency

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Modify logic graph on disk | Hot reload completes within 500 ms; state preserved | R-12.4.4 |

### TC-12.4.5.I1 UI Reload Full State Preservation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Reload UI with active scroll, focus, and animation | Scroll, focus, and animation all preserved | R-12.4.5 |

### TC-12.4.6.I1 Partial Reimport Latency

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Partial reimport 1 of 10 clips | Latency < 20% of full reimport time | R-12.4.6 |

### TC-12.4.7.I1 Editor-Runtime Sync Latency

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Change material parameter in editor | Runtime reflects change within 100 ms | R-12.4.7 |

### TC-12.4.7.I2 Multi-Device Sync

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Connect 3 runtime instances; push change from editor | All 3 instances receive synchronized change | R-12.4.7 |

### TC-12.4.2.I3 Hot Reload No Memory Leak

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Hot reload textures, meshes, shaders, logic graphs 100 times each | CPU and GPU memory usage does not grow unboundedly | R-12.4.2 |

### TC-12.4.1.I1 Buffer Overflow Recovery

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Write 10,000 files rapidly to trigger buffer overflow | Watcher recovers via full scan; no changes missed | R-12.4.1 |

### TC-12.4.2.I4 DCC Live Link Reload

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Push change through DCC plugin live link | Engine hot reloads the asset correctly | R-12.4.2 |

### TC-12.4.1.I2 Platform Watcher Latency

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Write file; measure time to event dispatch | Under 500 ms on all platforms | R-12.4.1 |

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
