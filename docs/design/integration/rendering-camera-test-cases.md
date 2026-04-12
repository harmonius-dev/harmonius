# Rendering ↔ Camera Integration Test Cases

All tests below are **CI-runnable**: pure CPU, no GPU dependency. The render thread is exercised via
a headless harness that drains the MPSC snapshot channel and runs the extractor and PI DRS
controller on a captured frame-time trace.

**Scope note.** 2D/2.5D cameras are intentionally out of scope for this integration (see design
document). No test cases cover 2D cameras here.

## Upstream Requirements Trace

Each integration requirement below corresponds to upstream camera and rendering requirements.
Regressions in any test case below also constitute regressions against the listed upstream IDs.

| IR-ID    | Upstream R-IDs                    |
|----------|-----------------------------------|
| IR-3.1.1 | R-13.25.2, R-2.10.4               |
| IR-3.1.2 | R-13.25.1, R-2.10.4               |
| IR-3.1.3 | R-13.25.36                        |
| IR-3.1.4 | R-2.10.5                          |
| IR-3.1.5 | R-2.3.5, R-2.3.6                  |
| IR-3.1.6 | R-2.3.11                          |

## Integration Tests

| ID | Test | Req |
|----|------|-----|
| TC-IR-3.1.1.1 | Perspective camera produces valid RenderView | IR-3.1.1 |
| TC-IR-3.1.1.2 | Orthographic camera produces valid RenderView | IR-3.1.1 |
| TC-IR-3.1.2.1 | Render layers filter objects (single bit match) | IR-3.1.2 |
| TC-IR-3.1.2.2 | Render layers `0xFF` sees all entities | IR-3.1.2 |
| TC-IR-3.1.3.1 | Single PP volume blends per camera | IR-3.1.3 |
| TC-IR-3.1.3.2 | Overlapping PP volumes, weights sum to one | IR-3.1.3 |
| TC-IR-3.1.3.3 | Nested PP volumes, inner priority wins | IR-3.1.3 |
| TC-IR-3.1.3.4 | Camera on volume boundary, deterministic pick | IR-3.1.3 |
| TC-IR-3.1.3.5 | All cameras outside all volumes, default used | IR-3.1.3 |
| TC-IR-3.1.4.1 | Multi-camera single snapshot produces 2 views | IR-3.1.4 |
| TC-IR-3.1.4.2 | Render order respected (positive ordering) | IR-3.1.4 |
| TC-IR-3.1.4.3 | Negative `render_order` renders before zero | IR-3.1.4 |
| TC-IR-3.1.4.4 | Equal `render_order` stable-sorted by ECS order | IR-3.1.4 |
| TC-IR-3.1.5.1 | ViewUniform matches CameraOutput inverse | IR-3.1.5 |
| TC-IR-3.1.5.2 | Reverse-Z near/far matrix entries correct | IR-3.1.5 |
| TC-IR-3.1.6.1 | DRS scale 0.75 feeds back to viewport | IR-3.1.6 |
| TC-IR-3.1.6.2 | DRS scale 0.0 clamped to `min_scale` | IR-3.1.6 |

### Test Case Details

1. **TC-IR-3.1.1.1** -- Input: `CameraOutput` with `fov_y_radians=1.047` (60 deg), `aspect=16/9`,
   `near=0.1`, `far=1000`, position `(0,5,-10)`, identity rotation. Expected: emitted
   `RenderViewFromCamera.projection[2][2]` approx `0.0`, `[2][3]` approx `0.1`; view-matrix
   translation component equals `(0,-5,10)`.
2. **TC-IR-3.1.1.2** -- Input:
   `Projection::Orthographic { half_height=10, aspect=2, near=-1, far=1 }`. Expected:
   `projection[0][0] = 0.05`, `[1][1] = 0.1`, and the resulting matrix maps `(20, 10, 0)` to NDC
   `(1, 1, z)`.
3. **TC-IR-3.1.2.1** -- Input: camera mask `0x01`, entity A mask `0x01`, entity B mask `0x02`.
   Expected: draw list contains entity A only.
4. **TC-IR-3.1.2.2** -- Input: camera mask `0xFF`, four entities with masks `0x01..0x08`. Expected:
   draw list contains all four.
5. **TC-IR-3.1.3.1** -- Input: two cameras, camera A inside a PP volume with `exposure=2.0`, camera
   B outside all volumes. Expected: A's resolved `exposure = 2.0`, B's `exposure = 1.0` (default).
6. **TC-IR-3.1.3.2** -- Input: camera inside two overlapping volumes, weights `0.6` and `0.4`,
   `exposure` values `1.0` and `3.0`. Expected: resolved `exposure = 1.8` (weighted average).
7. **TC-IR-3.1.3.3** -- Input: outer volume `priority=0`, inner volume `priority=1`, both containing
   the camera, both `weight=1.0`. Expected: resolved parameters equal the inner volume's values
   (higher priority wins the tie).
8. **TC-IR-3.1.3.4** -- Input: camera position lies exactly on a volume's AABB face. Expected: the
   test asserts a deterministic inclusion rule (inclusive on min faces, exclusive on max faces) and
   passes regardless of floating-point noise within `1e-6`.
9. **TC-IR-3.1.3.5** -- Input: one camera, zero volumes. Expected: `PostProcessStack.resolved`
   equals `PostProcessParams::default()`; no log spam (log-once gate honored).
