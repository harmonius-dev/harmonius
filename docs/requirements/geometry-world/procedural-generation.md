# R-3.6 — Procedural Generation Requirements

## R-3.6.1 Node-Based Procedural Content Graph

The engine **SHALL** provide a visual node graph for authoring procedural generation rules,
where nodes produce, filter, transform, and consume typed streams (points, meshes, splines,
heightmaps), with subgraph encapsulation and execution in both editor and runtime contexts.

- **Derived from:** [F-3.6.1](../../features/geometry-world/procedural-generation.md)
- **Rationale:** A node graph is the foundation for all PCG features, enabling visual authoring,
  reusable subgraphs, and consistent execution across editor preview and runtime generation.
- **Verification:** Integration test — build a graph with a point generator, filter, and mesh
  spawner node; execute in editor and at runtime; assert identical output entity counts and
  positions; verify subgraph encapsulation produces equivalent results to the inlined graph.

## R-3.6.2 Point Generation Nodes

The engine **SHALL** generate point distributions via surface sampling, volume sampling, grid
layouts (uniform, hex, Poisson disk), spline following, and noise-driven density maps, with
each point carrying position, rotation, scale, and arbitrary key-value attributes.

- **Derived from:** [F-3.6.2](../../features/geometry-world/procedural-generation.md)
- **Rationale:** Point generation is the primary input to all downstream PCG operations;
  multiple distribution modes cover terrain scattering, volume filling, and path following.
- **Verification:** Unit test — generate Poisson disk points on a flat surface; assert minimum
  inter-point distance meets the configured radius; verify all points carry the expected
  built-in attributes (position, rotation, scale).

## R-3.6.3 Point Filtering and Transformation

The engine **SHALL** filter points by attribute predicates, density culling, spatial exclusion,
and bounding shapes, and transform point position, rotation, and scale via jitter, snap-to-
surface, align-to-normal, and random-range operations, all non-destructively.

- **Derived from:** [F-3.6.3](../../features/geometry-world/procedural-generation.md)
- **Rationale:** Non-destructive filtering and transformation enable iterative refinement of
  point distributions without regeneration.
- **Verification:** Unit test — generate 1000 points; apply a slope-range filter excluding
  slopes > 45 degrees; assert all surviving points have slope <= 45; disable the filter and
  assert the original 1000 points are restored.

## R-3.6.4 Mesh and Instance Spawning from Points

The engine **SHALL** convert filtered point sets into ECS entities with mesh, material, and
transform components, supporting GPU-instanced, per-entity, and hierarchical instanced static
mesh (HISM) spawning modes with attribute-driven mesh variant selection.

- **Derived from:** [F-3.6.4](../../features/geometry-world/procedural-generation.md)
- **Rationale:** Point-to-entity conversion bridges procedural generation with the ECS runtime,
  and multiple spawning modes balance interactivity against rendering performance.
- **Verification:** Integration test — spawn 500 points in HISM mode and 500 in per-entity
  mode; assert entity counts match; verify mesh variant selection follows the configured
  attribute-based weights within statistical tolerance.

## R-3.6.5 Deterministic Seeding

The engine **SHALL** produce identical procedural output for identical seed values regardless
of generation order, platform, or thread scheduling, using a global world seed combined with
spatial coordinates via xxHash to derive per-region seeds.

- **Derived from:** [F-3.6.5](../../features/geometry-world/procedural-generation.md)
- **Rationale:** Deterministic seeding enables reproducible worlds for testing, world sharing,
  and consistent chunk regeneration during streaming.
- **Verification:** Unit test — generate the same region on two platforms (or two thread
  orderings) with the same seed; assert all output point positions and attributes are
  bit-identical.

## R-3.6.6 Point Attributes and Metadata

The engine **SHALL** maintain a typed attribute map on every generated point (built-in: position,
rotation, scale, density, color, bounds, seed, index; plus user-defined) with create, copy,
reduce, partition, and noise operations that propagate through the graph.

- **Derived from:** [F-3.6.6](../../features/geometry-world/procedural-generation.md)
- **Rationale:** Rich point attributes drive downstream decisions for mesh selection, material
  assignment, and spawn parameters across all PCG workflows.
- **Verification:** Unit test — create a user attribute "biome_id"; apply a partition operation;
  assert downstream nodes receive correctly partitioned point subsets grouped by biome_id.

## R-3.6.7 Point Set Operations

The engine **SHALL** support boolean operations on point collections (union, intersection,
difference, merge) using point bounds for spatial overlap tests.

- **Derived from:** [F-3.6.7](../../features/geometry-world/procedural-generation.md)
- **Rationale:** Set operations enable combining and subtracting point distributions (e.g.,
  removing vegetation under building footprints).
- **Verification:** Unit test — create two overlapping point sets A and B; assert union count
  equals A + B minus overlap; assert intersection contains only overlapping points; assert
  difference of A minus B excludes all points within B's bounds.

## R-3.6.8 Graph Control Flow (Loops, Branches, Subgraphs)

The engine **SHALL** support subgraph encapsulation with typed I/O pins, loop nodes with
per-element and partition-group iteration with feedback pins, branch nodes for conditional
execution, and select nodes for parameter-based data routing.

- **Derived from:** [F-3.6.8](../../features/geometry-world/procedural-generation.md)
- **Rationale:** Control flow constructs enable recursive generation patterns (L-systems,
  fractal subdivision) and conditional logic without custom code.
- **Verification:** Integration test — build an L-system graph using loop and branch nodes
  with 4 iterations; assert output geometry matches the expected fractal pattern; verify
  feedback pins carry state correctly between iterations.

## R-3.6.9 Non-Destructive Terrain Stamp System

The engine **SHALL** modify terrain height, texture, and vegetation via ordered stamp operations
with configurable blend modes (add, subtract, blend, flatten, smooth) that compose non-
destructively, rebuilding terrain when stamps are reordered or removed.

