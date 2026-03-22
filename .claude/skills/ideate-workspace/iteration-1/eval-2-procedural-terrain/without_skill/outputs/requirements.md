# R-3.6 — Procedural Terrain with Biome Support

## Biome Painting and Authoring

| ID       | Derived From |
|----------|--------------|
| R-3.6.65 | F-3.6.65     |
| R-3.6.66 | F-3.6.66     |
| R-3.6.67 | F-3.6.67     |

1. **R-3.6.65** — The engine **SHALL** provide an editor brush tool that paints biome IDs into
   per-tile biome mask textures, with fill, gradient, noise edge, and Voronoi scatter brush modes,
   composing painted masks non-destructively above climate-derived biome maps (F-3.6.11, F-3.6.44)
   with undo/redo and real-time preview of downstream terrain, vegetation, and weather changes.
   - **Rationale:** Designers need direct authorial control over biome placement to override or
     augment procedural climate simulation results for specific gameplay or aesthetic goals.
   - **Verification:** Paint a desert biome region onto climate-classified temperate terrain;
     confirm the biome mask updates, terrain regenerates with desert height modifiers, vegetation
     switches to desert species, and weather transitions to arid patterns. Undo the stroke and
     confirm full reversion.

2. **R-3.6.66** — The engine **SHALL** support biome definition assets that bundle terrain heightmap
   modifiers, material layer assignments, vegetation species palettes, prop scatter rules, weather
   probability tables, atmospheric profiles, and audio profiles into reusable ECS-compatible data
   objects referenced by biome ID.
   - **Rationale:** A single biome definition asset ensures consistent generation across terrain,
     vegetation, weather, and atmosphere without requiring designers to configure each system
     separately.
   - **Verification:** Create a biome definition with custom terrain noise, 4 material layers, 6
     vegetation species, and a weather table; assign it to a region; confirm all downstream systems
     generate content matching the definition parameters.

3. **R-3.6.67** — The engine **SHALL** blend adjacent biome regions using configurable
   per-biome-pair transition widths, interpolating terrain height modifiers, splatmap weights,
   vegetation densities, weather parameters, and atmospheric profiles across the transition zone,
   with noise-perturbed boundaries and optional ecotone species.
   - **Rationale:** Abrupt biome boundaries break visual immersion; gradient transitions with noise
     perturbation and unique ecotone species create believable environmental shifts.
   - **Verification:** Place a forest biome adjacent to a desert biome with a 50m transition width;
     confirm terrain height blends, both biomes contribute vegetation in the overlap, materials
     cross-fade, weather parameters interpolate, and the boundary is noise-perturbed (not a straight
     line).

## Per-Biome Terrain Generation

| ID       | Derived From |
|----------|--------------|
| R-3.6.68 | F-3.6.68     |
| R-3.6.69 | F-3.6.69     |
| R-3.6.70 | F-3.6.70     |

1. **R-3.6.68** — The engine **SHALL** generate terrain heightmap detail per tile using the biome
   definition's noise parameters and heightmap modifier graph, compositing biome-specific height
   contributions with the base terrain weighted by the biome mask, running on GPU compute for
   interactive preview speed.
   - **Rationale:** Biome-specific terrain shapes (dunes, jagged peaks, gentle plains) require
     per-biome noise parameters applied through the biome mask to maintain visual coherence within
     each region.
   - **Verification:** Paint three biomes (desert, mountain, tundra) on adjacent regions; confirm
     each region's terrain exhibits the expected landform character (dunes, peaks, plains) and
     transitions blend smoothly at boundaries.

