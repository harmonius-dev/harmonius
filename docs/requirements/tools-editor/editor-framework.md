# R-15.1 -- Editor Framework Requirements

## Dockable Panel Layout

### R-15.1.1 Dockable Panel System

The editor **SHALL** provide a dockable panel system
supporting drag-and-drop rearrangement, horizontal and
vertical splitting, tab grouping, and floating panels as
independent OS windows.

- **Derived from:**
  [F-15.1.1](../../features/tools-editor/editor-framework.md)
- **Rationale:** Artists, designers, and programmers each
  need tailored workspace layouts; a rigid layout blocks
  multi-discipline workflows.
- **Verification:** Integration test: drag a panel to each
  dock zone (left, right, top, bottom, tab, float), save
  the layout, restart the editor, and verify all positions
  and sizes are restored within 1 pixel.

### R-15.1.1a Layout Profile Persistence

The editor **SHALL** persist named layout profiles as
versioned JSON files stored in VCS with per-user override
files excluded from VCS.

- **Derived from:**
  [F-15.1.1](../../features/tools-editor/editor-framework.md)
- **Rationale:** Team-wide defaults must propagate via VCS
  while personal overrides remain local to avoid merge
  conflicts.
- **Verification:** Unit test: save a profile, modify it,
  save a per-user override, load both, and verify the
  override takes precedence. Remove the override and verify
  the team default restores.

## Multi-Viewport

### R-15.1.2 Multiple Simultaneous Viewports

The editor **SHALL** support at least four simultaneous 3D
viewports, each with an independent camera, render settings,
and debug overlays, rendering through the same render graph
as the game runtime.

- **Derived from:**
  [F-15.1.2](../../features/tools-editor/editor-framework.md)
- **Rationale:** Artists need lit/unlit comparison; designers
  need top-down and perspective simultaneously; multiplayer
  testing requires split-screen preview.
- **Verification:** Integration test: open four viewports
  with different camera transforms and render modes. Verify
  each renders independently and matches the game runtime
  output via screenshot comparison.

## Undo/Redo

### R-15.1.3 Command-Pattern Undo/Redo

The editor **SHALL** capture every user action as a
serializable, reversible command object, supporting grouped
transactions that undo atomically and crash-recovery replay
from a persisted command history.

- **Derived from:**
  [F-15.1.3](../../features/tools-editor/editor-framework.md)
- **Rationale:** Accidental changes in a large open-world
  editor must be reversible; crash recovery prevents work
  loss.
- **Verification:** Unit test: execute 100 commands, undo
  all, verify world state matches initial state. Simulate
  crash, replay history, and verify state matches
  pre-crash state.

### R-15.1.3a Undo/Redo Performance

Single-command undo/redo **SHALL** complete within 50 ms.
Grouped transactions of up to 1,000 sub-commands **SHALL**
undo within 200 ms. Crash recovery replay of 10,000
commands **SHALL** complete within 10 seconds.

- **Derived from:**
  [F-15.1.3](../../features/tools-editor/editor-framework.md)
- **Rationale:** Undo must feel instantaneous; crash
  recovery must complete before the user loses patience.
- **Verification:** Benchmark: time undo of a single
  command and a 1,000-command group. Benchmark crash
  recovery replay of 10,000 commands.

## Selection

### R-15.1.4 Unified Selection Model

The editor **SHALL** provide a unified selection model
supporting click, marquee, lasso, additive, subtractive,
and programmatic selection across entities, components,
assets, and sub-object elements (vertices, faces, bones),
with named selection sets that persist across sessions.

- **Derived from:**
  [F-15.1.4](../../features/tools-editor/editor-framework.md)
- **Rationale:** Consistent selection behavior across object
  types reduces cognitive load; named sets accelerate
  repeated batch operations.
- **Verification:** Integration test: select entities via
  each mode, save a named set, restart the editor, and
  verify the set restores correctly.

## Transform Gizmos

### R-15.1.5 Transform Gizmos

The editor **SHALL** provide interactive translate, rotate,
and scale gizmos with axis and plane constraints,
configurable snap increments, and local/world/custom
reference frames.

- **Derived from:**
  [F-15.1.5](../../features/tools-editor/editor-framework.md)
- **Rationale:** Precise entity manipulation is fundamental
  to level design; snap and constraints prevent misaligned
  placements.
