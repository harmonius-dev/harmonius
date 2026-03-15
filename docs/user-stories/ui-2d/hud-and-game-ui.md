# User Stories — 10.3 HUD & Game UI

## US-10.3.1 Monitor Health and Resources During Intense Combat

**As a** player (P-23), **I want** segmented and continuous health/mana/energy bars with animated
drain/fill, predicted damage overlays, and absorb shield indicators, **so that** I can monitor my
character's status at a glance during fast-paced combat.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Color-coded bars per resource type | F-10.3.1 | R-10.3.1 |
| Animated drain/fill transitions | F-10.3.1 | R-10.3.1 |
| Predicted damage overlay visible before hit applies | F-10.3.1 | R-10.3.1 |
| 40+ raid bars render at 60fps simultaneously | F-10.3.1 | R-10.3.1 |

## US-10.3.2 Design Stylized Health Bar Themes

**As an** artist (P-8), **I want** to customize health and resource bar visuals including
segmentation, fill colors, border styles, and absorb overlay graphics, **so that** bars match the
game's visual identity for different factions and character classes.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Segmented and continuous bar variants | F-10.3.1 | R-10.3.1 |
| Custom fill colors and border styles per resource type | F-10.3.1 | R-10.3.1 |
| Cast bar with interrupt indicator styling | F-10.3.1 | R-10.3.1 |

## US-10.3.3 Benchmark Raid Frame Rendering Performance

**As an** engine tester (P-27), **I want** to benchmark rendering of 40+ player health, mana, and
cast bars simultaneously, **so that** I can verify raid UI stays within the 2ms GPU budget during
worst-case raid encounters.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| 40 health bars + 40 mana bars + cast bars at 60fps | F-10.3.1 | R-10.3.1 |
| GPU time under 2ms for all raid bars combined | F-10.3.1 | R-X.12.1 |

## US-10.3.4 Track Active Buffs and Debuffs With Duration Overlays

**As a** player (P-23), **I want** grid-based buff and debuff icons with radial cooldown sweeps,
countdown text, and tooltip inspection, **so that** I can monitor 30+ active effects with priority
filtering during raids.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Radial cooldown sweep and countdown text on icons | F-10.3.2 | R-10.3.2 |
| Tooltip shows effect name, description, source, duration | F-10.3.2 | R-10.3.2 |
| Priority-based filtering and grouping for 30+ icons | F-10.3.2 | R-10.3.2 |

## US-10.3.5 Configure Buff Icon Grouping and Priority

**As a** designer (P-5), **I want** to configure how buff and debuff icons are grouped, prioritized,
and filtered (player buffs, raid buffs, debuffs, dispellable), **so that** the most important
effects are always visible even with many active.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Configurable grouping categories | F-10.3.2 | R-10.3.2 |
| Priority-based display ordering | F-10.3.2 | R-10.3.2 |
| Filtering by buff type (player, raid, debuff, dispellable) | F-10.3.2 | R-10.3.2 |

## US-10.3.6 Use Frame-Accurate Cooldown Indicators on Action Bars

**As a** player (P-23), **I want** configurable action bar grids with radial cooldown sweeps, charge
counts, range/usability tinting, and keybind labels, **so that** I know exactly when abilities are
ready without frame-inaccurate visual delays.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Radial cooldown sweeps frame-accurate with simulation | F-10.3.3 | R-10.3.3 |
| Charge counts and range/usability tinting | F-10.3.3 | R-10.3.3 |
| Multiple action bar pages and stance-conditional bars | F-10.3.3 | R-10.3.3 |
| Keybind labels displayed per slot | F-10.3.3 | R-10.3.3 |

## US-10.3.7 Verify Cooldown Indicator Timing Accuracy

**As an** engine tester (P-27), **I want** to verify that action bar cooldown indicators are
frame-accurate with the game simulation, **so that** abilities never appear ready before they
actually are and no visual desync occurs.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Cooldown visual matches simulation timer within 1 frame | F-10.3.3 | R-10.3.3 |
| No ability appears ready before actual cooldown expires | F-10.3.3 | R-10.3.3 |
| Frame-accuracy maintained under high system load | F-10.3.3 | R-10.3.3 |

