# Editor Framework Test Cases

Companion test cases for [editor-framework.md](editor-framework.md).

## Unit Tests

### TC-15.1.1.1 Dock Split Horizontal

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `split(panel_A, Horizontal, panel_B, 0.5)` | Root has two children: panel_A (left 50%), panel_B (right 50%) | R-15.1.1 |
| 2 | `split(panel_A, Horizontal, panel_B, 0.3)` | panel_A occupies 30%, panel_B occupies 70% | R-15.1.1 |

### TC-15.1.1.2 Dock Split Vertical

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `split(panel_A, Vertical, panel_B, 0.5)` | panel_A (top 50%), panel_B (bottom 50%) | R-15.1.1 |

### TC-15.1.1.3 Dock Add Tab

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `add_tab(panel_A, panel_B)` | Tab group contains `[panel_A, panel_B]`, `active_tab == 0` | R-15.1.1 |

### TC-15.1.1.4 Dock Float and Redock

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `float(panel_A, [100, 100], [800, 600])` | Returns `Ok(WindowHandle)`, panel removed from tree | R-15.1.1 |
| 2 | `dock(panel_A, panel_B, Left)` | Panel A docked to left of panel B, floating window destroyed | R-15.1.1 |

### TC-15.1.1.5 Dock Close Collapses Split

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Split into A and B, `close(B)` | Split node removed, A occupies full area | R-15.1.1 |

### TC-15.1.1.6 Layout Save Load

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Build tree with 4 panels, `save()`, `load()` | Loaded tree equals saved tree | R-15.1.1 |

### TC-15.1.1.7 Layout Schema Migration

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Load layout JSON from schema v1 | Migration produces valid v2 tree | R-15.1.1 |

### TC-15.1.2.1 Viewport Create Destroy

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `create()` 3 viewports | `count() == 3`, each has unique `ViewportId` | R-15.1.2 |
| 2 | `destroy(viewport_2)` | `count() == 2`, viewport_2 swapchain released | R-15.1.2 |

### TC-15.1.2.2 Viewport Screen to Ray

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Screen center `[960, 540]` on 1920x1080 perspective viewport | Ray direction is camera forward vector | R-15.1.2 |
| 2 | Screen corner `[0, 0]` on 1920x1080 perspective viewport | Ray direction matches top-left frustum edge | R-15.1.2 |

### TC-15.1.2.3 Viewport Camera Modes

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `CameraMode::FreeFly` | Perspective projection matrix | R-15.1.2 |
| 2 | `CameraMode::OrthoTop` | Orthographic projection, looking down -Y | R-15.1.2 |

### TC-15.1.3.1 Undo Single Command

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Execute `SetPropertyCommand(hp: 100 -> 200)`, undo | Entity hp == 100 | R-15.1.3 |
| 2 | Redo after undo | Entity hp == 200 | R-15.1.3 |

### TC-15.1.3.2 Undo Transaction

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Execute transaction of 5 `SetPropertyCommand`, undo once | All 5 properties reverted to original values | R-15.1.3 |

### TC-15.1.3.3 Undo Clears Redo

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Execute A, undo, execute B | `can_redo() == false`, redo stack empty | R-15.1.3 |

### TC-15.1.3.4 Undo Crash Recovery

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Execute 100 commands, `save_history()`, `load_and_replay()` | Final world state matches pre-save state | R-15.1.3 |

### TC-15.1.4.1 Selection Click

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `select([Entity(42)], Replace)` | `items() == [Entity(42)]`, `count() == 1` | R-15.1.4 |

### TC-15.1.4.2 Selection Marquee

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Frustum query returns 10 entities, `select(10_entities, Replace)` | `count() == 10`, all 10 in `items()` | R-15.1.4 |

### TC-15.1.4.3 Selection Additive

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `select([A], Replace)`, `select([B], Add)` | `items() == [A, B]` | R-15.1.4 |

### TC-15.1.4.4 Selection Subtractive

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `select([A, B], Replace)`, `select([B], Subtract)` | `items() == [A]` | R-15.1.4 |

