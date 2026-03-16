# R-15.6 -- World Building Requirements

## Terrain Sculpting

### R-15.6.1 Terrain Sculpting Brushes

The editor **SHALL** provide raise, lower, smooth, flatten, erode, and noise sculpting brushes with
configurable radius, strength, falloff curve, and shape mask, using incremental async disk I/O so
that a 16k x 16k heightmap stays under 512 MB peak memory.

- **Derived from:**
  [F-15.6.1](../../features/tools-editor/world-building.md)
- **Rationale:** Massive open-world terrains must be sculptable without loading the entire heightmap
  into memory.
- **Verification:** Benchmark: sculpt a 16k x 16k heightmap and verify peak memory stays under 512
  MB. Verify async I/O does not block worker threads.

### R-15.6.2 Terrain Erosion

The editor **SHALL** simulate hydraulic and thermal erosion on selected terrain regions via GPU
compute with configurable parameters (rain amount, sediment capacity, thermal angle), providing
real-time preview above 15 FPS on 2048x2048 regions.

- **Derived from:**
  [F-15.6.2](../../features/tools-editor/world-building.md)
- **Rationale:** Natural-looking valleys and sediment deposits require simulation; GPU compute
  enables real-time feedback.
- **Verification:** Benchmark: run erosion on 2048x2048 and verify preview stays above 15 FPS. Run
  twice with same params and verify identical output.

## Terrain Painting

### R-15.6.3 Terrain Material Painting

The editor **SHALL** support painting material layers onto terrain tiles with per-layer weight maps,
height-based and slope-based auto-painting rules, triplanar projection for cliff faces, and
macro-variation textures.

- **Derived from:**
  [F-15.6.3](../../features/tools-editor/world-building.md)
- **Rationale:** Multi-layer terrain texturing with auto rules accelerates large-world coverage.
- **Verification:** Unit test: verify weight maps sum to 1.0 at every texel after painting multiple
  layers.

## Water

### R-15.6.4 Water Body Placement

The editor **SHALL** place rivers via boundary splines with configurable width, depth, and flow
speed, and lakes that fill to a specified altitude with automatic shoreline masking, integrating
with reflections, refraction, caustics, and foam rendering.

- **Derived from:**
  [F-15.6.4](../../features/tools-editor/world-building.md)
- **Rationale:** Water bodies are fundamental to world building; spline-based rivers and
  altitude-based lakes match artist mental models.
- **Verification:** Unit test: create a lake at altitude 100 and verify surface height equals 100.

## Vegetation

### R-15.6.5 Vegetation Painting with Density Rules

The editor **SHALL** support density brushes for vegetation painting with per-species slope limits,
altitude bands, proximity exclusion radii, clustering behavior, and a biome rule system for
declarative auto-population with deterministic results given the same seed.

- **Derived from:**
  [F-15.6.5](../../features/tools-editor/world-building.md)
- **Rationale:** Ecological distribution rules and auto-population accelerate large-world vegetation
  authoring.
- **Verification:** Unit test: auto-populate with a seed, verify same result on repeated runs.
  Verify zero placement rule violations.

## Lighting

### R-15.6.6 Lighting Setup

The editor **SHALL** support light probe placement on tetrahedral grids or manually, reflection
probes with configurable capture volumes and blend distances, baked and real-time update modes, and
visualization overlays showing probe influence regions.

- **Derived from:**
  [F-15.6.6](../../features/tools-editor/world-building.md)
- **Rationale:** Indirect lighting and reflection quality depend on correct probe placement and
  coverage.
- **Verification:** Unit test: place a tetrahedral grid and verify probe count and spacing match
  configuration.

## Navigation

### R-15.6.7 Navmesh Preview

The editor **SHALL** display a translucent navmesh overlay with color-coded walkable areas and slope
limits, real-time navmesh regeneration for selected regions, and pathfinding test markers for start
and goal.

- **Derived from:**
  [F-15.6.7](../../features/tools-editor/world-building.md)
- **Rationale:** Immediate navmesh feedback prevents unreachable areas and broken AI paths.
- **Verification:** Unit test: verify the overlay marks steep slopes as non-walkable.

