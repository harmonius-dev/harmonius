# Editor Framework Test Cases

Companion test cases for [editor-framework.md](editor-framework.md).

## Unit Tests

### TC-15.1.1.1 Dock Split Horizontal

| # | Requirement |
|---|-------------|
| 1 | R-15.1.1    |
| 2 | R-15.1.1    |

1. **#1** — `split(panel_A, Horizontal, panel_B, 0.5)`
   - **Expected:** Root has two children: panel_A (left 50%), panel_B (right 50%)
2. **#2** — `split(panel_A, Horizontal, panel_B, 0.3)`
   - **Expected:** panel_A occupies 30%, panel_B occupies 70%

### TC-15.1.1.2 Dock Split Vertical

| # | Requirement |
|---|-------------|
| 1 | R-15.1.1    |

1. **#1** — `split(panel_A, Vertical, panel_B, 0.5)`
   - **Expected:** panel_A (top 50%), panel_B (bottom 50%)

### TC-15.1.1.3 Dock Add Tab

| # | Requirement |
|---|-------------|
| 1 | R-15.1.1    |

1. **#1** — `add_tab(panel_A, panel_B)`
   - **Expected:** Tab group contains `[panel_A, panel_B]`, `active_tab == 0`

### TC-15.1.1.4 Dock Float and Redock

| # | Requirement |
|---|-------------|
| 1 | R-15.1.1    |
| 2 | R-15.1.1    |

1. **#1** — `float(panel_A, [100, 100], [800, 600])`
   - **Expected:** Returns `Ok(WindowHandle)`, panel removed from tree
2. **#2** — `dock(panel_A, panel_B, Left)`
   - **Expected:** Panel A docked to left of panel B, floating window destroyed

### TC-15.1.1.5 Dock Close Collapses Split

| # | Requirement |
|---|-------------|
| 1 | R-15.1.1    |

1. **#1** — Split into A and B, `close(B)`
   - **Expected:** Split node removed, A occupies full area

### TC-15.1.1.6 Layout Save Load

| # | Requirement |
|---|-------------|
| 1 | R-15.1.1    |

1. **#1** — Build tree with 4 panels, `save()`, `load()`
   - **Expected:** Loaded tree equals saved tree

### TC-15.1.1.7 Layout Schema Migration

| # | Requirement |
|---|-------------|
| 1 | R-15.1.1    |

1. **#1** — Load layout JSON from schema v1
   - **Expected:** Migration produces valid v2 tree

### TC-15.1.2.1 Viewport Create Destroy

| # | Requirement |
|---|-------------|
| 1 | R-15.1.2    |
| 2 | R-15.1.2    |

1. **#1** — `create()` 3 viewports
   - **Expected:** `count() == 3`, each has unique `ViewportId`
2. **#2** — `destroy(viewport_2)`
   - **Expected:** `count() == 2`, viewport_2 swapchain released

### TC-15.1.2.2 Viewport Screen to Ray

| # | Requirement |
|---|-------------|
| 1 | R-15.1.2    |
| 2 | R-15.1.2    |

1. **#1** — Screen center `[960, 540]` on 1920x1080 perspective viewport
   - **Expected:** Ray direction is camera forward vector
2. **#2** — Screen corner `[0, 0]` on 1920x1080 perspective viewport
   - **Expected:** Ray direction matches top-left frustum edge

### TC-15.1.2.3 Viewport Camera Modes

| # | Requirement |
|---|-------------|
| 1 | R-15.1.2    |
| 2 | R-15.1.2    |

1. **#1** — `CameraMode::FreeFly`
   - **Expected:** Perspective projection matrix
2. **#2** — `CameraMode::OrthoTop`
   - **Expected:** Orthographic projection, looking down -Y

### TC-15.1.3.1 Undo Single Command

| # | Requirement |
|---|-------------|
| 1 | R-15.1.3    |
| 2 | R-15.1.3    |

