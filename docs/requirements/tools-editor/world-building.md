# R-15.6 -- World Building Requirements

## Terrain Sculpting

| ID       | Derived From |
|----------|--------------|
| R-15.6.1 |              |
| R-15.6.2 |              |

1. **R-15.6.1** — The editor **SHALL** provide raise, lower, smooth, flatten, erode, and noise
   sculpting brushes with configurable radius, strength, falloff curve, and shape mask, using
   incremental async disk I/O so that a 16k x 16k heightmap stays under 512 MB peak memory.
   [F-15.6.1](../../features/tools-editor/world-building.md) into memory. MB. Verify async I/O does
   not block worker threads.
   - **Rationale:** Massive open-world terrains must be sculptable without loading the entire
     heightmap
   - **Verification:** Benchmark: sculpt a 16k x 16k heightmap and verify peak memory stays under
     512
2. **R-15.6.2** — The editor **SHALL** simulate hydraulic and thermal erosion on selected terrain
   regions via GPU compute with configurable parameters (rain amount, sediment capacity, thermal
   angle), providing real-time preview above 15 FPS on 2048x2048 regions.
   [F-15.6.2](../../features/tools-editor/world-building.md) enables real-time feedback. twice with
   same params and verify identical output.
   - **Rationale:** Natural-looking valleys and sediment deposits require simulation; GPU compute
   - **Verification:** Benchmark: run erosion on 2048x2048 and verify preview stays above 15 FPS.
     Run

## Terrain Painting

| ID       | Derived From |
|----------|--------------|
| R-15.6.3 |              |

1. **R-15.6.3** — The editor **SHALL** support painting material layers onto terrain tiles with
   per-layer weight maps, height-based and slope-based auto-painting rules, triplanar projection for
   cliff faces, and macro-variation textures.
   [F-15.6.3](../../features/tools-editor/world-building.md) layers.
   - **Rationale:** Multi-layer terrain texturing with auto rules accelerates large-world coverage.
   - **Verification:** Unit test: verify weight maps sum to 1.0 at every texel after painting
     multiple

## Water

| ID       | Derived From |
|----------|--------------|
| R-15.6.4 |              |

1. **R-15.6.4** — The editor **SHALL** place rivers via boundary splines with configurable width,
   depth, and flow speed, and lakes that fill to a specified altitude with automatic shoreline
   masking, integrating with reflections, refraction, caustics, and foam rendering.
   [F-15.6.4](../../features/tools-editor/world-building.md) altitude-based lakes match artist
   mental models.
   - **Rationale:** Water bodies are fundamental to world building; spline-based rivers and
   - **Verification:** Unit test: create a lake at altitude 100 and verify surface height equals
     100.

## Vegetation

| ID       | Derived From |
|----------|--------------|
| R-15.6.5 |              |

1. **R-15.6.5** — The editor **SHALL** support density brushes for vegetation painting with
   per-species slope limits, altitude bands, proximity exclusion radii, clustering behavior, and a
   biome rule system for declarative auto-population with deterministic results given the same seed.
   [F-15.6.5](../../features/tools-editor/world-building.md) authoring. Verify zero placement rule
   violations.
   - **Rationale:** Ecological distribution rules and auto-population accelerate large-world
     vegetation
   - **Verification:** Unit test: auto-populate with a seed, verify same result on repeated runs.

## Lighting

| ID       | Derived From |
|----------|--------------|
| R-15.6.6 |              |

1. **R-15.6.6** — The editor **SHALL** support light probe placement on tetrahedral grids or
   manually, reflection probes with configurable capture volumes and blend distances, baked and
   real-time update modes, and visualization overlays showing probe influence regions.
   [F-15.6.6](../../features/tools-editor/world-building.md) coverage. configuration.
   - **Rationale:** Indirect lighting and reflection quality depend on correct probe placement and
   - **Verification:** Unit test: place a tetrahedral grid and verify probe count and spacing match

## Navigation

| ID       | Derived From |
|----------|--------------|
| R-15.6.7 |              |

1. **R-15.6.7** — The editor **SHALL** display a translucent navmesh overlay with color-coded
   walkable areas and slope limits, real-time navmesh regeneration for selected regions, and
   pathfinding test markers for start and goal.
   [F-15.6.7](../../features/tools-editor/world-building.md)
   - **Rationale:** Immediate navmesh feedback prevents unreachable areas and broken AI paths.
   - **Verification:** Unit test: verify the overlay marks steep slopes as non-walkable.

## World Partition

| ID       | Derived From |
|----------|--------------|
| R-15.6.8 |              |

