# User Stories -- 3.6 Procedural Generation

## PCG Graph System

| ID         | Persona                  |
|------------|--------------------------|
| US-3.6.1.1 | level designer (P-6)     |
| US-3.6.1.2 | technical artist (P-13)  |
| US-3.6.1.3 | environment artist (P-8) |
| US-3.6.2.1 | level designer (P-6)     |
| US-3.6.3.1 | environment artist (P-8) |
| US-3.6.4.1 | level designer (P-6)     |
| US-3.6.5.1 | engine developer (P-26)  |
| US-3.6.6.1 | technical artist (P-13)  |
| US-3.6.7.1 | level designer (P-6)     |
| US-3.6.8.1 | technical artist (P-13)  |

1. **US-3.6.1.1** -- **As a** level designer (P-6), **I want** a visual node graph for authoring
   procedural rules, **so that** I create generation logic without code.
2. **US-3.6.1.2** -- **As a** technical artist (P-13), **I want** graphs to execute in editor for
   interactive preview and at runtime for on-demand generation, **so that** I iterate quickly on
   procedural content.
3. **US-3.6.1.3** -- **As a** environment artist (P-8), **I want** data to flow between nodes as
   typed streams, **so that** I connect points, meshes, and splines safely.
4. **US-3.6.2.1** -- **As a** level designer (P-6), **I want** point generation via surface
   sampling, grid, Poisson disk, and noise density, **so that** I create varied distribution
   patterns.
5. **US-3.6.3.1** -- **As a** environment artist (P-8), **I want** to filter points by height,
   slope, distance, and biome with non-destructive toggling, **so that** I refine placement rules
   iteratively.
6. **US-3.6.4.1** -- **As a** level designer (P-6), **I want** filtered points converted to mesh
   instances or ECS entities, **so that** generated content appears in the world.
7. **US-3.6.5.1** -- **As a** engine developer (P-26), **I want** deterministic seeding from a
   global seed plus spatial coordinates, **so that** identical seeds produce identical output across
   platforms.
8. **US-3.6.6.1** -- **As a** technical artist (P-13), **I want** typed attributes on every
   generated point, **so that** downstream nodes use per-point metadata for mesh selection and
   material assignment.
9. **US-3.6.7.1** -- **As a** level designer (P-6), **I want** boolean set operations on point
   collections, **so that** I subtract building footprints from foliage.
10. **US-3.6.8.1** -- **As a** technical artist (P-13), **I want** subgraphs, loops, and branches in
    graph control flow, **so that** I build recursive patterns like L-systems.

## Terrain Stamps and Biomes

| ID          | Persona                  |
|-------------|--------------------------|
| US-3.6.9.1  | level designer (P-6)     |
| US-3.6.10.1 | environment artist (P-8) |
| US-3.6.11.1 | level designer (P-6)     |

1. **US-3.6.9.1** -- **As a** level designer (P-6), **I want** non-destructive terrain stamps with
   reorderable blend modes, **so that** I compose terrain modifications as a layer stack.
2. **US-3.6.10.1** -- **As a** environment artist (P-8), **I want** procedural texturing from
   terrain analysis (height, slope, curvature), **so that** snow, sand, and rock appear
   automatically.
3. **US-3.6.11.1** -- **As a** level designer (P-6), **I want** biome definitions as bundles of
   stamps driven by temperature and moisture, **so that** biome placement follows climate logic.

## Vegetation and Roads

| ID          | Persona                  |
|-------------|--------------------------|
| US-3.6.12.1 | environment artist (P-8) |
| US-3.6.13.1 | level designer (P-6)     |
| US-3.6.14.1 | level designer (P-6)     |
| US-3.6.15.1 | level designer (P-6)     |
| US-3.6.16.1 | engine developer (P-26)  |
| US-3.6.17.1 | level designer (P-6)     |

1. **US-3.6.12.1** -- **As a** environment artist (P-8), **I want** rule-based vegetation scatter
   per species with GPU compute evaluation, **so that** hundreds of thousands of instances populate
   terrain tiles.
2. **US-3.6.13.1** -- **As a** level designer (P-6), **I want** vegetation automatically cleared
   along road and river splines, **so that** paths stay free without manual editing.
