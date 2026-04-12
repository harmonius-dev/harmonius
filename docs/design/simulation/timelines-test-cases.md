# Timelines — Test Cases

Companion to [timelines.md](timelines.md).

Test case IDs use `TC-17.4.Z.N` format. Every row links to a specific R-17.4.Z or NFR-TL.N.

## Unit Tests

| ID            | Name                                | Req        |
|---------------|-------------------------------------|------------|
| TC-17.4.1.1   | `test_track_sample_linear_midpoint` | R-17.4.1   |
| TC-17.4.1.2   | `test_track_sample_step_hold`       | R-17.4.1   |
| TC-17.4.1.3   | `test_track_sample_cubic_bezier`    | R-17.4.1   |
| TC-17.4.1.4   | `test_track_sample_constant`        | R-17.4.1   |
| TC-17.4.1.5   | `test_track_sample_before_first_kf` | R-17.4.1   |
| TC-17.4.1.6   | `test_track_sample_after_last_kf`   | R-17.4.1   |
| TC-17.4.1.7   | `test_track_default_value_empty`    | R-17.4.1   |
| TC-17.4.1.8   | `test_keyframe_at_or_before`        | R-17.4.1   |
| TC-17.4.3.1   | `test_timeline_track_by_name`       | R-17.4.3   |
| TC-17.4.3.2   | `test_timeline_track_by_id`         | R-17.4.3   |
| TC-17.4.3.3   | `test_timeline_duration_max_track`  | R-17.4.3   |
| TC-17.4.3.4   | `test_timeline_sync_two_tracks`     | R-17.4.3   |
| TC-17.4.6.1   | `test_playback_advance_forward`     | R-17.4.6   |
| TC-17.4.6.2   | `test_playback_pause_no_advance`    | R-17.4.6   |
| TC-17.4.6.3   | `test_playback_speed_multiplier`    | R-17.4.6   |
| TC-17.4.6.4   | `test_playback_reverse_direction`   | R-17.4.6   |
| TC-17.4.6.5   | `test_playback_loop_increments`     | R-17.4.6   |
| TC-17.4.6.6   | `test_playback_pingpong`            | R-17.4.6   |
| TC-17.4.6.7   | `test_playback_clamp_forever`       | R-17.4.6   |
| TC-17.4.6.8   | `test_playback_seek_clamps`         | R-17.4.6   |
| TC-17.4.11.1  | `test_event_keyframe_crossed`       | R-17.4.11  |
| TC-17.4.11.2  | `test_event_track_complete`         | R-17.4.11  |
| TC-17.4.11.3  | `test_event_loop_point`             | R-17.4.11  |
| TC-17.4.12.1  | `test_rkyv_roundtrip_playback`      | R-17.4.12  |
| TC-17.4.12.2  | `test_rkyv_roundtrip_track`         | R-17.4.12  |
| TC-13.5.1.1   | `test_cutscene_multi_track_use`     | R-13.5.1   |
| TC-13.5.3.1   | `test_camera_rail_spline_drive`     | R-13.5.3   |
| TC-13.5.4.1   | `test_actor_anim_blend_timeline`    | R-13.5.4   |
| TC-13.19.4a.1 | `test_npc_schedule_timeline`        | R-13.19.4a |
| TC-13.23.4.1  | `test_login_reward_calendar`        | R-13.23.4  |

1. **TC-17.4.1.1** `test_track_sample_linear_midpoint` — Track with two keyframes at `t=0, v=0` and
   `t=1, v=10`, both linear. Sample at `t=0.5`. Assert value is `5.0`.
   - Input: `Track::<f32>` with two keyframes — `(time=0.0, value=0.0, Linear)` and
     `(time=1.0, value=10.0, Linear)` — and `default_value: 0.0`
   - Expected: `track.sample(0.5) == 5.0`

2. **TC-17.4.1.2** `test_track_sample_step_hold` — Track with step interpolation. Keyframes
   `(0, 1.0)` step, `(1, 5.0)` step. Sample at 0.99 and 1.0.
   - Input: two step keyframes
   - Expected: `track.sample(0.99) == 1.0`, `track.sample(1.0) == 5.0`

