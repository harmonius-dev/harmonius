# R-6.4 — Haptics & Feedback User Stories

## F-6.4.1 Dual-Motor Rumble with Pattern Sequencing

## US-6.4.1.1 Configure Rumble Patterns in Editor

**As a** designer (P-5), **I want to** create reusable rumble patterns as keyframe sequences
with attack, sustain, and decay envelopes in the editor, **so that** rumble feedback is
data-driven.

## US-6.4.1.2 Set Up Rumble for Gameplay Events

**As a** designer (P-5), **I want to** bind rumble patterns to gameplay events (hits, impacts,
locomotion) in the editor, **so that** tactile feedback is authored without code.

## US-6.4.1.3 Verify Independent Motor Control

**As an** engine tester (P-27), **I want to** set low-frequency to 0.8 and high-frequency to
0.2 and verify the output report contains correct per-motor intensities, **so that**
independent control works.

## US-6.4.1.4 Verify Envelope Timing

**As an** engine tester (P-27), **I want to** define a pattern with 100ms attack, 200ms
sustain, 100ms decay and verify the shape matches within 5ms tolerance, **so that** envelope
timing is accurate.

## US-6.4.1.5 Verify Priority-Based Interruption

**As an** engine tester (P-27), **I want to** trigger a priority-3 pattern during a priority-5
pattern and assert it is not interrupted, then trigger priority-7 and assert it does interrupt,
**so that** priority ordering works.

## US-6.4.1.6 Implement Dual-Motor Rumble System

**As an** engine developer (P-26), **I want to** implement independent motor control with
keyframe pattern sequencing, looping, blending, and priority interruption across all controller
types, **so that** rumble feedback is available.

## US-6.4.1.7 Test Rumble on All Controller Types

**As a** QA tester (P-19), **I want to** test rumble patterns on Xbox, DualSense, and Switch
Pro controllers, **so that** haptic compatibility is verified.

## US-6.4.1.8 Feel Impact When Hitting Enemies

**As a** player (P-23), **I want to** feel rumble feedback on hits, impacts, and environmental
effects, **so that** gameplay has tactile presence.

---

## F-6.4.2 DualSense Adaptive Trigger Effects

## US-6.4.2.1 Configure Adaptive Trigger Effects in Editor

**As a** designer (P-5), **I want to** set resistance, vibration, and sectioned resistance
modes per trigger in the editor, **so that** adaptive triggers are authored without code.

## US-6.4.2.2 Verify Resistance Effect Output

**As an** engine tester (P-27), **I want to** apply a resistance effect at 0.5 strength on
DualSense and verify the HID output report contains correct resistance parameters, **so that**
adaptive triggers produce correct output.

## US-6.4.2.3 Verify Graceful Degradation on Xbox

**As an** engine tester (P-27), **I want to** apply an adaptive trigger effect on an Xbox
controller and assert no error and no output report, **so that** degradation is silent.

## US-6.4.2.4 Implement Adaptive Trigger System

**As an** engine developer (P-26), **I want to** implement DualSense adaptive trigger control
via HID output reports with graceful no-op degradation on unsupported controllers, **so that**
adaptive triggers are available where hardware supports them.

## US-6.4.2.5 Test Adaptive Triggers for All Effect Modes

**As a** QA tester (P-19), **I want to** test all three adaptive trigger modes (resistance,
vibration, sectioned) on DualSense, **so that** all modes function correctly.

## US-6.4.2.6 Feel Bow Draw Tension on Triggers

**As a** player (P-23), **I want to** feel increasing resistance when drawing a bowstring with
L2/R2, **so that** the controller enhances immersion.

---

## F-6.4.3 High-Definition Haptic Playback

## US-6.4.3.1 Author HD Haptic Waveforms in Editor

**As a** designer (P-5), **I want to** create and preview HD haptic waveform assets in the
editor, **so that** surface textures and fine impacts are authored visually.

## US-6.4.3.2 Verify Common Waveform Format Conversion

**As an** engine tester (P-27), **I want to** load a 100Hz sine waveform and verify backend
conversion produces valid platform-specific output (frequency/amplitude pairs for Switch, raw
waveform for DualSense), **so that** cross-platform conversion works.

## US-6.4.3.3 Verify HD Haptic Amplitude Accuracy

**As an** engine tester (P-27), **I want to** play a waveform on DualSense and verify actuator
output matches expected shape within 10% amplitude tolerance, **so that** haptic fidelity is
confirmed.

## US-6.4.3.4 Implement HD Haptic Playback

**As an** engine developer (P-26), **I want to** implement HD haptic waveform playback using a
common format with platform-specific conversion at the backend, **so that** high-fidelity
haptics are cross-platform.