1. **R-15.6.8** — The editor **SHALL** display a 2D minimap of world partition cell boundaries, 3D
   viewport overlay of streaming states (loaded, pending, unloaded), cell ownership for multi-user
   editing, and entity/triangle budget violation flags per cell.
   [F-15.6.8](../../features/tools-editor/world-building.md) awareness. is set.
   - **Rationale:** Open-world streaming requires visual budget tracking and multi-user cell
     ownership
   - **Verification:** Unit test: add entities exceeding a cell budget and verify the over-budget
     flag

## Voxel Terrain Editing

| ID        | Derived From |
|-----------|--------------|
| R-15.6.9  |              |
| R-15.6.10 |              |
| R-15.6.11 |              |
| R-15.6.12 |              |

1. **R-15.6.9** — The editor **SHALL** provide SDF sculpting brushes (add, subtract, smooth,
   flatten, paint material) that operate on voxel terrain volumes with configurable radius,
   strength, falloff curve, and material ID. Modified chunks **SHALL** be re-meshed via GPU compute
   and displayed within the same frame as the brush stroke. Sculpting operations **SHALL** support
   undo/redo via a delta log of modified voxels. In hybrid terrain mode, sculpting **SHALL**
   automatically convert heightmap regions to voxel when the edit introduces vertical complexity.
   [F-15.6.9](../../features/tools-editor/world-building.md) visual feedback; GPU compute meshing
   enables frame-rate sculpting. within the same frame. Apply undo and verify the volume restores to
   its previous state.
   - **Rationale:** Voxel terrain authoring requires real-time interactive sculpting with immediate
   - **Verification:** Unit test: apply a subtract brush to a voxel volume and verify the mesh
     updates
2. **R-15.6.10** — The editor **SHALL** provide a visual fracture pattern editor that displays
   Voronoi fracture seeds as viewport gizmos with interactive add, remove, and reposition
   operations. The editor **SHALL** show real-time fracture preview (fragment geometry, connectivity
   graph, joint placement) and support simulated test impacts in a physics preview sandbox without
   affecting level state. Adjustable parameters **SHALL** include Voronoi seed count, break
   threshold per joint, fragment mass distribution, and debris lifetime.
   [F-15.6.10](../../features/tools-editor/world-building.md) preview; a sandbox simulation lets
   artists validate destruction without play-testing. shows 10 corresponding fragments. Simulate a
   test impact and verify fragments separate without modifying the level state.
   - **Rationale:** Destruction is a visual, physical feature that requires interactive authoring
     and
   - **Verification:** Unit test: place 10 Voronoi seeds on an object and verify the fracture
     preview
3. **R-15.6.11** — The editor **SHALL** support seamless editing across heightmap and voxel regions,
   automatically converting heightmap regions to voxel when vertical complexity is introduced and
   offering reverse conversion when voxel regions are flattened. The editor **SHALL** display visual
   indicators distinguishing heightmap from voxel regions and preview boundary stitching in real
   time. Material painting **SHALL** work identically across both representations.
   [F-15.6.11](../../features/tools-editor/world-building.md) conversion management breaks creative
   flow. conversion to voxel. Flatten the region and verify the conversion-back prompt appears.
   Verify material painting produces identical results on both representations.
   - **Rationale:** Artists need a unified editing experience across terrain representations; manual
   - **Verification:** Unit test: sculpt an overhang on a heightmap region and verify automatic
4. **R-15.6.12** — The editor **SHALL** provide a library of preset SDF brush shapes (sphere, cube,
   cylinder, cone, torus, noise, custom mesh) with per-brush parameters. Brush presets **SHALL** be
   saveable as reusable project assets. Custom mesh brushes **SHALL** import an arbitrary mesh and
   convert it to an SDF for sculpting. The brush library **SHALL** appear in a categorized palette
   in the sculpting toolbar. [F-15.6.12](../../features/tools-editor/world-building.md) enables
   stamping complex shapes without manual sculpting. modification matches the expected shape. Save a
   custom brush preset and verify it appears in the palette on reload.
   - **Rationale:** A rich brush library accelerates terrain authoring; custom mesh-to-SDF
     conversion
   - **Verification:** Unit test: apply each built-in brush shape and verify the resulting SDF

---

## US-15.6.1 Terrain Sculpting Brushes

| ID          | Persona       | Features | Requirements |
|-------------|---------------|----------|--------------|
| US-15.6.1.1 | artist        |          |              |
| US-15.6.1.2 | artist        |          |              |
| US-15.6.1.3 | artist        |          |              |
| US-15.6.1.4 | artist        |          |              |
| US-15.6.1.5 | designer      |          |              |
| US-15.6.1.6 | engine tester |          |              |

