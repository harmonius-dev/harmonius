# AI ↔ Physics Integration Test Cases

All tests are CI-runnable on the CPU. Physics and BVH queries use fake in-memory scenes (small cube
worlds, staircases, procedural slopes) constructed in each test.

## Integration Tests

| ID | Test | Req |
|----|------|-----|
| TC-IR-2.5.1.1 | Walkable flat ground returns `walkable=true` | IR-2.5.1 |
| TC-IR-2.5.1.2 | Slope 70° returns `SlopeTooSteep` | IR-2.5.1 |
| TC-IR-2.5.1.3 | No surface below returns `NoSurface` | IR-2.5.1 |
| TC-IR-2.5.1.4 | Material excluded returns `MaterialExcluded` | IR-2.5.1 |
| TC-IR-2.5.1.5 | Request id round-trips through MPSC | IR-2.5.1 |
| TC-IR-2.5.2.1 | Unobstructed jump returns landing | IR-2.5.2 |
| TC-IR-2.5.2.2 | Wall in flight path blocks at segment N | IR-2.5.2 |
| TC-IR-2.5.2.3 | Ceiling hit yields no landing | IR-2.5.2 |
| TC-IR-2.5.2.4 | Arc respects gravity vector | IR-2.5.2 |
| TC-IR-2.5.3.1 | Ledge at 1.2 m returns ledge_top | IR-2.5.3 |
| TC-IR-2.5.3.2 | Ledge above max_height returns `None` | IR-2.5.3 |
| TC-IR-2.5.3.3 | Curved wall capsule sweep correct | IR-2.5.3 |
| TC-IR-2.5.4.1 | Neighbors within radius listed | IR-2.5.4 |
| TC-IR-2.5.4.2 | Self entity excluded | IR-2.5.4 |
| TC-IR-2.5.4.3 | More than 16 neighbors truncated (FM-6) | IR-2.5.4 |
| TC-IR-2.5.4.4 | NeighborState velocity matches physics body | IR-2.5.4 |
| TC-IR-2.5.5.1 | AiGroundedState true when contact present | IR-2.5.5 |
| TC-IR-2.5.5.2 | AiGroundedState false when airborne | IR-2.5.5 |
| TC-IR-2.5.5.3 | Ground normal matches contact | IR-2.5.5 |
| TC-IR-2.5.6.1 | LOS unobstructed returns visible | IR-2.5.6 |
| TC-IR-2.5.6.2 | LOS behind wall returns occluded | IR-2.5.6 |

## Negative Tests

| ID | Test | Req |
|----|------|-----|
| TC-IR-2.5.1.N1 | BVH not built yet -> FM-1 | IR-2.5.1 |
| TC-IR-2.5.2.N1 | Segment count 0 -> no hit | IR-2.5.2 |
| TC-IR-2.5.3.N1 | Inverted capsule direction -> panic-free | IR-2.5.3 |
| TC-IR-2.5.4.N1 | Empty scene -> empty NeighborState list | IR-2.5.4 |
| TC-IR-2.5.1.N2 | CH-26 full drops oldest (FM-3) | IR-2.5.1 |
| TC-IR-2.5.5.N1 | Missing contact list -> prev grounded state (FM-4) | IR-2.5.5 |

### Test case details

1. **TC-IR-2.5.1.1** -- Input: flat ground plane at y=0, foot_position `(0, 0.01, 0)`,
   max_slope_deg=45. Expected: `walkable=true`, `surface_normal=(0,1,0)`, `reason=None`.
2. **TC-IR-2.5.1.2** -- Input: plane rotated 70° about X, foot at plane surface. Expected:
   `walkable=false`, `reason=SlopeTooSteep`.
3. **TC-IR-2.5.1.3** -- Input: foot high above any geometry. Expected: `walkable=false`,
   `reason=NoSurface`.
4. **TC-IR-2.5.1.4** -- Input: foot above a "lava" surface with `agent_mask` excluding lava.
   Expected: `walkable=false`, `reason=MaterialExcluded`.
5. **TC-IR-2.5.1.5** -- Input: 64 queries with distinct `request_id`s. Expected: all replies carry
   matching request_id.
6. **TC-IR-2.5.2.1** -- Input: `initial_velocity=(5,5,0)`, segment_count=16, no obstructions.
   Expected: `landing` = last segment endpoint under the ground plane; `blocked_segment=0`.
