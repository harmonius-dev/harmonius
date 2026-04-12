# Timelines ↔ Camera Integration Test Cases

All test cases are CI-runnable under `cargo test` with no GPU, audio device, or external services
required. The binding, sequencer, and blend systems are pure ECS systems exercised with real
(non-mock) `MultiTrackTimeline` assets, `PlaybackState` components, `Recomposer` stores, and
`DollyRig` rigs. Fakes are used only where a real collaborator is unavailable; no mock libraries.

## Upstream Requirements Trace

Each integration requirement below corresponds to upstream timeline and camera requirements.
Regressions in any test case below also constitute regressions against the listed upstream IDs.

| IR-ID    | Upstream R-IDs                | Upstream US-IDs        |
|----------|-------------------------------|------------------------|
| IR-4.8.1 | R-17.4.1                      | US-17.4.9              |
| IR-4.8.2 | R-17.4.3                      | US-17.4.7              |
| IR-4.8.3 | R-17.4.6                      | US-17.4.7              |
| IR-4.8.4 | R-13.25.35                    |                        |
| IR-4.8.5 | R-13.25.26                    |                        |
| IR-4.8.6 | R-13.25.36                    |                        |
| IR-4.8.7 | R-13.25.8                     |                        |

## Unit Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-4.8.1.U1 | Vec3 offset sample | Track `(0,0)->(1,(2,3,4))`, t=1 | `(2,3,4)` | IR-4.8.1 |
| TC-IR-4.8.1.U2 | Vec3 lerp mid | Linear, t=0.5 | Midpoint | IR-4.8.1 |
| TC-IR-4.8.2.U1 | Euler Vec3 sample | `(0,0,0)->(90,0,0)`, t=0.5 | `(45,0,0)` | IR-4.8.2 |
| TC-IR-4.8.3.U1 | FOV delta sample | F32 `0->10`, t=0.5 | `5.0` | IR-4.8.3 |
| TC-IR-4.8.4.U1 | Blend clamp low | blend_weight sample = -0.5 | Clamped to 0.0 | IR-4.8.4 |
| TC-IR-4.8.4.U2 | Blend clamp high | blend_weight sample = 1.7 | Clamped to 1.0 | IR-4.8.4 |
| TC-IR-4.8.5.U1 | SeqEntry default ref | Entry with no timeline_ref | `None` | IR-4.8.5 |
| TC-IR-4.8.6.U1 | Blend alpha zero tick | zero-tick frame | Reuse prev snapshot | IR-4.8.6 |
| TC-IR-4.8.6.U2 | Blend alpha multi | 2 fixed ticks in 1 frame | Lerp to latest | IR-4.8.6 |
| TC-IR-4.8.7.U1 | Dolly clamp high | F32 = 1.5 | Clamped to 1.0 | IR-4.8.7 |
| TC-IR-4.8.7.U2 | Dolly clamp low | F32 = -0.1 | Clamped to 0.0 | IR-4.8.7 |

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-4.8.1.1 | Pos drives Recomposer | Vec3 at t=1.0 | pos_offset == sample | IR-4.8.1 |
| TC-IR-4.8.1.2 | Pos lerps linearly | Linear, t=0.5 | Midpoint between KFs | IR-4.8.1 |
| TC-IR-4.8.1.3 | Camera-local space | Offset `(1,0,0)` | Recomposer local X+1 | IR-4.8.1 |
| TC-IR-4.8.2.1 | Rot drives Recomposer | Vec3 euler track | rot_offset == sample | IR-4.8.2 |
| TC-IR-4.8.2.2 | Euler lerp mid | Two euler KFs, t=0.5 | Component-wise midpoint | IR-4.8.2 |
| TC-IR-4.8.3.1 | FOV track animates | F32 0 to 30 | fov_delta changes | IR-4.8.3 |
| TC-IR-4.8.3.2 | FOV via modifier stack | fov_delta = 10 | Base FOV + 10 | IR-4.8.3 |
| TC-IR-4.8.4.1 | blend_weight 0 | sample = 0.0 | Full gameplay camera | IR-4.8.4 |
| TC-IR-4.8.4.2 | blend_weight 1 | sample = 1.0 | Full timeline camera | IR-4.8.4 |
| TC-IR-4.8.4.3 | blend_weight 0.5 | sample = 0.5 | 50/50 mix | IR-4.8.4 |
| TC-IR-4.8.4.4 | Recomposer authority | Binding writes 0.7 | Recomposer weight = 0.7 | IR-4.8.4 |
| TC-IR-4.8.5.1 | Entry starts timeline | timeline_ref active | PlaybackState playing | IR-4.8.5 |
| TC-IR-4.8.5.2 | Entry advances | Entry 0 completes | Entry 1 starts | IR-4.8.5 |
| TC-IR-4.8.5.3 | No timeline_ref preserved | Entry with `None` | Behavior unchanged | IR-4.8.5 |
| TC-IR-4.8.6.1 | Cinematic enter blends | Pri 10 to 100 | Smooth EaseInOut blend | IR-4.8.6 |
| TC-IR-4.8.6.2 | Cinematic exit blends | Pri 100 to 0 | Smooth blend to GP | IR-4.8.6 |
| TC-IR-4.8.6.3 | No pop on enter/exit | Enter + exit | No output discontinuity | IR-4.8.6 |
| TC-IR-4.8.6.4 | Phase 3->6 alpha lerp | 1 tick per render | Interpolated output | IR-4.8.6 |
| TC-IR-4.8.6.5 | Multi-tick coalesce | 3 ticks in 1 render | Last tick wins | IR-4.8.6 |
| TC-IR-4.8.7.1 | Dolly at spline start | F32 = 0.0 | Camera at origin | IR-4.8.7 |
| TC-IR-4.8.7.2 | Dolly at spline end | F32 = 1.0 | Camera at terminus | IR-4.8.7 |
| TC-IR-4.8.7.3 | Dolly clamped high | F32 = 1.5 | DollyRig.position == 1.0 | IR-4.8.7 |