## US-10.3.8 See Nameplates Above Characters in Crowded Cities

**As a** player (P-23), **I want** floating nameplates above characters and NPCs showing name,
guild, health bar, and cast bar with overlap avoidance and distance-based culling, **so that** I can
identify entities in crowded MMO cities without visual clutter.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Nameplates display name, guild, title, health, cast bar | F-10.3.4 | R-10.3.4 |
| LOD-based culling with distance fade | F-10.3.4 | R-10.3.4 |
| Overlap avoidance prevents stacking | F-10.3.4 | R-10.3.4 |
| 200+ nameplates on desktop, 50 on mobile | F-10.3.4 | R-10.3.4 |

## US-10.3.9 Stress-Test Nameplate Rendering in Dense Populations

**As an** engine tester (P-27), **I want** to stress-test nameplate rendering with 200+ visible
entities to verify overlap avoidance, occlusion culling, and draw call budget, **so that**
performance holds in worst-case city scenarios.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| 200 simultaneous nameplates render within GPU budget | F-10.3.4 | R-10.3.4 |
| Occlusion culling hides nameplates behind terrain | F-10.3.4 | R-10.3.4 |
| Overlap avoidance correct with rapid entity movement | F-10.3.4 | R-10.3.4 |

## US-10.3.10 See Floating Damage Numbers During Combat

**As a** player (P-23), **I want** animated damage numbers, healing numbers, and experience gains
that spawn at world positions with color-coding by damage type and merge repeated hits, **so that**
combat feedback is clear even during burst damage scenarios.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Damage, healing, and XP numbers animate from world positions | F-10.3.5 | R-10.3.5 |
| Color-coded by damage type (physical, fire, frost) | F-10.3.5 | R-10.3.5 |
| Repeated hits merge into cumulative totals | F-10.3.5 | R-10.3.5 |
| Configurable trajectories (rise-and-fade, arcing, sticky) | F-10.3.5 | R-10.3.5 |

## US-10.3.11 Configure Damage Number Styles and Trajectories

**As a** designer (P-5), **I want** to configure damage number fonts, colors per damage type,
animation trajectories, merge behavior, and burst handling, **so that** combat text feedback matches
the game's visual style and remains readable during intense encounters.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Per-damage-type color and font size configuration | F-10.3.5 | R-10.3.5 |
| Trajectory selection per number type | F-10.3.5 | R-10.3.5 |
| Merge threshold configurable for repeated hits | F-10.3.5 | R-10.3.5 |

## US-10.3.12 Navigate Using Minimap and World Map

**As a** player (P-23), **I want** a minimap showing party members, NPCs, quest objectives, and
POIs, plus a full world map with zoom, pan, fog-of-war, and waypoint placement, **so that** I can
navigate and track objectives without getting lost.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Circular or rectangular minimap with real-time icon updates | F-10.3.6 | R-10.3.6 |
| World map with zoom, pan, and fog-of-war | F-10.3.6 | R-10.3.6 |
| Waypoint placement on both map views | F-10.3.6 | R-10.3.6 |
| Dungeon floor plans supported | F-10.3.6 | R-10.3.6 |

## US-10.3.13 Track Quests With On-Screen Objectives and Waypoints

**As a** player (P-23), **I want** a persistent quest tracker showing active objectives with
progress indicators, world-space waypoint markers, compass indicators, and distance displays, **so
that** I always know what to do next and where to go.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Active objectives with kill/collection progress | F-10.3.7 | R-10.3.7 |
| World-space waypoint markers for tracked objectives | F-10.3.7 | R-10.3.7 |
| Multi-step chains and branching objectives supported | F-10.3.7 | R-10.3.7 |
| Daily/weekly resets and group shared progress | F-10.3.7 | R-10.3.7 |

## US-10.3.14 Design Quest Tracker Layouts for Complex Objectives

