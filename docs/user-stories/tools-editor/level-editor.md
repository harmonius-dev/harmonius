# User Stories: Level Editor

## F-15.2.1 Entity Placement and Snapping

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.2.1.1 | designer (P-5) | to drag-and-drop entities from the asset browser into the world with grid snapping at configurable cell sizes | I can populate levels quickly with precisely aligned objects |  |  |
| US-15.2.1.2 | artist (P-8) | surface snapping that projects entities onto terrain or meshes below | props and foliage sit naturally on uneven ground without manual height adjustment |  |  |
| US-15.2.1.3 | tech artist (P-13) | vertex snapping for precise alignment to geometry edges and corners | modular kit pieces connect seamlessly without visible gaps or overlaps |  |  |
| US-15.2.1.4 | developer (P-15) | all placement operations to respect undo/redo and multi-selection | I can revert batch placements and programmatic entity spawns as single operations |  |  |

## F-15.2.2 Prefab System with Nested Prefabs

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.2.2.1 | designer (P-5) | to define reusable entity hierarchies as prefab assets with nesting (e.g., a "village" prefab built from "house" prefabs containing "furniture" prefabs) | I can compose complex structures from modular building blocks |  |  |
| US-15.2.2.2 | artist (P-8) | changes to a parent prefab to propagate to all instances across the world unless overridden | updating a single source asset automatically refreshes thousands of placed instances |  |  |
| US-15.2.2.3 | creative director (P-2) | to browse and preview all prefab assets in a searchable library | I can review the building blocks available to the level design team and ensure visual consistency |  |  |
| US-15.2.2.4 | engine developer (P-26) | nested prefab propagation to update all levels of the hierarchy correctly when any ancestor prefab changes | deeply nested prefab structures remain consistent |  |  |

## F-15.2.3 Prefab Instance Overrides

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.2.3.1 | designer (P-5) | to override any property of a prefab instance while preserving the link to the original prefab | I can create localized variations (different door colors, enemy health values) without breaking the prefab connection |  |  |
| US-15.2.3.2 | artist (P-8) | to revert overrides per-property or apply them back to the source prefab | I can experiment with variations and either keep them local or share them with all instances |  |  |
| US-15.2.3.3 | artist (P-8) | overridden properties shown as bold or colored labels in the inspector | I can immediately see which values differ from the source prefab |  |  |
| US-15.2.3.4 | engine tester (P-27) | to verify that overrides persist through save/load cycles, prefab updates, and engine upgrades | per-instance customizations are never silently lost |  |  |

## F-15.2.4 Brush and CSG Tools

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.2.4.1 | designer (P-5) | additive and subtractive CSG primitives (box, cylinder, sphere, stairs, arch) for rapid blockout | I can prototype level layouts in minutes without waiting for final art assets |  |  |
| US-15.2.4.2 | artist (P-8) | to convert finalized brush geometry to static meshes | I can transition from blockout to production art without rebuilding level geometry from scratch |  |  |
| US-15.2.4.3 | developer (P-15) | boolean operations between brushes to produce editable meshes | I can create complex architectural forms by combining simple primitives |  |  |
| US-15.2.4.4 | engine tester (P-27) | to verify that CSG boolean operations produce watertight meshes without degenerate triangles | blockout geometry is suitable for collision and rendering without manual cleanup |  |  |

## F-15.2.5 Spline Editing

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.2.5.1 | designer (P-5) | to edit Bezier and Catmull-Rom splines directly in the viewport for roads, rivers, and rail paths | I can define natural-looking pathways with intuitive tangent handle controls |  |  |
| US-15.2.5.2 | artist (P-8) | meshes and entities distributed along splines with configurable spacing and randomization rules | I can create fences, power lines, and vegetation strips with a single spline edit |  |  |
| US-15.2.5.3 | tech artist (P-13) | per-point width, roll, and custom parameters on spline control points | roads can vary in width and rivers can vary in depth along their path |  |  |
| US-15.2.5.4 | engine developer (P-26) | spline interpolation to produce smooth, continuous curves with correct tangent computation | objects placed along splines do not exhibit visual discontinuities |  |  |

## F-15.2.6 Landscape Painting

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.2.6.1 | artist (P-8) | to paint terrain material layers directly in the viewport using configurable brush shapes, sizes, and falloff curves | I can blend grass, rock, dirt, and sand textures intuitively |  |  |
| US-15.2.6.2 | designer (P-5) | height-based and slope-based auto-painting rules | large terrain areas receive sensible material distribution automatically before I hand-refine the details |  |  |
| US-15.2.6.3 | tech artist (P-13) | the painter to enforce the per-tile material layer limit with a visual warning | I never exceed the shader's maximum layer count and cause rendering artifacts |  |  |
| US-15.2.6.4 | creative director (P-2) | terrain painting to preview at full material quality in real time | I can evaluate the final look during art reviews without needing a separate build step |  |  |

## F-15.2.7 Foliage Painting

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.2.7.1 | artist (P-8) | to paint foliage instances (grass, bushes, trees, rocks) onto surfaces with density brushes | I can populate environments naturally with configurable distribution |  |  |
| US-15.2.7.2 | designer (P-5) | per-foliage-type rules for slope limits, altitude ranges, exclusion zones, and random scale/rotation variation | foliage placement respects environmental constraints automatically |  |  |
| US-15.2.7.3 | tech artist (P-13) | the foliage painter to preview LOD transitions in real time | I can verify that distant foliage degrades gracefully without visible popping |  |  |
| US-15.2.7.4 | engine tester (P-27) | painted foliage instances stored in spatial grids for efficient streaming | loading and unloading foliage across world cells does not cause frame rate drops |  |  |
