# 3.6 — Procedural Generation

## PCG Graph System

### F-3.6.1 Node-Based Procedural Content Graph

A visual node graph for authoring procedural generation rules. Nodes produce, filter, transform, and
consume point sets, meshes, and metadata. Subgraphs encapsulate reusable rule sets. Graphs execute
in the editor for interactive preview and at runtime for on-demand generation. Data flows between
nodes as typed streams (points, meshes, splines, heightmaps).

- **Requirements:** R-3.6.1
- **Dependencies:** F-1.1.1 (ECS)
- **Platform notes:** Graph complexity (max node count per evaluation) capped per platform tier.
  Mobile limits runtime graph depth to reduce CPU generation cost.

### F-3.6.2 Point Generation Nodes

Generate point distributions via surface sampling (terrain, mesh surface), volume sampling (bounded
box, sphere), grid (uniform, hex, Poisson disk), spline following (at intervals or uniform spacing),
and noise-driven density maps. Each point carries position, rotation, scale, and arbitrary attribute
key-value pairs for downstream filtering and spawning.

- **Requirements:** R-3.6.2
- **Dependencies:** F-3.6.1
- **Platform notes:** Point budget per generation pass scales per tier: mobile 10K, Switch 50K,
  desktop 500K+. Poisson disk sampling radius increased on mobile.

### F-3.6.3 Point Filtering and Transformation

Filter points by attribute predicates (height range, slope threshold, distance to spline, biome ID,
custom tags), density culling (probabilistic removal), spatial exclusion (minimum distance between
points), and bounding shapes. Transform nodes modify position (jitter, snap to surface), rotation
(align to normal, random yaw), and scale (random range, distance-based). All filtering is
non-destructive — disabled filters restore removed points.

- **Requirements:** R-3.6.3
- **Dependencies:** F-3.6.2
- **Platform notes:** Filter chain length limited on mobile to bound per-frame generation cost.
  Spatial exclusion queries use coarser grids on mobile.

### F-3.6.4 Mesh and Instance Spawning from Points

Convert filtered point sets into ECS entities with mesh, material, and transform components.
Spawning modes: static mesh instances (GPU instanced), ECS entity per instance (for gameplay
interaction), or hierarchical instanced static mesh (HISM) batches for rendering performance. Points
select mesh variants from weighted asset lists based on attributes.

- **Requirements:** R-3.6.4
- **Dependencies:** F-3.6.3, F-1.1.1 (ECS)
- **Platform notes:** Instance spawn count capped per tier. Mobile favors HISM batching over
  individual ECS entities to reduce entity overhead.

### F-3.6.5 Deterministic Seeding

All procedural operations accept a seed value. A global world seed combined with spatial coordinates
(chunk position) produces per-region seeds via hash (xxHash). Identical seeds produce identical
output regardless of generation order, platform, or thread scheduling. Seeds are exposed as
user-facing parameters for world sharing and reproducible testing.

- **Requirements:** R-3.6.5
- **Dependencies:** F-3.6.1
- **Platform notes:** Deterministic output identical across all platforms and CPU architectures. No
  platform-specific scaling required.

### F-3.6.6 Point Attributes and Metadata

Every generated point carries a typed attribute map: built-in properties (position, rotation, scale,
density, color, bounds, seed, index) plus arbitrary user-defined attributes (biome ID, species tag,
road type). Attributes propagate through the graph — filter and transform nodes read and write
attributes. Attribute operations include create, copy, reduce (min/max/average), partition (group by
attribute value), and noise (procedural attribute modification). Attributes drive downstream
decisions like mesh selection, material assignment, and spawn parameters.

- **Requirements:** R-3.6.6
- **Dependencies:** F-3.6.2
- **Platform notes:** Attribute count per point limited on mobile to reduce memory per-point.
  User-defined attributes capped at 8 on mobile, 32 on desktop.

### F-3.6.7 Point Set Operations

Boolean operations on point collections: union (combine), intersection (keep overlapping),
difference (remove overlapping), and merge (concatenate). Set operations use point bounds for
spatial overlap tests. Used for subtracting building footprints from vegetation, intersecting road
buffers with terrain features, and combining multi-source point clouds into unified distributions.

- **Requirements:** R-3.6.7
- **Dependencies:** F-3.6.2
- **Platform notes:** Set operation point budget inherits from F-3.6.2 per-tier limits. No
  additional platform scaling required.

### F-3.6.8 Graph Control Flow (Loops, Branches, Subgraphs)

Subgraphs encapsulate reusable node networks with typed input/output pins. Loop nodes iterate over
point collections element-by-element or by attribute-partitioned groups, with feedback pins carrying
data from the previous iteration. Branch nodes select execution paths based on attribute conditions.
Select nodes route data based on runtime parameters. Control flow enables recursive generation
patterns (L-systems, fractal subdivision).

- **Requirements:** R-3.6.8
- **Dependencies:** F-3.6.1
- **Platform notes:** Recursion depth and loop iteration limits configurable per tier. Mobile caps
  recursion at 4 levels; desktop supports 8+.

## Terrain Stamps

### F-3.6.9 Non-Destructive Terrain Stamp System

Layer-based terrain modification using stamp operations applied in order. Each stamp modifies
height, texture, or vegetation within a bounded region with configurable blend modes (add, subtract,
blend, flatten, smooth). Stamps compose non-destructively — reordering or removing a stamp rebuilds
the terrain from the remaining stack. Stamp prefabs encapsulate reusable terrain recipes.

- **Requirements:** R-3.6.9
- **Dependencies:** F-3.2.1 (Heightfield Terrain)
- **Platform notes:** Stamp resolution matches terrain tile resolution, which scales per platform
  tier (see F-3.2.3). Stamp stack depth identical across platforms.

### F-3.6.10 Terrain Texture Stamps

Procedural texturing driven by terrain analysis: height range, slope angle, curvature (convex vs
concave), compass orientation, and noise perturbation. Each texture stamp targets a terrain material
layer and applies within falloff-controlled boundaries. Multiple texture stamps compose with
priority ordering. Used for automatic snow-on-peaks, sand-in-valleys, and rock-on-cliffs patterns.

- **Requirements:** R-3.6.10
- **Dependencies:** F-3.6.9, F-3.2.5 (Terrain Materials)
- **Platform notes:** Texture stamp layer count inherits per-tier limits from F-3.2.5. Noise
  perturbation resolution reduced on mobile.

### F-3.6.11 Biome Distribution System

Define biomes as bundles of terrain stamps (height, texture, vegetation, objects) with climate
parameters (temperature, precipitation, elevation range). A Whittaker-diagram lookup maps per-region
temperature and moisture values to biome IDs. Temperature derives from latitude and elevation;
moisture derives from wind simulation over the heightmap. Biome boundaries blend with
noise-perturbed transition zones.

- **Requirements:** R-3.6.11
- **Dependencies:** F-3.6.9, F-3.6.10
- **Platform notes:** Biome simulation resolution (cells per km) reduced on mobile. Transition zone
  blending width simplified on mobile.