**As a** designer (P-5), **I want** to design quest tracker layouts that handle complex MMO quest
structures including chains, branches, and shared group progress, **so that** players can follow
multi-layered quest lines without confusion.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Visual hierarchy for multi-step quest chains | F-10.3.7 | R-10.3.7 |
| Branching objectives displayed clearly | F-10.3.7 | R-10.3.7 |
| Directional compass indicators with distance | F-10.3.7 | R-10.3.7 |

## US-10.3.15 Chat With Hundreds of Players Using Multiple Channels

**As a** player (P-23), **I want** a chat window with multiple channels, tabbed views, scrollable
history, clickable player and item links, and inline emoji, **so that** I can communicate
effectively in MMO cities with high message throughput.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Say, party, guild, raid, whisper, trade, general channels | F-10.3.8 | R-10.3.8 |
| Tabbed channel views with scrollable message history | F-10.3.8 | R-10.3.8 |
| Clickable player name and item links | F-10.3.8 | R-10.3.8 |
| Profanity filtering, spam throttling, ignore lists | F-10.3.8 | R-10.3.8 |

## US-10.3.16 Verify Chat Handles High Message Throughput

**As an** engine tester (P-27), **I want** to verify the chat system handles 200+ messages per
second without dropping messages, lagging scroll, or UI hitches, **so that** chat remains responsive
in crowded MMO areas.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| 200+ messages/second sustained without drops | F-10.3.8 | R-10.3.8 |
| Scroll remains smooth during high throughput | F-10.3.8 | R-10.3.8 |
| Inline emoji and item links render without delay | F-10.3.8 | R-10.3.8 |

## US-10.3.17 Manage Inventory With Drag-Drop Grid Containers

**As a** player (P-23), **I want** grid-based inventory with drag-and-drop, stack splitting,
sorting, search filtering, and bag/tab organization, **so that** I can manage items across player
bags, bank, guild bank, vendor, loot, and mail attachment views.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Grid-based display with item quality borders | F-10.3.9 | R-10.3.9 |
| Drag-and-drop with stack splitting | F-10.3.9 | R-10.3.9 |
| Sort by name, type, rarity, level | F-10.3.9 | R-10.3.9 |
| Search filtering and bag/tab organization | F-10.3.9 | R-10.3.9 |

## US-10.3.18 Design Unified Container Widget for All Inventory Types

**As a** designer (P-5), **I want** a single grid widget that adapts to different container types
(bags, bank, vendor, trade, loot, mail) with context-specific behavior, **so that** all inventory
interfaces share consistent interaction patterns.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| One grid widget serves all container types | F-10.3.9 | R-10.3.9 |
| Context-specific behavior per container type | F-10.3.9 | R-10.3.9 |
| Visual item quality borders and stack count display | F-10.3.9 | R-10.3.9 |

## US-10.3.19 Navigate Using a Skyrim-Style Compass Bar

**As a** player (P-23), **I want** a horizontal compass strip showing cardinal directions and
tracked objective markers that rotate with my facing, **so that** I maintain directional awareness
while exploring without opening the map.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Compass strip shows N/S/E/W and rotates with player facing | F-10.3.10 | R-10.3.10 |
| Tracked objectives show icon and distance | F-10.3.10 | R-10.3.10 |
| Markers stack vertically when overlapping on same bearing | F-10.3.10 | R-10.3.10 |
| At least 3 compass styles (strip, arc, dot) | F-10.3.10 | R-10.3.10 |

## US-10.3.20 See Party Members and Custom Waypoints on Compass

**As a** player (P-23), **I want** party member positions and custom waypoints to appear on the
compass alongside quest markers with configurable distance fade, **so that** I can navigate toward
teammates and custom destinations without opening the map.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Custom waypoints appear as compass markers | F-10.3.10 | R-10.3.10 |
| Party members show on compass when off-screen | F-10.3.10 | R-10.3.10 |
| Markers fade at configurable distance thresholds | F-10.3.10 | R-10.3.10 |
| Compass works in both 2D and 3D camera modes | F-10.3.10 | R-10.3.10 |

## US-10.3.21 Configure Compass Style Per Project