- **Derived from:** [F-3.6.9](../../features/geometry-world/procedural-generation.md)
- **Rationale:** Non-destructive stamps enable iterative terrain authoring where any change
  can be undone or reordered without data loss.
- **Verification:** Integration test — apply three height stamps in order A-B-C; remove stamp
  B; assert the resulting terrain equals stamps A and C applied in order; verify reordering
  to C-A produces a different but valid result.

## R-3.6.10 Terrain Texture Stamps

The engine **SHALL** apply procedural terrain texturing driven by height range, slope angle,
curvature, compass orientation, and noise perturbation, targeting material layers within
falloff-controlled boundaries with priority-ordered composition.

- **Derived from:** [F-3.6.10](../../features/geometry-world/procedural-generation.md)
- **Rationale:** Automated terrain texturing based on terrain analysis eliminates manual
  painting for large worlds while producing natural material distribution patterns.
- **Verification:** Integration test — configure a snow texture stamp for altitudes > 2000m
  and slopes < 30 degrees; render terrain with peaks above and below 2000m; assert snow
  material is present only on qualifying surfaces.

## R-3.6.11 Biome Distribution System

The engine **SHALL** assign biome IDs to world regions using a Whittaker-diagram lookup from
temperature (latitude + elevation) and moisture (wind simulation over heightmap) values, with
noise-perturbed transition zone blending at biome boundaries.

- **Derived from:** [F-3.6.11](../../features/geometry-world/procedural-generation.md)
- **Rationale:** Climate-driven biome distribution produces ecologically plausible world variety
  that scales to planetary terrain without manual painting.
- **Verification:** Integration test — generate terrain with equatorial lowlands and polar
  highlands; assert equatorial lowland biome IDs differ from polar highland biome IDs; assert
  transition zones at biome boundaries have blended stamp contributions from both biomes.

## R-3.6.12 Rule-Based Vegetation Placement

The engine **SHALL** scatter vegetation instances using per-species rules (density, slope range,
altitude band, biome, inter-species dependencies, exclusion zones, random transform ranges)
evaluated on GPU compute, supporting hundreds of thousands of instances with GPU instanced
rendering.

- **Derived from:** [F-3.6.12](../../features/geometry-world/procedural-generation.md)
- **Rationale:** GPU-evaluated placement rules enable dense, biome-appropriate vegetation
  across large terrain areas without CPU bottlenecks.
- **Verification:** Integration test — configure a tree species restricted to slopes < 30
  degrees and altitude 200-800m; scatter across varied terrain; assert no instances appear
  outside the specified slope and altitude ranges; verify instance count exceeds 100,000
  without dropping below target frame rate.

## R-3.6.13 Vegetation Clearing Along Splines

The engine **SHALL** suppress vegetation within a configurable, per-segment distance of road,
river, and path splines, with density ramping from zero at the spline to full at the clearing
boundary, integrated non-destructively with the stamp system.

- **Derived from:** [F-3.6.13](../../features/geometry-world/procedural-generation.md)
- **Rationale:** Roads and rivers must have cleared vegetation for visual correctness, and
  non-destructive integration ensures clearing updates live as splines are edited.
- **Verification:** Integration test — place a road spline through dense vegetation; assert
  vegetation density is zero within the configured clearing width and ramps to full density
  at the boundary; move the spline and assert clearing updates accordingly.

## R-3.6.14 Spline-Based Road Generation

The engine **SHALL** generate road geometry extruded along spline entities with configurable
width, banking, lane count, curb height, and surface material, deforming terrain to match road
elevation with shoulder blending, and placing decorations procedurally via attachment rules.

- **Derived from:** [F-3.6.14](../../features/geometry-world/procedural-generation.md)
- **Rationale:** Spline-based roads are the primary infrastructure element for open-world
  environments, requiring integrated terrain deformation and procedural decoration.
- **Verification:** Integration test — create a road spline over hilly terrain; assert road
  surface is continuous with no gaps; assert terrain height matches road elevation within
  the shoulder blend distance; verify decorations (guard rails, signs) appear at configured
  intervals.

## R-3.6.15 Road Network Generation

The engine **SHALL** generate connected road networks from population density maps and terrain
analysis using L-system agents for primary roads and configurable fill patterns (grid, radial,
organic) for secondary roads, with intersection merging, dead-end pruning, and curve smoothing.

- **Derived from:** [F-3.6.15](../../features/geometry-world/procedural-generation.md)
- **Rationale:** Automated road network generation is required for procedural cities and large
  worlds where manual road placement is infeasible.
- **Verification:** Integration test — generate a road network from three population centers
  on varied terrain; assert all centers are connected via primary roads; assert secondary
  roads form closed blocks; verify no road segment exceeds the configured maximum slope.

## R-3.6.16 Spline SDF Optimization

The engine **SHALL** convert all splines in a terrain tile to a single cached signed distance
field texture, re-rendered only when a spline changes, enabling distance-to-nearest-spline
queries via a single texture sample instead of per-spline polynomial evaluation.

- **Derived from:** [F-3.6.16](../../features/geometry-world/procedural-generation.md)
- **Rationale:** SDF caching replaces O(N) spline distance evaluations with O(1) texture
  lookups, enabling scalable vegetation clearing and texture blending near hundreds of roads.
- **Verification:** Unit test — create 100 splines in a tile; generate the SDF texture; sample
  at 50 random positions and assert distance values match brute-force polynomial evaluation
  within 0.5m tolerance; modify one spline and assert only the affected SDF is re-rendered.

## R-3.6.17 Road Intersections and Junctions

The engine **SHALL** detect spline crossings and merges, generate appropriate junction geometry
(T-junctions, crossroads, roundabouts, ramps), place traffic signage and markings based on
junction type and road hierarchy, and smooth terrain height at intersections.

- **Derived from:** [F-3.6.17](../../features/geometry-world/procedural-generation.md)
- **Rationale:** Intersections are the most visually complex road elements and require automated
  geometry and signage generation to maintain consistency across generated networks.
