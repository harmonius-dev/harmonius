# 3.6 — Procedural Terrain Generation with Biome Support

## Biome Painting and Authoring

| ID       | Feature                                  | Requirements |
|----------|------------------------------------------|--------------|
| F-3.6.65 | Biome Region Painting Tool               | R-3.6.65     |
| F-3.6.66 | Biome Definition Asset                   | R-3.6.66     |
| F-3.6.67 | Biome Transition Blending                | R-3.6.67     |

1. **F-3.6.65** — An editor brush tool that lets designers paint biome region masks directly onto
   the world map or terrain viewport. The brush writes biome IDs into a per-tile biome mask texture
   at configurable resolution (default 1 texel per meter). Multiple brush modes: fill, gradient,
   noise edge, and Voronoi scatter. Painted regions override procedurally generated biome IDs
   (F-3.6.11, F-3.6.44) in a non-destructive layer stack — painted biome masks compose above
   climate-derived masks so designers can correct or customize any region. Undo/redo support and
   real-time preview of downstream terrain, vegetation, and weather changes as the designer paints.
   - **Deps:** F-3.6.11 (Biome Distribution), F-3.2.1 (Heightfield Terrain), F-15.6.1 (Terrain
     Sculpting)
   - **Platform:** Brush resolution matches terrain tile resolution, which scales per platform tier.
     Preview update rate reduced on mobile.

2. **F-3.6.66** — A biome definition asset that bundles all generation parameters for a single biome
   into a reusable, artist-authored data object. Each biome definition contains: terrain heightmap
   modifiers (amplitude range, fractal octaves, erosion style), terrain material layer assignments
   with splatmap weight curves, vegetation species palettes with per-species density and placement
   rules, prop and rock scatter rules, ambient audio profile, weather probability table,
   fog/atmosphere tint, and lighting overrides. Biome definitions are ECS-compatible data assets
   referenced by biome ID. The editor exposes all parameters in an inspector panel with real-time
   preview on a sample terrain patch.
   - **Deps:** F-3.6.11 (Biome Distribution), F-1.1.1 (ECS)
   - **Platform:** All parameters identical across platforms. Per-platform tier scaling is applied
     at generation time, not in the definition.

3. **F-3.6.67** — Smooth blending between adjacent biome regions using configurable transition
   zones. Transition width is set per biome-pair (e.g., forest-to-desert wider than
   forest-to-meadow). Within the transition zone, terrain height modifiers interpolate, splatmap
   material weights cross-fade, vegetation densities blend with species from both biomes present,
   and weather parameters lerp. Noise perturbation offsets the transition boundary to prevent
   straight-line biome edges. Ecotone species (unique to boundary zones) can be defined per
   biome-pair, spawning only in the overlap region.
   - **Deps:** F-3.6.65 (Biome Painting), F-3.6.66 (Biome Definition)
   - **Platform:** Transition zone noise resolution reduced on mobile. Ecotone species disabled on
     lowest tier.

## Per-Biome Terrain Generation

| ID       | Feature                                      | Requirements |
|----------|----------------------------------------------|--------------|
| F-3.6.68 | Biome-Driven Terrain Heightmap Generation    | R-3.6.68     |
| F-3.6.69 | Biome-Driven Terrain Material Assignment     | R-3.6.69     |
| F-3.6.70 | Biome Erosion Profiles                       | R-3.6.70     |

1. **F-3.6.68** — Generate terrain heightmap detail driven by the active biome at each terrain cell.
   Each biome definition (F-3.6.66) specifies noise parameters (frequency, amplitude, octave count,
   lacunarity, persistence) and a heightmap modifier graph that runs per-tile during generation.
   Desert biomes produce dune fields (low-frequency ridged noise), mountain biomes produce jagged
   peaks (high-frequency ridged multifractal), tundra produces flat plains with gentle undulation.
   The generator composites biome-specific height contributions with the base terrain using the
   biome mask as weight, so painted biome regions reshape the terrain accordingly. Generation runs
   on GPU compute (F-3.6.32) for interactive preview speed.
   - **Deps:** F-3.6.65 (Biome Painting), F-3.6.66 (Biome Definition), F-3.2.1 (Heightfield
     Terrain), F-3.6.32 (GPU Compute)
   - **Platform:** Noise octave count reduced on mobile. GPU compute fallback to CPU on platforms
     without compute shader support.

2. **F-3.6.69** — Automatically assign terrain material layers based on biome ID, slope, altitude
   within the biome, and curvature. Each biome definition specifies a material palette (up to 8
   materials) and assignment curves mapping terrain analysis values to material weights. Desert
   biomes assign sand on flat areas and exposed rock on slopes. Forest biomes assign grass on gentle
   slopes, dirt on paths, and moss on north-facing surfaces. Output writes directly to the terrain
   splatmap (F-3.2.5) per tile.
   - **Deps:** F-3.6.66 (Biome Definition), F-3.2.5 (Splatmap Blending), F-3.6.10 (Texture Stamps)
   - **Platform:** Material layer count per biome inherits per-tier limits from F-3.2.5.