- **Verification:** Unit test: translate an entity with grid
  snap enabled and verify the resulting position is quantized
  to the snap increment within epsilon.

## Measurement Gizmos

### R-15.1.6 Bounds and Measurement Gizmos

The editor **SHALL** provide distance rulers, angle
protractors, area measurement overlays, and axis-aligned
and oriented bounding box displays, accurate within 0.01
world units.

- **Derived from:**
  [F-15.1.6](../../features/tools-editor/editor-framework.md)
- **Rationale:** Designers verify gameplay metrics (corridor
  widths, jump distances) directly in the viewport.
- **Verification:** Unit test: place two points at known
  coordinates and verify the ruler reports the correct
  Euclidean distance within 0.01 units.

## Preferences

### R-15.1.7 Editor Preferences

The editor **SHALL** provide a centralized preference
system with input bindings, visual themes, viewport
defaults, and auto-save intervals, stored as versioned
JSON with per-user overrides separate from team defaults.

- **Derived from:**
  [F-15.1.7](../../features/tools-editor/editor-framework.md)
- **Rationale:** Team-wide consistency with personal
  customization prevents configuration drift while
  respecting individual workflows.
- **Verification:** Unit test: set a team default, apply a
  per-user override, verify the override takes effect.
  Remove the override and verify the team default restores.

## Plugin API

### R-15.1.8 Editor Extension and Plugin API

The editor **SHALL** expose a stable plugin API for adding
custom panels, gizmos, importers, context menu actions, and
toolbar buttons, with hot-reload support that applies
updated plugin behavior without restarting the editor.

- **Derived from:**
  [F-15.1.8](../../features/tools-editor/editor-framework.md)
- **Rationale:** Studios require project-specific tools
  without forking the editor; hot-reload accelerates plugin
  development iteration.
- **Verification:** Integration test: load a plugin, verify
  its panel appears. Modify the plugin, trigger hot-reload,
  and verify updated behavior without restart. Unload the
  plugin and verify its UI elements are removed cleanly.

## VR Editor

### R-15.1.9 VR Editor Mode

The editor **SHALL** support a VR editing mode via OpenXR
with motion controller gizmos for grab, rotate, and scale,
stereoscopic viewport rendering, and real-time
synchronization of VR edits to the desktop editor.

- **Derived from:**
  [F-15.1.9](../../features/tools-editor/editor-framework.md)
- **Rationale:** VR editing lets designers evaluate spatial
  proportions at true player scale with natural hand
  gestures.
- **Verification:** Integration test: enter VR mode, place
  an entity via motion controller, and verify the entity
  appears at the correct position in the desktop viewport.

## Non-Functional

### R-15.1.NF1 Editor UI Responsiveness

UI input events **SHALL** be acknowledged within 16 ms.
Panel layout operations **SHALL** complete within 100 ms.
Long-running tasks **SHALL** execute asynchronously and
**SHALL NOT** block the UI thread for more than 50 ms.

- **Derived from:**
  [F-15.1.1](../../features/tools-editor/editor-framework.md)
- **Rationale:** Sluggish UI interrupts creative flow and
  reduces productivity across all disciplines.
- **Verification:** Benchmark: measure 99th-percentile UI
  response time under a 10,000-entity stress test. Verify
  all targets are met.

---

## User Stories

## US-15.1.1 Dockable Panel Layout

### US-15.1.1.1
As a **designer (P-5)**, I want to drag and drop panels to rearrange my workspace so that I can
position the level hierarchy next to the viewport for fast entity selection.

### US-15.1.1.2
As a **designer (P-5)**, I want to save named layout profiles so that I can switch between
level-design and scripting workspaces instantly.

### US-15.1.1.3
As a **designer (P-5)**, I want to load a saved layout profile on editor startup so that my
preferred workspace is ready without manual arrangement each session.

### US-15.1.1.4
As a **artist (P-8)**, I want to split a panel into two side-by-side panes so that I can view a
material preview alongside the material graph simultaneously.

### US-15.1.1.5
As a **artist (P-8)**, I want to tab multiple panels into a single pane so that I can minimize
screen clutter while keeping panels accessible via tabs.

### US-15.1.1.6
As a **artist (P-8)**, I want to float a panel as an independent window so that I can move it to a
secondary monitor for expanded workspace.

