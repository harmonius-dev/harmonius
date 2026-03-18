# User Stories — 10.3 HUD & Game UI

## Player Status

| ID        | Persona              | Features | Requirements       |
|-----------|----------------------|----------|--------------------|
| US-10.3.1 | Player (P-23)        | F-10.3.1 | R-10.3.1           |
| US-10.3.2 | Artist (P-8)         | F-10.3.1 | R-10.3.1           |
| US-10.3.3 | Engine tester (P-27) | F-10.3.1 | R-10.3.1, R-X.12.1 |
| US-10.3.4 | Player (P-23)        | F-10.3.2 | R-10.3.2           |
| US-10.3.5 | Designer (P-5)       | F-10.3.2 | R-10.3.2           |
| US-10.3.6 | Player (P-23)        | F-10.3.3 | R-10.3.3           |
| US-10.3.7 | Engine tester (P-27) | F-10.3.3 | R-10.3.3           |

1. **US-10.3.1** — I want segmented and continuous health/mana/energy bars with animated drain/fill,
   predicted damage overlays, and absorb shield indicators, so that I can monitor my character's
   status at a glance during fast-paced combat.
   - **Acceptance:** Color-coded bars per resource type; animated drain/fill transitions; predicted
     damage overlay visible before hit applies; 40+ raid bars render at 60fps simultaneously
2. **US-10.3.2** — I want to customize health and resource bar visuals including segmentation, fill
   colors, border styles, and absorb overlay graphics, so that bars match the game's visual identity
   for different factions and character classes.
   - **Acceptance:** Segmented and continuous bar variants; custom fill colors and border styles per
     resource type; cast bar with interrupt indicator styling
3. **US-10.3.3** — I want to benchmark rendering of 40+ player health, mana, and cast bars
   simultaneously, so that I can verify raid UI stays within the 2ms GPU budget during worst-case
   raid encounters.
   - **Acceptance:** 40 health bars + 40 mana bars + cast bars at 60fps; GPU time under 2ms for all
     raid bars combined
4. **US-10.3.4** — I want grid-based buff and debuff icons with radial cooldown sweeps, countdown
   text, and tooltip inspection, so that I can monitor 30+ active effects with priority filtering
   during raids.
   - **Acceptance:** Radial cooldown sweep and countdown text on icons; tooltip shows effect name,
     description, source, duration; priority-based filtering and grouping for 30+ icons
5. **US-10.3.5** — I want to configure how buff and debuff icons are grouped, prioritized, and
   filtered (player buffs, raid buffs, debuffs, dispellable), so that the most important effects are
   always visible even with many active.
   - **Acceptance:** Configurable grouping categories; priority-based display ordering; filtering by
     buff type (player, raid, debuff, dispellable)
6. **US-10.3.6** — I want configurable action bar grids with radial cooldown sweeps, charge counts,
   range/usability tinting, and keybind labels, so that I know exactly when abilities are ready
   without frame-inaccurate visual delays.
   - **Acceptance:** Radial cooldown sweeps frame-accurate with simulation; charge counts and
     range/usability tinting; multiple action bar pages and stance-conditional bars; keybind labels
     displayed per slot
7. **US-10.3.7** — I want to verify that action bar cooldown indicators are frame-accurate with the
   game simulation, so that abilities never appear ready before they actually are and no visual
   desync occurs.
   - **Acceptance:** Cooldown visual matches simulation timer within 1 frame; no ability appears
     ready before actual cooldown expires; frame-accuracy maintained under high system load

## World-Anchored UI

| ID         | Persona              | Features | Requirements |
|------------|----------------------|----------|--------------|
| US-10.3.8  | Player (P-23)        | F-10.3.4 | R-10.3.4     |
| US-10.3.9  | Engine tester (P-27) | F-10.3.4 | R-10.3.4     |
| US-10.3.10 | Player (P-23)        | F-10.3.5 | R-10.3.5     |
| US-10.3.11 | Designer (P-5)       | F-10.3.5 | R-10.3.5     |

1. **US-10.3.8** — I want floating nameplates above characters and NPCs showing name, guild, health
   bar, and cast bar with overlap avoidance and distance-based culling, so that I can identify
   entities in crowded MMO cities without visual clutter.
   - **Acceptance:** Nameplates display name, guild, title, health, cast bar; LOD-based culling with
     distance fade; overlap avoidance prevents stacking; 200+ nameplates on desktop, 50 on mobile
