# User Stories: Editor Framework

## F-15.1.1 Dockable Panel Layout

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-15.1.1.1 | designer (P-5)          |          |              |
| US-15.1.1.2 | designer (P-5)          |          |              |
| US-15.1.1.3 | artist (P-8)            |          |              |
| US-15.1.1.4 | developer (P-15)        |          |              |
| US-15.1.1.5 | tech artist (P-13)      |          |              |
| US-15.1.1.6 | engine developer (P-26) |          |              |
| US-15.1.1.7 | engine tester (P-27)    |          |              |
| US-15.1.1.8 | creative director (P-2) |          |              |

1. **US-15.1.1.1** — to drag, drop, split, tab, and float editor panels into a custom workspace
   layout
   - **Acceptance:** I can arrange level design tools exactly where I need them without wasting
     screen space on panels I rarely use
2. **US-15.1.1.2** — to save and switch between named layout profiles (e.g., "Level Design",
   "Scripting", "Playtesting")
   - **Acceptance:** I can instantly reconfigure the editor for different tasks without rearranging
     panels each time
3. **US-15.1.1.3** — to float a material preview panel onto a second monitor while keeping the
   viewport on my primary display
   - **Acceptance:** I can compare my in-engine material to reference images side by side
4. **US-15.1.1.4** — layout profiles stored as versioned JSON that syncs through version control
   - **Acceptance:** the entire team inherits consistent workspace defaults while I keep my personal
     overrides local
5. **US-15.1.1.5** — to create a layout profile that combines the shader graph, profiler, and
   viewport in a single arrangement
   - **Acceptance:** I can debug shader performance without switching between separate workflows
6. **US-15.1.1.6** — panel layout state to survive editor restarts and crash recovery
   - **Acceptance:** users never lose their workspace arrangement due to an unexpected shutdown
7. **US-15.1.1.7** — to verify that floating panels integrate with native window management
   (NSWindow on macOS, virtual desktops on Windows)
   - **Acceptance:** panels behave like native windows on each platform
8. **US-15.1.1.8** — a "Review" workspace profile that maximizes the viewport with minimal toolbars
   - **Acceptance:** I can evaluate the game's visual quality during art reviews without UI clutter

## F-15.1.2 Multi-Viewport

| ID          | Persona              | Features | Requirements |
|-------------|----------------------|----------|--------------|
| US-15.1.2.1 | designer (P-5)       |          |              |
| US-15.1.2.2 | artist (P-8)         |          |              |
| US-15.1.2.3 | developer (P-15)     |          |              |
| US-15.1.2.4 | engine tester (P-27) |          |              |

1. **US-15.1.2.1** — to display top-down and perspective viewports side by side with independent
   cameras
   - **Acceptance:** I can verify entity placement from both tactical and player perspectives
     simultaneously
2. **US-15.1.2.2** — one viewport showing the player camera preview alongside my free-fly editing
   viewport
   - **Acceptance:** I can see exactly what the player sees while I adjust environment art
3. **US-15.1.2.3** — multiple viewports to render through the same render graph as the game with
   independent render settings
   - **Acceptance:** I can test split-screen multiplayer directly in the editor
4. **US-15.1.2.4** — to verify that GPU memory scales linearly with viewport count and each viewport
   allocates its own swapchain
   - **Acceptance:** multi-viewport usage does not cause unexpected memory spikes or resource
     conflicts

## F-15.1.3 Undo/Redo System (Command Pattern)

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-15.1.3.1 | designer (P-5)          |          |              |
| US-15.1.3.2 | artist (P-8)            |          |              |
| US-15.1.3.3 | developer (P-15)        |          |              |
| US-15.1.3.4 | engine developer (P-26) |          |              |

1. **US-15.1.3.1** — multi-entity moves batched into a single undo step via transaction grouping
   - **Acceptance:** I can revert accidental batch transformations with one undo action instead of
     clicking undo once per entity
2. **US-15.1.3.2** — the undo/redo command history saved per session for crash recovery
   - **Acceptance:** I can restore my work after an unexpected editor crash without losing recent
     edits
3. **US-15.1.3.3** — every editor action captured as a serializable, reversible command
   - **Acceptance:** I can replay command sequences for automated testing and debugging
4. **US-15.1.3.4** — every command type to implement both execute and reverse operations correctly
   - **Acceptance:** the undo/redo stack never leaves the editor in an inconsistent state

## F-15.1.4 Selection System

| ID          | Persona            | Features | Requirements |
|-------------|--------------------|----------|--------------|
| US-15.1.4.1 | designer (P-5)     |          |              |
| US-15.1.4.2 | artist (P-8)       |          |              |
| US-15.1.4.3 | tech artist (P-13) |          |              |
| US-15.1.4.4 | developer (P-15)   |          |              |

1. **US-15.1.4.1** — to select entities using click, marquee, and lasso modes with additive and
   subtractive modifiers
   - **Acceptance:** I can quickly isolate groups of objects for batch operations
2. **US-15.1.4.2** — to save named selection sets for repeated operations on the same group of
   objects
   - **Acceptance:** I can recall complex selections instantly without re-selecting each entity
     manually
3. **US-15.1.4.3** — to select sub-object elements such as vertices, faces, and bones
   - **Acceptance:** I can perform fine-grained editing operations on mesh and skeleton data
     directly in the editor
4. **US-15.1.4.4** — to select entities programmatically through the editor API
   - **Acceptance:** my custom tools and plugins can manipulate selection state consistently with
     the built-in selection system

