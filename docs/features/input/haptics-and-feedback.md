# 6.4 — Haptics & Feedback

## Rumble

| ID      | Feature                                   | Requirements |
|---------|-------------------------------------------|--------------|
| F-6.4.1 | Dual-Motor Rumble with Pattern Sequencing | R-6.4.1      |

1. **F-6.4.1** — Drive low-frequency (heavy) and high-frequency (light) rumble motors independently
   with configurable intensity, duration, and envelope (attack, sustain, decay). Define reusable
   rumble patterns as keyframe sequences supporting looping, blending, and priority-based
   interruption. Rumble provides tactile feedback for combat hits, ability impacts, environmental
   effects, and mount locomotion throughout MMO gameplay. DualSense uses the same dual-motor model.
   Switch Pro provides a basic rumble compatibility layer over HD Rumble. Motor intensity is
   normalized to 0.0-1.0 across all backends. Patterns should be authored in a data-driven format
   for designer iteration.
   - **Deps:** F-6.1.3
   - **Platform:** XInput exposes left (low-freq) and right (high-freq) motors directly.

## Adaptive Triggers

| ID      | Feature                            | Requirements |
|---------|------------------------------------|--------------|
| F-6.4.2 | DualSense Adaptive Trigger Effects | R-6.4.2      |

1. **F-6.4.2** — Control DualSense adaptive trigger resistance and vibration profiles per-trigger
   (L2/R2). Support effect modes: resistance (bowstring draw tension), vibration (machine gun
   recoil), and sectioned resistance (gear notches for vehicle shifting). Adaptive triggers add
   physicality to MMO interactions like drawing a bow, reeling in a fishing line, or pulling a
   crafting lever. platform-specific USB/Bluetooth communication. This feature is only available on
   DualSense hardware. The abstraction layer must gracefully degrade to no-op on controllers that
   lack adaptive trigger support.
   - **Deps:** F-6.1.3
   - **Platform:** DualSense adaptive triggers are controlled via HID output reports and require

## HD Haptics

| ID      | Feature                         | Requirements |
|---------|---------------------------------|--------------|
| F-6.4.3 | High-Definition Haptic Playback | R-6.4.3      |
| F-6.4.4 | Audio-Driven Haptic Generation  | R-6.4.4      |

1. **F-6.4.3** — Play high-fidelity haptic waveforms on controllers with HD haptics (Switch Joy-Con
   HD Rumble, DualSense haptic feedback actuators). HD haptics reproduce textures, surfaces, and
   fine impacts — footsteps vary between stone, grass, and sand; rain taps on a shield; lockpicking
   provides tumbler feedback. pairs at 5ms granularity. DualSense uses voice-coil actuators
   supporting arbitrary waveform playback. A common waveform format must be defined with
   platform-specific conversion at the backend layer.
   - **Deps:** F-6.1.3
   - **Platform:** Switch HD Rumble uses linear resonant actuators driven by frequency/amplitude
2. **F-6.4.4** — Automatically generate haptic waveforms from audio signals by extracting frequency
   bands and amplitude envelopes, synchronizing tactile feedback with sound effects. Audio-driven
   haptics ensure every explosion, spell impact, or ambient rumble has matching tactile presence
   without requiring hand-authored haptic assets for every sound. Frequency band extraction should
   target the low-frequency range (20-250 Hz) most perceptible through haptic actuators. Latency
   between audio and haptic playback must stay under 10ms to avoid perceptual desynchronization.
   - **Deps:** F-6.4.3
   - **Platform:** Requires access to the audio system's final mix or per-channel output.

## Force Feedback Profiles

| ID      | Feature                        | Requirements |
|---------|--------------------------------|--------------|
| F-6.4.5 | Custom Force Feedback Profiles | R-6.4.5      |

1. **F-6.4.5** — Define named, data-driven haptic profiles that combine rumble patterns, adaptive
   trigger effects, and HD haptic waveforms into a single reusable asset. Profiles are triggered by
   gameplay events and support parameter binding (e.g., impact force scales rumble intensity).
   Profiles degrade gracefully per-platform: a profile with adaptive triggers and HD haptics falls
   back to dual-motor rumble on Xbox controllers. The fallback chain is configurable within the
   profile asset. every profile produces feedback on every supported controller class (Xbox,
   DualSense, Switch Pro, generic rumble-only).
   - **Deps:** F-6.4.1, F-6.4.2, F-6.4.3
   - **Platform:** Profile fallback ordering must be validated at asset build time to guarantee
