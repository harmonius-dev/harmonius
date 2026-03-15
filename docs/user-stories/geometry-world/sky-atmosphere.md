# User Stories — 3.5 Sky & Atmosphere

## US-3.5.1 Fast Analytical Sky
**As a** technical artist, **I want** an analytical sky model (Preetham/Hosek-Wilkie) that evaluates
sky color for any sun position, **so that** I have a fast fallback when volumetric scattering
exceeds the budget.

## US-3.5.2 Atmospheric Haze Over Distance
**As a** player, **I want** physically-based atmosphere scattering with aerial perspective applied
via froxel volumes, **so that** distant terrain and structures fade naturally into atmospheric haze
across tens of kilometers.

## US-3.5.3 Volumetric Cloud Skies
**As a** player, **I want** ray-marched volumetric clouds driven by weather maps with temporal
reprojection, **so that** the sky has dynamic, physically-lit cloud formations.

## US-3.5.4 Cloud Shadows on Terrain
**As a** player, **I want** cloud coverage to cast large-scale moving shadows on terrain, foliage,
and water, **so that** I see dynamic shadow patterns that reinforce weather and world scale.

## US-3.5.5 Dynamic Day-Night Cycle
**As a** player, **I want** sun and moon positions to follow astronomical arcs with smooth
transitions through dawn, day, dusk, and night, **so that** the time-of-day cycle feels natural and
immersive.

## US-3.5.6 See Stars, Moon, and Planets
**As a** player, **I want** the night sky to show magnitude-based stars with twinkling, a
phase-accurate moon, and planetary bodies, **so that** the celestial sky is visually rich and
astronomically plausible.

## US-3.5.7 Ambient Lighting Matches Sky
**As a** technical artist, **I want** the sky, atmosphere, and clouds captured into an environment
cubemap on a round-robin schedule, **so that** scene ambient and specular lighting always reflects
the current sky state.
