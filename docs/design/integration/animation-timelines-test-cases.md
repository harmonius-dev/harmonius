# Animation ↔ Timelines Integration Test Cases

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-1.5.1.1 | Float writes param | Track f32 at t=1.0 | Param value set | IR-1.5.1 |
| TC-IR-1.5.1.2 | Bool writes trigger | Track true at t=2.0 | Trigger fired | IR-1.5.1 |
| TC-IR-1.5.1.3 | Interpolated val | Bezier at t=0.5 | Smooth interp value | IR-1.5.1 |
| TC-IR-1.5.2.1 | Override blocks GP | CinematicOverride on | GP params ignored | IR-1.5.2 |
| TC-IR-1.5.2.2 | Override removed | CinematicOverride off | GP params active | IR-1.5.2 |
| TC-IR-1.5.3.1 | Blend in ramps | blend_in = 0.5s | Weight 0->1 in 0.5s | IR-1.5.3 |
| TC-IR-1.5.3.2 | Blend out ramps | blend_out = 0.3s | Weight 1->0 in 0.3s | IR-1.5.3 |
| TC-IR-1.5.3.3 | Mid-blend smooth | Weight at 0.5 | No pop between poses | IR-1.5.3 |
| TC-IR-1.5.4.1 | Event montage | TimelineEvent at t=3 | ActiveMontage added | IR-1.5.4 |
| TC-IR-1.5.5.1 | Vec3 animates pos | Track Vec3 over 2s | Component updated | IR-1.5.5 |
| TC-IR-1.5.5.2 | Quat animates rot | Track Quat slerp | Smooth rotation | IR-1.5.5 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-1.5.1.B1 | 32-track timeline eval | < 0.5 ms | IR-1.5.1 |
| TC-IR-1.5.5.B1 | 1000 property curves | < 0.5 ms | IR-1.5.5 |
