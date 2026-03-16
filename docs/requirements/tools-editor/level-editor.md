# R-15.2 -- Level Editor Requirements

## Entity Placement

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.2.1 | The editor **SHALL** support drag-and-drop entity placement from the asset browser with grid, surface, and vertex snapping modes, where surface snapping projects entities onto terrain normals and vertex snapping aligns to the nearest mesh vertex. | [F-15.2.1](../../features/tools-editor/level-editor.md) | Fast, precise placement is fundamental to level design; multiple snap modes accommodate different alignment needs. | Unit test: place an entity at (1.3, 0, 2.7) with grid snap 1.0 and verify it snaps to (1, 0, 3). Raycast onto sloped terrain and verify surface normal alignment. |

## Prefab System

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.2.2 | The editor **SHALL** support reusable entity hierarchies as prefab assets with arbitrary nesting depth, where changes to a parent prefab propagate to all instances unless overridden. | [F-15.2.2](../../features/tools-editor/level-editor.md) | Compositional design (villages from houses from furniture) requires nested prefabs with automatic propagation. | Integration test: create a 3-level nested prefab, instantiate it, modify the innermost prefab, and verify all instances reflect the change. |
| R-15.2.3 | The editor **SHALL** allow per-instance property overrides on prefab instances with visual indicators for overridden properties, per-property revert to source, and apply-back to source prefab. | [F-15.2.3](../../features/tools-editor/level-editor.md) | Localized variation of shared prefabs is essential for world diversity without breaking the prefab link. | Unit test: set an override, verify the value differs. Revert, verify it matches source. Apply to source, verify all instances receive the new value. |

## Brush and CSG

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.2.4 | The editor **SHALL** provide additive and subtractive CSG primitives (box, cylinder, sphere, stairs, arch) with boolean operations producing watertight meshes, and conversion of brush geometry to static mesh assets. | [F-15.2.4](../../features/tools-editor/level-editor.md) | Rapid blockout is critical for level prototyping; conversion to static mesh supports the blockout-to-art workflow. | Unit test: combine two boxes with additive boolean, verify watertight mesh output. Subtract a cylinder from a box, verify hole geometry. |

## Splines

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.2.5 | The editor **SHALL** support Bezier and Catmull-Rom spline editing with tangent handles, per-point width and roll parameters, automatic C1 smoothing, and entity distribution along splines with configurable spacing and randomization. | [F-15.2.5](../../features/tools-editor/level-editor.md) | Roads, rivers, fences, and rail paths require smooth parametric curves with entity placement. | Unit test: create a Bezier spline and verify C1 continuity at control points. Distribute 10 entities along a spline and verify spacing. |

## Landscape Painting

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.2.6 | The editor **SHALL** support terrain material layer painting with configurable brush shapes, sizes, and falloff curves, height-based and slope-based auto-painting rules, and real-time preview at full material quality. | [F-15.2.6](../../features/tools-editor/level-editor.md) | Terrain texturing must be visual and immediate; auto-paint rules accelerate large-world coverage. | Unit test: apply a slope-based rule to test terrain and verify correct layer assignment. Verify weight maps sum to 1.0 per texel. |

## Foliage

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.2.7 | The editor **SHALL** support foliage painting with density brushes, per-foliage-type slope and altitude limits, exclusion zones, random scale and rotation variation, and spatial grid storage for streaming. | [F-15.2.7](../../features/tools-editor/level-editor.md) | Interactive vegetation placement with ecological rules produces natural-looking environments efficiently. | Unit test: paint foliage with a 30-degree slope limit and verify none appear on steeper slopes. Add an exclusion zone and verify no foliage inside. |

---

## User Story Traceability

User stories for this domain are maintained in
[user-stories/tools-editor/level-editor.md](../../user-stories/tools-editor/level-editor.md).
Requirements in this document are derived from those user stories.