## Vegetation Placement

### F-3.6.12 Rule-Based Vegetation Placement

Scatter vegetation instances using per-species rules: density, slope range, altitude band, preferred
biome, inter-species dependencies (underbrush near trees), exclusion zones, and random
rotation/scale ranges. Rules are evaluated on the GPU via compute shader — scatter points generated
per terrain tile and streamed relative to camera. Supports hundreds of thousands of instances with
GPU instanced rendering.

- **Requirements:** R-3.6.12
- **Dependencies:** F-3.6.3, F-3.3.1 (Instanced Foliage), F-3.3.2 (Procedural Placement)
- **Platform notes:** Vegetation instance density scales per tier (inherits F-3.3.1 caps). Species
  variety per tile reduced on mobile.

### F-3.6.13 Vegetation Clearing Along Splines

Automatically remove or suppress vegetation within a configurable distance of road, river, and path
splines. Clearing width varies per spline segment and tapers at endpoints. Vegetation density ramps
from zero at the spline to full at the clearing boundary. Integrated with the stamp system so
clearing is non-destructive and updates live as splines are edited.

- **Requirements:** R-3.6.13
- **Dependencies:** F-3.6.12, F-3.6.16
- **Platform notes:** Clearing logic is lightweight on all platforms. SDF sampling cost is uniform.
  No platform-specific scaling required.

## Procedural Roads and Paths

### F-3.6.14 Spline-Based Road Generation

Create roads, paths, trails, sidewalks, and highways as spline entities. Road geometry is extruded
along the spline with configurable width, banking, lane count, curb height, and surface material.
Terrain deforms to match road elevation with configurable shoulder blend. Decorations (guard rails,
signs, lamp posts, lane markings) are placed procedurally along the road using attachment rules.

- **Requirements:** R-3.6.14
- **Dependencies:** F-3.6.1, F-3.2.1 (Terrain)
- **Platform notes:** Road mesh tessellation scales with terrain LOD tier. Decoration density (guard
  rails, signs) reduced on mobile.

### F-3.6.15 Road Network Generation

Generate connected road networks from population density maps, terrain analysis, and user- placed
control points. L-system agents trace primary roads connecting population centers following valleys
and avoiding steep slopes. Secondary roads fill enclosed regions with configurable patterns (grid,
radial, organic). Post-processing merges intersections, prunes dead ends, and smooths curves with
Catmull-Rom fitting.

- **Requirements:** R-3.6.15
- **Dependencies:** F-3.6.14
- **Platform notes:** Road network generation is offline/editor-only. Runtime road loading uses
  pre-baked data on all platforms.

### F-3.6.16 Spline SDF Optimization

Convert all splines in a terrain tile to a single signed distance field (SDF) texture. Any system
that queries distance-to-nearest-spline (vegetation clearing, texture blending, road influence)
samples one texture instead of evaluating hundreds of spline polynomials. SDFs are cached and
re-rendered only when a spline changes. Supports hundreds of road splines per tile.

- **Requirements:** R-3.6.16
- **Dependencies:** F-3.6.14
- **Platform notes:** SDF texture resolution per tile scales per tier: mobile 64x64, desktop
  256x256. Cache eviction more aggressive on mobile.

### F-3.6.17 Road Intersections and Junctions

Detect and resolve road intersections where splines cross or merge. Generate appropriate junction
geometry: T-junctions, four-way crossroads, roundabouts, and highway on/off ramps. Traffic signage
and road markings are placed automatically based on junction type and road hierarchy. Terrain
blending at intersections smooths height discontinuities.

- **Requirements:** R-3.6.17
- **Dependencies:** F-3.6.14, F-3.6.15
- **Platform notes:** Junction geometry complexity (polygon count) reduced on mobile. Intersection
  generation is offline on all platforms.

## Building Generation

### F-3.6.18 Shape Grammar Building Generator

Generate building facades using hierarchical split grammars. A building mass is split into floors,
floors into tiles, tiles into facade elements (windows, doors, balconies, cornices). Split
operations divide shapes along any axis by absolute or relative amounts. Parameterized rules control
building height, floor count, window density, style, and material palette. Grammars are defined as
data assets and can be mixed per-district.

- **Requirements:** R-3.6.18
- **Dependencies:** F-3.6.1
- **Platform notes:** Grammar rule depth and facade element density reduced on mobile to lower
  resulting mesh complexity. Editor-time generation on all platforms.

### F-3.6.19 Modular Building Assembly

Assemble buildings from modular asset packs with standardized connection points (sockets). Walls,
corners, floors, roofs, doors, and windows connect via matching socket IDs on shared faces.
Composition rules enforce valid connections and structural constraints. Supports multi- story
buildings, L-shaped and U-shaped floor plans, and interior room generation via recursive
subdivision.

- **Requirements:** R-3.6.19
- **Dependencies:** F-3.6.18
- **Platform notes:** Module piece count per building capped on mobile to limit draw calls. Interior
  room generation disabled on mobile for distant buildings.

## Wave Function Collapse

### F-3.6.20 2D Tile-Based WFC

Generate 2D tile layouts (dungeons, floor plans, puzzle levels, city blocks) using Wave Function
Collapse. Each cell in a 2D grid starts in superposition. The algorithm observes the minimum-entropy
cell, collapses it to a weighted-random tile, and propagates adjacency constraints to neighbors.
Supports rotation variants, frequency-weighted selection, and backtracking on contradiction.

- **Requirements:** R-3.6.20
- **Dependencies:** F-3.6.5 (Seeding)
- **Platform notes:** Grid size capped per tier: mobile 32x32, desktop 128x128+. Backtracking depth
  limited on mobile to bound generation time.

### F-3.6.21 3D Voxel WFC

Extend WFC to three dimensions using cubic tiles with six-face socket matching. Generates buildings,
cave systems, space stations, and multi-story interiors from modular tile sets. Pre-computed
adjacency lookup tables and bitwise propagation optimize constraint solving. Supports chunked
generation for large worlds — adjacent chunks share boundary constraints for seamless transitions.

- **Requirements:** R-3.6.21
- **Dependencies:** F-3.6.20
- **Platform notes:** 3D grid volume capped per tier: mobile 16x16x8, desktop 64x64x32. Adjacency
  lookup tables shared across platforms.

### F-3.6.22 WFC Constraint Painting

Allow designers to pre-constrain specific cells before running WFC — pin certain tiles in place
(doors, stairs, boss rooms) and let the algorithm fill the rest. Paint zones with tile subset
restrictions (e.g., "this area must be corridors only"). Constraints are validated before generation
to detect unsatisfiable configurations early.

- **Requirements:** R-3.6.22
- **Dependencies:** F-3.6.20, F-3.6.21
- **Platform notes:** Constraint painting is an editor-only feature. No platform-specific runtime
  scaling required.

## General-Purpose Modular Assembly

### F-3.6.23 Socket-Based Modular Assembly Engine

