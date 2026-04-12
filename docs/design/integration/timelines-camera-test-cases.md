# Timelines ↔ Camera Integration Test Cases

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-4.8.1.1 | Position track drives camera | Vec3 track at t=1.0 | Recomposer.position = sampled | IR-4.8.1 |
| TC-IR-4.8.1.2 | Position interpolates linearly | Linear interp, t=0.5 | Midpoint between keyframes | IR-4.8.1 |
| TC-IR-4.8.2.1 | Rotation track drives camera | Quat track at t=1.0 | Recomposer.rotation = sampled | IR-4.8.2 |
| TC-IR-4.8.2.2 | Rotation slerps correctly | Two quat KFs, t=0.5 | Slerp midpoint | IR-4.8.2 |
| TC-IR-4.8.3.1 | FOV track animates projection | F32 track 60 to 90 | CameraOutput.fov changes | IR-4.8.3 |
| TC-IR-4.8.3.2 | FOV via modifier stack | FovOverride modifier | Overrides base FOV | IR-4.8.3 |
| TC-IR-4.8.4.1 | Recomposer blend weight 0 | blend_weight=0.0 | Full gameplay camera | IR-4.8.4 |
| TC-IR-4.8.4.2 | Recomposer blend weight 1 | blend_weight=1.0 | Full timeline camera | IR-4.8.4 |
| TC-IR-4.8.4.3 | Recomposer blend weight 0.5 | blend_weight=0.5 | 50/50 mix | IR-4.8.4 |
| TC-IR-4.8.5.1 | Sequencer plays timeline entry | Entry 0 active | Associated TL plays | IR-4.8.5 |
| TC-IR-4.8.5.2 | Sequencer advances entries | Entry 0 completes | Entry 1 starts | IR-4.8.5 |
| TC-IR-4.8.6.1 | Cinematic enter blends | Priority 10 to 100 | Smooth EaseInOut blend | IR-4.8.6 |
| TC-IR-4.8.6.2 | Cinematic exit blends back | Priority 100 to 0 | Smooth blend to gameplay | IR-4.8.6 |
| TC-IR-4.8.6.3 | No pop on enter/exit | Enter + exit | No discontinuity in output | IR-4.8.6 |
| TC-IR-4.8.7.1 | Dolly track 0.0 at spline start | F32=0.0 | Camera at spline origin | IR-4.8.7 |
| TC-IR-4.8.7.2 | Dolly track 1.0 at spline end | F32=1.0 | Camera at spline terminus | IR-4.8.7 |
| TC-IR-4.8.7.3 | Dolly track clamped | F32=1.5 | Clamped to 1.0 | IR-4.8.7 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-4.8.1.B1 | 8-track cinematic camera eval | < 0.1 ms | IR-4.8.1 |
| TC-IR-4.8.5.B1 | Sequencer with 16 entries | < 0.05 ms | IR-4.8.5 |
| TC-IR-4.8.6.B1 | Blend computation per frame | < 0.02 ms | IR-4.8.6 |