1. **#1** — Execute `SetPropertyCommand(hp: 100 -> 200)`, undo
   - **Expected:** Entity hp == 100
2. **#2** — Redo after undo
   - **Expected:** Entity hp == 200

### TC-15.1.3.2 Undo Transaction

| # | Requirement |
|---|-------------|
| 1 | R-15.1.3    |

1. **#1** — Execute transaction of 5 `SetPropertyCommand`, undo once
   - **Expected:** All 5 properties reverted to original values

### TC-15.1.3.3 Undo Clears Redo

| # | Requirement |
|---|-------------|
| 1 | R-15.1.3    |

1. **#1** — Execute A, undo, execute B
   - **Expected:** `can_redo() == false`, redo stack empty

### TC-15.1.3.4 Undo Crash Recovery

| # | Requirement |
|---|-------------|
| 1 | R-15.1.3    |

1. **#1** — Execute 100 commands, `save_history()`, `load_and_replay()`
   - **Expected:** Final world state matches pre-save state

### TC-15.1.4.1 Selection Click

| # | Requirement |
|---|-------------|
| 1 | R-15.1.4    |

1. **#1** — `select([Entity(42)], Replace)`
   - **Expected:** `items() == [Entity(42)]`, `count() == 1`

### TC-15.1.4.2 Selection Marquee

| # | Requirement |
|---|-------------|
| 1 | R-15.1.4    |

1. **#1** — Frustum query returns 10 entities, `select(10_entities, Replace)`
   - **Expected:** `count() == 10`, all 10 in `items()`

### TC-15.1.4.3 Selection Additive

| # | Requirement |
|---|-------------|
| 1 | R-15.1.4    |

1. **#1** — `select([A], Replace)`, `select([B], Add)`
   - **Expected:** `items() == [A, B]`

### TC-15.1.4.4 Selection Subtractive

| # | Requirement |
|---|-------------|
| 1 | R-15.1.4    |

1. **#1** — `select([A, B], Replace)`, `select([B], Subtract)`
   - **Expected:** `items() == [A]`

### TC-15.1.4.5 Selection Saved Sets

| # | Requirement |
|---|-------------|
| 1 | R-15.1.4    |

1. **#1** — Select [A, B], `save_set("lights")`, `clear()`, `restore_set("lights")`
   - **Expected:** `items() == [A, B]`

### TC-15.1.4.6 Selection Sub Object

| # | Requirement |
|---|-------------|
| 1 | R-15.1.4    |

1. **#1** — `select([SubObject { entity: E, element: Vertex(42) }], Replace)`
   - **Expected:** `items()[0] == SubObject { entity: E, element: Vertex(42) }`

### TC-15.1.5.1 Gizmo Translate Axis X

| # | Requirement |
|---|-------------|
| 1 | R-15.1.5    |

1. **#1** — `begin_manipulation(AxisX, ...)`, drag ray along X
   - **Expected:** `GizmoDelta { translation: [dx, 0.0, 0.0], .. }`

### TC-15.1.5.2 Gizmo Rotate Snap

| # | Requirement |
|---|-------------|
| 1 | R-15.1.5    |
| 2 | R-15.1.5    |

1. **#1** — `snap.rotate_increment = 15.0`, rotate 22 degrees
   - **Expected:** Snapped rotation = 15.0 degrees
2. **#2** — `snap.rotate_increment = 15.0`, rotate 38 degrees
   - **Expected:** Snapped rotation = 30.0 degrees

### TC-15.1.5.3 Gizmo Scale Uniform

| # | Requirement |
|---|-------------|
| 1 | R-15.1.5    |

1. **#1** — Uniform scale drag by factor 2.0
   - **Expected:** `GizmoDelta { scale: [2.0, 2.0, 2.0] }`

### TC-15.1.5.4 Gizmo Reference Frames

