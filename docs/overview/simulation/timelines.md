# Timelines

Timed sequences: cinematics, UI animations, and property animations.

## What it covers

- Tracks: timestamped keyframes with interpolation (linear, step, cubic, Bézier).
- Immutable timeline assets: 32 tracks evaluate in <0.5 ms per frame.
- Mutable per-entity playback state: time, play/pause, speed, direction, loop count.
- Cutscenes: synchronized child timelines with master clock and role-based actor binding.
- Animation integration: timeline animations blend with gameplay animations by priority.
- Property animation: codegen'd bindings animate any ECS component field.
- Keyframe events: fired through the ECS event channel at crossing time.
- Editor scrubber: evaluate at any time and insert keyframes while playing.
- Bit-identical serialization for save/load.

## Concepts

### Tracks and Interpolation

A track is a sequence of (time, value) keyframes with per-keyframe interpolation mode. Linear
interpolation smoothly transitions between keyframes. Step holds one keyframe's value until the next
keyframe. Cubic and Bézier allow artists to tune curves. Sampling any track at arbitrary time is
<100 ns per sample.

### Timeline Assets and Playback

A timeline asset is immutable: 32 tracks with keyframes. PlaybackState is a mutable ECS component:
time, play/pause, speed, direction, loops. Multiple entities can share one timeline asset but have
independent playback states, enabling shared sequences (environmental loops) and unique instances
(per-character variations).

### Cutscenes

A cutscene entity has a master clock and child timeline players. Role names bind characters to
actors in the cutscene. Skip mechanics evaluate all remaining keyframe events up to the skip target.
Dialogue choices pause the master clock, branch to alternate timelines, then resume.

## How it fits

- See [spatial-awareness](./spatial-awareness.md) for trigger conditions based on awareness
  state.
- See [event-logs](./event-logs.md) for memory-based cinematics (NPC-witnessed events play
  back).
- Integrates with [animation](../animation/skeletal-and-states.md) for skeletal animation
  blending.
- Integrates with [rendering](../rendering/pipeline.md) for camera and UI animations.
- Integrates with [audio](../audio/music-and-voice.md) for lip-sync and dialogue timing.
