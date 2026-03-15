# R-15.1 — Editor Framework Requirements

## Panel System

### R-15.1.1 Dockable Panel Layout

The engine **SHALL** provide a dockable panel system supporting drag-and-drop rearrangement,
splitting, tabbing, and floating of panels, with named layout profiles that persist across
sessions and can be switched without restarting the editor.

- **Derived from:** [F-15.1.1](../../features/tools-editor/editor-framework.md)
- **Rationale:** Artists, designers, and programmers require distinct workspace layouts; persistent
  profiles eliminate repetitive manual arrangement when switching tasks.
- **Verification:** Integration test: create a layout with four docked panels, float one panel,
  split another, and save as a named profile. Restart the editor, load the profile, and assert all
  panel positions, sizes, and tab groupings match the saved state. Repeat on macOS (verifying
  NSWindow floating) and Windows (verifying virtual desktop integration).

### R-15.1.2 Multi-Viewport

The engine **SHALL** support multiple simultaneous 3D viewports, each with an independent camera,
render settings, and debug overlays, all rendered through the same render graph used at runtime.

- **Derived from:** [F-15.1.2](../../features/tools-editor/editor-framework.md)
- **Rationale:** Side-by-side views (top-down and perspective, player camera and free-fly) are
  essential for spatial level design and split-screen multiplayer testing.
- **Verification:** Integration test: open three viewports with distinct cameras and debug overlays.
  Assert each viewport renders independently with correct camera transforms. Verify GPU memory
  usage scales linearly with viewport count by comparing allocations for one, two, and three
  viewports.

## Undo and Selection

### R-15.1.3 Undo/Redo System (Command Pattern)

The engine **SHALL** implement an undo/redo stack using serializable, reversible command objects,
supporting transaction grouping of related operations into a single undo step and persisting the
command history per session for crash recovery.

- **Derived from:** [F-15.1.3](../../features/tools-editor/editor-framework.md)
- **Rationale:** Reversible commands protect against accidental data loss and enable crash recovery
  by replaying the persisted history after an unexpected shutdown.
- **Verification:** Unit test: execute ten commands, undo five, redo three, and assert the state
  matches the expected outcome after each step. Group three commands as a transaction, undo once,
  and assert all three are reverted atomically. Integration test: execute commands, force-terminate
  the editor, relaunch, and verify the session recovers to the last committed state.

### R-15.1.4 Selection System

The engine **SHALL** provide a unified selection model for entities, components, assets, and
sub-object elements (vertices, faces, bones) with click, marquee, lasso, and programmatic
selection modes, supporting additive and subtractive modifiers and named, saveable selection sets.

- **Derived from:** [F-15.1.4](../../features/tools-editor/editor-framework.md)
- **Rationale:** A unified selection model prevents workflow fragmentation across different object
  types and enables saved selection sets for repeated batch operations.
- **Verification:** Unit test: perform click, marquee, and lasso selections on mixed entity and
  vertex targets. Assert additive modifier adds to the current selection and subtractive modifier
  removes from it. Save a named selection set, clear selection, reload the set, and verify the
  same objects are selected.

## Gizmos

### R-15.1.5 Transform Gizmos

The engine **SHALL** render interactive translate, rotate, and scale gizmos in the viewport
operating in local, world, or custom reference frames, with per-axis snap increments and visual
delta feedback during manipulation.

- **Derived from:** [F-15.1.5](../../features/tools-editor/editor-framework.md)
- **Rationale:** Axis-constrained manipulation with configurable snapping is fundamental for precise
  spatial placement of game objects in 3D space.
- **Verification:** Integration test: select an entity and use each gizmo mode (translate, rotate,
  scale) in local and world frames. Assert transforms match expected values. Enable grid snap at
  0.5 units, drag-translate, and verify the entity position is quantized to 0.5-unit increments.
  Verify the delta overlay displays the correct displacement during drag.

### R-15.1.6 Bounds and Measurement Gizmos

The engine **SHALL** display axis-aligned bounding boxes, oriented bounding boxes, distance rulers,
angle protractors, and area measurements in the viewport with values shown in world units.

- **Derived from:** [F-15.1.6](../../features/tools-editor/editor-framework.md)
- **Rationale:** Designers must verify gameplay metrics (corridor widths, jump distances, sightline
  clearances) directly in the viewport without external measurement tools.
- **Verification:** Integration test: place two entities at known positions, activate the distance
  ruler, and assert the displayed distance matches the computed Euclidean distance within 0.01
  units. Verify bounding box dimensions match the mesh extents. Verify the angle protractor reads
  90 degrees for orthogonal surfaces.

