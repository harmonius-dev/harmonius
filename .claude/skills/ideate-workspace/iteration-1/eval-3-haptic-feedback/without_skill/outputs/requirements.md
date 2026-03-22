# Proposed Requirements — Controller Haptic Feedback Authoring

## Haptic Effect Editor

| ID          | Derived From |
|-------------|--------------|
| R-15.21.13  | F-15.21.13   |
| R-15.21.13a | F-15.21.13   |
| R-15.21.13b | F-15.21.13   |
| R-15.21.14  | F-15.21.14   |
| R-15.21.14a | F-15.21.14   |
| R-15.21.15  | F-15.21.15   |
| R-15.21.15a | F-15.21.15   |

1. **R-15.21.13** — The editor **SHALL** provide a multi-track timeline for authoring haptic effects
   with separate tracks for low-frequency motor, high-frequency motor, L2 adaptive trigger, R2
   adaptive trigger, and HD haptic waveform. Each track **SHALL** support keyframe editing with drag
   handles, bezier curve interpolation, and numeric value entry.
   - **Rationale:** Designers need per-channel control to craft nuanced haptic experiences across
     all controller capabilities.
   - **Verification:** Integration test: create an effect with keyframes on all five tracks. Export.
     Assert the asset contains data for all five channels with correct interpolation values.
2. **R-15.21.13a** — The editor **SHALL** allow designers to configure per-platform fallback layers
   within a single haptic effect asset. The editor **SHALL** display which layers are available on
   each controller class (DualSense, Xbox, Switch Pro) and warn when a controller class has no
   fallback producing output.
   - **Rationale:** No-code engine constraint; fallback authoring must be visual and validated at
     edit time, not build time only.
   - **Verification:** Integration test: create an effect with only adaptive trigger data. Assert
     the editor warns that Xbox and Switch Pro have no haptic output. Add a rumble fallback layer.
     Assert the warning clears.
3. **R-15.21.13b** — The editor **SHALL** integrate with undo/redo (F-15.1.3), collaborative editing
   (F-15.12.3), and the asset pipeline (F-14.1.1). Haptic effect assets **SHALL** be serialized in
   the standard asset format and versioned alongside other game assets.
   - **Rationale:** Haptic assets must participate in standard editor workflows and team
     collaboration.
   - **Verification:** Integration test: edit a haptic asset, undo, assert previous state restored.
     Save, reload, assert identical asset data.
4. **R-15.21.14** — The editor **SHALL** provide a drawable waveform canvas where designers paint
   haptic waveforms freehand or select preset curves (sine, sawtooth, noise, impulse). The canvas
   **SHALL** display frequency (0-500 Hz) and amplitude (0.0-1.0) axes with configurable time scale
   (10 ms to 5 s per screen width).
   - **Rationale:** HD haptics require waveform-level authoring that goes beyond simple keyframe
     envelopes.
   - **Verification:** Integration test: draw a freehand waveform. Export. Assert output samples
     match drawn points within 5% amplitude tolerance. Select sine preset at 120 Hz. Assert output
     frequency is 120 Hz +/- 2 Hz.
5. **R-15.21.14a** — The waveform canvas **SHALL** support layering multiple waveform segments with
   configurable crossfade regions and envelope modifiers (attack, sustain, decay). Segments
   **SHALL** be individually selectable, movable, and deletable.
   - **Rationale:** Complex haptic textures (e.g., engine rumble with gear shifts) require layered
     waveform composition.
   - **Verification:** Integration test: create two overlapping segments with 50 ms crossfade.
     Assert blended output in the overlap region equals the average of both segments.
6. **R-15.21.15** — The editor **SHALL** display a split-pane preview showing simulated haptic
   output for DualSense, Xbox, and Switch Pro simultaneously. Each pane **SHALL** render animated
   intensity graphs for active motors/actuators, updated at 60 fps minimum.
   - **Rationale:** Designers must see how fallback degradation affects each controller without
     switching physical hardware.
   - **Verification:** Integration test: load a profile with all layers. Assert all three panes
     render. Assert DualSense pane shows five active channels, Xbox shows two, Switch shows HD
     Rumble output.