## World Partition

### R-15.6.8 World Partition Visualization

The editor **SHALL** display a 2D minimap of world partition cell boundaries, 3D viewport overlay of
streaming states (loaded, pending, unloaded), cell ownership for multi-user editing, and
entity/triangle budget violation flags per cell.

- **Derived from:**
  [F-15.6.8](../../features/tools-editor/world-building.md)
- **Rationale:** Open-world streaming requires visual budget tracking and multi-user cell ownership
  awareness.
- **Verification:** Unit test: add entities exceeding a cell budget and verify the over-budget flag
  is set.

---

## User Stories

## US-15.6.1 Terrain Sculpting Brushes

### US-15.6.1.1

As a **artist (P-8)**, I want raise, lower, smooth, flatten, erode, and noise brushes so that I can
shape terrain heightmaps directly in the viewport.

### US-15.6.1.2

As a **artist (P-8)**, I want configurable brush radius, strength, and falloff curve so that I can
sculpt at any scale from fine detail to broad strokes.

### US-15.6.1.3

As a **artist (P-8)**, I want shape masks on sculpting brushes so that I can create patterned
terrain features (footprints, tire tracks).

### US-15.6.1.4

As a **artist (P-8)**, I want incremental streaming to disk during sculpting so that I can shape
massive heightmaps without loading them entirely into memory.

### US-15.6.1.5

As a **designer (P-5)**, I want sculpting operations integrated with undo/redo so that I can revert
terrain changes incrementally.

### US-15.6.1.6

As an **engine tester (P-27)**, I want to verify sculpting a 16k x 16k heightmap stays under 512 MB
peak memory so that streaming sculpt performance is regression-tested.

---

## US-15.6.2 Terrain Erosion

### US-15.6.2.1

As a **artist (P-8)**, I want hydraulic erosion simulation on selected terrain regions so that I can
create natural-looking valleys and drainage patterns.

### US-15.6.2.2

As a **artist (P-8)**, I want thermal erosion for cliff weathering so that I can add realistic
sediment deposits and talus slopes.

### US-15.6.2.3

As a **artist (P-8)**, I want configurable erosion parameters so that I can control rain amount,
sediment capacity, and thermal angle.

### US-15.6.2.4

As a **artist (P-8)**, I want real-time erosion preview via GPU compute so that I can see results
interactively before committing.

### US-15.6.2.5

As an **engine tester (P-27)**, I want to verify erosion preview runs above 15 FPS on 2048x2048
regions so that GPU erosion performance is regression-tested.

---

## US-15.6.3 Terrain Material Painting

### US-15.6.3.1

As a **artist (P-8)**, I want to paint material layers onto terrain tiles so that I can texture
landscapes with grass, rock, dirt, and snow.

### US-15.6.3.2

As a **artist (P-8)**, I want per-layer weight maps for precise blending so that I can control
exactly where each material appears.

### US-15.6.3.3

As a **artist (P-8)**, I want height-based and slope-based auto-painting rules so that rock
auto-applies to steep slopes and snow to high elevations.

### US-15.6.3.4

As a **artist (P-8)**, I want triplanar projection for cliff faces so that vertical surfaces receive
properly oriented textures.

### US-15.6.3.5

As a **artist (P-8)**, I want macro-variation textures to break tiling so that large terrain
expanses avoid visible repetition patterns.

### US-15.6.3.6

As an **engine tester (P-27)**, I want to verify weight maps sum to 1.0 at every texel so that
material blending correctness is regression-tested.

---

## US-15.6.4 Water Body Placement

### US-15.6.4.1

As a **designer (P-5)**, I want to place rivers using boundary splines so that I can define water
paths with precise control.

### US-15.6.4.2

As a **designer (P-5)**, I want configurable river width, depth, and flow speed so that each river
segment has distinct visual and gameplay characteristics.

### US-15.6.4.3

As a **designer (P-5)**, I want lakes that fill to a specified altitude so that standing water
volumes conform to terrain automatically.

### US-15.6.4.4

As a **artist (P-8)**, I want automatic shoreline masking for lakes so that water edges blend
naturally with terrain materials.

### US-15.6.4.5