3. **TC-17.4.1.3** `test_track_sample_cubic_bezier` — Track with cubic bezier `c1 = (0.42, 0.0)`,
   `c2 = (0.58, 1.0)` (CSS ease-in-out). Sample at midpoint and assert value is `0.5 ± 1e-3`.
   - Input: keyframes `(0, 0.0)`, `(1, 1.0)` with
     `CubicBezier { c1: vec2(0.42, 0.0), c2: vec2(0.58, 1.0) }`
   - Expected: `(track.sample(0.5) - 0.5).abs() < 1e-3`

4. **TC-17.4.1.4** `test_track_sample_constant` — Track with `Constant` interpolation on the first
   keyframe. Sample at any later time.
   - Input: keyframes `(0, 7.0) Constant`, `(5, 99.0) Linear`
   - Expected: `track.sample(2.0) == 7.0`, `track.sample(4.99) == 7.0`

5. **TC-17.4.1.5** `test_track_sample_before_first_kf` — Sample at a time before any keyframe.
   Assert the first keyframe's value is returned (no extrapolation).
   - Input: keyframes `(1.0, 10.0)`, `(2.0, 20.0)`; sample at `t = 0.5`
   - Expected: `track.sample(0.5) == 10.0`

6. **TC-17.4.1.6** `test_track_sample_after_last_kf` — Sample after the last keyframe; assert the
   last value is held (no extrapolation past the end).
   - Input: keyframes `(0.0, 0.0)`, `(1.0, 10.0)`; sample at `t = 5.0`
   - Expected: `track.sample(5.0) == 10.0`

7. **TC-17.4.1.7** `test_track_default_value_empty` — Sample on a track with zero keyframes. Assert
   `default_value` is returned at any time.
   - Input: `Track::<f32> { keyframes: [], default_value: 42.0, .. }`
   - Expected: `track.sample(0.0) == 42.0`, `track.sample(100.0) == 42.0`

8. **TC-17.4.1.8** `test_keyframe_at_or_before` — Track with keyframes at `t = [1, 3, 5, 7]`. Query
   `keyframe_at_or_before(4.0)`. Assert returned keyframe time is 3.0.
   - Input: 4-keyframe track; query times 0.5, 3.0, 4.0, 8.0
   - Expected: `None`, `Some(3.0)`, `Some(3.0)`, `Some(7.0)`

9. **TC-17.4.3.1** `test_timeline_track_by_name` — `MultiTrackTimeline` with two tracks named
   "camera_fov" and "audio_volume". `track_by_name("camera_fov")` returns the camera track.
   - Input: timeline with two tracks
   - Expected: returned track's `name == "camera_fov"`; `track_by_name("missing")` returns `None`

10. **TC-17.4.3.2** `test_timeline_track_by_id` — Direct ID lookup. Assert returned track's id
    matches and lookup is O(1).
    - Input: 4-track timeline; lookup `TrackId(2)`
    - Expected: `track_by_id(TrackId(2)).unwrap().id == TrackId(2)`

11. **TC-17.4.3.3** `test_timeline_duration_max_track` — Timeline with three tracks of durations
    `[2.0, 5.0, 3.0]`. Assert timeline `duration == 5.0`.
    - Input: tracks with last keyframe times 2.0, 5.0, 3.0
    - Expected: `timeline.duration == 5.0`

12. **TC-17.4.3.4** `test_timeline_sync_two_tracks` — Timeline with a camera and audio track. Sample
    both at the same time T. Assert both return the value at exactly T (no drift).
    - Input: camera track keyframe `(1.0, fov=60)`, audio track `(1.0, volume=0.8)`; sample both at
      `t = 1.0`
    - Expected: camera sample == 60, audio sample == 0.8

13. **TC-17.4.6.1** `test_playback_advance_forward` — `PlaybackState` at `current_time = 0.0`,
    `speed = 1.0`, playing. Advance by `dt = 0.5`. Assert `current_time == 0.5`.
    - Input:
      `PlaybackState { current_time: 0.0, speed: 1.0, playing: true, direction: Forward, .. }`,
      advance 0.5 s
    - Expected: `state.current_time == 0.5`, no events fired (no keyframe crossed)

14. **TC-17.4.6.2** `test_playback_pause_no_advance` — Paused state. Advance by `dt = 1.0`. Assert
    `current_time` is unchanged.
    - Input: `playing: false, current_time: 2.0`
    - Expected: `state.current_time == 2.0`

15. **TC-17.4.6.3** `test_playback_speed_multiplier` — `speed = 2.0`. Advance by `dt = 0.25`. Assert
    `current_time` advances by `0.5`.
    - Input: `current_time: 0.0, speed: 2.0`, advance 0.25 s
    - Expected: `state.current_time == 0.5`

