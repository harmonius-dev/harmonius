# Editor ↔ Physics Integration Design

## Systems Involved

| System | Design | Domain |
|--------|--------|--------|
| Editor Core | [editor-core.md](../tools/editor-core.md) | Tools |
| Visual Editors | [visual-editors.md](../tools/visual-editors.md) | Tools |
| Physics Foundation | [foundation.md](../physics/foundation.md) | Physics |

## Integration Requirements

| ID | Requirement | Systems |
|----|-------------|---------|
| IR-5.4.1 | Collider shape editing with gizmos | Editor, Physics |
| IR-5.4.2 | Physics debug visualization in viewport | Editor, Physics |
| IR-5.4.3 | Physics simulation preview (play/pause) | Editor, Physics |
| IR-5.4.4 | Contact point and normal visualization | Editor, Physics |
| IR-5.4.5 | Collision layer editing in property panel | Editor, Physics |
| IR-5.4.6 | Trigger volume visualization and editing | Editor, Physics |
| IR-5.4.7 | Physics material assignment via drag-drop | Editor, Physics |

## Data Contracts

| Type | Defined in | Consumed by | Purpose |
|------|-----------|-------------|---------|
| `Collider` | Physics | Editor gizmos | Shape geometry |
| `ColliderShape` | Physics | Editor gizmos | Shape variant |
| `CollisionLayers` | Physics | Editor property panel | Layer/mask bits |
| `ContactManifold` | Physics | Editor debug overlay | Contact points |
| `PhysicsMaterial` | Physics | Editor drag-drop | Surface properties |
| `RigidBody` | Physics | Editor property panel | Body type config |

### Debug visualization color scheme

| State | Color | Hex |
|-------|-------|-----|
| Active (awake, non-trigger) | Green | `#00FF00` |
| Sleeping (body has `Sleeping` marker) | Gray | `#808080` |
| Trigger volume (`is_trigger = true`) | Yellow | `#FFFF00` |
| Selected (editor selection) | Cyan | `#00FFFF` |
| Error (invalid/degenerate shape) | Red | `#FF0000` |

Sleeping state is queried from `RigidBody` / `Sleeping` marker, not from the collider. Static
colliders without a `RigidBody` always use the Active color.

### Structs and commands

