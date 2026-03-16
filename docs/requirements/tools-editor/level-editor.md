# R-15.2 -- Level Editor Requirements

## Entity Placement

### R-15.2.1 Entity Placement and Snapping

The editor **SHALL** support drag-and-drop entity placement
from the asset browser with grid, surface, and vertex
snapping modes, where surface snapping projects entities
onto terrain normals and vertex snapping aligns to the
nearest mesh vertex.

- **Derived from:**
  [F-15.2.1](../../features/tools-editor/level-editor.md)
- **Rationale:** Fast, precise placement is fundamental to
  level design; multiple snap modes accommodate different
  alignment needs.
- **Verification:** Unit test: place an entity at (1.3, 0,
  2.7) with grid snap 1.0 and verify it snaps to (1, 0, 3).
  Raycast onto sloped terrain and verify surface normal
  alignment.

## Prefab System

### R-15.2.2 Prefab System with Nested Prefabs

The editor **SHALL** support reusable entity hierarchies
as prefab assets with arbitrary nesting depth, where
changes to a parent prefab propagate to all instances
unless overridden.

- **Derived from:**
  [F-15.2.2](../../features/tools-editor/level-editor.md)
- **Rationale:** Compositional design (villages from houses
  from furniture) requires nested prefabs with automatic
  propagation.
- **Verification:** Integration test: create a 3-level
  nested prefab, instantiate it, modify the innermost
  prefab, and verify all instances reflect the change.

### R-15.2.3 Prefab Instance Overrides

The editor **SHALL** allow per-instance property overrides
on prefab instances with visual indicators for overridden
properties, per-property revert to source, and apply-back
to source prefab.

- **Derived from:**
  [F-15.2.3](../../features/tools-editor/level-editor.md)
- **Rationale:** Localized variation of shared prefabs is
  essential for world diversity without breaking the
  prefab link.
- **Verification:** Unit test: set an override, verify the
  value differs. Revert, verify it matches source. Apply to
  source, verify all instances receive the new value.

## Brush and CSG

### R-15.2.4 Brush and CSG Tools

The editor **SHALL** provide additive and subtractive CSG
primitives (box, cylinder, sphere, stairs, arch) with
boolean operations producing watertight meshes, and
conversion of brush geometry to static mesh assets.

- **Derived from:**
  [F-15.2.4](../../features/tools-editor/level-editor.md)
- **Rationale:** Rapid blockout is critical for level
  prototyping; conversion to static mesh supports the
  blockout-to-art workflow.
- **Verification:** Unit test: combine two boxes with
  additive boolean, verify watertight mesh output. Subtract
  a cylinder from a box, verify hole geometry.

## Splines

### R-15.2.5 Spline Editing

The editor **SHALL** support Bezier and Catmull-Rom spline
editing with tangent handles, per-point width and roll
parameters, automatic C1 smoothing, and entity distribution
along splines with configurable spacing and randomization.

- **Derived from:**
  [F-15.2.5](../../features/tools-editor/level-editor.md)
- **Rationale:** Roads, rivers, fences, and rail paths
  require smooth parametric curves with entity placement.
- **Verification:** Unit test: create a Bezier spline and
  verify C1 continuity at control points. Distribute 10
  entities along a spline and verify spacing.

## Landscape Painting

### R-15.2.6 Landscape Material Painting

The editor **SHALL** support terrain material layer painting
with configurable brush shapes, sizes, and falloff curves,
height-based and slope-based auto-painting rules, and
real-time preview at full material quality.

- **Derived from:**
  [F-15.2.6](../../features/tools-editor/level-editor.md)
- **Rationale:** Terrain texturing must be visual and
  immediate; auto-paint rules accelerate large-world
  coverage.
- **Verification:** Unit test: apply a slope-based rule to
  test terrain and verify correct layer assignment. Verify
  weight maps sum to 1.0 per texel.

## Foliage

### R-15.2.7 Foliage Painting

The editor **SHALL** support foliage painting with density
brushes, per-foliage-type slope and altitude limits,
exclusion zones, random scale and rotation variation, and
spatial grid storage for streaming.