16. **TC-17.4.6.4** `test_playback_reverse_direction` — `direction = Reverse, current_time = 1.0`.
    Advance by `dt = 0.25`. Assert time decreases to `0.75`.
    - Input: `direction: Reverse, current_time: 1.0, speed: 1.0`
    - Expected: `state.current_time == 0.75`

17. **TC-17.4.6.5** `test_playback_loop_increments` — Timeline duration 1.0, `LoopMode::Loop`.
    Advance from 0.9 by `dt = 0.2`. Assert `current_time == 0.1` and `loop_count == 1`.
    - Input: `current_time: 0.9`, advance 0.2, `loop_mode: Loop, duration: 1.0`
    - Expected: `state.current_time ≈ 0.1`, `state.loop_count == 1`

18. **TC-17.4.6.6** `test_playback_pingpong` — `LoopMode::PingPong`, duration 1.0. Advance from 0.9
    by 0.2. Assert direction flips to Reverse and `current_time == 0.9`.
    - Input: `current_time: 0.9, direction: Forward, loop_mode: PingPong`, advance 0.2
    - Expected: `state.direction == Reverse`, `state.current_time ≈ 0.9`

19. **TC-17.4.6.7** `test_playback_clamp_forever` — `LoopMode::ClampForever`, duration 1.0. Advance
    past end. Assert `current_time == 1.0` and `playing` remains true.
    - Input: `current_time: 0.9`, advance 0.5, `loop_mode: ClampForever`
    - Expected: `state.current_time == 1.0`, `state.playing == true`

20. **TC-17.4.6.8** `test_playback_seek_clamps` — `seek(-1.0)` clamps to 0; `seek(2.0)` on a
    1.0-second timeline clamps to 1.0 (or wraps under Loop).
    - Input: `duration: 1.0, loop_mode: Once`, calls `seek(-1.0)` then `seek(2.0)`
    - Expected: after first call `current_time == 0.0`; after second `current_time == 1.0`

21. **TC-17.4.11.1** `test_event_keyframe_crossed` — Track with a keyframe at `t = 0.5` marked as a
    trigger. Advance from 0.4 to 0.6. Assert one `KeyframeCrossed` event is yielded.
    - Input: trigger keyframe at 0.5, advance 0.4 → 0.6
    - Expected: `advance` returns `SmallVec` containing one
      `TimelineEvent { kind: KeyframeCrossed, time: 0.5, .. }`

22. **TC-17.4.11.2** `test_event_track_complete` — Single-track timeline reaches its last keyframe.
    Assert `TrackComplete` event is yielded.
    - Input: track with last keyframe at 1.0; advance to 1.0
    - Expected: events contain `TrackComplete` for that track

23. **TC-17.4.11.3** `test_event_loop_point` — Looping timeline crosses the loop point. Assert one
    `LoopPoint` event is yielded each loop crossing.
    - Input: `LoopMode::Loop`, advance across the duration boundary three times
    - Expected: 3 `LoopPoint` events emitted across the calls; no extras

24. **TC-17.4.12.1** `test_rkyv_roundtrip_playback` — Serialize a `PlaybackState` mid-playback;
    deserialize; assert byte-identical state restored.
    - Input: `PlaybackState` with `timeline_id = AssetId(7)`, `current_time = 1.234`, `speed = 1.5`,
      `playing = true`, `direction = Reverse`, `loop_count = 4`
    - Expected: deserialized state equals original field-for-field

25. **TC-17.4.12.2** `test_rkyv_roundtrip_track` — Serialize a `Track<f32>` with 16 keyframes,
    deserialize via mmap. Assert keyframes match byte-for-byte.
    - Input: track with 16 keyframes spanning all interpolation modes
    - Expected: `archived.keyframes` matches input length and contents bit-identically

26. **TC-13.5.1.1** `test_cutscene_multi_track_use` — Build a cutscene entity referencing a
    `MultiTrackTimeline` with camera, audio, and actor tracks. Evaluate at several frames. Assert
    each track samples against the same `current_time` without drift.
    - Input: cutscene with 3 tracks; sample at t=0, 0.5, 1.0
    - Expected: at each sample, all three tracks report the same `current_time`; samples match
      independent computation