## F-15.1.5 Transform Gizmos

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-15.1.5.1 | designer (P-5)          |          |              |
| US-15.1.5.2 | artist (P-8)            |          |              |
| US-15.1.5.3 | creative director (P-2) |          |              |
| US-15.1.5.4 | engine tester (P-27)    |          |              |

1. **US-15.1.5.1** — translate, rotate, and scale gizmos with configurable per-axis snap increments
   - **Acceptance:** I can place entities at precise positions aligned to a grid without manual
     coordinate entry
2. **US-15.1.5.2** — gizmos that support axis constraints, plane constraints, and free-form
   manipulation in local, world, or custom reference frames
   - **Acceptance:** I can move objects along specific axes or planes during detailed environment
     assembly
3. **US-15.1.5.3** — gizmos to display the delta value during manipulation
   - **Acceptance:** I can see exactly how far and in which direction an object has been moved,
     rotated, or scaled
4. **US-15.1.5.4** — to verify that gizmo manipulations produce numerically accurate transforms
   matching the displayed delta values
   - **Acceptance:** designers can trust the visual feedback during precise placement

## F-15.1.6 Bounds and Measurement Gizmos

| ID          | Persona              | Features | Requirements |
|-------------|----------------------|----------|--------------|
| US-15.1.6.1 | designer (P-5)       |          |              |
| US-15.1.6.2 | artist (P-8)         |          |              |
| US-15.1.6.3 | developer (P-15)     |          |              |
| US-15.1.6.4 | engine tester (P-27) |          |              |

1. **US-15.1.6.1** — distance rulers, angle protractors, and area measurements in the viewport
   - **Acceptance:** I can verify gameplay metrics such as jump distances, corridor widths, and
     line-of-sight clearances
2. **US-15.1.6.2** — to display axis-aligned and oriented bounding boxes on selected entities
   - **Acceptance:** I can verify that collision bounds match visual geometry before finalizing
     placement
3. **US-15.1.6.3** — measurement gizmo values exposed through the editor API
   - **Acceptance:** my custom validation tools can read distances and dimensions programmatically
4. **US-15.1.6.4** — to verify that measurement gizmos produce accurate results at world scales up
   to hundreds of kilometers
   - **Acceptance:** designers working on large open-world zones get reliable measurement data

## F-15.1.7 Editor Preferences and Configuration

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-15.1.7.1 | designer (P-5)          |          |              |
| US-15.1.7.2 | artist (P-8)            |          |              |
| US-15.1.7.3 | DevOps engineer (P-16)  |          |              |
| US-15.1.7.4 | engine developer (P-26) |          |              |

1. **US-15.1.7.1** — to rebind keyboard shortcuts and configure viewport defaults, grid settings,
   and auto-save intervals
   - **Acceptance:** the editor adapts to my personal workflow preferences
2. **US-15.1.7.2** — to switch between light and dark editor themes
   - **Acceptance:** I can reduce eye strain during long sessions and match my preferred desktop
     aesthetic
3. **US-15.1.7.3** — preferences stored as versioned JSON and synced via version control so
   team-wide defaults propagate
   - **Acceptance:** new team members start with the studio's standard configuration without manual
     setup
4. **US-15.1.7.4** — preference files versioned with schema migration
   - **Acceptance:** upgrading the engine does not break or silently lose user preferences

## F-15.1.8 Editor Extensions and Plugin API

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-15.1.8.1 | developer (P-15)        |          |              |
| US-15.1.8.2 | tech artist (P-13)      |          |              |
| US-15.1.8.3 | modder (P-24)           |          |              |
| US-15.1.8.4 | engine developer (P-26) |          |              |

1. **US-15.1.8.1** — a stable plugin API to add custom panels, gizmos, importers, context menu
   actions, and toolbar buttons
   - **Acceptance:** I can extend the editor for studio-specific workflows without modifying engine
     source code
2. **US-15.1.8.2** — plugins hot-reloaded during development
   - **Acceptance:** I can iterate on custom tools such as quest editors and loot table
     configurators without restarting the editor
3. **US-15.1.8.3** — the plugin API to be stable across engine versions
   - **Acceptance:** my custom mod tools continue to work after game updates without rewriting them
4. **US-15.1.8.4** — plugins to run in isolation from the editor core
   - **Acceptance:** a buggy plugin cannot crash the editor or corrupt project data

## F-15.1.9 VR Editor Mode

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-15.1.9.1 | designer (P-5)          |          |              |
| US-15.1.9.2 | artist (P-8)            |          |              |
| US-15.1.9.3 | creative director (P-2) |          |              |
| US-15.1.9.4 | engine tester (P-27)    |          |              |

1. **US-15.1.9.1** — to grab, place, rotate, and scale objects with motion controllers in an
   immersive VR editing mode
   - **Acceptance:** I can assemble environments with natural hand gestures and experience them at
     true player scale
2. **US-15.1.9.2** — to access terrain sculpting, painting, and placement tools via a floating VR
   tool palette with laser pointer interaction
   - **Acceptance:** I can shape the environment while immersed in it
3. **US-15.1.9.3** — to walk through levels in VR at true player scale while seeing the same changes
   reflected on the desktop monitor
   - **Acceptance:** I can evaluate spatial design and atmosphere as the player will experience them
4. **US-15.1.9.4** — to verify that edits made in VR are immediately visible on the desktop editor
   and vice versa
   - **Acceptance:** VR editing and desktop editing coexist without sync issues