3. **US-3.6.14.1** -- **As a** level designer (P-6), **I want** spline-based road generation with
   terrain deformation and procedural decorations, **so that** roads integrate with the landscape.
4. **US-3.6.15.1** -- **As a** level designer (P-6), **I want** connected road networks generated
   from density maps and terrain analysis, **so that** settlements link naturally.
5. **US-3.6.16.1** -- **As a** engine developer (P-26), **I want** splines converted to a per-tile
   SDF texture, **so that** distance queries sample one texture instead of evaluating spline
   polynomials.
6. **US-3.6.17.1** -- **As a** level designer (P-6), **I want** road intersections auto-detected and
   resolved with junction geometry, **so that** crossroads and roundabouts generate correctly.

## Building Generation and WFC

| ID          | Persona                  |
|-------------|--------------------------|
| US-3.6.18.1 | environment artist (P-8) |
| US-3.6.19.1 | level designer (P-6)     |
| US-3.6.20.1 | level designer (P-6)     |
| US-3.6.21.1 | level designer (P-6)     |
| US-3.6.22.1 | level designer (P-6)     |

1. **US-3.6.18.1** -- **As a** environment artist (P-8), **I want** buildings generated via
   hierarchical split grammars with parameterized style, **so that** varied facades emerge from
   data-driven rules.
2. **US-3.6.19.1** -- **As a** level designer (P-6), **I want** buildings assembled from modular
   asset packs with socket-based connections, **so that** walls, roofs, and interiors compose
   validly.
3. **US-3.6.20.1** -- **As a** level designer (P-6), **I want** 2D WFC tile generation for dungeons
   and floor plans, **so that** layouts satisfy adjacency constraints automatically.
4. **US-3.6.21.1** -- **As a** level designer (P-6), **I want** 3D voxel WFC for multi-story
   interiors and cave systems, **so that** complex 3D structures generate from tile sets.
5. **US-3.6.22.1** -- **As a** level designer (P-6), **I want** to pin specific tiles before running
   WFC, **so that** key rooms are placed exactly where I choose.

## Modular Assembly and Authoring

| ID          | Persona                  |
|-------------|--------------------------|
| US-3.6.23.1 | level designer (P-6)     |
| US-3.6.24.1 | environment artist (P-8) |
| US-3.6.25.1 | technical artist (P-13)  |
| US-3.6.26.1 | level designer (P-6)     |
| US-3.6.27.1 | environment artist (P-8) |
| US-3.6.28.1 | level designer (P-6)     |

1. **US-3.6.23.1** -- **As a** level designer (P-6), **I want** a socket-based assembly engine for
   ships, weapons, and modular objects, **so that** complex assets compose from reusable pieces.
2. **US-3.6.24.1** -- **As a** environment artist (P-8), **I want** procedural assembly rules as PCG
   graph nodes, **so that** unique variants generate from a shared part library.
3. **US-3.6.25.1** -- **As a** technical artist (P-13), **I want** Houdini Digital Assets connected
   to the assembly engine, **so that** HDA-authored rules drive procedural geometry.
4. **US-3.6.26.1** -- **As a** level designer (P-6), **I want** hierarchical composition where
   assembled objects become modules at the next level, **so that** rooms compose into buildings and
   buildings into blocks.
5. **US-3.6.27.1** -- **As a** environment artist (P-8), **I want** interactive PCG tools for spline
   drawing, point painting, and socket wiring, **so that** I drive generation visually in the
   editor.
6. **US-3.6.28.1** -- **As a** level designer (P-6), **I want** to place high-level constraints and
   have PCG fill in the rest, **so that** hand-crafted landmarks coexist with procedural population.

## AI and Runtime Generation

| ID          | Persona                  |
|-------------|--------------------------|
| US-3.6.29.1 | technical artist (P-13)  |
| US-3.6.30.1 | level designer (P-6)     |
| US-3.6.31.1 | engine developer (P-26)  |
| US-3.6.32.1 | engine developer (P-26)  |
| US-3.6.33.1 | technical artist (P-13)  |

