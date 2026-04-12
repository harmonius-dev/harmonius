# Timelines ↔ Audio Integration Test Cases

All tests are CI-runnable. The audio thread runs against a null backend; MPSC queues and the
`AtomicBeatCounter` are exercised directly from test code with no physical audio device.

## Integration Tests

| ID | Test | Req |
|----|------|-----|
| TC-IR-4.7.1.1 | Music cue fires on cross | IR-4.7.1 |
| TC-IR-4.7.1.2 | Crossfade transition applied | IR-4.7.1 |
| TC-IR-4.7.1.3 | Beat-sync transition applied | IR-4.7.1 |
| TC-IR-4.7.1.4 | Seek past cue replays it | IR-4.7.1 |
| TC-IR-4.7.1.5 | Music transition overlap resolves | IR-4.7.1 |
| TC-IR-4.7.2.1 | Voice-over plays at keyframe | IR-4.7.2 |
| TC-IR-4.7.2.2 | Voice-over stops at end | IR-4.7.2 |
| TC-IR-4.7.2.3 | Voice allocation failure | IR-4.7.2 |
| TC-IR-4.7.3.1 | SFX one-shot fires | IR-4.7.3 |
| TC-IR-4.7.3.2 | SFX spatial position correct | IR-4.7.3 |
| TC-IR-4.7.3.3 | Missing asset logged | IR-4.7.3 |
| TC-IR-4.7.3.4 | Command MPSC full | IR-4.7.3 |
| TC-IR-4.7.4.1 | Beat-time timeline advances | IR-4.7.4 |
| TC-IR-4.7.4.2 | Tempo change updates rate | IR-4.7.4 |
| TC-IR-4.7.4.3 | Beat MPSC full | IR-4.7.4 |
| TC-IR-4.7.4.4 | BeatClock drift resync | IR-4.7.4 |
| TC-IR-4.7.4.5 | `AtomicBeatCounter` snapshot read | IR-4.7.4 |
| TC-IR-4.7.5.1 | Bus gain animated | IR-4.7.5 |
| TC-IR-4.7.5.2 | Effect param animated | IR-4.7.5 |
| TC-IR-4.7.5.3 | Unknown `SegmentId` no-op | IR-4.7.5 |
| TC-IR-4.7.6.1 | Dialogue + subtitle sync | IR-4.7.6 |
| TC-IR-4.7.6.2 | Subtitle duration matches clip | IR-4.7.6 |
| TC-IR-4.7.6.3 | Subtitle hide on stop | IR-4.7.6 |

### Details

1. **TC-IR-4.7.1.1** — Input: playback crosses a music-cue keyframe. Expected:
   `AudioCommand::MusicTransition { target }` enqueued into the command MPSC.
2. **TC-IR-4.7.1.2** — Input: cue edge has `TransitionRule::TimedCrossfade(1.0)`. Expected:
   `MusicStateMachine` fades over one second.
3. **TC-IR-4.7.1.3** — Input: cue edge has `TransitionRule::BeatSyncCrossfade`. Expected: transition
   starts at the next beat boundary.
4. **TC-IR-4.7.1.4** — Input: seek from `0.0` to `5.0` seconds. Expected: every cue in `[0, 5]` is
   re-enqueued in order.
5. **TC-IR-4.7.1.5** — Input: two music cues within 10 ms. Expected: second cancels the first at the
   next beat boundary; fallback is `TransitionRule::ImmediateCut` if the boundary is past.
6. **TC-IR-4.7.2.1** — Input: `AudioTrackTarget::VoiceOver` keyframe crossed. Expected:
   `AudioCommand::Play { voice_id, clip, BusId::VOICE, priority, timestamp }` enqueued.
7. **TC-IR-4.7.2.2** — Input: voice-over clip duration expires. Expected:
   `AudioCommand::Stop { voice_id, fade_samples }` enqueued.
8. **TC-IR-4.7.2.3** — Input: `VoiceManager::allocate` returns `None`. Expected: event skipped,
   warning logged, timeline keeps advancing.
9. **TC-IR-4.7.3.1** — Input: playback crosses an SFX keyframe. Expected: `Play` + `UpdateSpatial`
   pair enqueued.
10. **TC-IR-4.7.3.2** — Input: SFX at entity world position. Expected:
    `UpdateSpatial.position == entity.transform.position`.
11. **TC-IR-4.7.3.3** — Input: SFX `AssetHandle` not loaded. Expected: warning logged, no crash, no
    `Play` enqueued.
12. **TC-IR-4.7.3.4** — Input: fill the 4096-entry command MPSC. Expected: next `send` returns
    `Err`; `TimelineAudioDebug.dropped_commands` increments.