A general-purpose system for assembling complex objects from modular pieces via socket connections.
Any asset can define named sockets (attach points with position, rotation, type ID, and
compatibility rules). The assembly engine validates connections, resolves transforms, and merges
attached pieces into a single entity hierarchy. Used for assembling spaceships (hull + cockpit +
engines + weapons), naval vessels (hull + deck + mast + rigging), weapons (barrel + stock + scope +
magazine), armor sets, hand railings, and any modular object.

- **Requirements:** R-3.6.23
- **Dependencies:** F-3.6.1, F-1.1.14 (Relationships)
- **Platform notes:** Socket connection count per assembly capped on mobile to limit entity
  hierarchy depth. Assembly logic is platform-uniform.

### F-3.6.24 Procedural Object Generation Rules

Define assembly rules as PCG graph nodes that procedurally select and connect modular pieces. Rules
specify part categories per socket (e.g., "engine socket accepts any thruster-type part"), variation
weights, size constraints, and style tags. A generation pass walks the socket graph from a root
piece outward, selecting parts that satisfy constraints. Produces unique ships, vehicles, buildings,
weapons, and props from a shared part library.

- **Requirements:** R-3.6.24
- **Dependencies:** F-3.6.23, F-3.6.5 (Seeding)
- **Platform notes:** Part variety per socket reduced on mobile to limit combinatorial mesh variants
  in memory.

### F-3.6.25 Houdini Engine Procedural Object Pipeline

Connect Houdini Digital Assets to the modular assembly engine. Artists author procedural assembly
rules in Houdini (scatter engines on a hull, route pipes between components, generate railing along
a spline, place rivets on seams) and expose parameters. The engine evaluates the HDA via Houdini
Engine (F-12.6.3), receives the assembled geometry and instance data, and converts it to ECS
entities. Supports cook-on-parameter-change for interactive design.

- **Requirements:** R-3.6.25
- **Dependencies:** F-3.6.23, F-12.6.3 (Houdini Engine)
- **Platform notes:** Requires Houdini Engine license at edit time.

### F-3.6.26 Hierarchical Modular Composition

Compose assembled modular objects into larger modular assets recursively. A room is assembled from
wall/floor/ceiling pieces; rooms compose into a building floor; floors compose into a building;
buildings compose into a city block. Each composition level defines its own socket interface so it
can be used as a module at the next level up. Composition hierarchies are stored as asset prefabs —
an artist-authored tavern becomes a reusable module in a procedural village generator, which itself
becomes a module in a world-region generator.

- **Requirements:** R-3.6.26
- **Dependencies:** F-3.6.23, F-1.1.37 (Nested Prefabs)
- **Platform notes:** Composition hierarchy depth capped on mobile (3 levels vs 6+ on desktop).
  Deeper hierarchies flatten to merged meshes on mobile.

## Authoring Workflow

### F-3.6.27 Interactive PCG Authoring Tools

Editor tools for artists to drive PCG interactively: spline drawing (roads, rivers, fences), point
painting (scatter props by brushing), volume dragging (define generation regions), and socket wiring
(connect modular pieces by clicking sockets). Each tool is backed by a PCG graph with parameters
exposed in the inspector. Artists see results update in real-time as they paint, drag, or adjust
parameters — no code or graph editing required.

- **Requirements:** R-3.6.27
- **Dependencies:** F-3.6.1, F-15.1.1 (Editor Framework)
- **Platform notes:** Editor-only feature. Runs on desktop editor platforms only.

### F-3.6.28 Artist-Guided Constraint Authoring

Artists define high-level constraints (building footprints, road waypoints, landmark placements,
zone boundaries) and the PCG system fills in the rest procedurally. Constraints are ECS entities
with spatial components placed in the editor; the PCG graph reads them as fixed inputs and generates
content around them. Artists iterate by moving constraints and seeing the procedural output adapt
instantly. Combines hand-crafted hero content with procedural population.

- **Requirements:** R-3.6.28
- **Dependencies:** F-3.6.27, F-3.6.1
- **Platform notes:** Editor-only feature. Runs on desktop editor platforms only.

## AI-Driven Generation

### F-3.6.29 AI-Driven Content Generation

An AI agent interface that drives PCG graphs programmatically. AI systems (trained models or rule
engines) set PCG graph parameters, place constraints, select style presets, and evaluate output
quality metrics. Supports iterative generation: the AI generates a candidate, evaluates it against
objective functions (navigability, visual variety, gameplay flow), and re-generates with adjusted
parameters until quality thresholds are met.

- **Requirements:** R-3.6.29
- **Dependencies:** F-3.6.1, F-3.6.5 (Seeding)
- **Platform notes:** AI generation runs in editor or on server. Not a runtime client feature. No
  client platform scaling required.

### F-3.6.30 Constraint Satisfaction Solver

A general-purpose constraint solver for PCG that goes beyond WFC. Define spatial constraints
(minimum distance between buildings, roads must connect to at least two intersections, rivers flow
downhill, no floating structures) and the solver finds valid placements satisfying all constraints
simultaneously. Uses backtracking search with arc consistency pruning. Applicable to city layout,
dungeon connectivity, puzzle level design, and furniture placement.

- **Requirements:** R-3.6.30
- **Dependencies:** F-3.6.1
- **Platform notes:** Constraint variable count and backtracking depth capped on mobile to bound
  solve time. Desktop supports larger constraint networks.

## Runtime Generation

### F-3.6.31 Runtime Chunk-Based Procedural Generation

Generate world content at runtime as the player moves through the world. The world is divided into
fixed-size chunks; chunks generate on background threads (F-14.3.3) when they enter a configurable
activation radius ahead of the camera. Deterministic seeding (F-3.6.5) ensures revisited chunks
reproduce identically. A priority queue orders generation by distance and importance. Memory budgets
evict distant chunks.

- **Requirements:** R-3.6.31
- **Dependencies:** F-3.6.5, F-14.3.3 (Job System), F-3.2.1 (Heightfield Terrain)
- **Platform notes:** Chunk size and activation radius scale per tier: mobile uses larger chunks
  with smaller radius, desktop uses smaller chunks with larger radius. Generation time budget per
  frame: mobile 2ms, desktop 4-8ms.

### F-3.6.32 GPU Compute Procedural Generation

Generate heightmaps, normal maps, vegetation scatter, and noise fields on the GPU via compute
shaders. Fractal Brownian motion, Perlin, simplex, Worley, and domain-warped noise functions run
entirely on the GPU. Indirect dispatch drives vegetation instance buffer population. GPU generation
produces results in a single frame for seamless streaming without CPU stalls.

- **Requirements:** R-3.6.32
- **Dependencies:** F-3.6.31, F-2.1.1 (GPU Abstraction)
- **Platform notes:** Heightmap and noise generation resolution scales per tier: mobile 128x128,
  desktop 512x512+. Noise octave count reduced on mobile (4 vs 8).

### F-3.6.33 Noise Function Library

