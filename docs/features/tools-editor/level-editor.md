# 15.2 — Level Editor

## Entity Placement

### F-15.2.1 Entity Placement and Snapping

Places entities into the world via drag-and-drop from the asset browser or duplication of
existing entities. Supports grid snapping with configurable cell size, surface snapping that
projects entities onto terrain or meshes below, and vertex snapping for precise alignment to
geometry. Placement respects undo/redo and multi-selection.

- **Requirements:** R-15.2.1
- **Dependencies:** F-15.1.3, F-15.1.4, F-7.1.1
- **Platform notes:** None

### F-15.2.2 Prefab System with Nested Prefabs

Defines reusable entity hierarchies as prefab assets. Prefabs can contain other prefabs
(nesting) to enable compositional design such as a "village" prefab built from "house" prefabs
that themselves reference "furniture" prefabs. Changes to a parent prefab propagate to all
instances across the world unless overridden.

- **Requirements:** R-15.2.2
- **Dependencies:** F-15.2.1, F-7.4.1
- **Platform notes:** None

### F-15.2.3 Prefab Instance Overrides

Allows individual prefab instances to override any property of their source prefab while
preserving the link to the original. Override state is visualized in the inspector with
bold/colored labels, and overrides can be reverted per-property or applied back to the source
prefab. Critical for MMO worlds where thousands of instances share a base but need localized
variation.

- **Requirements:** R-15.2.3
- **Dependencies:** F-15.2.2
- **Platform notes:** None

## Brush and Shape Tools

### F-15.2.4 Brush and CSG Tools

Provides additive and subtractive CSG primitives (box, cylinder, sphere, stairs, arch) for
rapid blockout and prototyping. Boolean operations between brushes produce editable meshes.
Brush geometry can be converted to static meshes once the layout is finalized, supporting the
common blockout-to-art workflow used by level design teams.

- **Requirements:** R-15.2.4
- **Dependencies:** F-15.1.3, F-3.1.1
- **Platform notes:** None

### F-15.2.5 Spline Editing

Edits Bezier and Catmull-Rom splines directly in the viewport for roads, rivers, fences,
power lines, and rail paths. Control points support tangent handles, automatic smoothing, and
per-point width/roll parameters. Meshes and entities can be distributed along splines with
configurable spacing and randomization rules.

- **Requirements:** R-15.2.5
- **Dependencies:** F-15.1.3, F-15.1.5
- **Platform notes:** None

## Painting

### F-15.2.6 Landscape Painting

Paints terrain material layers directly in the viewport using configurable brush shapes, sizes,
and falloff curves. Supports blending up to the material layer limit per tile, with height-based
and slope-based auto-painting rules. Preview shows the painted result in real time at full
material quality.

- **Requirements:** R-15.2.6
- **Dependencies:** F-15.6.3, F-3.2.1
- **Platform notes:** None

### F-15.2.7 Foliage Painting

Paints foliage instances (grass, bushes, trees, rocks) onto surfaces with density brushes.
Supports per-foliage-type rules for slope limits, altitude ranges, exclusion zones, and random
scale/rotation variation. Painted instances are stored in spatial grids for efficient streaming,
and the painter previews LOD transitions in real time.

- **Requirements:** R-15.2.7
- **Dependencies:** F-3.3.1, F-15.2.1
- **Platform notes:** None
