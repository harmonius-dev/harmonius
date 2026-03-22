# User Stories — Biome-Driven Terrain Generation

## Biome Painting and Authoring

| ID        | Persona                | Features  | Requirements |
|-----------|------------------------|-----------|--------------|
| US-3.6.65 | game designer (P-5)    | F-3.6.65  | R-3.6.65     |
| US-3.6.66 | env. artist (P-8)      | F-3.6.65  | R-3.6.65a    |
| US-3.6.67 | level designer (P-6)   | F-3.6.66  | R-3.6.66     |
| US-3.6.68 | env. artist (P-8)      | F-3.6.67  | R-3.6.67     |
| US-3.6.69 | engine tester (P-27)   | F-3.6.65  | R-3.6.65     |

1. **US-3.6.65** — As a game designer, I want to paint biome regions directly onto terrain using a
   brush tool, so that I can define where forests, deserts, and tundra appear without writing
   generation rules.
   - **Acceptance:** Select a biome type, paint a region with a brush, and see the terrain update
     with appropriate materials and vegetation within 500ms.

2. **US-3.6.66** — As an environment artist, I want real-time preview of terrain materials,
   vegetation, and weather as I paint biome regions, so that I can iterate on world layout visually.
   - **Acceptance:** Terrain materials change under the brush stroke. Vegetation density updates in
     the viewport. Weather state shifts to match the painted biome.

3. **US-3.6.67** — As a level designer, I want to create custom biome templates combining terrain
   stamps, vegetation species, weather rules, and audio profiles, so that I can define new biome
   types for my game's setting.
   - **Acceptance:** Create a "volcanic wasteland" biome template with lava rock materials, dead
     trees, smoke particles, and thunderstorm weather. Paint it onto terrain and confirm all
     elements appear correctly.

4. **US-3.6.68** — As an environment artist, I want biome boundaries to blend with noise-perturbed
   organic transitions, so that biome edges look natural rather than straight lines.
   - **Acceptance:** Adjacent biome regions show irregular boundary shapes. Materials, vegetation,
     and weather interpolate smoothly across the transition zone.

5. **US-3.6.69** — As an engine tester, I want to verify that painted biome regions override
   procedural biome assignments, so that I can confirm the priority system works correctly.
   - **Acceptance:** Generate a world with procedural biomes. Paint over a region with a different
     biome. Confirm the painted biome takes priority. Remove the paint and confirm the procedural
     biome restores.

## Biome-Driven Terrain Generation

| ID        | Persona                | Features  | Requirements |
|-----------|------------------------|-----------|--------------|
| US-3.6.70 | level designer (P-6)   | F-3.6.68  | R-3.6.68     |
| US-3.6.71 | tech. artist (P-13)    | F-3.6.68  | R-3.6.68a    |
| US-3.6.72 | env. artist (P-8)      | F-3.6.69  | R-3.6.69     |
| US-3.6.73 | engine tester (P-27)   | F-3.6.69  | R-3.6.69     |

1. **US-3.6.70** — As a level designer, I want biome assignments to automatically generate
   appropriate terrain features (dunes for desert, jagged peaks for mountains, depressions for
   swamps), so that painting a biome creates believable landscape without manual sculpting.
   - **Acceptance:** Paint a desert biome and see dune ridges appear. Paint a mountain biome and see
     jagged peaks. Paint a swamp biome and see flattened terrain with shallow pools.

2. **US-3.6.71** — As a technical artist, I want biome-driven heightmap generation to complete
   within 5ms per tile asynchronously, so that terrain streams without frame drops.
   - **Acceptance:** Profile tile generation during streaming. Per-tile time is under 5ms. No
     main-thread stalls occur.

3. **US-3.6.72** — As an environment artist, I want biome regions to automatically assign terrain
   materials based on slope, altitude, and moisture, so that each biome has appropriate ground
   textures without manual splatmap painting.
   - **Acceptance:** A forest biome shows grass on flats and rock on slopes. A desert biome shows
     sand on flats and sandstone on slopes. Materials blend at biome boundaries.