### TC-15.1.4.5 Selection Saved Sets

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Select [A, B], `save_set("lights")`, `clear()`, `restore_set("lights")` | `items() == [A, B]` | R-15.1.4 |

### TC-15.1.4.6 Selection Sub Object

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `select([SubObject { entity: E, element: Vertex(42) }], Replace)` | `items()[0] == SubObject { entity: E, element: Vertex(42) }` | R-15.1.4 |

### TC-15.1.5.1 Gizmo Translate Axis X

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `begin_manipulation(AxisX, ...)`, drag ray along X | `GizmoDelta { translation: [dx, 0.0, 0.0], .. }` | R-15.1.5 |

### TC-15.1.5.2 Gizmo Rotate Snap

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `snap.rotate_increment = 15.0`, rotate 22 degrees | Snapped rotation = 15.0 degrees | R-15.1.5 |
| 2 | `snap.rotate_increment = 15.0`, rotate 38 degrees | Snapped rotation = 30.0 degrees | R-15.1.5 |

### TC-15.1.5.3 Gizmo Scale Uniform

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Uniform scale drag by factor 2.0 | `GizmoDelta { scale: [2.0, 2.0, 2.0] }` | R-15.1.5 |

### TC-15.1.5.4 Gizmo Reference Frames

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `GizmoFrame::World` on entity rotated 45 degrees | Gizmo axes aligned to world XYZ | R-15.1.5 |
| 2 | `GizmoFrame::Local` on entity rotated 45 degrees | Gizmo axes aligned to entity's local axes | R-15.1.5 |

### TC-15.1.6.1 Measurement Distance

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Points `[0,0,0]` and `[10,0,0]` | `MeasurementResult::Distance { meters: 10.0 }` | R-15.1.6 |

### TC-15.1.6.2 Measurement Angle

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Points `[1,0,0]`, `[0,0,0]`, `[0,1,0]` | `MeasurementResult::Angle { degrees: 90.0 }` | R-15.1.6 |

### TC-15.1.6.3 Measurement Bounds AABB

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Entity with mesh extents `[-1,-1,-1]` to `[1,1,1]` | `MeasurementResult::Bounds { min: [-1,-1,-1], max: [1,1,1] }` | R-15.1.6 |

### TC-15.1.7.1 Hotkey Bind Lookup

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `bind(Ctrl+Z, "undo")`, `lookup(Ctrl+Z)` | Returns `Some("undo")` | R-15.1.7 |

### TC-15.1.7.2 Hotkey Conflict Detection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `bind(Ctrl+Z, "undo")`, `bind(Ctrl+Z, "custom_action")` | `conflicts()` returns `[(Ctrl+Z, ["undo", "custom_action"])]` | R-15.1.7 |

### TC-15.1.7.3 Prefs User Override

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Team default `grid_size = 1.0`, `set("grid_size", 0.5)` | `get("grid_size") == 0.5` | R-15.1.7 |
| 2 | `remove_override("grid_size")` | `get("grid_size") == 1.0` | R-15.1.7 |

### TC-15.1.7.4 Prefs Schema Migration

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Load v1 preferences, `migrate(PreferenceVersion(2))` | New v2 keys populated with defaults | R-15.1.7 |

### TC-15.1.7.5 Inspector Generates Slider

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Struct with `#[reflect(range(0.0, 1.0))]` f32 field | Widget tree contains Slider widget with min=0.0, max=1.0 | R-15.1.7 |

### TC-15.1.7.6 Inspector Multi Select

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Entity A `hp = 100`, entity B `hp = 200`, both selected | Inspector shows "Mixed" for hp field | R-15.1.7 |
| 2 | Entity A `hp = 100`, entity B `hp = 100`, both selected | Inspector shows `100` for hp field | R-15.1.7 |

### TC-15.1.8.1 Plugin Register Panel

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Load plugin that calls `register_panel(descriptor)` | Panel appears in panel registry | R-15.1.8 |

### TC-15.1.8.2 Plugin Unload Cleanup

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Unload plugin | Panel, gizmo, and menu entries all removed | R-15.1.8 |