**As a** designer (P-5), **I want** to choose between full strip, arc, and minimal dot compass
styles per project and configure marker types and fade thresholds, **so that** the compass matches
the game's HUD aesthetic.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Compass style configurable per project | F-10.3.10 | R-10.3.10 |
| Custom marker types definable from gameplay logic | F-10.3.10 | R-10.3.10 |
| Mobile can substitute edge-of-screen directional arrows | F-10.3.10 | R-10.3.10 |

## US-10.3.22 Follow Edge Arrows to Off-Screen Objectives

**As a** player (P-23), **I want** directional arrows at screen edges pointing toward off-screen
tracked objectives with distance, icon, and priority-based coloring, **so that** I know which
direction to turn without pausing to open the map.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Arrow at screen edge for each off-screen tracked objective | F-10.3.11 | R-10.3.11 |
| Arrow shows direction, distance, and objective icon | F-10.3.11 | R-10.3.11 |
| Smooth transition to in-world marker when objective enters viewport | F-10.3.11 | R-10.3.11 |
| Priority coloring (gold main quest, silver side quest) | F-10.3.11 | R-10.3.11 |

## US-10.3.23 Verify Multiple Off-Screen Indicators Stack Correctly

**As a** QA engineer (P-19), **I want** to verify that multiple off-screen indicators stack along
screen edges without overlap and that per-marker visibility controls work, **so that** all tracked
objectives remain distinguishable.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Multiple indicators stack without overlap | F-10.3.11 | R-10.3.11 |
| Per-marker visibility control (compass only, arrow only, both) | F-10.3.11 | R-10.3.11 |
| System works with both 3D and 2D top-down cameras | F-10.3.11 | R-10.3.11 |

## US-10.3.24 Auto-Generate Minimap From Procedural Terrain Data

**As a** developer (P-15), **I want** the minimap to auto-generate from terrain, biome, building
footprint, and road spline data without artist intervention, **so that** procedurally generated
worlds have functional minimaps immediately.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Minimap colorized by biome/material type | F-10.3.12 | R-10.3.12 |
| Building footprints from collision geometry | F-10.3.12 | R-10.3.12 |
| Roads from road spline data | F-10.3.12 | R-10.3.12 |
| Newly streamed chunks extend minimap automatically | F-10.3.12 | R-10.3.12 |

## US-10.3.25 Explore Procedural Worlds With Progressive Map Reveal

**As a** player (P-23), **I want** the minimap to reveal terrain progressively as I explore with fog
of war hiding undiscovered areas, **so that** exploration feels rewarding and the map serves as a
record of where I have been.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Fog of war hides unexplored minimap areas | F-10.3.12 | R-10.3.12 |
| Explored areas remain visible after leaving | F-10.3.12 | R-10.3.12 |
| Voxel worlds show slice at player elevation | F-10.3.12 | R-10.3.12 |
| Minimap texture scales per platform (256 mobile, 512 desktop) | F-10.3.12 | R-10.3.12 |

## US-10.3.26 Browse the World Map With Multi-Layer Rendering

**As a** player (P-23), **I want** a world map with terrain, biome, political, road, settlement,
water, and fog-of-war layers that I can zoom, pan, and search, **so that** I can plan routes, find
locations, and set waypoints.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| All 7 map layers rendered (terrain through annotations) | F-10.3.13 | R-10.3.13 |
| Continuous zoom from world overview to local detail | F-10.3.13 | R-10.3.13 |
| Click-to-set-waypoint and click-to-fast-travel | F-10.3.13 | R-10.3.13 |
| Search-by-name for discovered locations | F-10.3.13 | R-10.3.13 |

## US-10.3.27 View Planet-Scale Worlds on a Zoomable Globe

**As a** player (P-23), **I want** the world map to render on a zoomable globe with zoom-to-surface
transitions for planet-scale worlds, **so that** the map matches the scale and curvature of the game
world.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Globe rendering for planet-scale worlds | F-10.3.13 | R-10.3.13 |
| Smooth zoom-to-surface transitions | F-10.3.13 | R-10.3.13 |
| Tiled image pyramids for efficient zoom rendering | F-10.3.13 | R-10.3.13 |
| Globe rendering desktop/console only | F-10.3.13 | R-10.3.13 |