## US-6.4.3.5 Test HD Haptics on Switch and DualSense

**As a** QA tester (P-19), **I want to** test HD haptic waveforms on both Switch HD Rumble and
DualSense, **so that** cross-platform haptic quality is verified.

## US-6.4.3.6 Feel Different Surface Textures

**As a** player (P-23), **I want** footsteps on stone, grass, and sand to feel different
through the controller, **so that** surfaces are tactile.

---

## F-6.4.4 Audio-Driven Haptic Generation

## US-6.4.4.1 Configure Audio-Haptic Generation

**As a** designer (P-5), **I want to** enable automatic haptic generation from audio signals
for specific events, **so that** every explosion and impact has matching haptics without
manual authoring.

## US-6.4.4.2 Verify Frequency Band Extraction

**As an** engine tester (P-27), **I want to** feed a 100Hz sine wave at 0.8 amplitude and
assert output waveform has dominant frequency in 80-120Hz and amplitude within 20%, **so that**
band extraction is correct.

## US-6.4.4.3 Verify Audio-Haptic Synchronization

**As an** engine tester (P-27), **I want to** measure timestamp delta between audio output and
haptic output and assert it is under 10ms, **so that** synchronization is perceptible.

## US-6.4.4.4 Verify High-Frequency Signal Rejection

**As an** engine tester (P-27), **I want to** feed a 5kHz signal and assert haptic output
amplitude is near zero, **so that** out-of-range frequencies are filtered.

## US-6.4.4.5 Implement Audio-Driven Haptic Generator

**As an** engine developer (P-26), **I want to** implement automatic haptic waveform generation
from audio signals by extracting 20-250Hz frequency bands and amplitude envelopes, **so that**
every sound can generate haptic feedback.

## US-6.4.4.6 Feel Synchronized Haptics with Sound Effects

**As a** player (P-23), **I want** explosions and impacts to feel in sync with their sounds,
**so that** audio and haptics reinforce each other.

---

## F-6.4.5 Custom Force Feedback Profiles

## US-6.4.5.1 Author Force Feedback Profiles in Editor

**As a** designer (P-5), **I want to** create named profiles combining rumble, adaptive
triggers, and HD haptics with parameter binding and fallback chains in the editor, **so that**
multi-layer haptic experiences are single assets.

## US-6.4.5.2 Configure Fallback Chains Per Controller

**As a** designer (P-5), **I want to** set fallback ordering (HD haptics to rumble, adaptive
triggers to no-op) per profile, **so that** every controller gets some feedback.

## US-6.4.5.3 Verify Full Profile on DualSense

**As an** engine tester (P-27), **I want to** play a profile with all three layers on
DualSense and assert all activate, **so that** full-featured playback works.

## US-6.4.5.4 Verify Fallback on Xbox

**As an** engine tester (P-27), **I want to** play the same profile on Xbox and verify only
the rumble layer activates, **so that** fallback works correctly.

## US-6.4.5.5 Verify Parameter Binding

**As an** engine tester (P-27), **I want to** bind impact force to 0.5 and verify rumble
intensity equals 50% of base, **so that** parameter binding scales correctly.

## US-6.4.5.6 Verify Asset-Time Fallback Validation

**As an** engine tester (P-27), **I want to** verify at build time that every profile has a
valid fallback for each controller class, **so that** no controller gets zero feedback.

## US-6.4.5.7 Implement Force Feedback Profile System

**As an** engine developer (P-26), **I want to** implement the profile system with multi-layer
haptics, parameter binding, and validated fallback chains, **so that** cross-controller haptic
experiences are unified.

## US-6.4.5.8 Test Profiles on All Controller Types

**As a** QA tester (P-19), **I want to** test every force feedback profile on every supported
controller class, **so that** fallback coverage is complete.

## US-6.4.5.9 Feel Haptic Feedback on Any Controller

**As a** player (P-23), **I want** every haptic effect to produce some feedback on my
controller regardless of its capabilities, **so that** I always feel the game.

---

## Non-Functional Requirements

### R-6.4.NF1 Haptic Output Latency

The engine **SHALL** deliver haptic output to the controller within 5 ms of the triggering
gameplay event, measured from event timestamp to HID output report submission.

- **Derived from:** F-6.4.1, F-6.4.4
- **Rationale:** Haptic feedback must be perceived as synchronous with visual and audio events.
  Latency above 5 ms risks exceeding the 10 ms perceptual threshold.
- **Verification:** Integration test: trigger a rumble event and measure time from dispatch to
  HID report submission. Assert p99 latency is below 5 ms.
