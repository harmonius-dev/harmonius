# Proposed User Stories — Haptic Effect Authoring

## Haptic Channel Mixer

| ID        | Persona                 | Features | Requirements |
|-----------|-------------------------|----------|--------------|
| US-6.4.6.1 | designer (P-5)        | F-6.4.6  | R-6.4.6      |
| US-6.4.6.2 | engine developer (P-26) | F-6.4.6 | R-6.4.6      |
| US-6.4.6.3 | player (P-23)         | F-6.4.6  | R-6.4.6      |
| US-6.4.6.4 | engine tester (P-27)  | F-6.4.6  | R-6.4.6a     |

1. **US-6.4.6.1** — As a designer, I want to assign haptic effects to named channels with
   priorities, so that overlapping effects blend predictably instead of stomping each other.
   - **Acceptance:** Two effects on different channels play simultaneously. An effect with higher
     priority overrides a lower-priority effect on the same channel.
2. **US-6.4.6.2** — As an engine developer, I want the haptic mixer to run as an ECS system with
   configurable blend modes, so that I can extend it with custom blend strategies.
   - **Acceptance:** The mixer reads HapticEffect components and writes final output per actuator
     each frame. Additive, override, and weighted modes produce correct output.
3. **US-6.4.6.3** — As a player, I want to feel footstep rumble and explosion impact at the same
   time, so that tactile feedback matches the intensity of what is happening on screen.
   - **Acceptance:** Walking during an explosion produces combined rumble that includes both
     effects, not just the explosion alone.
4. **US-6.4.6.4** — As an engine tester, I want to verify the mixer resolves 8 concurrent effects in
   under 50 microseconds, so that haptic mixing does not impact frame budget.
   - **Acceptance:** Benchmark with 8 effects on 4 channels shows p99 mixer duration under 50 us.

## Haptic Effect Hot Reload

| ID        | Persona                 | Features | Requirements |
|-----------|-------------------------|----------|--------------|
| US-6.4.7.1 | designer (P-5)        | F-6.4.7  | R-6.4.7      |
| US-6.4.7.2 | engine developer (P-26) | F-6.4.7 | R-6.4.7      |
| US-6.4.7.3 | engine tester (P-27)  | F-6.4.7  | R-6.4.7      |

1. **US-6.4.7.1** — As a designer, I want to edit a haptic waveform and immediately feel the change
   on my connected controller without restarting the game, so that I can iterate on haptic feel in
   real time.
   - **Acceptance:** Modify a haptic asset while the game runs. The controller output updates within
     200 ms with no tactile glitch.
2. **US-6.4.7.2** — As an engine developer, I want haptic hot-reload to integrate with the content
   pipeline's existing hot-reload system, so that I do not need a separate file-watching mechanism.
   - **Acceptance:** Haptic assets use the same hot-reload path as other assets (F-12.4.4). No
     additional file watchers are created.
3. **US-6.4.7.3** — As an engine tester, I want to verify that hot-reloading a haptic asset does not
   cause a frame spike above 1 ms, so that live editing does not disrupt gameplay.
   - **Acceptance:** Measure frame time during hot-reload. Assert no frame exceeds baseline + 1 ms.

## Haptic Waveform Editor

| ID           | Persona                | Features   | Requirements  |
|--------------|------------------------|------------|---------------|
| US-15.21.13.1 | designer (P-5)       | F-15.21.13 | R-15.21.13    |
| US-15.21.13.2 | designer (P-5)       | F-15.21.13 | R-15.21.13    |
| US-15.21.13.3 | engine tester (P-27) | F-15.21.13 | R-15.21.13a   |
| US-15.21.13.4 | audio designer (P-14) | F-15.21.13 | R-15.21.13   |
| US-15.21.13.5 | QA tester (P-19)     | F-15.21.13 | R-15.21.13    |

1. **US-15.21.13.1** — As a designer, I want to draw haptic amplitude curves on a visual timeline
   with keyframes and bezier handles, so that I can author haptic effects without writing code.
   - **Acceptance:** Open the waveform editor. Place keyframes on the low-freq rumble track. Drag
     bezier handles. The curve updates visually in real time.
2. **US-15.21.13.2** — As a designer, I want each timeline track to show a color-coded platform
   compatibility indicator, so that I know which controllers will play each track natively, via
   fallback, or not at all.
   - **Acceptance:** The adaptive trigger track shows green for DualSense, red for Xbox and Switch.
     The rumble track shows green for all three.
3. **US-15.21.13.3** — As an engine tester, I want to verify the waveform editor maintains 60 fps
   with 200 keyframes across 4 tracks, so that the editing experience remains smooth.
   - **Acceptance:** Load a dense haptic asset. Assert frame time stays under 16.6 ms throughout
     editing.
4. **US-15.21.13.4** — As an audio designer, I want to author haptic waveforms alongside the audio
   waveform of the triggering sound effect, so that I can synchronize tactile and auditory feedback.
   - **Acceptance:** The waveform editor shows an audio reference track. Haptic keyframes can be
     snapped to audio transients.
5. **US-15.21.13.5** — As a QA tester, I want to verify that undo/redo works for all waveform editor
   operations, so that designers can safely experiment without losing work.
   - **Acceptance:** Add keyframes, adjust curves, undo all changes. Assert the timeline returns to
     its original state.

## Haptic Live Preview

| ID           | Persona                | Features   | Requirements  |
|--------------|------------------------|------------|---------------|
| US-15.21.14.1 | designer (P-5)       | F-15.21.14 | R-15.21.14    |
| US-15.21.14.2 | designer (P-5)       | F-15.21.14 | R-15.21.14    |
| US-15.21.14.3 | engine tester (P-27) | F-15.21.14 | R-15.21.14a   |
| US-15.21.14.4 | QA tester (P-19)     | F-15.21.14 | R-15.21.14    |

