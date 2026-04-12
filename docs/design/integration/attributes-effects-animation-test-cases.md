# Attributes/Effects ↔ Animation Integration Test Cases

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-2.5.1.1 | Slow effect halves speed | 0.5x slow modifier | Player.speed = 0.5 | IR-2.5.1 |
| TC-IR-2.5.1.2 | Haste effect doubles speed | 2.0x haste modifier | Player.speed = 2.0 | IR-2.5.1 |
| TC-IR-2.5.1.3 | Stacked speed effects | 0.5x + 1.5x modifiers | Player.speed = 0.75 | IR-2.5.1 |
| TC-IR-2.5.1.4 | Speed effect expires | Slow removed | Player.speed = 1.0 | IR-2.5.1 |
| TC-IR-2.5.2.1 | Low health triggers anim | Health drops to 20% | Transition to wounded | IR-2.5.2 |
| TC-IR-2.5.2.2 | Health recovers above | Health rises to 50% | Transition to normal | IR-2.5.2 |
| TC-IR-2.5.3.1 | Fatigue drives blend | Fatigue attr = 0.7 | Tired blend weight 0.7 | IR-2.5.3 |
| TC-IR-2.5.3.2 | Zero fatigue normal | Fatigue attr = 0.0 | Normal blend weight 1.0 | IR-2.5.3 |
| TC-IR-2.5.4.1 | Freeze triggers anim | Freeze effect applied | Freeze clip plays | IR-2.5.4 |
| TC-IR-2.5.4.2 | Stun triggers anim | Stun effect applied | Stun clip plays once | IR-2.5.4 |
| TC-IR-2.5.5.1 | Hit frame applies damage | Attack anim at t=0.4 | Target takes damage | IR-2.5.5 |
| TC-IR-2.5.5.2 | Hit frame on despawned | Target entity gone | No crash, log warning | IR-2.5.5 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-2.5.1.B1 | 1000 speed syncs | < 0.1 ms | IR-2.5.1 |
| TC-IR-2.5.2.B1 | 500 threshold checks | < 0.05 ms | IR-2.5.2 |
| TC-IR-2.5.5.B1 | 200 anim event effects | < 0.2 ms | IR-2.5.5 |