13. **TC-IR-4.7.4.1** — Input: `BeatClock` running at 120 BPM. Expected: `advance_beats` yields two
    beats per second.
14. **TC-IR-4.7.4.2** — Input: BPM changes from 120 to 140. Expected: `BeatSnapshot.bpm` reflects
    140 within one audio buffer.
15. **TC-IR-4.7.4.3** — Input: fill the 256-entry beat MPSC. Expected:
    `TimelineAudioDebug.dropped_beats` increments; `AtomicBeatCounter` snapshot stays authoritative.
16. **TC-IR-4.7.4.4** — Input: inject 5 ms drift between wall clock and beat clock. Expected:
    `advance_beats` clamps to the snapshot `bpm` and resyncs at the next bar boundary.
17. **TC-IR-4.7.4.5** — Input: concurrent `store` and `snapshot` calls on `AtomicBeatCounter`.
    Expected: `Acquire` load always observes a complete `BeatSnapshot` with no field tearing.
18. **TC-IR-4.7.5.1** — Input: `F32` track animating 1.0 → 0.0 bound to `BusParam::Gain`. Expected:
    `SetBusParam { bus_id, Gain, v }` enqueued per sample.
19. **TC-IR-4.7.5.2** — Input: `F32` track animating 20000 → 500 on a biquad insert effect.
    Expected: `SetEffectParam { bus, node_index, param, v }` enqueued.
20. **TC-IR-4.7.5.3** — Input: target references a missing `SegmentId`. Expected: warning logged, no
    command enqueued.
21. **TC-IR-4.7.6.1** — Input: voice-over keyframe fires. Expected: `Play` and `SubtitleEvent::Show`
    dispatched in the same frame.
22. **TC-IR-4.7.6.2** — Input: five-second dialogue clip. Expected:
    `SubtitleEvent::Show.duration_ms == 5000`.
23. **TC-IR-4.7.6.3** — Input: voice stopped early via `AudioCommand::Stop`. Expected:
    `SubtitleEvent::Hide { line_id }` dispatched.

## Negative / Failure Mode Tests

| ID | Test | Req |
|----|------|-----|
| TC-IR-4.7.N.1 | Command queue full fallback | IR-4.7.3 |
| TC-IR-4.7.N.2 | Beat queue full fallback | IR-4.7.4 |
| TC-IR-4.7.N.3 | BeatClock drift > 1 bar | IR-4.7.4 |
| TC-IR-4.7.N.4 | Music overlap fallback | IR-4.7.1 |
| TC-IR-4.7.N.5 | Unknown asset fallback | IR-4.7.3 |
| TC-IR-4.7.N.6 | Voice steal fallback | IR-4.7.2 |
| TC-IR-4.7.N.7 | Debug toggle off-to-on | IR-4.7.3 |

### Details

1. **TC-IR-4.7.N.1** — Input: 4097th `Play` command. Expected: `send` returns `Err`; oldest
   low-priority command is dropped first; `dropped_commands` increments.
2. **TC-IR-4.7.N.2** — Input: 257th beat event. Expected: `send` returns `Err`; `AtomicBeatCounter`
   still advances; `dropped_beats` increments.
3. **TC-IR-4.7.N.3** — Input: forced 600 ms drift. Expected: resync on next bar; no double-fired
   beat events.
4. **TC-IR-4.7.N.4** — Input: transition triggered during an in-progress fade. Expected: fade
   cancels at the next beat; fallback `ImmediateCut` if boundary already passed.
5. **TC-IR-4.7.N.5** — Input: invalid `AssetHandle`. Expected: warning logged, timeline continues,
   no panic.
6. **TC-IR-4.7.N.6** — Input: all voices busy and a new `VoicePriority::Critical` request. Expected:
   lowest-priority voice is stolen; new `Play` succeeds.
7. **TC-IR-4.7.N.7** — Input: flip `TimelineAudioDebug.enabled` from `false` to `true`. Expected:
   ring buffer records dispatched commands within one frame.

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-4.7.1.B1 | 32-track timeline evaluation | < 0.5 ms | IR-4.7.1 |
| TC-IR-4.7.3.B1 | 100 SFX events per frame | < 0.2 ms | IR-4.7.3 |
| TC-IR-4.7.3.B2 | MPSC send p99 latency | < 5 us | IR-4.7.3 |
| TC-IR-4.7.4.B1 | BeatClock sync accuracy | < 1 ms drift | IR-4.7.4 |
| TC-IR-4.7.4.B2 | `AtomicBeatCounter` snapshot | < 100 ns | IR-4.7.4 |
| TC-IR-4.7.5.B1 | Bus param update latency | < 20 ms (1 buffer) | IR-4.7.5 |
| TC-IR-4.7.6.B1 | Dialogue dispatch overhead | < 50 us/event | IR-4.7.6 |