2. **US-10.3.9** — I want to stress-test nameplate rendering with 200+ visible entities to verify
   overlap avoidance, occlusion culling, and draw call budget, so that performance holds in
   worst-case city scenarios.
   - **Acceptance:** 200 simultaneous nameplates render within GPU budget; occlusion culling hides
     nameplates behind terrain; overlap avoidance correct with rapid entity movement
3. **US-10.3.10** — I want animated damage numbers, healing numbers, and experience gains that spawn
   at world positions with color-coding by damage type and merge repeated hits, so that combat
   feedback is clear even during burst damage scenarios.
   - **Acceptance:** Damage, healing, and XP numbers animate from world positions; color-coded by
     damage type (physical, fire, frost); repeated hits merge into cumulative totals; configurable
     trajectories (rise-and-fade, arcing, sticky)
4. **US-10.3.11** — I want to configure damage number fonts, colors per damage type, animation
   trajectories, merge behavior, and burst handling, so that combat text feedback matches the game's
   visual style and remains readable during intense encounters.
   - **Acceptance:** Per-damage-type color and font size configuration; trajectory selection per
     number type; merge threshold configurable for repeated hits

## Navigation and Tracking

| ID         | Persona        | Features | Requirements |
|------------|----------------|----------|--------------|
| US-10.3.12 | Player (P-23)  | F-10.3.6 | R-10.3.6     |
| US-10.3.13 | Player (P-23)  | F-10.3.7 | R-10.3.7     |
| US-10.3.14 | Designer (P-5) | F-10.3.7 | R-10.3.7     |

1. **US-10.3.12** — I want a minimap showing party members, NPCs, quest objectives, and POIs, plus a
   full world map with zoom, pan, fog-of-war, and waypoint placement, so that I can navigate and
   track objectives without getting lost.
   - **Acceptance:** Circular or rectangular minimap with real-time icon updates; world map with
     zoom, pan, and fog-of-war; waypoint placement on both map views; dungeon floor plans supported
2. **US-10.3.13** — I want a persistent quest tracker showing active objectives with progress
   indicators, world-space waypoint markers, compass indicators, and distance displays, so that I
   always know what to do next and where to go.
   - **Acceptance:** Active objectives with kill/collection progress; world-space waypoint markers
     for tracked objectives; multi-step chains and branching objectives supported; daily/weekly
     resets and group shared progress
3. **US-10.3.14** — I want to design quest tracker layouts that handle complex MMO quest structures
   including chains, branches, and shared group progress, so that players can follow multi-layered
   quest lines without confusion.
   - **Acceptance:** Visual hierarchy for multi-step quest chains; branching objectives displayed
     clearly; directional compass indicators with distance

## Communication

| ID         | Persona              | Features | Requirements |
|------------|----------------------|----------|--------------|
| US-10.3.15 | Player (P-23)        | F-10.3.8 | R-10.3.8     |
| US-10.3.16 | Engine tester (P-27) | F-10.3.8 | R-10.3.8     |

1. **US-10.3.15** — I want a chat window with multiple channels, tabbed views, scrollable history,
   clickable player and item links, and inline emoji, so that I can communicate effectively in MMO
   cities with high message throughput.
   - **Acceptance:** Say, party, guild, raid, whisper, trade, general channels; tabbed channel views
     with scrollable message history; clickable player name and item links; profanity filtering,
     spam throttling, ignore lists
2. **US-10.3.16** — I want to verify the chat system handles 200+ messages per second without
   dropping messages, lagging scroll, or UI hitches, so that chat remains responsive in crowded MMO
   areas.
   - **Acceptance:** 200+ messages/second sustained without drops; scroll remains smooth during high
     throughput; inline emoji and item links render without delay

## Inventory and Economy

| ID         | Persona        | Features | Requirements |
|------------|----------------|----------|--------------|
| US-10.3.17 | Player (P-23)  | F-10.3.9 | R-10.3.9     |
| US-10.3.18 | Designer (P-5) | F-10.3.9 | R-10.3.9     |

