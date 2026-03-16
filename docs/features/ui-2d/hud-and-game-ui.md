# 10.3 — HUD & Game UI

## Player Status

### F-10.3.1 Health, Resource, and Cast Bars

Segmented and continuous bar widgets for displaying health, mana, energy, rage, and other player
resources. Cast bars show spell/ability progress with interrupt indicators. Bars support color
coding by resource type, animated drain/fill, predicted damage overlays (showing incoming damage
before it applies), and absorb shield overlays. Must render efficiently for raid frames displaying
40+ player bars simultaneously.

- **Requirements:** R-10.3.1
- **Dependencies:** F-10.1.1, F-10.1.6
- **Platform notes:** None

### F-10.3.2 Buff and Debuff Icons

Grid-based icon displays for active buffs, debuffs, and status effects with remaining duration
overlays (radial cooldown sweep and countdown text). Icons must support tooltip inspection showing
effect name, description, source, and remaining duration. The system must handle large icon counts
(30+ effects in MMO raids) with priority-based filtering and grouping (player buffs, raid buffs,
debuffs, dispellable effects).

- **Requirements:** R-10.3.2
- **Dependencies:** F-10.1.1, F-10.2.6, F-10.1.6
- **Platform notes:** None

### F-10.3.3 Action Bars and Cooldown Indicators

Configurable grids of ability slots bound to keyboard/gamepad inputs, displaying ability icons with
radial cooldown sweeps, charge counts, range/usability tinting, and keybind labels. Support multiple
action bar pages, stance/form-conditional bars, and macro icons. Cooldown indicators must be
frame-accurate with the game simulation to avoid abilities appearing ready before they are.

- **Requirements:** R-10.3.3
- **Dependencies:** F-10.1.1, F-10.1.6, F-10.2.7
- **Platform notes:** None

## World-Anchored UI

### F-10.3.4 Nameplates and World-Space Labels

Floating UI elements anchored to 3D world positions (above characters, NPCs, resource nodes) that
are projected to screen space each frame. Nameplates display name, guild, title, health bar, and
cast bar. Must scale to hundreds of visible nameplates in MMO cities and battlegrounds with
LOD-based culling (distance fade, importance-based filtering), occlusion culling against terrain,
and overlap avoidance to prevent stacking.

- **Requirements:** R-10.3.4
- **Dependencies:** F-10.1.1, F-10.3.1, F-10.4.1
- **Platform notes:** Mobile caps visible nameplates lower (50 vs 200 on desktop) with more
  aggressive distance culling to stay within widget and draw call budgets.

### F-10.3.5 Floating Combat Text and Damage Numbers

Animated text elements spawned at world positions showing damage dealt, healing received, experience
gained, and other combat feedback. Numbers animate with configurable trajectories (rise-and-fade,
arcing, sticky), merge repeated hits into cumulative totals, and color-code by damage type
(physical, fire, frost, etc.). Must handle burst scenarios with dozens of simultaneous damage events
without visual clutter.

- **Requirements:** R-10.3.5
- **Dependencies:** F-10.1.2, F-10.3.4, F-10.4.2
- **Platform notes:** None

## Navigation and Tracking

### F-10.3.6 Minimap and World Map Overlays

Circular or rectangular minimap widget rendering a top-down view of the local area with icons for
party members, NPCs, quest objectives, resource nodes, and points of interest. The full world map
supports zoom, pan, fog-of-war, zone boundaries, waypoint placement, flight path overlays, and
dungeon floor plans. Both map views must update marker positions in real time for moving entities.

- **Requirements:** R-10.3.6
- **Dependencies:** F-10.1.1, F-10.4.4
- **Platform notes:** None

### F-10.3.7 Quest Tracker and Objective HUD

Persistent on-screen widget listing active quest objectives with progress indicators (kill counts,
collection progress, zone completion percentages). Supports world-space waypoint markers,
directional compass indicators, and distance displays for tracked objectives. The tracker must
handle complex MMO quest structures: multi-step chains, branching objectives, daily/weekly resets,
and group quest shared progress.

- **Requirements:** R-10.3.7
- **Dependencies:** F-10.1.1, F-10.1.6, F-10.3.6
- **Platform notes:** None

