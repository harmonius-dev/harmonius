# Proposed Requirements — Haptic Effect Authoring

## Haptic Channel Mixer

| ID       | Derived From |
|----------|--------------|
| R-6.4.6  | F-6.4.6      |
| R-6.4.6a | F-6.4.6      |

1. **R-6.4.6** — The engine **SHALL** mix multiple concurrent haptic effects across configurable
   named channels with per-channel priority, volume (0.0-1.0), and blend mode (additive, override,
   weighted). The mixer **SHALL** resolve all active effects into a single output value per actuator
   per frame. When two effects on the same channel have equal priority, the most recently triggered
   effect wins.
   - **Rationale:** Games routinely fire overlapping haptic events (footsteps + explosions +
     ambient). Without a mixer, effects stomp each other unpredictably.
   - **Verification:** Unit test: play two effects on the same channel at priorities 3 and 5. Assert
     priority-5 output dominates. Play two additive effects at 0.4 and 0.5. Assert combined output
     is 0.9.
2. **R-6.4.6a** — The haptic channel mixer **SHALL** complete its per-frame mixing pass in under 50
   microseconds for up to 8 simultaneous effects on 4 channels, measured on a single core.
   - **Rationale:** The mixer runs every frame; it must not contribute meaningful cost to the frame
     budget.
   - **Verification:** Benchmark: activate 8 effects across 4 channels. Measure mixer system
     duration over 1000 frames. Assert p99 is under 50 us.

## Haptic Effect Hot Reload

| ID      | Derived From |
|---------|--------------|
| R-6.4.7 | F-6.4.7      |

1. **R-6.4.7** — The engine **SHALL** detect changes to haptic effect assets on disk and hot-reload
   them within 200 ms in editor builds. Active effects referencing the changed asset **SHALL**
   restart with the updated data on the next frame. The reload **SHALL** not cause audible or
   tactile glitches on the active controller.
   - **Rationale:** Designers need instant tactile feedback when iterating on haptic waveforms. A
     restart-free workflow keeps them in flow.
   - **Verification:** Integration test: play a looping rumble effect. Modify the asset file on
     disk. Assert the controller output changes within 200 ms. Assert no frame spike above 1 ms from
     the reload.

## Haptic Waveform Editor

| ID          | Derived From |
|-------------|--------------|
| R-15.21.13  | F-15.21.13   |
| R-15.21.13a | F-15.21.13   |

1. **R-15.21.13** — The editor **SHALL** provide a visual timeline for authoring haptic effects with
   separate tracks for low-frequency rumble, high-frequency rumble, HD haptic waveform, and adaptive
   trigger effects. Each track **SHALL** support keyframe placement, bezier curve handles, and
   snap-to-grid. The timeline **SHALL** display a per-track platform compatibility indicator showing
   native support (green), fallback support (yellow), or unsupported (red) for Xbox, DualSense, and
   Switch controllers.
   - **Rationale:** Designers need visual, no-code authoring for haptic effects. Platform indicators
     prevent authoring effects that silently produce no output on a target controller.
   - **Verification:** Integration test: open the waveform editor. Create keyframes on the low-freq
     rumble track. Assert the curve renders. Verify the Xbox indicator shows green, Switch shows
     yellow (fallback from HD to rumble).
2. **R-15.21.13a** — The haptic waveform editor **SHALL** render the timeline at 60 fps with up to
   200 keyframes across 4 tracks without dropping below 60 fps on the minimum spec editor machine.
   - **Rationale:** Smooth editing experience requires responsive UI even with dense keyframe data.
   - **Verification:** Benchmark: load a haptic asset with 200 keyframes across 4 tracks. Measure
     editor frame time. Assert no frame exceeds 16.6 ms.

## Haptic Live Preview

| ID          | Derived From |
|-------------|--------------|
| R-15.21.14  | F-15.21.14   |
| R-15.21.14a | F-15.21.14   |

