# User Stories -- Block-Based Voxel Games (13.27)

## Block Registry

| ID            | Persona                 |
|---------------|-------------------------|
| US-13.27.1.1  | game designer (P-5)     |
| US-13.27.1.2  | game designer (P-5)     |
| US-13.27.1.3  | game developer (P-15)   |
| US-13.27.1.4  | player (P-23)           |
| US-13.27.1.5  | player (P-23)           |
| US-13.27.1.6  | modder (P-24)           |
| US-13.27.2.1  | game designer (P-5)     |
| US-13.27.2.2  | game developer (P-15)   |
| US-13.27.2.3  | player (P-23)           |
| US-13.27.2.4  | player (P-23)           |
| US-13.27.2.5  | level designer (P-6)    |

1. **US-13.27.1.1** -- **As a** game designer (P-5), **I want** to define block types with ID,
   texture, collision mode, hardness, tool requirement, and drop table as data assets, **so that**
   block creation requires no code.

2. **US-13.27.1.2** -- **As a** game designer (P-5), **I want** to wire logic graph behaviors to
   block types for custom interactions, **so that** interactive blocks have complex functionality.

3. **US-13.27.1.3** -- **As a** game developer (P-15), **I want** the block registry to support O(1)
   lookup by ID across hundreds of types, **so that** large voxel worlds perform well at scale.

4. **US-13.27.1.4** -- [game-specific] **As a** player (P-23), **I want** each block type to have a
   distinct visual texture and properties, **so that** I can identify blocks at a glance.

5. **US-13.27.1.5** -- [game-specific] **As a** player (P-23), **I want** block tools like furnaces
   and note blocks to perform their actions when interacted with, **so that** blocks feel
   functional.

6. **US-13.27.1.6** -- **As a** modder (P-24), **I want** to register new block types with custom
   textures, properties, and logic via the mod SDK, **so that** community-created blocks integrate
   seamlessly.

7. **US-13.27.2.1** -- **As a** game designer (P-5), **I want** placement rules to validate
   collision, slope, and adjacency before confirming a block, **so that** placement constraints are
   data-driven.

8. **US-13.27.2.2** -- **As a** game developer (P-15), **I want** block modifications in multiplayer
   to be server-authoritative, **so that** cheating through client-side edits is prevented.

9. **US-13.27.2.3** -- [game-specific] **As a** player (P-23), **I want** to raycast from my view to
   identify the target block face and place a new block there, **so that** placement is precise and
   intuitive.

10. **US-13.27.2.4** -- [game-specific] **As a** player (P-23), **I want** to hold the interact
    action to mine a block with a cracking overlay showing progress, **so that** mining feedback is
    clear.

11. **US-13.27.2.5** -- **As a** level designer (P-6), **I want** to define placement restriction
    zones, **so that** players cannot build in quest-critical or performance-sensitive areas.

## Chunk System

| ID            | Persona                 |
|---------------|-------------------------|
| US-13.27.3.1  | game designer (P-5)     |
| US-13.27.3.2  | game developer (P-15)   |
| US-13.27.3.3  | player (P-23)           |
| US-13.27.4.1  | game designer (P-5)     |
| US-13.27.4.2  | game developer (P-15)   |
| US-13.27.4.3  | player (P-23)           |

1. **US-13.27.3.1** -- **As a** game designer (P-5), **I want** render distance configurable per
   platform, **so that** performance scales appropriately across mobile and desktop.

2. **US-13.27.3.2** -- **As a** game developer (P-15), **I want** chunks to use palette compression
   for diverse blocks and single-value storage for uniform chunks, **so that** memory usage is
   minimized.

3. **US-13.27.3.3** -- [game-specific] **As a** player (P-23), **I want** the block world to stream
   chunks smoothly as I move, **so that** I explore freely without stuttering.

4. **US-13.27.4.1** -- **As a** game designer (P-5), **I want** greedy meshing to merge coplanar
   faces of the same type into larger quads, **so that** polygon count is minimized.

5. **US-13.27.4.2** -- **As a** game developer (P-15), **I want** meshing to be incremental so
   modifying one block re-meshes only the affected chunk and neighbors, **so that** updates stay
   fast.

6. **US-13.27.4.3** -- [game-specific] **As a** player (P-23), **I want** transparent blocks like
   glass and water to render with correct draw ordering, **so that** no flickering or sorting
   artifacts occur.

## Block Lighting

| ID            | Persona                 |
|---------------|-------------------------|
| US-13.27.5.1  | game designer (P-5)     |
| US-13.27.5.2  | game developer (P-15)   |
| US-13.27.5.3  | player (P-23)           |
| US-13.27.5.4  | player (P-23)           |

