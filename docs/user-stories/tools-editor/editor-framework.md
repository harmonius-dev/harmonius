# User Stories: Editor Framework

## F-15.1.1 Dockable Panel Layout

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.1.1.1 | designer (P-5) | to drag, drop, split, tab, and float editor panels into a custom workspace layout | I can arrange level design tools exactly where I need them without wasting screen space on panels I rarely use |  |  |
| US-15.1.1.2 | designer (P-5) | to save and switch between named layout profiles (e.g., "Level Design", "Scripting", "Playtesting") | I can instantly reconfigure the editor for different tasks without rearranging panels each time |  |  |
| US-15.1.1.3 | artist (P-8) | to float a material preview panel onto a second monitor while keeping the viewport on my primary display | I can compare my in-engine material to reference images side by side |  |  |
| US-15.1.1.4 | developer (P-15) | layout profiles stored as versioned JSON that syncs through version control | the entire team inherits consistent workspace defaults while I keep my personal overrides local |  |  |
| US-15.1.1.5 | tech artist (P-13) | to create a layout profile that combines the shader graph, profiler, and viewport in a single arrangement | I can debug shader performance without switching between separate workflows |  |  |
| US-15.1.1.6 | engine developer (P-26) | panel layout state to survive editor restarts and crash recovery | users never lose their workspace arrangement due to an unexpected shutdown |  |  |
| US-15.1.1.7 | engine tester (P-27) | to verify that floating panels integrate with native window management (NSWindow on macOS, virtual desktops on Windows) | panels behave like native windows on each platform |  |  |
| US-15.1.1.8 | creative director (P-2) | a "Review" workspace profile that maximizes the viewport with minimal toolbars | I can evaluate the game's visual quality during art reviews without UI clutter |  |  |

## F-15.1.2 Multi-Viewport

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.1.2.1 | designer (P-5) | to display top-down and perspective viewports side by side with independent cameras | I can verify entity placement from both tactical and player perspectives simultaneously |  |  |
| US-15.1.2.2 | artist (P-8) | one viewport showing the player camera preview alongside my free-fly editing viewport | I can see exactly what the player sees while I adjust environment art |  |  |
| US-15.1.2.3 | developer (P-15) | multiple viewports to render through the same render graph as the game with independent render settings | I can test split-screen multiplayer directly in the editor |  |  |
| US-15.1.2.4 | engine tester (P-27) | to verify that GPU memory scales linearly with viewport count and each viewport allocates its own swapchain | multi-viewport usage does not cause unexpected memory spikes or resource conflicts |  |  |

## F-15.1.3 Undo/Redo System (Command Pattern)

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.1.3.1 | designer (P-5) | multi-entity moves batched into a single undo step via transaction grouping | I can revert accidental batch transformations with one undo action instead of clicking undo once per entity |  |  |
| US-15.1.3.2 | artist (P-8) | the undo/redo command history saved per session for crash recovery | I can restore my work after an unexpected editor crash without losing recent edits |  |  |
| US-15.1.3.3 | developer (P-15) | every editor action captured as a serializable, reversible command | I can replay command sequences for automated testing and debugging |  |  |
| US-15.1.3.4 | engine developer (P-26) | every command type to implement both execute and reverse operations correctly | the undo/redo stack never leaves the editor in an inconsistent state |  |  |

## F-15.1.4 Selection System

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.1.4.1 | designer (P-5) | to select entities using click, marquee, and lasso modes with additive and subtractive modifiers | I can quickly isolate groups of objects for batch operations |  |  |
| US-15.1.4.2 | artist (P-8) | to save named selection sets for repeated operations on the same group of objects | I can recall complex selections instantly without re-selecting each entity manually |  |  |
| US-15.1.4.3 | tech artist (P-13) | to select sub-object elements such as vertices, faces, and bones | I can perform fine-grained editing operations on mesh and skeleton data directly in the editor |  |  |
| US-15.1.4.4 | developer (P-15) | to select entities programmatically through the editor API | my custom tools and plugins can manipulate selection state consistently with the built-in selection system |  |  |

