# R-3.6 — Biome-Driven Terrain Generation Requirements

## Biome Painting and Authoring

| ID        | Derived From                                             |
|-----------|----------------------------------------------------------|
| R-3.6.65  | [F-3.6.65](../../features/geometry-world/proc-gen.md)    |
| R-3.6.65a | [F-3.6.65](../../features/geometry-world/proc-gen.md)    |
| R-3.6.66  | [F-3.6.66](../../features/geometry-world/proc-gen.md)    |
| R-3.6.67  | [F-3.6.67](../../features/geometry-world/proc-gen.md)    |
| R-3.6.67a | [F-3.6.67](../../features/geometry-world/proc-gen.md)    |

1. **R-3.6.65** — The engine **SHALL** provide a brush-based editor tool that writes biome IDs into
   a per-tile biome map at terrain tile resolution. Painted biome regions **SHALL** override
   procedurally generated biome assignments (F-3.6.44). The tool **SHALL** support circle, square,
   and spline-lasso brush shapes.
   - **Rationale:** Designers need direct control over biome placement to create hand-crafted
     regions within procedurally generated worlds.
   - **Verification:** Paint three distinct biome regions onto adjacent terrain tiles. Confirm biome
     IDs are written correctly to the biome map. Confirm painted regions override procedural
     assignments. Confirm all three brush shapes produce correct region boundaries.

2. **R-3.6.65a** — The biome painting tool **SHALL** update terrain material preview, vegetation
   density preview, and weather state preview within 500ms of a brush stroke completing.
   - **Rationale:** Interactive preview is essential for rapid iteration on biome layout.
   - **Verification:** Paint a biome region and measure time from brush release to terrain material
     update, vegetation density update, and weather state change in the viewport. All three updates
     complete within 500ms.

3. **R-3.6.66** — The engine **SHALL** support biome templates as versioned data assets, each
   defining: terrain stamp rules, material splatmap weights, vegetation species list with density
   constraints, weather state probabilities, ambient audio profile, and fog/atmosphere overrides.
   Templates **SHALL** be composable from existing terrain stamps (F-3.6.9), vegetation rules
   (F-3.6.12), and weather states (F-2.7.10).
   - **Rationale:** Data-driven biome templates enable designers to create, share, and version biome
     definitions without code.
   - **Verification:** Create a biome template with all six properties. Apply it to a region.
     Confirm terrain, vegetation, weather, audio, and atmosphere all reflect the template settings.
     Modify the template and confirm hot-reload updates the region.

4. **R-3.6.67** — The engine **SHALL** blend biome properties (terrain height, material weights,
   vegetation density, weather parameters) across biome boundaries using noise-perturbed gradient
   transitions with configurable per-biome-pair transition width. Ecotone vegetation rules **SHALL**
   be evaluated in transition zones.
   - **Rationale:** Abrupt biome boundaries look artificial. Gradient transitions with noise
     perturbation produce organic, natural-looking boundaries.
   - **Verification:** Place two biome regions adjacent to each other. Confirm material weights
     interpolate smoothly across the boundary. Confirm noise perturbation produces irregular
     boundary shapes. Confirm ecotone species appear only in the transition zone.

5. **R-3.6.67a** — Biome boundary blending **SHALL** complete within 2ms per terrain tile on desktop
   GPU compute. Transition zone width **SHALL** be configurable from 5m to 500m per biome pair.
   - **Rationale:** Blending must be fast enough for interactive editing and runtime streaming.
   - **Verification:** Benchmark blending a tile with 4 biome boundaries. Confirm GPU compute time
     is under 2ms. Configure transition widths at 5m and 500m and confirm both produce correct
     results.

## Biome-Driven Terrain Generation

| ID        | Derived From                                             |
|-----------|----------------------------------------------------------|
| R-3.6.68  | [F-3.6.68](../../features/geometry-world/proc-gen.md)    |
| R-3.6.68a | [F-3.6.68](../../features/geometry-world/proc-gen.md)    |
| R-3.6.69  | [F-3.6.69](../../features/geometry-world/proc-gen.md)    |

1. **R-3.6.68** — The engine **SHALL** generate terrain heightmap detail per tile from biome
   assignments, applying biome-specific stamp stacks (dune ridges for desert, jagged peaks for
   mountain, shallow depressions for swamp). Generated height modifications **SHALL** composite
   non-destructively with the base heightfield via the stamp system (F-3.6.9).
   - **Rationale:** Biome-driven terrain shaping ensures landscape character matches biome type
     without manual sculpting.
   - **Verification:** Assign desert, mountain, and swamp biomes to three adjacent tiles. Confirm
     each tile's heightfield exhibits biome-appropriate terrain features. Remove a biome assignment
     and confirm the base heightfield is restored.

2. **R-3.6.68a** — Biome-driven heightmap generation **SHALL** complete within 5ms per terrain tile
   on desktop. Generation **SHALL** execute asynchronously without blocking the main thread.
   - **Rationale:** Async generation prevents frame drops during streaming and editing.
   - **Verification:** Stream 10 terrain tiles with biome-driven generation. Measure per-tile
     generation time (under 5ms). Confirm no main-thread stalls during generation.