```rust
/// Editor draws collider shapes as wireframe
/// overlays in the viewport.
/// Transform is read via ECS query on the entity's
/// Transform component — not duplicated here.
pub struct ColliderDebugData {
    pub entity: Entity,
    pub shape: ColliderShape,
    pub is_trigger: bool,
}

/// Editor reads contact data for debug lines.
/// Points are in world space. The debug overlay
/// system converts ContactManifold local-space
/// points (local_a, local_b) to world space during
/// Phase 7 (Frame Snapshot) using each entity's
/// Transform.
pub struct ContactDebugData {
    pub point_a: Vec3,
    pub point_b: Vec3,
    pub normal: Vec3,
    pub depth: f32,
}

/// Collider shape editing command via undo stack.
/// Clones full ColliderShape (including ConvexHull /
/// TriMesh vertex data) for undo correctness. The
/// clone cost is acceptable: edits are infrequent
/// user actions, not per-frame operations.
pub struct ColliderEditCommand {
    pub entity: Entity,
    pub old_shape: ColliderShape,
    pub new_shape: ColliderShape,
}

impl EditorCommand for ColliderEditCommand {
    fn description(&self) -> &str {
        "Edit collider shape"
    }
    fn execute(
        &mut self,
        world: &mut World,
    ) -> Result<(), CommandError> {
        // Write new_shape to entity's Collider component
        todo!()
    }
    fn undo(
        &mut self,
        world: &mut World,
    ) -> Result<(), CommandError> {
        // Restore old_shape to entity's Collider component
        todo!()
    }
    fn size_bytes(&self) -> usize {
        size_of::<Self>()
            + self.old_shape.heap_size()
            + self.new_shape.heap_size()
    }
}

/// Edit a single child of a CompoundCollider.
pub struct CompoundChildEditCommand {
    pub entity: Entity,
    pub child_index: usize,
    pub old_child: CompoundChild,
    pub new_child: CompoundChild,
}

impl EditorCommand for CompoundChildEditCommand {
    fn description(&self) -> &str {
        "Edit compound collider child"
    }
    fn execute(
        &mut self,
        world: &mut World,
    ) -> Result<(), CommandError> {
        // Write new_child at child_index in
        // entity's CompoundCollider
        todo!()
    }
    fn undo(
        &mut self,
        world: &mut World,
    ) -> Result<(), CommandError> {
        // Restore old_child at child_index
        todo!()
    }
    fn size_bytes(&self) -> usize {
        size_of::<Self>()
            + self.old_child.shape.heap_size()
            + self.new_child.shape.heap_size()
    }
}

/// Physics material assignment via drag-drop.
/// Undoable via the undo stack.
pub struct MaterialAssignCommand {
    pub entity: Entity,
    pub old_material: PhysicsMaterialHandle,
    pub new_material: PhysicsMaterialHandle,
}

impl EditorCommand for MaterialAssignCommand {
    fn description(&self) -> &str {
        "Assign physics material"
    }
    fn execute(
        &mut self,
        world: &mut World,
    ) -> Result<(), CommandError> {
        // Write new_material to entity's
        // PhysicsMaterialHandle component
        todo!()
    }
    fn undo(
        &mut self,
        world: &mut World,
    ) -> Result<(), CommandError> {
        // Restore old_material
        todo!()
    }
    fn size_bytes(&self) -> usize {
        size_of::<Self>()
    }
}
```

## Data Flow

```mermaid
sequenceDiagram
    participant VP as Editor Viewport
    participant GZ as Gizmo System
    participant US as Undo Stack
    participant EC as EditorCommands Phase
    participant EB as EventBridge Phase
    participant PH as Physics Systems
    participant BVH as Physics BVH
    participant DBG as Debug Overlay

    VP->>GZ: select entity with Collider
    GZ->>GZ: draw shape handles
    GZ->>US: push ColliderEditCommand
    US->>EC: flush commands to editor ECS
    EC->>EB: sync mutations to game world
    EB->>PH: update Collider component
    PH->>BVH: update AABB in physics BVH
    PH->>PH: broadphase + narrowphase
    PH-->>DBG: ContactManifold data
    DBG->>DBG: local-to-world transform
    DBG-->>VP: render contact normals
```

## Class Diagram

```mermaid
classDiagram
    class EditorCommand {
        <<trait>>
        +description() str
        +execute(world) Result
        +undo(world) Result
        +size_bytes() usize
    }
    class ColliderDebugData {
        +entity: Entity
        +shape: ColliderShape
        +is_trigger: bool
    }
    class ContactDebugData {
        +point_a: Vec3
        +point_b: Vec3
        +normal: Vec3
        +depth: f32
    }
    class ColliderEditCommand {
        +entity: Entity
        +old_shape: ColliderShape
        +new_shape: ColliderShape
    }
    class CompoundChildEditCommand {
        +entity: Entity
        +child_index: usize
        +old_child: CompoundChild
        +new_child: CompoundChild
    }
    class MaterialAssignCommand {
        +entity: Entity
        +old_material: PhysicsMaterialHandle
        +new_material: PhysicsMaterialHandle
    }
    class Collider {
        +shape: ColliderShape
        +offset: Vec3
    }
    class CompoundCollider {
        +children: Vec~CompoundChild~
    }
    class CompoundChild {
        +shape: ColliderShape
        +offset: Vec3
        +rotation: Quat
        +layers: CollisionLayers
        +material: PhysicsMaterialHandle
    }
    class ColliderShape {
        <<enum>>
        Sphere
        Box
        Capsule
        ConvexHull
        TriMesh
        Heightfield
    }
    class CollisionLayers {
        +membership: u32
        +mask: u32
    }
    class ContactManifold {
        +entity_a: Entity
        +entity_b: Entity
        +normal: Vec3
        +contacts: ArrayVec~ContactPoint~
    }
    class ContactPoint {
        +local_a: Vec3
        +local_b: Vec3
        +penetration: f32
    }
    class PhysicsMaterial {
        +static_friction: f32
        +dynamic_friction: f32
        +restitution: f32
    }

    ColliderEditCommand ..|> EditorCommand
    CompoundChildEditCommand ..|> EditorCommand
    MaterialAssignCommand ..|> EditorCommand
    ColliderDebugData --> ColliderShape
    ColliderEditCommand --> ColliderShape
    CompoundChildEditCommand --> CompoundChild
    ContactDebugData ..> ContactManifold : derived from
    Collider --> ColliderShape
    CompoundCollider *-- CompoundChild
    CompoundChild --> ColliderShape
    ContactManifold *-- ContactPoint
```

