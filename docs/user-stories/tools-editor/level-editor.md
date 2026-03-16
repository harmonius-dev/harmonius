# User Stories: Level Editor

## F-15.2.1 Entity Placement and Snapping

## US-15.2.1.1 Designer Drags Entity from Browser

**As a** designer (P-5), **I want** to drag-and-drop entities from the asset browser into the world
with grid snapping at configurable cell sizes, **so that** I can populate levels quickly with
precisely aligned objects.

## US-15.2.1.2 Artist Snaps to Surface

**As an** artist (P-8), **I want** surface snapping that projects entities onto terrain or meshes
below, **so that** props and foliage sit naturally on uneven ground without manual height
adjustment.

## US-15.2.1.3 Tech Artist Snaps to Vertex

**As a** tech artist (P-13), **I want** vertex snapping for precise alignment to geometry edges and
corners, **so that** modular kit pieces connect seamlessly without visible gaps or overlaps.

## US-15.2.1.4 Developer Verifies Undo Integration

**As a** developer (P-15), **I want** all placement operations to respect undo/redo and
multi-selection, **so that** I can revert batch placements and programmatic entity spawns as single
operations.

## F-15.2.2 Prefab System with Nested Prefabs

## US-15.2.2.1 Designer Creates Prefab Hierarchy

**As a** designer (P-5), **I want** to define reusable entity hierarchies as prefab assets with
nesting (e.g., a "village" prefab built from "house" prefabs containing "furniture" prefabs),
**so that** I can compose complex structures from modular building blocks.

## US-15.2.2.2 Artist Propagates Prefab Changes

**As an** artist (P-8), **I want** changes to a parent prefab to propagate to all instances across
the world unless overridden, **so that** updating a single source asset automatically refreshes
thousands of placed instances.

## US-15.2.2.3 Creative Director Reviews Prefab Library

**As a** creative director (P-2), **I want** to browse and preview all prefab assets in a searchable
library, **so that** I can review the building blocks available to the level design team and ensure
visual consistency.

## US-15.2.2.4 Engine Dev Tests Nested Propagation

**As an** engine developer (P-26), **I want** nested prefab propagation to update all levels of the
hierarchy correctly when any ancestor prefab changes, **so that** deeply nested prefab structures
remain consistent.

## F-15.2.3 Prefab Instance Overrides

## US-15.2.3.1 Designer Overrides Instance Property

**As a** designer (P-5), **I want** to override any property of a prefab instance while preserving
the link to the original prefab, **so that** I can create localized variations (different door
colors, enemy health values) without breaking the prefab connection.

## US-15.2.3.2 Artist Reverts Per-Property Override

**As an** artist (P-8), **I want** to revert overrides per-property or apply them back to the source
prefab, **so that** I can experiment with variations and either keep them local or share them with
all instances.

## US-15.2.3.3 Artist Sees Override Visualization

**As an** artist (P-8), **I want** overridden properties shown as bold or colored labels in the
inspector, **so that** I can immediately see which values differ from the source prefab.

## US-15.2.3.4 Engine Tester Validates Override Persistence

**As an** engine tester (P-27), **I want** to verify that overrides persist through save/load
cycles, prefab updates, and engine upgrades, **so that** per-instance customizations are never
silently lost.

## F-15.2.4 Brush and CSG Tools

## US-15.2.4.1 Designer Blockouts with CSG Primitives

**As a** designer (P-5), **I want** additive and subtractive CSG primitives (box, cylinder, sphere,
stairs, arch) for rapid blockout, **so that** I can prototype level layouts in minutes without
waiting for final art assets.

## US-15.2.4.2 Artist Converts Brushes to Static Mesh

**As an** artist (P-8), **I want** to convert finalized brush geometry to static meshes, **so that**
I can transition from blockout to production art without rebuilding level geometry from scratch.

## US-15.2.4.3 Developer Uses Boolean Operations

**As a** developer (P-15), **I want** boolean operations between brushes to produce editable meshes,
**so that** I can create complex architectural forms by combining simple primitives.

## US-15.2.4.4 Engine Tester Validates CSG Precision

**As an** engine tester (P-27), **I want** to verify that CSG boolean operations produce watertight
meshes without degenerate triangles, **so that** blockout geometry is suitable for collision and
rendering without manual cleanup.

## F-15.2.5 Spline Editing

## US-15.2.5.1 Designer Edits Road Spline

**As a** designer (P-5), **I want** to edit Bezier and Catmull-Rom splines directly in the viewport
for roads, rivers, and rail paths, **so that** I can define natural-looking pathways with intuitive
tangent handle controls.

## US-15.2.5.2 Artist Distributes Meshes Along Spline

**As an** artist (P-8), **I want** meshes and entities distributed along splines with configurable
spacing and randomization rules, **so that** I can create fences, power lines, and vegetation strips
with a single spline edit.

## US-15.2.5.3 Tech Artist Adjusts Per-Point Parameters

**As a** tech artist (P-13), **I want** per-point width, roll, and custom parameters on spline
control points, **so that** roads can vary in width and rivers can vary in depth along their path.

## US-15.2.5.4 Engine Dev Validates Spline Math

**As an** engine developer (P-26), **I want** spline interpolation to produce smooth, continuous
curves with correct tangent computation, **so that** objects placed along splines do not exhibit
visual discontinuities.

## F-15.2.6 Landscape Painting

## US-15.2.6.1 Artist Paints Terrain Materials

**As an** artist (P-8), **I want** to paint terrain material layers directly in the viewport using
configurable brush shapes, sizes, and falloff curves, **so that** I can blend grass, rock, dirt, and
sand textures intuitively.

## US-15.2.6.2 Designer Uses Auto-Paint Rules

**As a** designer (P-5), **I want** height-based and slope-based auto-painting rules, **so that**
large terrain areas receive sensible material distribution automatically before I hand-refine the
details.

## US-15.2.6.3 Tech Artist Validates Layer Limits

**As a** tech artist (P-13), **I want** the painter to enforce the per-tile material layer limit
with a visual warning, **so that** I never exceed the shader's maximum layer count and cause
rendering artifacts.

## US-15.2.6.4 Creative Director Previews Full Quality

**As a** creative director (P-2), **I want** terrain painting to preview at full material quality in
real time, **so that** I can evaluate the final look during art reviews without needing a separate
build step.

## F-15.2.7 Foliage Painting

## US-15.2.7.1 Artist Paints Foliage with Density Brush

**As an** artist (P-8), **I want** to paint foliage instances (grass, bushes, trees, rocks) onto
surfaces with density brushes, **so that** I can populate environments naturally with configurable
distribution.

## US-15.2.7.2 Designer Defines Placement Rules

**As a** designer (P-5), **I want** per-foliage-type rules for slope limits, altitude ranges,
exclusion zones, and random scale/rotation variation, **so that** foliage placement respects
environmental constraints automatically.

## US-15.2.7.3 Tech Artist Previews LOD Transitions

**As a** tech artist (P-13), **I want** the foliage painter to preview LOD transitions in real time,
**so that** I can verify that distant foliage degrades gracefully without visible popping.

## US-15.2.7.4 Engine Tester Validates Streaming Storage

**As an** engine tester (P-27), **I want** painted foliage instances stored in spatial grids for
efficient streaming, **so that** loading and unloading foliage across world cells does not cause
frame rate drops.