10. **TC-IR-3.1.4.1** -- Input: two `CameraBrain` entities active in the same frame. Expected: two
    `RenderViewFromCamera` messages observed on the MPSC channel in the same snapshot.
11. **TC-IR-3.1.4.2** -- Input: camera A `render_order=0`, camera B `render_order=1`. Expected:
    after extractor sort, A precedes B.
12. **TC-IR-3.1.4.3** -- Input: camera A `render_order=-5` (underlay), camera B `render_order=0`.
    Expected: A precedes B in the sorted draw list.
13. **TC-IR-3.1.4.4** -- Input: three cameras all with `render_order=0` spawned in ECS iteration
    order X, Y, Z. Expected: sorted list preserves order X, Y, Z (stable sort guarantee).
14. **TC-IR-3.1.5.1** -- Input: `CameraOutput` at position `(1,2,3)`, rotation 45 deg about Y.
    Expected: `ViewUniform.view` equals the inverse of the camera's world transform within `1e-5`.
15. **TC-IR-3.1.5.2** -- Input: `near=0.1`, `far=1000`, perspective. Expected: `projection[2][2]` in
    `[-1e-4, 1e-4]`, `projection[2][3]` within `1e-4` of `0.1` (reverse-Z formulation).
16. **TC-IR-3.1.6.1** -- Input: DRS `scale=0.75`, base viewport `1920x1080`. Expected: camera
    viewport updated to `1440x810`; feedback arrives within one frame via the bounded MPSC.
17. **TC-IR-3.1.6.2** -- Input: DRS `scale=0.0`, `min_scale=0.5`. Expected: applied scale clamps to
    `0.5`; viewport always non-zero.

## Negative Tests

| ID | Test | Req |
|----|------|-----|
| TC-IR-3.1.N1 | No active camera, fallback identity view used | IR-3.1.1 |
| TC-IR-3.1.N2 | `fov_y_radians=0` clamped to 1 deg, no NaN | IR-3.1.5 |
| TC-IR-3.1.N3 | Missing PP volume entity, entry skipped | IR-3.1.3 |
| TC-IR-3.1.N4 | Render layer mask `0` warns once, substitutes `0xFFFF_FFFF` | IR-3.1.2 |
| TC-IR-3.1.N5 | Duplicate `render_order` preserves ECS insertion order | IR-3.1.4 |
| TC-IR-3.1.N6 | DRS feedback channel full, oldest dropped, no deadlock | IR-3.1.6 |

### Negative Test Details

1. **TC-IR-3.1.N1** -- Input: no entity has an active `CameraBrain`. Expected: extractor emits a
   single fallback `RenderViewFromCamera` at the origin with identity view, logs exactly once.
2. **TC-IR-3.1.N2** -- Input: `Projection::Perspective { fov_y_radians=0.0, .. }`. Expected: clamped
   to 1 deg in radians; resulting `projection` matrix contains no NaN or infinity entries; log-once
   triggered.
3. **TC-IR-3.1.N3** -- Input: `PostProcessBlend { volume: Entity(BAD), weight: 1.0, priority: 0 }`
   referencing a despawned entity. Expected: the entry is skipped, `resolved` uses defaults, no
   panic.
4. **TC-IR-3.1.N4** -- Input: camera `render_layers = 0`. Expected: substitution to `0xFFFF_FFFF`
   occurs, warning logged exactly once per entity-session, draw list non-empty.
5. **TC-IR-3.1.N5** -- Input: four cameras with `render_order=0` spawned in deterministic ECS order.
   Expected: sorted order equals spawn order across 1000 repeated runs (determinism).
6. **TC-IR-3.1.N6** -- Input: render thread produces 10 `DynamicResolutionState` messages into the
   bounded (capacity 4) feedback channel in one frame. Expected: the oldest 6 are dropped, the
   newest 4 are received by the worker; no deadlock, no panic.

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-3.1.1.B1 | `RenderViewFromCamera` build from `CameraOutput` | < 0.01 ms | IR-3.1.1 |
| TC-IR-3.1.3.B1 | `PostProcessStack` resolve, 8 overlapping volumes | < 0.02 ms | IR-3.1.3 |
| TC-IR-3.1.4.B1 | 4-camera split-screen extraction and sort | < 0.1 ms | IR-3.1.4 |
| TC-IR-3.1.5.B1 | `ViewUniform` GPU upload (4 views) | < 0.05 ms | IR-3.1.5 |
| TC-IR-3.1.6.B1 | DRS PI-controller step | < 0.005 ms | IR-3.1.6 |

Benchmark rationale:

1. **TC-IR-3.1.1.B1** -- The snapshot step runs once per active camera per frame and must stay well
   below the 16.6 ms frame budget.
2. **TC-IR-3.1.3.B1** -- Eight-volume resolve represents the worst case for a dense urban scene with
   stacked atmosphere, interior, and color-grading volumes.
3. **TC-IR-3.1.4.B1** -- Four cameras represent the worst multiplayer split-screen case.
4. **TC-IR-3.1.5.B1** -- GPU upload measured by the headless render harness using a staging buffer
   memcpy; this bounds the CPU side only.
5. **TC-IR-3.1.6.B1** -- The PI controller runs once per frame on the render thread and must be
   effectively free relative to the frame budget.
