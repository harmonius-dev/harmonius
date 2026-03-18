# R-15.1 -- Editor Framework Requirements

## Dockable Panel Layout

| ID        | Derived From                                                |
|-----------|-------------------------------------------------------------|
| R-15.1.1  | [F-15.1.1](../../features/tools-editor/editor-framework.md) |
| R-15.1.1a | [F-15.1.1](../../features/tools-editor/editor-framework.md) |

1. **R-15.1.1** — The editor **SHALL** provide a dockable panel system supporting drag-and-drop
   rearrangement, horizontal and vertical splitting, tab grouping, and floating panels as
   independent OS windows.
   - **Rationale:** Artists, designers, and programmers each need tailored workspace layouts; a
     rigid layout blocks multi-discipline workflows.
   - **Verification:** Integration test: drag a panel to each dock zone (left, right, top, bottom,
     tab, float), save the layout, restart the editor, and verify all positions and sizes are
     restored within 1 pixel.
2. **R-15.1.1a** — The editor **SHALL** persist named layout profiles as versioned JSON files stored
   in VCS with per-user override files excluded from VCS.
   - **Rationale:** Team-wide defaults must propagate via VCS while personal overrides remain local
     to avoid merge conflicts.
   - **Verification:** Unit test: save a profile, modify it, save a per-user override, load both,
     and verify the override takes precedence. Remove the override and verify the team default
     restores.

## Multi-Viewport

| ID       | Derived From                                                |
|----------|-------------------------------------------------------------|
| R-15.1.2 | [F-15.1.2](../../features/tools-editor/editor-framework.md) |

1. **R-15.1.2** — The editor **SHALL** support at least four simultaneous 3D viewports, each with an
   independent camera, render settings, and debug overlays, rendering through the same render graph
   as the game runtime.
   - **Rationale:** Artists need lit/unlit comparison; designers need top-down and perspective
     simultaneously; multiplayer testing requires split-screen preview.
   - **Verification:** Integration test: open four viewports with different camera transforms and
     render modes. Verify each renders independently and matches the game runtime output via
     screenshot comparison.

## Undo/Redo

| ID        | Derived From                                                |
|-----------|-------------------------------------------------------------|
| R-15.1.3  | [F-15.1.3](../../features/tools-editor/editor-framework.md) |
| R-15.1.3a | [F-15.1.3](../../features/tools-editor/editor-framework.md) |

1. **R-15.1.3** — The editor **SHALL** capture every user action as a serializable, reversible
   command object, supporting grouped transactions that undo atomically and crash-recovery replay
   from a persisted command history.
   - **Rationale:** Accidental changes in a large open-world editor must be reversible; crash
     recovery prevents work loss.
   - **Verification:** Unit test: execute 100 commands, undo all, verify world state matches initial
     state. Simulate crash, replay history, and verify state matches pre-crash state.
2. **R-15.1.3a** — Single-command undo/redo **SHALL** complete within 50 ms. Grouped transactions of
   up to 1,000 sub-commands **SHALL** undo within 200 ms. Crash recovery replay of 10,000 commands
   **SHALL** complete within 10 seconds.
   - **Rationale:** Undo must feel instantaneous; crash recovery must complete before the user loses
     patience.
   - **Verification:** Benchmark: time undo of a single command and a 1,000-command group. Benchmark
     crash recovery replay of 10,000 commands.

## Selection

| ID       | Derived From                                                |
|----------|-------------------------------------------------------------|
| R-15.1.4 | [F-15.1.4](../../features/tools-editor/editor-framework.md) |

1. **R-15.1.4** — The editor **SHALL** provide a unified selection model supporting click, marquee,
   lasso, additive, subtractive, and programmatic selection across entities, components, assets, and
   sub-object elements (vertices, faces, bones), with named selection sets that persist across
   sessions.
   - **Rationale:** Consistent selection behavior across object types reduces cognitive load; named
     sets accelerate repeated batch operations.
   - **Verification:** Integration test: select entities via each mode, save a named set, restart
     the editor, and verify the set restores correctly.

## Transform Gizmos

| ID       | Derived From                                                |
|----------|-------------------------------------------------------------|
| R-15.1.5 | [F-15.1.5](../../features/tools-editor/editor-framework.md) |

