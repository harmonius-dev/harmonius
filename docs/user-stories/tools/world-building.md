# User Stories -- 15.6 World Building

## Stories

| ID           | Persona                 |
|--------------|-------------------------|
| US-15.6.1.1  | level designer (P-6)    |
| US-15.6.1.2  | game designer (P-5)     |
| US-15.6.2.1  | level designer (P-6)    |
| US-15.6.2.2  | technical artist (P-13) |
| US-15.6.3.1  | level designer (P-6)    |
| US-15.6.3.2  | technical artist (P-13) |
| US-15.6.4.1  | level designer (P-6)    |
| US-15.6.4.2  | game designer (P-5)     |
| US-15.6.5.1  | level designer (P-6)    |
| US-15.6.5.2  | game designer (P-5)     |
| US-15.6.6.1  | technical artist (P-13) |
| US-15.6.6.2  | level designer (P-6)    |
| US-15.6.7.1  | level designer (P-6)    |
| US-15.6.7.2  | game designer (P-5)     |
| US-15.6.8.1  | level designer (P-6)    |
| US-15.6.8.2  | game designer (P-5)     |
| US-15.6.9.1  | level designer (P-6)    |
| US-15.6.9.2  | game designer (P-5)     |
| US-15.6.10.1 | game designer (P-5)     |
| US-15.6.10.2 | technical artist (P-13) |
| US-15.6.11.1 | level designer (P-6)    |
| US-15.6.11.2 | technical artist (P-13) |
| US-15.6.12.1 | level designer (P-6)    |
| US-15.6.12.2 | technical artist (P-13) |
| US-15.6.13.1 | level designer (P-6)    |

1. **US-15.6.1.1** — **As a** level designer (P-6), **I want** sculpting brushes for raising,
   lowering, smoothing, and flattening terrain, **so that** I can shape heightmaps directly in the
   viewport.

2. **US-15.6.1.2** — **As a** game designer (P-5), **I want** sculpting to stream incrementally to
   disk, **so that** I can edit massive open-world terrain without loading everything into memory.

3. **US-15.6.2.1** — **As a** level designer (P-6), **I want** hydraulic and thermal erosion
   simulation on selected regions, **so that** terrain has natural-looking valleys and ridges.

4. **US-15.6.2.2** — **As a** technical artist (P-13), **I want** erosion to run on GPU compute with
   real-time preview, **so that** I can iterate parameters interactively.

5. **US-15.6.3.1** — **As a** level designer (P-6), **I want** to paint material layers onto terrain
   with per-layer weight maps, **so that** surfaces blend naturally.

6. **US-15.6.3.2** — **As a** technical artist (P-13), **I want** triplanar projection for cliff
   faces and macro-variation textures, **so that** tiling artifacts are eliminated.

7. **US-15.6.4.1** — **As a** level designer (P-6), **I want** to place water bodies using boundary
   splines and elevation, **so that** rivers and lakes integrate with the terrain.

8. **US-15.6.4.2** — **As a** game designer (P-5), **I want** water bodies to integrate with
   reflections, refraction, and caustics, **so that** they render convincingly.

9. **US-15.6.5.1** — **As a** level designer (P-6), **I want** density brushes with per-species
   placement rules, **so that** vegetation distributes naturally.

10. **US-15.6.5.2** — **As a** game designer (P-5), **I want** a biome rule system for declarative
    vegetation distribution, **so that** large regions auto-populate and are hand-refined.

11. **US-15.6.6.1** — **As a** technical artist (P-13), **I want** placement tools for light probes
    and reflection probes with baking and visualization overlays, **so that** I can tune global
    illumination.

12. **US-15.6.6.2** — **As a** level designer (P-6), **I want** probe influence region
    visualization, **so that** I can avoid gaps and overlaps in indirect lighting.

13. **US-15.6.7.1** — **As a** level designer (P-6), **I want** a navmesh overlay with color-coded
    walkable areas and real-time regeneration, **so that** I can verify AI pathing.

14. **US-15.6.7.2** — **As a** game designer (P-5), **I want** pathfinding test markers showing AI
    routes, **so that** I can validate navigation without running the full game.

15. **US-15.6.8.1** — **As a** level designer (P-6), **I want** a World Grid overlay showing cell
    boundaries and streaming states, **so that** I can manage zone assignments.

16. **US-15.6.8.2** — **As a** game designer (P-5), **I want** the World Grid display to flag cells
    exceeding entity or triangle budgets, **so that** I can balance content density.

17. **US-15.6.9.1** — **As a** level designer (P-6), **I want** SDF-based voxel sculpting brushes
    with real-time GPU meshing, **so that** I can carve caves and overhangs.

