# R-10.3 — HUD and Game UI Requirements

## R-10.3.1 Health, Resource, and Cast Bars

The engine **SHALL** render segmented and continuous bar widgets for health, mana, and other
resources with animated fill/drain, predicted-damage overlays, and absorb-shield overlays, at
a sustained 60 FPS when displaying 40 or more player bars simultaneously in a raid frame view.

- **Derived from:** [F-10.3.1](../../features/ui-2d/hud-and-game-ui.md)
- **Rationale:** MMO raid encounters require all party/raid member bars visible at once; sluggish
  bar updates or frame drops compromise healers' ability to react to incoming damage.
- **Verification:** Performance test — render a raid frame panel containing 40 bar widgets each
  animating drain/fill, predicted-damage overlay, and absorb overlay simultaneously; measure
  frame time and assert it remains below 16.67 ms for 300 consecutive frames.

## R-10.3.2 Buff and Debuff Icons

The engine **SHALL** display grid-based buff and debuff icon arrays with radial cooldown sweep
duration overlays, countdown text, tooltip inspection, and priority-based filtering that handles
30 or more simultaneous status effects without layout overflow or dropped icons.

- **Derived from:** [F-10.3.2](../../features/ui-2d/hud-and-game-ui.md)
- **Rationale:** Players must quickly identify dispellable debuffs and track buff timers during
  combat; missing or obscured icons lead to incorrect gameplay decisions.
- **Verification:** Unit test — apply 35 status effects with varying priorities and durations to
  an entity; assert the icon grid displays all icons grouped by category (player buffs, raid
  buffs, debuffs, dispellable), each showing correct remaining duration via sweep and text.

## R-10.3.3 Action Bars and Cooldown Indicators

The engine **SHALL** render configurable action bar grids with ability icons, frame-accurate
radial cooldown sweeps, charge counts, range/usability tinting, and keybind labels, where
cooldown completion is synchronized to the game simulation tick with no more than one frame of
display error.

- **Derived from:** [F-10.3.3](../../features/ui-2d/hud-and-game-ui.md)
- **Rationale:** Frame-accurate cooldown display prevents players from pressing abilities that
  appear ready but are still on cooldown, avoiding wasted input and frustration.
- **Verification:** Integration test — start a 5-second cooldown on an ability slot at a known
  simulation tick; assert the radial sweep reaches zero and usability tinting updates within
  one frame of the simulation tick at which the cooldown expires.

## R-10.3.4 Nameplates and World-Space Labels

The engine **SHALL** render floating nameplates anchored to 3D world positions, projected to
screen space each frame, with LOD-based distance fade, importance-based culling, terrain
occlusion culling, and overlap avoidance, scaling to 200 or more visible nameplates at 60 FPS.

- **Derived from:** [F-10.3.4](../../features/ui-2d/hud-and-game-ui.md)
- **Rationale:** MMO cities and battlegrounds contain hundreds of characters; without culling
  and overlap avoidance, nameplates stack illegibly and tank frame rate.
- **Verification:** Performance test — spawn 250 entities with nameplates at varying distances
  and depths; assert frame time stays below 16.67 ms, no two nameplates overlap by more than
  10% of their area, and occluded nameplates are not drawn.

## R-10.3.5 Floating Combat Text and Damage Numbers

The engine **SHALL** spawn animated floating text elements at world positions with configurable
trajectories (rise-and-fade, arcing, sticky), damage-type color coding, and cumulative merging
of repeated hits, handling 50 or more simultaneous damage events without visual clutter or
frame drops.

- **Derived from:** [F-10.3.5](../../features/ui-2d/hud-and-game-ui.md)
- **Rationale:** Burst damage scenarios in AoE combat produce dozens of damage numbers per
  frame; without merging and trajectory spreading, the text becomes unreadable.
- **Verification:** Stress test — emit 60 damage events within a single frame targeting the
  same world position; assert all events are displayed or merged into cumulative totals,
  no text overlaps illegibly, and frame time remains below 16.67 ms.

