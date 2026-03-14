# 6.4 — Haptics & Feedback

## Rumble

### F-6.4.1 Dual-Motor Rumble with Pattern Sequencing

Drive low-frequency (heavy) and high-frequency (light) rumble motors independently with
configurable intensity, duration, and envelope (attack, sustain, decay). Define reusable rumble
patterns as keyframe sequences supporting looping, blending, and priority-based interruption.
Rumble provides tactile feedback for combat hits, ability impacts, environmental effects, and
mount locomotion throughout MMO gameplay.

- **Requirements:** R-6.4.1
- **Dependencies:** F-6.1.3
- **Platform notes:** XInput exposes left (low-freq) and right (high-freq) motors directly.
  DualSense uses the same dual-motor model. Switch Pro provides a basic rumble compatibility
  layer over HD Rumble. Motor intensity is normalized to 0.0-1.0 across all backends. Patterns
  should be authored in a data-driven format for designer iteration.

## Adaptive Triggers

### F-6.4.2 DualSense Adaptive Trigger Effects

Control DualSense adaptive trigger resistance and vibration profiles per-trigger (L2/R2). Support
effect modes: resistance (bowstring draw tension), vibration (machine gun recoil), and sectioned
resistance (gear notches for vehicle shifting). Adaptive triggers add physicality to MMO
interactions like drawing a bow, reeling in a fishing line, or pulling a crafting lever.

- **Requirements:** R-6.4.2
- **Dependencies:** F-6.1.3
- **Platform notes:** DualSense adaptive triggers are controlled via HID output reports and
  require platform-specific USB/Bluetooth communication. This feature is only available on
  DualSense hardware. The abstraction layer must gracefully degrade to no-op on controllers
  that lack adaptive trigger support.

## HD Haptics

### F-6.4.3 High-Definition Haptic Playback

Play high-fidelity haptic waveforms on controllers with HD haptics (Switch Joy-Con HD Rumble,
DualSense haptic feedback actuators). HD haptics reproduce textures, surfaces, and fine impacts —
footsteps vary between stone, grass, and sand; rain taps on a shield; lockpicking provides
tumbler feedback.

- **Requirements:** R-6.4.3
- **Dependencies:** F-6.1.3
- **Platform notes:** Switch HD Rumble uses linear resonant actuators driven by
  frequency/amplitude pairs at 5ms granularity. DualSense uses voice-coil actuators supporting
  arbitrary waveform playback. A common waveform format must be defined with platform-specific
  conversion at the backend layer.

### F-6.4.4 Audio-Driven Haptic Generation

Automatically generate haptic waveforms from audio signals by extracting frequency bands and
amplitude envelopes, synchronizing tactile feedback with sound effects. Audio-driven haptics
ensure every explosion, spell impact, or ambient rumble has matching tactile presence without
requiring hand-authored haptic assets for every sound.

- **Requirements:** R-6.4.4
- **Dependencies:** F-6.4.3
- **Platform notes:** Requires access to the audio system's final mix or per-channel output.
  Frequency band extraction should target the low-frequency range (20-250 Hz) most perceptible
  through haptic actuators. Latency between audio and haptic playback must stay under 10ms to
  avoid perceptual desynchronization.

## Force Feedback Profiles

### F-6.4.5 Custom Force Feedback Profiles

Define named, data-driven haptic profiles that combine rumble patterns, adaptive trigger effects,
and HD haptic waveforms into a single reusable asset. Profiles are triggered by gameplay events
and support parameter binding (e.g., impact force scales rumble intensity). Profiles degrade
gracefully per-platform: a profile with adaptive triggers and HD haptics falls back to dual-motor
rumble on Xbox controllers. The fallback chain is configurable within the profile asset.

- **Requirements:** R-6.4.5
- **Dependencies:** F-6.4.1, F-6.4.2, F-6.4.3
- **Platform notes:** Profile fallback ordering must be validated at asset build time to
  guarantee every profile produces feedback on every supported controller class (Xbox, DualSense,
  Switch Pro, generic rumble-only).