### US-15.1.1.7
As a **developer (P-15)**, I want layout profiles stored as versioned JSON in VCS so that team-wide
layout defaults propagate automatically when I pull changes.

### US-15.1.1.8
As a **developer (P-15)**, I want per-user layout overrides separate from team defaults so that my
personal arrangement is preserved without causing merge conflicts.

### US-15.1.1.9
As a **technical artist (P-13)**, I want to create a custom workspace with shader and profiling
panels docked together so that I can optimize materials and monitor GPU performance in a single
layout.

### US-15.1.1.10
As a **creative director (P-2)**, I want to define and distribute a recommended layout profile for
each discipline so that new team members start with an optimal workspace for their role.

### US-15.1.1.11
As an **engine dev (P-26)**, I want the docking system to use native NSWindow on macOS for floating
panels so that floating panels integrate with Expose and Mission Control correctly.

### US-15.1.1.12
As an **engine dev (P-26)**, I want the docking system to integrate with Windows virtual desktop
APIs so that floating panels track the correct virtual desktop on Windows.

### US-15.1.1.13
As an **engine tester (P-27)**, I want to verify that a saved layout profile restores all panel
positions, sizes, and tab groupings after an editor restart so that layout persistence is
regression-tested across sessions.

---

## US-15.1.2 Multi-Viewport

### US-15.1.2.1
As a **designer (P-5)**, I want to open multiple 3D viewports side by side with independent cameras
so that I can view top-down and perspective angles simultaneously during level layout.

### US-15.1.2.2
As a **designer (P-5)**, I want to preview the player camera in one viewport while free-flying in
another so that I can verify gameplay sightlines while editing the environment.

### US-15.1.2.3
As a **artist (P-8)**, I want each viewport to have independent render settings so that I can
compare lit and unlit views of the same scene simultaneously.

### US-15.1.2.4
As a **artist (P-8)**, I want debug overlays (wireframe, normals, UV) per viewport so that I can
diagnose mesh issues without affecting my primary art viewport.

### US-15.1.2.5
As a **developer (P-15)**, I want each viewport to render through the same render graph as the game
runtime so that what I see in the editor matches the shipped game exactly.

### US-15.1.2.6
As a **developer (P-15)**, I want split-screen testing viewports for multiplayer so that I can test
local co-op camera behavior directly in the editor.

### US-15.1.2.7
As an **engine dev (P-26)**, I want each viewport to allocate its own swapchain so that GPU memory
scales linearly and predictably with viewport count.

### US-15.1.2.8
As an **engine tester (P-27)**, I want to verify that three viewports render independently with
correct camera transforms so that multi-viewport rendering is validated under automated test.

---

## US-15.1.3 Undo/Redo System (Command Pattern)

### US-15.1.3.1
As a **designer (P-5)**, I want every editor action captured as a reversible command so that I can
undo any accidental change and recover my previous state.

### US-15.1.3.2
As a **designer (P-5)**, I want to undo a multi-entity move as a single step so that grouped
operations revert atomically without leaving partial changes.

### US-15.1.3.3
As a **artist (P-8)**, I want the undo stack to persist across sessions for crash recovery so that I
do not lose work if the editor crashes unexpectedly.

### US-15.1.3.4
As a **artist (P-8)**, I want redo to restore undone actions in the correct order so that I can
re-apply changes without manually repeating them.

### US-15.1.3.5
As a **developer (P-15)**, I want command objects to be serializable so that the undo history can be
saved to disk for crash recovery replay.

### US-15.1.3.6
As an **engine dev (P-26)**, I want undo/redo operations to complete within 50ms per command so that
reverting actions feels instantaneous to the user.

### US-15.1.3.7
As an **engine dev (P-26)**, I want grouped transactions of up to 1000 sub-commands to undo within
200ms so that large batch operations revert without perceptible delay.

### US-15.1.3.8
As an **engine tester (P-27)**, I want to verify crash recovery replays 10,000 commands within 10
seconds so that session recovery performance meets the specified target.

### US-15.1.3.9
As a **technical artist (P-13)**, I want transaction grouping for batch terrain paint operations so
that a complex painting session undoes as a single step.

---

## US-15.1.4 Selection System