## Timing and Ordering

| System | Game loop phase | Timestep | Ordering |
|--------|----------------|----------|----------|
| Editor Input | PreUpdate | Variable | Gizmo interaction |
| Editor Commands | EditorCommands | Variable | Flush collider edits |
| Physics Sim | Phase 5 Physics | Fixed | Broadphase + solve |
| Debug Overlay | Phase 7 Snapshot | Variable | Read contacts |
| Viewport Render | Render thread | Variable | Draw debug lines |

Collider edits flow through the undo stack via EditorCommands. The physics system picks up the
changed Collider component on the next fixed tick. Debug visualization reads ContactManifold data at
the snapshot phase for rendering.

Phase references: [Phase 5 Physics](../core-runtime/game-loop.md),
[Phase 7 Frame Snapshot](../core-runtime/game-loop.md),
[EditorCommands / EventBridge](../tools/editor-core.md).

## Failure Modes

| Failure | Impact | Recovery |
|---------|--------|----------|
| Invalid collider dimensions | Physics crash | Clamp to minimum size (0.01) |
| Degenerate convex hull | Narrowphase failure | Fall back to AABB approximation |
| Sleeping body not waking | Stale debug display | Force wake on editor select |
| Layer mask all-zero | No collisions | Warn in property panel |
| Contact buffer overflow | Missing debug lines | Per-frame global cap at 1000 |

### Fallback paths

1. **Invalid collider dimensions** -- when any dimension is <= 0 or NaN,
   `ColliderEditCommand::execute` clamps all extents to a minimum of 0.01 before writing the
   Collider component. The undo stack records the clamped value as `new_shape`.
2. **Degenerate convex hull** -- if the convex hull has fewer than 4 non-coplanar vertices, the
   narrowphase substitutes the collider's AABB as an approximation. A warning diagnostic is emitted
   via the editor's problem panel.
3. **Sleeping body not waking** -- selecting an entity with a `Sleeping` marker in the editor forces
   a wake by removing the `Sleeping` component and resetting `SleepTimer`. This ensures debug
   visualization and gizmo interaction reflect live physics state.
4. **Layer mask all-zero** -- the property panel displays a warning badge when
   `CollisionLayers.membership == 0` or `CollisionLayers.mask == 0`. No simulation fallback; the
   entity simply does not participate in collisions.
5. **Contact buffer overflow** -- the 1000-contact cap is per-frame global and affects visualization
   only. The debug overlay collects up to 1000 `ContactDebugData` entries per frame. Excess contacts
   are silently dropped from the debug display. The physics simulation is unaffected; all contacts
   are still solved.

## Platform Considerations

None -- identical across all platforms. Physics simulation is deterministic and
platform-independent. Debug visualization uses the same debug draw API on all GPU backends.

