# Containers/Slots ↔ Rendering Integration Test Cases

All test cases are CI-runnable under `cargo test` with no external services or GPU required.
Rendering cases use the headless renderer fake from `rendering-core-test-cases.md`.

## Unit Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-5.8.1.U1 | compute_attachment_transform | See detail 1 | Correct Mat4 | IR-5.8.1 |
| TC-IR-5.8.1.U2 | Identity socket offset | See detail 2 | Parent transform | IR-5.8.1 |
| TC-IR-5.8.1.U3 | PreviousGlobalTransform copy | See detail 3 | Prev == old current | IR-5.8.1 |
| TC-IR-5.8.2.U1 | Typed mesh override handle | See detail 4 | Compiles, typed | IR-5.8.2 |
| TC-IR-5.8.5.U1 | Hysteresis activation | See detail 5 | Activate at r | IR-5.8.5 |
| TC-IR-5.8.5.U2 | Hysteresis deactivation | See detail 6 | Deactivate at 1.1r | IR-5.8.5 |
| TC-IR-5.8.1.U4 | Render layer inheritance | See detail 7 | parent & override | IR-5.8.1 |
| TC-IR-5.8.1.U5 | Layer inherit no override | See detail 8 | Equals parent | IR-5.8.1 |

1. Parent GlobalTransform = translation (1,2,3), SocketDefinition offset = (0,1,0). Expected: child
   GlobalTransform translation = (1,3,3), rotation preserved.
2. SocketDefinition with `Vec3::ZERO` and `Quat::IDENTITY`. Expected: child transform equals the
   parent transform exactly.
3. Set GlobalTransform to value A, run VisualBinding tick, then set to value B and run again.
   Expected: PreviousGlobalTransform equals A on the second tick.
4. Construct `VisualOverride { mesh_override: Some(AssetHandle::<Mesh>::new(1)), .. }`. Expected:
   compiles; the field type is `Option<AssetHandle<Mesh>>`, not untyped.
5. Snap preview inactive, cursor distance = snap_radius. Expected: preview activates.
6. Snap preview active, cursor distance = snap_radius * 1.05. Expected: preview stays active
   (hysteresis band). Cursor distance = snap_radius * 1.11. Expected: preview deactivates.
7. `parent = 0b1100`, `override_layers = Some(0b1010)`. Expected: result = `0b1000`.
8. `parent = 0b0110`, `override_layers = None`. Expected: result = `0b0110`.

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-5.8.1.1 | Sword at hand socket | See detail 1 | Correct transform | IR-5.8.1 |
| TC-IR-5.8.1.2 | Helmet at head socket | See detail 2 | Correct transform | IR-5.8.1 |
| TC-IR-5.8.1.3 | Detach removes draw | See detail 3 | Not in draw list | IR-5.8.1 |
| TC-IR-5.8.1.4 | Interpolation no jitter | See detail 4 | Smooth lerp path | IR-5.8.1 |
| TC-IR-5.8.2.1 | Mesh override on attach | See detail 5 | Override mesh used | IR-5.8.2 |
| TC-IR-5.8.2.2 | Material override | See detail 6 | Override material | IR-5.8.2 |
| TC-IR-5.8.2.3 | Revert on detach | See detail 7 | Original restored | IR-5.8.2 |
| TC-IR-5.8.3.1 | Debug spheres on toggle | See detail 8 | Gizmos present | IR-5.8.3 |
| TC-IR-5.8.3.2 | Debug labels on toggle | See detail 9 | Text at sockets | IR-5.8.3 |
| TC-IR-5.8.3.3 | Debug toggle off | See detail 10 | Gizmos removed | IR-5.8.3 |
| TC-IR-5.8.4.1 | Hide socket visual | See detail 11 | Base mesh hidden | IR-5.8.4 |
| TC-IR-5.8.4.2 | Show on detach | See detail 12 | Base mesh visible | IR-5.8.4 |
| TC-IR-5.8.5.1 | Snap preview ghost | See detail 13 | Ghost at snap pos | IR-5.8.5 |
| TC-IR-5.8.5.2 | Snap preview clear | See detail 14 | Ghost removed | IR-5.8.5 |
| TC-IR-5.8.6.1 | Equipment proxy update | See detail 15 | Proxy updated | IR-5.8.6 |
| TC-IR-5.8.6.2 | Batch equipment swap | See detail 16 | All 8 updated | IR-5.8.6 |
| TC-IR-5.8.1.5 | Layer inherit in draw | See detail 17 | Same camera sees | IR-5.8.1 |
| TC-IR-5.8.1.6 | Extract via ECS | See detail 18 | No direct PS write | IR-5.8.1 |

1. Attach sword entity to right_hand socket. After Phase 3 + Phase 7, ProxyStore contains the
   sword's mesh at `parent_hand_transform * socket_offset`.
2. Attach helmet entity to head socket. After one tick, helmet mesh proxy transform equals the head
   bone world transform.
3. Detach the sword. After one tick, no mesh proxy for the sword exists in the draw list.
4. Run VisualBinding for 4 fixed ticks with parent moving in a straight line. Capture interpolated
   transforms at `alpha = 0.0, 0.5, 1.0`. Expected: strictly monotonic; no jitter between frames.
5. Set `VisualOverride.mesh_override = Some(handle_A)`. After Phase 7, attached entity's proxy mesh
   equals `handle_A`.
6. Set `VisualOverride.material_override = Some(handle_M)`. After Phase 7, attached entity's proxy
   material equals `handle_M`.
