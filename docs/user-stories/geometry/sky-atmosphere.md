# User Stories -- 3.5 Sky & Atmosphere

## Procedural Sky

| ID         | Persona                  |
|------------|--------------------------|
| US-3.5.1.1 | environment artist (P-8) |
| US-3.5.1.2 | level designer (P-6)     |
| US-3.5.1.3 | technical artist (P-13)  |

1. **US-3.5.1.1** -- **As a** environment artist (P-8), **I want** a physically-motivated analytical
   sky model that computes sky color for any sun position, **so that** I get correct sky gradients
   without manual painting.
2. **US-3.5.1.2** -- **As a** level designer (P-6), **I want** the analytical sky to serve as a
   fallback when volumetric atmosphere is budget-limited, **so that** every platform has a valid
   sky.
3. **US-3.5.1.3** -- **As a** technical artist (P-13), **I want** atmospheric turbidity exposed as a
   tunable parameter, **so that** I can control haze and sky clarity per scene.

## Physically-Based Atmosphere Scattering

| ID         | Persona                  |
|------------|--------------------------|
| US-3.5.2.1 | environment artist (P-8) |
| US-3.5.2.2 | technical artist (P-13)  |
| US-3.5.2.3 | level designer (P-6)     |
| US-3.5.2.4 | engine developer (P-26)  |

1. **US-3.5.2.1** -- **As a** environment artist (P-8), **I want** precomputed multi-scattering
   atmosphere LUTs with Rayleigh, Mie, and ozone absorption, **so that** the sky and aerial
   perspective are physically accurate.
2. **US-3.5.2.2** -- **As a** technical artist (P-13), **I want** LUTs recomputed when atmosphere
   parameters change, **so that** I can adjust scattering in real time.
3. **US-3.5.2.3** -- **As a** level designer (P-6), **I want** aerial perspective applied via a
   froxel volume to all scene geometry, **so that** distant terrain fades naturally into haze.
4. **US-3.5.2.4** -- **As a** engine developer (P-26), **I want** LUT resolution to scale per
   platform tier, **so that** mobile uses fewer scattering samples.

## Volumetric Clouds

| ID         | Persona                  |
|------------|--------------------------|
| US-3.5.3.1 | environment artist (P-8) |
| US-3.5.3.2 | technical artist (P-13)  |
| US-3.5.3.3 | level designer (P-6)     |
| US-3.5.3.4 | engine developer (P-26)  |
| US-3.5.4.1 | level designer (P-6)     |
| US-3.5.4.2 | environment artist (P-8) |

1. **US-3.5.3.1** -- **As a** environment artist (P-8), **I want** clouds rendered by ray marching
   through layered noise volumes, **so that** clouds have volumetric depth and realistic lighting.
2. **US-3.5.3.2** -- **As a** technical artist (P-13), **I want** cloud coverage and type driven by
   a weather map texture, **so that** I can animate or script weather patterns.
3. **US-3.5.3.3** -- **As a** level designer (P-6), **I want** temporal reprojection to amortize
   ray-march cost across frames, **so that** volumetric clouds are affordable on all platforms.
4. **US-3.5.3.4** -- **As a** engine developer (P-26), **I want** ray-march step count configurable
   per platform tier, **so that** mobile uses fewer steps while desktop uses full quality.
5. **US-3.5.4.1** -- **As a** level designer (P-6), **I want** cloud shadows projected onto terrain
   and scene geometry, **so that** moving cloud shadows reinforce the sense of weather and scale.
6. **US-3.5.4.2** -- **As a** environment artist (P-8), **I want** cloud shadow map resolution
   configurable per platform tier, **so that** mobile uses lower-resolution shadow maps.

## Dynamic Time of Day

| ID         | Persona                  |
|------------|--------------------------|
| US-3.5.5.1 | level designer (P-6)     |
| US-3.5.5.2 | environment artist (P-8) |
| US-3.5.5.3 | technical artist (P-13)  |