- **Verification:** Integration test — create two crossing road splines; assert a crossroads
  junction is generated with connected geometry; verify signage entities are spawned; verify
  terrain height is continuous across the junction area.

## R-3.6.18 Shape Grammar Building Generator

The engine **SHALL** generate building facades via hierarchical split grammars that divide
building mass into floors, floors into tiles, and tiles into facade elements, controlled by
parameterized data-asset rules for height, floor count, window density, style, and materials.

- **Derived from:** [F-3.6.18](../../features/geometry-world/procedural-generation.md)
- **Rationale:** Shape grammars produce architecturally varied buildings from compact rule sets,
  enabling diverse cityscapes without individual building modeling.
- **Verification:** Integration test — execute a grammar with 5-floor, 3-window-per-floor
  parameters; assert the output mesh has 5 horizontal divisions and 15 window elements;
  verify two different style assets produce visually distinct facades.

## R-3.6.19 Modular Building Assembly

The engine **SHALL** assemble buildings from modular asset packs via socket-ID-matched
connection points, enforcing valid connections and structural constraints, supporting multi-
story buildings, non-rectangular floor plans, and interior room generation via recursive
subdivision.

- **Derived from:** [F-3.6.19](../../features/geometry-world/procedural-generation.md)
- **Rationale:** Modular assembly enables large building variety from a limited asset library
  while enforcing structural validity through socket constraints.
- **Verification:** Integration test — assemble a 3-story L-shaped building from a modular
  kit; assert all socket connections have matching IDs; assert no overlapping geometry; verify
  interior rooms are generated via subdivision with valid door connectivity.

## R-3.6.20 2D Tile-Based WFC

The engine **SHALL** generate 2D tile layouts using Wave Function Collapse with minimum-entropy
cell observation, weighted-random tile collapse, adjacency constraint propagation, rotation
variants, and backtracking on contradiction.

- **Derived from:** [F-3.6.20](../../features/geometry-world/procedural-generation.md)
- **Rationale:** WFC produces globally coherent tile layouts for dungeons, floor plans, and city
  blocks from local adjacency rules, avoiding the repetition of purely random placement.
- **Verification:** Unit test — run WFC on a 20x20 grid with a 10-tile set; assert all cells
  are collapsed; assert every adjacent cell pair satisfies the defined adjacency constraints;
  run twice with the same seed and assert identical output.

## R-3.6.21 3D Voxel WFC

The engine **SHALL** extend WFC to three dimensions using cubic tiles with six-face socket
matching, supporting chunked generation with boundary-constraint sharing for seamless
cross-chunk transitions.

- **Derived from:** [F-3.6.21](../../features/geometry-world/procedural-generation.md)
- **Rationale:** 3D WFC enables generation of multi-story interiors, cave systems, and complex
  structures that cannot be expressed in 2D tile grids.
- **Verification:** Integration test — generate two adjacent 8x8x8 chunks; assert all six-face
  adjacency constraints are satisfied within each chunk and across the shared boundary;
  verify no contradictions occur during propagation.

## R-3.6.22 WFC Constraint Painting

The engine **SHALL** allow designers to pre-constrain WFC cells by pinning specific tiles and
painting zone restrictions before generation, validating constraints for satisfiability before
execution.

- **Derived from:** [F-3.6.22](../../features/geometry-world/procedural-generation.md)
- **Rationale:** Pre-constraints enable hand-crafted landmarks (boss rooms, stairs, doors)
  within otherwise procedural layouts, blending authored and generated content.
- **Verification:** Integration test — pin a "boss room" tile at cell (5,5) and restrict a
  region to corridor-only tiles; run WFC; assert cell (5,5) contains the pinned tile and
  the restricted region contains only corridor tiles; verify unsatisfiable constraints are
  detected before generation begins.

## R-3.6.23 Socket-Based Modular Assembly Engine

The engine **SHALL** provide a general-purpose modular assembly system where assets define named
sockets (position, rotation, type ID, compatibility rules) and the engine validates connections,
resolves transforms, and merges attached pieces into entity hierarchies.

- **Derived from:** [F-3.6.23](../../features/geometry-world/procedural-generation.md)
- **Rationale:** A generic socket system enables modular composition of any object type
  (vehicles, weapons, armor, buildings) from shared part libraries.
- **Verification:** Integration test — define a hull asset with two engine sockets; attach
  engine parts with matching socket type IDs; assert transforms are resolved correctly and
  the merged entity hierarchy contains all pieces; verify mismatched socket types are rejected.

## R-3.6.24 Procedural Object Generation Rules

The engine **SHALL** define assembly rules as PCG graph nodes that procedurally select and
connect modular pieces per socket, respecting part-category constraints, variation weights,
size limits, and style tags to produce unique assembled objects from a shared part library.

- **Derived from:** [F-3.6.24](../../features/geometry-world/procedural-generation.md)
- **Rationale:** Procedural assembly rules generate high-variety objects (ships, vehicles,
  weapons) without manual per-variant authoring.
- **Verification:** Integration test — generate 100 ships from a 20-part library with
  deterministic seeding; assert all socket constraints are satisfied; assert at least 80%
  of generated ships are visually distinct (differ in at least one part selection).

## R-3.6.25 Houdini Engine Procedural Object Pipeline

The engine **SHALL** evaluate Houdini Digital Assets via Houdini Engine to drive modular
assembly, receiving assembled geometry and instance data and converting results to ECS
entities, with cook-on-parameter-change for interactive design.

- **Derived from:** [F-3.6.25](../../features/geometry-world/procedural-generation.md)
- **Rationale:** Houdini integration leverages artist-authored procedural rules for complex
  assembly patterns (pipe routing, rivet placement) beyond what the built-in PCG graph covers.
- **Verification:** Integration test — load an HDA that scatters engine instances on a hull;
  modify an exposed parameter; assert the HDA re-cooks and produces updated ECS entities
  with correct transforms matching the HDA output.

