# Timelines ↔ Audio Integration Test Cases

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-4.7.1.1 | Music cue fires on cross | Playback crosses cue KF | MusicCue command enqueued | IR-4.7.1 |
| TC-IR-4.7.1.2 | Crossfade transition triggers | Cue with Crossfade(1.0) | Music fades over 1 second | IR-4.7.1 |
| TC-IR-4.7.1.3 | Beat-sync transition triggers | Cue with BeatSync | Transition at next beat | IR-4.7.1 |
| TC-IR-4.7.2.1 | Voice-over plays at keyframe | Playback crosses VO KF | AudioSource plays clip | IR-4.7.2 |
| TC-IR-4.7.2.2 | Voice-over stops at end | VO clip duration expires | Playback stops cleanly | IR-4.7.2 |
| TC-IR-4.7.3.1 | SFX one-shot fires | Playback crosses SFX KF | PlayOneShot command sent | IR-4.7.3 |
| TC-IR-4.7.3.2 | SFX spatial position correct | SFX at entity position | 3D positioned playback | IR-4.7.3 |
| TC-IR-4.7.4.1 | Beat-time timeline advances | BeatClock at 120 BPM | Timeline at 2 beats/sec | IR-4.7.4 |
| TC-IR-4.7.4.2 | Tempo change updates rate | BPM changes 120 to 140 | Timeline rate adjusts | IR-4.7.4 |
| TC-IR-4.7.5.1 | Bus gain animated | F32 track 1.0 to 0.0 | Bus gain fades to 0 | IR-4.7.5 |
| TC-IR-4.7.5.2 | Low-pass cutoff animated | F32 track 20000 to 500 | Filter sweeps down | IR-4.7.5 |
| TC-IR-4.7.6.1 | Dialogue + subtitle sync | VO keyframe fires | Audio + SubtitleEvent same frame | IR-4.7.6 |
| TC-IR-4.7.6.2 | Subtitle duration matches clip | 5 s dialogue clip | Subtitle visible for 5 s | IR-4.7.6 |
| TC-IR-4.7.1.4 | Seek past cue fires it | Seek from 0 to 5.0 s | All cues between 0-5 fire | IR-4.7.1 |
| TC-IR-4.7.3.3 | Missing asset logged | SFX asset not loaded | Warning logged, no crash | IR-4.7.3 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-4.7.1.B1 | 32-track timeline evaluation | < 0.5 ms | IR-4.7.1 |
| TC-IR-4.7.3.B1 | 100 SFX events per frame | < 0.2 ms | IR-4.7.3 |
| TC-IR-4.7.4.B1 | BeatClock sync accuracy | < 1 ms drift | IR-4.7.4 |
| TC-IR-4.7.5.B1 | Bus param update latency | < 20 ms (1 buffer) | IR-4.7.5 |
