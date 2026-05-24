# HUD and 2D

In-game heads-up displays, menus, and 2D rendering.

## What it covers

- HUD layouts: positioning health, ammo, objectives on-screen.
- Minimap: real-time view of game world.
- Objectives and quest markers: guiding player toward goals.
- Compass and waypoints: directional indicators.
- Damage indicators: showing damage direction and sources.
- Ability bars: skill cooldowns and availability.
- Inventory UI: displaying carried items and equipment.
- Menu systems: pause menus, settings, main menu.
- Transitions and animations: fade, slide, pop-in effects for UI elements.
- Screen-space 2D rendering: drawing UI primitives (lines, shapes, text) over 3D scene.

## Concepts

### HUD Layout and Real-Time Data

HUD widgets display real-time game state: health bar shows current HP; ammo counter shows rounds;
objective text shows current goal. HUD elements anchor to screen edges and corners, staying visible
regardless of viewport or resolution. HUD data refreshes each frame, reflecting game state changes
instantly.

### Minimap and World Representation

Minimap renders a small representation of the world: camera draws top-down view of nearby area into
a texture. Icons overlay on minimap: player position (center), enemies (red), objectives (gold).
Minimap zooms and pans to follow player. Minimap click-to-move enables player to click on minimap
to set waypoint.

### Guidance and Waypoints

Objectives display text describing current goals. Quest markers appear on-screen pointing toward
objectives. Compass shows objective direction (compass needle points toward goal). Waypoints appear
on-screen or in-world (golden markers). These guide player without handholding.

### Damage Feedback

Damage indicators show damage direction: arrow pointing toward damage source. Intensity indicates
damage severity. Damage UI fades after a few seconds. Multiple damage sources show multiple arrows.
This enables players to react to threats without overhead labels or screen-covering text.

### Menu System

Menus present options in list or grid layout. Pause menu pauses game (stops simulation) and presents
resume, settings, quit options. Settings menu adjusts graphics, audio, controls. Main menu appears at
game startup. Transitions (fade, slide) make menu changes smooth.

## How it fits

- See [widgets-and-framework.md](./widgets-and-framework.md) for UI widget fundamentals.
- See [accessibility.md](./accessibility.md) for accessible information presentation.
- See [../rendering/effects-and-styles.md](../rendering/effects-and-styles.md) for UI rendering
  and post-processing.