## R-3.6.26 Hierarchical Modular Composition

The engine **SHALL** compose assembled modular objects into larger modular assets recursively,
where each composition level defines a socket interface usable as a module at the next level,
with composition hierarchies stored as reusable asset prefabs.

- **Derived from:** [F-3.6.26](../../features/geometry-world/procedural-generation.md)
- **Rationale:** Hierarchical composition scales modular assembly from individual objects to
  rooms, buildings, districts, and world regions through recursive reuse.
- **Verification:** Integration test — compose wall pieces into a room, rooms into a floor,
  floors into a building; assert each level exposes valid sockets; assert the final building
  entity hierarchy contains all constituent pieces with correct world-space transforms.

## R-3.6.27 Interactive PCG Authoring Tools

The engine **SHALL** provide editor tools for spline drawing, point painting, volume dragging,
and socket wiring backed by PCG graphs with inspector-exposed parameters, updating results
in real-time as artists interact without requiring code or graph editing.

- **Derived from:** [F-3.6.27](../../features/geometry-world/procedural-generation.md)
- **Rationale:** No-code interactive tools are essential for artist productivity and align with
  the engine's no-code design philosophy.
- **Verification:** Integration test — use the point painting tool to brush 200 scatter points
  on terrain; adjust a density parameter in the inspector; assert point count updates within
  one editor frame; verify no code or graph editing is required to complete the workflow.

## R-3.6.28 Artist-Guided Constraint Authoring

The engine **SHALL** allow artists to place high-level constraints (building footprints, road
waypoints, landmarks, zone boundaries) as ECS entities with spatial components that the PCG
graph reads as fixed inputs, regenerating surrounding content in real-time when constraints
are moved.

- **Derived from:** [F-3.6.28](../../features/geometry-world/procedural-generation.md)
- **Rationale:** Constraint-based authoring combines hand-crafted hero content with procedural
  population, giving artists control over key landmarks while automating the rest.
- **Verification:** Integration test — place a landmark constraint entity; run the PCG graph;
  assert generated content respects the landmark's exclusion zone; move the landmark and
  assert surrounding content regenerates to accommodate the new position.

## R-3.6.29 AI-Driven Content Generation

The engine **SHALL** expose a programmatic interface for AI agents to set PCG graph parameters,
place constraints, select style presets, evaluate output quality metrics, and iteratively
regenerate until configurable quality thresholds are met.

- **Derived from:** [F-3.6.29](../../features/geometry-world/procedural-generation.md)
- **Rationale:** AI-driven generation enables automated world population at scale with quality
  guarantees that manual or purely random generation cannot provide.
- **Verification:** Integration test — configure an AI agent with a navigability quality metric;
  generate a dungeon layout; assert the agent iterates at least twice and the final layout's
  navigability score exceeds the configured threshold.

## R-3.6.30 Constraint Satisfaction Solver

The engine **SHALL** provide a general-purpose constraint solver using backtracking search with
arc consistency pruning that enforces spatial constraints (minimum distances, connectivity
requirements, elevation rules, structural validity) across placed objects.

- **Derived from:** [F-3.6.30](../../features/geometry-world/procedural-generation.md)
- **Rationale:** A general solver handles complex spatial relationships (city layout, dungeon
  connectivity, furniture placement) that exceed WFC's tile-adjacency model.
- **Verification:** Unit test — define constraints: minimum 50m between buildings, all roads
  connect to at least two intersections, rivers flow downhill; run the solver; assert all
  constraints are satisfied in the output; verify unsatisfiable constraint sets report failure.

## R-3.6.31 Runtime Chunk-Based Procedural Generation

The engine **SHALL** generate world content at runtime in fixed-size chunks on background
threads when chunks enter an activation radius, using deterministic seeding for reproducible
revisits, with priority ordering by distance and memory-budget-driven eviction of distant
chunks.

- **Derived from:** [F-3.6.31](../../features/geometry-world/procedural-generation.md)
- **Rationale:** Runtime chunk generation enables infinite worlds without pre-baking, and
  deterministic seeding ensures revisited areas are identical.
- **Verification:** Integration test — move the camera through 20 chunk boundaries; assert each
  chunk generates before entering the visible range; revisit a previous chunk and assert
  output is identical to the first visit; verify distant chunks are evicted when the memory
  budget is exceeded.

## R-3.6.32 GPU Compute Procedural Generation

The engine **SHALL** generate heightmaps, normal maps, vegetation scatter, and noise fields
on the GPU via compute shaders, producing results within a single frame for seamless streaming
without CPU stalls.

- **Derived from:** [F-3.6.32](../../features/geometry-world/procedural-generation.md)
- **Rationale:** GPU generation eliminates CPU bottlenecks for data-parallel procedural work,
  enabling single-frame chunk generation that avoids visible pop-in during streaming.
- **Verification:** Performance test — generate a 512x512 heightmap via GPU compute; assert
  completion within one frame (< 16.6ms); verify output matches CPU reference within floating
  point tolerance.

## R-3.6.33 Noise Function Library

The engine **SHALL** provide value noise, Perlin, simplex, Worley/cellular, fractal Brownian
motion, domain warping, ridged multifractal, and billowed noise functions usable in PCG graphs,
compute shaders, and CPU code, with deterministic cross-platform results for a given seed.

- **Derived from:** [F-3.6.33](../../features/geometry-world/procedural-generation.md)
- **Rationale:** A comprehensive noise library is a foundational dependency for terrain, cloud,
  texture, and placement generation across all PCG systems.
- **Verification:** Unit test — evaluate each noise function at 1000 sample points on CPU and
  GPU with the same seed; assert results are bit-identical across CPU, GPU, and platforms.

## R-3.6.34 Planetary Terrain Generation

The engine **SHALL** generate planet surfaces on a spherical icosahedral mesh with tectonic-
plate-driven continent placement, climate simulation (latitude, altitude, ocean currents, wind)
producing temperature/moisture maps for biome assignment, and on-demand detail generation via
chunk streaming with progressive LOD for distant terrain.