27. **TC-13.5.3.1** `test_camera_rail_spline_drive` — Drive a camera along a spline by binding a
    `Track<Vec3>` to `Camera.position` and `Track<Quat>` to `Camera.rotation`. Advance the timeline
    and assert each frame's camera transform follows the track samples exactly.
    - Input: two keyframed tracks; 60 frames
    - Expected: camera position and rotation at each frame equal the sampled track values

28. **TC-13.5.4.1** `test_actor_anim_blend_timeline` — Use a timeline bone-override track to modify
    an actor's pose while gameplay animation is active. Assert the blend weights yield the correct
    final pose.
    - Input: gameplay layer weight 0.3; timeline override weight 0.7
    - Expected: final bone transforms are `0.3 * gameplay + 0.7 * override`

29. **TC-13.19.4a.1** `test_npc_schedule_timeline` — Model an NPC daily schedule as a timeline with
    `KeyframeCrossed` trigger events at each waypoint. Advance 24 in-game hours; assert each
    schedule event fires exactly once.
    - Input: timeline with 6 schedule triggers over 24 hours
    - Expected: 6 unique `KeyframeCrossed` events delivered; no duplicate or missed triggers

30. **TC-13.23.4.1** `test_login_reward_calendar` — Author a daily login reward calendar as a
    timeline with trigger events at day boundaries. Advance 7 days; assert 7 reward triggers fire.
    - Input: calendar timeline with 7 daily triggers
    - Expected: 7 `KeyframeCrossed` events, one per day boundary

## Integration Tests

| ID            | Name                                  | Req        |
|---------------|---------------------------------------|------------|
| TC-17.4.I.1   | `test_32_track_evaluate_under_budget` | R-17.4.4   |
| TC-17.4.I.2   | `test_1000_playback_advance_budget`   | R-17.4.5   |
| TC-17.4.I.3   | `test_cutscene_synchronized_playback` | R-17.4.7   |
| TC-17.4.I.4   | `test_cutscene_skip_terminal_state`   | R-17.4.7   |
| TC-17.4.I.5   | `test_animation_blend_priority`       | R-17.4.8   |
| TC-17.4.I.6   | `test_property_animation_codegen`     | R-17.4.9   |
| TC-17.4.I.7   | `test_scrubber_seek_and_record`       | R-17.4.10  |
| TC-17.4.I.8   | `test_save_resume_mid_playback`       | R-17.4.12  |
| TC-17.4.1.I1  | `test_author_generic_track`           | US-17.4.1  |
| TC-17.4.2.I1  | `test_sim_track_sample_budget`        | US-17.4.2  |
| TC-17.4.3.I1  | `test_author_multitrack_timeline`     | US-17.4.3  |
| TC-17.4.4.I1  | `test_sim_32_track_eval`              | US-17.4.4  |
| TC-17.4.5.I1  | `test_sim_1k_playback_advance`        | US-17.4.5  |
| TC-17.4.6.I1  | `test_author_playback_controls`       | US-17.4.6  |
| TC-17.4.7.I1  | `test_author_cutscene_sync`           | US-17.4.7  |
| TC-17.4.8.I1  | `test_author_anim_blend_override`     | US-17.4.8  |
| TC-17.4.9.I1  | `test_author_property_anim`           | US-17.4.9  |
| TC-17.4.10.I1 | `test_author_scrubber_editor`         | US-17.4.10 |
| TC-17.4.11.I1 | `test_author_event_on_keyframe`       | US-17.4.11 |
| TC-17.4.12.I1 | `test_author_save_resume`             | US-17.4.12 |

1. **TC-17.4.I.1** `test_32_track_evaluate_under_budget` — Construct a single timeline with 32
   `Track<TrackValue>` instances of varying types. Run `evaluate_all` for one frame on a single
   bound entity. Assert wall time < 0.5 ms.
   - Input: 32 tracks (mix of F32, Vec3, Quat, Color), each with 8 keyframes
   - Expected: `evaluate_all` completes in < 0.5 ms; each track value matches an
     independently-computed sample

2. **TC-17.4.I.2** `test_1000_playback_advance_budget` — Spawn 1,000 entities each with a
   `PlaybackState` referencing one of 4 shared timeline assets. Run one `TimelineAdvanceSystem` pass
   with `dt = 0.016`. Assert total wall time < 0.5 ms.
   - Input: 1,000 `PlaybackState` components
   - Expected: stage time < 0.5 ms; every `current_time` advanced by `0.016 * speed`