| # | Requirement |
|---|-------------|
| 1 | R-15.1.5    |
| 2 | R-15.1.5    |

1. **#1** — `GizmoFrame::World` on entity rotated 45 degrees
   - **Expected:** Gizmo axes aligned to world XYZ
2. **#2** — `GizmoFrame::Local` on entity rotated 45 degrees
   - **Expected:** Gizmo axes aligned to entity's local axes

### TC-15.1.6.1 Measurement Distance

| # | Requirement |
|---|-------------|
| 1 | R-15.1.6    |

1. **#1** — Points `[0,0,0]` and `[10,0,0]`
   - **Expected:** `MeasurementResult::Distance { meters: 10.0 }`

### TC-15.1.6.2 Measurement Angle

| # | Requirement |
|---|-------------|
| 1 | R-15.1.6    |

1. **#1** — Points `[1,0,0]`, `[0,0,0]`, `[0,1,0]`
   - **Expected:** `MeasurementResult::Angle { degrees: 90.0 }`

### TC-15.1.6.3 Measurement Bounds AABB

| # | Requirement |
|---|-------------|
| 1 | R-15.1.6    |

1. **#1** — Entity with mesh extents `[-1,-1,-1]` to `[1,1,1]`
   - **Expected:** `MeasurementResult::Bounds { min: [-1,-1,-1], max: [1,1,1] }`

### TC-15.1.7.1 Hotkey Bind Lookup

| # | Requirement |
|---|-------------|
| 1 | R-15.1.7    |

1. **#1** — `bind(Ctrl+Z, "undo")`, `lookup(Ctrl+Z)`
   - **Expected:** Returns `Some("undo")`

### TC-15.1.7.2 Hotkey Conflict Detection

| # | Requirement |
|---|-------------|
| 1 | R-15.1.7    |

1. **#1** — `bind(Ctrl+Z, "undo")`, `bind(Ctrl+Z, "custom_action")`
   - **Expected:** `conflicts()` returns `[(Ctrl+Z, ["undo", "custom_action"])]`

### TC-15.1.7.3 Prefs User Override

| # | Requirement |
|---|-------------|
| 1 | R-15.1.7    |
| 2 | R-15.1.7    |

1. **#1** — Team default `grid_size = 1.0`, `set("grid_size", 0.5)`
   - **Expected:** `get("grid_size") == 0.5`
2. **#2** — `remove_override("grid_size")`
   - **Expected:** `get("grid_size") == 1.0`

### TC-15.1.7.4 Prefs Schema Migration

| # | Requirement |
|---|-------------|
| 1 | R-15.1.7    |

1. **#1** — Load v1 preferences, `migrate(PreferenceVersion(2))`
   - **Expected:** New v2 keys populated with defaults

### TC-15.1.7.5 Inspector Generates Slider

| # | Requirement |
|---|-------------|
| 1 | R-15.1.7    |

1. **#1** — Struct with `#[reflect(range(0.0, 1.0))]` f32 field
   - **Expected:** Widget tree contains Slider widget with min=0.0, max=1.0

### TC-15.1.7.6 Inspector Multi Select

| # | Requirement |
|---|-------------|
| 1 | R-15.1.7    |
| 2 | R-15.1.7    |

1. **#1** — Entity A `hp = 100`, entity B `hp = 200`, both selected
   - **Expected:** Inspector shows "Mixed" for hp field
2. **#2** — Entity A `hp = 100`, entity B `hp = 100`, both selected
   - **Expected:** Inspector shows `100` for hp field

### TC-15.1.8.1 Plugin Register Panel

| # | Requirement |
|---|-------------|
| 1 | R-15.1.8    |

1. **#1** — Load plugin that calls `register_panel(descriptor)`
   - **Expected:** Panel appears in panel registry

### TC-15.1.8.2 Plugin Unload Cleanup

| # | Requirement |
|---|-------------|
| 1 | R-15.1.8    |