## Extensibility

### R-15.1.7 Editor Preferences and Configuration

The engine **SHALL** provide a centralized preference system covering input bindings, visual themes,
viewport defaults, grid settings, and auto-save intervals, stored as versioned JSON that supports
team-wide defaults with per-user local overrides.

- **Derived from:** [F-15.1.7](../../features/tools-editor/editor-framework.md)
- **Rationale:** Versioned, VCS-synced preferences propagate team standards automatically while
  allowing individuals to maintain personal overrides without merge conflicts.
- **Verification:** Unit test: set a team-wide default grid size, apply a per-user override, and
  assert the override takes precedence. Remove the override and assert the team default is
  restored. Verify the JSON output is valid and keys are sorted. Integration test: commit
  preferences to version control, clone on a second machine, and verify team defaults apply.

### R-15.1.8 Editor Extensions and Plugin API

The engine **SHALL** expose a stable plugin API for adding custom panels, gizmos, importers,
context menu actions, and toolbar buttons, with hot-reload support enabling plugin code changes
to take effect without restarting the editor.

- **Derived from:** [F-15.1.8](../../features/tools-editor/editor-framework.md)
- **Rationale:** Studios need to extend the editor with project-specific tools (quest editors,
  dialogue trees, loot tables) without forking or modifying the editor core.
- **Verification:** Integration test: load a plugin that registers a custom panel and toolbar
  button. Assert the panel appears in the panel menu and the button triggers its callback. Modify
  the plugin source, trigger hot-reload, and verify the updated behavior takes effect without
  restarting the editor. Verify unloading the plugin removes its UI elements cleanly.

## VR Editing

## Non-Functional Requirements

### R-15.1.NF1 Editor UI Responsiveness

The editor **SHALL** maintain interactive responsiveness at all times. UI input events (clicks,
drags, key presses) **SHALL** be acknowledged within 16ms (one frame at 60 FPS). Panel layout
operations (dock, undock, split, tab) **SHALL** complete within 100ms. Property inspector updates
**SHALL** reflect value changes within one frame. The editor **SHALL NOT** block the UI thread
for operations exceeding 50ms; long-running tasks **SHALL** execute asynchronously with progress
indicators.

- **Derived from:** General usability requirement across all editor features (F-15.1.1 through
  F-15.1.9).
- **Rationale:** Editor responsiveness directly impacts artist and designer productivity. Latency
  above perceptual thresholds breaks flow state and reduces iteration speed.
- **Verification:** Instrument all UI event handlers and assert 99th percentile response time is
  under 16ms during a stress test with 10,000 entities loaded. Perform 100 panel dock/undock cycles
  and assert all complete within 100ms. Trigger a long-running asset import and verify the UI
  remains interactive throughout.

### R-15.1.NF2 Undo/Redo Performance

The undo/redo system **SHALL** execute undo and redo operations within 50ms for any single command
and within 200ms for grouped transactions containing up to 1,000 sub-commands. The command history
**SHALL** support at least 10,000 entries without degrading undo/redo performance. Crash recovery
replay of the full command history **SHALL** complete within 10 seconds for histories containing up
to 10,000 commands.

- **Derived from:** F-15.1.3 (Undo/Redo System).
- **Rationale:** Slow undo/redo disrupts the rapid experimentation workflow critical for level
  design and material authoring. Large transaction groups must undo atomically without perceptible
  delay.
- **Verification:** Record 10,000 commands; measure undo latency for the most recent command and
  assert it is under 50ms. Group 1,000 entity-move commands into a transaction, undo, and assert
  completion within 200ms. Force-terminate and replay the 10,000-command history; assert recovery
  completes within 10 seconds.

### R-15.1.9 VR Editor Mode

The engine **SHALL** provide an immersive VR editing mode that renders the editor viewport
stereoscopically with head tracking and motion controller input, supporting grab, place, rotate,
and scale operations via controller gizmos, with changes synchronized to the desktop editor in
real time.

- **Derived from:** [F-15.1.9](../../features/tools-editor/editor-framework.md)
- **Rationale:** Spatial editing in VR lets designers experience levels at true player scale,
  improving spatial judgment for layout, proportions, and sightlines.
- **Verification:** Integration test with an OpenXR-compatible headset: enter VR mode, grab an
  entity with a motion controller, move it to a new position, and verify the transform updates on
  the desktop mirror. Verify head tracking drives the viewport camera by comparing headset pose
  with rendered camera transform. Verify the floating tool palette is accessible via laser pointer
  and triggers the correct operations.