3. **TC-17.4.I.3** `test_cutscene_synchronized_playback` — Cutscene entity owns a camera track, an
   audio track, and an actor animation track. Play for 1 second. Assert all three tracks evaluate
   against the same `current_time` each frame.
   - Input: cutscene asset with 3 child tracks; advance to 1.0 s in 60 frames
   - Expected: each frame, sampled times across the three tracks are identical (no drift)

4. **TC-17.4.I.4** `test_cutscene_skip_terminal_state` — Start a cutscene, advance to 0.5 s, call
   `skip()`. Assert `current_time` jumps to `duration` and the terminal state side effects (final
   camera pose, end-of-scene events) are evaluated.
   - Input: cutscene with duration 5.0 and a `KeyframeCrossed` trigger at 4.9
   - Expected: `current_time == 5.0`, the 4.9 trigger fires once, terminal camera pose applied

5. **TC-17.4.I.5** `test_animation_blend_priority` — Run a cutscene with a bone-override track while
   gameplay animation is also active on the same skeleton. Assert blended pose matches the priority
   weights (cutscene = 1.0 overrides gameplay = 0.5).
   - Input: skeleton entity with both gameplay AnimationLayer and cutscene track override
   - Expected: blended pose's bone transforms equal cutscene-only result for fully-weighted bones

6. **TC-17.4.I.6** `test_property_animation_codegen` — Bind a `Track<Color>` to `Light.color` via
   the codegen'd binding function. Advance the timeline for 60 frames. Assert each frame's
   `Light.color` matches the sampled track value.
   - Input: `Light { color: Color::WHITE }`, color track keyframes `(0, RED)`, `(1, BLUE)`
   - Expected: at `t = 0.5`, `Light.color` ≈ `Color::lerp(RED, BLUE, 0.5)`

7. **TC-17.4.I.7** `test_scrubber_seek_and_record` — Editor scrubs a loaded timeline to `t = 1.5`;
   entity properties are updated by `evaluate_all`. User edits the entity's transform; a new
   keyframe is inserted at `t = 1.5`.
   - Input: 3-keyframe transform track; scrub to 1.5; edit position to `(10, 0, 0)`
   - Expected: new keyframe inserted at exactly `t = 1.5`; track keyframe count increases by 1

8. **TC-17.4.I.8** `test_save_resume_mid_playback` — Start a timeline, advance 2.5 s, save the
   world, reload, continue advancing. Assert continued playback resumes from `current_time = 2.5`
   and progresses normally.
   - Input: timeline duration 5.0; advance 2.5 s; save; reload; advance 1.0 s
   - Expected: post-reload `current_time == 3.5`, all events from 2.5–3.5 fire exactly once

9. **TC-17.4.1.I1** `test_author_generic_track` (US-17.4.1) — As a designer, author a generic
   `Track<f32>` in the timeline editor for a "camera_fov" curve. Save and reload. Assert keyframes
   and interpolation modes round-trip.
   - Input: editor-authored 6-keyframe track with mixed interpolation
   - Expected: reload yields identical keyframes and modes

10. **TC-17.4.2.I1** `test_sim_track_sample_budget` (US-17.4.2) — As a developer, sample one track
    10,000 times during a frame. Assert per-sample cost meets the documented budget (< 100 ns
    amortized).
    - Input: 100-keyframe track; 10,000 samples
    - Expected: amortized per-sample time < 100 ns

11. **TC-17.4.3.I1** `test_author_multitrack_timeline` (US-17.4.3) — As a designer, author a
    multi-track timeline containing camera, audio, and actor animation tracks. Lookup tracks by name
    and by ID. Assert lookups return the expected tracks.
    - Input: 3-track timeline
    - Expected: `track_by_name` and `track_by_id` return the correct tracks; misses return None

12. **TC-17.4.4.I1** `test_sim_32_track_eval` (US-17.4.4) — As a developer, evaluate a
    32-track-per-entity timeline for one frame and assert the full evaluation meets 0.5 ms.
    - Input: 32-track timeline bound to one entity
    - Expected: `evaluate_all` wall time < 0.5 ms

13. **TC-17.4.5.I1** `test_sim_1k_playback_advance` (US-17.4.5) — As a developer, run 1,000
    `PlaybackState` advances in one pass and assert the pass meets 0.5 ms budget.
    - Input: 1k playback states
    - Expected: pass wall time < 0.5 ms