## US-10.3.28 Implement World Map Generation Pipeline

**As an** engine developer (P-26), **I want** to implement a world map generation pipeline that
produces tiled image pyramids from terrain, biome, political, and infrastructure data, **so that**
maps render efficiently at all zoom levels with cached tiles.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Map generated from procedural generation output | F-10.3.13 | R-10.3.13 |
| Tiled image pyramids cached for zoom rendering | F-10.3.13 | R-10.3.13 |
| Mobile uses lower-resolution tiles and fewer zoom levels | F-10.3.13 | R-10.3.13 |

## US-10.3.29 Apply Artistic Post-Processing Styles to the World Map

**As an** artist (P-8), **I want** to apply post-processing styles to the world map including
parchment, painted, tactical, satellite, and schematic modes, **so that** the map reinforces the
game's visual identity.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| At least 5 post-processing styles available | F-10.3.14 | R-10.3.14 |
| Parchment style with sepia tone and ink-drawn lines | F-10.3.14 | R-10.3.14 |
| Artistic layers composed in the visual editor | F-10.3.14 | R-10.3.14 |

## US-10.3.30 Substitute Hand-Painted Maps for Key Zones

**As an** artist (P-8), **I want** to substitute static hand-painted map images for specific zones
while dynamic data layers (player position, quest markers, fog of war) overlay correctly, **so
that** important areas get maximum artistic quality.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Static maps replace generated maps per zone | F-10.3.14 | R-10.3.14 |
| Dynamic markers overlay correctly on static maps | F-10.3.14 | R-10.3.14 |
| Artistic and generated maps blend in rendering pipeline | F-10.3.14 | R-10.3.14 |

## US-10.3.31 Design Map Overlay Assets and Decorative Elements

**As a** designer (P-5), **I want** to compose map overlay assets like compass roses, decorative
borders, illustrated landmarks, and sea monster illustrations in the visual editor, **so that** the
map has a polished, immersive presentation.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Hand-drawn overlays layer on generated map data | F-10.3.14 | R-10.3.14 |
| Compass roses, borders, and illustrated landmarks supported | F-10.3.14 | R-10.3.14 |
| Overlays composed in visual editor | F-10.3.14 | R-10.3.14 |

## US-10.3.32 See Quest Markers on Minimap, Compass, and World Map

**As a** player (P-23), **I want** quest markers to appear simultaneously on the minimap, compass,
and world map with consistent icons, **so that** I always know where my objectives are regardless of
which navigation tool I check.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Tracked markers appear on minimap, compass, and world map | F-10.3.15 | R-10.3.15 |
| Consistent icons across all navigation views | F-10.3.15 | R-10.3.15 |
| Untracking a quest hides its markers everywhere | F-10.3.15 | R-10.3.15 |

## US-10.3.33 Define Data-Driven Map Marker Categories

**As a** developer (P-15), **I want** map markers defined as data assets with icon, color, category,
visibility rules, and source type, **so that** quest objectives, NPCs, resources, party members,
enemies, and waypoints are all managed through a unified marker system.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| At least 8 marker categories supported | F-10.3.15 | R-10.3.15 |
| Markers defined as data assets with icon, color, category | F-10.3.15 | R-10.3.15 |
| Visibility rules per marker (always, tracked only, zoom range) | F-10.3.15 | R-10.3.15 |
| Quest tracker drives marker visibility | F-10.3.15 | R-10.3.15 |

## US-10.3.34 Zoom Out and See Markers Cluster Automatically

**As a** player (P-23), **I want** overlapping markers to cluster into grouped icons at low zoom
levels that expand when I zoom in, **so that** the map remains readable even in marker-dense areas.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Markers cluster when icons would overlap | F-10.3.15 | R-10.3.15 |
| Clusters expand into individual markers on zoom-in | F-10.3.15 | R-10.3.15 |
| Marker count capped per view for performance | F-10.3.15 | R-10.3.15 |
