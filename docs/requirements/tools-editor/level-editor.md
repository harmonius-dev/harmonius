# R-15.2 — Level Editor Requirements

## Entity Placement

### R-15.2.1 Entity Placement and Snapping

The engine **SHALL** support entity placement via drag-and-drop from the asset browser and
duplication, with grid snapping (configurable cell size), surface snapping (projection onto
terrain or meshes), and vertex snapping (alignment to geometry vertices), all integrated with
undo/redo and multi-selection.

- **Derived from:** [F-15.2.1](../../features/tools-editor/level-editor.md)
- **Rationale:** Precise placement with multiple snap modes eliminates manual coordinate entry and
  accelerates blockout and final art placement workflows.
- **Verification:** Integration test: drag an asset from the browser into the viewport and assert
  an entity is created at the drop location. Enable grid snap at 1.0 units, place an entity, and
  verify its position is quantized to integer coordinates. Enable surface snap, place an entity
  above terrain, and verify its Y position matches the terrain height at that XZ. Undo placement
  and verify the entity is removed.

### R-15.2.2 Prefab System with Nested Prefabs

The engine **SHALL** support reusable prefab assets containing entity hierarchies with arbitrary
nesting depth, where modifications to a parent prefab propagate to all instances unless a
property has been overridden at the instance level.

- **Derived from:** [F-15.2.2](../../features/tools-editor/level-editor.md)
- **Rationale:** Compositional prefabs (e.g., village from houses from furniture) enable large-scale
  world building with consistent propagation of design changes across thousands of instances.
- **Verification:** Unit test: create a three-level nested prefab (A contains B contains C). Modify
  a property on C's source prefab and assert the change propagates to all instances of A that have
  not overridden that property. Integration test: place 100 instances of a prefab, modify the
  source, and verify all instances update within one editor frame.

### R-15.2.3 Prefab Instance Overrides

The engine **SHALL** allow prefab instances to override any property of their source prefab while
preserving the prefab link, visually indicating overridden properties in the inspector, and
supporting per-property revert and apply-back-to-source operations.

- **Derived from:** [F-15.2.3](../../features/tools-editor/level-editor.md)
- **Rationale:** Instance overrides enable localized variation (e.g., unique shop signs in a
  village) without breaking the shared prefab link needed for global updates.
- **Verification:** Unit test: instantiate a prefab, override a material property, and assert the
  override persists while non-overridden properties still track the source. Revert the override
  and assert the property returns to the source value. Apply the override back to the source and
  assert all other instances adopt the new value.

## Brush and Shape Tools

### R-15.2.4 Brush and CSG Tools

The engine **SHALL** provide additive and subtractive CSG primitives (box, cylinder, sphere,
stairs, arch) with boolean operations between brushes, and support converting brush geometry to
static mesh assets.

- **Derived from:** [F-15.2.4](../../features/tools-editor/level-editor.md)
- **Rationale:** CSG tools enable rapid blockout and prototyping without requiring external DCC
  tools, supporting the standard blockout-to-art level design workflow.
- **Verification:** Integration test: create an additive box and a subtractive cylinder, perform a
  boolean subtraction, and verify the resulting mesh has a cylindrical hole. Convert the brush to a
  static mesh and verify it renders identically. Undo the conversion and verify the brush geometry
  is restored.

### R-15.2.5 Spline Editing

The engine **SHALL** support viewport editing of Bezier and Catmull-Rom splines with tangent
handles, automatic smoothing, per-point width and roll parameters, and configurable distribution
of meshes and entities along the spline with spacing and randomization controls.

- **Derived from:** [F-15.2.5](../../features/tools-editor/level-editor.md)
- **Rationale:** Spline-based authoring is the standard workflow for roads, rivers, fences, and
  rail paths, requiring direct viewport manipulation for intuitive spatial editing.
- **Verification:** Integration test: create a Bezier spline with four control points, adjust
  tangent handles, and verify the curve passes through all control points with correct tangent
  directions. Distribute mesh instances along the spline at 2-unit spacing and verify instance
  count matches spline length divided by spacing. Enable randomization and verify instance
  transforms vary within configured bounds.

## Painting

### R-15.2.6 Landscape Painting

The engine **SHALL** provide landscape material painting through the world-building terrain painting
system (R-15.6.3). The level editor **SHALL** expose the terrain painting tools in its viewport
toolbar.

- **Derived from:** [F-15.2.6](../../features/tools-editor/level-editor.md)
- **Rationale:** Direct viewport painting with real-time preview enables artists to iterate on
  terrain materials in context without switching between external tools.
- **Verification:** Integration test: paint two material layers onto a terrain tile with a circular
  brush. Verify the blend weights are correct at the brush center (full layer A) and at the
  falloff edge (blended A+B). Verify painting a layer beyond the per-tile limit is either clamped
  or reported as an error. Verify the preview matches the final rendered result.

### R-15.2.7 Foliage Painting

The engine **SHALL** provide foliage painting through the world-building vegetation system
(R-15.6.5). The level editor **SHALL** expose the vegetation painting tools in its viewport
toolbar.

- **Derived from:** [F-15.2.7](../../features/tools-editor/level-editor.md)
- **Rationale:** Density-based foliage painting with constraint rules produces natural distribution
  patterns faster than manual placement across large open-world environments.
- **Verification:** Integration test: paint grass foliage onto a slope exceeding the configured
  slope limit and verify no instances are placed. Paint onto a valid surface and verify instance
  density matches the brush density setting. Verify instances within an exclusion zone are removed.
  Verify painted instances appear in the spatial grid and stream correctly when moving the camera.