- **Derived from:** [F-3.6.34](../../features/geometry-world/procedural-generation.md)
- **Rationale:** Planet-scale generation is the top-level world creation system that drives all
  downstream terrain, biome, settlement, and ecosystem generation.
- **Verification:** Integration test — generate a planet with at least 3 distinct continents;
  assert biome distribution correlates with latitude and altitude; verify terrain detail
  increases as the camera approaches the surface; verify distant terrain renders at reduced
  LOD without visual discontinuities.

## R-3.6.35 City and Settlement Generation

The engine **SHALL** procedurally generate settlements with road networks, building lots, zoning
(residential, commercial, industrial, civic), and population-scaled density, using terrain-
constrained L-system road patterns and shape grammar or modular building systems for structure
generation.

- **Derived from:** [F-3.6.35](../../features/geometry-world/procedural-generation.md)
- **Rationale:** Procedural settlements populate the generated world with inhabited areas that
  scale from hamlets to capital cities based on population parameters.
- **Verification:** Integration test — generate a settlement with population 10,000; assert
  road network connects all districts; assert building lot count scales with population;
  verify zoning produces distinct residential and commercial areas; verify settlement
  connects to the global road network.

## R-3.6.36 Faction and Civilization Generation

The engine **SHALL** generate factions with territories, diplomatic relationships, military
strength, economic resources, and cultural traits (architecture style, banners, naming), placed
via constraint-based positioning in defensible terrain near resources with buffer zones between
rivals.

- **Derived from:** [F-3.6.36](../../features/geometry-world/procedural-generation.md)
- **Rationale:** Factions provide geopolitical context that drives NPC behavior, quest
  generation, enemy spawning, and visual world variety through cultural theming.
- **Verification:** Integration test — generate 5 factions on a continent; assert each controls
  at least one settlement; assert diplomatic relationships exist between all faction pairs;
  verify faction cultural traits propagate to owned settlement architecture; verify buffer
  zones separate rival faction territories.

## R-3.6.37 Procedural Quest Generation

The engine **SHALL** generate quests from narrative templates parameterized by world state,
with slots for objective location, target entities, reward items, dialogue text, and difficulty
scaled to player level, chaining into arcs via prerequisite graphs.

- **Derived from:** [F-3.6.37](../../features/geometry-world/procedural-generation.md)
- **Rationale:** Procedural quests provide infinite replayable content tailored to the generated
  world state, faction context, and player progression.
- **Verification:** Integration test — generate 10 quests for a faction-controlled zone; assert
  each quest references valid POIs, spawned entities, and loot table entries in the generated
  world; verify quest difficulty scales with the zone danger rating; verify prerequisite
  chains form a valid DAG.

## R-3.6.38 Dynamic Ecosystem Simulation

The engine **SHALL** simulate wildlife populations per region with predator-prey dynamics
(Lotka-Volterra), seasonal migration, food availability, and player interaction effects
(hunting depletes populations, conservation restores balance), persisting ecosystem state
across sessions.

- **Derived from:** [F-3.6.38](../../features/geometry-world/procedural-generation.md)
- **Rationale:** Dynamic ecosystems make the world feel alive and responsive to player actions,
  affecting loot availability, encounter rates, and ambient atmosphere.
- **Verification:** Integration test — initialize a region with 100 prey and 20 predators;
  advance 100 simulation ticks; assert population oscillates per Lotka-Volterra dynamics;
  simulate hunting that removes 50% of prey; assert prey population decreases and predator
  population subsequently declines; verify state persists across save/load.

## R-3.6.39 Civilization Time-Scale Simulation

The engine **SHALL** simulate world history in configurable epoch steps before player entry,
producing ruins, cultural layers, legendary NPCs, and ongoing conflicts, with factions rising
and falling, wars reshaping borders, trade routes forming, and abandoned settlements becoming
dungeons.

- **Derived from:** [F-3.6.39](../../features/geometry-world/procedural-generation.md)
- **Rationale:** Pre-simulated history creates a lived-in world with discoverable lore, ancient
  ruins, and geopolitical context that enriches exploration and quest content.
- **Verification:** Integration test — run 10 epoch steps; assert at least one faction has
  fallen and its settlements are marked as ruins; assert trade routes connect surviving
  faction cities; verify increasing epoch count produces more historical artifacts; verify
  generation time scales linearly with epoch count.

## R-3.6.40 Procedural Enemy and Creature Placement

The engine **SHALL** place enemies, wildlife, and NPCs based on zone danger rating, faction
control, biome type, and ecosystem state, with per-creature rules for valid biomes, altitude,
time-of-day activity, pack size, patrol routes, and lair locations, scaling density with zone
level and player proximity.

- **Derived from:** [F-3.6.40](../../features/geometry-world/procedural-generation.md)
- **Rationale:** Context-aware creature placement ensures encounters are appropriate to the
  environment, faction, and player progression level.
- **Verification:** Integration test — configure a forest biome zone at danger level 5; place
  creatures; assert all spawned creatures have "forest" in their valid biome list; assert
  creature levels are within +/- 2 of zone danger level; verify boss creatures appear only
  at designated POIs; verify distant zones use sparse placement until player approaches.

## R-3.6.41 Procedural Loot and Economy Distribution

The engine **SHALL** distribute loot, resources, and economic value across the world based on
zone difficulty, faction wealth, biome, and geological rules, with faction-themed loot,
quality scaling with zone danger and distance from start, and currency inflow/outflow balancing
per player progression stage.

- **Derived from:** [F-3.6.41](../../features/geometry-world/procedural-generation.md)
- **Rationale:** Balanced loot and economy distribution ensures player progression is rewarding
  and consistent across procedurally generated worlds.
- **Verification:** Integration test — generate loot for zones at danger levels 1, 5, and 10;
  assert average loot quality increases with danger level; assert faction-controlled zones
  contain faction-themed items; verify currency inflow/outflow ratio stays within configured
  target range across 100 simulated player-hours.

