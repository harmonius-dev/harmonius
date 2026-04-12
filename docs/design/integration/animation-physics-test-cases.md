# Animation ↔ Physics Integration Test Cases

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-1.3.1.1 | Ragdoll init from pose | Trigger ragdoll | Bodies match bone poses | IR-1.3.1 |
| TC-IR-1.3.1.2 | Velocity inheritance | Running + ragdoll | Bodies carry momentum | IR-1.3.1 |
| TC-IR-1.3.1.3 | Anim stops on ragdoll | RagdollActive added | AnimationPlayer paused | IR-1.3.1 |
| TC-IR-1.3.2.1 | Hit box follows bone | Swing arm anim | Collider at hand pos | IR-1.3.2 |
| TC-IR-1.3.2.2 | Shield collider tracks | Block anim | Shield shape at forearm | IR-1.3.2 |
| TC-IR-1.3.3.1 | Root motion moves char | Walk clip with root | CC moves forward | IR-1.3.3 |
| TC-IR-1.3.3.2 | Root rotation turns char | Turn clip | CC rotates matching | IR-1.3.3 |
| TC-IR-1.3.4.1 | Recovery blends to anim | Recovery start | Blend 0->1 over duration | IR-1.3.4 |
| TC-IR-1.3.4.2 | Recovery plays get-up | Recovery complete | Get-up clip finishes | IR-1.3.4 |
| TC-IR-1.3.5.1 | Weapon collider on/off | HitWindow active | Layers enabled in window | IR-1.3.5 |
| TC-IR-1.3.5.2 | Weapon tracks hand bone | Swing anim | Collider at bone pos | IR-1.3.5 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-1.3.1.B1 | 50 ragdoll inits | < 1 ms | IR-1.3.1 |
| TC-IR-1.3.2.B1 | 500 bone collider syncs | < 0.5 ms | IR-1.3.2 |
| TC-IR-1.3.3.B1 | 200 root motion applies | < 0.1 ms | IR-1.3.3 |