1. **US-3.6.29.1** -- **As a** technical artist (P-13), **I want** an AI agent interface that drives
   PCG graphs programmatically, **so that** trained models generate and evaluate content
   iteratively.
2. **US-3.6.30.1** -- **As a** level designer (P-6), **I want** a constraint satisfaction solver for
   spatial placement, **so that** buildings, roads, and rivers obey spatial rules simultaneously.
3. **US-3.6.31.1** -- **As a** engine developer (P-26), **I want** runtime chunk-based generation on
   background threads with deterministic seeding, **so that** content generates ahead of the player.
4. **US-3.6.32.1** -- **As a** engine developer (P-26), **I want** GPU compute generation of
   heightmaps, noise fields, and vegetation scatter, **so that** generation completes in a single
   frame.
5. **US-3.6.33.1** -- **As a** technical artist (P-13), **I want** a comprehensive noise library
   (Perlin, simplex, Worley, fBm, domain warping), **so that** I have building blocks for all
   procedural patterns.

## Planet-Scale Generation

| ID          | Persona                  |
|-------------|--------------------------|
| US-3.6.34.1 | level designer (P-6)     |
| US-3.6.35.1 | level designer (P-6)     |
| US-3.6.36.1 | level designer (P-6)     |
| US-3.6.37.1 | level designer (P-6)     |
| US-3.6.38.1 | level designer (P-6)     |
| US-3.6.39.1 | level designer (P-6)     |
| US-3.6.40.1 | level designer (P-6)     |
| US-3.6.41.1 | level designer (P-6)     |
| US-3.6.42.1 | technical artist (P-13)  |
| US-3.6.43.1 | technical artist (P-13)  |
| US-3.6.44.1 | environment artist (P-8) |
| US-3.6.45.1 | environment artist (P-8) |
| US-3.6.46.1 | environment artist (P-8) |
| US-3.6.47.1 | level designer (P-6)     |
| US-3.6.48.1 | level designer (P-6)     |

1. **US-3.6.34.1** -- **As a** level designer (P-6), **I want** entire planet surfaces generated
   with continents, mountains, rivers, and climate zones, **so that** I get a plausible world from a
   single seed.
2. **US-3.6.35.1** -- **As a** level designer (P-6), **I want** cities and settlements generated
   with road networks and zoning, **so that** populated areas emerge from population density data.
3. **US-3.6.36.1** -- **As a** level designer (P-6), **I want** factions generated with territories
   and diplomatic relationships, **so that** the world has geopolitical context.
4. **US-3.6.37.1** -- **As a** level designer (P-6), **I want** quests generated from narrative
   templates parameterized by world state, **so that** content scales with the generated world.
5. **US-3.6.38.1** -- **As a** level designer (P-6), **I want** dynamic ecosystem simulation with
   predator-prey dynamics, **so that** wildlife populations respond to player actions.
6. **US-3.6.39.1** -- **As a** level designer (P-6), **I want** the world simulated through
   historical epochs before the player enters, **so that** the world has ruins, lore, and history.
7. **US-3.6.40.1** -- **As a** level designer (P-6), **I want** enemies and NPCs placed based on
   zone danger, biome, and faction control, **so that** encounters match the environment.
8. **US-3.6.41.1** -- **As a** level designer (P-6), **I want** loot and resources distributed based
   on zone difficulty and geological rules, **so that** economy balances naturally.
9. **US-3.6.42.1** -- **As a** technical artist (P-13), **I want** plate tectonic simulation driving
   continent shapes and mountain formation, **so that** geology is physically plausible.
10. **US-3.6.43.1** -- **As a** technical artist (P-13), **I want** climate simulation considering
    latitude, altitude, and ocean currents, **so that** biome distribution follows physical
    constraints.
11. **US-3.6.44.1** -- **As a** environment artist (P-8), **I want** terrain classified into biomes
    with gradient ecotone transitions, **so that** biome boundaries look natural.
12. **US-3.6.45.1** -- **As a** environment artist (P-8), **I want** hydrological simulation
    producing rivers, lakes, and watersheds, **so that** water bodies follow terrain drainage.
