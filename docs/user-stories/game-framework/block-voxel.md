# User Stories — 13.27 Block-Based Voxel Games

## Block Registry

### US-13.27.1 Register Block Types Through the Visual Editor

**As a** game designer (P-5), **I want** to define new block types in the visual editor by
specifying an ID, texture, collision mode, hardness, tool requirement, drop table, and custom
properties, **so that** I can add content to the block world without writing code or waiting for
a programmer.

### US-13.27.2 Attach Custom Logic to Interactive Blocks

**As a** game designer (P-5), **I want** to wire logic graph behaviors to block types (furnaces
smelt items, doors toggle, note blocks play sounds), **so that** I can create interactive blocks
with complex functionality using the visual scripting system.

### US-13.27.3 Add Modded Block Types to the Registry

**As a** modder (P-24), **I want** to register new block types with custom textures, properties,
and logic graph behaviors through the mod SDK, **so that** I can extend the block palette with
community-created content that integrates seamlessly with the base game.

## Block Placement and Destruction

### US-13.27.4 Place and Break Blocks With Satisfying Feedback

**As a** player (P-23), **I want** to point at a block face to place a new block and hold the
interact button to mine blocks with a cracking overlay that shows progress, **so that** building
and mining feel responsive, precise, and visually clear.

### US-13.27.5 Configure Tool Tiers and Mining Speeds

**As a** game designer (P-5), **I want** to set per-block hardness values and per-tool tier
multipliers in gameplay databases, **so that** I can balance the mining progression curve (wood
pickaxe mines slowly, diamond pickaxe mines quickly) without code changes.

### US-13.27.6 Trust That Block Changes Are Fair in Multiplayer

**As a** player (P-23), **I want** block placement and destruction to be server-authoritative,
**so that** other players cannot cheat by instantly mining or placing blocks through hacks.

### US-13.27.7 Design Block Placement Rules for Puzzle Levels

**As a** level designer (P-6), **I want** to define placement restriction zones and custom
adjacency rules per block type, **so that** I can create puzzle environments where players must
figure out valid block arrangements to progress.

## Chunk-Based Block Storage

### US-13.27.8 Explore a Large Block World Without Lag

**As a** player (P-23), **I want** the block world to stream chunks smoothly as I move, with
distant chunks loading before I reach them, **so that** I can explore freely without stuttering
or visible pop-in.

### US-13.27.9 Configure Render Distance Per Platform

**As a** game designer (P-5), **I want** to set render distance per platform (8 chunks on mobile,
16-32 on desktop) in a configuration asset, **so that** I can optimize performance for each target
device without code changes.

### US-13.27.10 Build Massive Structures Without Memory Issues

**As a** player (P-23), **I want** the chunk system to compress block data efficiently (palette
compression for diverse chunks, single-value for uniform chunks), **so that** I can build in
worlds with long render distances without running out of memory.

## Block Chunk Meshing

### US-13.27.11 See Smooth Ambient Occlusion on Block Edges

**As a** player (P-23), **I want** block edges and corners to show smooth ambient occlusion
shading based on neighboring blocks, **so that** the voxel world looks polished and three-
dimensional rather than flat and sterile.

### US-13.27.12 Place a Block and See the Mesh Update Instantly

**As a** player (P-23), **I want** chunk meshes to update within one frame when I place or break
a block, **so that** my modifications are visible immediately without waiting for background
processing.

### US-13.27.13 See Through Glass and Water Correctly

**As a** player (P-23), **I want** transparent blocks like glass and water to render with correct
draw ordering (no flickering or sorting artifacts), **so that** underwater bases and glass
structures look correct from every angle.

### US-13.27.14 Optimize Meshing for Large Flat Surfaces

**As a** level designer (P-6), **I want** greedy meshing to merge large flat surfaces of the same
block type into fewer polygons, **so that** superflat worlds and large builds run at high frame
rates with minimal triangle count.

## Block Light Propagation

### US-13.27.15 Light a Cave With Torches and See Smooth Gradients

**As a** player (P-23), **I want** to place torches that cast warm light with smooth gradients
fading over distance, **so that** caves and underground builds feel atmospheric and I can see
where I need more light sources.

### US-13.27.16 See Sunlight Flood In When Opening a Ceiling

**As a** player (P-23), **I want** sunlight to immediately propagate downward when I remove a
block from the ceiling of an underground room, **so that** natural light behaves realistically
and I can design skylights and open-roof builds.

### US-13.27.17 Use Darkness for Mob Spawning Rules

**As a** game designer (P-5), **I want** the block lighting system to expose per-block light
levels to gameplay logic so I can configure mob spawning rules (monsters spawn below light
level 7), **so that** lighting becomes a strategic gameplay element, not just a visual feature.

### US-13.27.18 Create Custom Light-Emitting Blocks

**As a** modder (P-24), **I want** to set the light emission level (0-15) on any block type I
create, **so that** I can add glowing ores, magical crystals, or decorative lanterns that
integrate with the flood-fill lighting system.

## Block Physics and Fluid Simulation

### US-13.27.19 Watch Sand Fall When I Remove Support

