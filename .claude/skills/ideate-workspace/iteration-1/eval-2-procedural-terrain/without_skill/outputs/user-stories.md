# User Stories — Procedural Terrain with Biome Support

## Biome Painting and Authoring

| ID        | Persona          | Features | Requirements |
|-----------|------------------|----------|--------------|
| US-3.6.65a | level designer   | F-3.6.65 | R-3.6.65     |
| US-3.6.65b | level designer   | F-3.6.65 | R-3.6.65     |
| US-3.6.65c | level designer   | F-3.6.65 | R-3.6.65     |
| US-3.6.66a | level designer   | F-3.6.66 | R-3.6.66     |
| US-3.6.66b | environment artist | F-3.6.66 | R-3.6.66   |
| US-3.6.66c | level designer   | F-3.6.66 | R-3.6.66     |
| US-3.6.67a | level designer   | F-3.6.67 | R-3.6.67     |
| US-3.6.67b | environment artist | F-3.6.67 | R-3.6.67   |

1. **US-3.6.65a** — As a level designer, I want to paint biome regions onto the terrain using a
   brush tool so that I can define where each biome begins and ends
   - **Acceptance:** I can select a biome type and paint it onto the world map with immediate visual
     feedback

2. **US-3.6.65b** — As a level designer, I want painted biome masks to override procedurally
   generated biome assignments so that I can correct or customize climate-derived biome placement
   - **Acceptance:** My painted desert overrides a climate-classified forest and all downstream
     systems update

3. **US-3.6.65c** — As a level designer, I want noise-edge and Voronoi scatter brush modes so that I
   can create organic biome boundaries without manual edge painting
   - **Acceptance:** Biome boundaries have natural, irregular shapes rather than sharp brush strokes

4. **US-3.6.66a** — As a level designer, I want to create biome definition assets that bundle
   terrain, vegetation, weather, and atmosphere settings so that I can apply a complete biome with a
   single assignment
   - **Acceptance:** Assigning a biome definition to a region configures all generation systems at
     once

5. **US-3.6.66b** — As an environment artist, I want to preview a biome definition on a sample
   terrain patch in the editor so that I can tune parameters before applying to the full world
   - **Acceptance:** The inspector shows a live preview of terrain, vegetation, and atmosphere for
     the selected biome

6. **US-3.6.66c** — As a level designer, I want biome definitions to be reusable assets so that I
   can share them across multiple worlds and projects
   - **Acceptance:** I can import a biome definition from another project and apply it to new
     terrain

7. **US-3.6.67a** — As a level designer, I want to set transition widths per biome pair so that
   forest-to-desert transitions are wider than forest-to-meadow transitions
   - **Acceptance:** Each biome-pair has an independent transition width and the blend looks natural

8. **US-3.6.67b** — As an environment artist, I want ecotone species that appear only in biome
   transition zones so that boundary regions have unique vegetation character
   - **Acceptance:** Transition zones between forest and desert contain scrubby transitional plants
     found in neither biome alone

## Per-Biome Terrain Generation

| ID        | Persona          | Features | Requirements |
|-----------|------------------|----------|--------------|
| US-3.6.68a | level designer   | F-3.6.68 | R-3.6.68     |
| US-3.6.68b | level designer   | F-3.6.68 | R-3.6.68     |
| US-3.6.69a | environment artist | F-3.6.69 | R-3.6.69   |
| US-3.6.69b | environment artist | F-3.6.69 | R-3.6.69   |
| US-3.6.70a | level designer   | F-3.6.70 | R-3.6.70     |
| US-3.6.70b | technical artist | F-3.6.70 | R-3.6.70     |

1. **US-3.6.68a** — As a level designer, I want each biome to generate distinct terrain shapes so
   that desert regions have dunes and mountain regions have jagged peaks
   - **Acceptance:** Painting a desert biome produces low-frequency dune terrain; painting a
     mountain biome produces high-frequency jagged terrain

2. **US-3.6.68b** — As a level designer, I want terrain heightmap generation to update interactively
   as I paint biome regions so that I can see the results immediately
   - **Acceptance:** Terrain reshapes within 1 second of painting a new biome region in the editor

3. **US-3.6.69a** — As an environment artist, I want terrain materials to be assigned automatically
   based on biome, slope, and altitude so that I do not have to manually splatmap-paint the entire
   world
   - **Acceptance:** Desert biomes show sand on flats and rock on slopes without manual material
     painting

4. **US-3.6.69b** — As an environment artist, I want each biome to have its own material palette of
   up to 8 materials so that different biomes use visually distinct ground textures
   - **Acceptance:** Forest biomes use grass, dirt, and moss; desert biomes use sand, gravel, and
     cracked earth

5. **US-3.6.70a** — As a level designer, I want per-biome erosion profiles so that desert terrain
   shows wind erosion and alpine terrain shows frost weathering
   - **Acceptance:** Identical base terrain erodes differently under desert vs. alpine biome
     profiles