3. **R-3.6.69** — The engine **SHALL** assign terrain material splatmap weights per tile based on
   biome type and terrain analysis (slope, altitude, curvature, moisture). Material assignments
   **SHALL** blend across biome boundaries (F-3.6.67). Designer texture stamp overrides (F-3.6.10)
   **SHALL** take priority over biome-generated materials.
   - **Rationale:** Automatic material assignment from biome data eliminates manual splatmap
     painting for large worlds.
   - **Verification:** Assign a forest biome and confirm grass/dirt materials dominate flat areas
     and rock dominates slopes. Place a texture stamp override and confirm it takes priority. Verify
     material blending at a biome boundary.

## Biome-Driven Vegetation and Weather

| ID        | Derived From                                             |
|-----------|----------------------------------------------------------|
| R-3.6.70  | [F-3.6.70](../../features/geometry-world/proc-gen.md)    |
| R-3.6.70a | [F-3.6.70](../../features/geometry-world/proc-gen.md)    |
| R-3.6.71  | [F-3.6.71](../../features/geometry-world/proc-gen.md)    |
| R-3.6.71a | [F-3.6.71](../../features/geometry-world/proc-gen.md)    |
| R-3.6.72  | [F-3.6.72](../../features/geometry-world/proc-gen.md)    |

1. **R-3.6.70** — The engine **SHALL** scatter vegetation instances per terrain tile using
   biome-specific species lists, density curves, clustering rules, and altitude/slope constraints
   from biome templates (F-3.6.66). Biome-exclusive species **SHALL** appear only in their assigned
   biome. Transition zones **SHALL** blend species from adjacent biomes with ecotone rules.
   - **Rationale:** Biome-driven vegetation creates visually distinct regions with appropriate
     flora.
   - **Verification:** Assign forest and desert biomes to adjacent tiles. Confirm forest tile has
     trees and undergrowth. Confirm desert tile has cacti and sparse scrub. Confirm no forest trees
     appear in the desert tile. Confirm ecotone shrubs appear in the transition zone.

2. **R-3.6.70a** — Biome-driven vegetation scattering **SHALL** support at least 50 species per
   biome template and scatter at least 100K instances per visible tile on desktop within 3ms per
   tile via GPU compute.
   - **Rationale:** Rich vegetation diversity and density are needed for visually convincing biomes
     at scale.
   - **Verification:** Configure a biome with 50 species. Scatter vegetation on a visible tile.
     Confirm at least 100K instances. Benchmark GPU compute scatter time at under 3ms.

3. **R-3.6.71** — The engine **SHALL** drive the weather system (F-2.7.10) from biome regions,
   adjusting weather state probabilities, fog density, wind intensity, cloud coverage, and
   precipitation type per biome. Weather **SHALL** transition smoothly as the player moves between
   biome regions, interpolating weather parameters across biome boundaries.
   - **Rationale:** Biome-linked weather makes each region feel distinct and immersive.
   - **Verification:** Configure a tropical biome (frequent rain) adjacent to a desert biome (rare
     rain, dust storms). Move the player from tropical to desert. Confirm weather transitions
     smoothly: rain fades, dust storm probability increases, fog and cloud coverage change. Confirm
     no abrupt weather state jumps.

4. **R-3.6.71a** — Weather zone transitions **SHALL** complete within 10 seconds of the player
   crossing a biome boundary. Weather parameter interpolation **SHALL** consume less than 0.5ms per
   frame.
   - **Rationale:** Smooth transitions avoid jarring weather jumps while keeping the per-frame cost
     negligible.
   - **Verification:** Cross a biome boundary and measure time from boundary crossing to stable
     weather state. Confirm under 10 seconds. Profile per-frame weather interpolation cost and
     confirm under 0.5ms.

5. **R-3.6.72** — The engine **SHALL** support per-biome seasonal variation with vegetation color
   shifts, material weight adjustments (snow in winter, mud in spring), weather probability
   overrides, and vegetation density changes driven by a global season clock. Season duration
   **SHALL** be designer-configurable.
   - **Rationale:** Seasonal variation adds temporal depth and visual diversity to biome regions.
   - **Verification:** Advance the season clock through four seasons in a temperate forest biome.
     Confirm autumn foliage color shift, winter snow coverage, spring mud and bloom, and summer full
     green. Confirm deciduous trees lose leaves in winter.

## Additional Features

| ID       | Derived From                                              |
|----------|-----------------------------------------------------------|
| R-3.6.73 | [F-3.6.73](../../features/geometry-world/proc-gen.md)     |
| R-3.6.74 | [F-3.6.74](../../features/geometry-world/proc-gen.md)     |

1. **R-3.6.73** — The engine **SHALL** crossfade ambient audio profiles between biome regions as the
   listener moves, with per-season audio overrides. Concurrent ambient source count **SHALL** be at
   least 8 on desktop and 4 on mobile.
   - **Rationale:** Audio reinforces biome identity and seasonal atmosphere.
   - **Verification:** Move the listener from a forest biome (birdsong) to a tundra biome (wind).
     Confirm crossfade occurs over the transition zone. Switch to winter season and confirm ambient
     audio changes (fewer birds, more wind).

2. **R-3.6.74** — The engine **SHALL** provide a debug overlay showing biome boundaries, biome IDs,
   transition zones, and per-tile generation status. A minimap view **SHALL** render biome-colored
   terrain at world scale.
   - **Rationale:** Debug visualization is essential for verifying biome coverage and diagnosing
     generation issues.
   - **Verification:** Enable the biome debug overlay. Confirm biome boundaries, IDs, and transition
     zones are visible. Open the minimap and confirm biome colors match the assigned biome types
     across the world.
