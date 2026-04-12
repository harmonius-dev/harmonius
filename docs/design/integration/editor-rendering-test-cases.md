# Editor ↔ Rendering Integration Test Cases

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-5.5.1.1 | Viewport renders scene | 100 entities | All meshes visible | IR-5.5.1 |
| TC-IR-5.5.1.2 | Viewport camera orbit | MMB drag | Camera orbits pivot | IR-5.5.1 |
| TC-IR-5.5.1.3 | Editor layers ignored | Gameplay cam + grid | Grid not in game cam | IR-5.5.1 |
| TC-IR-5.5.2.1 | Translate gizmo visible | Translate mode | RGB arrows at origin | IR-5.5.2 |
| TC-IR-5.5.2.2 | Rotate gizmo visible | Rotate mode | RGB rings at origin | IR-5.5.2 |
| TC-IR-5.5.2.3 | Scale gizmo visible | Scale mode | RGB cubes at origin | IR-5.5.2 |
| TC-IR-5.5.2.4 | Gizmo snap transform | X drag, snap=1.0 | Moves in 1.0 steps | IR-5.5.2 |
| TC-IR-5.5.2.5 | Gizmo undo/redo | Move + undo | Returns to original | IR-5.5.2 |
| TC-IR-5.5.2.6 | Gizmo depth test mode | Occluded gizmo | Hidden by geometry | IR-5.5.2 |
| TC-IR-5.5.3.1 | Outline single entity | Click entity | Outline around mesh | IR-5.5.3 |
| TC-IR-5.5.3.2 | Outline multiple ents | Marquee 5 ents | All 5 outlined | IR-5.5.3 |
| TC-IR-5.5.3.3 | Raycast nearest hit | Overlapping meshes | Nearest mesh picked | IR-5.5.3 |
| TC-IR-5.5.4.1 | Wireframe toggle | Enable wireframe | All meshes wireframe | IR-5.5.4 |
| TC-IR-5.5.4.2 | Normal visualization | WorldNormals mode | Normals as RGB | IR-5.5.4 |
| TC-IR-5.5.4.3 | Runtime toggle on | Toggle at runtime | Uniform updates live | IR-5.5.4 |
| TC-IR-5.5.5.1 | Dual viewport | Split viewport | Two independent views | IR-5.5.5 |
| TC-IR-5.5.5.2 | Viewport independence | Orbit one | Other unchanged | IR-5.5.5 |
| TC-IR-5.5.6.1 | Albedo vis | Select Albedo mode | Albedo only displayed | IR-5.5.6 |
| TC-IR-5.5.6.2 | Overdraw vis | Select Overdraw | Overdraw heatmap | IR-5.5.6 |
| TC-IR-5.5.7.1 | Editor grid renders | Open viewport | Infinite XZ grid | IR-5.5.7 |
| TC-IR-5.5.7.2 | Measurement gizmo | Activate measure | Distance label shown | IR-5.5.7 |

## Negative / Failure Mode Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-5.5.1.N1 | Viewport cap 8 | Open 9th viewport | Refused with error | IR-5.5.1 |
| TC-IR-5.5.1.N2 | Viewport 5+ half-res | Open 5 viewports | 5th at half res | IR-5.5.1 |
| TC-IR-5.5.2.N1 | Occluded ghost | Toggle `occluded_ghost` | 50% opacity redraw | IR-5.5.2 |
| TC-IR-5.5.2.N2 | Gizmo arena overflow | 10K commands queued | Oldest dropped + warn | IR-5.5.2 |
| TC-IR-5.5.2.N3 | Unknown GizmoTypeId | Stale `.dylib` id | Fallback to Cube | IR-5.5.2 |
| TC-IR-5.5.3.N1 | BVH stale one frame | Move then pick | One-frame lag only | IR-5.5.3 |
| TC-IR-5.5.3.N2 | Selection > 256 ents | Select 10K | Cap + wireframe rest | IR-5.5.3 |
| TC-IR-5.5.3.N3 | No stencil buffer | Disable stencil | Color ID edge-detect | IR-5.5.3 |
| TC-IR-5.5.3.N4 | Raycast miss | Click empty space | Selection unchanged | IR-5.5.3 |
| TC-IR-5.5.4.N1 | GPU timeout overlay | Overdraw hangs GPU | Fallback to lit mode | IR-5.5.4 |
| TC-IR-5.5.4.N2 | No cfg shader gate | Release build grep | No cfg debug blocks | IR-5.5.4 |
| TC-IR-5.5.5.N1 | Viewport resize zero | Set size 0x0 | Skip render, no panic | IR-5.5.5 |
| TC-IR-5.5.6.N1 | Invalid vis mode | Corrupted enum | Fallback to Albedo | IR-5.5.6 |
| TC-IR-5.5.7.N1 | Grid far clip | Camera at 10km | Grid still visible | IR-5.5.7 |
| TC-IR-Alloc.N1 | HashMap guard | Instrument hot path | Zero HashMap ops | IR-5.5.* |
| TC-IR-Alloc.N2 | Arc/Rc guard | Instrument hot path | Zero Arc/Rc usage | IR-5.5.* |
| TC-IR-Alloc.N3 | Heap alloc guard | 100-frame capture | Zero heap on extract | IR-5.5.* |