## Negative Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-4.8.1.N1 | Asset not loaded | Handle, no asset | No write; state held | IR-4.8.1 |
| TC-IR-4.8.1.N2 | Dangling handle | Freed asset slot | Treated as not loaded | IR-4.8.1 |
| TC-IR-4.8.1.N3 | NaN Vec3 sample | Keyframe NaN | Skip write; warn log | IR-4.8.1 |
| TC-IR-4.8.2.N1 | Track id missing | Invalid rot track id | Channel skipped only | IR-4.8.2 |
| TC-IR-4.8.2.N2 | Track type mismatch | Non-Vec3 on rot | Rejected at load | IR-4.8.2 |
| TC-IR-4.8.3.N1 | NaN f32 sample | FOV track NaN | fov_delta unchanged | IR-4.8.3 |
| TC-IR-4.8.4.N1 | Blend weight -5 | OOR sample | Clamped to 0.0 | IR-4.8.4 |
| TC-IR-4.8.4.N2 | Blend weight 9.0 | OOR sample | Clamped to 1.0 | IR-4.8.4 |
| TC-IR-4.8.5.N1 | Entry cam despawned | camera entity gone | Stay on current, warn | IR-4.8.5 |
| TC-IR-4.8.5.N2 | Unexpected loop | LoopMode::Loop | Respect LoopMode | IR-4.8.5 |
| TC-IR-4.8.5.N3 | Paused PlaybackState | playing == false | Held time, stable | IR-4.8.5 |
| TC-IR-4.8.6.N1 | Zero-tick frame | No fixed tick | Prev snapshot reused | IR-4.8.6 |
| TC-IR-4.8.7.N1 | Dolly > 1.0 | F32 = 2.3 | Clamped to 1.0 | IR-4.8.7 |
| TC-IR-4.8.7.N2 | Dolly < 0.0 | F32 = -0.2 | Clamped to 0.0 | IR-4.8.7 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-4.8.1.B1 | 8-track cinematic camera eval | < 0.10 ms | IR-4.8.1 |
| TC-IR-4.8.1.B2 | Binding system 64 entities | < 0.20 ms | IR-4.8.1 |
| TC-IR-4.8.3.B1 | FOV sample + modifier apply | < 0.005 ms | IR-4.8.3 |
| TC-IR-4.8.5.B1 | Sequencer with 16 entries | < 0.05 ms | IR-4.8.5 |
| TC-IR-4.8.6.B1 | Blend computation per frame | < 0.02 ms | IR-4.8.6 |
| TC-IR-4.8.6.B2 | Phase 3->6 alpha lerp | < 0.005 ms | IR-4.8.6 |
| TC-IR-4.8.7.B1 | Dolly sample + spline eval | < 0.01 ms | IR-4.8.7 |

## Sub-story and Variant Trace

The upstream design lists the following refined sub-stories and letter-variant stories. Each is
covered by the parent-ID TC rows above; a regression in any parent TC constitutes a regression
against the listed sub-story or variant.

- US-13.25.26.1
- US-13.25.35.1
- US-13.25.8.1