1. **US-15.21.14.1** — As a designer, I want to press "Play" in the waveform editor and feel the
   effect on my connected controller immediately, so that I can judge the tactile quality without
   running the game.
   - **Acceptance:** Press Play with a DualSense connected. Feel haptic output within 30 ms. The
     effect matches the timeline visually.
2. **US-15.21.14.2** — As a designer, I want loop mode so the effect repeats continuously while I
   adjust curves, so that I can tune the feel interactively.
   - **Acceptance:** Enable loop mode. Adjust a keyframe. The next loop iteration reflects the
     change. Output is continuous with no gap between loops.
3. **US-15.21.14.3** — As an engine tester, I want to verify that play-to-output latency is under 30
   ms, so that the preview feels instantaneous.
   - **Acceptance:** Measure timestamp from Play press to first HID output. Assert under 30 ms.
4. **US-15.21.14.4** — As a QA tester, I want to preview on two controllers simultaneously
   (DualSense and Xbox), so that I can compare cross-platform feel side by side.
   - **Acceptance:** Connect both controllers. Press Play. Both produce output. DualSense plays HD
     haptics; Xbox plays rumble-only fallback.

## Haptic Effect Template Library

| ID           | Persona                | Features   | Requirements |
|--------------|------------------------|------------|--------------|
| US-15.21.15.1 | designer (P-5)       | F-15.21.15 | R-15.21.15   |
| US-15.21.15.2 | designer (P-5)       | F-15.21.15 | R-15.21.15   |
| US-15.21.15.3 | engine tester (P-27) | F-15.21.15 | R-15.21.15   |
| US-15.21.15.4 | prototyper (P-7)     | F-15.21.15 | R-15.21.15   |

1. **US-15.21.15.1** — As a designer, I want to browse a library of haptic effect templates by
   category, so that I can start from a proven pattern instead of authoring from scratch.
   - **Acceptance:** Open the template library. Browse by combat, locomotion, environment. Each
     template loads into the waveform editor with all tracks populated.
2. **US-15.21.15.2** — As a designer, I want to save my custom haptic effects back to the library as
   project templates, so that my team can reuse them.
   - **Acceptance:** Author a custom effect. Save to library with a name and category tag. The
     template appears in the library for all team members.
3. **US-15.21.15.3** — As an engine tester, I want to verify every built-in template has valid
   fallback chains for all three controller families, so that no template produces zero output on
   any supported controller.
   - **Acceptance:** Load each of the 30+ templates. Assert every template has at least one active
     track for Xbox, DualSense, and Switch.
4. **US-15.21.15.4** — As a prototyper, I want to drag a template onto a gameplay event in the logic
   graph to get haptic feedback working in minutes, so that I can validate game feel quickly.
   - **Acceptance:** Drag "explosion_heavy" template onto a collision event node. Play the game.
     Feel haptic feedback on impact.

## Per-Platform Haptic Testing

| ID           | Persona                | Features   | Requirements  |
|--------------|------------------------|------------|---------------|
| US-15.21.16.1 | designer (P-5)       | F-15.21.16 | R-15.21.16    |
| US-15.21.16.2 | QA tester (P-19)     | F-15.21.16 | R-15.21.16    |
| US-15.21.16.3 | engine tester (P-27) | F-15.21.16 | R-15.21.16a   |

1. **US-15.21.16.1** — As a designer, I want to select "Simulate Xbox" to hear/see only the rumble
   fallback that Xbox players would feel, so that I can verify the fallback quality without owning
   an Xbox controller.
   - **Acceptance:** Select Simulate Xbox. The preview shows only dual-motor rumble output. Adaptive
     trigger and HD haptic tracks are suppressed.
2. **US-15.21.16.2** — As a QA tester, I want to test all three platform simulation modes for every
   haptic profile in the project, so that I can verify no profile produces zero output on any
   platform.
   - **Acceptance:** Run each profile through Xbox, DualSense, and Switch simulation. Assert
     non-zero output in every case.
3. **US-15.21.16.3** — As an engine tester, I want to verify the visual simulation preview matches
   real hardware output within 5% amplitude, so that the simulation is trustworthy.
   - **Acceptance:** Compare simulation waveform to recorded hardware output. Assert amplitude
     difference is within 5% at all sample points.

## Haptic Debug Overlay

| ID           | Persona                  | Features   | Requirements |
|--------------|--------------------------|------------|--------------|
| US-15.21.17.1 | designer (P-5)         | F-15.21.17 | R-15.21.17   |
| US-15.21.17.2 | engine developer (P-26) | F-15.21.17 | R-15.21.17  |
| US-15.21.17.3 | engine tester (P-27)   | F-15.21.17 | R-15.21.17   |

1. **US-15.21.17.1** — As a designer, I want to toggle a debug overlay that shows active haptic
   effects and their intensities, so that I can see why a haptic effect feels wrong during
   playtesting.
   - **Acceptance:** Toggle the overlay in play mode. See effect names, motor intensity bars, and
     remaining durations updating in real time.
2. **US-15.21.17.2** — As an engine developer, I want the debug overlay to show channel mixer state
   (active channels, priorities, blend modes), so that I can diagnose haptic mixing issues.
   - **Acceptance:** The overlay shows per-channel breakdown with active effect count, blend mode,
     and resolved output value.
3. **US-15.21.17.3** — As an engine tester, I want to verify the debug overlay adds less than 0.5 ms
   to frame time, so that it does not interfere with gameplay during testing.
   - **Acceptance:** Measure frame time with overlay on vs off. Assert delta is under 0.5 ms.
