# 13.14 — Building and Survival

## Building Placement

### F-13.14.1 Modular Building Placement System

Snap-based placement of modular building pieces (walls, floors, roofs, doors, stairs, ramps,
foundations, pillars) using a socket/anchor system. Each piece defines attachment sockets that
connect to compatible sockets on adjacent pieces. During placement, a ghost preview mesh follows the
cursor with green/red validity indication based on collision, slope, and adjacency rules. Pieces
snap to valid sockets with rotation increments (90-degree for square, 60-degree for hex). Free-form
placement mode allows unconstrained positioning with physics-based ground alignment. All placement
rules are data-driven building piece assets authored in the visual editor.

- **Requirements:** R-13.14.1
- **Dependencies:** F-1.9.4 (Spatial Query), F-4.2.1 (Collision), F-2.10.1 (ECS-to-Renderer Bridge)
- **Platform notes:** None

### F-13.14.2 Construction Phase and Progress

Buildings are placed as scaffolds/blueprints that require resources and time (or worker assignment)
to complete. During construction, a progress bar displays completion percentage. The building mesh
transitions through visual stages (foundation -> frame -> walls -> complete) keyed to progress
thresholds. Construction can be paused, cancelled (partial resource refund), or accelerated with
additional workers/materials. Incomplete buildings have proportionally reduced HP and functionality.
Construction integrates with the resource system — required materials are deducted from the
builder's inventory on placement.

- **Requirements:** R-13.14.2
- **Dependencies:** F-13.14.1, F-13.9.1 (Inventory), F-13.7.6 (Currency)
- **Platform notes:** None

### F-13.14.3 Structural Integrity

Stability propagation from foundations — unsupported structures collapse. Each building piece has a
stability value that decreases with distance from the nearest grounded foundation. Pieces below a
stability threshold display visual warnings (cracks, wobble) and collapse when stability reaches
zero, destroying the piece and reducing stability of connected pieces (cascade collapse). Stone and
metal pieces have higher stability than wood. Structural integrity is computed incrementally when
pieces are added, removed, or damaged, not every frame. Used for survival games (Valheim model) and
destruction physics integration.

- **Requirements:** R-13.14.3
- **Dependencies:** F-13.14.1, F-4.6.5 (Stress Propagation)
- **Platform notes:** Mobile caps placed building pieces (500 vs 2000+ on desktop) to limit
  structural integrity graph traversal cost per frame.

### F-13.14.4 Building Upgrade and Repair

Upgrade placed building pieces from one material tier to the next (wood -> stone -> metal ->
reinforced) by targeting them with upgrade action and paying material costs. Upgrades change the
piece's visual mesh, HP, stability bonus, and resistance properties. Repair reverses damage by
consuming materials proportional to the damage amount. Building decay occurs over real time when
structures are not maintained (configurable rate, can be disabled). Decay rate and repair costs are
defined per material tier in gameplay databases.

- **Requirements:** R-13.14.4
- **Dependencies:** F-13.14.1, F-13.9.1 (Inventory)
- **Platform notes:** None

### F-13.14.5a Housing Plot and Instance System

Designated housing plots or instanced housing zones where players build personal spaces. Housing
instances support visitor permissions (public, friends, guild, private). Housing state persists
through the save system (F-13.3.1) and is replicated for visitors.

- **Requirements:** R-13.14.5a
- **Dependencies:** F-13.14.1, F-13.3.1 (Save System)
- **Platform notes:** None

### F-13.14.5b Furniture Placement

Furniture placement uses a separate grid/free placement system within interior spaces. Players place
beds, tables, storage chests, crafting stations (F-13.12.4), and decoration items. Decorative
furniture is purely cosmetic.

- **Requirements:** R-13.14.5b
- **Dependencies:** F-13.14.5a, F-13.14.1
- **Platform notes:** None

### F-13.14.5c Functional Furniture Effects

Functional furniture provides gameplay benefits: beds set respawn points, storage chests extend
inventory capacity, and crafting stations enable profession crafting. Effects are data-driven and
configurable per furniture type in gameplay databases.

- **Requirements:** R-13.14.5c
- **Dependencies:** F-13.14.5b, F-13.12.4 (Crafting Stations), F-13.9.1 (Inventory)
- **Platform notes:** None

## Survival Systems

### F-13.14.6a Hunger and Thirst System

Hunger drains at a base rate accelerated by physical activity; eating restores hunger and may
provide temporary stat buffs. Thirst drains faster in hot biomes; drinking restores thirst. All
drain rates and restoration values are configurable in gameplay databases.

- **Requirements:** R-13.14.6a
- **Dependencies:** F-13.7.9 (Stat Tables), F-13.10.3 (Gameplay Effects)
- **Platform notes:** None