1. **R-15.1.5** — The editor **SHALL** provide interactive translate, rotate, and scale gizmos with
   axis and plane constraints, configurable snap increments, and local/world/custom reference
   frames.
   - **Rationale:** Precise entity manipulation is fundamental to level design; snap and constraints
     prevent misaligned placements.
   - **Verification:** Unit test: translate an entity with grid snap enabled and verify the
     resulting position is quantized to the snap increment within epsilon.

## Measurement Gizmos

| ID       | Derived From                                                |
|----------|-------------------------------------------------------------|
| R-15.1.6 | [F-15.1.6](../../features/tools-editor/editor-framework.md) |

1. **R-15.1.6** — The editor **SHALL** provide distance rulers, angle protractors, area measurement
   overlays, and axis-aligned and oriented bounding box displays, accurate within 0.01 world units.
   - **Rationale:** Designers verify gameplay metrics (corridor widths, jump distances) directly in
     the viewport.
   - **Verification:** Unit test: place two points at known coordinates and verify the ruler reports
     the correct Euclidean distance within 0.01 units.

## Preferences

| ID       | Derived From                                                |
|----------|-------------------------------------------------------------|
| R-15.1.7 | [F-15.1.7](../../features/tools-editor/editor-framework.md) |

1. **R-15.1.7** — The editor **SHALL** provide a centralized preference system with input bindings,
   visual themes, viewport defaults, and auto-save intervals, stored as versioned JSON with per-user
   overrides separate from team defaults.
   - **Rationale:** Team-wide consistency with personal customization prevents configuration drift
     while respecting individual workflows.
   - **Verification:** Unit test: set a team default, apply a per-user override, verify the override
     takes effect. Remove the override and verify the team default restores.

## Plugin API

| ID       | Derived From                                                |
|----------|-------------------------------------------------------------|
| R-15.1.8 | [F-15.1.8](../../features/tools-editor/editor-framework.md) |

1. **R-15.1.8** — The editor **SHALL** expose a stable plugin API for adding custom panels, gizmos,
   importers, context menu actions, and toolbar buttons, with hot-reload support that applies
   updated plugin behavior without restarting the editor.
   - **Rationale:** Studios require project-specific tools without forking the editor; hot-reload
     accelerates plugin development iteration.
   - **Verification:** Integration test: load a plugin, verify its panel appears. Modify the plugin,
     trigger hot-reload, and verify updated behavior without restart. Unload the plugin and verify
     its UI elements are removed cleanly.

## VR Editor

| ID       | Derived From                                                |
|----------|-------------------------------------------------------------|
| R-15.1.9 | [F-15.1.9](../../features/tools-editor/editor-framework.md) |

1. **R-15.1.9** — The editor **SHALL** support a VR editing mode via OpenXR with motion controller
   gizmos for grab, rotate, and scale, stereoscopic viewport rendering, and real-time
   synchronization of VR edits to the desktop editor.
   - **Rationale:** VR editing lets designers evaluate spatial proportions at true player scale with
     natural hand gestures.
   - **Verification:** Integration test: enter VR mode, place an entity via motion controller, and
     verify the entity appears at the correct position in the desktop viewport.

## Non-Functional

| ID         | Derived From                                                |
|------------|-------------------------------------------------------------|
| R-15.1.NF1 | [F-15.1.1](../../features/tools-editor/editor-framework.md) |

1. **R-15.1.NF1** — UI input events **SHALL** be acknowledged within 16 ms. Panel layout operations
   **SHALL** complete within 100 ms. Long-running tasks **SHALL** execute asynchronously and
   **SHALL NOT** block the UI thread for more than 50 ms.
   - **Rationale:** Sluggish UI interrupts creative flow and reduces productivity across all
     disciplines.
   - **Verification:** Benchmark: measure 99th-percentile UI response time under a 10,000-entity
     stress test. Verify all targets are met.

---

## User Story Traceability

User stories for this domain are maintained in
[user-stories/tools-editor/editor-framework.md](../../user-stories/tools-editor/editor-framework.md).
Requirements in this document are derived from those user stories.