13. **US-3.6.46.1** -- **As a** environment artist (P-8), **I want** geological landforms (canyons,
    fjords, dunes) placed by formation process, **so that** terrain features match their geological
    context.
14. **US-3.6.47.1** -- **As a** level designer (P-6), **I want** real-world GIS data imported to
    generate Earth-accurate terrain, **so that** real locations can seed procedural worlds.
15. **US-3.6.48.1** -- **As a** level designer (P-6), **I want** a unified planet configuration
    asset with presets (Earth-like, desert, ocean, ice), **so that** I configure planet generation
    visually.

## Stellar and Cosmological Generation

| ID          | Persona                  |
|-------------|--------------------------|
| US-3.6.49.1 | level designer (P-6)     |
| US-3.6.50.1 | technical artist (P-13)  |
| US-3.6.51.1 | engine developer (P-26)  |
| US-3.6.52.1 | environment artist (P-8) |
| US-3.6.53.1 | environment artist (P-8) |
| US-3.6.54.1 | technical artist (P-13)  |
| US-3.6.55.1 | level designer (P-6)     |
| US-3.6.56.1 | technical artist (P-13)  |
| US-3.6.57.1 | engine developer (P-26)  |
| US-3.6.58.1 | engine developer (P-26)  |
| US-3.6.59.1 | engine developer (P-26)  |
| US-3.6.60.1 | level designer (P-6)     |
| US-3.6.61.1 | level designer (P-6)     |

1. **US-3.6.49.1** -- **As a** level designer (P-6), **I want** star systems generated with spectral
   types and habitable zones, **so that** stellar neighborhoods are astrophysically motivated.
2. **US-3.6.50.1** -- **As a** technical artist (P-13), **I want** planetary systems formed from
   protoplanetary disk accretion, **so that** planet count, orbits, and compositions follow physical
   models.
3. **US-3.6.51.1** -- **As a** engine developer (P-26), **I want** giant impact simulation
   determining planet mass, moons, and axial tilt, **so that** collision history shapes planetary
   properties.
4. **US-3.6.52.1** -- **As a** environment artist (P-8), **I want** non-terrestrial planet types
   (gas giant, ice world, volcanic hellscape) with dedicated terrain generators, **so that** diverse
   planets are visually distinct.
5. **US-3.6.53.1** -- **As a** environment artist (P-8), **I want** moon and ring systems generated
   from formation history, **so that** each planet has a unique satellite configuration.
6. **US-3.6.54.1** -- **As a** technical artist (P-13), **I want** automatic planet type
   classification from physical constraints, **so that** planet features are self-consistent without
   manual setup.
7. **US-3.6.55.1** -- **As a** level designer (P-6), **I want** galaxies generated with spiral,
   elliptical, and irregular structures, **so that** star density varies realistically across
   galactic space.
8. **US-3.6.56.1** -- **As a** technical artist (P-13), **I want** supermassive black holes with
   gravitational lensing and accretion disks, **so that** galactic cores are visually dramatic.
9. **US-3.6.57.1** -- **As a** engine developer (P-26), **I want** dark matter halos shaping
   galactic rotation and cosmic web structure, **so that** galaxy placement follows large-scale
   structure.
10. **US-3.6.58.1** -- **As a** engine developer (P-26), **I want** stellar collision simulation
    modifying the star catalog, **so that** mergers and supernovae affect the generated universe.
11. **US-3.6.59.1** -- **As a** engine developer (P-26), **I want** black hole formation and merger
    simulation, **so that** compact objects appear from massive star deaths.
12. **US-3.6.60.1** -- **As a** level designer (P-6), **I want** a top-down universe pipeline from
    Big Bang to present with configurable scope, **so that** I generate anything from a single
    planet to a full universe.
13. **US-3.6.61.1** -- **As a** level designer (P-6), **I want** per-planet mineral compositions
    derived from formation history, **so that** resource distribution is geologically motivated.

## Universe Infrastructure

| ID          | Persona                  |
|-------------|--------------------------|
| US-3.6.62.1 | engine developer (P-26)  |
| US-3.6.63.1 | engine developer (P-26)  |
| US-3.6.64.1 | engine developer (P-26)  |