2. **R-3.6.69** — The engine **SHALL** assign terrain material layers per tile based on biome ID,
   slope, altitude, and curvature, using per-biome material palettes of up to 8 materials with
   assignment curves, writing output to the terrain splatmap (F-3.2.5).
   - **Rationale:** Automatic material assignment driven by biome and terrain analysis eliminates
     manual splatmap painting across large worlds while maintaining per-biome visual identity.
   - **Verification:** Generate a terrain tile in a desert biome and confirm sand on flat areas,
     rock on slopes; regenerate the same tile as forest biome and confirm grass on flats, dirt on
     slopes; verify splatmap weights sum to 1.0 at every texel.

3. **R-3.6.70** — The engine **SHALL** apply per-biome erosion simulation as a post-process pass on
   generated tiles, supporting hydraulic, thermal, wind, and coastal erosion types with per-biome
   intensity parameters.
   - **Rationale:** Different biomes exhibit distinct erosion patterns; desert wind erosion, alpine
     thermal erosion, and tropical hydraulic erosion create visually distinct terrain character.
   - **Verification:** Generate identical base terrain with desert vs. alpine biome erosion
     profiles; confirm desert terrain shows dune ridges and wind-sculpted features while alpine
     terrain shows scree slopes and frost-weathered surfaces.

## Per-Biome Vegetation

| ID       | Derived From |
|----------|--------------|
| R-3.6.71 | F-3.6.71     |
| R-3.6.72 | F-3.6.72     |

1. **R-3.6.71** — The engine **SHALL** select vegetation species and densities per terrain tile from
   the active biome's vegetation palette, supporting per-species density, scale range, slope
   tolerance, altitude band, and clustering behavior, with designer-override capability per painted
   region.
   - **Rationale:** Each biome needs a distinct vegetation profile; per-region overrides let
     designers customize specific areas without changing the global biome definition.
   - **Verification:** Paint a forest biome and confirm dense tree and undergrowth placement; paint
     adjacent desert biome and confirm sparse cacti and scrub. Override desert tree density to 2x in
     one region and confirm the override applies only to that region.

2. **R-3.6.72** — The engine **SHALL** distribute tree species per biome with dominant, secondary,
   and understory layers, varying species mix by elevation and moisture within the biome, including
   dead and fallen trees at configurable frequency.
   - **Rationale:** Multi-layer tree distribution with elevation-dependent species variation creates
     ecologically believable forests that change character across altitude bands.
   - **Verification:** Generate a tropical rainforest biome from sea level to 2000m; confirm canopy
     trees dominate at low elevation, species mix shifts with altitude, understory is present under
     canopy, and fallen trees appear at the configured rate.

## Per-Biome Weather

| ID       | Derived From |
|----------|--------------|
| R-3.6.73 | F-3.6.73     |
| R-3.6.74 | F-3.6.74     |

1. **R-3.6.73** — The engine **SHALL** drive weather state selection from per-biome weather
   probability tables, supporting at least 10 weather states (clear, overcast, light rain, heavy
   rain, thunderstorm, fog, snow, blizzard, sandstorm, heatwave) with per-state probability weights
   and duration ranges, transitioning weather when the player crosses biome boundaries, and
   supporting simultaneous weather zones in different world regions.
   - **Rationale:** Biome-driven weather creates environmental variety as players traverse the
     world; simultaneous zones ensure distant regions maintain independent weather for visual
     coherence.
   - **Verification:** Place the camera in a tropical biome and confirm rain events occur at the
     configured probability; move to an adjacent desert biome and confirm weather transitions to
     arid states; observe the tropical region still raining in the distance.

2. **R-3.6.74** — The engine **SHALL** apply per-biome atmospheric color grading, ambient lighting,
   fog density, fog color, sky tint, and time-of-day modifiers, interpolating all parameters
   smoothly across biome transition zones.
   - **Rationale:** Per-biome atmosphere creates distinct mood per environment; smooth interpolation
     prevents jarring visual shifts at biome boundaries.
   - **Verification:** Walk from a swamp biome (green fog, low visibility) to a desert biome (warm
     tint, high visibility) and confirm all atmospheric parameters interpolate smoothly over the
     transition zone with no discontinuities.
