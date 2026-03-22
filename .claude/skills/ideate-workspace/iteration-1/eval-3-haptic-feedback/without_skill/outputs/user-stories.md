# Proposed User Stories — Controller Haptic Feedback Authoring

## Personas Referenced

| ID   | Persona          |
|------|------------------|
| P-5  | designer         |
| P-19 | QA tester        |
| P-23 | player           |
| P-26 | engine developer |
| P-27 | engine tester    |

## Haptic Effect Editor

| ID            | Persona                 | Features   | Requirements  |
|---------------|-------------------------|------------|---------------|
| US-15.21.13.1 | designer (P-5)          | F-15.21.13 | R-15.21.13    |
| US-15.21.13.2 | designer (P-5)          | F-15.21.13 | R-15.21.13a   |
| US-15.21.13.3 | designer (P-5)          | F-15.21.13 | R-15.21.13b   |
| US-15.21.13.4 | engine developer (P-26) | F-15.21.13 | R-15.21.13    |
| US-15.21.13.5 | engine tester (P-27)    | F-15.21.13 | R-15.21.13    |
| US-15.21.13.6 | QA tester (P-19)        | F-15.21.13 | R-15.21.13a   |
| US-15.21.14.1 | designer (P-5)          | F-15.21.14 | R-15.21.14    |
| US-15.21.14.2 | designer (P-5)          | F-15.21.14 | R-15.21.14a   |
| US-15.21.14.3 | engine tester (P-27)    | F-15.21.14 | R-15.21.14    |
| US-15.21.14.4 | designer (P-5)          | F-15.21.14 | R-15.21.14    |
| US-15.21.15.1 | designer (P-5)          | F-15.21.15 | R-15.21.15    |
| US-15.21.15.2 | designer (P-5)          | F-15.21.15 | R-15.21.15a   |
| US-15.21.15.3 | engine tester (P-27)    | F-15.21.15 | R-15.21.15    |
| US-15.21.15.4 | QA tester (P-19)        | F-15.21.15 | R-15.21.15a   |
| US-15.21.15.5 | player (P-23)           | F-15.21.15 | R-15.21.15    |

1. **US-15.21.13.1** — As a designer, I want to author haptic effects on a multi-track timeline so
   that I can control each haptic channel (motors, triggers, waveform) independently.
   - **Acceptance:** all five timeline tracks accept keyframes
2. **US-15.21.13.2** — As a designer, I want to configure per-platform fallback layers in the haptic
   editor so that I can ensure every controller class receives appropriate feedback.
   - **Acceptance:** fallback warnings appear for uncovered platforms
3. **US-15.21.13.3** — As a designer, I want haptic effect edits to integrate with undo/redo so that
   I can revert mistakes without losing work.
   - **Acceptance:** Ctrl+Z restores previous haptic state
4. **US-15.21.13.4** — As an engine developer, I want haptic effect assets serialized in the
   standard asset format so that they integrate with the existing asset pipeline.
   - **Acceptance:** haptic assets load via the standard asset loader
5. **US-15.21.13.5** — As an engine tester, I want to verify that the haptic editor creates valid
   assets for all five channels so that exported effects are complete.
   - **Acceptance:** export produces data for all configured tracks
6. **US-15.21.13.6** — As a QA tester, I want to verify that the editor warns when a platform has no
   haptic fallback so that designers cannot ship silent controllers.
   - **Acceptance:** removing all Xbox-compatible layers triggers a warning
7. **US-15.21.14.1** — As a designer, I want to draw haptic waveforms freehand on a canvas so that I
   can create unique tactile textures not possible with preset curves.
   - **Acceptance:** freehand strokes produce valid waveform data
8. **US-15.21.14.2** — As a designer, I want to layer multiple waveform segments with crossfades so
   that I can build complex haptic textures like engine rumble with gear shifts.
   - **Acceptance:** overlapping segments blend in the crossfade region
9. **US-15.21.14.3** — As an engine tester, I want to verify waveform canvas output matches drawn
   points within 5% tolerance so that authoring is accurate.
   - **Acceptance:** exported samples match canvas display
10. **US-15.21.14.4** — As a designer, I want to select from preset waveform curves (sine, sawtooth,
    noise, impulse) so that I can quickly start from a known shape.
    - **Acceptance:** preset selection populates the canvas
11. **US-15.21.15.1** — As a designer, I want to see simulated haptic output for DualSense, Xbox,
    and Switch side-by-side so that I can verify fallback behavior without switching hardware.
    - **Acceptance:** three preview panes render simultaneously
12. **US-15.21.15.2** — As a designer, I want to play haptic effects on a connected controller
    directly from the editor so that I can feel the effect while tuning it.
    - **Acceptance:** pressing play produces haptic output on the connected controller within 20 ms
13. **US-15.21.15.3** — As an engine tester, I want to verify that the preview panel shows correct
    active/degraded layers per controller class so that visual simulation is accurate.
    - **Acceptance:** DualSense shows five channels, Xbox shows two
14. **US-15.21.15.4** — As a QA tester, I want to verify that preview-to-hardware latency is under
    20 ms so that real-time iteration is responsive.
    - **Acceptance:** measured latency from play to first HID output is under 20 ms
15. **US-15.21.15.5** — As a player, I want haptic effects authored in the editor to feel consistent
    across DualSense, Xbox, and Switch controllers so that my controller choice does not degrade the
    experience.
    - **Acceptance:** all three controllers produce perceptible feedback for the same effect