1. **US-3.6.62.1** -- **As a** engine developer (P-26), **I want** universe generation on a server
   cluster with shard-aligned caching, **so that** game servers fetch pre-generated data instead of
   regenerating.
2. **US-3.6.63.1** -- **As a** engine developer (P-26), **I want** sparse hierarchical octree
   storage with 128-bit position keys, **so that** cosmic-scale data uses memory proportional to
   occupied space.
3. **US-3.6.64.1** -- **As a** engine developer (P-26), **I want** on-demand detail resolution
   across 6+ LOD tiers with prefetching, **so that** only the local region is loaded at full detail.

## Simulation Nodes

| ID           | Persona                  |
|--------------|--------------------------|
| US-3.6.65.1  | environment artist (P-8) |
| US-3.6.66.1  | level designer (P-6)     |
| US-3.6.67.1  | level designer (P-6)     |

1. **US-3.6.65.1** -- **As an** environment artist (P-8), **I want** an erosion node that applies
   thermal and hydraulic erosion to heightmaps with tunable iteration count and sediment parameters,
   **so that** mountains, canyons, and riverbeds look naturally weathered.
2. **US-3.6.66.1** -- **As a** level designer (P-6), **I want** a BSP partition node that
   recursively subdivides a region into rooms connected by corridors, **so that** I generate classic
   grid dungeons without writing generation code.
3. **US-3.6.67.1** -- **As a** level designer (P-6), **I want** a cellular automata node with
   configurable birth and survival rules, **so that** I generate organic cave layouts and islands
   from standard roguelike rule sets.

## Parent Stories

The 3-segment parent stories below are umbrella rollups for the refined 4-segment sub-stories listed
above. Each parent inherits the persona of its first sub-story and describes the umbrella capability
that the sub-stories refine.

| ID | Persona |
|----|---------|
| US-3.6.1 | level designer (P-6) |
| US-3.6.10 | environment artist (P-8) |
| US-3.6.11 | level designer (P-6) |
| US-3.6.12 | environment artist (P-8) |
| US-3.6.13 | level designer (P-6) |
| US-3.6.14 | level designer (P-6) |
| US-3.6.15 | level designer (P-6) |
| US-3.6.16 | engine developer (P-26) |
| US-3.6.17 | level designer (P-6) |
| US-3.6.18 | environment artist (P-8) |
| US-3.6.19 | level designer (P-6) |
| US-3.6.2 | level designer (P-6) |
| US-3.6.20 | level designer (P-6) |
| US-3.6.21 | level designer (P-6) |
| US-3.6.22 | level designer (P-6) |
| US-3.6.23 | level designer (P-6) |
| US-3.6.24 | environment artist (P-8) |
| US-3.6.25 | technical artist (P-13) |
| US-3.6.26 | level designer (P-6) |
| US-3.6.27 | environment artist (P-8) |
| US-3.6.28 | level designer (P-6) |
| US-3.6.29 | technical artist (P-13) |
| US-3.6.3 | environment artist (P-8) |
| US-3.6.30 | level designer (P-6) |
| US-3.6.31 | engine developer (P-26) |
| US-3.6.32 | engine developer (P-26) |
| US-3.6.33 | technical artist (P-13) |
| US-3.6.34 | level designer (P-6) |
| US-3.6.35 | level designer (P-6) |
| US-3.6.36 | level designer (P-6) |
| US-3.6.37 | level designer (P-6) |
| US-3.6.38 | level designer (P-6) |
| US-3.6.39 | level designer (P-6) |
| US-3.6.4 | level designer (P-6) |
| US-3.6.40 | level designer (P-6) |
| US-3.6.41 | level designer (P-6) |
| US-3.6.42 | technical artist (P-13) |
| US-3.6.43 | technical artist (P-13) |
| US-3.6.44 | environment artist (P-8) |
| US-3.6.45 | environment artist (P-8) |
| US-3.6.46 | environment artist (P-8) |
| US-3.6.47 | level designer (P-6) |
| US-3.6.48 | level designer (P-6) |
| US-3.6.49 | level designer (P-6) |
| US-3.6.5 | engine developer (P-26) |
| US-3.6.50 | technical artist (P-13) |
| US-3.6.51 | engine developer (P-26) |
| US-3.6.52 | environment artist (P-8) |
| US-3.6.53 | environment artist (P-8) |
| US-3.6.54 | technical artist (P-13) |
| US-3.6.55 | level designer (P-6) |
| US-3.6.56 | technical artist (P-13) |
| US-3.6.57 | engine developer (P-26) |
| US-3.6.58 | engine developer (P-26) |
| US-3.6.59 | engine developer (P-26) |
| US-3.6.6 | technical artist (P-13) |
| US-3.6.60 | level designer (P-6) |
| US-3.6.61 | level designer (P-6) |
| US-3.6.62 | engine developer (P-26) |
| US-3.6.63 | engine developer (P-26) |
| US-3.6.64 | engine developer (P-26) |
| US-3.6.65 | environment artist (P-8) |
| US-3.6.66 | level designer (P-6) |
| US-3.6.67 | level designer (P-6) |
| US-3.6.7 | level designer (P-6) |
| US-3.6.8 | technical artist (P-13) |
| US-3.6.9 | level designer (P-6) |