- **Derived from:**
  [F-15.2.7](../../features/tools-editor/level-editor.md)
- **Rationale:** Interactive vegetation placement with
  ecological rules produces natural-looking environments
  efficiently.
- **Verification:** Unit test: paint foliage with a
  30-degree slope limit and verify none appear on steeper
  slopes. Add an exclusion zone and verify no foliage
  inside.

---

## User Stories

## US-15.2.1 Entity Placement and Snapping

### US-15.2.1.1
As a **designer (P-5)**, I want to drag entities from the asset browser into the viewport so that I
can place objects without navigating menus.

### US-15.2.1.2
As a **designer (P-5)**, I want grid snapping with configurable cell size so that I can align
entities to a uniform grid for consistent layout.

### US-15.2.1.3
As a **designer (P-5)**, I want surface snapping that projects entities onto terrain so that placed
objects sit flush on uneven ground.

### US-15.2.1.4
As a **designer (P-5)**, I want vertex snapping for alignment to geometry so that I can position two
meshes at exact vertex positions.

### US-15.2.1.5
As a **designer (P-5)**, I want to duplicate existing entities with their transforms so that I can
repeat placement patterns without re-dragging from the browser.

### US-15.2.1.6
As a **artist (P-8)**, I want placement to respect undo/redo so that I can revert any accidental
placements instantly.

### US-15.2.1.7
As a **artist (P-8)**, I want multi-selection placement for batch positioning so that I can move
groups of entities while maintaining relative offsets.

### US-15.2.1.8
As an **engine tester (P-27)**, I want to verify surface snapping projects entities correctly onto
terrain normals so that placement accuracy on slopes is regression-tested.

---

## US-15.2.2 Prefab System with Nested Prefabs

### US-15.2.2.1
As a **designer (P-5)**, I want to define reusable entity hierarchies as prefab assets so that I can
instantiate consistent structures across the world.

### US-15.2.2.2
As a **designer (P-5)**, I want prefabs that contain other prefabs (nesting) so that I can compose
complex structures from simpler building blocks.

### US-15.2.2.3
As a **designer (P-5)**, I want changes to a parent prefab to propagate to all instances so that
updating a base design automatically updates every placed instance.

### US-15.2.2.4
As a **artist (P-8)**, I want to create a prefab from a selection of entities so that I can save a
finished art composition as a reusable asset.

### US-15.2.2.5
As a **developer (P-15)**, I want prefab data stored as serializable assets so that prefabs
version-control cleanly and merge predictably.

### US-15.2.2.6
As a **creative director (P-2)**, I want nested prefabs for compositional design so that a village
built from house prefabs updates when house design changes.

### US-15.2.2.7
As an **engine tester (P-27)**, I want to verify nested prefab changes propagate through all nesting
levels so that cascading updates are validated for correctness.

---

## US-15.2.3 Prefab Instance Overrides

### US-15.2.3.1
As a **designer (P-5)**, I want to override any property on a prefab instance so that I can create
localized variations while keeping the base link.

### US-15.2.3.2
As a **designer (P-5)**, I want override state visualized with bold or colored labels so that I can
see which properties differ from the source prefab.

### US-15.2.3.3
As a **designer (P-5)**, I want to revert a single overridden property to the source value so that I
can selectively undo customizations per-property.

### US-15.2.3.4
As a **designer (P-5)**, I want to apply an override back to the source prefab so that a successful
variation becomes the new default for all instances.

### US-15.2.3.5
As a **artist (P-8)**, I want per-instance material overrides on prefab meshes so that I can tint or
weather individual instances of the same building.

### US-15.2.3.6
As an **engine tester (P-27)**, I want to verify reverting an override restores the source prefab
value exactly so that override revert correctness is regression-tested.

---

## US-15.2.4 Brush and CSG Tools

### US-15.2.4.1
As a **designer (P-5)**, I want additive CSG primitives (box, cylinder, sphere, stairs) so that I
can block out level geometry rapidly during prototyping.

