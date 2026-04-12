# Editor ↔ Physics Integration Test Cases

All integration, unit, and negative tests below are CI-runnable. No GPU hardware is required: debug
draw calls are captured and verified against expected primitives.

## Integration Tests

| ID | Test | Req |
|----|------|-----|
| TC-IR-5.4.1.1 | Box collider resize gizmo | IR-5.4.1 |
| TC-IR-5.4.1.2 | Sphere collider radius gizmo | IR-5.4.1 |
| TC-IR-5.4.1.3 | Capsule collider height gizmo | IR-5.4.1 |
| TC-IR-5.4.1.4 | Collider edit undo/redo | IR-5.4.1 |
| TC-IR-5.4.2.1 | Wireframe collider overlay | IR-5.4.2 |
| TC-IR-5.4.2.2 | Sleeping body color change | IR-5.4.2 |
| TC-IR-5.4.2.3 | Compound collider display | IR-5.4.2 |
| TC-IR-5.4.2.4 | Compound child edit undo/redo | IR-5.4.2 |
| TC-IR-5.4.3.1 | Physics play mode | IR-5.4.3 |
| TC-IR-5.4.3.2 | Physics pause and step | IR-5.4.3 |
| TC-IR-5.4.3.3 | Physics stop restores state | IR-5.4.3 |
| TC-IR-5.4.4.1 | Contact point markers | IR-5.4.4 |
| TC-IR-5.4.4.2 | Contact normal arrows | IR-5.4.4 |
| TC-IR-5.4.5.1 | Layer mask editing | IR-5.4.5 |
| TC-IR-5.4.5.2 | Layer mask preview | IR-5.4.5 |
| TC-IR-5.4.6.1 | Trigger volume wireframe | IR-5.4.6 |
| TC-IR-5.4.6.2 | Trigger overlap event | IR-5.4.6 |
| TC-IR-5.4.7.1 | Physics material drag-drop | IR-5.4.7 |
| TC-IR-5.4.7.2 | Physics material assign undo/redo | IR-5.4.7 |

### Integration test details

1. **TC-IR-5.4.1.1** — input: drag `+X` handle on a `ColliderShape::Box` by `0.5` units; expected:
   `half_extents.x` increases by `0.5`, command pushed to undo stack.
2. **TC-IR-5.4.1.2** — input: drag radius handle on `ColliderShape::Sphere` by `0.25`; expected:
   `radius` increases by `0.25`.
3. **TC-IR-5.4.1.3** — input: drag top-cap handle of `ColliderShape::Capsule`; expected: capsule
   `half_height` updated, `radius` unchanged.
4. **TC-IR-5.4.1.4** — input: resize box, then invoke `undo`; expected: original `half_extents`
   restored byte-for-byte; redo restores edited value.
5. **TC-IR-5.4.2.1** — input: entity with `ColliderShape::Box`; expected: wireframe box drawn in
   viewport using Active color `#00FF00`.
6. **TC-IR-5.4.2.2** — input: `RigidBody` enters sleep (adds `Sleeping`); expected: overlay color
   changes to `#808080`.
7. **TC-IR-5.4.2.3** — input: entity with `CompoundCollider` of 3 children; expected: 3 children
   drawn at their respective local offsets.
8. **TC-IR-5.4.2.4** — input: edit child `1` of a compound collider, then `undo`; expected: child
   `1` restored to original shape; other children untouched.
9. **TC-IR-5.4.3.1** — input: press Play; expected: dynamic bodies fall under gravity each tick.
10. **TC-IR-5.4.3.2** — input: pause, then step 1 substep; expected: bodies advance exactly one
    fixed-timestep integration.
11. **TC-IR-5.4.3.3** — input: press Stop; expected: all body poses equal their pre-play snapshot.
12. **TC-IR-5.4.4.1** — input: two resting boxes; expected: debug markers drawn at each
    `ContactPoint` world position.
13. **TC-IR-5.4.4.2** — input: two resting boxes; expected: arrow primitives drawn along
    `ContactManifold::normal`.
14. **TC-IR-5.4.5.1** — input: toggle layer `3` in property panel; expected: `CollisionLayers`
    `membership` bit `3` flipped.
15. **TC-IR-5.4.5.2** — input: set masks so only layers `0 & 1` match; expected: only colliders on
    those layers generate contacts.
16. **TC-IR-5.4.6.1** — input: entity with `is_trigger = true`; expected: wireframe rendered using
    Trigger color `#FFFF00`.
17. **TC-IR-5.4.6.2** — input: rigid body enters trigger volume; expected: `TriggerEvent` emitted
    with correct `entity_a` / `entity_b`.
18. **TC-IR-5.4.7.1** — input: drag `PhysicsMaterial` asset onto entity; expected:
    `PhysicsMaterialHandle` replaced; restitution reflects the new material.
