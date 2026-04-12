# Animation ↔ Audio Integration Test Cases

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-1.2.1.1 | Footstep fires | Foot-plant frame | Play enqueued | IR-1.2.1 |
| TC-IR-1.2.1.2 | No dup fire | Advance past event | No duplicate | IR-1.2.1 |
| TC-IR-1.2.2.1 | Hit window sound | Weapon swing frame | Impact at bone pos | IR-1.2.2 |
| TC-IR-1.2.3.1 | Sync at 1x speed | Walk at 1.0 speed | Within 1 frame | IR-1.2.3 |
| TC-IR-1.2.3.2 | Sync at 2x speed | Walk at 2.0 speed | Events at 2x rate | IR-1.2.3 |
| TC-IR-1.2.4.1 | Stone surface | Raycast hits stone | Stone sound | IR-1.2.4 |
| TC-IR-1.2.4.2 | Grass surface | Raycast hits grass | Grass sound | IR-1.2.4 |
| TC-IR-1.2.4.3 | Raycast miss | No ground hit | Default sound | IR-1.2.4 |
| TC-IR-1.2.5.1 | Run raises pitch | Speed 2.0 | Pitch ~1.1x | IR-1.2.5 |
| TC-IR-1.2.5.2 | Walk normal pitch | Speed 1.0 | Pitch == 1.0 | IR-1.2.5 |

### Failure Mode Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-1.2.1.3 | Sound bank fallback | Unknown material | Fallback clip plays | IR-1.2.1 |
| TC-IR-1.2.1.4 | Voice limit steal | 129 footsteps | Lowest-prio culled | IR-1.2.1 |
| TC-IR-1.2.1.5 | Buffer underrun | Overloaded audio cb | Silence then resume | IR-1.2.1 |
| TC-IR-1.2.1.6 | Queue full | 4097 commands/frame | send returns Err | IR-1.2.1 |

1. **TC-IR-1.2.1.3** -- `SoundBank` has no entry for the raycast material. `pick` returns
   `self.fallback`. The fallback clip plays at the bone position.
2. **TC-IR-1.2.1.4** -- Spawn 129 simultaneous footstep voices (exceeding `max_real_voices`). The
   audio engine virtualizes the lowest-priority voice. Verify the highest-priority voices still
   produce output.
3. **TC-IR-1.2.1.5** -- Simulate an audio callback that exceeds its deadline. The next callback
   outputs silence for the missed buffer, then drains queued commands and resumes normal playback.
4. **TC-IR-1.2.1.6** -- Enqueue 4097 commands in one frame (exceeding the bounded MPSC capacity of
   4096). `CommandSender::send` returns `Err` for the overflow command. No panic or deadlock occurs.

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-1.2.1.B1 | 200 footstep events/frame | < 0.2 ms | IR-1.2.1 |
| TC-IR-1.2.4.B1 | 200 surface raycasts | < 0.3 ms | IR-1.2.4 |
