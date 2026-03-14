# 3.6 — Procedural Generation

## PCG Graph System

### F-3.6.1 Node-Based Procedural Content Graph

A visual node graph for authoring procedural generation rules. Nodes produce, filter,
transform, and consume point sets, meshes, and metadata. Subgraphs encapsulate reusable
rule sets. Graphs execute in the editor for interactive preview and at runtime for on-demand
generation. Data flows between nodes as typed streams (points, meshes, splines, heightmaps).

- **Requirements:** R-3.6.1
- **Dependencies:** F-1.1.1 (ECS)
- **Platform notes:** None

### F-3.6.2 Point Generation Nodes

Generate point distributions via surface sampling (terrain, mesh surface), volume sampling
(bounded box, sphere), grid (uniform, hex, Poisson disk), spline following (at intervals or
uniform spacing), and noise-driven density maps. Each point carries position, rotation, scale,
and arbitrary attribute key-value pairs for downstream filtering and spawning.

- **Requirements:** R-3.6.2
- **Dependencies:** F-3.6.1
- **Platform notes:** None

### F-3.6.3 Point Filtering and Transformation

Filter points by attribute predicates (height range, slope threshold, distance to spline,
biome ID, custom tags), density culling (probabilistic removal), spatial exclusion (minimum
distance between points), and bounding shapes. Transform nodes modify position (jitter, snap
to surface), rotation (align to normal, random yaw), and scale (random range, distance-based).
All filtering is non-destructive — disabled filters restore removed points.

- **Requirements:** R-3.6.3
- **Dependencies:** F-3.6.2
- **Platform notes:** None

### F-3.6.4 Mesh and Instance Spawning from Points

Convert filtered point sets into ECS entities with mesh, material, and transform components.
Spawning modes: static mesh instances (GPU instanced), ECS entity per instance (for gameplay
interaction), or hierarchical instanced static mesh (HISM) batches for rendering performance.
Points select mesh variants from weighted asset lists based on attributes.

- **Requirements:** R-3.6.4
- **Dependencies:** F-3.6.3, F-1.1.1 (ECS)
- **Platform notes:** None

### F-3.6.5 Deterministic Seeding

All procedural operations accept a seed value. A global world seed combined with spatial
coordinates (chunk position) produces per-region seeds via hash (xxHash). Identical seeds
produce identical output regardless of generation order, platform, or thread scheduling.
Seeds are exposed as user-facing parameters for world sharing and reproducible testing.

- **Requirements:** R-3.6.5
- **Dependencies:** F-3.6.1
- **Platform notes:** None

### F-3.6.6 Point Attributes and Metadata

Every generated point carries a typed attribute map: built-in properties (position, rotation,
scale, density, color, bounds, seed, index) plus arbitrary user-defined attributes (biome ID,
species tag, road type). Attributes propagate through the graph — filter and transform nodes
read and write attributes. Attribute operations include create, copy, reduce (min/max/average),
partition (group by attribute value), and noise (procedural attribute modification). Attributes
drive downstream decisions like mesh selection, material assignment, and spawn parameters.

- **Requirements:** R-3.6.6
- **Dependencies:** F-3.6.2
- **Platform notes:** None

### F-3.6.7 Point Set Operations

Boolean operations on point collections: union (combine), intersection (keep overlapping),
difference (remove overlapping), and merge (concatenate). Set operations use point bounds for
spatial overlap tests. Used for subtracting building footprints from vegetation, intersecting
road buffers with terrain features, and combining multi-source point clouds into unified
distributions.

- **Requirements:** R-3.6.7
- **Dependencies:** F-3.6.2
- **Platform notes:** None

### F-3.6.8 Graph Control Flow (Loops, Branches, Subgraphs)

Subgraphs encapsulate reusable node networks with typed input/output pins. Loop nodes iterate
over point collections element-by-element or by attribute-partitioned groups, with feedback
pins carrying data from the previous iteration. Branch nodes select execution paths based on
attribute conditions. Select nodes route data based on runtime parameters. Control flow enables
recursive generation patterns (L-systems, fractal subdivision).

- **Requirements:** R-3.6.8
- **Dependencies:** F-3.6.1
- **Platform notes:** None

## Terrain Stamps

### F-3.6.9 Non-Destructive Terrain Stamp System

Layer-based terrain modification using stamp operations applied in order. Each stamp modifies
height, texture, or vegetation within a bounded region with configurable blend modes (add,
subtract, blend, flatten, smooth). Stamps compose non-destructively — reordering or removing
a stamp rebuilds the terrain from the remaining stack. Stamp prefabs encapsulate reusable
terrain recipes.

