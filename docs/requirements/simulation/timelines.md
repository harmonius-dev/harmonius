# R-17.4 — Timelines Requirements

## Tracks and Keyframes

1. **R-17.4.1** — The engine **SHALL** provide a generic `Track<T>` primitive as a named channel of
   timestamped `Keyframe<T>` values with per-keyframe interpolation mode (linear, step, cubic,
   bezier) and a default value when no keyframes exist.
   - **Rationale:** Tracks are the universal primitive for any time-varying value; one generic type
     serves camera, audio, animation, UI, and custom property animation.
   - **Verification:** Unit test: author a Track<f32> with keyframes at 0s=0 and 1s=10 linear;
     sample at 0.5s; assert value 5.0.
2. **R-17.4.2** — The engine **SHALL** sample a single track value at an arbitrary time in under 100
   ns on desktop hardware.
   - **Rationale:** Dense tracks (32+ per timeline, 1,000+ active timelines) demand per-sample cost
     in the sub-microsecond range.
   - **Verification:** Benchmark: sample a 100-keyframe Track<f32> at 10,000 random times; assert
     mean per-sample cost under 100 ns.

## Multi-Track Timelines

1. **R-17.4.3** — The engine **SHALL** provide `MultiTrackTimeline` as an immutable asset containing
   multiple tracks that play in sync, authored in the visual sequencer editor.
   - **Rationale:** Cinematics, cutscenes, and synchronized schedules require multiple tracks
     sharing a single clock.
   - **Verification:** Unit test: load a multi-track timeline with camera and audio tracks; advance
     time; assert both tracks evaluate at the same timestamp.
2. **R-17.4.4** — The engine **SHALL** evaluate 32 active tracks on a single timeline within 0.5 ms
   per frame.
   - **Rationale:** Complex cutscenes stack many tracks (camera, multiple actors, audio, VFX,
     subtitles) and must still evaluate within frame budget.
   - **Verification:** Benchmark: construct a timeline with 32 tracks; advance one frame; assert
     total time under 0.5 ms.
3. **R-17.4.5** — The engine **SHALL** advance 1,000 active playback states in under 0.5 ms per
   frame.
   - **Rationale:** Ambient timelines (schedules, lights, environmental loops) may run in large
     numbers simultaneously; aggregate advance cost must fit in the frame.
   - **Verification:** Benchmark: spawn 1,000 PlaybackState components; run one advance pass; assert
     total time under 0.5 ms.

## Playback State

1. **R-17.4.6** — The engine **SHALL** store per-entity playback state as a mutable ECS component
   containing current time, play/pause, speed multiplier, direction, and loop count, distinct from
   the immutable timeline asset.
   - **Rationale:** Many entities can share one timeline asset with independent progress; separating
     state from definition enables reuse and zero-copy access.
   - **Verification:** Unit test: bind one timeline to three entities; advance each differently;
     assert each has independent current time.

## Cutscenes

1. **R-17.4.7** — The engine **SHALL** provide a cutscene entity owning synchronized child timelines
   for camera, actors, audio, VFX, and subtitles with master clock, actor binding by role name,
   player control handoff, skip with side-effect evaluation, and dialogue branching.
   - **Rationale:** Cutscenes are a composite feature on top of timelines; the composition itself
     must be engine-level so designers can author cinematics without code.
   - **Verification:** Integration test: author a cutscene with camera + audio + actor; play; assert
     synchronized execution. Skip mid-playback; assert terminal state reached.

## Animation Integration

1. **R-17.4.8** — The engine **SHALL** support timeline tracks driving animation playback, bone
   transform overrides, morph target weights, and IK targets with priority-based blending against
   gameplay animation.
   - **Rationale:** Cinematics must override gameplay animation while preserving character
     authoring; blend priority avoids hardcoded override semantics.
   - **Verification:** Integration test: run a cutscene with a bone override track while gameplay
     animation is active; assert blended pose matches expected priority weights.

## Property Animation

1. **R-17.4.9** — The engine **SHALL** animate any ECS component field via timeline tracks using
   codegen'd binding functions, supporting transform, material, light, camera, audio, UI, and custom
   component targets.
   - **Rationale:** Property animation is universal; codegen eliminates manual binding boilerplate
     and keeps the binding fast.
   - **Verification:** Integration test: bind a Track<Color> to a Light.color field; advance the
     timeline; assert the light color changes to the expected sampled value each frame.

## Scrubbing

1. **R-17.4.10** — The engine **SHALL** evaluate all timeline tracks at an arbitrary time T and
   write results to ECS components for editor viewport scrubbing, with keyframe creation on entity
   property edits at the current scrubber time.
   - **Rationale:** Scrubber-based authoring requires fast seek-and-evaluate and record-on-edit to
     feel like a video editor.
   - **Verification:** Integration test: scrub a loaded timeline; assert entities at their animated
     positions. Edit a property; assert a new keyframe is inserted at the current scrubber time.

## Events

1. **R-17.4.11** — The engine **SHALL** fire timeline events at keyframe crossings that trigger
   audio cues, VFX spawns, dialogue entries, quest state changes, and UI transitions in consuming
   subsystems through the ECS event channel.
   - **Rationale:** Timelines as producers must notify consuming systems of significant moments; one
     event channel decouples the producer from every consumer.
   - **Verification:** Integration test: author a track with a "trigger" keyframe at 2s; advance
     past 2s; assert exactly one timeline event emitted.

## Persistence

1. **R-17.4.12** — The engine **SHALL** save and restore playback state via rkyv serialization,
   preserving current time, direction, and loop count bit-identically across save/load.
   - **Rationale:** Ongoing cinematics and schedules must resume after load at the exact position
     they were saved.
   - **Verification:** Round-trip test: save a playing timeline mid-advance; reload; assert state
     byte-identical and playback continues from the saved time.
