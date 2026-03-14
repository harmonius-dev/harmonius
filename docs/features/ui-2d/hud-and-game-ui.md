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
overlays (radial cooldown sweep and countdown text). Icons must support tooltip inspection
showing effect name, description, source, and remaining duration. The system must handle
large icon counts (30+ effects in MMO raids) with priority-based filtering and grouping
(player buffs, raid buffs, debuffs, dispellable effects).

- **Requirements:** R-10.3.2
- **Dependencies:** F-10.1.1, F-10.2.6, F-10.1.6
- **Platform notes:** None

### F-10.3.3 Action Bars and Cooldown Indicators

Configurable grids of ability slots bound to keyboard/gamepad inputs, displaying ability icons
with radial cooldown sweeps, charge counts, range/usability tinting, and keybind labels. Support
multiple action bar pages, stance/form-conditional bars, and macro icons. Cooldown indicators
must be frame-accurate with the game simulation to avoid abilities appearing ready before they
are.

- **Requirements:** R-10.3.3
- **Dependencies:** F-10.1.1, F-10.1.6, F-10.2.7
- **Platform notes:** None

## World-Anchored UI

### F-10.3.4 Nameplates and World-Space Labels

Floating UI elements anchored to 3D world positions (above characters, NPCs, resource nodes)
that are projected to screen space each frame. Nameplates display name, guild, title, health
bar, and cast bar. Must scale to hundreds of visible nameplates in MMO cities and battlegrounds
with LOD-based culling (distance fade, importance-based filtering), occlusion culling against
terrain, and overlap avoidance to prevent stacking.

- **Requirements:** R-10.3.4
- **Dependencies:** F-10.1.1, F-10.3.1, F-10.4.1
- **Platform notes:** None

### F-10.3.5 Floating Combat Text and Damage Numbers

Animated text elements spawned at world positions showing damage dealt, healing received,
experience gained, and other combat feedback. Numbers animate with configurable trajectories
(rise-and-fade, arcing, sticky), merge repeated hits into cumulative totals, and color-code
by damage type (physical, fire, frost, etc.). Must handle burst scenarios with dozens of
simultaneous damage events without visual clutter.

- **Requirements:** R-10.3.5
- **Dependencies:** F-10.1.2, F-10.3.4, F-10.4.2
- **Platform notes:** None

## Navigation and Tracking

### F-10.3.6 Minimap and World Map Overlays

Circular or rectangular minimap widget rendering a top-down view of the local area with icons
for party members, NPCs, quest objectives, resource nodes, and points of interest. The full
world map supports zoom, pan, fog-of-war, zone boundaries, waypoint placement, flight path
overlays, and dungeon floor plans. Both map views must update marker positions in real time
for moving entities.

- **Requirements:** R-10.3.6
- **Dependencies:** F-10.1.1, F-10.4.4
- **Platform notes:** None

### F-10.3.7 Quest Tracker and Objective HUD

Persistent on-screen widget listing active quest objectives with progress indicators (kill
counts, collection progress, zone completion percentages). Supports world-space waypoint
markers, directional compass indicators, and distance displays for tracked objectives.
The tracker must handle complex MMO quest structures: multi-step chains, branching objectives,
daily/weekly resets, and group quest shared progress.

- **Requirements:** R-10.3.7
- **Dependencies:** F-10.1.1, F-10.1.6, F-10.3.6
- **Platform notes:** None

## Communication

### F-10.3.8 Chat System

Full-featured chat window supporting multiple channels (say, party, guild, raid, whisper, trade,
general, custom), tabbed channel views, scrollable message history with timestamps, clickable
player name and item links, and inline icon/emoji support. Moderation features include profanity
filtering, spam throttling, ignore lists, and report functionality. Must handle high message
throughput in MMO cities where hundreds of players chat simultaneously.

- **Requirements:** R-10.3.8
- **Dependencies:** F-10.2.1, F-10.2.2, F-10.2.5, F-10.1.6
- **Platform notes:** None

## Inventory and Economy

### F-10.3.9 Inventory Grids and Container Management

Grid-based inventory display with drag-and-drop item movement, stack splitting, item sorting
(by name, type, rarity, level), search filtering, bag/tab organization, and visual item
quality borders. Supports multiple container types: player bags, bank tabs, guild bank, vendor
buy/sell, loot windows, trade windows, and mail attachments. All container views share the
same grid widget with context-specific behavior.

- **Requirements:** R-10.3.9
- **Dependencies:** F-10.1.1, F-10.2.7, F-10.2.5, F-10.2.6
- **Platform notes:** None