## R-3.6.42 Plate Tectonics and Geological Simulation

The engine **SHALL** simulate tectonic plate movement on a planetary sphere with configurable
plate count, producing convergent boundaries (mountain ranges), divergent boundaries (rift
valleys), transform faults, and hotspot volcanism. Geological simulation **SHALL** accept planet
type parameters (rocky, super-earth, tidally locked) that influence plate behavior. The
simulation **SHALL** be deterministic given identical seed and parameters.

- **Derived from:** [F-3.6.42](../../features/geometry-world/procedural-generation.md)
- **Rationale:** Physically-driven geology produces more believable continent shapes and mountain
  placement than pure noise.
- **Verification:** Generate two planets with identical seeds; verify identical plate boundaries
  and elevation fields. Verify convergent boundaries produce elevated terrain and divergent
  boundaries produce depressions.

## R-3.6.43 Climate and Atmospheric Simulation

The engine **SHALL** simulate planetary climate considering latitude, altitude, ocean currents,
prevailing winds, rain shadow effects, and axial tilt. The simulation **SHALL** output per-cell
temperature range, precipitation, humidity, and wind data. Multi-sun systems **SHALL** compute
combined solar flux from configurable orbital parameters. Climate **SHALL** produce Koppen-
classified biome maps that match physical expectations (deserts leeward of mountains, jungles
near equator with high rainfall).

- **Derived from:** [F-3.6.43](../../features/geometry-world/procedural-generation.md)
- **Rationale:** Climate simulation ensures biome placement follows physical rules rather than
  arbitrary noise, producing worlds that feel geographically coherent.
- **Verification:** Generate a planet with a mountain range; verify desert biome forms on the
  leeward side. Verify equatorial regions receive higher temperatures than polar regions. Verify
  coastal regions have moderated temperatures compared to continental interiors.

## R-3.6.44 Biome Classification and Distribution

The engine **SHALL** classify terrain into at least 16 distinct biome types based on climate
data, with gradient ecotone transitions of configurable width between adjacent biomes. Micro-
biomes (oases, bogs, thermal vents) **SHALL** spawn based on local geological and hydrological
features. Each biome **SHALL** define vegetation types, soil color, wildlife pools, and weather
patterns that drive downstream terrain material, vegetation, and creature placement systems.

- **Derived from:** [F-3.6.44](../../features/geometry-world/procedural-generation.md)
- **Rationale:** Diverse biomes with smooth transitions create visually rich worlds. Micro-biomes
  add local interest and exploration rewards.
- **Verification:** Generate a planet; verify at least 16 biome types are present. Verify biome
  boundaries use gradient transitions (no hard edges). Verify an oasis spawns in a desert region
  near a water source.

## R-3.6.45 Hydrological System and Water Body Generation

The engine **SHALL** simulate precipitation-driven water flow producing rivers, lakes, watersheds,
and coastal features. Rivers **SHALL** follow terrain valleys with accumulating flow (width and
depth increase downstream). Lakes **SHALL** form in terrain depressions. Coastal features
(estuaries, deltas, lagoons) **SHALL** form where rivers meet oceans. The hydrological system
**SHALL** compute erosion that modifies the terrain heightfield along river paths.

- **Derived from:** [F-3.6.45](../../features/geometry-world/procedural-generation.md)
- **Rationale:** Physically-simulated hydrology produces river networks that look natural and
  enables gameplay around water resources, fishing, and naval transport.
- **Verification:** Generate terrain with mountains and precipitation; verify rivers flow downhill,
  merge at confluences, and widen toward the ocean. Verify lakes form in closed basins. Verify
  delta sediment deposits at river mouths.

## R-3.6.46 Geological Landform Generation

The engine **SHALL** generate at least 30 distinct landform types classified by formation process
(tectonic, erosive, glacial, coastal, volcanic, aeolian). Each landform **SHALL** have placement
rules based on geological context (buttes in arid sedimentary terrain, fjords in glaciated
coasts, dunes in arid regions with sand supply). Landform geometry **SHALL** be generated as
heightmap modifications or mesh overlays that integrate seamlessly with the base terrain.

- **Derived from:** [F-3.6.46](../../features/geometry-world/procedural-generation.md)
- **Rationale:** Specific landforms (canyons, mesas, fjords, volcanic calderas) create memorable
  landmarks and diverse gameplay environments.
- **Verification:** Generate a planet with varied geology; identify and verify at least 30
  distinct landform types are placed in geologically appropriate contexts. Verify a canyon forms
  where a river crosses resistant rock, and fjords form on glaciated coasts.

## R-3.6.47 Earth Import and GIS Data Integration

The engine **SHALL** import real-world geographic data from SRTM/ASTER heightmaps, satellite
imagery, and OpenStreetMap datasets. The import pipeline **SHALL** reproject WGS84/UTM
coordinates to engine world-space. Imported regions **SHALL** produce terrain heightfields,
water bodies, road networks, and settlement locations that match the source data within 10m
horizontal accuracy at SRTM 30m resolution. Imported data **SHALL** be usable as-is or as a
seed for procedural elaboration.

- **Derived from:** [F-3.6.47](../../features/geometry-world/procedural-generation.md)
- **Rationale:** Real-world data enables realistic settings (historical games, flight simulators)
  and provides reference for procedural world design.
- **Verification:** Import a 100km x 100km region of Earth; compare generated terrain to source
  heightmap; verify RMS elevation error under 5m. Verify roads and water bodies match
  OpenStreetMap source data.

## R-3.6.48 Configurable Planet Parameters

The engine **SHALL** expose a unified planet configuration asset with parameters for: radius,
mass, axial tilt, orbital period, rotation period, sun count and luminosities, moon count,
ocean coverage, atmospheric composition, tectonic activity, magnetic field, and planetary age.
At least 7 preset planet templates **SHALL** be provided (Earth-like, Mars-like, ocean world,
ice world, desert world, jungle world, volcanic world). All parameters **SHALL** feed into the
geological, climate, and biome simulation pipelines and produce visually distinct worlds.

