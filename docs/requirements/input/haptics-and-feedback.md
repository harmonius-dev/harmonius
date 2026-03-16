# R-6.4 — Haptics & Feedback Requirements

## Rumble

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-6.4.1 | The engine **SHALL** drive low-frequency and high-frequency rumble motors independently with intensity values in [0.0, 1.0], supporting all controller types (Xbox, DualSense, Switch Pro) via platform-native output APIs. [F-6.4.1](../../features/input/haptics-and-feedback.md) low-freq, light impacts use high-freq). report contains correct per-motor intensity values. |  | Independent motor control enables nuanced tactile feedback (heavy hits use | Unit test: set low-frequency to 0.8 and high-frequency to 0.2. Assert the output |
| R-6.4.1a | The engine **SHALL** support reusable rumble patterns as keyframe sequences with attack, sustain, and decay envelopes, looping, blending, and priority-based interruption. Patterns **SHALL** be authored in the visual editor as data assets. [F-6.4.1](../../features/input/haptics-and-feedback.md) prevents important effects from being masked. Assert output shape matches within 5 ms tolerance. Trigger a priority-3 pattern during a priority-5 pattern. Assert no interruption. Trigger priority-7. Assert it interrupts. |  | Data-driven patterns enable designer iteration without code; priority ordering | Unit test: define a pattern with 100 ms attack, 200 ms sustain, 100 ms decay. |
| R-6.4.1b | The engine **SHALL** normalize motor intensity to [0.0, 1.0] across all controller backends, so pattern assets produce equivalent haptic output on Xbox, DualSense, and Switch Pro. [F-6.4.1](../../features/input/haptics-and-feedback.md) breaking authored intent. values map to the equivalent hardware range for each controller. |  | Without normalization, the same pattern feels different across controllers, | Unit test: set intensity 0.5 on all three controller types. Assert output report |

## Adaptive Triggers

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-6.4.2 | The engine **SHALL** control DualSense L2/R2 adaptive trigger resistance and vibration via HID output reports, supporting resistance, vibration, and sectioned resistance effect modes. On controllers without adaptive trigger support, the engine **SHALL** gracefully degrade to a no-op without errors. [F-6.4.2](../../features/input/haptics-and-feedback.md) graceful degradation ensures portability. report contains correct parameters. Apply the same effect on Xbox. Assert no error and no output report. |  | Adaptive triggers add physicality (bowstring draw, fishing reel, gear notches); | Unit test: apply resistance effect at 0.5 strength on DualSense. Assert HID |
| R-6.4.2a | The engine **SHALL** allow designers to configure adaptive trigger effect modes and parameters per trigger in the visual editor. [F-6.4.2](../../features/input/haptics-and-feedback.md) Play on DualSense. Assert resistance matches configuration. |  | No-code engine constraint; adaptive trigger tuning must be visual. | Integration test: configure resistance mode at 0.7 strength on L2 in the editor. |

## HD Haptics

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-6.4.3 | The engine **SHALL** play high-fidelity haptic waveforms on controllers with HD haptics (Switch HD Rumble, DualSense) using a common waveform format with platform-specific conversion at the backend. Waveform assets **SHALL** be authored in the visual editor. [F-6.4.3](../../features/input/haptics-and-feedback.md) common format avoids per-platform authoring. output for Switch (frequency/amplitude pairs) and DualSense (raw waveform). Verify actuator output matches expected amplitude within 10% tolerance. |  | HD haptics reproduce textures and fine impacts that dual-motor rumble cannot; a | Unit test: load a 100 Hz sine waveform. Assert backend conversion produces valid |
| R-6.4.4 | The engine **SHALL** automatically generate haptic waveforms from audio signals by extracting 20-250 Hz frequency bands and amplitude envelopes, synchronizing haptic output with audio output to within 10 ms. [F-6.4.4](../../features/input/haptics-and-feedback.md) generation ensures every explosion and impact has matching haptics. frequency in 80-120 Hz and amplitude within 20%. Feed a 5 kHz signal. Assert output amplitude is near zero. Measure audio-to-haptic timestamp delta. Assert under 10 ms. |  | Manual authoring of haptic assets for every sound is impractical; automatic | Unit test: feed a 100 Hz sine wave at 0.8 amplitude. Assert output has dominant |

## Force Feedback Profiles

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-6.4.5 | The engine **SHALL** support named profiles combining rumble patterns, adaptive trigger effects, and HD haptic waveforms into a single data asset with parameter binding (e.g., impact force scales intensity). Profiles **SHALL** be authored in the visual editor. [F-6.4.5](../../features/input/haptics-and-feedback.md) enables dynamic scaling. activate. Bind impact force to 0.5. Assert rumble intensity equals 50% of base. |  | Multi-layer haptic experiences must be single reusable assets; parameter binding | Unit test: play a profile with all three layers on DualSense. Assert all layers |
| R-6.4.5a | The engine **SHALL** degrade profiles per controller using configurable fallback chains (HD haptics profile has a valid fallback for each supported controller class. [F-6.4.5](../../features/input/haptics-and-feedback.md) shipping profiles that produce zero output on a controller class. haptics and adaptive triggers fall back). Verify build-time validation rejects a profile that has no fallback for Xbox. |  | Every controller must produce some feedback; build-time validation prevents | Unit test: play a profile on Xbox. Assert only the rumble layer activates (HD |
| R-6.4.5b | The engine **SHALL** dispatch haptic effects through the ECS event system, triggering profiles from gameplay events (hit, explosion, locomotion) authored as ECS observers. [F-6.4.1](../../features/input/haptics-and-feedback.md) observer system. Emit a HitEvent. Assert the rumble profile plays on the active controller. | [F-6.4.5](../../features/input/haptics-and-feedback.md), | 100% ECS-based constraint; haptic dispatch must integrate with the ECS event and | Unit test: register an observer on a "HitEvent" that triggers a rumble profile. |

---