18. **US-15.6.9.2** — **As a** game designer (P-5), **I want** voxel sculpting to support undo/redo
    via a voxel delta log, **so that** destructive edits are reversible.

19. **US-15.6.10.1** — **As a** game designer (P-5), **I want** a visual fracture pattern editor
    with Voronoi seed gizmos, **so that** I can author destruction behavior interactively.

20. **US-15.6.10.2** — **As a** technical artist (P-13), **I want** real-time fracture preview with
    physics sandbox simulation, **so that** I can test impact behavior without running the game.

21. **US-15.6.11.1** — **As a** level designer (P-6), **I want** seamless editing across heightmap
    and voxel regions, **so that** I can sculpt overhangs without manual region conversion.

22. **US-15.6.11.2** — **As a** technical artist (P-13), **I want** a visual indicator showing which
    regions are heightmap versus voxel, **so that** I understand the underlying representation.

23. **US-15.6.12.1** — **As a** level designer (P-6), **I want** a library of preset SDF brush
    shapes including sphere, cube, cylinder, and noise, **so that** I have diverse sculpting tools.

24. **US-15.6.12.2** — **As a** technical artist (P-13), **I want** to create and share custom brush
    presets with per-brush parameter defaults, **so that** the team has consistent sculpting tools.

25. **US-15.6.13.1** — **As a** level designer (P-6), **I want** to lock an individual world cell
    for editing so that my colleague can simultaneously edit a different cell, **so that** two
    designers can collaborate on the same world without stepping on each other's changes.

## Parent Stories

The 3-segment parent stories below are umbrella rollups for the refined 4-segment sub-stories listed
above. Each parent inherits the persona of its first sub-story and describes the umbrella capability
that the sub-stories refine.

| ID | Persona |
|----|---------|
| US-15.6.1 | level designer (P-6) |
| US-15.6.10 | game designer (P-5) |
| US-15.6.11 | level designer (P-6) |
| US-15.6.12 | level designer (P-6) |
| US-15.6.13 | level designer (P-6) |
| US-15.6.2 | level designer (P-6) |
| US-15.6.3 | level designer (P-6) |
| US-15.6.4 | level designer (P-6) |
| US-15.6.5 | level designer (P-6) |
| US-15.6.6 | technical artist (P-13) |
| US-15.6.7 | level designer (P-6) |
| US-15.6.8 | level designer (P-6) |
| US-15.6.9 | level designer (P-6) |

1. **US-15.6.1** -- **As a** level designer (P-6), **I want** the capabilities defined in
   sub-stories US-15.6.1.1 through US-15.6.1.2 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

2. **US-15.6.10** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-15.6.10.1 through US-15.6.10.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

3. **US-15.6.11** -- **As a** level designer (P-6), **I want** the capabilities defined in
   sub-stories US-15.6.11.1 through US-15.6.11.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

4. **US-15.6.12** -- **As a** level designer (P-6), **I want** the capabilities defined in
   sub-stories US-15.6.12.1 through US-15.6.12.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

5. **US-15.6.13** -- **As a** level designer (P-6), **I want** the capabilities defined in
   sub-stories US-15.6.13.1 through US-15.6.13.1 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

6. **US-15.6.2** -- **As a** level designer (P-6), **I want** the capabilities defined in
   sub-stories US-15.6.2.1 through US-15.6.2.2 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

7. **US-15.6.3** -- **As a** level designer (P-6), **I want** the capabilities defined in
   sub-stories US-15.6.3.1 through US-15.6.3.2 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

8. **US-15.6.4** -- **As a** level designer (P-6), **I want** the capabilities defined in
   sub-stories US-15.6.4.1 through US-15.6.4.2 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

9. **US-15.6.5** -- **As a** level designer (P-6), **I want** the capabilities defined in
   sub-stories US-15.6.5.1 through US-15.6.5.2 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

10. **US-15.6.6** -- **As a** technical artist (P-13), **I want** the capabilities defined in
    sub-stories
US-15.6.6.1 through US-15.6.6.2 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

11. **US-15.6.7** -- **As a** level designer (P-6), **I want** the capabilities defined in
    sub-stories
US-15.6.7.1 through US-15.6.7.2 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

12. **US-15.6.8** -- **As a** level designer (P-6), **I want** the capabilities defined in
    sub-stories
US-15.6.8.1 through US-15.6.8.2 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

13. **US-15.6.9** -- **As a** level designer (P-6), **I want** the capabilities defined in
    sub-stories
US-15.6.9.1 through US-15.6.9.2 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.