19. **TC-IR-5.4.7.2** — input: drag new material onto entity, then `undo`; expected: original
    `PhysicsMaterialHandle` restored; redo reapplies new handle.

## Unit Tests

| ID | Test | Req |
|----|------|-----|
| TC-IR-5.4.U1 | `ColliderEditCommand::execute` writes new shape | IR-5.4.1 |
| TC-IR-5.4.U2 | `ColliderEditCommand::undo` restores old shape | IR-5.4.1 |
| TC-IR-5.4.U3 | `ColliderEditCommand::size_bytes` includes heap | IR-5.4.1 |
| TC-IR-5.4.U4 | `CompoundChildEditCommand::undo` restores child | IR-5.4.2 |
| TC-IR-5.4.U5 | `MaterialAssignCommand::undo` restores handle | IR-5.4.7 |
| TC-IR-5.4.U6 | Color scheme maps states to hex values | IR-5.4.2 |
| TC-IR-5.4.U7 | Local-to-world transform of `ContactPoint` | IR-5.4.4 |

### Unit test details

1. **TC-IR-5.4.U1** — call `execute` with a new `ColliderShape::Box`; assert entity's `Collider`
   component equals `new_shape`.
2. **TC-IR-5.4.U2** — call `undo`; assert entity's `Collider` component equals `old_shape`.
3. **TC-IR-5.4.U3** — for a `ColliderShape::TriMesh` with `N` vertices, assert
   `size_bytes() >= N * size_of::<Vec3>()`.
4. **TC-IR-5.4.U4** — edit compound child then `undo`; assert that child restored and siblings
   untouched.
5. **TC-IR-5.4.U5** — assign new material then `undo`; assert original handle restored.
6. **TC-IR-5.4.U6** — map each `(sleeping, is_trigger, selected)` state to its color; assert hex
   values match the "Debug visualization color scheme" table exactly.
7. **TC-IR-5.4.U7** — given `ContactPoint { local_a, local_b }` and a `Transform`, assert
   world-space conversion equals `transform * local_a` (same for `b`).

## Negative Tests

| ID | Test | Req |
|----|------|-----|
| TC-IR-5.4.N1 | Collider dimension zero clamped | IR-5.4.1 |
| TC-IR-5.4.N2 | Collider dimension NaN rejected | IR-5.4.1 |
| TC-IR-5.4.N3 | Degenerate convex hull falls back to AABB | IR-5.4.1 |
| TC-IR-5.4.N4 | Layer mask all-zero emits warning | IR-5.4.5 |
| TC-IR-5.4.N5 | Contact overflow drops excess visualization | IR-5.4.4 |
| TC-IR-5.4.N6 | Undo empty stack is a no-op | IR-5.4.1 |
| TC-IR-5.4.N7 | Editor selecting sleeping body force-wakes it | IR-5.4.2 |
| TC-IR-5.4.N8 | Compound child index out of range rejected | IR-5.4.2 |

### Negative test details

1. **TC-IR-5.4.N1** — input: box `half_extents.x = 0.0`; expected: clamped to `0.01`, diagnostic
   logged, undo records clamped value.
2. **TC-IR-5.4.N2** — input: sphere `radius = NaN`; expected: `execute` returns
   `CommandError::InvalidInput`; Collider unchanged.
3. **TC-IR-5.4.N3** — input: `ConvexHull` with 3 coplanar points; expected: narrowphase uses AABB
   fallback and a warning is emitted to the problem panel.
4. **TC-IR-5.4.N4** — input: `CollisionLayers.membership = 0`; expected: warning badge shown in
   panel; entity stays in simulation but generates no contacts.
5. **TC-IR-5.4.N5** — input: `1500` contacts in a single frame; expected: first `1000`
   `ContactDebugData` entries rendered; remaining `500` dropped; physics solve unaffected.
6. **TC-IR-5.4.N6** — input: empty undo stack; call `undo`; expected: returns `Ok(())`, no panic.
7. **TC-IR-5.4.N7** — input: editor selects entity with `Sleeping` marker; expected: `Sleeping`
   component removed, `SleepTimer` reset, overlay switches to Active color.
8. **TC-IR-5.4.N8** — input: `CompoundChildEditCommand { child_index: 99 }` on a 3-child compound;
   expected: `execute` returns `CommandError::InvalidIndex`; state unchanged.

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-5.4.2.B1 | Debug wireframe 500 colliders | < 1 ms GPU | IR-5.4.2 |
| TC-IR-5.4.4.B1 | Contact debug 1000 contacts | < 0.5 ms GPU | IR-5.4.4 |
| TC-IR-5.4.3.B1 | Editor play mode startup | < 100 ms | IR-5.4.3 |
| TC-IR-5.4.1.B1 | Collider edit command dispatch | < 0.1 ms CPU | IR-5.4.1 |
| TC-IR-5.4.7.B1 | Material drag-drop assign | < 0.1 ms CPU | IR-5.4.7 |
