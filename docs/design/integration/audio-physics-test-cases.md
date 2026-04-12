# Audio ↔ Physics Integration Test Cases

All tests are CI-runnable: no manual audio verification, no GPU, no external services. Sound output
is asserted by inspecting the MPSC command queue contents, not by listening.

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-1.8.1.1 | Collision sound | Box hits floor | Play enqueued | IR-1.8.1 |
| TC-IR-1.8.1.2 | Contact point pos | Hit at (3,0,0) | Sound at (3,0,0) | IR-1.8.1 |
| TC-IR-1.8.1.3 | Below threshold | Impulse < thresh | No command enqueued | IR-1.8.1 |
| TC-IR-1.8.1.4 | Cooldown suppress | Two hits < cd_sec | Only one Play cmd | IR-1.8.1 |
| TC-IR-1.8.1.5 | Cooldown expiry | Two hits > cd_sec | Two Play cmds | IR-1.8.1 |
| TC-IR-1.8.2.1 | Metal on wood | Metal + Wood mats | Metal-wood clip | IR-1.8.2 |
| TC-IR-1.8.2.2 | Unknown fallback | Unknown + Unknown | Default sound set | IR-1.8.2 |
| TC-IR-1.8.2.3 | Pair order same | (Wood,Metal) pair | Same sound set | IR-1.8.2 |
| TC-IR-1.8.3.1 | Hard hit loud | Impulse 100.0 | Gain near 1.0 | IR-1.8.3 |
| TC-IR-1.8.3.2 | Soft tap quiet | Impulse 5.0 | Gain near 0.1 | IR-1.8.3 |
| TC-IR-1.8.3.3 | Pitch scales vel | Impulse 80.0 | Pitch > 1.0 | IR-1.8.3 |
| TC-IR-1.8.4.1 | Trigger enter | Enter ReverbZone | Reverb zone active | IR-1.8.4 |
| TC-IR-1.8.4.2 | Trigger exit | Exit ReverbZone | Reverb zone inactive | IR-1.8.4 |
| TC-IR-1.8.4.3 | Ambient loop start | Enter AmbientLoop | Play(AMBIENT) enqueued | IR-1.8.4 |
| TC-IR-1.8.4.4 | Ambient loop stop | Exit AmbientLoop | Stop with fade enqueued | IR-1.8.4 |
| TC-IR-1.8.5.1 | Sliding friction | Persisted + tang | Friction sound plays | IR-1.8.5 |
| TC-IR-1.8.5.2 | Slide stop ends | CollisionEnded | Friction stops | IR-1.8.5 |
| TC-IR-1.8.5.3 | Slide speed gain | Fast slide | Higher gain | IR-1.8.5 |
| TC-IR-1.8.5.4 | Slide below tang | tang_speed < 0.01 | No command / stop | IR-1.8.5 |

## Negative Tests

Negative tests verify that the bridge handles invalid or edge-case inputs without enqueuing spurious
commands or panicking.

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-1.8.N1 | Missing material | No PhysicsMaterial | SurfaceType::Default used | IR-1.8.2 |
| TC-IR-1.8.N2 | Empty ImpactSoundSet | No clips for pair | Fallback to default set | IR-1.8.2 |
| TC-IR-1.8.N3 | Queue overflow | 2000 cmds/frame | Low priority dropped | IR-1.8.1 |
| TC-IR-1.8.N4 | Burst underrun | 1000 impacts/frame | Drop-oldest, ctr +1 | IR-1.8.1 |
| TC-IR-1.8.N5 | Zero impulse | impulse = 0.0 | No Play command | IR-1.8.1 |
| TC-IR-1.8.N6 | Self collision | entity_a == entity_b | No Play command | IR-1.8.1 |
| TC-IR-1.8.N7 | Cooldown capacity | 300 pairs | FIFO eviction, no OOB | IR-1.8.1 |
| TC-IR-1.8.N8 | Trigger no zone | Trigger, no component | No command enqueued | IR-1.8.4 |
| TC-IR-1.8.N9 | Unknown VoiceParam | SetParam invalid | Audio thread ignores | IR-1.8.5 |

Detailed test specifications:

1. **TC-IR-1.8.N1** — Entity with collider but no `PhysicsMaterialHandle` component. The bridge must
   substitute `SurfaceType::Default` and look up the default row in `ImpactSoundTable`.
2. **TC-IR-1.8.N2** — `ImpactSoundTable::entries[lo][hi].clips[0]` is `None`. The `get` method
   returns `&self.default`. Play is enqueued with the default clip.
3. **TC-IR-1.8.N3** — Enqueue 2000 `Play` commands in one frame (queue capacity 1024). Assert that
   all `VoicePriority::High` commands survive and `Low` commands are discarded.
4. **TC-IR-1.8.N4** — Simulate audio thread running slow: inject 1000 impacts, verify the underrun
   counter (runtime debug toggle) increments and oldest commands are dropped first.
5. **TC-IR-1.8.N5** — `event.total_impulse == 0.0` triggers early return; queue length unchanged.
6. **TC-IR-1.8.N6** — Degenerate self-contact from physics. Bridge treats as no-op; no Play command.
   Verifies no panic on equal entity IDs.
7. **TC-IR-1.8.N7** — Insert 300 distinct cooldown pairs. `ImpactCooldownTracker` must FIFO-evict
   the oldest 44 without panicking or allocating.
8. **TC-IR-1.8.N8** — `TriggerEnter` event on an entity with neither `ReverbZone` nor `AmbientLoop`
   components. Bridge is a no-op.
9. **TC-IR-1.8.N9** — An invalid `VoiceParam` variant reaches the audio thread (forward-compat). The
   audio thread ignores and continues; no crash.

## Benchmarks

All benchmarks target **Apple M2 (8-core, 3.5 GHz)** with a 256-frame audio buffer at 48 kHz as the
reference hardware. CI runs them with `cargo bench` on the macOS runner; variance tolerance is +/-
15%.

| ID | Benchmark | Target | Hardware | Req |
|----|-----------|--------|----------|-----|
| TC-IR-1.8.1.B1 | 500 impacts/frame | < 0.5 ms | Apple M2 | IR-1.8.1 |
| TC-IR-1.8.1.B2 | Cooldown tracker tick | < 0.01 ms | Apple M2 | IR-1.8.1 |
| TC-IR-1.8.2.B1 | Material table lookup | < 0.001 ms | Apple M2 | IR-1.8.2 |
| TC-IR-1.8.4.B1 | 50 trigger events/frame | < 0.05 ms | Apple M2 | IR-1.8.4 |
| TC-IR-1.8.5.B1 | 100 friction sources | < 0.1 ms | Apple M2 | IR-1.8.5 |
| TC-IR-1.8.Q.B1 | MPSC drain 1024 cmds | < 0.05 ms | Apple M2 | IR-1.8.1 |
