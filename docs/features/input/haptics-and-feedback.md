# 6.4 — Haptics & Feedback

## Rumble

| ID      | Feature                                   |
|---------|-------------------------------------------|
| F-6.4.1 | Dual-Motor Rumble with Pattern Sequencing |

1. **F-6.4.1** — Drive low-frequency (heavy) and high-frequency (light) rumble motors independently
   with configurable intensity, duration, and envelope (attack, sustain, decay). Define reusable
   rumble patterns as keyframe sequences supporting looping, blending, and priority-based
   interruption. Motor intensity is normalized to 0.0-1.0 across all backends. Patterns should be
   authored in a data-driven format for designer iteration.
   - **Deps:** F-6.1.3
   - **Platform:** XInput exposes left (low-freq) and right (high-freq) motors directly. DualSense
     uses the same dual-motor model. Switch Pro provides a basic rumble compatibility layer over HD
     Rumble.

## Adaptive Triggers

| ID      | Feature                            |
|---------|------------------------------------|
| F-6.4.2 | DualSense Adaptive Trigger Effects |

1. **F-6.4.2** — Control DualSense adaptive trigger resistance and vibration profiles per-trigger
   (L2/R2). Support effect modes: resistance (bowstring draw tension), vibration (machine gun
   recoil), and sectioned resistance (gear notches for vehicle shifting). The abstraction layer must
   gracefully degrade to no-op on controllers that lack adaptive trigger support.
   - **Deps:** F-6.1.3
   - **Platform:** DualSense adaptive triggers are controlled via HID output reports and require
     platform-specific USB/Bluetooth communication. This feature is only available on DualSense
     hardware.

## HD Haptics

| ID      | Feature                         |
|---------|---------------------------------|
| F-6.4.3 | High-Definition Haptic Playback |
| F-6.4.4 | Audio-Driven Haptic Generation  |

1. **F-6.4.3** — Play high-fidelity haptic waveforms on controllers with HD haptics (Switch Joy-Con
   HD Rumble, DualSense haptic feedback actuators). HD haptics reproduce textures, surfaces, and
   fine impacts. A common waveform format must be defined with platform-specific conversion at the
   backend layer.
   - **Deps:** F-6.1.3
   - **Platform:** Switch HD Rumble uses linear resonant actuators driven by frequency/amplitude
     pairs at 5ms granularity. DualSense uses voice-coil actuators supporting arbitrary waveform
     playback.
2. **F-6.4.4** — Automatically generate haptic waveforms from audio signals by extracting frequency
   bands and amplitude envelopes, synchronizing tactile feedback with sound effects. Frequency band
   extraction should target the low-frequency range (20-250 Hz) most perceptible through haptic
   actuators. Latency between audio and haptic playback must stay under 10ms.
   - **Deps:** F-6.4.3
   - **Platform:** Requires access to the audio system's final mix or per-channel output.

## Force Feedback Profiles

| ID      | Feature                        |
|---------|--------------------------------|
| F-6.4.5 | Custom Force Feedback Profiles |

1. **F-6.4.5** — Define named, data-driven haptic profiles that combine rumble patterns, adaptive
   trigger effects, and HD haptic waveforms into a single reusable asset. Profiles are triggered by
   gameplay events and support parameter binding (e.g., impact force scales rumble intensity).
   Profiles degrade gracefully per-platform: a profile with adaptive triggers and HD haptics falls
   back to dual-motor rumble on Xbox controllers. The fallback chain is configurable within the
   profile asset.
   - **Deps:** F-6.4.1, F-6.4.2, F-6.4.3
   - **Platform:** Profile fallback ordering must be validated at asset build time to guarantee
     every profile produces feedback on every supported controller class (Xbox, DualSense, Switch
     Pro, generic).
