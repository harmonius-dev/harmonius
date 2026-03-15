# R-6.4 — Haptics & Feedback Requirements

## Rumble

### R-6.4.1 Dual-Motor Rumble with Pattern Sequencing

The engine **SHALL** drive low-frequency and high-frequency rumble motors independently with
intensity normalized to 0.0-1.0, configurable duration and envelope (attack, sustain, decay), and
support reusable rumble patterns defined as data-driven keyframe sequences with looping, blending,
and priority-based interruption.

- **Derived from:** [F-6.4.1](../../features/input/haptics-and-feedback.md)
- **Rationale:** Independent motor control enables distinct tactile textures (heavy impacts vs. fine
  vibrations), and data-driven patterns let designers iterate without code changes.
- **Verification:** Unit test: set low-frequency motor to 0.8 and high-frequency to 0.2; verify
  the output report contains the correct per-motor intensities. Define a keyframe pattern with
  attack 100ms, sustain 200ms, decay 100ms; verify the envelope shape matches within 5ms tolerance.
  Trigger a priority-3 pattern during a priority-5 pattern; assert the priority-5 pattern is not
  interrupted. Trigger a priority-7 pattern; assert it interrupts the priority-5 pattern.

## Adaptive Triggers

### R-6.4.2 DualSense Adaptive Trigger Effects

The engine **SHALL** control DualSense adaptive trigger resistance and vibration per trigger
(L2/R2), supporting resistance, vibration, and sectioned resistance effect modes, and **SHALL**
degrade gracefully to a no-op on controllers lacking adaptive trigger hardware.

- **Derived from:** [F-6.4.2](../../features/input/haptics-and-feedback.md)
- **Rationale:** Adaptive triggers add physicality to interactions like bow drawing and crafting
  levers; graceful degradation ensures no errors on unsupported controllers.
- **Verification:** Integration test on DualSense: apply a resistance effect at 0.5 strength;
  verify the HID output report contains the correct resistance parameters. Apply vibration and
  sectioned resistance modes; verify correct output reports. Integration test on Xbox controller:
  apply an adaptive trigger effect; assert no error is raised and no output report is sent.

## HD Haptics

### R-6.4.3 High-Definition Haptic Playback

The engine **SHALL** play high-fidelity haptic waveforms on controllers with HD haptic actuators
(Switch HD Rumble, DualSense voice-coil) using a common waveform format that is converted to
platform-specific representations at the backend layer.

- **Derived from:** [F-6.4.3](../../features/input/haptics-and-feedback.md)
- **Rationale:** HD haptics reproduce surface textures and fine impacts (footsteps on stone vs.
  grass) that dual-motor rumble cannot approximate.
- **Verification:** Unit test: load a waveform asset containing a 100Hz sine at 0.5 amplitude;
  verify the backend conversion produces valid platform-specific output (frequency/amplitude
  pairs for Switch, raw waveform samples for DualSense). Integration test: play the waveform
  on a DualSense and verify actuator output matches the expected waveform shape within 10%
  amplitude tolerance.

### R-6.4.4 Audio-Driven Haptic Generation

The engine **SHALL** generate haptic waveforms from audio signals by extracting frequency bands
(20-250 Hz) and amplitude envelopes, maintaining latency between audio and haptic playback under
10ms.

- **Derived from:** [F-6.4.4](../../features/input/haptics-and-feedback.md)
- **Rationale:** Automatic generation from audio ensures every sound effect has matching tactile
  presence without requiring hand-authored haptic assets for each sound.
- **Verification:** Unit test: feed a known audio signal (100Hz sine wave at 0.8 amplitude) into
  the haptic generator; assert the output waveform has dominant frequency in the 80-120Hz range
  and amplitude within 20% of the input. Measure the timestamp delta between audio output and
  haptic output; assert it is under 10ms. Feed a 5kHz signal (above haptic range); assert the
  output amplitude is near zero.

## Force Feedback Profiles

### R-6.4.5 Custom Force Feedback Profiles

The engine **SHALL** support named, data-driven haptic profiles that combine rumble patterns,
adaptive trigger effects, and HD haptic waveforms into a single reusable asset with parameter
binding (e.g., impact force scales intensity), and **SHALL** degrade each profile through a
configurable fallback chain so that every profile produces feedback on every supported controller
class.

- **Derived from:** [F-6.4.5](../../features/input/haptics-and-feedback.md)
- **Rationale:** Profiles let designers author rich multi-layer haptic experiences as single assets
  with guaranteed cross-controller compatibility through validated fallback chains.
- **Verification:** Integration test: create a profile containing adaptive triggers, HD haptics,
  and rumble. Play on DualSense; assert all three layers activate. Play on Xbox controller; assert
  only the rumble layer activates (fallback). Play on a generic controller with basic rumble; assert
  the rumble fallback activates. Bind impact force parameter to 0.5; assert rumble intensity equals
  50% of the profile's base intensity. Validate at asset build time that every profile has a valid
  fallback for each controller class.

---

## Non-Functional Requirements

### R-6.4.NF1 Haptic Output Latency

The engine **SHALL** deliver haptic output to the controller within 5 ms of the triggering
gameplay event, measured from event timestamp to HID output report submission.

- **Derived from:** [F-6.4.1](../../features/input/haptics-and-feedback.md),
  [F-6.4.4](../../features/input/haptics-and-feedback.md) (10 ms audio-haptic sync budget)
- **Rationale:** Haptic feedback must be perceived as synchronous with the visual and audio
  event. Latency above 5 ms on the engine side, combined with controller-side processing,
  risks exceeding the 10 ms perceptual threshold.
- **Verification:** Integration test: trigger a rumble event and measure the time from event
  dispatch to HID output report submission. Assert p99 latency is below 5 ms across all
  supported controller types.
