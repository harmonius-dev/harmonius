# User Stories: Level Editor

## F-15.2.1 Entity Placement and Snapping

| ID          | Persona            | Features | Requirements |
|-------------|--------------------|----------|--------------|
| US-15.2.1.1 | designer (P-5)     |          |              |
| US-15.2.1.2 | artist (P-8)       |          |              |
| US-15.2.1.3 | tech artist (P-13) |          |              |
| US-15.2.1.4 | developer (P-15)   |          |              |

1. **US-15.2.1.1** — to drag-and-drop entities from the asset browser into the world with grid
   snapping at configurable cell sizes
   - **Acceptance:** I can populate levels quickly with precisely aligned objects
2. **US-15.2.1.2** — surface snapping that projects entities onto terrain or meshes below
   - **Acceptance:** props and foliage sit naturally on uneven ground without manual height
     adjustment
3. **US-15.2.1.3** — vertex snapping for precise alignment to geometry edges and corners
   - **Acceptance:** modular kit pieces connect seamlessly without visible gaps or overlaps
4. **US-15.2.1.4** — all placement operations to respect undo/redo and multi-selection
   - **Acceptance:** I can revert batch placements and programmatic entity spawns as single
     operations

## F-15.2.2 Prefab System with Nested Prefabs

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-15.2.2.1 | designer (P-5)          |          |              |
| US-15.2.2.2 | artist (P-8)            |          |              |
| US-15.2.2.3 | creative director (P-2) |          |              |
| US-15.2.2.4 | engine developer (P-26) |          |              |

1. **US-15.2.2.1** — to define reusable entity hierarchies as prefab assets with nesting (e.g., a
   "village" prefab built from "house" prefabs containing "furniture" prefabs)
   - **Acceptance:** I can compose complex structures from modular building blocks
2. **US-15.2.2.2** — changes to a parent prefab to propagate to all instances across the world
   unless overridden
   - **Acceptance:** updating a single source asset automatically refreshes thousands of placed
     instances
3. **US-15.2.2.3** — to browse and preview all prefab assets in a searchable library
   - **Acceptance:** I can review the building blocks available to the level design team and ensure
     visual consistency
4. **US-15.2.2.4** — nested prefab propagation to update all levels of the hierarchy correctly when
   any ancestor prefab changes
   - **Acceptance:** deeply nested prefab structures remain consistent

## F-15.2.3 Prefab Instance Overrides

| ID          | Persona              | Features | Requirements |
|-------------|----------------------|----------|--------------|
| US-15.2.3.1 | designer (P-5)       |          |              |
| US-15.2.3.2 | artist (P-8)         |          |              |
| US-15.2.3.3 | artist (P-8)         |          |              |
| US-15.2.3.4 | engine tester (P-27) |          |              |

1. **US-15.2.3.1** — to override any property of a prefab instance while preserving the link to the
   original prefab
   - **Acceptance:** I can create localized variations (different door colors, enemy health values)
     without breaking the prefab connection
2. **US-15.2.3.2** — to revert overrides per-property or apply them back to the source prefab
   - **Acceptance:** I can experiment with variations and either keep them local or share them with
     all instances
3. **US-15.2.3.3** — overridden properties shown as bold or colored labels in the inspector
   - **Acceptance:** I can immediately see which values differ from the source prefab
4. **US-15.2.3.4** — to verify that overrides persist through save/load cycles, prefab updates, and
   engine upgrades
   - **Acceptance:** per-instance customizations are never silently lost

## F-15.2.4 Brush and CSG Tools

| ID          | Persona              | Features | Requirements |
|-------------|----------------------|----------|--------------|
| US-15.2.4.1 | designer (P-5)       |          |              |
| US-15.2.4.2 | artist (P-8)         |          |              |
| US-15.2.4.3 | developer (P-15)     |          |              |
| US-15.2.4.4 | engine tester (P-27) |          |              |