1. **TC-IR-5.5.1.N1** -- verifies the 8-viewport cap. Opening a 9th viewport surfaces a user-visible
   error and does not allocate render targets.
2. **TC-IR-5.5.1.N2** -- verifies half-resolution degradation kicks in for the fifth and later
   viewports, per failure mode 3.
3. **TC-IR-5.5.2.N1** -- verifies the `occluded_ghost` runtime toggle triggers a second gizmo pass
   with depth test disabled at 50% opacity.
4. **TC-IR-5.5.2.N2** -- verifies `GizmoDrawCommand` arena overflow drops oldest commands and logs a
   warning. Arena resets each frame.
5. **TC-IR-5.5.2.N3** -- verifies a `GizmoTypeId` from a stale `.dylib` falls back to
   `GizmoShape::Cube` after hot-reload remapping.
6. **TC-IR-5.5.3.N1** -- verifies BVH rebuild lag is bounded to one frame. Pick on frame N+1 returns
   the moved entity.
7. **TC-IR-5.5.3.N2** -- verifies the 256-entity outline cap and the bounding-box wireframe fallback
   for remaining selected entities.
8. **TC-IR-5.5.3.N3** -- verifies the stencil-less fallback path uses the Sobel kernel on entity IDs
   written to a color attachment.
9. **TC-IR-5.5.3.N4** -- verifies a raycast miss (click in empty space) leaves `SelectionSet`
   unchanged and does not log an error.
10. **TC-IR-5.5.4.N1** -- verifies GPU timeout recovery disables the offending `BufferVisMode` for
    the remainder of the frame and restores standard lit rendering.
11. **TC-IR-5.5.4.N2** -- static scan of the release-build binary confirms debug shader branches are
    present (no `#[cfg(debug_assertions)]` gating). Runtime toggles are the only gate.
12. **TC-IR-5.5.5.N1** -- verifies a viewport resized to 0x0 skips rendering without panicking.
13. **TC-IR-5.5.6.N1** -- verifies an invalid `BufferVisMode` value falls back to `Albedo` and logs
    a warning.
14. **TC-IR-5.5.7.N1** -- verifies the infinite grid shader's analytical derivatives hold at extreme
    camera distances (10 km from origin).
15. **TC-IR-Alloc.N1** -- runtime-toggleable debug counter asserts zero `HashMap` operations on the
    Phase 7 extract, selection, and gizmo build hot paths.
16. **TC-IR-Alloc.N2** -- static scan of the integration crate confirms zero occurrences of `Arc<`,
    `Rc<`, `Cell<`, or `RefCell<` in the editor-rendering integration module.
17. **TC-IR-Alloc.N3** -- allocation counter wrapped around 100 frames of Phase 7 extract asserts
    zero heap allocations outside the per-frame arena.

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-5.5.1.B1 | Viewport render 10K ents | < 16 ms total | IR-5.5.1 |
| TC-IR-5.5.1.B2 | Four-viewport layout | < 20 ms total | IR-5.5.1 |
| TC-IR-5.5.2.B1 | Gizmo overlay 100 sel | < 0.5 ms GPU | IR-5.5.2 |
| TC-IR-5.5.2.B2 | Gizmo build CPU 100 | < 0.1 ms CPU | IR-5.5.2 |
| TC-IR-5.5.3.B1 | Raycast 50K meshlets | < 2 ms CPU | IR-5.5.3 |
| TC-IR-5.5.3.B2 | Outline pass 256 ents | < 0.5 ms GPU | IR-5.5.3 |
| TC-IR-5.5.3.B3 | Sobel compute 1080p | < 0.3 ms GPU | IR-5.5.3 |
| TC-IR-5.5.4.B1 | Debug runtime switch | < 0.05 ms CPU | IR-5.5.4 |
| TC-IR-5.5.5.B1 | Phase 7 extract cost | < 0.5 ms CPU | IR-5.5.5 |
| TC-IR-5.5.7.B1 | Grid full-screen pass | < 0.2 ms GPU | IR-5.5.7 |

1. **TC-IR-5.5.1.B1** -- measures total frame time rendering 10,000 entities through a single editor
   viewport using the standard render graph.
2. **TC-IR-5.5.1.B2** -- measures total frame time with four open viewports sharing a single
   `ProxyStore`, each with an independent camera.
3. **TC-IR-5.5.2.B2** -- measures CPU cost of building 100 `GizmoDrawCommand` entries into the
   per-frame arena during the `EditorCommands` phase.
4. **TC-IR-5.5.3.B3** -- measures GPU cost of the Sobel edge-detect compute pass at 1920x1080
   resolution, isolated from the stencil write pass.
5. **TC-IR-5.5.4.B1** -- measures CPU overhead of switching `BufferVisMode` at runtime. Toggle
   updates a uniform only; no shader recompilation.
6. **TC-IR-5.5.5.B1** -- measures the Phase 7 extract cost for a scene with one editor viewport, 100
   selected entities, and 100 gizmo commands.

All benchmarks run under `cargo bench` and are CI-executable. Targets are measured on the reference
hardware profile defined in [core-runtime/game-loop.md](../core-runtime/game-loop.md).