## R-10.3.6 Minimap and World Map Overlays

The engine **SHALL** render a minimap widget with a real-time top-down view of the local area
showing icons for party members, NPCs, quest objectives, and points of interest, and a full
world map with zoom, pan, fog-of-war, zone boundaries, and waypoint placement, with all marker
positions updating each frame for moving entities.

- **Derived from:** [F-10.3.6](../../features/ui-2d/hud-and-game-ui.md)
- **Rationale:** The minimap is the primary spatial awareness tool; stale marker positions cause
  players to lose track of party members and objectives.
- **Verification:** Integration test — place 20 moving entities in the world updating position
  each tick; assert minimap icon positions match projected world positions within 1 pixel after
  each frame, and world map fog-of-war correctly reveals explored regions.

## R-10.3.7 Quest Tracker and Objective HUD

The engine **SHALL** display a persistent on-screen quest tracker listing active objectives with
progress indicators, world-space waypoint markers, directional compass indicators, and distance
displays, supporting multi-step quest chains, branching objectives, and group-shared progress.

- **Derived from:** [F-10.3.7](../../features/ui-2d/hud-and-game-ui.md)
- **Rationale:** Complex quest structures with branching paths and group progress require
  clear visual tracking so players always know their next objective.
- **Verification:** Integration test — create a quest chain with 3 sequential steps, a branch
  at step 2, and group-shared kill progress; assert the tracker updates step completion,
  displays the correct branch, shows shared kill count, and renders a waypoint marker at the
  objective world position.

## R-10.3.8 Chat System

The engine **SHALL** provide a chat window supporting multiple named channels (say, party,
guild, raid, whisper, trade, general, and custom), tabbed views, scrollable message history
with timestamps, clickable player-name and item links, inline emoji support, and moderation
features including profanity filtering, spam throttling, and ignore lists, sustaining 200 or
more messages per second without dropped messages or UI stalls.

- **Derived from:** [F-10.3.8](../../features/ui-2d/hud-and-game-ui.md)
- **Rationale:** MMO city hubs generate extreme chat throughput; dropped messages or UI freezes
  during chat spam degrade the social experience.
- **Verification:** Throughput test — enqueue 250 chat messages per second across 5 channels
  for 10 seconds; assert all messages appear in the correct channel tabs, timestamps are
  monotonically increasing, profanity-filtered words are masked, and frame time stays below
  16.67 ms.

## R-10.3.9 Inventory Grids and Container Management

The engine **SHALL** render grid-based inventory displays with drag-and-drop item movement,
stack splitting, sorting by name/type/rarity/level, search filtering, and visual item-quality
borders, sharing a single grid widget across player bags, bank tabs, guild bank, vendor,
loot, trade, and mail attachment container contexts.

- **Derived from:** [F-10.3.9](../../features/ui-2d/hud-and-game-ui.md)
- **Rationale:** A unified grid widget across all container types reduces implementation
  surface, ensures consistent drag-and-drop behavior, and simplifies player muscle memory.
- **Verification:** Integration test — open player bag, bank, and vendor containers
  simultaneously; drag an item from bag to vendor (sell), split a stack in the bank, sort bag
  by rarity, and filter by search term; assert each operation completes correctly with visual
  quality borders matching item rarity.

## R-10.3.10 Compass Bar HUD

The engine **SHALL** render a directional compass strip showing cardinal directions and
tracked objective markers that rotate with the player's facing. Markers **SHALL** show icon,
distance, and stack vertically when overlapping. The compass **SHALL** work in both 2D and 3D
camera modes. At least 3 compass styles **SHALL** be configurable (full strip, arc, minimal
dot).

- **Derived from:** [F-10.3.10](../../features/ui-2d/hud-and-game-ui.md)
- **Rationale:** A compass provides constant directional awareness without opening the map,
  essential for exploration-focused games.
