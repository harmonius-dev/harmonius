# User Stories — 3.3 Foliage & Vegetation

## Stories

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-3.3.1 | player | I want foliage to render via GPU-driven instancing with compute-shader culling | I see dense forests across the open world at high frame rates |  |  |
| US-3.3.2 | world artist | I want to define density maps, biome rules, and slope/altitude constraints that populate terrain with foliage at runtime | I do not need to store per-instance placement data on disk |  |  |
| US-3.3.3 | player | I want distant foliage to transition through mesh, simplified mesh, and impostor LODs with crossfade dithering | I see no visible LOD pop |  |  |
| US-3.3.4 | world artist | I want foliage to sample a shared wind field texture with three-layer animation (trunk sway, branch oscillation, leaf flutter) | vegetation responds naturally to wind gusts |  |  |
| US-3.3.5 | player | I want grass to part around my character and bushes to bend from impacts with persistent displacement that decays over time | the world feels physically reactive |  |  |
| US-3.3.6 | world artist | I want procedurally generated grass blades driven by terrain material layers and noise functions | meadows and fields look lush with hundreds of thousands of visible blades |  |  |
| US-3.3.7 | world artist | I want trees with bark PBR and two-sided leaf shading with subsurface transmission, per-species wind skeletons, and seamless LOD | forests look visually convincing when backlit |  |  |