1. **US-15.6.1.1** — I want raise, lower, smooth, flatten, erode, and noise brushes so that I can
   shape terrain heightmaps directly in the viewport.
2. **US-15.6.1.2** — I want configurable brush radius, strength, and falloff curve so that I can
   sculpt at any scale from fine detail to broad strokes.
3. **US-15.6.1.3** — I want shape masks on sculpting brushes so that I can create patterned terrain
   features (footprints, tire tracks).
4. **US-15.6.1.4** — I want incremental streaming to disk during sculpting so that I can shape
   massive heightmaps without loading them entirely into memory.
5. **US-15.6.1.5** — I want sculpting operations integrated with undo/redo so that I can revert
   terrain changes incrementally.
6. **US-15.6.1.6** — I want to verify sculpting a 16k x 16k heightmap stays under 512 MB peak memory
   so that streaming sculpt performance is regression-tested.

## US-15.6.2 Terrain Erosion

| ID          | Persona       | Features | Requirements |
|-------------|---------------|----------|--------------|
| US-15.6.2.1 | artist        |          |              |
| US-15.6.2.2 | artist        |          |              |
| US-15.6.2.3 | artist        |          |              |
| US-15.6.2.4 | artist        |          |              |
| US-15.6.2.5 | engine tester |          |              |

1. **US-15.6.2.1** — I want hydraulic erosion simulation on selected terrain regions so that I can
   create natural-looking valleys and drainage patterns.
2. **US-15.6.2.2** — I want thermal erosion for cliff weathering so that I can add realistic
   sediment deposits and talus slopes.
3. **US-15.6.2.3** — I want configurable erosion parameters so that I can control rain amount,
   sediment capacity, and thermal angle.
4. **US-15.6.2.4** — I want real-time erosion preview via GPU compute so that I can see results
   interactively before committing.
5. **US-15.6.2.5** — I want to verify erosion preview runs above 15 FPS on 2048x2048 regions so that
   GPU erosion performance is regression-tested.

## US-15.6.3 Terrain Material Painting

| ID          | Persona       | Features | Requirements |
|-------------|---------------|----------|--------------|
| US-15.6.3.1 | artist        |          |              |
| US-15.6.3.2 | artist        |          |              |
| US-15.6.3.3 | artist        |          |              |
| US-15.6.3.4 | artist        |          |              |
| US-15.6.3.5 | artist        |          |              |
| US-15.6.3.6 | engine tester |          |              |

1. **US-15.6.3.1** — I want to paint material layers onto terrain tiles so that I can texture
   landscapes with grass, rock, dirt, and snow.
2. **US-15.6.3.2** — I want per-layer weight maps for precise blending so that I can control exactly
   where each material appears.
3. **US-15.6.3.3** — I want height-based and slope-based auto-painting rules so that rock
   auto-applies to steep slopes and snow to high elevations.
4. **US-15.6.3.4** — I want triplanar projection for cliff faces so that vertical surfaces receive
   properly oriented textures.
5. **US-15.6.3.5** — I want macro-variation textures to break tiling so that large terrain expanses
   avoid visible repetition patterns.
6. **US-15.6.3.6** — I want to verify weight maps sum to 1.0 at every texel so that material
   blending correctness is regression-tested.

## US-15.6.4 Water Body Placement

| ID          | Persona       | Features | Requirements |
|-------------|---------------|----------|--------------|
| US-15.6.4.1 | designer      |          |              |
| US-15.6.4.2 | designer      |          |              |
| US-15.6.4.3 | designer      |          |              |
| US-15.6.4.4 | artist        |          |              |
| US-15.6.4.5 | artist        |          |              |
| US-15.6.4.6 | engine tester |          |              |

1. **US-15.6.4.1** — I want to place rivers using boundary splines so that I can define water paths
   with precise control.
2. **US-15.6.4.2** — I want configurable river width, depth, and flow speed so that each river
   segment has distinct visual and gameplay characteristics.
3. **US-15.6.4.3** — I want lakes that fill to a specified altitude so that standing water volumes
   conform to terrain automatically.
4. **US-15.6.4.4** — I want automatic shoreline masking for lakes so that water edges blend
   naturally with terrain materials.
5. **US-15.6.4.5** — I want water bodies to integrate with reflections, refraction, caustics, and
   foam rendering so that water looks realistic without manual shader setup.
6. **US-15.6.4.6** — I want to verify lake surface height matches the specified altitude so that
   water placement precision is regression-tested.

## US-15.6.5 Vegetation Painting with Density Rules