A comprehensive library of noise functions usable in PCG graphs, compute shaders, and CPU code:
value noise, Perlin, simplex, Worley/cellular, fractal Brownian motion (configurable octaves,
lacunarity, gain), domain warping, ridged multifractal, and billowed noise. All functions are
deterministic and produce identical results on CPU and GPU for a given seed.

- **Requirements:** R-3.6.33
- **Dependencies:** None
- **Platform notes:** CPU and GPU noise functions are platform-uniform and produce identical
  results. Octave count for fBm capped per tier (see F-3.6.32).

## Planet-Scale Generation

### F-3.6.34 Planetary Terrain Generation

Generate entire planet surfaces with continental landmasses, ocean basins, mountain ranges, river
networks, and climate zones. The generator operates on a spherical icosahedral mesh subdivided to
configurable detail levels, with tectonic plate simulation driving continent placement and mountain
formation. Climate simulation (latitude, altitude, ocean currents, prevailing winds) produces
temperature and moisture maps that drive biome assignment. Terrain detail is generated on-demand
using the chunk streaming system (F-3.6.31) — only terrain near the player is at full resolution;
distant terrain uses progressively lower LOD. Supports planets from Earth-scale to moon-scale.

- **Requirements:** R-3.6.34
- **Dependencies:** F-3.6.1 (PCG Graph), F-3.6.5 (Deterministic Seeding), F-3.6.33 (Noise Library),
  F-3.6.31 (Runtime Chunk Streaming)
- **Platform notes:** Icosahedral subdivision level scales per tier: mobile 4-5 levels, desktop 7-8.
  Detail generation radius reduced on mobile.

### F-3.6.35 City and Settlement Generation

Procedurally generate cities, towns, villages, and outposts with road networks, building lots,
zoning (residential, commercial, industrial, civic), and population-appropriate density. Road
generation uses L-system patterns constrained by terrain (roads follow valleys, bridge rivers,
switchback up slopes). Building lots are subdivided from city blocks and filled with structures from
the shape grammar system (F-3.6.18) or modular building system (F-3.6.19). Settlement size scales
with population — hamlets have a few buildings on a dirt road; capital cities have districts, walls,
markets, and landmarks. Settlements connect to the global road network (F-3.6.15).

- **Requirements:** R-3.6.35
- **Dependencies:** F-3.6.15 (Road Network), F-3.6.18 (Shape Grammar Buildings), F-3.6.19 (Modular
  Buildings), F-3.6.34 (Planetary Terrain)
- **Platform notes:** Building detail level per settlement reduced on mobile. Distant settlements
  use impostor cards on mobile instead of full geometry.

### F-3.6.36 Faction and Civilization Generation

Generate geopolitical entities — factions, nations, tribes, guilds — with territories, diplomatic
relationships, military strength, economic resources, and cultural traits. Each faction controls
settlements (F-3.6.35), claims territory on the world map, and has relationships (allied, neutral,
hostile, vassal, trade partner) with other factions. Faction generation uses constraint-based
placement: factions spawn in defensible terrain, near resources, with buffer zones between rivals.
Cultural traits (architecture style, banner colors, naming conventions, preferred unit types) are
selected from trait pools and propagate to all faction-owned content. Faction borders are visible on
the world map and influence enemy spawning, NPC behavior, and available quests.

- **Requirements:** R-3.6.36
- **Dependencies:** F-3.6.34 (Planetary Terrain), F-3.6.35 (City Generation), F-3.6.5 (Deterministic
  Seeding)
- **Platform notes:** Faction generation is offline/server-side. Runtime faction data is a
  lightweight lookup on all platforms.

### F-3.6.37 Procedural Quest Generation

Generate quest content from narrative templates parameterized by world state. Templates define quest
structure (fetch, escort, kill, investigate, defend, craft, deliver) with slots for: objective
location (selected from known POIs), target entities (selected from spawned enemies or NPCs), reward
items (selected from loot tables F-13.7.8), dialogue text (generated from narrative templates with
faction/location substitution), and difficulty (scaled to player level and zone danger rating).
Quests chain into arcs through prerequisite graphs. Generated quests reference the quest/dialogue
system (F-13.6.1) for runtime tracking and presentation.

- **Requirements:** R-3.6.37
- **Dependencies:** F-3.6.36 (Factions), F-3.6.35 (Settlements), F-13.6.1 (Quest System), F-13.7.8
  (Loot Tables)
- **Platform notes:** Quest generation is server-side. Client receives quest data as lightweight
  records. No client platform scaling required.

### F-3.6.38 Dynamic Ecosystem Simulation

Simulate wildlife populations, migration patterns, predator-prey dynamics, and resource cycles
across the generated world. Each ecosystem region tracks population counts per species, food
availability, and environmental health. Predator-prey relationships follow Lotka-Volterra dynamics
with spatial movement — herds migrate seasonally, predators follow prey, and overgrazing depletes
regions. Players interact with ecosystems: hunting reduces populations, building disrupts habitats,
and conservation quests restore balance. Ecosystem state is persistent and affects loot availability
(rare creatures become rarer when overhunted), enemy encounter rates, and ambient world atmosphere.

- **Requirements:** R-3.6.38
- **Dependencies:** F-3.6.34 (Planetary Terrain), F-3.6.5 (Deterministic Seeding)
- **Platform notes:** Ecosystem simulation is server-side. Clients receive population snapshots. No
  client platform scaling required.

### F-3.6.39 Civilization Time-Scale Simulation

Run the world simulation forward through historical time before the player enters, generating a
lived-in world with history, ruins, cultural artifacts, and geopolitical context. The simulation
advances in epoch steps: factions rise and fall, wars reshape borders, trade routes form between
cities, abandoned settlements become dungeons, and natural disasters reshape terrain. The resulting
world state includes: historical ruins (explorable dungeons with faction-era loot), cultural layers
(architecture styles change over time), legendary NPCs (generated historical figures referenced in
lore), and ongoing conflicts rooted in simulated history. Inspired by Dwarf Fortress world
generation. Simulation depth (number of epochs) is configurable to balance world richness against
generation time.

- **Requirements:** R-3.6.39
- **Dependencies:** F-3.6.36 (Factions), F-3.6.35 (Cities), F-3.6.38 (Ecosystems), F-3.6.34
  (Planetary Terrain)
- **Platform notes:** History simulation is server/editor batch process. Clients receive
  pre-computed history data. No client platform scaling required.

### F-3.6.40 Procedural Enemy and Creature Placement

Populate the generated world with enemies, wildlife, and NPCs based on zone danger ratings, faction
control, biome types, and ecosystem state (F-3.6.38). Placement rules define per- creature: valid
biomes, altitude range, time-of-day activity, pack size distribution, patrol routes (along roads or
territory boundaries), lair locations (caves, ruins, camps), and respawn behavior. Boss creatures
are placed at significant POIs (dungeon endpoints, mountain peaks, faction strongholds) with unique
loot tables. Placement density scales with zone level and player proximity — distant zones use
sparse representations until the player approaches.

