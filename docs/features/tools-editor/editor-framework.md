# 15.1 — Editor Framework

## Panel System

| ID       | Feature               | Requirements |
|----------|-----------------------|--------------|
| F-15.1.1 | Dockable Panel Layout | R-15.1.1     |
| F-15.1.2 | Multi-Viewport        | R-15.1.2     |

1. **F-15.1.1** — Provides a fully dockable panel system where users can drag, drop, split, tab, and
   float panels to create custom workspace layouts. Persistent layout profiles allow team members to
   switch between level design, material authoring, and animation workspaces instantly. Essential
   for MMO-scale teams where artists, designers, and programmers each need tailored workflows.
   integration. On Windows, panels integrate with virtual desktop APIs.
   - **Deps:** F-14.1.1
   - **Platform:** On macOS, floating panels use native NSWindow for proper focus and Expose
2. **F-15.1.2** — Supports multiple simultaneous 3D viewports within the editor, each with
   independent camera, render settings, and debug overlays. Enables workflows such as top-down and
   perspective views side-by-side, player camera preview alongside free-fly editing, and
   split-screen testing for multiplayer. Each viewport renders through the same render graph as the
   game. viewport count.
   - **Deps:** F-15.1.1, F-2.3.8
   - **Platform:** Each viewport allocates its own swapchain; GPU memory scales linearly with

## Undo and Selection

| ID       | Feature                            | Requirements |
|----------|------------------------------------|--------------|
| F-15.1.3 | Undo/Redo System (Command Pattern) | R-15.1.3     |
| F-15.1.4 | Selection System                   | R-15.1.4     |

1. **F-15.1.3** — Implements a full undo/redo stack using the command pattern where every editor
   action is captured as a serializable, reversible command. Supports transaction grouping to batch
   related operations (e.g., multi-entity move) into a single undo step. The command history is
   saved per editor session for crash recovery.
   - **Deps:** F-7.4.1
   - **Platform:** Desktop only. Not available on mobile or console runtime.
2. **F-15.1.4** — Provides a unified selection model for entities, components, assets, and
   sub-object elements (vertices, faces, bones). Supports click, marquee, lasso, and programmatic
   selection modes with additive and subtractive modifiers. Selection sets can be named and saved
   for repeated operations on the same group of objects.
   - **Deps:** F-7.1.1
   - **Platform:** Desktop only. Not available on mobile or console runtime.

## Gizmos

| ID       | Feature                       | Requirements |
|----------|-------------------------------|--------------|
| F-15.1.5 | Transform Gizmos              | R-15.1.5     |
| F-15.1.6 | Bounds and Measurement Gizmos | R-15.1.6     |

1. **F-15.1.5** — Interactive translate, rotate, and scale gizmos rendered as 3D overlays in the
   viewport. Gizmos operate in local, world, or custom reference frames and support axis
   constraints, plane constraints, and free-form manipulation. Snap increments are configurable
   per-axis. Visual feedback shows the delta during manipulation.
   - **Deps:** F-15.1.4, F-15.1.3
   - **Platform:** Desktop only. Not available on mobile or console runtime.
2. **F-15.1.6** — Displays axis-aligned and oriented bounding boxes, distance rulers, angle
   protractors, and area measurements in the viewport. Designers use these to verify gameplay
   metrics such as corridor widths, jump distances, and line-of-sight clearances across large
   open-world zones.
   - **Deps:** F-15.1.5
   - **Platform:** Desktop only. Not available on mobile or console runtime.

## Extensibility

| ID       | Feature                              | Requirements |
|----------|--------------------------------------|--------------|
| F-15.1.7 | Editor Preferences and Configuration | R-15.1.7     |
| F-15.1.8 | Editor Extensions and Plugin API     | R-15.1.8     |

1. **F-15.1.7** — Centralized preference system covering input bindings, visual themes, viewport
   defaults, grid settings, auto-save intervals, and per-user overrides. Preferences are stored as
   versioned JSON and synced via version control so team-wide defaults propagate while personal
   overrides remain local.
   - **Deps:** F-7.4.1
   - **Platform:** Desktop only. Not available on mobile or console runtime.
2. **F-15.1.8** — Exposes a stable plugin API that allows teams to add custom panels, gizmos,
   importers, context menu actions, and toolbar buttons without modifying the editor core. Plugins
   are hot-reloaded during development, enabling rapid iteration on studio-specific tools such as
   quest editors, dialogue trees, and loot table configurators.
   - **Deps:** F-15.1.1, F-7.6.1
   - **Platform:** Desktop only. Not available on mobile or console runtime.

## VR Editing

| ID       | Feature        | Requirements |
|----------|----------------|--------------|
| F-15.1.9 | VR Editor Mode | R-15.1.9     |

1. **F-15.1.9** — An immersive VR editing mode that renders the editor viewport in a VR headset,
   enabling spatial editing with motion controllers. Designers grab, place, rotate, and scale
   objects with natural hand gestures. The 3D viewport is rendered stereoscopically with head
   tracking (F-6.5.1). Controller-based gizmos replace mouse-based gizmos for transform
   manipulation. A floating tool palette provides access to common operations (selection, paint,
   sculpt) via laser pointer interaction. World-scale preview lets designers experience their levels
   at true player scale. VR mode coexists with the desktop editor — changes made in VR are
   immediately visible on the desktop monitor. Controllers) editor rendering pipeline.
   - **Deps:** F-15.1.1 (Editor Framework), F-6.5.1 (HMD Tracking), F-6.5.2 (Motion
   - **Platform:** Requires OpenXR-compatible headset. Desktop mirror view uses the existing
