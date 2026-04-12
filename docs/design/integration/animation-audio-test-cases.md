# Animation ↔ Audio Integration Test Cases

All tests are CI-runnable. They use real `SoundBank`, real `CommandSender` (crossbeam MPSC), and a
fake `AudioBackend` that records commands instead of driving hardware. No mocking libraries are
used. Per-test setup builds an ECS world from scratch with deterministic `Rng` seed `0x1234`.

## Integration Tests

### Positive Cases

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-1.2.1.1 | Footstep fires | Foot-plant frame | Play enqueued | IR-1.2.1 |
| TC-IR-1.2.1.2 | No dup fire | Advance past event | No duplicate | IR-1.2.1 |
| TC-IR-1.2.1.7 | Same-frame consume | Event emit + bridge run | Drained same frame | IR-1.2.1 |
| TC-IR-1.2.2.1 | Hit window sound | Weapon swing frame | Impact at bone pos | IR-1.2.2 |
| TC-IR-1.2.2.2 | Hit high priority | Hit + footstep | Hit survives steal | IR-1.2.2 |
| TC-IR-1.2.3.1 | Sync at 1x speed | Walk at 1.0 speed | Within 1 frame | IR-1.2.3 |
| TC-IR-1.2.3.2 | Sync at 2x speed | Walk at 2.0 speed | Events at 2x rate | IR-1.2.3 |
| TC-IR-1.2.4.1 | Stone surface | Raycast hits stone | Stone sound | IR-1.2.4 |
| TC-IR-1.2.4.2 | Grass surface | Raycast hits grass | Grass sound | IR-1.2.4 |
| TC-IR-1.2.4.3 | Raycast miss | No ground hit | Default sound | IR-1.2.4 |
| TC-IR-1.2.5.1 | Run raises pitch | Speed 2.0 | Pitch ~1.1x | IR-1.2.5 |
| TC-IR-1.2.5.2 | Walk normal pitch | Speed 1.0 | Pitch == 1.0 | IR-1.2.5 |
| TC-IR-1.2.5.3 | Slow lowers pitch | Speed 0.5 | Pitch ~0.95x | IR-1.2.5 |

### Negative / Failure Mode Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-1.2.1.3 | Sound bank fallback | Unknown material | Fallback clip plays | IR-1.2.1 |
| TC-IR-1.2.1.4 | Voice limit steal | 129 footsteps | Lowest-prio culled | IR-1.2.1 |
| TC-IR-1.2.1.5 | Buffer underrun | Overloaded audio cb | Silence then resume | IR-1.2.1 |
| TC-IR-1.2.1.6 | Queue full | 4097 commands/frame | send returns Err | IR-1.2.1 |
| TC-IR-1.2.1.8 | Empty pool | Material with empty Vec | Fallback clip plays | IR-1.2.1 |
| TC-IR-1.2.2.3 | No impact entry | Bank lacks "impact" | Fallback clip plays | IR-1.2.2 |
| TC-IR-1.2.5.4 | Missing voice | No active entry | Skip, no panic | IR-1.2.5 |
| TC-IR-1.2.5.5 | Queue full on pitch | Saturate queue | Pitch stays previous | IR-1.2.5 |

1. **TC-IR-1.2.1.3** -- `SoundBank` has no entry for the raycast material. `pick` returns
   `self.fallback`. The fallback clip plays at the bone position.
2. **TC-IR-1.2.1.4** -- Spawn 129 simultaneous footstep voices (exceeding `max_real_voices`). The
   audio engine virtualizes the lowest-priority voice. Verify the highest-priority voices still
   produce output.
3. **TC-IR-1.2.1.5** -- Simulate an audio callback that exceeds its deadline. The next callback
   outputs silence for the missed buffer, then drains queued commands and resumes normal playback.
4. **TC-IR-1.2.1.6** -- Enqueue 4097 commands in one frame (exceeding the bounded MPSC capacity of
   4096). `CommandSender::send` returns `Err` for the overflow command. No panic or deadlock occurs.
5. **TC-IR-1.2.1.7** -- Assert that `animation_advance_system` emits in frame N and the
   `footstep_bridge_system` drains and enqueues to `CommandSender` within the same frame N. No
   one-frame delay.
6. **TC-IR-1.2.1.8** -- `SoundBank` has a material key but the `Vec<AssetHandle>` pool is empty.
   `pick` detects `pool.len() == 0`, skips `gen_range`, returns `self.fallback`. No panic.
7. **TC-IR-1.2.2.2** -- Fire a hit and a footstep in the same frame with `max_real_voices = 1`. The
   voice stealer culls the `VoicePriority::Medium` footstep first, preserving the
   `VoicePriority::High` hit.
8. **TC-IR-1.2.2.3** -- `SoundBank` has no `"impact"` entry. `pick` returns `self.fallback`.
9. **TC-IR-1.2.5.3** -- `AnimationPlayer.speed = 0.5`. The bridge computes
   `pitch = 1.0 + (0.5 - 1.0) * 0.1 = 0.95`. `SetParam` carries `value = 0.95`.
10. **TC-IR-1.2.5.4** -- `FootstepVoiceTracker::active` has no entry for the queried entity. The
    speed-sync system skips without sending any command and without panicking.
11. **TC-IR-1.2.5.5** -- The MPSC queue is full. `SetParam` `send` returns `Err`. The audio voice's
    pitch remains at its previous value; no visible stutter.

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-1.2.1.B1 | 200 footstep events/frame | < 0.2 ms | IR-1.2.1 |
| TC-IR-1.2.1.B2 | MPSC send p99 | < 5 us | IR-1.2.1 |
| TC-IR-1.2.2.B1 | 50 hit events/frame | < 0.1 ms | IR-1.2.2 |
| TC-IR-1.2.4.B1 | 200 surface raycasts | < 0.3 ms | IR-1.2.4 |
| TC-IR-1.2.5.B1 | 200 pitch updates/frame | < 0.1 ms | IR-1.2.5 |

Benchmarks run via `cargo bench` under the `animation_audio_integration` harness. Each benchmark
asserts its target in CI as a hard gate (not advisory).
