# User Stories -- 11.4 Weather and Environmental FX

## Precipitation

| ID          | Persona                  | Features | Requirements |
|-------------|--------------------------|----------|--------------|
| US-11.4.1.1 | Player (P-23)            | F-11.4.1 | R-11.4.1     |
| US-11.4.1.2 | Effects artist (P-12)    | F-11.4.1 | R-11.4.1     |
| US-11.4.1.3 | Engine tester (P-27)     | F-11.4.1 | R-11.4.1     |
| US-11.4.2.1 | Player (P-23)            | F-11.4.2 | R-11.4.2     |
| US-11.4.2.2 | Engine tester (P-27)     | F-11.4.2 | R-11.4.2     |
| US-11.4.2.3 | Technical artist (P-13)  | F-11.4.2 | R-11.4.2     |
| US-11.4.3.1 | Player (P-23)            | F-11.4.3 | R-11.4.3     |
| US-11.4.3.2 | Environment artist (P-8) | F-11.4.3 | R-11.4.3     |
| US-11.4.3.3 | Engine tester (P-27)     | F-11.4.3 | R-11.4.3     |

1. **US-11.4.1.1** — I want multi-layered rain combining GPU particle streaks, screen-space camera
   droplets that merge and evaporate, and ripple normals on wet surfaces, so that rain feels
   immersive from world scale down to close-up camera effects.
   - **Acceptance:** World-space particle streaks visible; screen-space droplets flow downward and
     merge; ripple normals appear on surfaces; screen droplets cease under shelter
2. **US-11.4.1.2** — I want particle density and screen droplet frequency scaled by weather
   intensity values, so that light drizzle and heavy downpour look and feel distinct.
   - **Acceptance:** Particle density decreases proportionally with weather intensity; distinct
     visual difference between drizzle and downpour
3. **US-11.4.1.3** — I want to verify that mobile uses a single particle layer with no screen
   droplets, rain density at 25% of desktop, and half-res ripple normals, so that rain rendering
   fits within mobile bandwidth and compute budgets.
   - **Acceptance:** Mobile: single particle layer, no screen droplets, 25% density, half-res
     ripples
4. **US-11.4.2.1** — I want dynamic puddles that form based on terrain heightfield accumulation,
   modify surface roughness to mirror-smooth, darken albedo, and show animated ripples during active
   rainfall, so that rain transforms the ground surface convincingly.
   - **Acceptance:** Puddles form in concave areas; surface roughness and albedo modified; animated
     ripple normals during rain; puddles drain after rain stops
5. **US-11.4.2.2** — I want to verify that mobile uses pre-placed puddle decals with albedo-darken
   only (no dynamic heightfield, no roughness modification), so that puddle rendering is cheap on
   mobile.
   - **Acceptance:** Mobile: pre-placed puddle decals, albedo-darken only, no dynamic heightfield
6. **US-11.4.2.3** — I want per-material wet surface responses (stone darkens, metal reflects, dirt
   becomes mud), so that each surface type reacts to rain in a physically appropriate way.
   - **Acceptance:** Stone surfaces darken; metal surfaces increase reflectivity; dirt transitions
     to mud
7. **US-11.4.3.1** — I want vertex-displacement snow that accumulates on surfaces over time based on
   weather state, with footprints and vehicle tracks carving deformation trails, so that snowfall
   gradually transforms the landscape.
   - **Acceptance:** Snow accumulates on upward-facing surfaces; deformation trails appear from
     movement; trails fill in under continued snowfall; walls remain snow-free
8. **US-11.4.3.2** — I want configurable snow accumulation rate, maximum depth, and deformation
   trail fade speed per zone, so that tundra zones accumulate snow faster than mountain passes.
   - **Acceptance:** Per-zone accumulation rate and max depth configurable; deformation trail fade
     speed configurable
9. **US-11.4.3.3** — I want to verify that mobile uses texture-blend snow with decal-based
   deformation instead of vertex displacement, so that snow rendering stays within mobile vertex
   processing budgets.
   - **Acceptance:** Mobile: texture-blend snow, decal-based deformation, no vertex displacement

## Atmospheric Effects