1. **US-15.2.4.1** — additive and subtractive CSG primitives (box, cylinder, sphere, stairs, arch)
   for rapid blockout
   - **Acceptance:** I can prototype level layouts in minutes without waiting for final art assets
2. **US-15.2.4.2** — to convert finalized brush geometry to static meshes
   - **Acceptance:** I can transition from blockout to production art without rebuilding level
     geometry from scratch
3. **US-15.2.4.3** — boolean operations between brushes to produce editable meshes
   - **Acceptance:** I can create complex architectural forms by combining simple primitives
4. **US-15.2.4.4** — to verify that CSG boolean operations produce watertight meshes without
   degenerate triangles
   - **Acceptance:** blockout geometry is suitable for collision and rendering without manual
     cleanup

## F-15.2.5 Spline Editing

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-15.2.5.1 | designer (P-5)          |          |              |
| US-15.2.5.2 | artist (P-8)            |          |              |
| US-15.2.5.3 | tech artist (P-13)      |          |              |
| US-15.2.5.4 | engine developer (P-26) |          |              |

1. **US-15.2.5.1** — to edit Bezier and Catmull-Rom splines directly in the viewport for roads,
   rivers, and rail paths
   - **Acceptance:** I can define natural-looking pathways with intuitive tangent handle controls
2. **US-15.2.5.2** — meshes and entities distributed along splines with configurable spacing and
   randomization rules
   - **Acceptance:** I can create fences, power lines, and vegetation strips with a single spline
     edit
3. **US-15.2.5.3** — per-point width, roll, and custom parameters on spline control points
   - **Acceptance:** roads can vary in width and rivers can vary in depth along their path
4. **US-15.2.5.4** — spline interpolation to produce smooth, continuous curves with correct tangent
   computation
   - **Acceptance:** objects placed along splines do not exhibit visual discontinuities

## F-15.2.6 Landscape Painting

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-15.2.6.1 | artist (P-8)            |          |              |
| US-15.2.6.2 | designer (P-5)          |          |              |
| US-15.2.6.3 | tech artist (P-13)      |          |              |
| US-15.2.6.4 | creative director (P-2) |          |              |

1. **US-15.2.6.1** — to paint terrain material layers directly in the viewport using configurable
   brush shapes, sizes, and falloff curves
   - **Acceptance:** I can blend grass, rock, dirt, and sand textures intuitively
2. **US-15.2.6.2** — height-based and slope-based auto-painting rules
   - **Acceptance:** large terrain areas receive sensible material distribution automatically before
     I hand-refine the details
3. **US-15.2.6.3** — the painter to enforce the per-tile material layer limit with a visual warning
   - **Acceptance:** I never exceed the shader's maximum layer count and cause rendering artifacts
4. **US-15.2.6.4** — terrain painting to preview at full material quality in real time
   - **Acceptance:** I can evaluate the final look during art reviews without needing a separate
     build step

## F-15.2.7 Foliage Painting

| ID          | Persona              | Features | Requirements |
|-------------|----------------------|----------|--------------|
| US-15.2.7.1 | artist (P-8)         |          |              |
| US-15.2.7.2 | designer (P-5)       |          |              |
| US-15.2.7.3 | tech artist (P-13)   |          |              |
| US-15.2.7.4 | engine tester (P-27) |          |              |

1. **US-15.2.7.1** — to paint foliage instances (grass, bushes, trees, rocks) onto surfaces with
   density brushes
   - **Acceptance:** I can populate environments naturally with configurable distribution
2. **US-15.2.7.2** — per-foliage-type rules for slope limits, altitude ranges, exclusion zones, and
   random scale/rotation variation
   - **Acceptance:** foliage placement respects environmental constraints automatically
3. **US-15.2.7.3** — the foliage painter to preview LOD transitions in real time
   - **Acceptance:** I can verify that distant foliage degrades gracefully without visible popping
4. **US-15.2.7.4** — painted foliage instances stored in spatial grids for efficient streaming
   - **Acceptance:** loading and unloading foliage across world cells does not cause frame rate
     drops