1. **#1** — Unload plugin
   - **Expected:** Panel, gizmo, and menu entries all removed

### TC-15.1.8.3 Plugin Hot Reload

| # | Requirement |
|---|-------------|
| 1 | R-15.1.8    |

1. **#1** — Load plugin v1, hot-reload with v2
   - **Expected:** Panel shows v2 behavior

### TC-15.1.NF.1 Console Filter by Level

| # | Requirement |
|---|-------------|
| 1 | F-15.1.1    |

1. **#1** — Push 50 Info + 50 Error entries, filter `min_level: Error`
   - **Expected:** Returns exactly 50 entries, all Error level

### TC-15.1.NF.2 Console Capacity Eviction

| # | Requirement |
|---|-------------|
| 1 | F-15.1.1    |

1. **#1** — Capacity 1000, push 1500 entries
   - **Expected:** `count() == 1000`, oldest 500 evicted

## Integration Tests

### TC-15.1.3.I1 Full Edit Cycle

| # | Requirement                  |
|---|------------------------------|
| 1 | R-15.1.3, R-15.1.4, R-15.1.5 |
| 2 | R-15.1.3                     |

1. **#1** — Select entity, translate +10 X via gizmo, undo
   - **Expected:** Entity at original position
2. **#2** — Redo after undo
   - **Expected:** Entity at +10 X position

### TC-15.1.3.I2 Play Mode Snapshot Restore

| # | Requirement |
|---|-------------|
| 1 | R-15.1.3    |

1. **#1** — Enter play mode, move entity via simulation, stop
   - **Expected:** Entity position matches pre-play state

### TC-15.1.1.I1 Layout Profile Switch

| # | Requirement |
|---|-------------|
| 1 | R-15.1.1    |
| 2 | R-15.1.1    |

1. **#1** — Save profile "A" (4 panels), save profile "B" (2 panels), switch to B
   - **Expected:** 2 panels visible
2. **#2** — Switch back to A
   - **Expected:** 4 panels visible

### TC-15.1.2.I1 Multi Viewport Independent

| # | Requirement |
|---|-------------|
| 1 | R-15.1.2    |

1. **#1** — 3 viewports: FreeFly, OrthoTop, OrthoFront
   - **Expected:** Each renders with correct projection and orientation

### TC-15.1.3.I3 Property Change Undo

| # | Requirement |
|---|-------------|
| 1 | R-15.1.3    |

1. **#1** — Change `hp` from 100 to 200 via inspector, undo
   - **Expected:** `hp == 100`

### TC-15.1.1.I2 Float Panel Cross Monitor

| # | Requirement |
|---|-------------|
| 1 | R-15.1.1    |

1. **#1** — Float panel to second display, save layout, load on single display
   - **Expected:** Panel repositioned to primary display

### TC-15.1.3.I4 Event Bridge Sync

| # | Requirement |
|---|-------------|
| 1 | R-15.1.3    |

1. **#1** — Editor command sets `hp = 200` on entity
   - **Expected:** Game world query for entity hp returns 200

### TC-15.1.8.I1 Plugin Isolation

| # | Requirement |
|---|-------------|
| 1 | R-15.1.8    |

1. **#1** — Load plugin that panics in `update()`
   - **Expected:** Editor continues running, plugin unloaded

### TC-15.1.4.I1 Selection to Inspector

| # | Requirement |
|---|-------------|
| 1 | R-15.1.4    |
| 2 | R-15.1.4    |

1. **#1** — Select entity A
   - **Expected:** Inspector shows A's components
2. **#2** — Select entity B
   - **Expected:** Inspector updates to show B's components

### TC-15.1.1.I3 Platform Floating Panel

| # | Requirement |
|---|-------------|
| 1 | R-15.1.1    |

1. **#1** — Float panel on current platform
   - **Expected:** Native window created (NSWindow / CreateWindowEx / xcb)

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