| ID          | Persona       | Features | Requirements |
|-------------|---------------|----------|--------------|
| US-15.6.5.1 | artist        |          |              |
| US-15.6.5.2 | artist        |          |              |
| US-15.6.5.3 | artist        |          |              |
| US-15.6.5.4 | artist        |          |              |
| US-15.6.5.5 | designer      |          |              |
| US-15.6.5.6 | designer      |          |              |
| US-15.6.5.7 | engine tester |          |              |

1. **US-15.6.5.1** — I want density brushes for painting vegetation instances so that I can populate
   terrain with per-species control.
2. **US-15.6.5.2** — I want per-species slope limits and altitude bands so that vegetation respects
   ecological distribution rules.
3. **US-15.6.5.3** — I want proximity exclusion radii between species so that trees do not overlap
   or crowd unnaturally.
4. **US-15.6.5.4** — I want clustering behavior for grouped vegetation so that forest patches form
   naturally rather than uniformly.
5. **US-15.6.5.5** — I want a biome rule system for declarative vegetation so that large regions
   auto-populate with ecologically consistent flora.
6. **US-15.6.5.6** — I want hand-refinement after auto-population so that auto-generated vegetation
   can be adjusted per-instance.
7. **US-15.6.5.7** — I want to verify no instances violate placement rules after auto-population so
   that biome rule enforcement is regression-tested.

## US-15.6.6 Lighting Setup (Light Probes and Reflection Probes)

| ID          | Persona       | Features | Requirements |
|-------------|---------------|----------|--------------|
| US-15.6.6.1 | artist        |          |              |
| US-15.6.6.2 | artist        |          |              |
| US-15.6.6.3 | artist        |          |              |
| US-15.6.6.4 | artist        |          |              |
| US-15.6.6.5 | artist        |          |              |
| US-15.6.6.6 | engine tester |          |              |

1. **US-15.6.6.1** — I want to place light probes on tetrahedral grids or manually so that I can
   control indirect lighting sample distribution.
2. **US-15.6.6.2** — I want reflection probes with configurable capture volumes so that reflections
   match the local environment accurately.
3. **US-15.6.6.3** — I want blend distances on reflection probes so that transitions between probe
   volumes are smooth.
4. **US-15.6.6.4** — I want visualization overlays showing probe influence regions so that I can
   verify coverage and identify gaps.
5. **US-15.6.6.5** — I want baking and real-time update support for probes so that I can choose
   quality vs. performance per probe.
6. **US-15.6.6.6** — I want to verify probe influence regions render correctly in the overlay so
   that visualization accuracy is regression-tested.

## US-15.6.7 Navmesh Preview

| ID          | Persona       | Features | Requirements |
|-------------|---------------|----------|--------------|
| US-15.6.7.1 | designer      |          |              |
| US-15.6.7.2 | designer      |          |              |
| US-15.6.7.3 | designer      |          |              |
| US-15.6.7.4 | designer      |          |              |
| US-15.6.7.5 | engine tester |          |              |

1. **US-15.6.7.1** — I want a translucent navmesh overlay in the viewport so that I can see walkable
   areas while editing the world.
2. **US-15.6.7.2** — I want color-coded walkable areas and slope limits so that I can identify
   non-walkable zones visually.
3. **US-15.6.7.3** — I want real-time navmesh regeneration for selected regions so that I can verify
   pathing after terrain edits immediately.
4. **US-15.6.7.4** — I want pathfinding test markers for start and goal so that I can test AI paths
   directly in the viewport.
5. **US-15.6.7.5** — I want to verify the navmesh overlay marks steep slopes as non-walkable so that
   slope limit visualization is regression-tested.

## US-15.6.8 World Partition Visualization

| ID          | Persona           | Features | Requirements |
|-------------|-------------------|----------|--------------|
| US-15.6.8.1 | designer          |          |              |
| US-15.6.8.2 | designer          |          |              |
| US-15.6.8.3 | designer          |          |              |
| US-15.6.8.4 | designer          |          |              |
| US-15.6.8.5 | creative director |          |              |
| US-15.6.8.6 | engine tester     |          |              |

1. **US-15.6.8.1** — I want a 2D minimap showing world partition cell boundaries so that I can see
   the full world grid layout at a glance.
2. **US-15.6.8.2** — I want 3D viewport overlay of streaming states so that I can see which cells
   are loaded, pending, or unloaded.
3. **US-15.6.8.3** — I want cell ownership display for multiplayer editing so that I can see which
   team member is editing which zone.
4. **US-15.6.8.4** — I want cells exceeding entity or triangle budgets flagged so that I can
   identify and fix over-budget zones.
5. **US-15.6.8.5** — I want LOD streaming distance visualization so that I can verify draw distance
   settings across the world.
6. **US-15.6.8.6** — I want to verify over-budget cells display a warning indicator so that budget
   violation visualization is regression-tested.