### TC-15.1.8.3 Plugin Hot Reload

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Load plugin v1, hot-reload with v2 | Panel shows v2 behavior | R-15.1.8 |

### TC-15.1.NF.1 Console Filter by Level

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Push 50 Info + 50 Error entries, filter `min_level: Error` | Returns exactly 50 entries, all Error level | F-15.1.1 |

### TC-15.1.NF.2 Console Capacity Eviction

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Capacity 1000, push 1500 entries | `count() == 1000`, oldest 500 evicted | F-15.1.1 |

## Integration Tests

### TC-15.1.3.I1 Full Edit Cycle

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Select entity, translate +10 X via gizmo, undo | Entity at original position | R-15.1.3, R-15.1.4, R-15.1.5 |
| 2 | Redo after undo | Entity at +10 X position | R-15.1.3 |

### TC-15.1.3.I2 Play Mode Snapshot Restore

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Enter play mode, move entity via simulation, stop | Entity position matches pre-play state | R-15.1.3 |

### TC-15.1.1.I1 Layout Profile Switch

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Save profile "A" (4 panels), save profile "B" (2 panels), switch to B | 2 panels visible | R-15.1.1 |
| 2 | Switch back to A | 4 panels visible | R-15.1.1 |

### TC-15.1.2.I1 Multi Viewport Independent

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 3 viewports: FreeFly, OrthoTop, OrthoFront | Each renders with correct projection and orientation | R-15.1.2 |

### TC-15.1.3.I3 Property Change Undo

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Change `hp` from 100 to 200 via inspector, undo | `hp == 100` | R-15.1.3 |

### TC-15.1.1.I2 Float Panel Cross Monitor

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Float panel to second display, save layout, load on single display | Panel repositioned to primary display | R-15.1.1 |

### TC-15.1.3.I4 Event Bridge Sync

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Editor command sets `hp = 200` on entity | Game world query for entity hp returns 200 | R-15.1.3 |

### TC-15.1.8.I1 Plugin Isolation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Load plugin that panics in `update()` | Editor continues running, plugin unloaded | R-15.1.8 |

### TC-15.1.4.I1 Selection to Inspector

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Select entity A | Inspector shows A's components | R-15.1.4 |
| 2 | Select entity B | Inspector updates to show B's components | R-15.1.4 |

### TC-15.1.1.I3 Platform Floating Panel

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Float panel on current platform | Native window created (NSWindow / CreateWindowEx / xcb) | R-15.1.1 |

## Benchmarks

### TC-15.1.NF1.B1 UI Event Acknowledgement

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Input event to visual feedback, 10k entities loaded | p99 latency | < 16 ms | R-15.1.1 |

### TC-15.1.NF1.B2 Panel Split Tab Operation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Split panel with 8 existing panels | Duration | < 100 ms | R-15.1.1 |

### TC-15.1.3.B1 Undo Single Command

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Undo `TransformCommand` on 100 entities | Duration | < 50 ms | R-15.1.3 |

### TC-15.1.3.B2 Undo 1000 Command Transaction

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Undo transaction containing 1000 `SetPropertyCommand` | Duration | < 200 ms | R-15.1.3 |

### TC-15.1.3.B3 Crash Recovery 10k Commands

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | `load_and_replay` 10,000 commands from disk | Duration | < 10 s | R-15.1.3 |

### TC-15.1.4.B1 Selection Update 10k Entities

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Marquee select 10,000 entities | Duration | < 16 ms | R-15.1.4 |

### TC-15.1.5.B1 Gizmo Hit Test

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | `hit_test(ray)` against translate gizmo | Duration | < 1 ms | R-15.1.5 |

### TC-15.1.7.B1 Inspector Rebuild

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | `build_widgets` for entity with 50 components | Duration | < 16 ms | R-15.1.7 |

### TC-15.1.1.B1 Layout Save Load

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Save and load layout profile with 12 panels and 3 floating | Duration | < 100 ms | R-15.1.1 |

### TC-15.1.NF1.B3 P99 UI Response

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | All UI operations with 10k entities loaded | p99 response | Measured (baseline) | R-15.1.1 |