- **Requirements:** R-3.6.9
- **Dependencies:** F-3.2.1 (Heightfield Terrain)
- **Platform notes:** None

### F-3.6.10 Terrain Texture Stamps

Procedural texturing driven by terrain analysis: height range, slope angle, curvature
(convex vs concave), compass orientation, and noise perturbation. Each texture stamp targets
a terrain material layer and applies within falloff-controlled boundaries. Multiple texture
stamps compose with priority ordering. Used for automatic snow-on-peaks, sand-in-valleys,
and rock-on-cliffs patterns.

- **Requirements:** R-3.6.10
- **Dependencies:** F-3.6.9, F-3.2.4 (Terrain Materials)
- **Platform notes:** None

### F-3.6.11 Biome Distribution System

Define biomes as bundles of terrain stamps (height, texture, vegetation, objects) with
climate parameters (temperature, precipitation, elevation range). A Whittaker-diagram lookup
maps per-region temperature and moisture values to biome IDs. Temperature derives from
latitude and elevation; moisture derives from wind simulation over the heightmap. Biome
boundaries blend with noise-perturbed transition zones.

- **Requirements:** R-3.6.11
- **Dependencies:** F-3.6.9, F-3.6.10
- **Platform notes:** None

## Vegetation Placement

### F-3.6.12 Rule-Based Vegetation Placement

Scatter vegetation instances using per-species rules: density, slope range, altitude band,
preferred biome, inter-species dependencies (underbrush near trees), exclusion zones, and
random rotation/scale ranges. Rules are evaluated on the GPU via compute shader — scatter
points generated per terrain tile and streamed relative to camera. Supports hundreds of
thousands of instances with GPU instanced rendering.

- **Requirements:** R-3.6.12
- **Dependencies:** F-3.6.3, F-3.3.1 (Instanced Foliage), F-3.3.2 (Procedural Placement)
- **Platform notes:** None

### F-3.6.13 Vegetation Clearing Along Splines

Automatically remove or suppress vegetation within a configurable distance of road, river,
and path splines. Clearing width varies per spline segment and tapers at endpoints. Vegetation
density ramps from zero at the spline to full at the clearing boundary. Integrated with the
stamp system so clearing is non-destructive and updates live as splines are edited.

- **Requirements:** R-3.6.13
- **Dependencies:** F-3.6.12, F-3.6.16
- **Platform notes:** None

## Procedural Roads and Paths

### F-3.6.14 Spline-Based Road Generation

Create roads, paths, trails, sidewalks, and highways as spline entities. Road geometry is
extruded along the spline with configurable width, banking, lane count, curb height, and
surface material. Terrain deforms to match road elevation with configurable shoulder blend.
Decorations (guard rails, signs, lamp posts, lane markings) are placed procedurally along
the road using attachment rules.

- **Requirements:** R-3.6.14
- **Dependencies:** F-3.6.1, F-3.2.1 (Terrain)
- **Platform notes:** None

### F-3.6.15 Road Network Generation

Generate connected road networks from population density maps, terrain analysis, and user-
placed control points. L-system agents trace primary roads connecting population centers
following valleys and avoiding steep slopes. Secondary roads fill enclosed regions with
configurable patterns (grid, radial, organic). Post-processing merges intersections, prunes
dead ends, and smooths curves with Catmull-Rom fitting.

- **Requirements:** R-3.6.15
- **Dependencies:** F-3.6.14
- **Platform notes:** None

### F-3.6.16 Spline SDF Optimization

Convert all splines in a terrain tile to a single signed distance field (SDF) texture. Any
system that queries distance-to-nearest-spline (vegetation clearing, texture blending, road
influence) samples one texture instead of evaluating hundreds of spline polynomials. SDFs are
cached and re-rendered only when a spline changes. Supports hundreds of road splines per tile.

- **Requirements:** R-3.6.16
- **Dependencies:** F-3.6.14
- **Platform notes:** None

### F-3.6.17 Road Intersections and Junctions

Detect and resolve road intersections where splines cross or merge. Generate appropriate
junction geometry: T-junctions, four-way crossroads, roundabouts, and highway on/off ramps.
Traffic signage and road markings are placed automatically based on junction type and road
hierarchy. Terrain blending at intersections smooths height discontinuities.

- **Requirements:** R-3.6.17
- **Dependencies:** F-3.6.14, F-3.6.15
- **Platform notes:** None

## Building Generation

### F-3.6.18 Shape Grammar Building Generator