As a **artist (P-8)**, I want water bodies to integrate with reflections, refraction, caustics, and
foam rendering so that water looks realistic without manual shader setup.

### US-15.6.4.6

As an **engine tester (P-27)**, I want to verify lake surface height matches the specified altitude
so that water placement precision is regression-tested.

---

## US-15.6.5 Vegetation Painting with Density Rules

### US-15.6.5.1

As a **artist (P-8)**, I want density brushes for painting vegetation instances so that I can
populate terrain with per-species control.

### US-15.6.5.2

As a **artist (P-8)**, I want per-species slope limits and altitude bands so that vegetation
respects ecological distribution rules.

### US-15.6.5.3

As a **artist (P-8)**, I want proximity exclusion radii between species so that trees do not overlap
or crowd unnaturally.

### US-15.6.5.4

As a **artist (P-8)**, I want clustering behavior for grouped vegetation so that forest patches form
naturally rather than uniformly.

### US-15.6.5.5

As a **designer (P-5)**, I want a biome rule system for declarative vegetation so that large regions
auto-populate with ecologically consistent flora.

### US-15.6.5.6

As a **designer (P-5)**, I want hand-refinement after auto-population so that auto-generated
vegetation can be adjusted per-instance.

### US-15.6.5.7

As an **engine tester (P-27)**, I want to verify no instances violate placement rules after
auto-population so that biome rule enforcement is regression-tested.

---

## US-15.6.6 Lighting Setup (Light Probes and Reflection Probes)

### US-15.6.6.1

As a **artist (P-8)**, I want to place light probes on tetrahedral grids or manually so that I can
control indirect lighting sample distribution.

### US-15.6.6.2

As a **artist (P-8)**, I want reflection probes with configurable capture volumes so that
reflections match the local environment accurately.

### US-15.6.6.3

As a **artist (P-8)**, I want blend distances on reflection probes so that transitions between probe
volumes are smooth.

### US-15.6.6.4

As a **artist (P-8)**, I want visualization overlays showing probe influence regions so that I can
verify coverage and identify gaps.

### US-15.6.6.5

As a **artist (P-8)**, I want baking and real-time update support for probes so that I can choose
quality vs. performance per probe.

### US-15.6.6.6

As an **engine tester (P-27)**, I want to verify probe influence regions render correctly in the
overlay so that visualization accuracy is regression-tested.

---

## US-15.6.7 Navmesh Preview

### US-15.6.7.1

As a **designer (P-5)**, I want a translucent navmesh overlay in the viewport so that I can see
walkable areas while editing the world.

### US-15.6.7.2

As a **designer (P-5)**, I want color-coded walkable areas and slope limits so that I can identify
non-walkable zones visually.

### US-15.6.7.3

As a **designer (P-5)**, I want real-time navmesh regeneration for selected regions so that I can
verify pathing after terrain edits immediately.

### US-15.6.7.4

As a **designer (P-5)**, I want pathfinding test markers for start and goal so that I can test AI
paths directly in the viewport.

### US-15.6.7.5

As an **engine tester (P-27)**, I want to verify the navmesh overlay marks steep slopes as
non-walkable so that slope limit visualization is regression-tested.

---

## US-15.6.8 World Partition Visualization

### US-15.6.8.1

As a **designer (P-5)**, I want a 2D minimap showing world partition cell boundaries so that I can
see the full world grid layout at a glance.

### US-15.6.8.2

As a **designer (P-5)**, I want 3D viewport overlay of streaming states so that I can see which
cells are loaded, pending, or unloaded.

### US-15.6.8.3

As a **designer (P-5)**, I want cell ownership display for multiplayer editing so that I can see
which team member is editing which zone.

### US-15.6.8.4

As a **designer (P-5)**, I want cells exceeding entity or triangle budgets flagged so that I can
identify and fix over-budget zones.

### US-15.6.8.5

As a **creative director (P-2)**, I want LOD streaming distance visualization so that I can verify
draw distance settings across the world.

### US-15.6.8.6

As an **engine tester (P-27)**, I want to verify over-budget cells display a warning indicator so
that budget violation visualization is regression-tested.