1. **US-3.6.1** -- **As a** level designer (P-6), **I want** the capabilities defined in sub-stories
   US-3.6.1.1 through US-3.6.1.3 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

2. **US-3.6.10** -- **As a** environment artist (P-8), **I want** the capabilities defined in
   sub-stories US-3.6.10.1 through US-3.6.10.1 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

3. **US-3.6.11** -- **As a** level designer (P-6), **I want** the capabilities defined in
   sub-stories US-3.6.11.1 through US-3.6.11.1 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

4. **US-3.6.12** -- **As a** environment artist (P-8), **I want** the capabilities defined in
   sub-stories US-3.6.12.1 through US-3.6.12.1 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

5. **US-3.6.13** -- **As a** level designer (P-6), **I want** the capabilities defined in
   sub-stories US-3.6.13.1 through US-3.6.13.1 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

6. **US-3.6.14** -- **As a** level designer (P-6), **I want** the capabilities defined in
   sub-stories US-3.6.14.1 through US-3.6.14.1 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

7. **US-3.6.15** -- **As a** level designer (P-6), **I want** the capabilities defined in
   sub-stories US-3.6.15.1 through US-3.6.15.1 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

8. **US-3.6.16** -- **As a** engine developer (P-26), **I want** the capabilities defined in
   sub-stories US-3.6.16.1 through US-3.6.16.1 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

9. **US-3.6.17** -- **As a** level designer (P-6), **I want** the capabilities defined in
   sub-stories US-3.6.17.1 through US-3.6.17.1 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

10. **US-3.6.18** -- **As a** environment artist (P-8), **I want** the capabilities defined in
    sub-stories
US-3.6.18.1 through US-3.6.18.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

11. **US-3.6.19** -- **As a** level designer (P-6), **I want** the capabilities defined in
    sub-stories
US-3.6.19.1 through US-3.6.19.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

12. **US-3.6.2** -- **As a** level designer (P-6), **I want** the capabilities defined in
    sub-stories
US-3.6.2.1 through US-3.6.2.1 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.

13. **US-3.6.20** -- **As a** level designer (P-6), **I want** the capabilities defined in
    sub-stories
US-3.6.20.1 through US-3.6.20.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

14. **US-3.6.21** -- **As a** level designer (P-6), **I want** the capabilities defined in
    sub-stories
US-3.6.21.1 through US-3.6.21.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

15. **US-3.6.22** -- **As a** level designer (P-6), **I want** the capabilities defined in
    sub-stories
US-3.6.22.1 through US-3.6.22.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

16. **US-3.6.23** -- **As a** level designer (P-6), **I want** the capabilities defined in
    sub-stories
US-3.6.23.1 through US-3.6.23.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

17. **US-3.6.24** -- **As a** environment artist (P-8), **I want** the capabilities defined in
    sub-stories
US-3.6.24.1 through US-3.6.24.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

18. **US-3.6.25** -- **As a** technical artist (P-13), **I want** the capabilities defined in
    sub-stories
US-3.6.25.1 through US-3.6.25.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

