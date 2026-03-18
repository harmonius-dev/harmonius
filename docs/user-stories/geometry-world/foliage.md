# User Stories — 3.3 Foliage & Vegetation

## Stories

| ID       | Persona      | Features | Requirements |
|----------|--------------|----------|--------------|
| US-3.3.1 | player       |          |              |
| US-3.3.2 | world artist |          |              |
| US-3.3.3 | player       |          |              |
| US-3.3.4 | world artist |          |              |
| US-3.3.5 | player       |          |              |
| US-3.3.6 | world artist |          |              |
| US-3.3.7 | world artist |          |              |

1. **US-3.3.1** — I want foliage to render via GPU-driven instancing with compute-shader culling
   - **Acceptance:** I see dense forests across the open world at high frame rates
2. **US-3.3.2** — I want to define density maps, biome rules, and slope/altitude constraints that
   populate terrain with foliage at runtime
   - **Acceptance:** I do not need to store per-instance placement data on disk
3. **US-3.3.3** — I want distant foliage to transition through mesh, simplified mesh, and impostor
   LODs with crossfade dithering
   - **Acceptance:** I see no visible LOD pop
4. **US-3.3.4** — I want foliage to sample a shared wind field texture with three-layer animation
   (trunk sway, branch oscillation, leaf flutter)
   - **Acceptance:** vegetation responds naturally to wind gusts
5. **US-3.3.5** — I want grass to part around my character and bushes to bend from impacts with
   persistent displacement that decays over time
   - **Acceptance:** the world feels physically reactive
6. **US-3.3.6** — I want procedurally generated grass blades driven by terrain material layers and
   noise functions
   - **Acceptance:** meadows and fields look lush with hundreds of thousands of visible blades
7. **US-3.3.7** — I want trees with bark PBR and two-sided leaf shading with subsurface
   transmission, per-species wind skeletons, and seamless LOD
   - **Acceptance:** forests look visually convincing when backlit