## Communication

### F-10.3.8 Chat System

Full-featured chat window supporting multiple channels (say, party, guild, raid, whisper, trade,
general, custom), tabbed channel views, scrollable message history with timestamps, clickable player
name and item links, and inline icon/emoji support. Moderation features include profanity filtering,
spam throttling, ignore lists, and report functionality. Must handle high message throughput in MMO
cities where hundreds of players chat simultaneously.

- **Requirements:** R-10.3.8
- **Dependencies:** F-10.2.1, F-10.2.2, F-10.2.5, F-10.1.6
- **Platform notes:** None

## Inventory and Economy

### F-10.3.9 Inventory Grids and Container Management

Grid-based inventory display with drag-and-drop item movement, stack splitting, item sorting (by
name, type, rarity, level), search filtering, bag/tab organization, and visual item quality borders.
Supports multiple container types: player bags, bank tabs, guild bank, vendor buy/sell, loot
windows, trade windows, and mail attachments. All container views share the same grid widget with
context-specific behavior.

- **Requirements:** R-10.3.9
- **Dependencies:** F-10.1.1, F-10.2.7, F-10.2.5, F-10.2.6
- **Platform notes:** None

## Compass and Bearing

### F-10.3.10 Compass Bar HUD

A Skyrim-style horizontal compass strip at the top of the screen showing cardinal/intercardinal
directions and directional markers for tracked objectives. The compass rotates with the player's
facing direction. Marker types: quest objectives (icon + distance), custom waypoints (player- placed
pins), enemy indicators (combat targets), party members (when off-screen), points of interest
(discovered locations), and custom markers (from gameplay logic graphs F-15.8.4). Markers stack
vertically when overlapping on the same bearing and fade at configurable distance thresholds. The
compass supports both 2D (top-down) and 3D (first/third-person) camera modes. Compass style (full
strip, arc, minimal dot) is configurable per project. On mobile, the compass can be replaced with or
augmented by edge-of-screen directional arrows (off-screen indicators) for tracked objectives.

- **Requirements:** R-10.3.10
- **Dependencies:** F-10.1.1 (Widget Tree), F-10.3.7 (Quest Tracker), F-6.2.1 (Input Actions)
- **Platform notes:** Mobile uses simplified compass with fewer markers to avoid clutter.

### F-10.3.11 Off-Screen Objective Indicators

HUD arrows or chevrons at screen edges pointing toward tracked objectives that are outside the
viewport. Indicators show: direction to the objective, distance (text or bar), objective icon, and
priority-based coloring (main quest gold, side quest silver, party member blue, enemy red).
Indicators clamp to the screen edge and smoothly transition to the in-world marker when the
objective enters the viewport. Multiple off-screen indicators stack along the screen edge with
overlap avoidance. The system works with both 3D perspectives and 2D top-down cameras. Indicator
visibility is controlled per-marker (some objectives show only on the compass, others show
off-screen arrows, both, or neither).

- **Requirements:** R-10.3.11
- **Dependencies:** F-10.3.10 (Compass), F-10.3.7 (Quest Tracker), F-2.10.4 (View Setup)
- **Platform notes:** None

## Map System

### F-10.3.12 Procedural Minimap Generation

Automatically generate the minimap from world data without artist intervention. The minimap renderer
captures a top-down orthographic view of the terrain (F-3.2.1), colorized by biome/ material type:
green for grass, brown for dirt, blue for water, grey for stone, white for snow. Building footprints
are rendered as filled shapes from collision geometry. Roads appear as lines from the road spline
data (F-3.6.15). The generated minimap updates as the world loads — newly streamed chunks extend the
minimap. For voxel worlds (F-3.2.9), the minimap shows a slice at the player's current elevation
with cave/tunnel outlines. For procedurally generated worlds, the minimap generates as the player
explores, revealing terrain progressively (integrating with fog of war F-13.20.1 if enabled). The
minimap is rendered to a texture that the minimap widget (F-10.3.6) displays with rotation, zoom,
and icon overlay.

- **Requirements:** R-10.3.12
- **Dependencies:** F-10.3.6 (Minimap Widget), F-3.2.1 (Terrain), F-13.20.1 (Fog of War), F-3.6.15
  (Road Network)
