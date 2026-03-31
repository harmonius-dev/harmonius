# R-15.6 -- World Building Requirements

## Requirements

1. **R-15.6.1** — The engine **SHALL** provide terrain sculpting brushes (raise, lower, smooth,
   flatten, erode, noise) with configurable radius, strength, falloff, and incremental disk
   streaming.
   - **Rationale:** Sculpting is the primary terrain authoring workflow; streaming enables editing
     massive worlds.
   - **Verification:** Sculpt a 4 km x 4 km terrain and verify memory usage stays below the
     configured budget.

2. **R-15.6.2** — The engine **SHALL** simulate hydraulic and thermal erosion on GPU compute with
   real-time preview on regions up to 2048x2048 samples.
   - **Rationale:** Erosion produces natural terrain features faster than manual sculpting.
   - **Verification:** Run erosion on a 2048x2048 region and verify the preview updates at
     interactive frame rates.

3. **R-15.6.3** — The engine **SHALL** support terrain material painting with per-layer weight maps,
   triplanar projection, and macro-variation textures.
   - **Rationale:** Layer painting controls terrain surface appearance.
   - **Verification:** Paint three material layers and verify correct blending at layer boundaries.

4. **R-15.6.4** — The engine **SHALL** support water body placement via boundary splines and
   elevation with rendering integration for reflections, refraction, and caustics.
   - **Rationale:** Water bodies are fundamental to open-world environments.
   - **Verification:** Place a river along a spline and verify reflections render correctly at the
     water surface.

5. **R-15.6.5** — The engine **SHALL** provide vegetation painting with density brushes, per-species
   rules, and a biome rule system for declarative distribution.
   - **Rationale:** Procedural vegetation distribution handles large areas efficiently while
     allowing hand refinement.
   - **Verification:** Define a biome rule and verify vegetation spawns within the specified
     altitude and slope ranges.

6. **R-15.6.6** — The engine **SHALL** provide light probe and reflection probe placement tools with
   baking, real-time update, and influence region visualization.
   - **Rationale:** Probe placement controls global illumination quality.
   - **Verification:** Place a reflection probe, bake it, and verify reflections update on nearby
     surfaces.

7. **R-15.6.7** — The engine **SHALL** render a navmesh overlay with color-coded walkable areas,
   real-time regeneration, and pathfinding test markers.
   - **Rationale:** Navigation preview enables designers to verify AI pathing after geometry
     changes.
   - **Verification:** Edit terrain, regenerate the navmesh, and verify the overlay updates within 2
     seconds.

8. **R-15.6.8** — The engine **SHALL** display a World Grid overlay showing cell boundaries,
   streaming states, and budget warnings for cells exceeding entity or triangle limits.
   - **Rationale:** World grid management is critical for open-world streaming performance.
   - **Verification:** Exceed the entity budget in a cell and verify the overlay displays a warning
     indicator.

9. **R-15.6.9** — The engine **SHALL** provide SDF-based voxel sculpting brushes with real-time GPU
   meshing and undo/redo via a voxel delta log.
   - **Rationale:** Voxel sculpting enables caves, overhangs, and tunnels that heightmaps cannot
     represent.
   - **Verification:** Carve a tunnel, undo, and verify the terrain restores to its previous state.

10. **R-15.6.10** — The engine **SHALL** provide a visual fracture editor with Voronoi seed gizmos,
    real-time fragment preview, and a physics sandbox for test impacts.
    - **Rationale:** Destruction authoring requires interactive preview to tune fracture parameters.
    - **Verification:** Place seeds, apply a test impact, and verify fragments separate according to
      the configured break thresholds.

11. **R-15.6.11** — The engine **SHALL** support seamless editing across heightmap and voxel regions
    with automatic conversion when vertical complexity is introduced.
    - **Rationale:** Hybrid terrain combines heightmap efficiency with voxel flexibility.
    - **Verification:** Sculpt an overhang on a heightmap region and verify it converts to voxel
      with no visible seam.

12. **R-15.6.12** — The engine **SHALL** provide a library of preset SDF brush shapes and support
    user-created presets saved as reusable assets.
    - **Rationale:** Diverse brush shapes accelerate sculpting workflows.
    - **Verification:** Create a custom brush, save it, and verify it appears in the brush palette
      for all team members.