14. **TC-17.4.6.I1** `test_author_playback_controls` (US-17.4.6) — As a designer, control playback
    with play/pause/seek/speed/reverse/loop. Exercise every control and assert state updates
    accordingly.
    - Input: timeline duration 2.0; call play, pause, seek(1.0), speed(2.0), reverse, loop
    - Expected: each call results in the expected `PlaybackState` field change

15. **TC-17.4.7.I1** `test_author_cutscene_sync` (US-17.4.7) — As a cinematic designer, author a
    cutscene with camera, audio, and actor tracks. Play for 1 second. Assert all three tracks sample
    against the same `current_time` each frame (no drift).
    - Input: cutscene with 3 tracks
    - Expected: per-frame sample time identical across all 3 tracks

16. **TC-17.4.8.I1** `test_author_anim_blend_override` (US-17.4.8) — As an animator, use a timeline
    to override specific bones on top of gameplay animation. Play the timeline; assert the blended
    pose matches the designed weights.
    - Input: gameplay layer weight 0.4; timeline override weight 0.6
    - Expected: blended bone transforms equal `0.4*A + 0.6*B`

17. **TC-17.4.9.I1** `test_author_property_anim` (US-17.4.9) — As a designer, bind a timeline track
    to a codegen'd property (e.g., `Light.color`). Play the timeline; assert the target property
    updates every frame from the sampled track value.
    - Input: color track bound to `Light.color`; 60 frames
    - Expected: each frame's color equals the track sample

18. **TC-17.4.10.I1** `test_author_scrubber_editor` (US-17.4.10) — As a designer, scrub the timeline
    editor to an arbitrary time, edit an entity property, and record a new keyframe at that time.
    Assert the new keyframe is inserted at exactly the scrub position.
    - Input: scrub to 1.5; edit property; record
    - Expected: track keyframe count increases by 1; new keyframe time == 1.5

19. **TC-17.4.11.I1** `test_author_event_on_keyframe` (US-17.4.11) — As a designer, mark a keyframe
    as a trigger. Play the timeline across that keyframe. Assert exactly one `KeyframeCrossed` event
    is emitted.
    - Input: trigger keyframe at 0.5; advance 0.4→0.6
    - Expected: one `KeyframeCrossed` event at time 0.5

20. **TC-17.4.12.I1** `test_author_save_resume` (US-17.4.12) — As a player, save mid-cutscene and
    reload later. Assert playback resumes at the saved time and events are not duplicated.
    - Input: save at t=2.5; reload; advance 1.0 s
    - Expected: post-reload current_time == 3.5; no duplicate events for t < 2.5

## Benchmarks

| ID            | Benchmark                            | Target     | Req         |
|---------------|--------------------------------------|------------|-------------|
| TC-17.4.B.1   | Single track sample (100 keyframes)  | < 100 ns   | R-17.4.2    |
| TC-17.4.B.2   | 32-track timeline evaluate           | < 0.5 ms   | R-17.4.4    |
| TC-17.4.B.3   | 1,000 playback advance pass          | < 0.5 ms   | R-17.4.5    |
| TC-17.4.B.4   | rkyv mmap load (1k-keyframe track)   | < 1 ms     | R-17.4.12   |
| TC-17.4.B.5   | Property animation 100 entities      | < 0.2 ms   | R-17.4.9    |

1. **TC-17.4.B.1** — Pre-build a `Track<f32>` with 100 keyframes. Sample at 10,000 random times.
   Mean per-sample wall time recorded with `criterion`. Validates the `O(log n)` binary search plus
   interpolation hot path.

2. **TC-17.4.B.2** — Single `MultiTrackTimeline` with 32 mixed-type tracks (F32, Vec3, Quat, Color).
   Measure one full `evaluate_all` call against a bound entity at a fixed time.

3. **TC-17.4.B.3** — Spawn 1,000 entities with `PlaybackState` referencing a shared timeline.
   Measure one `TimelineAdvanceSystem` pass with `dt = 0.016`. Wall time end to end including ECS
   query iteration.

4. **TC-17.4.B.4** — `MultiTrackTimeline` containing one track with 1,000 keyframes serialized to a
   binary file. Measure mmap-load + first `sample()` call. Must allocate zero bytes (zero-copy
   deserialization).

5. **TC-17.4.B.5** — 100 entities each animating a `Light.color` field via codegen'd bindings.
   Measure one full advance + evaluate + write-back pass. Wall time end to end.