**As a** player (P-23), **I want** gravity-affected blocks like sand and gravel to fall smoothly
when I break the block beneath them, **so that** cave-ins and sand traps feel dynamic and I can
use gravity in creative builds.

### US-13.27.20 Create Water Features With Natural Flow

**As a** player (P-23), **I want** to place a water source and watch it flow outward and downhill
with decreasing levels, filling depressions and draining through openings, **so that** I can
build fountains, moats, canals, and irrigation channels.

### US-13.27.21 Design Lava Puzzle Hazards

**As a** level designer (P-6), **I want** lava to ignite nearby flammable blocks and water to
convert lava into cobblestone or obsidian, **so that** I can design environmental puzzle
challenges where players use fluid interactions to create safe paths.

### US-13.27.21a Define Custom Fluid Interaction Rules

**As a** game designer (P-5), **I want** to define fluid-block interaction rules (what happens
when water meets lava, when lava meets wood) as data entries per fluid-block pair, **so that**
I can customize fluid behaviors for my game without code changes.

### US-13.27.22 Tune Fluid and Gravity Physics Per Game

**As a** game designer (P-5), **I want** to configure the block physics tick rate, fluid flow
speed, gravity block fall speed, and fluid interaction rules through data assets, **so that**
I can make physics feel snappy for an action game or deliberate for a puzzle game without code
changes.

## Redstone-Style Logic Circuits

### US-13.27.23 Build a Redstone Door That Opens With a Lever

**As a** player (P-23), **I want** to connect a lever to a door using wire blocks, **so that**
I can build mechanical contraptions like hidden entrances, drawbridges, and trap doors.

### US-13.27.23a Use Pressure Plates and Daylight Sensors

**As a** player (P-23), **I want** pressure plates that activate when I step on them and
daylight sensors that activate based on time of day, **so that** I can build automated
mechanisms triggered by presence and time.

### US-13.27.24 Design a Dungeon With Block Logic Puzzles

**As a** level designer (P-6), **I want** to use pressure plates, repeaters, comparators, and
pistons to build logic puzzles in dungeons, **so that** players must solve wiring challenges to
open doors, reveal treasure, and progress through the level.

### US-13.27.25 Create Automated Farms With Hoppers and Dispensers

**As a** player (P-23), **I want** to chain hoppers to transfer items between containers and use
dispensers to automate planting and harvesting, **so that** I can build automated resource farms
that run while I explore.

### US-13.27.25a Build Piston Doors and Moving Block Structures

**As a** player (P-23), **I want** pistons to push and pull blocks when powered, **so that** I
can build moving doors, hidden passages, and mechanical structures.

### US-13.27.26 Trust That Circuits Behave Identically Every Time

**As a** player (P-23), **I want** circuit evaluation to be deterministic with a defined update
order, **so that** my complex contraptions produce the same result every time and I can design
reliable machines.

### US-13.27.26a Have Circuit Complexity Limits Prevent Server Lag

**As a** player (P-23), **I want** per-chunk circuit complexity budgets to prevent massive
circuits from causing lag, **so that** the server stays responsive for all players.

### US-13.27.27 Create Custom Circuit Components as Mod Blocks

**As a** modder (P-24), **I want** to create new signal-emitting, transmitting, or receiving
blocks through the mod SDK, **so that** I can add logic gates, sensors, or actuators that
community members can use in their circuit designs.

## Procedural Block World Generation

### US-13.27.28 Explore a Unique World From a Shared Seed

**As a** player (P-23), **I want** to enter a world seed and generate a deterministic world that
is identical across all platforms, **so that** I can share seeds with friends and explore the
same landscapes, caves, and biome layouts they discovered.

### US-13.27.28a Explore Caves and Overhangs

**As a** player (P-23), **I want** 3D noise to carve caves and overhangs into the terrain,
**so that** exploration feels three-dimensional with tunnels, caverns, and cliff formations.

### US-13.27.29 Configure Biome Rules and Ore Distribution

**As a** game designer (P-5), **I want** to define biome types (plains, desert, forest, ocean,
mountains, tundra) with per-biome block composition rules in the visual editor, **so that**
I can create diverse worlds without writing generation code.

### US-13.27.29a Configure Per-Ore Frequency and Depth

**As a** game designer (P-5), **I want** to set per-ore spawn frequency, depth range, and
cluster size in gameplay databases, **so that** I can balance the mining progression curve
without code changes.

### US-13.27.30 Place Structures in Generated Worlds

**As a** level designer (P-6), **I want** the world generator to place structures (trees,
villages, temples, dungeons) using the rule-based placement system with configurable frequency
and biome constraints, **so that** generated worlds contain interesting landmarks and exploration
targets.

### US-13.27.31 Create Custom Biomes and Structures as Mods

**As a** modder (P-24), **I want** to define new biome types with custom block compositions and
new structure templates that the world generator can place, **so that** I can create entirely new
world themes (volcanic islands, crystal caverns, floating islands) that feel native to the
generation system.

### US-13.27.32 Explore Without Waiting for World Generation

**As a** player (P-23), **I want** chunk generation to run on background threads prioritized by
my distance, **so that** the terrain ahead of me is always ready by the time I walk there and I
never stall waiting for world generation.
