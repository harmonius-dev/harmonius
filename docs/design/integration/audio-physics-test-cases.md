# Audio ↔ Physics Integration Test Cases

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-1.8.1.1 | Collision sound | Box hits floor | Play enqueued | IR-1.8.1 |
| TC-IR-1.8.1.2 | Contact point pos | Hit at (3,0,0) | Sound at (3,0,0) | IR-1.8.1 |
| TC-IR-1.8.1.3 | Below threshold | Impulse < thresh | No command enqueued | IR-1.8.1 |
| TC-IR-1.8.2.1 | Metal on wood | Metal + Wood mats | Metal-wood clip | IR-1.8.2 |
| TC-IR-1.8.2.2 | Unknown fallback | Unknown + Unknown | Default sound set | IR-1.8.2 |
| TC-IR-1.8.2.3 | Pair order same | (Wood,Metal) pair | Same sound set | IR-1.8.2 |
| TC-IR-1.8.3.1 | Hard hit loud | Impulse 100.0 | Gain near 1.0 | IR-1.8.3 |
| TC-IR-1.8.3.2 | Soft tap quiet | Impulse 5.0 | Gain near 0.1 | IR-1.8.3 |
| TC-IR-1.8.3.3 | Pitch scales vel | Impulse 80.0 | Pitch > 1.0 | IR-1.8.3 |
| TC-IR-1.8.4.1 | Trigger enter | Enter ReverbZone | Reverb zone active | IR-1.8.4 |
| TC-IR-1.8.4.2 | Trigger exit | Exit ReverbZone | Reverb zone inactive | IR-1.8.4 |
| TC-IR-1.8.5.1 | Sliding friction | Persisted + tang | Friction sound plays | IR-1.8.5 |
| TC-IR-1.8.5.2 | Slide stop ends | CollisionEnded | Friction stops | IR-1.8.5 |
| TC-IR-1.8.5.3 | Slide speed gain | Fast slide | Higher gain | IR-1.8.5 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-1.8.1.B1 | 500 impacts/frame | < 0.5 ms | IR-1.8.1 |
| TC-IR-1.8.2.B1 | Material table lookup | < 0.001 ms | IR-1.8.2 |
| TC-IR-1.8.5.B1 | 100 friction sources | < 0.1 ms | IR-1.8.5 |