### F-13.14.6b Temperature and Warmth System

Warmth is affected by clothing insulation values, proximity to fire sources, shelter detection, and
weather conditions (F-11.4.1). Exposure to cold without mitigation drains warmth over time.

- **Requirements:** R-13.14.6b
- **Dependencies:** F-13.7.9 (Stat Tables), F-11.4.1 (Weather)
- **Platform notes:** None

### F-13.14.6c Stamina and Fatigue System

Stamina depletes from sprinting, jumping, and combat actions. Stamina recovers during rest at a
configurable rate. Fatigue accumulates from prolonged exertion and slows recovery until the
character rests.

- **Requirements:** R-13.14.6c
- **Dependencies:** F-13.7.9 (Stat Tables)
- **Platform notes:** None

### F-13.14.6d Vital Debuff System

Critically low vitals impose debuffs: starvation reduces max HP, dehydration slows movement,
hypothermia applies periodic damage. Debuff thresholds and penalty values are configurable per vital
in gameplay databases.

- **Requirements:** R-13.14.6d
- **Dependencies:** F-13.14.6a, F-13.14.6b, F-13.14.6c, F-13.10.3 (Gameplay Effects)
- **Platform notes:** None

### F-13.14.7a Resource Node Definition

Harvestable world-placed resource nodes (ore veins, trees, herb patches, rock deposits, fishing
spots). Each node defines: resource type yielded, gather time, tool requirement (pickaxe for ore,
axe for trees), gather animation, node HP (deplete through multiple gathers), respawn timer, and
rare yield chance.

- **Requirements:** R-13.14.7a
- **Dependencies:** F-13.7.1 (Table Schema)
- **Platform notes:** None

### F-13.14.7b Gathering Interaction Loop

Gathering triggers an interaction (F-13.17.1) and plays a looped animation until the node is
depleted or the player cancels. Gathered resources are added to inventory. Gathering skill level
(F-13.12.3a) determines yield quantity and rare proc rate.

- **Requirements:** R-13.14.7b
- **Dependencies:** F-13.14.7a, F-13.12.3a (Profession Data Model), F-13.9.1 (Inventory)
- **Platform notes:** None

### F-13.14.7c Resource Node Procedural Distribution

Node placement integrates with the procedural generation system (F-3.6.40) for open-world
distribution. Biome-specific node density, type distribution, and clustering rules are configurable
per world region.

- **Requirements:** R-13.14.7c
- **Dependencies:** F-13.14.7a, F-3.6.40 (Enemy and Creature Placement)
- **Platform notes:** None

### F-13.14.8 Farming and Crop System

Agricultural gameplay loop: till soil, plant seeds, water crops, wait for growth, harvest produce.
Crop growth occurs in configurable stages (seed -> sprout -> growing -> mature -> harvestable) with
per-stage duration. Crops require watering (manual or irrigation) and wither from neglect after a
configurable grace period. Soil quality and fertilizer application affect growth speed and yield
quantity. Seasonal constraints restrict which crops grow in which seasons. Harvested crops are
inventory items usable in cooking recipes (F-13.7.7). Farm plots are placed in player housing areas
or designated farming zones.

- **Requirements:** R-13.14.8
- **Dependencies:** F-13.14.5a (Housing), F-13.7.7 (Crafting Recipes), F-13.9.1 (Inventory)
- **Platform notes:** None

### F-13.14.9a Animal Needs and Happiness

Domesticated animals (chickens, cows, sheep, horses) have needs (hunger, happiness) that affect
production rate — fed and happy animals produce more. Animal care actions (feed, pet, clean) are
interactions (F-13.17.1) that affect happiness. Animals produce harvestable resources over time
(eggs, milk, wool) at rates scaling with happiness.

- **Requirements:** R-13.14.9a
- **Dependencies:** F-13.15.1 (Companion AI)
- **Platform notes:** None

### F-13.14.9b Animal Housing

Animals are housed in buildable structures (coops, barns, stables) placed in housing zones. Each
structure type has a capacity limit and is compatible with specific animal species. Animals are
purchased from NPC vendors, tamed from wild populations (F-13.15.4), or bred from existing stock.

- **Requirements:** R-13.14.9b
- **Dependencies:** F-13.14.9a, F-13.14.5a (Housing Plot and Instance System)
- **Platform notes:** None

### F-13.14.9c Animal Breeding

Breeding pairs produce offspring with inherited trait variations (color, size, productivity stats).
Breeding requires compatible species, a suitable housing structure, and a gestation timer. Trait
inheritance uses configurable genetic rules with random variation.

- **Requirements:** R-13.14.9c
- **Dependencies:** F-13.14.9a, F-13.14.9b
- **Platform notes:** None
