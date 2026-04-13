# UI ↔ Physics Integration Test Cases

All tests are CI-runnable on CPU. Cameras and physics scenes are constructed inline using
fake/in-memory fixtures.

## Integration Tests

| ID | Test | Req |
|----|------|-----|
| TC-IR-4.3.1.1 | Cursor pick hits box | IR-4.3.1 |
| TC-IR-4.3.1.2 | Cursor pick miss returns `None` | IR-4.3.1 |
| TC-IR-4.3.1.3 | Ray mask excludes filtered layers | IR-4.3.1 |
| TC-IR-4.3.1.4 | Nearest hit wins | IR-4.3.1 |
| TC-IR-4.3.2.1 | World position projects to screen | IR-4.3.2 |
| TC-IR-4.3.2.2 | Off-screen clamped when flagged | IR-4.3.2 |
| TC-IR-4.3.2.3 | Visible false behind geometry | IR-4.3.2 |
| TC-IR-4.3.2.4 | Batch projection order stable | IR-4.3.2 |
| TC-IR-4.3.3.1 | Hover with `TooltipComponent` shows | IR-4.3.3 |
| TC-IR-4.3.3.2 | Hover without `TooltipComponent` hides | IR-4.3.3 |
| TC-IR-4.3.3.3 | Hover delay respected | IR-4.3.3 |
| TC-IR-4.3.4.1 | Diegetic panel behind wall hidden | IR-4.3.4 |
| TC-IR-4.3.5.1 | Reticle snaps to nearest target | IR-4.3.5 |
| TC-IR-4.3.5.2 | Snap confidence falls with distance | IR-4.3.5 |
| TC-IR-4.3.5.3 | No target in cone returns `None` | IR-4.3.5 |

## Negative Tests

| ID | Test | Req |
|----|------|-----|
| TC-IR-4.3.1.N1 | Cursor outside window (FM-1) | IR-4.3.1 |
| TC-IR-4.3.1.N2 | Physics BVH not built (FM-2) | IR-4.3.1 |
| TC-IR-4.3.2.N1 | Unknown camera id (FM-6) | IR-4.3.2 |
| TC-IR-4.3.2.N2 | Depth readback stale (FM-3) | IR-4.3.2 |
| TC-IR-4.3.1.N3 | CH-27 overflow drops (FM-4) | IR-4.3.1 |

### Test case details

1. **TC-IR-4.3.1.1** -- Input: cursor NDC `(0, 0)`, camera looking down -Z, cube at `(0,0,-5)`.
   Expected: `entity` matches cube; `world_position.z ≈ -5`; `normal ≈ (0,0,1)`.
2. **TC-IR-4.3.1.2** -- Input: cursor aimed into empty space. Expected: `entity=None`; downstream
   tooltip hidden.
3. **TC-IR-4.3.1.3** -- Input: ray_mask excluding layer `L_ENEMY`; only enemy in path. Expected:
   `entity=None`.
4. **TC-IR-4.3.1.4** -- Input: two boxes at z=-3 and z=-5. Expected: returned entity is the nearer
   (z=-3).
5. **TC-IR-4.3.2.1** -- Input: world pos `(0,1,-5)`, identity view, perspective matrix. Expected:
   screen near center top; `visible=true`.
6. **TC-IR-4.3.2.2** -- Input: world pos far off-screen right, `ClampToScreen` flag. Expected:
   `screen_position.x == window_width`.
7. **TC-IR-4.3.2.3** -- Input: world pos behind a wall relative to camera, `NeedVisibility` flag.
   Expected: `visible=false`.
8. **TC-IR-4.3.2.4** -- Input: 64 projection requests. Expected: results in the same order as
   requests.
9. **TC-IR-4.3.3.1** -- Input: cursor on entity with `TooltipComponent`. Expected: UI publishes
   tooltip render entry after `hover_delay_s`.
10. **TC-IR-4.3.3.2** -- Input: cursor on entity without `TooltipComponent`. Expected: no tooltip
    render entry.
11. **TC-IR-4.3.3.3** -- Input: cursor hovers for `hover_delay_s - 0.01` then moves. Expected: no
    tooltip published.
12. **TC-IR-4.3.4.1** -- Input: diegetic panel placed behind a wall. Expected: `visible=false` per
    depth test; UI skips rendering.
13. **TC-IR-4.3.5.1** -- Input: two aimable entities, left entity closer to cursor. Expected:
    `ReticleSnap.target` = left entity.
14. **TC-IR-4.3.5.2** -- Input: target at 2 m vs 10 m from cursor center. Expected: confidence
    higher at 2 m.
15. **TC-IR-4.3.5.3** -- Input: no entities in snap cone. Expected: `ReticleSnap.target=None`.
16. **TC-IR-4.3.1.N1** -- Input: cursor NDC outside [-1, 1]. Expected: result `entity=None`; `FM-1`
    counter increments.
17. **TC-IR-4.3.1.N2** -- Input: pick before physics BVH is built. Expected: `entity=None`; `FM-2`
    counter increments.
18. **TC-IR-4.3.2.N1** -- Input: `WorldProjectRequest` with unknown `CameraId`. Expected:
    `visible=false`, `screen_position=Vec2::ZERO`; `FM-6` counter increments.
19. **TC-IR-4.3.2.N2** -- Input: depth readback not updated for >1 frame. Expected: visibility uses
    previous frame depth; `FM-3` counter increments.
20. **TC-IR-4.3.1.N3** -- Input: 32 pick requests against `CH-27` cap=8. Expected: 24 oldest
    dropped; `FM-4` counter increments by 24.

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-4.3.1.B1 | 10 cursor picks | < 0.05 ms | IR-4.3.1 |
| TC-IR-4.3.2.B1 | 64 world projections | < 0.05 ms | IR-4.3.2 |
| TC-IR-4.3.3.B1 | 10 tooltip resolves | < 0.02 ms | IR-4.3.3 |
| TC-IR-4.3.5.B1 | Reticle snap scan 32 targets | < 0.02 ms | IR-4.3.5 |

All benchmarks run under `cargo bench` in CI; thresholds enforced via the benchmark harness.
