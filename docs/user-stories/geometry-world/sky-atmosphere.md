# User Stories — 3.5 Sky & Atmosphere

## Stories

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-3.5.1 | technical artist | I want an analytical sky model (Preetham/Hosek-Wilkie) that evaluates sky color for any sun position | I have a fast fallback when volumetric scattering exceeds the budget |  |  |
| US-3.5.2 | player | I want physically-based atmosphere scattering with aerial perspective applied via froxel volumes | distant terrain and structures fade naturally into atmospheric haze across tens of kilometers |  |  |
| US-3.5.3 | player | I want ray-marched volumetric clouds driven by weather maps with temporal reprojection | the sky has dynamic, physically-lit cloud formations |  |  |
| US-3.5.4 | player | I want cloud coverage to cast large-scale moving shadows on terrain, foliage, and water | I see dynamic shadow patterns that reinforce weather and world scale |  |  |
| US-3.5.5 | player | I want sun and moon positions to follow astronomical arcs with smooth transitions through dawn, day, dusk, and night | the time-of-day cycle feels natural and immersive |  |  |
| US-3.5.6 | player | I want the night sky to show magnitude-based stars with twinkling, a phase-accurate moon, and planetary bodies | the celestial sky is visually rich and astronomically plausible |  |  |
| US-3.5.7 | technical artist | I want the sky, atmosphere, and clouds captured into an environment cubemap on a round-robin schedule | scene ambient and specular lighting always reflects the current sky state |  |  |