- **Derived from:** [F-3.6.48](../../features/geometry-world/procedural-generation.md)
- **Rationale:** Configurable parameters enable diverse sci-fi and fantasy settings without
  requiring manual terrain authoring for each world type.
- **Verification:** Generate planets using each of the 7 presets; verify each produces visually
  distinct terrain, climate, and biome distributions. Verify changing axial tilt produces
  different seasonal patterns.

## R-3.6.49 Star System Generation and Stellar Lifecycle

The engine **SHALL** generate star systems with physically-derived properties: spectral
classification (O through M) from initial mass, main-sequence lifetime from mass-luminosity
relations, and evolutionary phase based on age. Binary and trinary systems **SHALL** use
Keplerian orbital mechanics. Each star's habitable zone boundaries **SHALL** be computed from
luminosity. Generated stellar properties **SHALL** be deterministic from seed.

- **Derived from:** [F-3.6.49](../../features/geometry-world/procedural-generation.md)
- **Rationale:** Physically-grounded star generation ensures that planetary systems have
  self-consistent illumination, habitable zones, and visual appearance.
- **Verification:** Generate 100 star systems; verify spectral type distribution follows the
  initial mass function. Verify binary orbital periods match Kepler's third law within 1%.
  Verify habitable zone scales with luminosity.

## R-3.6.50 Protoplanetary Disk and Accretion Simulation

The engine **SHALL** simulate planet formation from a protoplanetary disk producing a set of
planets with physically-consistent orbital radii, masses, compositions, and orbital mechanics.
Gas giants **SHALL** form preferentially beyond the frost line. The simulation **SHALL** output
planet count, orbits, masses, compositions, rotation rates, axial tilts, and moon counts as
inputs to downstream generators.

- **Derived from:** [F-3.6.50](../../features/geometry-world/procedural-generation.md)
- **Rationale:** Accretion-based formation produces star systems that match observed exoplanet
  demographics (close rocky planets, distant gas giants).
- **Verification:** Generate 50 planetary systems; verify gas giants form predominantly beyond
  the frost line. Verify no two planets share overlapping orbital zones. Verify total system
  mass is less than initial disk mass.

## R-3.6.51 Planetary Collision and Giant Impact Simulation

The engine **SHALL** simulate planetary collisions using a coarse SPH model that determines
mass distribution, angular momentum transfer, and debris trajectories. Giant impacts **SHALL**
produce moons from debris disks when angular momentum is sufficient. Collision outcomes
**SHALL** modify planet mass, axial tilt, rotation rate, and surface state. The simulation
**SHALL** complete within 60 seconds per collision event on reference hardware.

- **Derived from:** [F-3.6.51](../../features/geometry-world/procedural-generation.md)
- **Rationale:** Giant impacts explain moon formation, axial tilts, and planetary composition
  differences — making generated systems feel scientifically grounded.
- **Verification:** Simulate a Mars-mass impactor hitting an Earth-mass target at 45 degrees;
  verify a debris disk forms and coalesces into a moon-mass body. Verify the target's axial
  tilt changes. Measure simulation time.

## R-3.6.52 Gas Giant and Non-Terrestrial Planet Generation

The engine **SHALL** generate at least 7 distinct non-terrestrial planet types: gas giant, ice
giant, airless rocky, frozen (ice shell), volcanic, hothouse, and tidally locked. Each type
**SHALL** have a dedicated terrain/atmosphere generator producing visually distinct surfaces.
Gas giants **SHALL** produce banded atmospheric textures with storm systems and ring particle
distributions. Planet type **SHALL** be automatically selected from mass, orbital position, and
composition, or manually overridden.

- **Derived from:** [F-3.6.52](../../features/geometry-world/procedural-generation.md)
- **Rationale:** Non-Earth-like planets enable diverse sci-fi settings and scientifically
  plausible star system exploration.
- **Verification:** Generate a star system with varied planet masses and positions; verify at
  least 4 distinct planet types are automatically assigned. Verify gas giant atmosphere
  rendering produces banded structures with visible storm features.

## R-3.6.53 Moon and Ring System Generation

The engine **SHALL** generate moon systems through capture, co-formation, and giant impact
pathways. Each moon **SHALL** be a full planet entity with its own terrain generation. Tidal
interactions **SHALL** drive heating, locking, and orbital resonances. Ring systems **SHALL**
render with gap structures from shepherd moon gravitational influence and cast shadows onto
the parent planet.

- **Derived from:** [F-3.6.53](../../features/geometry-world/procedural-generation.md)
- **Rationale:** Moon and ring systems add visual spectacle and gameplay variety (moon bases,
  ring mining, tidal effects on planetary surfaces).
- **Verification:** Generate a gas giant; verify it has multiple moons with varied terrain
  types. Verify ring shadow is visible on the planet surface. Verify tidally-locked moons
  always face the parent planet.

## R-3.6.54 Automatic Planet Type Classification

The engine **SHALL** automatically classify planets based on orbital position, mass, stellar
flux, magnetic field, age, and tidal forces — assigning surface type, atmosphere type,
hydrosphere type, geological activity, and habitability class. Classification **SHALL** be
deterministic from physical parameters. Artists **SHALL** be able to override any classified
parameter. The classifier **SHALL** be exposed as a PCG graph node.

- **Derived from:** [F-3.6.54](../../features/geometry-world/procedural-generation.md)
- **Rationale:** Automatic classification removes the burden of manually configuring each planet
  while maintaining physical plausibility across an entire star system.
- **Verification:** Generate a system with 8 planets at varied orbital distances; verify inner
  planets are classified as rocky and outer planets as gas/ice giants. Override one
  classification; verify the override takes effect without affecting other planets.

## R-3.6.55 Galaxy Structure Generation

