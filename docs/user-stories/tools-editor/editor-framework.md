# User Stories: Editor Framework

## F-15.1.1 Dockable Panel Layout

## US-15.1.1.1 Designer Customizes Workspace

**As a** designer (P-5), **I want** to drag, drop, split, tab, and float editor panels into a custom
workspace layout, **so that** I can arrange level design tools exactly where I need them without
wasting screen space on panels I rarely use.

## US-15.1.1.2 Designer Switches Workspace Profiles

**As a** designer (P-5), **I want** to save and switch between named layout profiles (e.g., "Level
Design", "Scripting", "Playtesting"), **so that** I can instantly reconfigure the editor for
different tasks without rearranging panels each time.

## US-15.1.1.3 Artist Floats Reference Panel

**As an** artist (P-8), **I want** to float a material preview panel onto a second monitor while
keeping the viewport on my primary display, **so that** I can compare my in-engine material to
reference images side by side.

## US-15.1.1.4 Developer Shares Layout with Team

**As a** developer (P-15), **I want** layout profiles stored as versioned JSON that syncs through
version control, **so that** the entire team inherits consistent workspace defaults while I keep my
personal overrides local.

## US-15.1.1.5 Tech Artist Creates Tool-Specific Layout

**As a** tech artist (P-13), **I want** to create a layout profile that combines the shader graph,
profiler, and viewport in a single arrangement, **so that** I can debug shader performance without
switching between separate workflows.

## US-15.1.1.6 Engine Dev Tests Panel Persistence

**As an** engine developer (P-26), **I want** panel layout state to survive editor restarts and
crash recovery, **so that** users never lose their workspace arrangement due to an unexpected
shutdown.

## US-15.1.1.7 Engine Tester Verifies Platform Integration

**As an** engine tester (P-27), **I want** to verify that floating panels integrate with native
window management (NSWindow on macOS, virtual desktops on Windows), **so that** panels behave like
native windows on each platform.

## US-15.1.1.8 Creative Director Opens Review Layout

**As a** creative director (P-2), **I want** a "Review" workspace profile that maximizes the
viewport with minimal toolbars, **so that** I can evaluate the game's visual quality during art
reviews without UI clutter.

## F-15.1.2 Multi-Viewport

## US-15.1.2.1 Designer Views Top-Down and Perspective

**As a** designer (P-5), **I want** to display top-down and perspective viewports side by side with
independent cameras, **so that** I can verify entity placement from both tactical and player
perspectives simultaneously.

## US-15.1.2.2 Artist Previews Player Camera

**As an** artist (P-8), **I want** one viewport showing the player camera preview alongside my
free-fly editing viewport, **so that** I can see exactly what the player sees while I adjust
environment art.

## US-15.1.2.3 Developer Tests Split-Screen

**As a** developer (P-15), **I want** multiple viewports to render through the same render graph as
the game with independent render settings, **so that** I can test split-screen multiplayer directly
in the editor.

## US-15.1.2.4 Engine Tester Validates GPU Scaling

**As an** engine tester (P-27), **I want** to verify that GPU memory scales linearly with viewport
count and each viewport allocates its own swapchain, **so that** multi-viewport usage does not cause
unexpected memory spikes or resource conflicts.

## F-15.1.3 Undo/Redo System (Command Pattern)

## US-15.1.3.1 Designer Undoes Grouped Operation

**As a** designer (P-5), **I want** multi-entity moves batched into a single undo step via
transaction grouping, **so that** I can revert accidental batch transformations with one undo action
instead of clicking undo once per entity.

## US-15.1.3.2 Artist Recovers from Crash

**As an** artist (P-8), **I want** the undo/redo command history saved per session for crash
recovery, **so that** I can restore my work after an unexpected editor crash without losing recent
edits.

## US-15.1.3.3 Developer Serializes Commands

**As a** developer (P-15), **I want** every editor action captured as a serializable, reversible
command, **so that** I can replay command sequences for automated testing and debugging.

## US-15.1.3.4 Engine Dev Verifies Command Reversibility

**As an** engine developer (P-26), **I want** every command type to implement both execute and
reverse operations correctly, **so that** the undo/redo stack never leaves the editor in an
inconsistent state.

## F-15.1.4 Selection System

## US-15.1.4.1 Designer Selects by Marquee

**As a** designer (P-5), **I want** to select entities using click, marquee, and lasso modes with
additive and subtractive modifiers, **so that** I can quickly isolate groups of objects for batch
operations.

## US-15.1.4.2 Artist Saves Named Selection Sets

**As an** artist (P-8), **I want** to save named selection sets for repeated operations on the same
group of objects, **so that** I can recall complex selections instantly without re-selecting each
entity manually.

## US-15.1.4.3 Tech Artist Selects Sub-Objects

**As a** tech artist (P-13), **I want** to select sub-object elements such as vertices, faces, and
bones, **so that** I can perform fine-grained editing operations on mesh and skeleton data directly
in the editor.

## US-15.1.4.4 Developer Selects Programmatically

**As a** developer (P-15), **I want** to select entities programmatically through the editor API,
**so that** my custom tools and plugins can manipulate selection state consistently with the
built-in selection system.

## F-15.1.5 Transform Gizmos

## US-15.1.5.1 Designer Snaps to Grid

**As a** designer (P-5), **I want** translate, rotate, and scale gizmos with configurable per-axis
snap increments, **so that** I can place entities at precise positions aligned to a grid without
manual coordinate entry.

## US-15.1.5.2 Artist Constrains to Plane

**As an** artist (P-8), **I want** gizmos that support axis constraints, plane constraints, and
free-form manipulation in local, world, or custom reference frames, **so that** I can move objects
along specific axes or planes during detailed environment assembly.

## US-15.1.5.3 Creative Director Sees Visual Feedback

**As a** creative director (P-2), **I want** gizmos to display the delta value during manipulation,
**so that** I can see exactly how far and in which direction an object has been moved, rotated, or
scaled.

## US-15.1.5.4 Engine Tester Validates Gizmo Accuracy

**As an** engine tester (P-27), **I want** to verify that gizmo manipulations produce numerically
accurate transforms matching the displayed delta values, **so that** designers can trust the visual
feedback during precise placement.

## F-15.1.6 Bounds and Measurement Gizmos

## US-15.1.6.1 Designer Measures Gameplay Metrics

**As a** designer (P-5), **I want** distance rulers, angle protractors, and area measurements in the
viewport, **so that** I can verify gameplay metrics such as jump distances, corridor widths, and
line-of-sight clearances.

## US-15.1.6.2 Artist Displays Bounding Boxes

**As an** artist (P-8), **I want** to display axis-aligned and oriented bounding boxes on selected
entities, **so that** I can verify that collision bounds match visual geometry before finalizing
placement.

## US-15.1.6.3 Developer Reads Measurement API

**As a** developer (P-15), **I want** measurement gizmo values exposed through the editor API,
**so that** my custom validation tools can read distances and dimensions programmatically.

## US-15.1.6.4 Engine Tester Validates Measurement Precision

**As an** engine tester (P-27), **I want** to verify that measurement gizmos produce accurate
results at world scales up to hundreds of kilometers, **so that** designers working on large
open-world zones get reliable measurement data.

## F-15.1.7 Editor Preferences and Configuration

## US-15.1.7.1 Designer Rebinds Input

**As a** designer (P-5), **I want** to rebind keyboard shortcuts and configure viewport defaults,
grid settings, and auto-save intervals, **so that** the editor adapts to my personal workflow
preferences.

## US-15.1.7.2 Artist Switches Visual Theme

**As an** artist (P-8), **I want** to switch between light and dark editor themes, **so that** I can
reduce eye strain during long sessions and match my preferred desktop aesthetic.

## US-15.1.7.3 DevOps Propagates Team Defaults

**As a** DevOps engineer (P-16), **I want** preferences stored as versioned JSON and synced via
version control so team-wide defaults propagate, **so that** new team members start with the
studio's standard configuration without manual setup.

## US-15.1.7.4 Engine Dev Versions Preference Schema

**As an** engine developer (P-26), **I want** preference files versioned with schema migration,
**so that** upgrading the engine does not break or silently lose user preferences.

## F-15.1.8 Editor Extensions and Plugin API

## US-15.1.8.1 Developer Adds Custom Panel

**As a** developer (P-15), **I want** a stable plugin API to add custom panels, gizmos, importers,
context menu actions, and toolbar buttons, **so that** I can extend the editor for studio-specific
workflows without modifying engine source code.

## US-15.1.8.2 Tech Artist Hot-Reloads Plugin

**As a** tech artist (P-13), **I want** plugins hot-reloaded during development, **so that** I can
iterate on custom tools such as quest editors and loot table configurators without restarting the
editor.

## US-15.1.8.3 Modder Extends via Plugin

**As a** modder (P-24), **I want** the plugin API to be stable across engine versions, **so that**
my custom mod tools continue to work after game updates without rewriting them.

## US-15.1.8.4 Engine Dev Tests Plugin Isolation

**As an** engine developer (P-26), **I want** plugins to run in isolation from the editor core,
**so that** a buggy plugin cannot crash the editor or corrupt project data.

## F-15.1.9 VR Editor Mode

## US-15.1.9.1 Designer Places Objects in VR

**As a** designer (P-5), **I want** to grab, place, rotate, and scale objects with motion
controllers in an immersive VR editing mode, **so that** I can assemble environments with natural
hand gestures and experience them at true player scale.

## US-15.1.9.2 Artist Sculpts Terrain in VR

**As an** artist (P-8), **I want** to access terrain sculpting, painting, and placement tools via a
floating VR tool palette with laser pointer interaction, **so that** I can shape the environment
while immersed in it.

## US-15.1.9.3 Creative Director Previews at Player Scale

**As a** creative director (P-2), **I want** to walk through levels in VR at true player scale while
seeing the same changes reflected on the desktop monitor, **so that** I can evaluate spatial design
and atmosphere as the player will experience them.

## US-15.1.9.4 Engine Tester Verifies VR-Desktop Sync

**As an** engine tester (P-27), **I want** to verify that edits made in VR are immediately visible
on the desktop editor and vice versa, **so that** VR editing and desktop editing coexist without
sync issues.
