# R-15.2 -- Level Editor Requirements

## Requirements

1. **R-15.2.1** — The engine **SHALL** support entity placement via drag-and-drop with grid,
   surface, and vertex snapping, integrated with undo/redo and multi-selection.
   - **Rationale:** Fast, precise placement is the most frequent level design operation.
   - **Verification:** Drag an entity onto terrain with surface snap enabled and verify it lands on
     the surface.

2. **R-15.2.2** — The engine **SHALL** support Entity Template hierarchies with nesting, where
   changes to a parent template propagate to all instances unless overridden.
   - **Rationale:** Compositional templates reduce duplication across large worlds.
   - **Verification:** Modify a parent template property and verify all non-overridden instances
     update.

3. **R-15.2.3** — The engine **SHALL** allow per-property overrides on Entity Template instances
   while preserving the template link, with visual indicators in the inspector.
   - **Rationale:** Instance variation is essential for open-world levels with thousands of shared
     templates.
   - **Verification:** Override a property, verify the inspector marks it bold, then revert and
     confirm the original value restores.

4. **R-15.2.4** — The engine **SHALL** provide additive and subtractive CSG primitives with boolean
   operations and conversion to static meshes.
   - **Rationale:** CSG brushes accelerate blockout and prototyping before final art is available.
   - **Verification:** Create additive and subtractive brushes, perform a boolean operation, and
     convert the result to a static mesh.

5. **R-15.2.5** — The engine **SHALL** provide Bezier and Catmull-Rom spline editing with tangent
   handles, per-point width and roll, and mesh/entity distribution along splines.
   - **Rationale:** Splines define roads, rivers, fences, and rail paths in open-world environments.
   - **Verification:** Create a spline, distribute meshes along it with 2 m spacing, and verify
     uniform placement.

6. **R-15.2.6** — The engine **SHALL** support terrain material layer painting with configurable
   brush shape, size, falloff, and auto-painting rules based on height and slope.
   - **Rationale:** Material painting controls visual quality of terrain surfaces.
   - **Verification:** Paint a material layer on a slope, enable slope-based auto-paint, and verify
     correct layer assignment.

7. **R-15.2.7** — The engine **SHALL** support foliage instance painting with density brushes and
   per-type rules for slope, altitude, exclusion zones, and random scale/rotation.
   - **Rationale:** Procedural placement rules produce organic vegetation distributions efficiently.
   - **Verification:** Paint foliage with a slope limit of 30 degrees and verify no instances appear
     on steeper surfaces.

8. **R-15.2.8** — The engine **SHALL** support entity duplication from the current selection with
   offset placement and undo support.
   - **Rationale:** Duplication is one of the most frequent level editing operations and must be
     fast and reversible.
   - **Verification:** Select three entities, duplicate them, verify offset placement, undo, and
     confirm the duplicates are removed.

9. **R-15.2.9** — The engine **SHALL** allow creating a reusable entity template from a selection of
   existing entities in the viewport.
   - **Rationale:** Templates from selections enable rapid pattern reuse without pre-planning the
     template hierarchy.
   - **Verification:** Select a group of entities, create a template, stamp it elsewhere, and verify
     it matches the original arrangement.

10. **R-15.2.10** — The engine **SHALL** support 2D entity placement with XY-plane snapping and
    Z-order, tilemap editing with auto-tiling rules, and isometric/hex grid placement modes.
    - **Rationale:** 2D and 2.5D games require dedicated placement modes that differ from 3D
      workflows.
    - **Verification:** Place tiles on an isometric grid with auto-tiling and verify correct tile
      selection at edges and corners.