### US-15.1.4.1
As a **designer (P-5)**, I want to click-select entities in the viewport so that I can pick
individual objects for inspection or transformation.

### US-15.1.4.2
As a **designer (P-5)**, I want marquee (box) selection across the viewport so that I can select
multiple entities in a rectangular region quickly.

### US-15.1.4.3
As a **designer (P-5)**, I want lasso selection with freeform drawing so that I can select
irregularly shaped groups of entities precisely.

### US-15.1.4.4
As a **designer (P-5)**, I want additive selection with a modifier key so that I can add objects to
an existing selection without clearing it.

### US-15.1.4.5
As a **designer (P-5)**, I want subtractive selection with a modifier key so that I can remove
individual objects from the current selection.

### US-15.1.4.6
As a **designer (P-5)**, I want to save named selection sets so that I can re-select the same group
of objects for repeated batch operations.

### US-15.1.4.7
As a **artist (P-8)**, I want to select sub-object elements (vertices, faces, bones) so that I can
perform fine-grained edits on mesh and skeleton data.

### US-15.1.4.8
As a **artist (P-8)**, I want programmatic selection of assets by type or tag so that I can
bulk-select all materials matching a specific category.

### US-15.1.4.9
As a **developer (P-15)**, I want a unified selection model across entities, components, and assets
so that selection behavior is consistent regardless of object type.

### US-15.1.4.10
As an **engine tester (P-27)**, I want to verify that named selection sets persist across editor
sessions so that saved selections survive restarts without data loss.

---

## US-15.1.5 Transform Gizmos

### US-15.1.5.1
As a **designer (P-5)**, I want interactive translate gizmos with axis constraints so that I can
move entities along a single axis precisely.

### US-15.1.5.2
As a **designer (P-5)**, I want rotate gizmos with visual angle feedback so that I can orient
entities with exact rotation values.

### US-15.1.5.3
As a **designer (P-5)**, I want scale gizmos with uniform and per-axis modes so that I can resize
entities proportionally or along individual axes.

### US-15.1.5.4
As a **designer (P-5)**, I want configurable snap increments per axis so that I can align entities
to a grid during placement.

### US-15.1.5.5
As a **designer (P-5)**, I want gizmos that operate in local, world, or custom frames so that I can
manipulate entities relative to their own orientation or world axes.

### US-15.1.5.6
As a **artist (P-8)**, I want plane-constrained gizmo manipulation so that I can move entities
freely on a 2D plane within 3D space.

### US-15.1.5.7
As a **artist (P-8)**, I want visual delta feedback during gizmo manipulation so that I can see the
exact displacement as I drag.

### US-15.1.5.8
As an **engine tester (P-27)**, I want to verify that gizmo snap produces quantized positions at the
configured increment so that snapping precision is validated under automated test.

---

## US-15.1.6 Bounds and Measurement Gizmos

### US-15.1.6.1
As a **designer (P-5)**, I want distance rulers between two points in the viewport so that I can
verify corridor widths and jump distances for gameplay metrics.

### US-15.1.6.2
As a **designer (P-5)**, I want angle protractors in the viewport so that I can measure slope angles
and sightline clearances.

### US-15.1.6.3
As a **designer (P-5)**, I want area measurement overlays so that I can verify zone sizes for
gameplay balancing.

### US-15.1.6.4
As a **artist (P-8)**, I want axis-aligned bounding box display on selected entities so that I can
verify mesh extents match art specifications.

### US-15.1.6.5
As a **artist (P-8)**, I want oriented bounding box display so that I can inspect tight-fitting
bounds on rotated objects.

### US-15.1.6.6
As an **engine tester (P-27)**, I want to verify that the distance ruler reports the correct
Euclidean distance within 0.01 units so that measurement accuracy is regression-tested.

---

## US-15.1.7 Editor Preferences and Configuration

### US-15.1.7.1
As a **designer (P-5)**, I want to customize input bindings in a preferences panel so that I can use
keyboard shortcuts that match my workflow.

### US-15.1.7.2
As a **artist (P-8)**, I want to select a visual theme (dark, light, custom) so that the editor is
comfortable for extended use.

### US-15.1.7.3
As a **artist (P-8)**, I want to configure viewport defaults (grid size, camera speed) so that new
viewports open with my preferred settings.