7. Detach an item whose VisualOverride had replaced the base mesh. After one tick, proxy mesh equals
   the original base mesh handle.
8. Set `DebugConfig.sockets_enabled = true`, spawn 3 sockets. After one tick, 3 `DebugSocketGizmo`
   components exist, each with the socket's world transform.
9. With debug enabled, each gizmo has a non-empty `socket_name` matching the SocketDefinition.
10. Flip `DebugConfig.sockets_enabled` to false. After one tick, all `DebugSocketGizmo` components
    removed; RenderExtractor sees the change-detection removal.
11. Attach with `hide_socket_visual = true`. Base socket mesh proxy removed from draw list.
12. Detach the item. Base socket mesh proxy re-added to draw list.
13. Drag an item so its cursor enters a snap point's `snap_radius`. `SnapPreview` entity spawned
    with ghost mesh at the snap point's world transform.
14. Drag the item out of `snap_radius * 1.1`. `SnapPreview` entity despawned.
15. Swap a weapon in an equipment slot. Within one fixed tick + one extract, the render proxy's mesh
    handle matches the new weapon.
16. Swap 8 equipment slots in a single frame. All 8 proxies updated; no flicker (every proxy has a
    valid mesh every frame).
17. Attach item with `RenderLayers = Some(parent & 0b1011)`. The resulting proxy is visible in
    cameras matching both parent and override bits.
18. Assert at the end of Phase 3 that `ProxyStore` was not written. Run Phase 7; assert the attached
    entity's transform now appears in `ProxyStore.transforms` via the extractor path.

## Negative Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-5.8.1.N1 | Socket def zero offset | See detail 1 | Identity fallback | IR-5.8.1 |
| TC-IR-5.8.2.N1 | Mesh override not ready | See detail 2 | Placeholder cube | IR-5.8.2 |
| TC-IR-5.8.2.N2 | Override on no-mesh | See detail 3 | Warn, skip | IR-5.8.2 |
| TC-IR-5.8.1.N2 | Orphaned attachment | See detail 4 | Detach emitted | IR-5.8.1 |
| TC-IR-5.8.5.N1 | Snap boundary oscillate | See detail 5 | Stable, no flicker | IR-5.8.5 |
| TC-IR-5.8.1.N3 | First-frame prev missing | See detail 6 | prev == current | IR-5.8.1 |
| TC-IR-5.8.1.N4 | Layer mismatch | See detail 7 | Visible w/ parent | IR-5.8.1 |
| TC-IR-5.8.3.N1 | Toggle race removal | See detail 8 | No stale gizmos | IR-5.8.3 |
| TC-IR-5.8.6.N1 | Equipment swap backpressure | See detail 9 | Oldest dropped | IR-5.8.6 |

1. SocketDefinition with all-zero fields. Expected: VisualBinding applies `Mat4::IDENTITY` offset;
   attached item renders at parent origin; debug warning logged once.
2. `mesh_override` handle points to an asset whose `AssetTable::state` returns
   `AssetState::Loading`. Expected: placeholder unit cube used; re-check next frame; swap to real
   mesh when state becomes `Ready`.
3. Apply VisualOverride to an entity lacking MeshComponent. Expected: debug warning; override
   skipped; no panic.
4. Despawn the parent socket entity while an attachment still references it. Expected: DetachSystem
   emits DetachEvent; Attachment component removed; VisualBinding skips the entity.
5. Hold cursor exactly at `snap_radius * 1.02` for 10 frames. Expected: `SnapPreview` stays in a
   stable state (no flicker between spawned/despawned).
6. Newly attached entity has no PreviousGlobalTransform. Expected: VisualBinding initializes
   PreviousGlobalTransform equal to the new GlobalTransform; `lerp` yields no jitter.
7. Attached item override mask = `0b0000`, parent mask = `0b1111`. Expected: result = `0b0000`.
   Assert the integration test then clamps to parent (fallback path per FM-7) when no override
   specified; when an explicit zero mask is provided, the item is legitimately hidden and test
   passes with a logged warning.
8. Flip `DebugConfig.sockets_enabled` to false in the same frame that DebugSockets inserts new
   gizmos. Expected: next tick removes all gizmos; RenderExtractor change-detection clears the Debug
   phase bucket; no stale gizmos for more than 1 frame.
9. Push 257 attach requests into the 256-slot MPSC in one frame. Expected: oldest request dropped,
   warning logged, remaining 256 processed.

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-5.8.1.B1 | VisualBinding 64 sockets | < 0.5 ms | IR-5.8.1 |
| TC-IR-5.8.1.B2 | VisualBinding 1024 sockets | < 4 ms | IR-5.8.1 |
| TC-IR-5.8.2.B1 | 100 attach+detach w/ override | < 1 ms | IR-5.8.2 |
| TC-IR-5.8.2.B2 | Override apply/revert burst | < 0.5 ms | IR-5.8.2 |
| TC-IR-5.8.5.B1 | Snap query 500 sockets | < 2 ms | IR-5.8.5 |
| TC-IR-5.8.6.B1 | Proxy update 100 equipment | < 0.2 ms | IR-5.8.6 |
| TC-IR-5.8.1.B3 | Extract 1024 changed attachments | < 1 ms | IR-5.8.1 |
| TC-IR-5.8.3.B1 | Debug gizmo extract 256 sockets | < 0.3 ms | IR-5.8.3 |