- **Platform notes:** Minimap texture resolution scales with platform (256x256 mobile, 512x512
  desktop).

### F-10.3.13 World Map Generation and Rendering

Generate a full world map from terrain, biome, political, and infrastructure data. Map layers:
**terrain** (heightmap-derived elevation shading with relief), **biome** (color-coded regions),
**political** (faction territory borders and colors from F-3.6.36), **roads and paths** (from road
network F-3.6.15), **settlements** (city/town/village icons from F-3.6.35), **water** (rivers,
lakes, oceans), **fog of war** (unexplored regions hidden/shrouded from F-13.20.1), and
**player annotations** (waypoints, notes, custom pins). The world map supports: continuous zoom from
world overview to local detail, smooth panning, click-to-set-waypoint, click-to- fast-travel
(F-13.24.5), and search-by-name for discovered locations. For planet-scale worlds (F-3.2.11), the
world map renders on a globe with zoom-to-surface transitions. Map data is generated from the
procedural generation output (F-3.6.x) and cached as tiled image pyramids for efficient zoom
rendering.

- **Requirements:** R-10.3.13
- **Dependencies:** F-10.3.6 (Minimap Widget), F-3.6.36 (Factions), F-3.6.35 (Cities), F-13.20.1
  (Fog of War), F-13.24.5 (Fast Travel)
- **Platform notes:** Mobile uses lower-resolution tiled image pyramids and fewer zoom levels to
  reduce GPU memory. Planet-scale globe rendering is desktop/console only.

### F-10.3.14 Artist-Authored Map Overlays and Post-Processing

Support hand-painted or stylized map presentations layered on top of the generated map data. Artists
create map overlay assets: parchment textures, hand-drawn coastlines, illustrated landmarks,
decorative compass roses, sea monster illustrations, and artistic border frames. Post-processing
filters transform the generated map into stylized presentations: **parchment** (sepia tone,
ink-drawn lines, aged paper texture), **painted** (watercolor wash, artistic terrain coloring),
**tactical** (high-contrast, military-style contour lines), **satellite** (photorealistic aerial
view from terrain materials), and **schematic** (flat colors, clean lines, labeled grid). Artists
compose these in the visual editor by layering overlay assets on the generated map and selecting a
post-processing style. Static maps (hand-painted images for specific zones) can replace the
generated map entirely for areas requiring maximum artistic control. The map rendering pipeline
blends static artistic layers with dynamic data layers (player position, quest markers, fog of war)
so artistic maps remain functional.

- **Requirements:** R-10.3.14
- **Dependencies:** F-10.3.13 (World Map), F-10.4.4 (UI Atlas), F-2.9.1 (Post-Processing)
- **Platform notes:** None

### F-10.3.15 Dynamic Map Markers and Quest Integration

A data-driven marker system for both the minimap and world map with deep quest integration. Marker
categories: **quest objectives** (main quest, side quest, daily — with progress indicators),
**NPCs** (vendors, quest givers, trainers — filtered by type), **resources** (mining nodes, herbs,
fishing spots — shown when the profession is active), **party/raid** (member positions, pings,
drawing), **enemies** (from perception or fog of war reveal), **player waypoints** (custom pins with
labels and colors), **discovered POIs** (with completion percentage), and **dynamic events** (world
bosses, invasions, seasonal events with countdown timers). Markers are defined as data assets with:
icon, color, category, visibility rules (always visible, only when tracked, only in specific zoom
range), and source (static placement, quest system F-13.6.1, procedural F-3.6.40, player-created).
Markers appear on BOTH the minimap and the compass (F-10.3.10) simultaneously with consistent icons.
The quest tracker (F-10.3.7) drives marker visibility — tracking a quest shows its markers,
untracking hides them. Markers support clustering at zoom levels where individual icons would
overlap.

- **Requirements:** R-10.3.15
- **Dependencies:** F-10.3.6 (Minimap), F-10.3.10 (Compass), F-10.3.7 (Quest Tracker), F-13.6.1
  (Quest System), F-3.6.40 (Creature Placement)
- **Platform notes:** Marker count is capped per view to maintain performance. Mobile shows fewer
  simultaneous markers.