### US-15.1.7.4
As a **developer (P-15)**, I want preferences stored as versioned JSON synced via VCS so that
team-wide defaults propagate automatically without manual distribution.

### US-15.1.7.5
As a **developer (P-15)**, I want per-user overrides separate from team defaults so that personal
settings do not conflict with shared configuration.

### US-15.1.7.6
As a **developer (P-15)**, I want configurable auto-save intervals so that unsaved work is protected
at a frequency I control.

### US-15.1.7.7
As a **creative director (P-2)**, I want to set team-wide default preferences so that all team
members start with consistent editor settings.

### US-15.1.7.8
As an **engine tester (P-27)**, I want to verify that removing a per-user override restores the team
default value so that the override/default hierarchy works correctly.

---

## US-15.1.8 Editor Extensions and Plugin API

### US-15.1.8.1
As a **developer (P-15)**, I want a stable plugin API for adding custom panels so that I can extend
the editor without modifying its core source.

### US-15.1.8.2
As a **developer (P-15)**, I want to register custom gizmos through the plugin API so that
project-specific visualization tools appear in the viewport.

### US-15.1.8.3
As a **developer (P-15)**, I want to add custom importers via the plugin API so that the editor
handles proprietary asset formats our studio requires.

### US-15.1.8.4
As a **developer (P-15)**, I want to add context menu actions and toolbar buttons so that
studio-specific operations are accessible from the editor UI.

### US-15.1.8.5
As a **developer (P-15)**, I want hot-reload of plugin code during development so that I can iterate
on plugin behavior without restarting the editor.

### US-15.1.8.6
As a **technical artist (P-13)**, I want to build a custom quest editor as a plugin so that
designers can author quests without touching engine code.

### US-15.1.8.7
As an **engine dev (P-26)**, I want unloading a plugin to remove its UI elements cleanly so that the
editor remains stable after plugin removal.

### US-15.1.8.8
As an **engine tester (P-27)**, I want to verify that hot-reloading a modified plugin applies the
updated behavior without restart so that the hot-reload mechanism is regression-tested.

### US-15.1.8.9
As a **modder (P-24)**, I want to develop editor plugins using the public API so that I can add
custom tools for my modding workflow.

---

## US-15.1.9 VR Editor Mode

### US-15.1.9.1
As a **designer (P-5)**, I want to enter a VR editing mode with my headset so that I can place and
scale objects with natural hand gestures at true player scale.

### US-15.1.9.2
As a **designer (P-5)**, I want motion controller gizmos for grab, rotate, and scale so that I can
manipulate entities spatially without mouse-based gizmos.

### US-15.1.9.3
As a **designer (P-5)**, I want a floating tool palette accessible via laser pointer so that I can
switch between selection, paint, and sculpt tools in VR.

### US-15.1.9.4
As a **artist (P-8)**, I want stereoscopic viewport rendering with head tracking so that I
experience the scene at correct depth and scale while editing.

### US-15.1.9.5
As a **artist (P-8)**, I want changes made in VR to appear on the desktop monitor immediately so
that collaborators viewing my desktop can follow my edits in real time.

### US-15.1.9.6
As a **creative director (P-2)**, I want to review level layouts in VR at player scale so that I can
evaluate spatial proportions and sightlines immersively.

### US-15.1.9.7
As an **engine dev (P-26)**, I want VR mode to use OpenXR for headset compatibility so that the
editor works with any OpenXR-compatible headset.

### US-15.1.9.8
As an **engine tester (P-27)**, I want to verify that VR transform operations sync to the desktop
editor so that VR-to-desktop synchronization is validated.

---

## US-15.1.NF1 Editor UI Responsiveness

### US-15.1.NF1.1
As an **engine dev (P-26)**, I want UI input events acknowledged within 16ms so that clicks and
drags feel instantaneous at 60 FPS.

### US-15.1.NF1.2
As an **engine dev (P-26)**, I want panel layout operations to complete within 100ms so that docking
and splitting panels does not interrupt workflow.

### US-15.1.NF1.3
As an **engine dev (P-26)**, I want long-running tasks to execute asynchronously with progress
indicators so that the UI thread is never blocked for more than 50ms.

### US-15.1.NF1.4
As an **engine tester (P-27)**, I want to measure 99th percentile UI response time under a
10,000-entity stress test so that responsiveness targets are validated under load.
