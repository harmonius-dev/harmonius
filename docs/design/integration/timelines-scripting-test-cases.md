# Timelines ↔ Scripting Integration Test Cases

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-4.9.1.1 | Keyframe fires graph entry | Cross entity KF | "on_timeline_event" invoked | IR-4.9.1 |
| TC-IR-4.9.1.2 | Multiple KFs fire in order | 3 KFs in one tick | 3 events dispatched in order | IR-4.9.1 |
| TC-IR-4.9.1.3 | Missing entry point skips | No "on_timeline_event" | Warning logged, no crash | IR-4.9.1 |
| TC-IR-4.9.2.1 | Graph calls play() | Graph node invokes play | PlaybackState.playing = true | IR-4.9.2 |
| TC-IR-4.9.2.2 | Graph calls pause() | Graph node invokes pause | PlaybackState.playing = false | IR-4.9.2 |
| TC-IR-4.9.2.3 | Graph calls seek() | Graph node seek(3.0) | PlaybackState.current_time=3.0 | IR-4.9.2 |
| TC-IR-4.9.3.1 | Coroutine waits for complete | WaitCondition::TimelineComplete | Suspended until complete | IR-4.9.3 |
| TC-IR-4.9.3.2 | Coroutine resumes on complete | Timeline finishes | Coroutine resumes next frame | IR-4.9.3 |
| TC-IR-4.9.3.3 | Coroutine timeout fires | Timeline never completes | Timeout after threshold | IR-4.9.3 |
| TC-IR-4.9.4.1 | F32 track drives variable | Track samples 0.5 | VariableStore slot = 0.5 | IR-4.9.4 |
| TC-IR-4.9.4.2 | Bool track drives variable | Track samples true | VariableStore slot = true | IR-4.9.4 |
| TC-IR-4.9.4.3 | Vec3 track drives variable | Track samples (1,2,3) | VariableStore slot = (1,2,3) | IR-4.9.4 |
| TC-IR-4.9.5.1 | Branch seeks to choice A | Condition selects A | PlaybackState seeks to time_A | IR-4.9.5 |
| TC-IR-4.9.5.2 | Branch seeks to choice B | Condition selects B | PlaybackState seeks to time_B | IR-4.9.5 |
| TC-IR-4.9.5.3 | Default branch on error | Condition evaluation fails | Default branch taken | IR-4.9.5 |
| TC-IR-4.9.6.1 | Graph emits seek event | TimelineSeekEvent(5.0) | PlaybackState.current_time=5.0 | IR-4.9.6 |
| TC-IR-4.9.6.2 | Seek clamped to duration | Seek(999.0), duration=10 | current_time = 10.0 | IR-4.9.6 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-4.9.1.B1 | 100 graph events per frame | < 0.5 ms | IR-4.9.1 |
| TC-IR-4.9.3.B1 | 50 coroutine resume checks | < 0.1 ms | IR-4.9.3 |
| TC-IR-4.9.4.B1 | 32 track-to-variable bindings | < 0.05 ms | IR-4.9.4 |
