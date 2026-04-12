# Timelines User Stories

## Tracks and Keyframes

| ID        | Persona                 |
|-----------|-------------------------|
| US-17.4.1 | game developer (P-15)   |
| US-17.4.2 | engine developer (P-26) |

1. **US-17.4.1** — **As a** game developer (P-15), **I want** a generic Track<T> primitive with
   linear, step, cubic, and bezier interpolation modes, **so that** camera, audio, animation, and UI
   property animation all share one type.
2. **US-17.4.2** — **As an** engine developer (P-26), **I want** per-sample track evaluation in
   under 100 ns, **so that** dense tracks on thousands of active timelines stay within frame budget.

## Multi-Track Timelines

| ID        | Persona                 |
|-----------|-------------------------|
| US-17.4.3 | game designer (P-5)     |
| US-17.4.4 | engine developer (P-26) |
| US-17.4.5 | engine developer (P-26) |

1. **US-17.4.3** — **As a** game designer (P-5), **I want** MultiTrackTimeline assets containing
   multiple synchronized tracks authored in a visual sequencer editor, **so that** I can compose
   cutscenes without writing code.
2. **US-17.4.4** — **As an** engine developer (P-26), **I want** 32-track timelines to evaluate in
   under 0.5 ms per frame, **so that** complex cutscenes with camera, actors, audio, and VFX stay in
   budget.
3. **US-17.4.5** — **As an** engine developer (P-26), **I want** 1,000 PlaybackStates to advance in
   under 0.5 ms per frame, **so that** ambient timelines and schedules scale to crowd sizes.

## Playback State

| ID        | Persona               |
|-----------|-----------------------|
| US-17.4.6 | game developer (P-15) |

1. **US-17.4.6** — **As a** game developer (P-15), **I want** per-entity PlaybackState as a mutable
   component separate from the timeline asset, **so that** many entities can share one timeline with
   independent progress.

## Cutscenes

| ID        | Persona             |
|-----------|---------------------|
| US-17.4.7 | game designer (P-5) |

1. **US-17.4.7** — **As a** game designer (P-5), **I want** to author cutscenes with synchronized
   camera, actor, audio, VFX, and subtitle tracks plus master clock, skip, and branching,
   **so that** cinematic moments play out with precise timing and choice.

## Animation Integration

| ID        | Persona                   |
|-----------|---------------------------|
| US-17.4.8 | character animator (P-11) |

1. **US-17.4.8** — **As a** character animator (P-11), **I want** timeline tracks driving bone
   overrides, morph weights, and IK targets with priority-based blending against gameplay animation,
   **so that** cutscenes layer cleanly over character locomotion.

## Property Animation

| ID        | Persona                |
|-----------|------------------------|
| US-17.4.9 | level designer (P-6)   |

1. **US-17.4.9** — **As a** level designer (P-6), **I want** to animate any entity property (light
   color, door rotation, material opacity) via timeline keyframes, **so that** scripted sequences
   work without writing code.

## Scrubbing

| ID         | Persona             |
|------------|---------------------|
| US-17.4.10 | game designer (P-5) |

1. **US-17.4.10** — **As a** game designer (P-5), **I want** to scrub the timeline in the editor and
   see entities at their animated positions, **so that** I can preview and adjust timing before
   runtime.

## Events

| ID         | Persona               |
|------------|-----------------------|
| US-17.4.11 | game developer (P-15) |

1. **US-17.4.11** — **As a** game developer (P-15), **I want** timeline events fired at keyframe
   crossings to trigger audio, VFX, dialogue, quest changes, and UI transitions, **so that** the
   producing timeline can notify consumers without tight coupling.

## Persistence

| ID         | Persona      |
|------------|--------------|
| US-17.4.12 | gamer (P-23) |

1. **US-17.4.12** — **As a** gamer (P-23), **I want** in-progress cutscenes and schedules to resume
   at the exact position they were saved, **so that** save/load never skips or replays parts of a
   running sequence.
