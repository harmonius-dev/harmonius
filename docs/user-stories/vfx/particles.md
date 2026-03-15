# User Stories — 11.1 Particle System

## US-11.1.1 Fill the Screen With Spell Effects Without Dropping Frames
**As an** artist, **I want** GPU-driven particle simulation with composable modules, sprite/
ribbon/mesh rendering, LOD budgeting, sub-emitters, particle lights, and fluid simulation,
**so that** I can author visually rich combat effects that scale to 40-player raids without
frame drops.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Compute shader simulation for millions of particles | F-11.1.1 | R-11.1.1 |
| Composable modules fused into single dispatch | F-11.1.2 | R-11.1.2 |
| Sprite, ribbon, and mesh rendering modes | F-11.1.3 | R-11.1.3 |
| LOD tiers with global budget and priority culling | F-11.1.4 | R-11.1.4 |
| Event-driven sub-emitters for cascading effects | F-11.1.5 | R-11.1.5 |
| Particle lights in clustered light buffer with cap | F-11.1.6 | R-11.1.6 |
| GPU fluid simulation for fire, smoke, and liquid | F-11.1.7 | R-11.1.7 |
| Max 500K alive particles with 2ms GPU compute budget | F-11.1.4 | R-X.13.1 |
