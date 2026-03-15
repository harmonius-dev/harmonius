# 13.27 — Block-Based Voxel Games

## Block Registry

### F-13.27.1 Block Type Registry and Properties

A data-driven registry of block types where each block defines: unique ID, display name, visual mesh
or texture atlas UV, collision mode (solid, passable, liquid, climbable), hardness (mining time),
tool requirement (pickaxe for stone, axe for wood), drop table (what items drop when broken), light
emission level, light attenuation, transparency, flammability, blast resistance, and custom
properties via the gameplay database (F-13.7.1). Block types are authored as data assets in the
visual editor. The registry supports hundreds of block types with O(1) lookup by ID. Blocks can
reference logic graph behaviors (F-15.8.4) for custom interactions (furnaces smelt items, doors
toggle open/close, note blocks play sounds).

- **Requirements:** R-13.27.1
- **Dependencies:** F-13.7.1 (Table Schema), F-15.8.4 (Logic Graphs)
- **Platform notes:** None

### F-13.27.2 Block Placement and Destruction

Players place and break blocks in the world using an interaction system integrated with the input
actions (F-6.2.1). Placement: raycast from the player's view to identify the target block face,
validate placement rules (not inside the player, not in restricted zones, required block below for
gravity-affected blocks), and instantiate the block in the chunk data. Destruction: hold the
interact action to mine a block over time proportional to hardness and tool tier, with a mining
progress visual (cracking overlay). Destroyed blocks drop items (F-13.9.1) at the block position.
Placement and destruction trigger chunk re-meshing (incremental), lighting updates, and physics
updates. In multiplayer, block modifications are server-authoritative (F-8.3.1).

- **Requirements:** R-13.27.2
- **Dependencies:** F-6.2.1 (Input Actions), F-13.9.1 (Inventory), F-8.3.1 (RPCs)
- **Platform notes:** None

## Chunk System

### F-13.27.3 Chunk-Based Block Storage

The block world is divided into fixed-size chunks (16x16x16 blocks default, configurable). Each
chunk stores block IDs in a compact array with palette compression: chunks with fewer than 16 unique
block types use 4-bit indices into a per-chunk palette, reducing memory from 4096 bytes to 2048
bytes per chunk. Fully uniform chunks (all air, all stone) are stored as a single value. Chunks are
the unit of streaming, meshing, lighting, and network replication. The chunk manager tracks loaded,
loading, and queued chunks in a configurable render distance around each player.

- **Requirements:** R-13.27.3
- **Dependencies:** F-1.7.6 (Memory Budgets)
- **Platform notes:** Render distance scales with platform: mobile 8 chunks, desktop 16-32 chunks.

### F-13.27.4 Block Chunk Meshing

Convert block chunk data into renderable meshes using greedy meshing (merge adjacent coplanar faces
of the same block type into larger quads to reduce polygon count). Only exposed faces are meshed —
faces between two opaque blocks are culled. Transparent blocks (glass, water, leaves) are meshed
separately for correct draw ordering. Ambient occlusion is computed per-vertex from neighboring
block occupancy. Chunk meshes feed into the meshlet pipeline (F-3.1.1). Meshing is incremental:
modifying one block re-meshes only the affected chunk and its neighbors (for face culling at
boundaries). Meshing runs on worker threads.

- **Requirements:** R-13.27.4
- **Dependencies:** F-13.27.3, F-3.1.1 (Meshlet Pipeline)
- **Platform notes:** None

## Block Lighting

### F-13.27.5 Block Light Propagation

Flood-fill lighting system for block worlds. Two light channels: sunlight (propagates downward from
sky with full intensity, attenuates horizontally) and block light (emitted by light-emitting blocks
like torches, glowstone, lava, propagating in all directions with per-step attenuation). Light
values are stored per-block as two 4-bit values (0-15 per channel). Light propagation uses BFS flood
fill that updates incrementally when blocks are placed or removed — only affected blocks are
recalculated, not the entire chunk. Smooth lighting interpolates light values across block faces for
soft ambient occlusion gradients. Light data affects mob spawning rules (monsters spawn in darkness)
via gameplay logic.

- **Requirements:** R-13.27.5
- **Dependencies:** F-13.27.3
- **Platform notes:** None

## Block Physics

### F-13.27.6a Gravity Block Physics

Gravity-affected blocks (sand, gravel) fall when unsupported, checking the block below each tick and
converting to a falling entity that moves down until it lands on a solid surface. Landing triggers
chunk re-meshing and lighting updates. Block physics updates run in a dedicated ECS system at a
configurable tick rate (default: 20 ticks/second). The physics is deterministic for multiplayer
consistency.

- **Requirements:** R-13.27.6a
- **Dependencies:** F-13.27.3, F-4.1.1 (Rigid Body for falling entities)
- **Platform notes:** None

### F-13.27.6b Fluid Flow Simulation

