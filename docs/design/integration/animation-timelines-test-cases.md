# Animation ↔ Timelines Integration Test Cases

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-1.5.1.1 | Float writes param | Track f32 at t=1.0 | Param value set | IR-1.5.1 |
| TC-IR-1.5.1.2 | Bool writes trigger | Track true at t=2.0 | Trigger fired | IR-1.5.1 |
| TC-IR-1.5.1.3 | Interpolated val | Bezier at t=0.5 | Smooth interp value | IR-1.5.1 |
| TC-IR-1.5.2.1 | Override blocks GP | CinematicOverride on | GP params ignored | IR-1.5.2 |
| TC-IR-1.5.2.2 | Override removed | CinematicOverride off | GP params active | IR-1.5.2 |
| TC-IR-1.5.2.3 | Override no delay | Add override frame N | Active same frame N | IR-1.5.2 |
| TC-IR-1.5.3.1 | Blend in ramps | blend_in = 0.5s | Weight 0->1 in 0.5s | IR-1.5.3 |
| TC-IR-1.5.3.2 | Blend out ramps | blend_out = 0.3s | Weight 1->0 in 0.3s | IR-1.5.3 |
| TC-IR-1.5.3.3 | Mid-blend smooth | Weight at 0.5 | No pop between poses | IR-1.5.3 |
| TC-IR-1.5.4.1 | Event montage | TimelineEvent at t=3 | ActiveMontage added | IR-1.5.4 |
| TC-IR-1.5.4.2 | End of timeline | Event at final frame | Montage added once | IR-1.5.4 |
| TC-IR-1.5.4.3 | Multiple tracks | Events on 3 tracks | 3 montages added | IR-1.5.4 |
| TC-IR-1.5.4.4 | Mid-blend trigger | Event during blend_in | Montage respects weight | IR-1.5.4 |
| TC-IR-1.5.5.1 | Vec3 animates pos | Track Vec3 over 2s | Component updated | IR-1.5.5 |
| TC-IR-1.5.5.2 | Quat animates rot | Track Quat slerp | Smooth rotation | IR-1.5.5 |
| TC-IR-1.5.5.3 | Bypasses ParamMap | Vec3 track evaluated | ParamMap untouched | IR-1.5.5 |

1. **TC-IR-1.5.2.3** verifies no one-frame delay: `CinematicOverride` is inserted on frame N and the
   bridge system must show `blend_weight > 0` and gameplay params blocked on the same frame.
2. **TC-IR-1.5.4.2** verifies idempotent end-of-timeline trigger: the montage must be inserted
   exactly once when `current_time` reaches `duration`.
3. **TC-IR-1.5.4.3** verifies per-track independence: three simultaneous `TimelineEvent`s on
   different tracks must each produce an `ActiveMontage` component.
4. **TC-IR-1.5.4.4** verifies that a montage trigger fired while `CinematicBlendState.blend_weight`
   is between 0 and 1 still produces an `ActiveMontage`; the cross-fade weight applies at the
   `AnimationLayer` stage and does not suppress the insert.
5. **TC-IR-1.5.5.3** verifies routing: `ParameterMap` must remain unchanged when only property curve
   tracks are present -- only the property curve bridge writes occur.

## Negative Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-1.5.N.1 | No ParameterMap | Override on entity w/o PM | Skip, log error | IR-1.5.2 |
| TC-IR-1.5.N.2 | No StateInstance | Override on entity w/o SI | Skip, log error | IR-1.5.2 |
| TC-IR-1.5.N.3 | Type mismatch | Track f32 -> Bool param | Skip, log warn | IR-1.5.1 |
| TC-IR-1.5.N.4 | Asset not loaded | Timeline id not in Assets | Skip, log error | IR-1.5.1 |
| TC-IR-1.5.N.5 | Weight out of range | blend_weight = 1.3 | Clamped to 1.0 | IR-1.5.3 |
| TC-IR-1.5.N.6 | Curve target gone | Vec3 track, entity gone | Skip track, log warn | IR-1.5.5 |
| TC-IR-1.5.N.7 | Montage asset gone | Event refs missing asset | Skip insert, log warn | IR-1.5.4 |
| TC-IR-1.5.N.8 | Guard blocks GP write | PG set, GP writes param | ParameterMap unchanged | IR-1.5.2 |

1. All negative tests are CI-runnable via `cargo test` -- they use real `World`/`Assets` fixtures
   with empty or mismatched data, no mocks.

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-1.5.1.B1 | 32-track timeline eval | < 0.5 ms | IR-1.5.1 |
| TC-IR-1.5.3.B1 | Blend weight update 256 actors | < 0.2 ms | IR-1.5.3 |
| TC-IR-1.5.4.B1 | Montage trigger scan 64 tracks | < 0.3 ms | IR-1.5.4 |
| TC-IR-1.5.5.B1 | 1000 property curves | < 0.5 ms | IR-1.5.5 |

1. Benchmarks run via `cargo bench` on CI; each uses real ECS `World` plus authored timeline assets.
   No mocks.