19. **US-3.6.26** -- **As a** level designer (P-6), **I want** the capabilities defined in
    sub-stories
US-3.6.26.1 through US-3.6.26.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

20. **US-3.6.27** -- **As a** environment artist (P-8), **I want** the capabilities defined in
    sub-stories
US-3.6.27.1 through US-3.6.27.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

21. **US-3.6.28** -- **As a** level designer (P-6), **I want** the capabilities defined in
    sub-stories
US-3.6.28.1 through US-3.6.28.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

22. **US-3.6.29** -- **As a** technical artist (P-13), **I want** the capabilities defined in
    sub-stories
US-3.6.29.1 through US-3.6.29.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

23. **US-3.6.3** -- **As a** environment artist (P-8), **I want** the capabilities defined in
    sub-stories
US-3.6.3.1 through US-3.6.3.1 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.

24. **US-3.6.30** -- **As a** level designer (P-6), **I want** the capabilities defined in
    sub-stories
US-3.6.30.1 through US-3.6.30.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

25. **US-3.6.31** -- **As a** engine developer (P-26), **I want** the capabilities defined in
    sub-stories
US-3.6.31.1 through US-3.6.31.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

26. **US-3.6.32** -- **As a** engine developer (P-26), **I want** the capabilities defined in
    sub-stories
US-3.6.32.1 through US-3.6.32.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

27. **US-3.6.33** -- **As a** technical artist (P-13), **I want** the capabilities defined in
    sub-stories
US-3.6.33.1 through US-3.6.33.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

28. **US-3.6.34** -- **As a** level designer (P-6), **I want** the capabilities defined in
    sub-stories
US-3.6.34.1 through US-3.6.34.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

29. **US-3.6.35** -- **As a** level designer (P-6), **I want** the capabilities defined in
    sub-stories
US-3.6.35.1 through US-3.6.35.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

30. **US-3.6.36** -- **As a** level designer (P-6), **I want** the capabilities defined in
    sub-stories
US-3.6.36.1 through US-3.6.36.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

31. **US-3.6.37** -- **As a** level designer (P-6), **I want** the capabilities defined in
    sub-stories
US-3.6.37.1 through US-3.6.37.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

32. **US-3.6.38** -- **As a** level designer (P-6), **I want** the capabilities defined in
    sub-stories
US-3.6.38.1 through US-3.6.38.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

33. **US-3.6.39** -- **As a** level designer (P-6), **I want** the capabilities defined in
    sub-stories
US-3.6.39.1 through US-3.6.39.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

34. **US-3.6.4** -- **As a** level designer (P-6), **I want** the capabilities defined in
    sub-stories
US-3.6.4.1 through US-3.6.4.1 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.

35. **US-3.6.40** -- **As a** level designer (P-6), **I want** the capabilities defined in
    sub-stories
US-3.6.40.1 through US-3.6.40.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

36. **US-3.6.41** -- **As a** level designer (P-6), **I want** the capabilities defined in
    sub-stories
US-3.6.41.1 through US-3.6.41.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

37. **US-3.6.42** -- **As a** technical artist (P-13), **I want** the capabilities defined in
    sub-stories
US-3.6.42.1 through US-3.6.42.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

38. **US-3.6.43** -- **As a** technical artist (P-13), **I want** the capabilities defined in
    sub-stories
US-3.6.43.1 through US-3.6.43.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

39. **US-3.6.44** -- **As a** environment artist (P-8), **I want** the capabilities defined in
    sub-stories
US-3.6.44.1 through US-3.6.44.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

40. **US-3.6.45** -- **As a** environment artist (P-8), **I want** the capabilities defined in
    sub-stories
US-3.6.45.1 through US-3.6.45.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

41. **US-3.6.46** -- **As a** environment artist (P-8), **I want** the capabilities defined in
    sub-stories
US-3.6.46.1 through US-3.6.46.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

42. **US-3.6.47** -- **As a** level designer (P-6), **I want** the capabilities defined in
    sub-stories
US-3.6.47.1 through US-3.6.47.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

43. **US-3.6.48** -- **As a** level designer (P-6), **I want** the capabilities defined in
    sub-stories
