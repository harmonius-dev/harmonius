# Proposed Features — Haptic Effect Authoring

## Context

The user wants designers to author haptic effects in the editor and have them play back on
DualSense, Xbox, and Switch controllers with platform-appropriate fallbacks.

Existing features F-6.4.1 through F-6.4.5 cover runtime haptic capabilities (rumble, adaptive
triggers, HD haptics, audio-driven haptics, force feedback profiles). The existing features
reference "authored in the visual editor" but no dedicated editor features exist for haptic
authoring. This proposal adds editor-side tooling in domain 15 (Tools & Editor) under group 15.21
(Specialized Editors), plus two new runtime features in domain 6.4 to support the authoring
workflow.

## New Runtime Features (Input — 6.4)

### Haptic Channel Mixer

| ID      | Feature                  | Requirements        |
|---------|--------------------------|---------------------|
| F-6.4.6 | Haptic Channel Mixer    | R-6.4.6, R-6.4.6a  |

1. **F-6.4.6** — Mix multiple simultaneous haptic effects across independent channels with
   per-channel priority, volume, and blend mode (additive, override, weighted). The mixer resolves
   concurrent effects (e.g., footstep rumble during an explosion) into a single output per actuator.
   Channel count is configurable per project. The mixer runs as an ECS system each frame, reading
   active haptic effect components and writing final actuator output. Designers assign effects to
   named channels in the editor.
   - **Deps:** F-6.4.1, F-6.4.3, F-6.4.5, F-1.1.1
   - **Platform:** All controller backends. Output is per-actuator after platform conversion.

### Haptic Effect Hot Reload

| ID      | Feature                     | Requirements   |
|---------|-----------------------------|----------------|
| F-6.4.7 | Haptic Effect Hot Reload   | R-6.4.7        |

1. **F-6.4.7** — Hot-reload haptic effect assets at runtime when the source asset changes on disk,
   enabling designers to edit a haptic waveform or profile in the editor and immediately feel the
   updated result on a connected controller without restarting the game. Integrates with the content
   pipeline hot-reload system (F-12.4.4). Active effects that reference the changed asset restart
   with the new data.
   - **Deps:** F-6.4.5, F-12.4.4
   - **Platform:** Editor-only. Not available in shipping builds.

## New Editor Features (Tools & Editor — 15.21)

### Haptic Waveform Editor

| ID         | Feature                 | Requirements           |
|------------|-------------------------|------------------------|
| F-15.21.13 | Haptic Waveform Editor | R-15.21.13, R-15.21.13a |

1. **F-15.21.13** — A visual timeline editor for authoring haptic waveforms and rumble patterns.
   Designers draw amplitude envelopes, frequency curves, and motor intensity keyframes on a timeline
   with handles, bezier curves, and snap-to-grid. The timeline supports multiple tracks:
   low-frequency rumble, high-frequency rumble, HD haptic waveform, and adaptive trigger. Each track
   shows a platform compatibility indicator (green/yellow/red) reflecting whether the target
   controller supports that track natively, via fallback, or not at all. Undo/redo integrates with
   the editor stack (F-15.1.3).
   - **Deps:** F-15.1.3, F-15.1.4, F-6.4.1, F-6.4.3, F-6.4.5
   - **Platform:** Desktop editor only.

### Haptic Live Preview

| ID         | Feature              | Requirements           |
|------------|----------------------|------------------------|
| F-15.21.14 | Haptic Live Preview | R-15.21.14, R-15.21.14a |

1. **F-15.21.14** — Real-time haptic preview on a connected controller while editing in the waveform
   editor. A "Play" button sends the current waveform to the active controller immediately. A "Loop"
   mode replays continuously while the designer adjusts curves. The preview system detects which
   controller is connected and shows the designer which tracks will play natively versus fallback.
   Supports simultaneous preview on multiple connected controllers (e.g., DualSense and Xbox side by
   side) to compare feel.
   - **Deps:** F-15.21.13, F-6.4.5, F-6.1.5
   - **Platform:** Desktop editor only. Requires a connected controller.

### Haptic Effect Template Library

| ID         | Feature                        | Requirements  |
|------------|--------------------------------|---------------|
| F-15.21.15 | Haptic Effect Template Library | R-15.21.15    |

1. **F-15.21.15** — A browsable library of built-in haptic effect templates organized by category
   (combat, locomotion, environment, UI, vehicle). Templates provide starting points that designers
   customize rather than authoring from scratch. Each template includes pre-configured fallback
   chains for all three controller families. Designers can save custom effects back to the library
   as project-local or shared templates. Categories and tags are searchable.
   - **Deps:** F-15.21.13, F-6.4.5
   - **Platform:** Desktop editor only.

### Per-Platform Haptic Testing

| ID         | Feature                       | Requirements           |
|------------|-------------------------------|------------------------|
| F-15.21.16 | Per-Platform Haptic Testing  | R-15.21.16, R-15.21.16a |

1. **F-15.21.16** — A testing panel that lets designers play a haptic effect as each platform would
   render it, even without that controller connected. A "Simulate Xbox" mode plays only the
   dual-motor rumble fallback. A "Simulate DualSense" mode enables adaptive triggers and HD haptics.
   A "Simulate Switch" mode uses HD Rumble conversion. When a matching controller is connected, the
   simulation plays on hardware. When no matching controller is connected, the panel shows a visual
   waveform preview of what the output would be.
   - **Deps:** F-15.21.13, F-15.21.14, F-6.4.5
   - **Platform:** Desktop editor only.

### Haptic Debug Overlay

| ID         | Feature               | Requirements  |
|------------|-----------------------|---------------|
| F-15.21.17 | Haptic Debug Overlay | R-15.21.17    |

1. **F-15.21.17** — A runtime debug overlay that visualizes active haptic effects as waveform
   graphs, showing per-motor intensity, HD haptic amplitude, and adaptive trigger state in real
   time. The overlay displays the channel mixer output, active effect names, priorities, and
   remaining durations. Available in editor play mode and development builds. Toggled via a debug
   console command or editor menu.
   - **Deps:** F-6.4.6, F-15.5.6
   - **Platform:** Desktop and console development builds.

## Summary of Proposed Features

| ID         | Name                           | Domain       |
|------------|--------------------------------|--------------|
| F-6.4.6    | Haptic Channel Mixer           | Input        |
| F-6.4.7    | Haptic Effect Hot Reload       | Input        |
| F-15.21.13 | Haptic Waveform Editor         | Tools/Editor |
| F-15.21.14 | Haptic Live Preview            | Tools/Editor |
| F-15.21.15 | Haptic Effect Template Library | Tools/Editor |
| F-15.21.16 | Per-Platform Haptic Testing    | Tools/Editor |
| F-15.21.17 | Haptic Debug Overlay           | Tools/Editor |

## Feature Dependency Graph

```text
F-6.4.1 (Rumble) ──┐
F-6.4.3 (HD)    ───┼─► F-6.4.6 (Channel Mixer)
F-6.4.5 (Profiles) ┘         │
       │                      │
       ├─► F-6.4.7 (Hot Reload)
       │         │
       ▼         ▼
F-15.21.13 (Waveform Editor)
       │
       ├─► F-15.21.14 (Live Preview)
       │         │
       ├─► F-15.21.15 (Template Library)
       │         │
       ├─► F-15.21.16 (Platform Testing)
       │
F-6.4.6 ──► F-15.21.17 (Debug Overlay)
```