1. **US-10.3.17** — I want grid-based inventory with drag-and-drop, stack splitting, sorting, search
   filtering, and bag/tab organization, so that I can manage items across player bags, bank, guild
   bank, vendor, loot, and mail attachment views.
   - **Acceptance:** Grid-based display with item quality borders; drag-and-drop with stack
     splitting; sort by name, type, rarity, level; search filtering and bag/tab organization
2. **US-10.3.18** — I want a single grid widget that adapts to different container types (bags,
   bank, vendor, trade, loot, mail) with context-specific behavior, so that all inventory interfaces
   share consistent interaction patterns.
   - **Acceptance:** One grid widget serves all container types; context-specific behavior per
     container type; visual item quality borders and stack count display

## Compass and Bearing

| ID         | Persona            | Features  | Requirements |
|------------|--------------------|-----------|--------------|
| US-10.3.19 | Player (P-23)      | F-10.3.10 | R-10.3.10    |
| US-10.3.20 | Player (P-23)      | F-10.3.10 | R-10.3.10    |
| US-10.3.21 | Designer (P-5)     | F-10.3.10 | R-10.3.10    |
| US-10.3.22 | Player (P-23)      | F-10.3.11 | R-10.3.11    |
| US-10.3.23 | QA engineer (P-19) | F-10.3.11 | R-10.3.11    |

1. **US-10.3.19** — I want a horizontal compass strip showing cardinal directions and tracked
   objective markers that rotate with my facing, so that I maintain directional awareness while
   exploring without opening the map.
   - **Acceptance:** Compass strip shows N/S/E/W and rotates with player facing; tracked objectives
     show icon and distance; markers stack vertically when overlapping on same bearing; at least 3
     compass styles (strip, arc, dot)
2. **US-10.3.20** — I want party member positions and custom waypoints to appear on the compass
   alongside quest markers with configurable distance fade, so that I can navigate toward teammates
   and custom destinations without opening the map.
   - **Acceptance:** Custom waypoints appear as compass markers; party members show on compass when
     off-screen; markers fade at configurable distance thresholds; compass works in both 2D and 3D
     camera modes
3. **US-10.3.21** — I want to choose between full strip, arc, and minimal dot compass styles per
   project and configure marker types and fade thresholds, so that the compass matches the game's
   HUD aesthetic.
   - **Acceptance:** Compass style configurable per project; custom marker types definable from
     gameplay logic; mobile can substitute edge-of-screen directional arrows
4. **US-10.3.22** — I want directional arrows at screen edges pointing toward off-screen tracked
   objectives with distance, icon, and priority-based coloring, so that I know which direction to
   turn without pausing to open the map.
   - **Acceptance:** Arrow at screen edge for each off-screen tracked objective; arrow shows
     direction, distance, and objective icon; smooth transition to in-world marker when objective
     enters viewport; priority coloring (gold main quest, silver side quest)
5. **US-10.3.23** — I want to verify that multiple off-screen indicators stack along screen edges
   without overlap and that per-marker visibility controls work, so that all tracked objectives
   remain distinguishable.
   - **Acceptance:** Multiple indicators stack without overlap; per-marker visibility control
     (compass only, arrow only, both); system works with both 3D and 2D top-down cameras

## Map System

| ID         | Persona                 | Features  | Requirements |
|------------|-------------------------|-----------|--------------|
| US-10.3.24 | Developer (P-15)        | F-10.3.12 | R-10.3.12    |
| US-10.3.25 | Player (P-23)           | F-10.3.12 | R-10.3.12    |
| US-10.3.26 | Player (P-23)           | F-10.3.13 | R-10.3.13    |
| US-10.3.27 | Player (P-23)           | F-10.3.13 | R-10.3.13    |
| US-10.3.28 | Engine developer (P-26) | F-10.3.13 | R-10.3.13    |
| US-10.3.29 | Artist (P-8)            | F-10.3.14 | R-10.3.14    |
| US-10.3.30 | Artist (P-8)            | F-10.3.14 | R-10.3.14    |
| US-10.3.31 | Designer (P-5)          | F-10.3.14 | R-10.3.14    |
| US-10.3.32 | Player (P-23)           | F-10.3.15 | R-10.3.15    |
| US-10.3.33 | Developer (P-15)        | F-10.3.15 | R-10.3.15    |
| US-10.3.34 | Player (P-23)           | F-10.3.15 | R-10.3.15    |

