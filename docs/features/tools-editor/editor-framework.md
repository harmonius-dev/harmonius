# 15.1 — Editor Framework

## Panel System

### F-15.1.1 Dockable Panel Layout

Provides a fully dockable panel system where users can drag, drop, split, tab, and float panels
to create custom workspace layouts. Persistent layout profiles allow team members to switch
between level design, material authoring, and animation workspaces instantly. Essential for
MMO-scale teams where artists, designers, and programmers each need tailored workflows.

- **Requirements:** R-15.1.1
- **Dependencies:** F-14.1.1
- **Platform notes:** On macOS, floating panels use native NSWindow for proper focus and
  Expose integration. On Windows, panels integrate with virtual desktop APIs.

### F-15.1.2 Multi-Viewport

Supports multiple simultaneous 3D viewports within the editor, each with independent camera,
render settings, and debug overlays. Enables workflows such as top-down and perspective views
side-by-side, player camera preview alongside free-fly editing, and split-screen testing for
multiplayer. Each viewport renders through the same render graph as the game.

- **Requirements:** R-15.1.2
- **Dependencies:** F-15.1.1, F-2.3.8
- **Platform notes:** Each viewport allocates its own swapchain; GPU memory scales linearly
  with viewport count.

## Undo and Selection

### F-15.1.3 Undo/Redo System (Command Pattern)

Implements a full undo/redo stack using the command pattern where every editor action is
captured as a serializable, reversible command. Supports transaction grouping to batch related
operations (e.g., multi-entity move) into a single undo step. The command history is saved per
editor session for crash recovery.

- **Requirements:** R-15.1.3
- **Dependencies:** F-7.4.1
- **Platform notes:** None

### F-15.1.4 Selection System

Provides a unified selection model for entities, components, assets, and sub-object elements
(vertices, faces, bones). Supports click, marquee, lasso, and programmatic selection modes
with additive and subtractive modifiers. Selection sets can be named and saved for repeated
operations on the same group of objects.

- **Requirements:** R-15.1.4
- **Dependencies:** F-7.1.1
- **Platform notes:** None

## Gizmos

### F-15.1.5 Transform Gizmos

Interactive translate, rotate, and scale gizmos rendered as 3D overlays in the viewport.
Gizmos operate in local, world, or custom reference frames and support axis constraints,
plane constraints, and free-form manipulation. Snap increments are configurable per-axis.
Visual feedback shows the delta during manipulation.

- **Requirements:** R-15.1.5
- **Dependencies:** F-15.1.4, F-15.1.3
- **Platform notes:** None

### F-15.1.6 Bounds and Measurement Gizmos

Displays axis-aligned and oriented bounding boxes, distance rulers, angle protractors, and
area measurements in the viewport. Designers use these to verify gameplay metrics such as
corridor widths, jump distances, and line-of-sight clearances across large open-world zones.

- **Requirements:** R-15.1.6
- **Dependencies:** F-15.1.5
- **Platform notes:** None

## Extensibility

### F-15.1.7 Editor Preferences and Configuration

Centralized preference system covering input bindings, visual themes, viewport defaults, grid
settings, auto-save intervals, and per-user overrides. Preferences are stored as versioned
JSON and synced via version control so team-wide defaults propagate while personal overrides
remain local.

- **Requirements:** R-15.1.7
- **Dependencies:** F-7.4.1
- **Platform notes:** None

### F-15.1.8 Editor Extensions and Plugin API

Exposes a stable plugin API that allows teams to add custom panels, gizmos, importers, context
menu actions, and toolbar buttons without modifying the editor core. Plugins are hot-reloaded
during development, enabling rapid iteration on studio-specific tools such as quest editors,
dialogue trees, and loot table configurators.

- **Requirements:** R-15.1.8
- **Dependencies:** F-15.1.1, F-7.6.1
- **Platform notes:** None