1. **US-3.5.5.1** -- **As a** level designer (P-6), **I want** a time-of-day controller driving sun
   and moon positions along astronomical arcs, **so that** sky, light, and shadow update
   continuously.
2. **US-3.5.5.2** -- **As a** environment artist (P-8), **I want** smooth dawn-day-dusk-night
   transitions that update atmosphere LUTs and ambient lighting, **so that** time progression looks
   natural.
3. **US-3.5.5.3** -- **As a** technical artist (P-13), **I want** a time scale parameter for
   gameplay-controlled day/night cycles, **so that** accelerated cycles work for various game
   genres.

## Celestial Bodies

| ID         | Persona                  |
|------------|--------------------------|
| US-3.5.6.1 | environment artist (P-8) |
| US-3.5.6.2 | level designer (P-6)     |
| US-3.5.6.3 | technical artist (P-13)  |

1. **US-3.5.6.1** -- **As a** environment artist (P-8), **I want** sun, moon, stars, and planetary
   bodies rendered as part of the sky dome, **so that** the sky looks complete at all times of day.
2. **US-3.5.6.2** -- **As a** level designer (P-6), **I want** all celestial positions
   astronomically computed or artist-overridden, **so that** I can choose realism or stylization per
   project.
3. **US-3.5.6.3** -- **As a** technical artist (P-13), **I want** star catalog rendering with
   magnitude-based brightness and atmospheric extinction, **so that** night skies are convincing.

## Sky Capture

| ID         | Persona                  |
|------------|--------------------------|
| US-3.5.7.1 | engine developer (P-26)  |
| US-3.5.7.2 | environment artist (P-8) |
| US-3.5.7.3 | technical artist (P-13)  |

1. **US-3.5.7.1** -- **As a** engine developer (P-26), **I want** the sky captured into a
   low-resolution environment cubemap on a round-robin schedule, **so that** IBL reflections always
   match the current sky.
2. **US-3.5.7.2** -- **As a** environment artist (P-8), **I want** ambient diffuse and specular
   lighting derived from the captured cubemap, **so that** scene lighting responds to sky changes
   automatically.
3. **US-3.5.7.3** -- **As a** technical artist (P-13), **I want** cubemap capture resolution and
   update rate configurable per platform tier, **so that** mobile amortizes capture cost over more
   frames.

## Parent Stories

The 3-segment parent stories below are umbrella rollups for the refined 4-segment sub-stories listed
above. Each parent inherits the persona of its first sub-story and describes the umbrella capability
that the sub-stories refine.

| ID | Persona |
|----|---------|
| US-3.5.1 | environment artist (P-8) |
| US-3.5.2 | environment artist (P-8) |
| US-3.5.3 | environment artist (P-8) |
| US-3.5.4 | level designer (P-6) |
| US-3.5.5 | level designer (P-6) |
| US-3.5.6 | environment artist (P-8) |
| US-3.5.7 | engine developer (P-26) |

1. **US-3.5.1** -- **As a** environment artist (P-8), **I want** the capabilities defined in
   sub-stories US-3.5.1.1 through US-3.5.1.3 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

2. **US-3.5.2** -- **As a** environment artist (P-8), **I want** the capabilities defined in
   sub-stories US-3.5.2.1 through US-3.5.2.4 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

3. **US-3.5.3** -- **As a** environment artist (P-8), **I want** the capabilities defined in
   sub-stories US-3.5.3.1 through US-3.5.3.4 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

4. **US-3.5.4** -- **As a** level designer (P-6), **I want** the capabilities defined in sub-stories
   US-3.5.4.1 through US-3.5.4.2 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

5. **US-3.5.5** -- **As a** level designer (P-6), **I want** the capabilities defined in sub-stories
   US-3.5.5.1 through US-3.5.5.3 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

6. **US-3.5.6** -- **As a** environment artist (P-8), **I want** the capabilities defined in
   sub-stories US-3.5.6.1 through US-3.5.6.3 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

7. **US-3.5.7** -- **As a** engine developer (P-26), **I want** the capabilities defined in
   sub-stories US-3.5.7.1 through US-3.5.7.3 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.