- **Verification:** Track 5 objectives at different bearings; rotate the player; verify
  markers move correctly on the compass. Verify markers stack when overlapping. Switch
  between compass styles.

## R-10.3.11 Off-Screen Objective Indicators

The engine **SHALL** display directional arrows at screen edges pointing toward off-screen
tracked objectives with distance, icon, and priority-based coloring. Indicators **SHALL**
smoothly transition to in-world markers when the objective enters the viewport.

- **Derived from:** [F-10.3.11](../../features/ui-2d/hud-and-game-ui.md)
- **Rationale:** Off-screen indicators guide players without requiring map checks, maintaining
  gameplay flow.
- **Verification:** Track an objective behind the player; verify an edge arrow appears. Turn
  toward it; verify the arrow transitions to the in-world marker smoothly.

## R-10.3.12 Procedural Minimap Generation

The engine **SHALL** auto-generate minimap textures from world terrain, biome, building, and
road data without artist intervention. The minimap **SHALL** update as world chunks stream in.
For voxel worlds, the minimap **SHALL** show a slice at the player's elevation. Minimap
generation **SHALL** integrate with fog of war when enabled.

- **Derived from:** [F-10.3.12](../../features/ui-2d/hud-and-game-ui.md)
- **Rationale:** Procedural minimap generation eliminates manual map authoring for large and
  infinite worlds.
- **Verification:** Load a procedurally generated world; verify the minimap renders terrain,
  roads, and buildings. Stream a new chunk; verify the minimap extends. Enable fog of war;
  verify unexplored areas are hidden.

## R-10.3.13 World Map Generation and Rendering

The engine **SHALL** generate a multi-layer world map (terrain, biome, political, roads,
settlements, water, fog of war, annotations) with continuous zoom, panning, waypoint
placement, and fast travel integration. For planetary worlds, the map **SHALL** render on a
zoomable globe. Map data **SHALL** be cached as tiled image pyramids.

- **Derived from:** [F-10.3.13](../../features/ui-2d/hud-and-game-ui.md)
- **Rationale:** A comprehensive world map is the primary navigation tool for open-world and
  MMO games.
- **Verification:** Open the world map; verify all 7 layers render. Zoom from world to
  local; verify smooth transitions. Place a waypoint; verify it appears on the minimap and
  compass. Click a fast travel point; verify travel initiates.

## R-10.3.14 Artist-Authored Map Overlays and Post-Processing

The engine **SHALL** support layering hand-painted artistic assets on top of generated map
data. At least 5 post-processing styles **SHALL** be provided (parchment, painted, tactical,
satellite, schematic). Static artist-painted maps **SHALL** be substitutable for generated
maps while preserving dynamic marker functionality.

- **Derived from:** [F-10.3.14](../../features/ui-2d/hud-and-game-ui.md)
- **Rationale:** Artistic map styles reinforce the game's visual identity. Hand-painted maps
  for key zones provide maximum quality.
- **Verification:** Apply the parchment style; verify sepia tone and ink-line rendering.
  Replace a zone's map with a static image; verify quest markers and player position still
  overlay correctly.

## R-10.3.15 Dynamic Map Markers and Quest Integration

The engine **SHALL** display data-driven markers on both the minimap and compass
simultaneously with consistent icons. Markers **SHALL** support at least 8 categories (quest,
NPC, resource, party, enemy, waypoint, POI, event). Tracking a quest **SHALL** show its
markers; untracking **SHALL** hide them. Markers **SHALL** cluster at zoom levels where
individual icons overlap.

- **Derived from:** [F-10.3.15](../../features/ui-2d/hud-and-game-ui.md)
- **Rationale:** Consistent markers across minimap, compass, and world map give players
  reliable spatial awareness without needing to learn multiple icon sets.
- **Verification:** Track a quest; verify its markers appear on minimap, compass, and world
  map. Untrack; verify markers disappear from all three. Zoom out on the world map until
  markers overlap; verify clustering activates.