US-3.6.48.1 through US-3.6.48.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

44. **US-3.6.49** -- **As a** level designer (P-6), **I want** the capabilities defined in
    sub-stories
US-3.6.49.1 through US-3.6.49.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

45. **US-3.6.5** -- **As a** engine developer (P-26), **I want** the capabilities defined in
    sub-stories
US-3.6.5.1 through US-3.6.5.1 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.

46. **US-3.6.50** -- **As a** technical artist (P-13), **I want** the capabilities defined in
    sub-stories
US-3.6.50.1 through US-3.6.50.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

47. **US-3.6.51** -- **As a** engine developer (P-26), **I want** the capabilities defined in
    sub-stories
US-3.6.51.1 through US-3.6.51.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

48. **US-3.6.52** -- **As a** environment artist (P-8), **I want** the capabilities defined in
    sub-stories
US-3.6.52.1 through US-3.6.52.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

49. **US-3.6.53** -- **As a** environment artist (P-8), **I want** the capabilities defined in
    sub-stories
US-3.6.53.1 through US-3.6.53.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

50. **US-3.6.54** -- **As a** technical artist (P-13), **I want** the capabilities defined in
    sub-stories
US-3.6.54.1 through US-3.6.54.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

51. **US-3.6.55** -- **As a** level designer (P-6), **I want** the capabilities defined in
    sub-stories
US-3.6.55.1 through US-3.6.55.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

52. **US-3.6.56** -- **As a** technical artist (P-13), **I want** the capabilities defined in
    sub-stories
US-3.6.56.1 through US-3.6.56.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

53. **US-3.6.57** -- **As a** engine developer (P-26), **I want** the capabilities defined in
    sub-stories
US-3.6.57.1 through US-3.6.57.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

54. **US-3.6.58** -- **As a** engine developer (P-26), **I want** the capabilities defined in
    sub-stories
US-3.6.58.1 through US-3.6.58.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

55. **US-3.6.59** -- **As a** engine developer (P-26), **I want** the capabilities defined in
    sub-stories
US-3.6.59.1 through US-3.6.59.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

56. **US-3.6.6** -- **As a** technical artist (P-13), **I want** the capabilities defined in
    sub-stories
US-3.6.6.1 through US-3.6.6.1 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.

57. **US-3.6.60** -- **As a** level designer (P-6), **I want** the capabilities defined in
    sub-stories
US-3.6.60.1 through US-3.6.60.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

58. **US-3.6.61** -- **As a** level designer (P-6), **I want** the capabilities defined in
    sub-stories
US-3.6.61.1 through US-3.6.61.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

59. **US-3.6.62** -- **As a** engine developer (P-26), **I want** the capabilities defined in
    sub-stories
US-3.6.62.1 through US-3.6.62.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

60. **US-3.6.63** -- **As a** engine developer (P-26), **I want** the capabilities defined in
    sub-stories
US-3.6.63.1 through US-3.6.63.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

61. **US-3.6.64** -- **As a** engine developer (P-26), **I want** the capabilities defined in
    sub-stories
US-3.6.64.1 through US-3.6.64.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

62. **US-3.6.65** -- **As a** environment artist (P-8), **I want** the capabilities defined in
    sub-stories
US-3.6.65.1 through US-3.6.65.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

63. **US-3.6.66** -- **As a** level designer (P-6), **I want** the capabilities defined in
    sub-stories
US-3.6.66.1 through US-3.6.66.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

64. **US-3.6.67** -- **As a** level designer (P-6), **I want** the capabilities defined in
    sub-stories
US-3.6.67.1 through US-3.6.67.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

65. **US-3.6.7** -- **As a** level designer (P-6), **I want** the capabilities defined in
    sub-stories
US-3.6.7.1 through US-3.6.7.1 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.

66. **US-3.6.8** -- **As a** technical artist (P-13), **I want** the capabilities defined in
    sub-stories
US-3.6.8.1 through US-3.6.8.1 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.

67. **US-3.6.9** -- **As a** level designer (P-6), **I want** the capabilities defined in
    sub-stories
US-3.6.9.1 through US-3.6.9.1 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.
