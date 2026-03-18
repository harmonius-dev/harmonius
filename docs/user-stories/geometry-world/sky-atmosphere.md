# User Stories — 3.5 Sky & Atmosphere

## Stories

| ID       | Persona          | Features | Requirements |
|----------|------------------|----------|--------------|
| US-3.5.1 | technical artist |          |              |
| US-3.5.2 | player           |          |              |
| US-3.5.3 | player           |          |              |
| US-3.5.4 | player           |          |              |
| US-3.5.5 | player           |          |              |
| US-3.5.6 | player           |          |              |
| US-3.5.7 | technical artist |          |              |

1. **US-3.5.1** — I want an analytical sky model (Preetham/Hosek-Wilkie) that evaluates sky color
   for any sun position
   - **Acceptance:** I have a fast fallback when volumetric scattering exceeds the budget
2. **US-3.5.2** — I want physically-based atmosphere scattering with aerial perspective applied via
   froxel volumes
   - **Acceptance:** distant terrain and structures fade naturally into atmospheric haze across tens
     of kilometers
3. **US-3.5.3** — I want ray-marched volumetric clouds driven by weather maps with temporal
   reprojection
   - **Acceptance:** the sky has dynamic, physically-lit cloud formations
4. **US-3.5.4** — I want cloud coverage to cast large-scale moving shadows on terrain, foliage, and
   water
   - **Acceptance:** I see dynamic shadow patterns that reinforce weather and world scale
5. **US-3.5.5** — I want sun and moon positions to follow astronomical arcs with smooth transitions
   through dawn, day, dusk, and night
   - **Acceptance:** the time-of-day cycle feels natural and immersive
6. **US-3.5.6** — I want the night sky to show magnitude-based stars with twinkling, a
   phase-accurate moon, and planetary bodies
   - **Acceptance:** the celestial sky is visually rich and astronomically plausible
7. **US-3.5.7** — I want the sky, atmosphere, and clouds captured into an environment cubemap on a
   round-robin schedule
   - **Acceptance:** scene ambient and specular lighting always reflects the current sky state