7. **R-15.21.15a** — When a physical controller is connected, the preview panel **SHALL** play
   haptic output on the real hardware with latency under 20 ms from the editor play button press to
   first haptic output. The preview **SHALL** loop the effect with a configurable pause between
   repetitions.
   - **Rationale:** Visual simulation cannot replace physical feel; designers must iterate on real
     hardware.
   - **Verification:** Integration test: connect a DualSense controller. Press play on a rumble
     effect. Measure time from button press to first HID output report. Assert under 20 ms.

## Haptic Effect Library

| ID       | Derived From |
|----------|--------------|
| R-6.4.6  | F-6.4.6      |
| R-6.4.6a | F-6.4.6      |
| R-6.4.7  | F-6.4.7      |
| R-6.4.7a | F-6.4.7      |
| R-6.4.8  | F-6.4.8      |
| R-6.4.8a | F-6.4.8      |

1. **R-6.4.6** — The engine **SHALL** ship a built-in library of categorized haptic effect assets
   (combat, locomotion, environment, UI) with pre-authored platform layers for DualSense, Xbox, and
   Switch Pro. Each library asset **SHALL** pass fallback chain validation (R-6.4.5a) at build time.
   - **Rationale:** A starter library reduces time-to-first-haptic and provides reference
     implementations for designers.
   - **Verification:** Unit test: load every library asset. Assert each has valid fallback chains
     for all three controller classes. Assert category metadata is present and non-empty.
2. **R-6.4.6a** — The library **SHALL** be searchable by name, category, and tag. Designers
   **SHALL** be able to preview any library asset in the haptic effect editor and use it as a
   template for custom effects via a "duplicate and edit" action.
   - **Rationale:** Discoverability and reuse reduce authoring time; templates accelerate learning.
   - **Verification:** Integration test: search for "explosion." Assert at least one result in the
     "environment" category. Duplicate it. Assert a new editable asset is created with identical
     data.
3. **R-6.4.7** — The engine **SHALL** provide a logic graph node that binds float, vector, and enum
   input pins to haptic profile parameters (intensity scale, frequency shift, duration multiplier,
   motor balance). The node **SHALL** apply bindings at runtime with per-frame update latency under
   1 ms.
   - **Rationale:** Dynamic haptic response (e.g., rumble scaling with vehicle speed) requires
     parameter binding without code.
   - **Verification:** Unit test: bind a float input at 0.5 to intensity scale. Play profile. Assert
     motor intensity equals 50% of base. Change input to 1.0. Assert intensity equals 100%. Measure
     binding evaluation time. Assert under 1 ms.
4. **R-6.4.7a** — The parameter binding node **SHALL** support curve remapping with linear,
   exponential, and step transfer functions. Designers **SHALL** author remap curves visually in the
   logic graph editor.
   - **Rationale:** Raw gameplay values rarely map 1:1 to ideal haptic intensities; curve shaping
     enables feel tuning.
   - **Verification:** Unit test: configure exponential remap with exponent 2.0. Input 0.5. Assert
     output is 0.25. Input 1.0. Assert output is 1.0.
5. **R-6.4.8** — The engine **SHALL** support live haptic parameter tuning during play mode,
   propagating editor changes to the running game session within 50 ms without requiring level
   reload or game restart.
   - **Rationale:** Tactile feel is subjective and requires rapid iteration; reloading breaks flow
     and wastes time.
   - **Verification:** Integration test: enter play mode with a haptic profile active. Change rumble
     intensity in the editor. Measure time until the running game applies the new value. Assert
     under 50 ms.
6. **R-6.4.8a** — Live tuning changes **SHALL** be ephemeral by default. The editor **SHALL**
   provide explicit "apply to asset" and "discard changes" actions. Applying writes back to the
   source asset via the standard asset pipeline.
   - **Rationale:** Exploratory tuning must not accidentally corrupt authored assets; explicit
     commit prevents data loss.
   - **Verification:** Integration test: change a parameter during live tuning. Stop play mode
     without applying. Assert asset retains original values. Repeat and apply. Assert asset reflects
     the new value.