Fluid simulation for water and lava: fluids flow outward and downward from source blocks with
decreasing flow level (7 levels), creating flowing streams that fill depressions and drain through
openings. Fluids apply current force to entities standing in them. Fluid propagation runs at the
block physics tick rate and is deterministic for multiplayer consistency.

- **Requirements:** R-13.27.6b
- **Dependencies:** F-13.27.3
- **Platform notes:** Mobile reduces fluid simulation tick rate (10 vs 20 ticks/second) and limits
  simultaneous fluid source propagation to control CPU cost.

### F-13.27.6c Fluid-Block Interactions

Lava ignites flammable blocks within range. Water extinguishes fire and lava, producing cobblestone
when water flows over lava or obsidian when a lava source is covered. Interaction rules are
data-driven and authored per fluid-block pair in gameplay databases. Interaction checks run during
the fluid propagation tick.

- **Requirements:** R-13.27.6c
- **Dependencies:** F-13.27.6b, F-13.27.1 (Block Properties)
- **Platform notes:** None

### F-13.27.7a Signal Source and Wire Blocks

Signal sources (buttons, levers, pressure plates, daylight sensors) emit power when activated. Wire
blocks (redstone dust equivalent) transmit power with distance attenuation (15 blocks max). Signal
propagation updates incrementally when a source changes state. Wire routing supports branching and
merging paths.

- **Requirements:** R-13.27.7a
- **Dependencies:** F-13.27.3, F-13.27.2 (Block Interaction)
- **Platform notes:** None

### F-13.27.7b Logic Gate Blocks

Logic gate blocks manipulate signals in the circuit network. Repeaters delay and boost signal
strength. Comparators measure container contents and output proportional signal levels. NOT gates
(torches on powered blocks) invert signals. AND/OR logic is achieved via wiring patterns and gate
combinations.

- **Requirements:** R-13.27.7b
- **Dependencies:** F-13.27.7a
- **Platform notes:** None

### F-13.27.7c Mechanism Blocks

Powered blocks activate mechanisms: pistons push/pull adjacent blocks, doors open/close, dispensers
fire projectiles, hoppers transfer items between containers, lights toggle on/off, and note blocks
play configurable sounds. Each mechanism type defines its activation behavior and power consumption
in the block registry.

- **Requirements:** R-13.27.7c
- **Dependencies:** F-13.27.7a, F-13.27.1 (Block Registry)
- **Platform notes:** None

### F-13.27.7d Circuit Evaluation and Budget

Circuit evaluation is deterministic with a defined update order to ensure identical behavior across
clients. Circuit complexity is budgeted per chunk — the number of active circuit components and
signal propagation steps per tick is capped. Exceeding the budget raises a warning and depowers
excess components.

- **Requirements:** R-13.27.7d
- **Dependencies:** F-13.27.7a, F-13.27.7b, F-13.27.7c
- **Platform notes:** Mobile enforces tighter per-chunk circuit complexity budgets to prevent CPU
  spikes from large redstone contraptions.

## Block World Generation

### F-13.27.8a Block Terrain Generation

Generates chunk-sized block data using 2D heightmap noise for surface elevation and 3D noise for
caves and overhangs. The generator runs on worker threads prioritized by distance from players.
World seed is configurable and deterministic — the same seed always produces the same terrain across
all platforms.

- **Requirements:** R-13.27.8a
- **Dependencies:** F-3.6.1 (PCG Graph), F-3.6.5 (Deterministic Seeding), F-3.6.33 (Noise Library),
  F-13.27.3 (Chunk Storage)
- **Platform notes:** None

### F-13.27.8b Block Biome System

Biome noise selects biome type per chunk (plains, desert, forest, ocean, mountains, tundra). Each
biome defines block composition rules (grass/dirt/stone for plains, sand/sandstone for desert).
Biome boundaries blend smoothly over a configurable transition width. Biome definitions are data
assets authored in the visual editor.

- **Requirements:** R-13.27.8b
- **Dependencies:** F-13.27.8a, F-3.6.33 (Noise Library)
- **Platform notes:** None

### F-13.27.8c Block Ore Placement

Ore veins are placed using 3D noise with per-ore frequency and depth constraints. Each ore type
defines cluster size, spawn depth range, and spawn density in gameplay databases. Ore placement runs
as a post-processing pass after terrain generation within the same worker thread.

- **Requirements:** R-13.27.8c
- **Dependencies:** F-13.27.8a, F-3.6.33 (Noise Library), F-13.7.1 (Table Schema)
- **Platform notes:** None

### F-13.27.8d Block Structure Generation

Structures (trees, villages, temples, dungeons) are placed using the rule-based placement system
(F-3.6.4) with configurable frequency and biome constraints. Structure templates define block
layouts, loot tables, and spawner placements. Structure generation runs after terrain and biome
generation to ensure correct ground placement.

- **Requirements:** R-13.27.8d
- **Dependencies:** F-13.27.8b, F-3.6.4 (Rule-Based Placement)
- **Platform notes:** None