4. **US-3.6.73** — As an engine tester, I want to verify that designer texture stamp overrides take
   priority over biome material assignments, so that hero areas render correctly.
   - **Acceptance:** Apply a biome material assignment, then place a texture stamp override. Confirm
     the stamp takes priority in the overlapping region.

## Biome-Driven Vegetation and Weather

| ID        | Persona                | Features  | Requirements |
|-----------|------------------------|-----------|--------------|
| US-3.6.74 | level designer (P-6)   | F-3.6.70  | R-3.6.70     |
| US-3.6.75 | game designer (P-5)    | F-3.6.71  | R-3.6.71     |
| US-3.6.76 | player (P-23)          | F-3.6.71  | R-3.6.71     |
| US-3.6.77 | env. artist (P-8)      | F-3.6.72  | R-3.6.72     |
| US-3.6.78 | engine tester (P-27)   | F-3.6.70  | R-3.6.70a    |

1. **US-3.6.74** — As a level designer, I want each biome to automatically scatter appropriate
   vegetation species with biome-specific density and clustering, so that forests have dense trees
   and deserts have sparse cacti without manual placement.
   - **Acceptance:** A forest biome scatters deciduous trees, underbrush, and ferns. A desert biome
     scatters cacti and scrub. No forest trees appear in the desert. Ecotone shrubs appear at the
     boundary.

2. **US-3.6.75** — As a game designer, I want weather to change based on which biome the player is
   in, so that each region feels distinct (tropical rain, desert dust storms, tundra blizzards).
   - **Acceptance:** Walk from a tropical biome into a desert biome. Rain fades out, dust storm
     probability increases. Fog, cloud coverage, and precipitation type change smoothly during the
     transition.

3. **US-3.6.76** — As a player, I want smooth weather transitions as I travel between biome regions,
   so that I experience immersive environmental changes without jarring weather jumps.
   - **Acceptance:** Cross a biome boundary. Weather transitions gradually over approximately 10
     seconds. No abrupt state changes. Cloud coverage, fog, and precipitation interpolate smoothly.

4. **US-3.6.77** — As an environment artist, I want biomes to change with in-game seasons (autumn
   foliage, winter snow, spring blooms), so that the world feels alive and dynamic across time.
   - **Acceptance:** Advance through four seasons in a temperate forest. Autumn shows orange/red
     foliage. Winter shows snow on ground and bare trees. Spring shows blooms and mud. Summer shows
     full green canopy.

5. **US-3.6.78** — As an engine tester, I want to verify that biome vegetation scattering handles 50
   species per biome and 100K instances per tile within the performance budget, so that I can
   confirm scalability.
   - **Acceptance:** Configure a biome with 50 species. Scatter on a visible tile. Count at least
     100K instances. GPU compute time is under 3ms.

## Additional Features

| ID        | Persona                | Features  | Requirements |
|-----------|------------------------|-----------|--------------|
| US-3.6.79 | audio designer (P-14)  | F-3.6.73  | R-3.6.73     |
| US-3.6.80 | level designer (P-6)   | F-3.6.74  | R-3.6.74     |
| US-3.6.81 | engine tester (P-27)   | F-3.6.73  | R-3.6.73     |

1. **US-3.6.79** — As an audio designer, I want each biome to have a distinct ambient soundscape
   that crossfades as the player moves between regions, so that audio reinforces the visual biome
   identity.
   - **Acceptance:** Walk from forest (birdsong) to tundra (wind). Audio crossfades across the
     transition zone. Switch to winter and confirm ambient audio changes.

2. **US-3.6.80** — As a level designer, I want a debug overlay and minimap showing biome boundaries,
   IDs, and generation status, so that I can verify coverage and diagnose issues.
   - **Acceptance:** Enable the overlay. Biome boundaries, IDs, and transition zones are visible.
     The minimap shows biome-colored terrain at world scale.

3. **US-3.6.81** — As an engine tester, I want to verify biome audio crossfade timing and concurrent
   source limits, so that I can confirm audio transitions work within the budget.
   - **Acceptance:** Cross a biome boundary and confirm crossfade completes within the transition
     zone. Verify at least 8 concurrent ambient sources on desktop and 4 on mobile.