## Haptic Effect Library

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-6.4.6.1  | designer (P-5)          | F-6.4.6  | R-6.4.6      |
| US-6.4.6.2  | designer (P-5)          | F-6.4.6  | R-6.4.6a     |
| US-6.4.6.3  | designer (P-5)          | F-6.4.6  | R-6.4.6a     |
| US-6.4.6.4  | engine tester (P-27)    | F-6.4.6  | R-6.4.6      |
| US-6.4.6.5  | player (P-23)           | F-6.4.6  | R-6.4.6      |
| US-6.4.6.6  | QA tester (P-19)        | F-6.4.6  | R-6.4.6      |
| US-6.4.7.1  | designer (P-5)          | F-6.4.7  | R-6.4.7      |
| US-6.4.7.2  | designer (P-5)          | F-6.4.7  | R-6.4.7a     |
| US-6.4.7.3  | engine developer (P-26) | F-6.4.7  | R-6.4.7      |
| US-6.4.7.4  | engine tester (P-27)    | F-6.4.7  | R-6.4.7      |
| US-6.4.7.5  | player (P-23)           | F-6.4.7  | R-6.4.7      |
| US-6.4.7.6  | QA tester (P-19)        | F-6.4.7  | R-6.4.7a     |
| US-6.4.8.1  | designer (P-5)          | F-6.4.8  | R-6.4.8      |
| US-6.4.8.2  | designer (P-5)          | F-6.4.8  | R-6.4.8a     |
| US-6.4.8.3  | engine developer (P-26) | F-6.4.8  | R-6.4.8      |
| US-6.4.8.4  | engine tester (P-27)    | F-6.4.8  | R-6.4.8      |
| US-6.4.8.5  | QA tester (P-19)        | F-6.4.8  | R-6.4.8a     |
| US-6.4.8.6  | player (P-23)           | F-6.4.8  | R-6.4.8      |

1. **US-6.4.6.1** — As a designer, I want a built-in library of categorized haptic effects so that I
   can add feedback to gameplay events without authoring from scratch.
   - **Acceptance:** library contains effects for combat, locomotion, environment, and UI categories
2. **US-6.4.6.2** — As a designer, I want to search the haptic library by name, category, and tag so
   that I can find the right effect quickly.
   - **Acceptance:** searching "explosion" returns environment category results
3. **US-6.4.6.3** — As a designer, I want to duplicate a library effect as a starting template so
   that I can customize it without modifying the original.
   - **Acceptance:** duplicate creates an editable copy with identical data
4. **US-6.4.6.4** — As an engine tester, I want to verify every library asset has valid fallback
   chains for DualSense, Xbox, and Switch Pro so that no library effect is platform-incomplete.
   - **Acceptance:** all library assets pass fallback validation
5. **US-6.4.6.5** — As a player, I want built-in haptic effects for common actions (hit, footstep,
   explosion) so that the game feels tactile out of the box.
   - **Acceptance:** default gameplay events produce haptic feedback
6. **US-6.4.6.6** — As a QA tester, I want to verify that every library asset produces perceptible
   output on all three controller classes so that no platform is neglected.
   - **Acceptance:** each asset plays non-zero output on DualSense, Xbox, and Switch Pro
7. **US-6.4.7.1** — As a designer, I want to bind haptic intensity to gameplay values (e.g., vehicle
   speed) using a visual node so that feedback scales dynamically without code.
   - **Acceptance:** wiring speed to intensity scales rumble with vehicle velocity
8. **US-6.4.7.2** — As a designer, I want to apply curve remapping (linear, exponential, step) to
   haptic parameter bindings so that I can shape the feel of dynamic feedback.
   - **Acceptance:** exponential remap with exponent 2.0 maps input 0.5 to output 0.25
9. **US-6.4.7.3** — As an engine developer, I want the parameter binding node to evaluate in under 1
   ms per frame so that haptic bindings do not impact frame budget.
   - **Acceptance:** measured evaluation time is under 1 ms
10. **US-6.4.7.4** — As an engine tester, I want to verify that bound parameters update haptic
    output correctly each frame so that dynamic feedback is responsive.
    - **Acceptance:** changing input from 0.5 to 1.0 updates motor intensity from 50% to 100% within
      one frame
11. **US-6.4.7.5** — As a player, I want haptic feedback to scale with in-game intensity (stronger
    hits, faster speeds) so that the controller reflects what is happening in the game.
    - **Acceptance:** harder hits produce stronger rumble
12. **US-6.4.7.6** — As a QA tester, I want to verify that curve remapping produces correct output
    values so that designer-tuned feel is mathematically accurate.
    - **Acceptance:** step function maps input below threshold to 0 and above to 1
13. **US-6.4.8.1** — As a designer, I want to adjust haptic parameters in real time during play mode
    so that I can feel changes immediately on my controller.
    - **Acceptance:** editing intensity during play updates the connected controller within 50 ms
14. **US-6.4.8.2** — As a designer, I want live tuning changes to be ephemeral until I explicitly
    apply them so that exploratory tweaks do not corrupt my saved assets.
    - **Acceptance:** stopping play mode without applying discards changes
15. **US-6.4.8.3** — As an engine developer, I want live tuning to propagate via the
    editor-to-runtime bridge so that it reuses the existing hot-reload infrastructure.
    - **Acceptance:** parameter updates use the standard property sync path
16. **US-6.4.8.4** — As an engine tester, I want to verify that live tuning propagation latency is
    under 50 ms so that designers get immediate feedback.
    - **Acceptance:** measured propagation time is under 50 ms
17. **US-6.4.8.5** — As a QA tester, I want to verify that discarding live tuning changes restores
    original asset values so that no accidental modifications persist.
    - **Acceptance:** asset file is byte-identical after discard
18. **US-6.4.8.6** — As a player, I want haptic effects tuned via live iteration to feel polished so
    that tactile feedback enhances rather than distracts from gameplay.
    - **Acceptance:** shipped haptic effects feel intentional and consistent across controller types
