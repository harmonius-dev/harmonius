# 17.4 — Timelines

## Tracks and Keyframes

| ID       | Feature                     |
|----------|-----------------------------|
| F-17.4.1 | Generic Track Primitive    |
| F-17.4.2 | Fast Per-Sample Evaluation |

1. **F-17.4.1** — `Track<T>` is a generic named channel of timestamped `Keyframe<T>` values with
   per-keyframe interpolation mode (linear, step, cubic, bezier) and a default value. One primitive
   serves camera, audio, animation, UI, and custom property animation.
   - **Deps:** F-1.1.1, F-1.4.1
2. **F-17.4.2** — Single-sample evaluation runs in under 100 ns per call on desktop hardware,
   supporting dense tracks at thousands of active timelines.
   - **Deps:** F-17.4.1

## Multi-Track Timelines

| ID       | Feature                     |
|----------|-----------------------------|
| F-17.4.3 | MultiTrackTimeline Asset   |
| F-17.4.4 | 32-Track Evaluation Budget |
| F-17.4.5 | 1K Playback Advance Budget |

1. **F-17.4.3** — `MultiTrackTimeline` is an immutable asset containing multiple tracks that play in
   sync. Authored in the visual sequencer editor.
   - **Deps:** F-17.4.1, F-1.6
2. **F-17.4.4** — 32-track timelines evaluate within 0.5 ms per frame, supporting complex cutscenes
   stacking camera, actors, audio, VFX, and subtitles.
   - **Deps:** F-17.4.3
3. **F-17.4.5** — 1,000 active PlaybackStates advance in under 0.5 ms per frame. Ambient timelines,
   schedules, and environmental loops scale to crowd sizes.
   - **Deps:** F-17.4.6

## Playback State

| ID       | Feature                       |
|----------|-------------------------------|
| F-17.4.6 | Per-Entity PlaybackState     |

1. **F-17.4.6** — PlaybackState is a mutable ECS component holding current time, play/pause, speed
   multiplier, direction, and loop count. Many entities share one timeline asset with independent
   progress.
   - **Deps:** F-17.4.3, F-1.1.1

## Cutscenes

| ID       | Feature                        |
|----------|--------------------------------|
| F-17.4.7 | Composable Cutscene Entities  |

1. **F-17.4.7** — Cutscene entities own synchronized child timelines for camera, actors, audio, VFX,
   and subtitles with master clock, actor binding by role name, player control handoff, skip with
   side-effect evaluation, and dialogue branching.
   - **Deps:** F-17.4.3, F-17.4.6, F-13.5 (Cinematics)

## Animation Integration

| ID       | Feature                        |
|----------|--------------------------------|
| F-17.4.8 | Timeline-Driven Animation     |

1. **F-17.4.8** — Timeline tracks drive animation playback, bone transform overrides, morph target
   weights, and IK targets with priority-based blending against gameplay animation.
   - **Deps:** F-17.4.3, F-9.4.1 (Animation State Machine)

## Property Animation

| ID       | Feature                        |
|----------|--------------------------------|
| F-17.4.9 | Universal Property Animation  |

1. **F-17.4.9** — Any ECS component field can be animated via timeline tracks using codegen'd
   binding functions. Transform, material, light, camera, audio, UI, and custom components all
   animate uniformly.
   - **Deps:** F-17.4.3, F-1.1.1, F-1.3

## Scrubbing

| ID        | Feature                       |
|-----------|-------------------------------|
| F-17.4.10 | Editor Scrubber Support      |

1. **F-17.4.10** — The engine evaluates all tracks at arbitrary time T and writes results to ECS
   components for editor viewport scrubbing. Editing a property at the scrubber time inserts a new
   keyframe.
   - **Deps:** F-17.4.3, F-15 (Editor)

## Events

| ID        | Feature                        |
|-----------|--------------------------------|
| F-17.4.11 | Keyframe Event Dispatch       |

1. **F-17.4.11** — Timeline events fire at keyframe crossings through the ECS event channel,
   triggering audio cues, VFX spawns, dialogue entries, quest state changes, and UI transitions in
   consuming subsystems.
   - **Deps:** F-17.4.3, F-1.5.1

## Persistence

| ID        | Feature                        |
|-----------|--------------------------------|
| F-17.4.12 | Playback State Persistence    |

1. **F-17.4.12** — PlaybackState saves and restores via rkyv, preserving current time, direction,
   and loop count bit-identically across save/load.
   - **Deps:** F-17.4.6, F-1.4.1