Generate building facades using hierarchical split grammars. A building mass is split into
floors, floors into tiles, tiles into facade elements (windows, doors, balconies, cornices).
Split operations divide shapes along any axis by absolute or relative amounts. Parameterized
rules control building height, floor count, window density, style, and material palette.
Grammars are defined as data assets and can be mixed per-district.

- **Requirements:** R-3.6.15
- **Dependencies:** F-3.6.1
- **Platform notes:** None

### F-3.6.19 Modular Building Assembly

Assemble buildings from modular asset packs with standardized connection points (sockets).
Walls, corners, floors, roofs, doors, and windows connect via matching socket IDs on shared
faces. Composition rules enforce valid connections and structural constraints. Supports multi-
story buildings, L-shaped and U-shaped floor plans, and interior room generation via recursive
subdivision.

- **Requirements:** R-3.6.19
- **Dependencies:** F-3.6.18
- **Platform notes:** None

## Wave Function Collapse

### F-3.6.20 2D Tile-Based WFC

Generate 2D tile layouts (dungeons, floor plans, puzzle levels, city blocks) using Wave
Function Collapse. Each cell in a 2D grid starts in superposition. The algorithm observes
the minimum-entropy cell, collapses it to a weighted-random tile, and propagates adjacency
constraints to neighbors. Supports rotation variants, frequency-weighted selection, and
backtracking on contradiction.

- **Requirements:** R-3.6.20
- **Dependencies:** F-3.6.5 (Seeding)
- **Platform notes:** None

### F-3.6.21 3D Voxel WFC

Extend WFC to three dimensions using cubic tiles with six-face socket matching. Generates
buildings, cave systems, space stations, and multi-story interiors from modular tile sets.
Pre-computed adjacency lookup tables and bitwise propagation optimize constraint solving.
Supports chunked generation for large worlds — adjacent chunks share boundary constraints
for seamless transitions.

- **Requirements:** R-3.6.21
- **Dependencies:** F-3.6.20
- **Platform notes:** None

### F-3.6.22 WFC Constraint Painting

Allow designers to pre-constrain specific cells before running WFC — pin certain tiles in
place (doors, stairs, boss rooms) and let the algorithm fill the rest. Paint zones with
tile subset restrictions (e.g., "this area must be corridors only"). Constraints are
validated before generation to detect unsatisfiable configurations early.

- **Requirements:** R-3.6.22
- **Dependencies:** F-3.6.20, F-3.6.21
- **Platform notes:** None

## General-Purpose Modular Assembly

### F-3.6.23 Socket-Based Modular Assembly Engine

A general-purpose system for assembling complex objects from modular pieces via socket
connections. Any asset can define named sockets (attach points with position, rotation, type
ID, and compatibility rules). The assembly engine validates connections, resolves transforms,
and merges attached pieces into a single entity hierarchy. Used for assembling spaceships
(hull + cockpit + engines + weapons), naval vessels (hull + deck + mast + rigging), weapons
(barrel + stock + scope + magazine), armor sets, hand railings, and any modular object.

- **Requirements:** R-3.6.23
- **Dependencies:** F-3.6.1, F-1.1.14 (Relationships)
- **Platform notes:** None

### F-3.6.24 Procedural Object Generation Rules