## F-15.1.5 Transform Gizmos

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.1.5.1 | designer (P-5) | translate, rotate, and scale gizmos with configurable per-axis snap increments | I can place entities at precise positions aligned to a grid without manual coordinate entry |  |  |
| US-15.1.5.2 | artist (P-8) | gizmos that support axis constraints, plane constraints, and free-form manipulation in local, world, or custom reference frames | I can move objects along specific axes or planes during detailed environment assembly |  |  |
| US-15.1.5.3 | creative director (P-2) | gizmos to display the delta value during manipulation | I can see exactly how far and in which direction an object has been moved, rotated, or scaled |  |  |
| US-15.1.5.4 | engine tester (P-27) | to verify that gizmo manipulations produce numerically accurate transforms matching the displayed delta values | designers can trust the visual feedback during precise placement |  |  |

## F-15.1.6 Bounds and Measurement Gizmos

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.1.6.1 | designer (P-5) | distance rulers, angle protractors, and area measurements in the viewport | I can verify gameplay metrics such as jump distances, corridor widths, and line-of-sight clearances |  |  |
| US-15.1.6.2 | artist (P-8) | to display axis-aligned and oriented bounding boxes on selected entities | I can verify that collision bounds match visual geometry before finalizing placement |  |  |
| US-15.1.6.3 | developer (P-15) | measurement gizmo values exposed through the editor API | my custom validation tools can read distances and dimensions programmatically |  |  |
| US-15.1.6.4 | engine tester (P-27) | to verify that measurement gizmos produce accurate results at world scales up to hundreds of kilometers | designers working on large open-world zones get reliable measurement data |  |  |

## F-15.1.7 Editor Preferences and Configuration

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.1.7.1 | designer (P-5) | to rebind keyboard shortcuts and configure viewport defaults, grid settings, and auto-save intervals | the editor adapts to my personal workflow preferences |  |  |
| US-15.1.7.2 | artist (P-8) | to switch between light and dark editor themes | I can reduce eye strain during long sessions and match my preferred desktop aesthetic |  |  |
| US-15.1.7.3 | DevOps engineer (P-16) | preferences stored as versioned JSON and synced via version control so team-wide defaults propagate | new team members start with the studio's standard configuration without manual setup |  |  |
| US-15.1.7.4 | engine developer (P-26) | preference files versioned with schema migration | upgrading the engine does not break or silently lose user preferences |  |  |

## F-15.1.8 Editor Extensions and Plugin API

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.1.8.1 | developer (P-15) | a stable plugin API to add custom panels, gizmos, importers, context menu actions, and toolbar buttons | I can extend the editor for studio-specific workflows without modifying engine source code |  |  |
| US-15.1.8.2 | tech artist (P-13) | plugins hot-reloaded during development | I can iterate on custom tools such as quest editors and loot table configurators without restarting the editor |  |  |
| US-15.1.8.3 | modder (P-24) | the plugin API to be stable across engine versions | my custom mod tools continue to work after game updates without rewriting them |  |  |
| US-15.1.8.4 | engine developer (P-26) | plugins to run in isolation from the editor core | a buggy plugin cannot crash the editor or corrupt project data |  |  |

## F-15.1.9 VR Editor Mode

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.1.9.1 | designer (P-5) | to grab, place, rotate, and scale objects with motion controllers in an immersive VR editing mode | I can assemble environments with natural hand gestures and experience them at true player scale |  |  |
| US-15.1.9.2 | artist (P-8) | to access terrain sculpting, painting, and placement tools via a floating VR tool palette with laser pointer interaction | I can shape the environment while immersed in it |  |  |
| US-15.1.9.3 | creative director (P-2) | to walk through levels in VR at true player scale while seeing the same changes reflected on the desktop monitor | I can evaluate spatial design and atmosphere as the player will experience them |  |  |
| US-15.1.9.4 | engine tester (P-27) | to verify that edits made in VR are immediately visible on the desktop editor and vice versa | VR editing and desktop editing coexist without sync issues |  |  |
