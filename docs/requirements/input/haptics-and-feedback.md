# R-6.4 — Haptics & Feedback Requirements

## Rumble

1. **R-6.4.1** — The engine **SHALL** drive low-frequency and high-frequency rumble motors
   independently with intensity values in [0.0, 1.0], supporting all controller types via
   platform-native output APIs.
   - **Rationale:** Independent motor control enables nuanced feedback (heavy hits use low-freq,
     light impacts use high-freq).
   - **Verification:** Set low-frequency to 0.8 and high-frequency to 0.2. Assert output report
     contains correct per-motor intensity values.

2. **R-6.4.2** — The engine **SHALL** support reusable rumble patterns as keyframe sequences with
   attack, sustain, and decay envelopes, looping, blending, and priority-based interruption.
   Patterns **SHALL** be authored in the visual editor as data assets.
   - **Rationale:** Data-driven patterns enable designer iteration without code; priority ordering
     prevents important effects from being masked.
   - **Verification:** Define a pattern with 100 ms attack, 200 ms sustain, 100 ms decay. Assert
     output matches within 5 ms. Trigger priority-3 during priority-5. Assert no interruption.
     Trigger priority-7. Assert it interrupts.

3. **R-6.4.3** — The engine **SHALL** normalize motor intensity to [0.0, 1.0] across all controller
   backends.
   - **Rationale:** Without normalization, the same pattern feels different across controllers.
   - **Verification:** Set intensity 0.5 on Xbox, DualSense, and Switch Pro. Assert output maps to
     equivalent hardware range for each.

## Adaptive Triggers

4. **R-6.4.4** — The engine **SHALL** control DualSense L2/R2 adaptive trigger resistance and
   vibration via HID output reports, supporting resistance, vibration, and sectioned resistance
   modes. On unsupported controllers, **SHALL** degrade to no-op without errors.
   - **Rationale:** Adaptive triggers add physicality; graceful degradation ensures portability.
   - **Verification:** Apply resistance at 0.5 on DualSense. Assert correct HID report. Apply on
     Xbox. Assert no error and no output.

5. **R-6.4.5** — The engine **SHALL** allow designers to configure adaptive trigger effects per
   trigger in the visual editor.
   - **Rationale:** No-code constraint; adaptive trigger tuning must be visual.
   - **Verification:** Configure resistance at 0.7 on L2 in the editor. Play on DualSense. Assert
     matches configuration.

## HD Haptics

6. **R-6.4.6** — The engine **SHALL** play HD haptic waveforms on supported controllers using a
   common waveform format with platform-specific backend conversion. Waveform assets **SHALL** be
   authored in the visual editor.
   - **Rationale:** HD haptics reproduce textures and fine impacts; a common format avoids
     per-platform authoring.
   - **Verification:** Load a 100 Hz sine waveform. Assert backend conversion produces valid output
     for Switch and DualSense. Verify amplitude within 10%.

7. **R-6.4.7** — The engine **SHALL** generate haptic waveforms from audio signals by extracting
   20-250 Hz bands and amplitude envelopes, synchronizing haptic output with audio to within 10 ms.
   - **Rationale:** Manual authoring for every sound is impractical; automatic generation ensures
     coverage.
   - **Verification:** Feed 100 Hz sine at 0.8. Assert output frequency in 80-120 Hz and amplitude
     within 20%. Feed 5 kHz. Assert near-zero output. Assert audio-to-haptic latency under 10 ms.

## Force Feedback Profiles

8. **R-6.4.8** — The engine **SHALL** support named profiles combining rumble, adaptive triggers,
   and HD haptics into a single data asset with parameter binding. Profiles **SHALL** be authored in
   the visual editor.
   - **Rationale:** Multi-layer haptic experiences must be reusable assets; parameter binding
     enables dynamic scaling.
   - **Verification:** Play a profile with all layers on DualSense. Assert all activate. Bind impact
     force to 0.5. Assert rumble intensity equals 50% of base.

9. **R-6.4.9** — The engine **SHALL** degrade profiles per controller using configurable fallback
   chains. Build-time validation **SHALL** reject profiles missing a fallback for any controller
   class.
   - **Rationale:** Every controller must produce some feedback; build-time validation prevents
     zero-output profiles.
   - **Verification:** Play a profile on Xbox. Assert only rumble activates. Verify build rejects a
     profile with no Xbox fallback.

10. **R-6.4.10** — The engine **SHALL** dispatch haptic effects through the ECS event system,
    triggering profiles from gameplay events authored as ECS observers.
    - **Rationale:** 100% ECS-based constraint; haptic dispatch must integrate with the ECS event
      and observer system.
    - **Verification:** Register observer on "HitEvent" that triggers a rumble profile. Emit
      HitEvent. Assert profile plays on the active controller.