Define assembly rules as PCG graph nodes that procedurally select and connect modular pieces.
Rules specify part categories per socket (e.g., "engine socket accepts any thruster-type
part"), variation weights, size constraints, and style tags. A generation pass walks the
socket graph from a root piece outward, selecting parts that satisfy constraints. Produces
unique ships, vehicles, buildings, weapons, and props from a shared part library.

- **Requirements:** R-3.6.24
- **Dependencies:** F-3.6.23, F-3.6.5 (Seeding)
- **Platform notes:** None

### F-3.6.25 Houdini Engine Procedural Object Pipeline

Connect Houdini Digital Assets to the modular assembly engine. Artists author procedural
assembly rules in Houdini (scatter engines on a hull, route pipes between components, generate
railing along a spline, place rivets on seams) and expose parameters. The engine evaluates
the HDA via Houdini Engine (F-12.6.3), receives the assembled geometry and instance data,
and converts it to ECS entities. Supports cook-on-parameter-change for interactive design.

- **Requirements:** R-3.6.25
- **Dependencies:** F-3.6.23, F-12.6.3 (Houdini Engine)
- **Platform notes:** Requires Houdini Engine license at edit time.

### F-3.6.26 Hierarchical Modular Composition

Compose assembled modular objects into larger modular assets recursively. A room is assembled
from wall/floor/ceiling pieces; rooms compose into a building floor; floors compose into a
building; buildings compose into a city block. Each composition level defines its own socket
interface so it can be used as a module at the next level up. Composition hierarchies are
stored as asset prefabs — an artist-authored tavern becomes a reusable module in a procedural
village generator, which itself becomes a module in a world-region generator.

- **Requirements:** R-3.6.26
- **Dependencies:** F-3.6.23, F-1.1.37 (Nested Prefabs)
- **Platform notes:** None

## Authoring Workflow

### F-3.6.27 Interactive PCG Authoring Tools

Editor tools for artists to drive PCG interactively: spline drawing (roads, rivers, fences),
point painting (scatter props by brushing), volume dragging (define generation regions), and
socket wiring (connect modular pieces by clicking sockets). Each tool is backed by a PCG graph
with parameters exposed in the inspector. Artists see results update in real-time as they
paint, drag, or adjust parameters — no code or graph editing required.

- **Requirements:** R-3.6.27
- **Dependencies:** F-3.6.1, F-15.1.1 (Editor Framework)
- **Platform notes:** None

### F-3.6.28 Artist-Guided Constraint Authoring

Artists define high-level constraints (building footprints, road waypoints, landmark
placements, zone boundaries) and the PCG system fills in the rest procedurally. Constraints
are ECS entities with spatial components placed in the editor; the PCG graph reads them as
fixed inputs and generates content around them. Artists iterate by moving constraints and
seeing the procedural output adapt instantly. Combines hand-crafted hero content with
procedural population.

- **Requirements:** R-3.6.28
- **Dependencies:** F-3.6.27, F-3.6.1
- **Platform notes:** None

## AI-Driven Generation

### F-3.6.29 AI-Driven Content Generation

An AI agent interface that drives PCG graphs programmatically. AI systems (trained models or
rule engines) set PCG graph parameters, place constraints, select style presets, and evaluate
output quality metrics. Supports iterative generation: the AI generates a candidate, evaluates
it against objective functions (navigability, visual variety, gameplay flow), and re-generates
with adjusted parameters until quality thresholds are met.

- **Requirements:** R-3.6.29
- **Dependencies:** F-3.6.1, F-3.6.5 (Seeding)
- **Platform notes:** None

### F-3.6.30 Constraint Satisfaction Solver

A general-purpose constraint solver for PCG that goes beyond WFC. Define spatial constraints
(minimum distance between buildings, roads must connect to at least two intersections, rivers
flow downhill, no floating structures) and the solver finds valid placements satisfying all
constraints simultaneously. Uses backtracking search with arc consistency pruning. Applicable
to city layout, dungeon connectivity, puzzle level design, and furniture placement.

- **Requirements:** R-3.6.30
- **Dependencies:** F-3.6.1
- **Platform notes:** None

## Runtime Generation

### F-3.6.31 Runtime Chunk-Based Procedural Generation

Generate world content at runtime as the player moves through the world. The world is divided
into fixed-size chunks; chunks generate on background threads (F-14.3.3) when they enter a
configurable activation radius ahead of the camera. Deterministic seeding (F-3.6.5) ensures
revisited chunks reproduce identically. A priority queue orders generation by distance and
importance. Memory budgets evict distant chunks.

- **Requirements:** R-3.6.31
- **Dependencies:** F-3.6.5, F-14.3.3 (Job System), F-3.2.3 (Terrain Streaming)
- **Platform notes:** None

### F-3.6.32 GPU Compute Procedural Generation

Generate heightmaps, normal maps, vegetation scatter, and noise fields on the GPU via compute
shaders. Fractal Brownian motion, Perlin, simplex, Worley, and domain-warped noise functions
run entirely on the GPU. Indirect dispatch drives vegetation instance buffer population.
GPU generation produces results in a single frame for seamless streaming without CPU stalls.

- **Requirements:** R-3.6.32
- **Dependencies:** F-3.6.31, F-2.1.1 (GPU Abstraction)
- **Platform notes:** None

### F-3.6.33 Noise Function Library

A comprehensive library of noise functions usable in PCG graphs, compute shaders, and CPU
code: value noise, Perlin, simplex, Worley/cellular, fractal Brownian motion (configurable
octaves, lacunarity, gain), domain warping, ridged multifractal, and billowed noise. All
functions are deterministic and produce identical results on CPU and GPU for a given seed.

- **Requirements:** R-3.6.33
- **Dependencies:** None
- **Platform notes:** None