1. **R-15.21.14** — The editor **SHALL** play the current haptic effect on a connected controller
   when the designer presses "Play" in the waveform editor. The preview **SHALL** support loop mode
   for continuous playback during editing. The preview **SHALL** detect connected controller type
   and display which tracks play natively versus fallback. Multi-controller preview **SHALL** be
   supported when multiple controllers are connected simultaneously.
   - **Rationale:** Tactile feedback cannot be evaluated visually. Designers must feel the effect on
     real hardware to judge quality.
   - **Verification:** Integration test: connect a DualSense. Open a haptic effect with HD haptic
     and rumble tracks. Press Play. Assert the DualSense receives output on both tracks. Connect an
     Xbox controller. Press Play. Assert it receives rumble-only output.
2. **R-15.21.14a** — The latency between pressing "Play" and the first haptic output on the
   connected controller **SHALL** be under 30 ms.
   - **Rationale:** Perceptible delay between pressing Play and feeling the effect breaks the
     authoring flow.
   - **Verification:** Measure timestamp delta between Play button event and first HID output
     report. Assert under 30 ms.

## Haptic Effect Template Library

| ID         | Derived From |
|------------|--------------|
| R-15.21.15 | F-15.21.15   |

1. **R-15.21.15** — The editor **SHALL** provide a browsable library of at least 30 built-in haptic
   effect templates organized by category (combat, locomotion, environment, UI, vehicle). Each
   template **SHALL** include pre-configured fallback chains for Xbox, DualSense, and Switch
   controllers. Designers **SHALL** be able to save custom effects to the library as project-local
   templates. The library **SHALL** support search by name, category, and tag.
   - **Rationale:** Starting from a template is faster than authoring from scratch. Built-in
     templates teach designers the authoring patterns and fallback conventions.
   - **Verification:** Integration test: open the template library. Assert at least 30 templates are
     listed. Filter by "combat". Assert results contain only combat templates. Load a template.
     Assert fallback chains exist for all three controller families.

## Per-Platform Haptic Testing

| ID          | Derived From |
|-------------|--------------|
| R-15.21.16  | F-15.21.16   |
| R-15.21.16a | F-15.21.16   |

1. **R-15.21.16** — The editor **SHALL** provide platform simulation modes (Xbox, DualSense, Switch)
   that play a haptic effect using only the tracks and fallbacks that platform would use at runtime.
   When the matching controller is connected, simulation plays on hardware. When not connected, the
   editor **SHALL** display a visual waveform preview of the simulated output.
   - **Rationale:** Designers rarely have all three controller types on their desk. Simulation modes
     let them verify fallback behavior without hardware.
   - **Verification:** Integration test: load a profile with HD haptics and adaptive triggers.
     Select "Simulate Xbox". Assert only dual-motor rumble tracks appear in the output preview.
     Select "Simulate DualSense". Assert all tracks appear.
2. **R-15.21.16a** — The visual waveform preview in simulation mode **SHALL** accurately represent
   the platform-converted output within 5% amplitude tolerance compared to actual hardware output.
   - **Rationale:** If the visual preview diverges from real hardware, designers cannot trust the
     simulation.
   - **Verification:** Record hardware output from a DualSense. Compare to the visual preview
     waveform. Assert amplitude difference is within 5% at all sample points.

## Haptic Debug Overlay

| ID         | Derived From |
|------------|--------------|
| R-15.21.17 | F-15.21.17   |

1. **R-15.21.17** — The engine **SHALL** provide a toggleable debug overlay in editor play mode and
   development builds that displays: active haptic effect names, per-motor intensity as bar graphs,
   HD haptic amplitude as a scrolling waveform, adaptive trigger state, channel mixer output, effect
   priorities, and remaining durations. The overlay **SHALL** be toggled via debug console command
   or editor menu.
   - **Rationale:** Haptic output is invisible. Without visualization, debugging why an effect does
     not feel right requires guesswork.
   - **Verification:** Integration test: enable the overlay. Trigger a haptic effect. Assert the
     overlay shows the effect name, motor intensity bars updating in real time, and remaining
     duration counting down. Assert overlay rendering adds less than 0.5 ms to frame time.