- **Requirements:** R-3.6.40
- **Dependencies:** F-3.6.34 (Planetary Terrain), F-3.6.36 (Factions), F-3.6.38 (Ecosystems),
  F-3.6.4 (Spawning)
- **Platform notes:** Active creature instance count capped per tier: mobile 50-100, Switch 200,
  desktop 1000+. Placement density radius reduced on mobile.

### F-3.6.41 Procedural Loot and Economy Distribution

Distribute loot, resources, and economic value across the generated world based on zone difficulty,
faction wealth, and ecosystem rarity. Resource nodes (mining, herbalism, woodcutting) are placed
according to biome and geological rules. Loot quality scales with zone danger rating and distance
from starting areas. Faction-controlled zones feature faction-themed loot (weapons, armor, crafting
materials matching the faction's cultural style). Economy balancing ensures that currency inflow
(loot sell value, quest rewards) and outflow (vendor prices, crafting costs, repair fees) maintain
target ratios per player progression stage. References gameplay database loot tables (F-13.7.8) and
currency systems (F-13.7.6).

- **Requirements:** R-3.6.41
- **Dependencies:** F-3.6.34 (Planetary Terrain), F-3.6.36 (Factions), F-13.7.8 (Loot Tables),
  F-13.7.6 (Currency Definitions)
- **Platform notes:** Loot distribution is server-authoritative. Client receives loot records on
  demand. No client platform scaling required.

### F-3.6.42 Plate Tectonics and Geological Simulation

Simulate planetary geological processes to generate physically plausible continent shapes, mountain
ranges, and geological features. The generator models tectonic plates on the planetary sphere:
plates move, collide, subduct, and rift over simulated geological time. Convergent boundaries
produce mountain ranges and volcanic arcs (Himalayas, Andes). Divergent boundaries produce rift
valleys and mid-ocean ridges. Transform boundaries produce fault lines (San Andreas). Hotspot
volcanism produces island chains (Hawaii). The simulation runs in configurable time steps (millions
of years per step) and outputs: plate boundary map, elevation field driven by collision and erosion,
volcanic activity zones, earthquake fault lines, and mineral deposit locations. Planet type
parameters (rocky, super-earth, tidally locked, binary star) influence plate count, activity level,
and geological timescale. All parameters are exposed in the PCG graph editor for artist control.

- **Requirements:** R-3.6.42
- **Dependencies:** F-3.6.34 (Planetary Terrain), F-3.6.1 (PCG Graph), F-3.6.5 (Deterministic
  Seeding)
- **Platform notes:** Tectonic simulation is an editor/server batch process. Results are baked data
  consumed at runtime on all platforms.

### F-3.6.43 Climate and Atmospheric Simulation

Simulate planetary climate to generate biome distribution based on physical constraints. The climate
model considers: latitude (solar energy decreases toward poles), altitude (temperature drops with
elevation), ocean currents (warm/cold currents modify coastal temperatures), prevailing winds
(Hadley, Ferrel, and polar cells drive moisture transport), rain shadow effects (mountains block
moisture, creating deserts on leeward sides), and axial tilt (seasonal variation). For multi-sun
systems, the model computes combined solar flux based on orbital parameters. The simulation outputs
per-cell: temperature range (min/max across seasons), precipitation (annual rainfall), humidity,
wind direction and strength, and a derived Koppen climate classification. These drive biome
assignment (F-3.6.44) and vegetation distribution. Climate parameters are configurable: greenhouse
factor, ocean coverage percentage, axial tilt, orbital eccentricity, and number/position of suns.

- **Requirements:** R-3.6.43
- **Dependencies:** F-3.6.42 (Plate Tectonics), F-3.6.34 (Planetary Terrain), F-3.6.5 (Seeding)
- **Platform notes:** Climate simulation is an editor/server batch process. Runtime clients read
  pre-baked climate maps on all platforms.

### F-3.6.44 Biome Classification and Distribution

Classify terrain cells into biomes based on climate simulation output (F-3.6.43) and elevation.
Supported biome types: tropical rainforest (jungle), tropical savanna, hot desert, cold desert,
Mediterranean, temperate oceanic forest, temperate continental forest, boreal/taiga, tundra, ice
cap, steppe/grassland, temperate grassland (prairie/plains), wetland/swamp, mangrove, alpine meadow,
and volcanic wasteland. Each biome defines: dominant vegetation types, soil color, rock types,
wildlife species pools, ambient audio profile, weather patterns, and resource node distributions.
Biome boundaries use gradient transitions with configurable ecotone width — forests gradually thin
into grassland rather than switching abruptly. Micro-biomes (oases in deserts, bogs in forests,
thermal vents in tundra) spawn based on local water table and geological features. Biome data drives
the terrain material splatmap (F-3.2.5), vegetation placement (F-3.6.12), and creature spawning
(F-3.6.40).

- **Requirements:** R-3.6.44
- **Dependencies:** F-3.6.43 (Climate Simulation), F-3.2.5 (Terrain Materials), F-3.6.12 (Vegetation
  Placement), F-3.6.40 (Creature Placement)
- **Platform notes:** Biome classification is pre-baked. Runtime biome lookups are lightweight
  texture samples on all platforms.

### F-3.6.45 Hydrological System and Water Body Generation

Simulate water flow across the generated terrain to produce rivers, lakes, watersheds, and coastal
features. The hydrological model: (1) distributes precipitation from the climate simulation across
the terrain, (2) traces water flow downhill using steepest-descent drainage, (3) accumulates flow
into streams that merge into rivers following terrain valleys, (4) fills depressions to form lakes
(endorheic basins in deserts, glacial lakes in mountains), (5) computes erosion along river paths
(widening valleys, depositing sediment in deltas and floodplains), and (6) generates coastal
features where rivers meet oceans (estuaries, deltas, lagoons, bays). Water body types: rivers (with
flow rate, width, and depth varying along length), lakes (freshwater and salt lakes based on
drainage), swamps and marshes (flat terrain with high water table), underground aquifers (visible as
springs and oases), waterfalls (where rivers cross elevation discontinuities), and fjords (glacially
carved coastal inlets). River networks integrate with the road generation system (F-3.6.15) for
bridge placement and the settlement system (F-3.6.35) for riverside city placement.

- **Requirements:** R-3.6.45
- **Dependencies:** F-3.6.43 (Climate), F-3.6.42 (Geology), F-3.6.34 (Planetary Terrain), F-3.4.1
  (FFT Ocean), F-3.6.15 (Road Network), F-3.6.35 (City Generation)
- **Platform notes:** Hydrological simulation is an editor/server batch process. Runtime river and
  lake data is streamed as pre-baked splines and meshes.

### F-3.6.46 Geological Landform Generation

Generate detailed landform features driven by the geological and hydrological simulations. Feature
types organized by formation process: **Tectonic**: mountain ranges, volcanoes, fault scarps, rift
valleys, plateaus, mesas, buttes (erosion-isolated plateaus). **Erosive**: canyons (river-carved),
badlands (soft rock erosion), sea cliffs, arches, hoodoos, karst topography (sinkholes, caves,
limestone pillars). **Glacial**: U-shaped valleys, cirques, moraines, drumlins, fjords, icebergs
(calving from glacial fronts). **Coastal**: beaches (sandy, rocky, pebble), peninsulas, barrier
islands, lagoons, atolls, sea stacks, bays, inlets, tombolo (sand bar connecting island to
mainland). **Volcanic**: calderas, lava fields, volcanic islands, geysers, hot springs, obsidian
flows. **Aeolian**: sand dunes (barchan, star, linear), loess plains. Each landform has a placement
rule based on geological context (buttes only in arid sedimentary terrain, fjords only in glaciated
coasts) and a mesh/heightmap generator that produces the specific geometry. Landform density and
scale are configurable per region in the PCG graph.

- **Requirements:** R-3.6.46
- **Dependencies:** F-3.6.42 (Plate Tectonics), F-3.6.45 (Hydrology), F-3.6.34 (Planetary Terrain),
  F-3.6.33 (Noise Library)
- **Platform notes:** Landform generation is editor/server batch. Runtime landform meshes use
  standard LOD streaming on all platforms.

### F-3.6.47 Earth Import and GIS Data Integration

Import real-world geographic data to generate Earth-accurate terrain or use real-world data as a
reference for procedural generation. Supported data sources: high-resolution heightmap data (SRTM
30m, ASTER GDEM, ALOS), satellite imagery (for terrain material reference), OpenStreetMap data
(roads, buildings, water bodies, land use), and climate datasets (WorldClim, CRU). The import
pipeline reprojects geographic coordinates (WGS84, UTM) to the engine's world-space coordinate
system. Users can import a rectangular region of Earth at configurable resolution, generating
terrain heightfields, water body placement, road networks, and settlement locations from real data.
The imported data can be used as-is for realistic settings or as a seed for procedural elaboration —
import Earth's continent shapes but generate fantasy biomes, cities, and civilizations on top.
Import is a batch process in the editor, not a runtime feature.

- **Requirements:** R-3.6.47
- **Dependencies:** F-3.6.34 (Planetary Terrain), F-3.6.45 (Hydrology), F-3.6.35 (Cities), F-3.6.15
  (Road Network), F-12.1.1 (Native Asset Ingestion)
- **Platform notes:** SRTM and ASTER data are publicly available from USGS. OpenStreetMap data uses
  the OSM XML/PBF format. Processing large GIS datasets (continental scale) may require significant
  disk space and processing time.

### F-3.6.48 Configurable Planet Parameters

Expose all planetary generation parameters as a unified planet configuration asset authored in the
visual editor. Parameters include: planet radius and mass (affects gravity, atmosphere thickness),
axial tilt (drives seasonal variation), orbital period and eccentricity, rotation period (day
length), number of suns (single, binary, trinary) with relative positions and luminosities, number
of moons (tidal effects on oceans), ocean coverage percentage, atmospheric composition (affects sky
color, greenhouse warming), tectonic activity level (0 = dead world, 1 = Earth-like, 2 =
hyperactive), magnetic field strength (affects radiation and aurora), and age (young planets have
more volcanic activity, old planets have more erosion). Preset planet templates provide starting
points: Earth-like, Mars-like (thin atmosphere, no oceans, ancient riverbeds), ocean world (>90%
water), ice world (frozen surface, subsurface ocean), desert world (Arrakis-style, no surface
water), jungle world (high humidity, dense vegetation), and volcanic world (active tectonics, lava
fields). All parameters feed into the geological (F-3.6.42), climate (F-3.6.43), and biome
(F-3.6.44) simulation pipelines.

- **Requirements:** R-3.6.48
- **Dependencies:** F-3.6.42 (Plate Tectonics), F-3.6.43 (Climate), F-3.6.44 (Biomes), F-3.6.34
  (Planetary Terrain)
- **Platform notes:** Planet configuration is an editor/authoring asset. Runtime parameter lookups
  are uniform across platforms.

## Stellar and Planetary Formation

### F-3.6.49 Star System Generation and Stellar Lifecycle

Generate entire star systems from nebula-to-main-sequence using astrophysical models. The generator
simulates: stellar nucleosynthesis classification (O, B, A, F, G, K, M spectral types) based on
initial mass, main sequence lifetime derived from mass-luminosity relations (massive stars burn
brighter but shorter), stellar evolution phases (protostar, main sequence, red giant, white dwarf
for solar-mass; supergiant, supernova, neutron star or black hole for massive stars), and
binary/trinary star orbital mechanics (Keplerian orbits with configurable eccentricity, period, and
inclination). Each star's luminosity, temperature, color, and habitable zone boundaries are derived
from its spectral type and age. The system generates a complete stellar neighborhood: single stars,
binary pairs, and star clusters with physically-consistent orbital parameters. Star data feeds into
the climate simulation (F-3.6.43) for illumination and the sky rendering system (F-3.5.1) for
visible stellar objects. Artists override any derived value through the PCG graph editor.

- **Requirements:** R-3.6.49
- **Dependencies:** F-3.6.48 (Configurable Planet Parameters), F-3.6.5 (Deterministic Seeding),
  F-3.5.1 (Procedural Sky)
- **Platform notes:** Star system generation is editor/server batch. Runtime star catalog is a
  lightweight lookup on all platforms.

### F-3.6.50 Protoplanetary Disk and Accretion Simulation

Simulate planetary system formation from a protoplanetary disk around a newly formed star. The
simulation models: disk mass distribution (denser material closer to the star), dust grain accretion
into planetesimals (runaway growth model), planetesimal collisions forming protoplanets (oligarchic
growth), gas giant formation via core accretion (rocky core captures gas envelope beyond the frost
line) or disk instability (direct gravitational collapse for massive disks), terrestrial planet
formation via giant impacts (late-stage collisions that determine final mass, composition, and moon
formation), orbital migration (planets drift inward or outward due to disk interactions, producing
hot Jupiters or resonant chains), and orbital clearing (planets sweep their orbital zones,
establishing stable spacing). The simulation outputs: planet count, orbital radii, masses,
compositions (rocky, ice giant, gas giant), rotation rates, axial tilts, and moon systems. Each
output planet becomes an input to the planetary terrain generator (F-3.6.34) or gas giant renderer
(F-3.6.52). The simulation is deterministic from a seed and configurable parameters (disk mass,
metallicity, star mass).

- **Requirements:** R-3.6.50
- **Dependencies:** F-3.6.49 (Star System), F-3.6.5 (Deterministic Seeding)
- **Platform notes:** Accretion simulation is editor/server batch. Planet orbital data is a
  lightweight lookup at runtime on all platforms.

### F-3.6.51 Planetary Collision and Giant Impact Simulation

Simulate large-scale planetary collisions that shape planetary formation outcomes. The collision
model handles: giant impacts (Mars-sized body striking a proto-Earth, producing Earth + Moon),
grazing collisions (partial mass transfer, debris rings), head-on mergers (combined mass, molten
surface), and catastrophic disruptions (target shattered into asteroid field). Collision physics
uses a simplified SPH (smoothed particle hydrodynamics) model operating on a coarse particle grid
(thousands, not millions of particles) to determine mass distribution, angular momentum transfer,
and debris trajectories in reasonable generation time. Collision outcomes determine: planet mass and
density, moon formation (debris disk coalesces into moons), axial tilt (off-center impacts tilt the
rotation axis), rotation rate (impacts spin up or slow down rotation), and surface state (recent
impacts produce magma oceans that cool over geological time). Collision history is recorded and
available to the civilization simulation (F-3.6.39) as ancient geological events.

- **Requirements:** R-3.6.51
- **Dependencies:** F-3.6.50 (Accretion), F-3.6.42 (Plate Tectonics)
- **Platform notes:** SPH simulation is compute-intensive; generation runs as a background task in
  the editor, not at runtime.

### F-3.6.52 Gas Giant and Non-Terrestrial Planet Generation

Generate planets that are fundamentally unlike Earth: gas giants (Jupiter, Saturn-type with banded
atmospheres, ring systems, and dozens of moons), ice giants (Uranus, Neptune-type with thick ice
mantles and sparse ring systems), airless rocky bodies (Mercury, Moon-type with cratered surfaces,
no atmosphere, extreme temperature swings), frozen worlds (Europa-type with ice shell over
subsurface ocean), volcanic hellscapes (Io-type with tidal heating driving constant eruptions),
hothouse worlds (Venus-type with runaway greenhouse, crushing atmosphere, surface volcanism), and
tidally locked worlds (one face permanently lit, the other in eternal darkness, with a habitable
twilight zone). Each planet type has a dedicated terrain generator: gas giants produce atmospheric
band textures with storm systems (Great Red Spot analogs) and ring particle distributions; airless
bodies produce cratered heightfields via impact simulation; frozen worlds produce cracked ice
terrain with cryovolcanic features. The planet type is automatically selected based on mass, orbital
position, and composition from the accretion simulation (F-3.6.50), or manually chosen by the
artist.

- **Requirements:** R-3.6.52
- **Dependencies:** F-3.6.50 (Accretion), F-3.6.34 (Planetary Terrain), F-3.6.48 (Planet Parameters)
- **Platform notes:** Gas giant atmospheric rendering uses volumetric shader techniques from F-3.5.3
  (Volumetric Clouds) adapted for planetary scale.

### F-3.6.53 Moon and Ring System Generation

Generate moon systems and ring systems for planets based on formation history. Moon generation
considers: capture (irregular orbits, retrograde), co-formation (regular orbits in the equatorial
plane), and giant impact debris (large moon from collision, like Earth's Moon from F-3.6.51). Each
moon is a full planet entity with its own terrain generation pipeline — rocky moons get cratered
surfaces, icy moons get cracked ice terrain, large moons can have atmospheres and biomes. Tidal
interactions between planet and moons drive: tidal heating (volcanism on close moons like Io), tidal
locking (moon always shows same face), ocean tides on the parent planet, and orbital resonances
(Laplace resonance stabilizes moon orbits). Ring systems are generated from: disrupted moons (Roche
limit breakup), captured asteroid debris, or primordial disk remnants. Rings are rendered as
particle distributions with gap structures from shepherd moon gravitational influence. Ring shadow
casting onto the parent planet uses the shadow map system.

- **Requirements:** R-3.6.53
- **Dependencies:** F-3.6.52 (Gas Giants), F-3.6.50 (Accretion), F-3.6.51 (Collisions), F-3.6.34
  (Planetary Terrain)
- **Platform notes:** Moon/ring generation is editor/server batch. Ring particle render count and
  resolution scale per platform tier at runtime.

### F-3.6.54 Automatic Planet Type Classification

Automatically determine appropriate planet types and features based on physical constraints without
manual selection. The classifier evaluates: orbital position relative to the frost line (determines
rocky vs gas/ice giant), stellar flux (determines habitable zone, too hot, too cold), planetary mass
(determines atmosphere retention — low mass loses atmosphere, high mass retains thick atmosphere or
becomes gas giant), magnetic field (determines atmospheric erosion rate from stellar wind), age
(determines geological activity level and atmospheric evolution), and tidal forces from nearby
bodies (determines tidal locking and heating). The classifier assigns: surface type (rocky, gaseous,
icy, molten), atmosphere type (none, thin, Earth-like, thick, crushing), hydrosphere type (none,
subsurface ocean, surface oceans, global ocean), geological activity level, and habitability class
(uninhabitable, marginally habitable, habitable, garden world). Artists can accept the
classification or override any parameter. The classifier is exposed as a PCG graph node that takes
orbital and physical parameters as inputs and outputs the planet configuration.

- **Requirements:** R-3.6.54
- **Dependencies:** F-3.6.49 (Star System), F-3.6.50 (Accretion), F-3.6.48 (Planet Parameters)
- **Platform notes:** Classification is editor/server batch. Output is a small data record consumed
  at runtime on all platforms.

## Cosmological and Galactic Simulation

### F-3.6.55 Galaxy Structure Generation

Generate galaxies with physically-motivated structure: spiral (density wave arms), elliptical,
irregular, lenticular, and dwarf types. Star density, metallicity, and age vary with galactic
radius. The galaxy is sectorized for LOD streaming — only nearby sectors resolve to individual star
systems. Multiple galaxies support galaxy cluster scenarios. A supermassive black hole anchors each
galactic core (F-3.6.56).

- **Requirements:** R-3.6.55
- **Dependencies:** F-3.6.49 (Star Systems), F-3.6.5 (Seeding), F-3.6.31 (Chunk Streaming)
- **Platform notes:** Galaxy generation is editor/server batch. Runtime sector streaming count
  scales per tier: mobile loads fewer sectors simultaneously.

### F-3.6.56 Supermassive Black Hole and Galactic Core

Generate supermassive black holes at galactic centers with mass correlated to bulge mass (M-sigma
relation). Render gravitational lensing (ray-deflection shader), accretion disks (volumetric), and
relativistic jets. Three accretion states: dormant, active (Seyfert), quasar. Tidal disruption
events destroy stars passing too close. Time dilation near the event horizon enables gameplay
mechanics.

- **Requirements:** R-3.6.56
- **Dependencies:** F-3.6.55 (Galaxy), F-3.6.49 (Stars), F-3.5.1 (Procedural Sky)
- **Platform notes:** Gravitational lensing uses screen-space ray deflection.

### F-3.6.57 Dark Matter Halo and Large-Scale Structure

Model dark matter as invisible mass shaping galactic rotation curves (NFW/Einasto profiles), galaxy
cluster binding, and cosmic web structure (filaments, voids, nodes). Dark matter density peaks
determine galaxy placement. Generated from initial perturbation field via Zel'dovich approximation.
Never rendered directly but affects gravitational lensing and galaxy distribution.

- **Requirements:** R-3.6.57
- **Dependencies:** F-3.6.55 (Galaxy), F-3.6.5 (Seeding)
- **Platform notes:** Cosmological N-body runs as batch editor process.

### F-3.6.58 Stellar Collision and Merger Simulation

Simulate stellar collisions: mergers (blue stragglers), common envelope, neutron star mergers
(kilonovae), white dwarf supernovae (Type Ia), and tidal captures. Outcomes modify the star catalog
— new objects, debris nebulae, local metallicity enrichment. Collision probability scales with
stellar density (high in globular clusters, negligible in outskirts).

- **Requirements:** R-3.6.58
- **Dependencies:** F-3.6.49 (Stars), F-3.6.55 (Galaxy)
- **Platform notes:** Stellar collision simulation is editor/server batch. No runtime client cost.

### F-3.6.59 Black Hole Formation and Collision

Generate stellar-mass black holes from massive star deaths (direct collapse, supernova fallback).
Simulate black hole mergers: gravitational wave chirp inspiral, remnant mass/spin from fitting
formulae, gravitational recoil kicks. Supermassive black hole growth via gas accretion and galaxy
mergers. Events logged in universe history for civilization simulation reference.

- **Requirements:** R-3.6.59
- **Dependencies:** F-3.6.56 (SMBH), F-3.6.49 (Stars), F-3.6.58 (Stellar Collisions)
- **Platform notes:** Black hole simulation is editor/server batch. Gravitational lensing rendering
  cost scales per tier (see F-3.6.56).

### F-3.6.60 Universe Generation Pipeline

Top-down pipeline from Big Bang to present: (1) primordial density perturbations, (2) dark matter
evolution into cosmic web, (3) galaxy formation at density peaks, (4) stellar populations with
lifecycles and collisions, (5) planetary systems via accretion, (6) surface detail (geology,
climate, biomes, hydrology, landforms), (7) optional civilization seeding. Configurable scope:
single planet to observable universe. LOD ensures only the local region is detailed. Deterministic
from a single master seed.

- **Requirements:** R-3.6.60
- **Dependencies:** F-3.6.57, F-3.6.55, F-3.6.49, F-3.6.50, F-3.6.54, F-3.6.42, F-3.6.43, F-3.6.44,
  F-3.6.45, F-3.6.46, F-3.6.36, F-3.6.39
- **Platform notes:** Full universe generation is a batch editor process. Runtime uses LOD
  streaming.

## Planetary Composition and Resources

### F-3.6.61 Planetary Mineralogy and Resource Distribution

Generate per-planet mineral and resource compositions based on the physical formation history from
the accretion simulation (F-3.6.50), stellar metallicity (F-3.6.49), and collision history
(F-3.6.51). The composition model tracks elemental abundances inherited from the protoplanetary disk
— planets forming closer to the star are enriched in refractory elements (iron, nickel, silicon,
aluminum) while planets beyond the frost line accumulate volatiles (water ice, ammonia, methane) and
lighter elements. Stellar metallicity determines the baseline abundance: metal-rich stars produce
planets with more heavy elements (gold, platinum, rare earths), while metal-poor stars yield
resource-scarce worlds. Giant impacts (F-3.6.51) redistribute material — a differentiated planet's
core is iron-rich and mantle is silicate-rich; a giant impact can expose core material at the
surface (creating iron-rich terrain patches) or strip the mantle entirely (Mercury-like iron world).
Geological processes further concentrate minerals: plate tectonics (F-3.6.42) produce ore deposits
at subduction zones (copper, gold, tin) and volcanic arcs (sulfur, obsidian). Hydrothermal activity
concentrates metals along mid-ocean ridges and hot springs. Sedimentary processes create fossil fuel
deposits, limestone, and evaporite minerals (salt, gypsum) in ancient lake beds. The output is a
per-region mineral density map consumed by the resource node system (F-13.14.7) for gameplay
resource placement — mining in geologically appropriate locations yields richer deposits. Each
planet's unique mineral fingerprint enables trade economies between worlds (F-13.7.6) where one
planet exports rare metals another lacks. Mineral data is exposed in the PCG graph for artist
override.

- **Requirements:** R-3.6.61
- **Dependencies:** F-3.6.50 (Accretion), F-3.6.49 (Star Systems), F-3.6.51 (Collisions), F-3.6.42
  (Plate Tectonics), F-3.6.45 (Hydrology), F-13.14.7 (Resource Gathering), F-13.7.6 (Currency/Trade)
- **Platform notes:** Mineralogy generation is editor/server batch. Runtime resource node lookups
  are lightweight on all platforms.

## Universe Infrastructure

### F-3.6.62 Server-Side Universe Generation and Sharding

Universe generation runs on a server cluster, storing results in a shared database. Game servers
request pre-generated data for their spatial shard rather than regenerating. Shards align with
natural boundaries (galaxy sectors, star system Oort clouds). Unexplored regions generate on demand
from deterministic seeds and cache for subsequent requests. Generation workers scale horizontally.

- **Requirements:** R-3.6.62
- **Dependencies:** F-3.6.60 (Universe Pipeline), F-8.7.1 (World Sharding), F-8.7.5 (Persistent
  State), F-8.7.8 (Inter-Server Bus)
- **Platform notes:** Generation servers run headless. Database uses PostgreSQL (F-15.12.7).

### F-3.6.63 Sparse Cosmic Data Storage

Store universe data in sparse hierarchical octrees with variable-depth subdivision. Dense regions
subdivide deeply; intergalactic voids are single empty nodes. Position keys use 128-bit fixed-point
coordinates (sub-meter precision at cosmic scale). Star catalogs use columnar compressed storage
with delta encoding. Unvisited planets store only orbital parameters and seed (under 1KB).
Integrates with streaming (F-12.5.1) and persistent state (F-8.7.5).

- **Requirements:** R-3.6.63
- **Dependencies:** F-3.6.60 (Universe Pipeline), F-1.7.9 (Arbitrary Precision), F-3.2.7 (Large
  World Coords), F-12.5.1 (VFS), F-8.7.5 (Persistent State)
- **Platform notes:** 128-bit keys CPU-only. GPU uses camera-relative f64->f32.

### F-3.6.64 On-Demand Hierarchical Detail Resolution

Resolve universe detail across 6+ LOD tiers (cosmic web -> galaxy -> sector -> star system -> planet
orbit -> surface). Each tier generates on demand from seeds or fetches from cache (F-3.6.62).
Leaving a region releases data back to seed-only. Total loaded data stays within memory budget.
Prefetching predicts exploration direction and pre-generates the next tier.

- **Requirements:** R-3.6.64
- **Dependencies:** F-3.6.62 (Server Generation), F-3.6.63 (Sparse Storage), F-3.6.60 (Pipeline),
  F-12.5.6 (Priority Streaming)
- **Platform notes:** Loaded tier count scales per tier: mobile keeps fewer LOD tiers resident.
  Prefetch distance reduced on mobile to save memory.