1. **US-13.27.5.1** -- **As a** game designer (P-5), **I want** per-block light emission levels for
   two channels (sun and block), **so that** lighting is data-driven and tunable.

2. **US-13.27.5.2** -- **As a** game developer (P-15), **I want** light propagation to use
   incremental BFS that recalculates only affected blocks, **so that** placing lights does not stall
   the frame.

3. **US-13.27.5.3** -- [game-specific] **As a** player (P-23), **I want** torches to cast warm light
   with smooth gradients fading over distance, **so that** caves feel atmospheric.

4. **US-13.27.5.4** -- [game-specific] **As a** player (P-23), **I want** sunlight to propagate
   downward when I open a ceiling, **so that** natural light enters underground spaces
   realistically.

## Block Physics and Circuits

| ID            | Persona                 |
|---------------|-------------------------|
| US-13.27.6.1  | game designer (P-5)     |
| US-13.27.6.2  | game developer (P-15)   |
| US-13.27.6.3  | player (P-23)           |
| US-13.27.6.4  | player (P-23)           |
| US-13.27.7.1  | game designer (P-5)     |
| US-13.27.7.2  | game developer (P-15)   |
| US-13.27.7.3  | player (P-23)           |
| US-13.27.7.4  | player (P-23)           |
| US-13.27.7.5  | modder (P-24)           |

1. **US-13.27.6.1** -- **As a** game designer (P-5), **I want** fluid-block interaction rules
   authored per pair in gameplay databases, **so that** I can customize reactions without code.

2. **US-13.27.6.2** -- **As a** game developer (P-15), **I want** gravity and fluid physics to be
   deterministic at a configurable tick rate, **so that** multiplayer clients always agree on block
   state.

3. **US-13.27.6.3** -- [game-specific] **As a** player (P-23), **I want** sand and gravel to fall
   when I break the block beneath them, **so that** gravity creates dynamic cave-ins.

4. **US-13.27.6.4** -- [game-specific] **As a** player (P-23), **I want** water to flow outward and
   downward from source blocks with decreasing levels, **so that** I can build fountains and canals.

5. **US-13.27.7.1** -- **As a** game designer (P-5), **I want** signal propagation to update
   incrementally and circuit complexity to be budgeted per chunk, **so that** circuit updates are
   efficient.

6. **US-13.27.7.2** -- **As a** game developer (P-15), **I want** circuit evaluation to be
   deterministic with a defined update order, **so that** complex contraptions produce identical
   results across clients.

7. **US-13.27.7.3** -- [game-specific] **As a** player (P-23), **I want** to connect levers,
   buttons, and pressure plates to doors and mechanisms using wire blocks, **so that** I can build
   contraptions.

8. **US-13.27.7.4** -- [game-specific] **As a** player (P-23), **I want** pistons to push adjacent
   blocks and hoppers to transfer items between containers, **so that** I can build automated
   machines.

9. **US-13.27.7.5** -- **As a** modder (P-24), **I want** to create custom signal sources and
   mechanism blocks via the mod SDK, **so that** community-created circuit components extend the
   system.

## Block World Generation

| ID            | Persona                 |
|---------------|-------------------------|
| US-13.27.8.1  | game designer (P-5)     |
| US-13.27.8.2  | game designer (P-5)     |
| US-13.27.8.3  | game developer (P-15)   |
| US-13.27.8.4  | player (P-23)           |
| US-13.27.8.5  | player (P-23)           |
| US-13.27.8.6  | modder (P-24)           |

1. **US-13.27.8.1** -- **As a** game designer (P-5), **I want** biome definitions as data assets
   with per-biome block composition rules and smooth boundary blending, **so that** biome authoring
   is visual.

2. **US-13.27.8.2** -- **As a** game designer (P-5), **I want** structure templates with block
   layouts, loot tables, and spawner placements, **so that** generated structures are content-rich.

3. **US-13.27.8.3** -- **As a** game developer (P-15), **I want** world generation to run on worker
   threads prioritized by player distance with a configurable deterministic seed, **so that**
   generation is fast and reproducible.

4. **US-13.27.8.4** -- [game-specific] **As a** player (P-23), **I want** to enter a world seed and
   generate a world identical across all platforms, **so that** I can share seeds with friends.

5. **US-13.27.8.5** -- [game-specific] **As a** player (P-23), **I want** generated worlds to
   contain trees, villages, temples, and dungeons, **so that** exploration reveals interesting
   landmarks.

6. **US-13.27.8.6** -- **As a** modder (P-24), **I want** to create custom biome types and structure
   templates for the world generator, **so that** mods can add entirely new world themes.