6. **US-3.6.70b** — As a technical artist, I want to configure erosion type and intensity per biome
   definition so that I can control the level of terrain detail per biome
   - **Acceptance:** I can set hydraulic erosion iterations for tropical biomes and thermal erosion
     for arctic biomes

## Per-Biome Vegetation

| ID        | Persona          | Features | Requirements |
|-----------|------------------|----------|--------------|
| US-3.6.71a | level designer   | F-3.6.71 | R-3.6.71     |
| US-3.6.71b | level designer   | F-3.6.71 | R-3.6.71     |
| US-3.6.71c | environment artist | F-3.6.71 | R-3.6.71   |
| US-3.6.72a | environment artist | F-3.6.72 | R-3.6.72   |
| US-3.6.72b | level designer   | F-3.6.72 | R-3.6.72     |
| US-3.6.72c | environment artist | F-3.6.72 | R-3.6.72   |

1. **US-3.6.71a** — As a level designer, I want each biome to populate with its own vegetation
   species at biome-specific densities so that forests are dense and deserts are sparse
   - **Acceptance:** Forest biomes fill with trees and undergrowth; desert biomes show scattered
     cacti and scrub

2. **US-3.6.71b** — As a level designer, I want to override vegetation density per painted biome
   region so that I can make a specific forest clearing sparser without changing the global forest
   biome definition
   - **Acceptance:** Per-region density overrides apply only to the painted region, not to all
     forest biome regions

3. **US-3.6.71c** — As an environment artist, I want vegetation species to respect slope and
   altitude constraints per biome so that plants appear only in plausible locations
   - **Acceptance:** Alpine flowers do not appear on vertical cliff faces; cacti do not appear above
     the snow line

4. **US-3.6.72a** — As an environment artist, I want multi-layer tree distribution with dominant,
   secondary, and understory species so that forests have ecological depth
   - **Acceptance:** Rainforest biomes show tall canopy trees above shorter secondary species above
     ground-level understory

5. **US-3.6.72b** — As a level designer, I want tree species to change with elevation within a biome
   so that mountain forests transition from deciduous at low altitude to conifer at high altitude
   - **Acceptance:** Tree species mix shifts visibly as I move uphill within a single biome region

6. **US-3.6.72c** — As an environment artist, I want dead and fallen trees scattered at configurable
   frequency so that forests look natural rather than artificially uniform
   - **Acceptance:** A configurable percentage of trees appear as fallen logs or standing dead
     trunks

## Per-Biome Weather

| ID        | Persona          | Features | Requirements |
|-----------|------------------|----------|--------------|
| US-3.6.73a | player           | F-3.6.73 | R-3.6.73     |
| US-3.6.73b | level designer   | F-3.6.73 | R-3.6.73     |
| US-3.6.73c | player           | F-3.6.73 | R-3.6.73     |
| US-3.6.73d | level designer   | F-3.6.73 | R-3.6.73     |
| US-3.6.74a | player           | F-3.6.74 | R-3.6.74     |
| US-3.6.74b | environment artist | F-3.6.74 | R-3.6.74   |
| US-3.6.74c | creative director | F-3.6.74 | R-3.6.74    |

1. **US-3.6.73a** — As a player, I want weather to change as I travel between biomes so that each
   region feels environmentally distinct
   - **Acceptance:** Walking from a desert into a rainforest gradually introduces rain and cloud
     cover

2. **US-3.6.73b** — As a level designer, I want to define weather probability tables per biome so
   that tropical biomes have frequent rain and desert biomes have rare sandstorms
   - **Acceptance:** Each biome's weather patterns match the configured probability weights and
     duration ranges

3. **US-3.6.73c** — As a player, I want to see different weather in distant biome regions so that
   the world feels alive with independent weather systems
   - **Acceptance:** I can see rain falling in a distant forest while standing in a clear desert

4. **US-3.6.73d** — As a level designer, I want weather to transition smoothly when crossing biome
   boundaries so that there is no abrupt weather pop
   - **Acceptance:** Weather effects fade in and out over the biome transition zone, not at a hard
     boundary

5. **US-3.6.74a** — As a player, I want each biome to have its own atmospheric mood so that swamps
   feel murky and deserts feel harsh and bright
   - **Acceptance:** Fog color, density, lighting, and sky tint differ per biome and match the
     biome's character

6. **US-3.6.74b** — As an environment artist, I want to configure atmospheric profiles per biome
   including fog, sky tint, and color grading so that each biome has a unique visual identity
   - **Acceptance:** I can set green-tinted fog for swamps and warm amber haze for deserts
     independently

7. **US-3.6.74c** — As a creative director, I want atmospheric parameters to interpolate smoothly
   between biomes so that transitions feel cinematic rather than jarring
   - **Acceptance:** Moving between biomes produces a gradual mood shift with no hard color or
     lighting discontinuities