7. **TC-IR-2.5.2.2** -- Input: wall at x=3; jump starting at (0,0,0) with velocity reaching wall.
   Expected: `blocked_segment` matches first segment crossing x=3; `landing=None`.
8. **TC-IR-2.5.2.3** -- Input: low ceiling above jump arc apex. Expected: `landing=None`;
   `blocked_segment` at the ceiling hit.
9. **TC-IR-2.5.2.4** -- Input: gravity `(0,-9.81,0)`. Expected: computed arc y-values match
   analytical formula `y = v_y*t - 0.5*g*t^2` within 1e-3.
10. **TC-IR-2.5.3.1** -- Input: wall with a ledge at 1.2 m relative to agent, capsule radius 0.3,
    max_height 1.5. Expected: `ledge_top.y ≈ agent_y + 1.2`; `climb_height ≈ 1.2`.
11. **TC-IR-2.5.3.2** -- Input: ledge at 2.0 m, max_height 1.5. Expected: `ledge_top=None`.
12. **TC-IR-2.5.3.3** -- Input: cylinder wall; capsule sweep must find tangent ledge. Expected:
    result matches analytical tangent position.
13. **TC-IR-2.5.4.1** -- Input: 10 dynamic bodies within radius, 5 outside. Expected:
    `AvoidanceResult.neighbors.len() == 10`.
14. **TC-IR-2.5.4.2** -- Input: self entity inside radius. Expected: not in neighbors list.
15. **TC-IR-2.5.4.3** -- Input: 32 dynamic bodies within radius. Expected: 16 nearest retained by
    distance ascending; FM-6 counter increments.
16. **TC-IR-2.5.4.4** -- Input: body with `RigidBody::Dynamic { linvel: (3,0,0) }`. Expected:
    `NeighborState.velocity == (3,0,0)`.
17. **TC-IR-2.5.5.1** -- Input: contact between agent capsule and floor. Expected:
    `AiGroundedState.grounded=true`, normal=(0,1,0), `ground_entity` matches floor.
18. **TC-IR-2.5.5.2** -- Input: no contacts for agent. Expected: `grounded=false`.
19. **TC-IR-2.5.5.3** -- Input: contact on slope with normal `(0, 0.7, 0.7).normalize()`. Expected:
    `AiGroundedState.ground_normal` matches.
20. **TC-IR-2.5.6.1** -- Input: observer and target with clear LOS. Expected: LOS query returns
    visible; no hit entity.
21. **TC-IR-2.5.6.2** -- Input: wall between observer and target. Expected: LOS query returns
    occluded; hit entity = wall.
22. **TC-IR-2.5.1.N1** -- Input: query issued before spatial index build. Expected:
    `walkable=false`, `reason=NoSurface`, `FM-1` counter increments.
23. **TC-IR-2.5.2.N1** -- Input: `segment_count=0`. Expected: `landing=None`; no physics calls.
24. **TC-IR-2.5.3.N1** -- Input: `direction=(0,0,0)`. Expected: early-return with `ledge_top=None`,
    no panic.
25. **TC-IR-2.5.4.N1** -- Input: empty scene. Expected: `neighbors.len()==0`.
26. **TC-IR-2.5.1.N2** -- Input: 400 queries enqueued against CH-26 cap=256. Expected: 144 oldest
    dropped; `FM-3` counter increments by 144.
27. **TC-IR-2.5.5.N1** -- Input: no contact list this frame. Expected: previous `AiGroundedState`
    reused; `FM-4` counter increments.

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-2.5.1.B1 | 1000 walkability queries | < 0.5 ms | IR-2.5.1 |
| TC-IR-2.5.2.B1 | 100 jump arcs × 16 segments | < 0.3 ms | IR-2.5.2 |
| TC-IR-2.5.3.B1 | 100 climb sweeps | < 0.1 ms | IR-2.5.3 |
| TC-IR-2.5.4.B1 | 500 avoidance enumerations radius 5 m | < 0.5 ms | IR-2.5.4 |
| TC-IR-2.5.5.B1 | 500 grounded state reads from contact list | < 0.05 ms | IR-2.5.5 |

All benchmarks run under `cargo bench` in CI; thresholds enforced via the benchmark harness.
