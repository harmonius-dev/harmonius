# Animation ↔ Physics Integration Test Cases

All test cases are CI-runnable under `cargo test` with no external services or GPU required.

## Unit Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-1.3.1.U1 | RagdollDef rkyv round-trip | Serialize/deserialize | Equal struct | IR-1.3.1 |
| TC-IR-1.3.1.U2 | RagdollBone handle lookup | Handle into store | Returns bone def | IR-1.3.1 |
| TC-IR-1.3.3.U1 | RootMotionDelta compose | Two deltas, SLERP | Combined delta | IR-1.3.3 |
| TC-IR-1.3.3.U2 | Delta consumed + cleared | Apply once | Delta reset to identity | IR-1.3.3 |
| TC-IR-1.3.4.U1 | SLERP blend weight ramp | t=0 to t=1 linear | Monotonic weight | IR-1.3.4 |
| TC-IR-1.3.4.U2 | Per-bone velocity clamp | lin 200, ang 100 | Clamp to 100, 50 | IR-1.3.4 |
| TC-IR-1.3.1.U3 | Cone-twist limit | Rot beyond swing | Clamped to limit | IR-1.3.1 |

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
| TC-IR-1.3.3.3 | Zero-frame root latency | Delta written frame N | Applied Phase 5 N+1 | IR-1.3.3 |
| TC-IR-1.3.2.3 | Zero-frame bone sync | Bones written frame N | Collider Phase 5 N+1 | IR-1.3.2 |

## Negative Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-1.3.1.N1 | RagdollDef missing bone | Def skips bone 5 | Warning, bone frozen | IR-1.3.1 |
| TC-IR-1.3.1.N2 | Invalid RagdollRef handle | Stale gen index | Error logged, no init | IR-1.3.1 |
| TC-IR-1.3.3.N1 | Root motion on sleeping | Sleep body + delta | wake_body called | IR-1.3.3 |
| TC-IR-1.3.3.N2 | Root motion on static | Static body + delta | Delta discarded, warn | IR-1.3.3 |
| TC-IR-1.3.4.N1 | Missing recovery clip | Recovery triggered | Snap to bind, warn | IR-1.3.4 |
| TC-IR-1.3.2.N1 | BonePaletteGpu missing | Entity without palette | Sync skipped, warn | IR-1.3.2 |
| TC-IR-1.3.4.N2 | Constraint violation | Huge impulse applied | Velocity clamped | IR-1.3.4 |
| TC-IR-1.3.4.N3 | Swing beyond cone limit | Rot exceeds limit | Clamped, no explode | IR-1.3.4 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-1.3.1.B1 | 50 ragdoll inits | < 1 ms | IR-1.3.1 |
| TC-IR-1.3.2.B1 | 500 bone collider syncs | < 0.5 ms | IR-1.3.2 |
| TC-IR-1.3.3.B1 | 200 root motion applies | < 0.1 ms | IR-1.3.3 |