1. **US-10.3.24** — I want the minimap to auto-generate from terrain, biome, building footprint, and
   road spline data without artist intervention, so that procedurally generated worlds have
   functional minimaps immediately.
   - **Acceptance:** Minimap colorized by biome/material type; building footprints from collision
     geometry; roads from road spline data; newly streamed chunks extend minimap automatically
2. **US-10.3.25** — I want the minimap to reveal terrain progressively as I explore with fog of war
   hiding undiscovered areas, so that exploration feels rewarding and the map serves as a record of
   where I have been.
   - **Acceptance:** Fog of war hides unexplored minimap areas; explored areas remain visible after
     leaving; voxel worlds show slice at player elevation; minimap texture scales per platform (256
     mobile, 512 desktop)
3. **US-10.3.26** — I want a world map with terrain, biome, political, road, settlement, water, and
   fog-of-war layers that I can zoom, pan, and search, so that I can plan routes, find locations,
   and set waypoints.
   - **Acceptance:** All 7 map layers rendered (terrain through annotations); continuous zoom from
     world overview to local detail; click-to-set-waypoint and click-to-fast-travel; search-by-name
     for discovered locations
4. **US-10.3.27** — I want the world map to render on a zoomable globe with zoom-to-surface
   transitions for planet-scale worlds, so that the map matches the scale and curvature of the game
   world.
   - **Acceptance:** Globe rendering for planet-scale worlds; smooth zoom-to-surface transitions;
     tiled image pyramids for efficient zoom rendering; globe rendering desktop/console only
5. **US-10.3.28** — I want to implement a world map generation pipeline that produces tiled image
   pyramids from terrain, biome, political, and infrastructure data, so that maps render efficiently
   at all zoom levels with cached tiles.
   - **Acceptance:** Map generated from procedural generation output; tiled image pyramids cached
     for zoom rendering; mobile uses lower-resolution tiles and fewer zoom levels
6. **US-10.3.29** — I want to apply post-processing styles to the world map including parchment,
   painted, tactical, satellite, and schematic modes, so that the map reinforces the game's visual
   identity.
   - **Acceptance:** At least 5 post-processing styles available; parchment style with sepia tone
     and ink-drawn lines; artistic layers composed in the visual editor
7. **US-10.3.30** — I want to substitute static hand-painted map images for specific zones while
   dynamic data layers (player position, quest markers, fog of war) overlay correctly, so that
   important areas get maximum artistic quality.
   - **Acceptance:** Static maps replace generated maps per zone; dynamic markers overlay correctly
     on static maps; artistic and generated maps blend in rendering pipeline
8. **US-10.3.31** — I want to compose map overlay assets like compass roses, decorative borders,
   illustrated landmarks, and sea monster illustrations in the visual editor, so that the map has a
   polished, immersive presentation.
   - **Acceptance:** Hand-drawn overlays layer on generated map data; compass roses, borders, and
     illustrated landmarks supported; overlays composed in visual editor
9. **US-10.3.32** — I want quest markers to appear simultaneously on the minimap, compass, and world
   map with consistent icons, so that I always know where my objectives are regardless of which
   navigation tool I check.
   - **Acceptance:** Tracked markers appear on minimap, compass, and world map; consistent icons
     across all navigation views; untracking a quest hides its markers everywhere
10. **US-10.3.33** — I want map markers defined as data assets with icon, color, category,
    visibility rules, and source type, so that quest objectives, NPCs, resources, party members,
    enemies, and waypoints are all managed through a unified marker system.
    - **Acceptance:** At least 8 marker categories supported; markers defined as data assets with
      icon, color, category; visibility rules per marker (always, tracked only, zoom range); quest
      tracker drives marker visibility
11. **US-10.3.34** — I want overlapping markers to cluster into grouped icons at low zoom levels
    that expand when I zoom in, so that the map remains readable even in marker-dense areas.
    - **Acceptance:** Markers cluster when icons would overlap; clusters expand into individual
      markers on zoom-in; marker count capped per view for performance
