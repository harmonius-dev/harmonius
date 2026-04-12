# Networking ↔ Physics Integration Test Cases

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-4.5.1.1 | Server physics state replicated | Server moves body | Client receives new position | IR-4.5.1 |
| TC-IR-4.5.1.2 | Client cannot modify server body | Client writes Velocity | Server ignores, overwrites | IR-4.5.1 |
| TC-IR-4.5.2.1 | Prediction applies local input | Input at tick T | Local body moves immediately | IR-4.5.2 |
| TC-IR-4.5.2.2 | Prediction uses same substep | Client substep pipeline | Matches server pipeline | IR-4.5.2 |
| TC-IR-4.5.3.1 | Rollback corrects mismatch | Server pos differs by 0.5 | State corrected after replay | IR-4.5.3 |
| TC-IR-4.5.3.2 | Rollback replays N ticks | Mismatch at T, current T+5 | 5 ticks resimulated | IR-4.5.3 |
| TC-IR-4.5.3.3 | Rollback cap prevents spike | Mismatch at T-20 | Max 10 ticks replayed | IR-4.5.3 |
| TC-IR-4.5.4.1 | Cross-platform determinism | Same input, Win+Mac+Linux | Bit-identical positions | IR-4.5.4 |
| TC-IR-4.5.4.2 | No fast-math in physics | Compile flags check | ffast-math disabled | IR-4.5.4 |
| TC-IR-4.5.5.1 | Hitbox rewind to past tick | Rewind 100 ms | Colliders at past positions | IR-4.5.5 |
| TC-IR-4.5.5.2 | Raycast hits rewound hitbox | Aim at past position | Hit confirmed | IR-4.5.5 |
| TC-IR-4.5.5.3 | Raycast misses current pos | Aim at past, target moved | Miss at current, hit rewound | IR-4.5.5 |
| TC-IR-4.5.6.1 | Interpolation smooths remote | Two snapshots 50 ms apart | Smooth position between them | IR-4.5.6 |
| TC-IR-4.5.6.2 | Extrapolation extends velocity | Snapshot late by 1 tick | Position extrapolated | IR-4.5.6 |
| TC-IR-4.5.7.1 | Physics snapshot captures all | Snapshot at tick T | pos, rot, vel, angvel stored | IR-4.5.7 |
| TC-IR-4.5.7.2 | Snapshot restore is exact | Restore tick T snapshot | State == original tick T | IR-4.5.7 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-4.5.3.B1 | Rollback 10 ticks, 100 bodies | < 4 ms | IR-4.5.3 |
| TC-IR-4.5.5.B1 | Hitbox rewind 64 entities | < 0.2 ms | IR-4.5.5 |
| TC-IR-4.5.6.B1 | Interpolation 1000 remote bodies | < 0.3 ms | IR-4.5.6 |
| TC-IR-4.5.7.B1 | Snapshot capture 2000 bodies | < 0.5 ms | IR-4.5.7 |