3. **F-3.6.70** — Per-biome erosion simulation profiles that modify terrain after initial heightmap
   generation. Each biome defines erosion type and intensity: hydraulic erosion for temperate and
   tropical biomes (river carving, sediment deposition), thermal erosion for alpine and arctic
   biomes (scree slopes, frost weathering), wind erosion for desert biomes (dune formation,
   ventifact sculpting), and coastal erosion for shoreline biomes (sea cliffs, wave-cut platforms).
   Erosion runs as a post-process pass on generated tiles before material assignment.
   - **Deps:** F-3.6.68 (Biome Height Generation), F-3.6.66 (Biome Def)
   - **Platform:** Erosion iteration count reduced on mobile. Wind erosion simulation disabled on
     lowest tier (pre-baked dune shapes used instead).

## Per-Biome Vegetation

| ID       | Feature                                  | Requirements |
|----------|------------------------------------------|--------------|
| F-3.6.71 | Biome Vegetation Palette and Placement   | R-3.6.71     |
| F-3.6.72 | Biome-Aware Tree Species Distribution    | R-3.6.72     |

1. **F-3.6.71** — Each biome definition (F-3.6.66) specifies a vegetation palette: a list of plant
   species with per-species density, scale range, slope tolerance, altitude band, and clustering
   behavior. The procedural placement system (F-3.3.2) reads the biome mask to select the active
   palette per terrain tile and scatters vegetation accordingly. Designers can override individual
   species densities per painted biome region without modifying the global biome definition.
   Placement respects exclusion zones (F-3.6.13) and inter-species rules (F-3.6.12).
   - **Deps:** F-3.6.66 (Biome Definition), F-3.3.2 (Procedural Placement), F-3.6.12 (Vegetation
     Rules)
   - **Platform:** Species variety per biome reduced on mobile (top 4 species by density). Instance
     count inherits F-3.3.1 tier caps.

2. **F-3.6.72** — Specialized tree distribution logic that assigns dominant, secondary, and
   understory tree species per biome with elevation and moisture gradients within the biome.
   Tropical rainforest biomes produce dense multi-canopy forests with tall emergent trees. Boreal
   biomes produce conifer-dominated stands with sparse spacing. Savanna biomes scatter isolated
   large trees over grassland. Tree density and species mix shift smoothly across biome transitions
   (F-3.6.67). Dead and fallen trees are scattered at configurable frequency for visual variety.
   - **Deps:** F-3.6.71 (Vegetation Palette), F-3.3.7 (Tree Rendering), F-3.6.67 (Biome Transitions)
   - **Platform:** Understory tree layer disabled on mobile. Fallen tree density reduced on mobile.

## Per-Biome Weather

| ID       | Feature                                  | Requirements |
|----------|------------------------------------------|--------------|
| F-3.6.73 | Biome Weather Zone System                | R-3.6.73     |
| F-3.6.74 | Biome Atmospheric and Lighting Profiles  | R-3.6.74     |

1. **F-3.6.73** — A weather zone system driven by biome regions. Each biome definition (F-3.6.66)
   specifies a weather probability table: the set of weather states (clear, overcast, light rain,
   heavy rain, thunderstorm, fog, snow, blizzard, sandstorm, heatwave) with per-state probability
   weights, duration ranges, and transition rules. The weather system evaluates the active biome at
   the camera position and drives the precipitation (F-11.4.1), snow (F-11.4.3), fog (F-11.4.4),
   lightning (F-11.4.5), and wind (F-11.4.6) effects accordingly. Weather transitions occur when the
   player crosses biome boundaries — leaving a desert into a temperate forest gradually introduces
   rain probability. Multiple weather zones can coexist simultaneously in different world regions.
   - **Deps:** F-3.6.66 (Biome Definition), F-11.4.1 (Rain), F-11.4.3 (Snow), F-11.4.4 (Fog),
     F-11.4.5 (Lightning), F-11.4.6 (Wind)
   - **Platform:** Simultaneous active weather zones capped per tier: mobile 1, Switch 2, desktop
     4+. Weather particle density inherits VFX tier scaling.

2. **F-3.6.74** — Per-biome atmospheric color grading, ambient lighting, fog density, fog color, sky
   tint, and time-of-day modifiers. Tropical biomes use warm, saturated light with high humidity
   haze. Arctic biomes use cool, desaturated light with crisp visibility. Swamp biomes use
   green-tinted fog with low visibility. These profiles blend smoothly as the player moves between
   biomes, interpolating all lighting and atmospheric parameters over the transition zone
   (F-3.6.67). Designers can override any parameter per biome region.
   - **Deps:** F-3.6.66 (Biome Definition), F-3.6.67 (Biome Transitions), F-3.5.1 (Procedural Sky)
   - **Platform:** Color grading LUT interpolation is universal. Fog parameter interpolation
     simplified on mobile (stepped, not smooth).