| ID          | Persona                  | Features | Requirements |
|-------------|--------------------------|----------|--------------|
| US-11.4.4.1 | Environment artist (P-8) | F-11.4.4 | R-11.4.4     |
| US-11.4.4.2 | Engine tester (P-27)     | F-11.4.4 | R-11.4.4     |
| US-11.4.5.1 | Player (P-23)            | F-11.4.5 | R-11.4.5     |
| US-11.4.5.2 | Effects artist (P-12)    | F-11.4.5 | R-11.4.5     |
| US-11.4.5.3 | Engine tester (P-27)     | F-11.4.5 | R-11.4.5     |
| US-11.4.6.1 | Player (P-23)            | F-11.4.6 | R-11.4.6     |
| US-11.4.6.2 | Effects artist (P-12)    | F-11.4.6 | R-11.4.6     |
| US-11.4.6.3 | Engine tester (P-27)     | F-11.4.6 | R-11.4.6     |

1. **US-11.4.4.1** — I want oriented box and convex hull fog volumes with per-volume density, color,
   and height falloff that inject into the global froxel grid, so that specific areas have localized
   atmospheric effects without affecting global fog.
   - **Acceptance:** Fog density confined to volume bounds; density, color, and height falloff
     independently configurable; areas outside volume unaffected
2. **US-11.4.4.2** — I want to verify that mobile uses screen-space height fog instead of volumetric
   froxel injection, with limited concurrent fog volumes, so that fog effects work on mobile without
   volumetric rendering.
   - **Acceptance:** Mobile: screen-space height fog, no froxel injection, limited concurrent
     volumes
3. **US-11.4.5.1** — I want procedural branching lightning bolts that emit light bursts illuminating
   terrain and clouds for a single frame with exponential decay, so that thunderstorms have
   dramatic, dynamic lighting events.
   - **Acceptance:** Branching geometry with randomized structure; single-frame illumination with
     exponential decay; multiple simultaneous bolts supported
4. **US-11.4.5.2** — I want configurable branching depth, segment length, and simultaneous bolt
   count for storm sequences, so that I can build intensity from single bolts to full electrical
   storms.
   - **Acceptance:** Branching depth and segment length configurable; simultaneous bolt count
     configurable; thunder audio delay increases with distance
5. **US-11.4.5.3** — I want to verify that mobile limits branching depth to 2 with max 1
   simultaneous bolt and uses a single directional light flash, so that lightning rendering stays
   within mobile compute and lighting budgets.
   - **Acceptance:** Mobile: branching depth 2, max 1 bolt, single directional flash
6. **US-11.4.6.1** — I want GPU particle-driven wind visualization with leaves, dust, and debris
   carried by the global wind field coherently with vegetation sway, so that wind feels like a
   unified physical force affecting all visual elements.
   - **Acceptance:** Leaves and debris move coherently with vegetation sway; particle velocity
     sampled from shared wind volume
7. **US-11.4.6.2** — I want dust storms that inject scattering density into atmospheric fog and tint
   the sky toward storm color, so that approaching dust storms create dramatic visibility reduction
   and color shift across the world.
   - **Acceptance:** Visibility decreases as scattering density increases; sky color tints toward
     storm color
8. **US-11.4.6.3** — I want to verify that mobile uses 10% of desktop wind particle count and dust
   storms use distance fog only without volumetric scattering, so that wind visualization fits
   within mobile budgets.
   - **Acceptance:** Mobile: 10% wind particle count, distance fog only for dust storms

## Underwater Effects

| ID          | Persona                  | Features | Requirements |
|-------------|--------------------------|----------|--------------|
| US-11.4.7.1 | Player (P-23)            | F-11.4.7 | R-11.4.7     |
| US-11.4.7.2 | Environment artist (P-8) | F-11.4.7 | R-11.4.7     |
| US-11.4.7.3 | Engine tester (P-27)     | F-11.4.7 | R-11.4.7     |

1. **US-11.4.7.1** — I want animated caustic light patterns projected onto underwater surfaces with
   wavelength-dependent depth fog (reds fade before blues) and bubble streams, so that diving
   underwater feels atmospheric and visually rich.
   - **Acceptance:** Animated caustics on submerged geometry; exponential depth fog with red
     attenuation before blue; visible bubble streams
2. **US-11.4.7.2** — I want a refraction distortion layer at the water surface boundary and
   screen-space god rays from above, so that underwater environments have the characteristic
   rippling interface and light shaft effects.
   - **Acceptance:** Refraction distortion at water surface boundary; screen-space god rays from
     surface direction
3. **US-11.4.7.3** — I want to verify that mobile skips caustic projection and god rays, using depth
   fog and simplified blue tint only with bubbles at 25% count, so that underwater rendering stays
   within mobile GPU budgets.
   - **Acceptance:** Mobile: no caustics, no god rays, depth fog with blue tint only, 25% bubble
     count