## Test Plan

See companion [editor-physics-test-cases.md](editor-physics-test-cases.md).

## Review Feedback

1. [CONFIDENT] The sequence diagram routes physics broadphase updates through "Shared BVH," but
   `constraints.md` specifies a **physics-private BVH** separate from the shared BVH. The diagram
   and prose must use the physics-private BVH for collision work.

2. [CONFIDENT] `ColliderEditCommand` is a plain struct but the editor-core design defines
   `EditorCommand` as a `trait`. The struct should show `impl EditorCommand for ColliderEditCommand`
   with `execute`/`undo` methods so it integrates with the undo stack properly.

3. [CONFIDENT] No 2D / 2.5D coverage. The constraints require first-class 2D support including 2D
   collider shapes (circle, rectangle, capsule, polygon) and 2D physics. The data contracts, gizmos,
   and test cases all assume 3D-only (`Vec3`, `Mat4`, `ColliderShape` variants are 3D).

4. [CONFIDENT] `ColliderDebugData` stores a full `Mat4 world_transform` as an owned copy. The
   immutable-first constraint and no-copy preferences suggest reading the `Transform` component
   directly via ECS query rather than duplicating it into a separate struct.

5. [UNCERTAIN] `ColliderEditCommand` stores `old_shape: ColliderShape` and
   `new_shape: ColliderShape`. `ColliderShape::ConvexHull` and `ColliderShape::TriMesh` contain
   `Vec<Vec3>`, making clones expensive. Consider storing a delta or handle instead of cloning full
   mesh data for undo.

6. [CONFIDENT] The `color` field on `ColliderDebugData` is `LinearColor`, but the document never
   defines the color scheme (green = active, gray = sleeping, yellow = trigger). The test cases
   assume these colors. The data contract or a constants table should specify the mapping.

7. [CONFIDENT] `ContactDebugData` uses `Vec3` for `point_a` and `point_b`, but `ContactManifold` in
   the physics foundation stores `local_a` and `local_b` in local space. The integration must
   specify who performs the local-to-world transform and at which phase.

8. [CONFIDENT] The document references `EventBridge` as a participant in the sequence diagram, but
   it is not listed in the Data Contracts table and has no Rust pseudocode. It should be documented
   or replaced with the `EditorCommands` phase used in the prose.

9. [CONFIDENT] Missing `CompoundCollider` handling. The physics foundation defines
   `CompoundCollider` with child shapes, and test case TC-IR-5.4.2.3 tests compound display, but
   `ColliderEditCommand` only handles a single `ColliderShape`. There is no command for editing
   individual children of a compound collider.

10. [CONFIDENT] No class diagram. The design CLAUDE.md requires "a Mermaid classDiagram covering ALL
    types: structs, enums, traits, type aliases, and their relationships." This document has only a
    sequence diagram.

11. [UNCERTAIN] The Failure Modes table lists "Contact buffer overflow" capped at 1000 debug
    contacts. The benchmark TC-IR-5.4.4.B1 also targets 1000 contacts. It is unclear whether this
    cap is enforced per-frame globally or per-entity, and whether discarded contacts affect
    simulation correctness or only visualization.

12. [CONFIDENT] Test case TC-IR-5.4.7.1 (physics material drag-drop) has no undo/redo counterpart.
    Every other editing IR (5.4.1, 5.4.5) includes an undo test. Material assignment should also be
    undoable.

13. [CONFIDENT] The Timing and Ordering table references "Phase 5 Physics" and "Phase 7 Snapshot"
    but does not cite the game loop design where these phases are defined. Add cross-references to
    the game loop design document for traceability.

14. [CONFIDENT] `ColliderDebugData` contains `is_sleeping: bool` but sleeping is a property of
    `RigidBody` / `SleepTimer`, not the collider. A collider can exist without a rigid body (static
    geometry). The field conflates collider and body state.
