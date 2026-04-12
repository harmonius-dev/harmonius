# Editor ↔ Physics Integration Test Cases

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-5.4.1.1 | Box collider resize gizmo | Drag handle on box collider | ColliderShape::Box half-extents updated | IR-5.4.1 |
| TC-IR-5.4.1.2 | Sphere collider radius gizmo | Drag radius handle | ColliderShape::Sphere radius updated | IR-5.4.1 |
| TC-IR-5.4.1.3 | Capsule collider height gizmo | Drag top cap handle | Capsule height and radius updated | IR-5.4.1 |
| TC-IR-5.4.1.4 | Collider edit undo/redo | Resize box, undo | Original half-extents restored | IR-5.4.1 |
| TC-IR-5.4.2.1 | Wireframe collider overlay | Entity with box Collider | Green wireframe box in viewport | IR-5.4.2 |
| TC-IR-5.4.2.2 | Sleeping body color change | Body enters sleep state | Wireframe changes to gray | IR-5.4.2 |
| TC-IR-5.4.2.3 | Compound collider display | Entity with 3-child compound | All 3 child shapes drawn | IR-5.4.2 |
| TC-IR-5.4.3.1 | Physics play mode | Press play in editor | Bodies fall under gravity | IR-5.4.3 |
| TC-IR-5.4.3.2 | Physics pause and step | Pause, step 1 tick | Bodies advance 1 substep | IR-5.4.3 |
| TC-IR-5.4.3.3 | Physics stop restores state | Stop play mode | All bodies return to pre-play pose | IR-5.4.3 |
| TC-IR-5.4.4.1 | Contact point markers | Two boxes resting | Green dots at contact points | IR-5.4.4 |
| TC-IR-5.4.4.2 | Contact normal arrows | Two boxes resting | Blue arrows along contact normals | IR-5.4.4 |
| TC-IR-5.4.5.1 | Layer mask editing | Toggle layer 3 in panel | CollisionLayers bit 3 flipped | IR-5.4.5 |
| TC-IR-5.4.5.2 | Layer mask preview | Set layer mask, run sim | Only matching layers collide | IR-5.4.5 |
| TC-IR-5.4.6.1 | Trigger volume wireframe | Entity with is_trigger=true | Yellow wireframe (distinct color) | IR-5.4.6 |
| TC-IR-5.4.6.2 | Trigger overlap event | Body enters trigger | TriggerEvent fires, log entry | IR-5.4.6 |
| TC-IR-5.4.7.1 | Physics material drag-drop | Drag PhysicsMaterial onto entity | Material assigned, restitution updated | IR-5.4.7 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-5.4.2.B1 | Debug wireframe 500 colliders | < 1 ms GPU | IR-5.4.2 |
| TC-IR-5.4.4.B1 | Contact debug 1000 contacts | < 0.5 ms GPU | IR-5.4.4 |
| TC-IR-5.4.3.B1 | Editor play mode startup | < 100 ms | IR-5.4.3 |