The engine **SHALL** generate galaxies with at least 5 types (spiral, elliptical, irregular, lenticular,
dwarf). Star density and metallicity **SHALL** vary with galactic radius. Galaxies **SHALL** be sectorized
for LOD streaming.

- **Derived from:** [F-3.6.55](../../features/geometry-world/procedural-generation.md)
- **Rationale:** Physically-motivated galaxies enable believable space exploration.
- **Verification:** Generate each type; verify spiral arms follow log-spiral patterns; verify inner stars
  are older.

## R-3.6.56 Supermassive Black Hole

The engine **SHALL** place a SMBH at each galaxy center with mass correlated to bulge mass. **SHALL** render
gravitational lensing, accretion disk, and jets. At least 3 accretion states **SHALL** be supported.

- **Derived from:** [F-3.6.56](../../features/geometry-world/procedural-generation.md)
- **Rationale:** SMBHs are visually spectacular and scientifically central to galactic structure.
- **Verification:** Verify lensing distortion visible; verify 3 accretion states produce distinct visuals.

## R-3.6.57 Dark Matter and Large-Scale Structure

The engine **SHALL** model dark matter halos with NFW/Einasto profiles producing flat rotation curves. Galaxy
placement **SHALL** correlate with dark matter peaks. Dark matter **SHALL NOT** render directly.

- **Derived from:** [F-3.6.57](../../features/geometry-world/procedural-generation.md)
- **Rationale:** Dark matter determines large-scale cosmic structure.
- **Verification:** Verify flat rotation curves; verify galaxies cluster along filaments.

## R-3.6.58 Stellar Collisions

The engine **SHALL** simulate at least 5 collision types. Outcomes **SHALL** modify the star catalog.
Collision probability **SHALL** scale with density.

- **Derived from:** [F-3.6.58](../../features/geometry-world/procedural-generation.md)
- **Rationale:** Collisions produce exotic objects adding scientific depth.
- **Verification:** Verify higher collision rate in clusters than galactic disk.

## R-3.6.59 Black Hole Formation and Collision

The engine **SHALL** generate stellar black holes from massive star deaths. Mergers **SHALL** compute remnant
mass/spin. SMBH growth **SHALL** occur through accretion and mergers.

- **Derived from:** [F-3.6.59](../../features/geometry-world/procedural-generation.md)
- **Rationale:** Black hole events shape galactic evolution.
- **Verification:** Verify BH forms from massive star; verify merger remnant mass conserves energy.

## R-3.6.60 Universe Generation Pipeline

The engine **SHALL** provide a 7-phase pipeline (perturbations -> dark matter -> galaxies -> stars -> planets
-> surfaces -> civilizations). **SHALL** be configurable from single planet to full universe. **SHALL** be
deterministic from master seed.

- **Derived from:** [F-3.6.60](../../features/geometry-world/procedural-generation.md)
- **Rationale:** Unified pipeline ensures physical consistency across all scales.
- **Verification:** Zoom from cosmic to surface; verify consistent LOD. Regenerate from seed; verify
  identical.

## R-3.6.61 Planetary Mineralogy and Resource Distribution

The engine **SHALL** compute per-planet elemental abundances from protoplanetary disk
composition, stellar metallicity, orbital position relative to the frost line, and collision
history. Mineral concentrations **SHALL** vary spatially within each planet based on geological
processes: subduction zones SHALL concentrate copper/gold/tin, volcanic regions SHALL
concentrate sulfur and ignite minerals, hydrothermal vents SHALL concentrate heavy metals,
and sedimentary basins SHALL contain fossil fuels and evaporites. The mineral density map
**SHALL** feed into the gameplay resource node system for geologically-informed resource
placement. Each planet **SHALL** have a unique mineral fingerprint enabling inter-planetary
trade differentiation. Mineral distributions **SHALL** be deterministic from the planet's
formation seed.

- **Derived from:** [F-3.6.61](../../features/geometry-world/procedural-generation.md)
- **Rationale:** Physically-grounded mineral distribution creates meaningful exploration and
  trade incentives — players mine specific planets for specific resources rather than finding
  everything everywhere.
- **Verification:** Generate two planets at different orbital radii around the same star; verify
  the inner planet has higher iron/nickel concentration and the outer planet has higher volatile
  concentration. Generate a planet with tectonic subduction zones; verify ore deposits
  concentrate along plate boundaries. Generate two planets from the same seed; verify identical
  mineral maps.

## R-3.6.62 Server-Side Universe Generation

The engine **SHALL** generate universe data on a server cluster with shared database. New servers **SHALL**
request data not regenerate. Sharding **SHALL** align with natural boundaries. On-demand generation **SHALL**
use deterministic seeds.

- **Derived from:** [F-3.6.62](../../features/geometry-world/procedural-generation.md)
- **Rationale:** Server-side generation ensures consistency across all players.
- **Verification:** Generate sector on server A; request from B; verify identical.

## R-3.6.63 Sparse Cosmic Data Storage

The engine **SHALL** use sparse octrees with 128-bit fixed-point keys. Unvisited planets **SHALL** store
under 1KB. A galaxy with 100 billion statistical stars **SHALL** fit under 100GB.

- **Derived from:** [F-3.6.63](../../features/geometry-world/procedural-generation.md)
- **Rationale:** Sparse storage makes universe-scale data feasible.
- **Verification:** Verify void regions under 1KB; verify unvisited planets under 1KB; verify galaxy under
  100GB.

## R-3.6.64 On-Demand Hierarchical Detail Resolution

The engine **SHALL** resolve 6+ LOD tiers on demand. Leaving a region **SHALL** release data. Total memory
**SHALL** stay within budget. Prefetching **SHALL** pre-generate the next tier.

- **Derived from:** [F-3.6.64](../../features/geometry-world/procedural-generation.md)
- **Rationale:** Hierarchical resolution enables seamless cosmic-to-surface exploration.
- **Verification:** Explore 100 systems; verify memory stays in budget; verify LOD loads within 5s.
