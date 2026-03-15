# User Stories — 3.3 Foliage & Vegetation

## US-3.3.1 Render Millions of Foliage Instances
**As a** player, **I want** foliage to render via GPU-driven instancing with compute-shader culling,
**so that** I see dense forests across the open world at high frame rates.

## US-3.3.2 Place Foliage Procedurally from Rules
**As a** world artist, **I want** to define density maps, biome rules, and slope/altitude
constraints that populate terrain with foliage at runtime, **so that** I do not need to store
per-instance placement data on disk.

## US-3.3.3 Smooth Foliage LOD at Distance
**As a** player, **I want** distant foliage to transition through mesh, simplified mesh, and
impostor LODs with crossfade dithering, **so that** I see no visible LOD pop.

## US-3.3.4 Animate Foliage with Wind
**As a** world artist, **I want** foliage to sample a shared wind field texture with three-layer
animation (trunk sway, branch oscillation, leaf flutter), **so that** vegetation responds naturally
to wind gusts.

## US-3.3.5 Interact with Vegetation
**As a** player, **I want** grass to part around my character and bushes to bend from impacts with
persistent displacement that decays over time, **so that** the world feels physically reactive.

## US-3.3.6 Render Dense Grass Fields
**As a** world artist, **I want** procedurally generated grass blades driven by terrain material
layers and noise functions, **so that** meadows and fields look lush with hundreds of thousands of
visible blades.

## US-3.3.7 Render Trees with Subsurface Leaves
**As a** world artist, **I want** trees with bark PBR and two-sided leaf shading with subsurface
transmission, per-species wind skeletons, and seamless LOD, **so that** forests look visually
convincing when backlit.