### US-15.2.4.2
As a **designer (P-5)**, I want subtractive CSG primitives for carving holes so that I can create
doorways and windows in existing geometry.

### US-15.2.4.3
As a **designer (P-5)**, I want boolean operations between brushes so that I can combine and
subtract shapes to produce complex forms.

### US-15.2.4.4
As a **designer (P-5)**, I want to convert brush geometry to static meshes so that finalized
blockouts transition to production art assets.

### US-15.2.4.5
As a **artist (P-8)**, I want arch and stair CSG primitives so that I can prototype architectural
elements quickly.

### US-15.2.4.6
As an **engine tester (P-27)**, I want to verify boolean operations produce watertight meshes so
that CSG output is validated for rendering and collision correctness.

---

## US-15.2.5 Spline Editing

### US-15.2.5.1
As a **designer (P-5)**, I want to create Bezier and Catmull-Rom splines in the viewport so that I
can define paths for roads, rivers, fences, and rail routes.

### US-15.2.5.2
As a **designer (P-5)**, I want to add, move, and delete spline control points so that I can adjust
path shapes interactively.

### US-15.2.5.3
As a **designer (P-5)**, I want tangent handles on control points so that I can control curve
sharpness and direction at each point.

### US-15.2.5.4
As a **designer (P-5)**, I want per-point width and roll parameters so that roads can widen at
intersections and bank through curves.

### US-15.2.5.5
As a **artist (P-8)**, I want meshes distributed along splines with spacing control so that fence
posts or power poles auto-populate along a drawn path.

### US-15.2.5.6
As a **artist (P-8)**, I want randomization rules for spline-distributed entities so that foliage
along a river path varies naturally.

### US-15.2.5.7
As an **engine tester (P-27)**, I want to verify that automatic smoothing produces C1-continuous
curves so that spline continuity is regression-tested.

---

## US-15.2.6 Landscape Painting

### US-15.2.6.1
As a **designer (P-5)**, I want to paint terrain material layers in the viewport so that I can
texture the landscape without leaving the 3D view.

### US-15.2.6.2
As a **designer (P-5)**, I want configurable brush shapes, sizes, and falloff curves so that I can
paint with precision from broad strokes to fine detail.

### US-15.2.6.3
As a **designer (P-5)**, I want height-based and slope-based auto-painting rules so that snow
accumulates on peaks and rock appears on cliffs automatically.

### US-15.2.6.4
As a **artist (P-8)**, I want real-time preview at full material quality so that I can evaluate
painted results immediately without a bake step.

### US-15.2.6.5
As a **artist (P-8)**, I want blending up to the material layer limit per tile so that I can create
rich, multi-layer terrain surfaces.

### US-15.2.6.6
As an **engine tester (P-27)**, I want to verify auto-painting rules apply correct layers based on
heightmap slope values so that rule-based painting is validated against known test terrain.

---

## US-15.2.7 Foliage Painting

### US-15.2.7.1
As a **designer (P-5)**, I want to paint foliage instances with density brushes so that I can
populate terrain with vegetation interactively.

### US-15.2.7.2
As a **designer (P-5)**, I want per-foliage-type slope and altitude limits so that trees do not
appear on cliff faces or above the treeline.

### US-15.2.7.3
As a **designer (P-5)**, I want exclusion zones that prevent foliage so that paths, buildings, and
clearings remain foliage-free.

### US-15.2.7.4
As a **artist (P-8)**, I want random scale and rotation variation per instance so that foliage looks
natural rather than uniformly placed.

### US-15.2.7.5
As a **artist (P-8)**, I want real-time LOD transition preview while painting so that I can verify
foliage appearance at all viewing distances.

### US-15.2.7.6
As a **artist (P-8)**, I want painted instances stored in spatial grids for streaming so that
foliage loads efficiently in large open worlds.

### US-15.2.7.7
As an **engine tester (P-27)**, I want to verify slope-limited foliage does not appear on surfaces
exceeding the configured angle so that placement rules are regression-tested.
