# User Stories — 6.4 Haptics & Feedback

## US-6.4.1.1 Drive Dual-Motor Rumble Independently

**As an** engine developer (P-26), **I want** low-frequency and high-frequency rumble motors driven
independently with intensity and duration, **so that** tactile feedback is detailed.

## US-6.4.1.2 Define Reusable Rumble Patterns

**As a** designer (P-5), **I want** reusable rumble patterns as keyframe sequences with attack,
sustain, and decay, **so that** haptic assets are authored visually.

## US-6.4.1.3 Feel Combat Hits Through Controller

**As a** player (P-23), **I want** distinct rumble for combat hits, ability impacts, and mount
locomotion, **so that** actions have tactile presence.

## US-6.4.1.4 Support Pattern Looping and Blending

**As an** engine developer (P-26), **I want** rumble patterns supporting looping, blending, and
priority-based interruption, **so that** multiple haptic events compose correctly.

## US-6.4.1.5 Normalize Motor Intensity to 0-1

**As an** engine developer (P-26), **I want** motor intensity normalized to 0.0-1.0 across all
backends, **so that** rumble is consistent across controllers.

## US-6.4.1.6 Author Rumble Patterns in Visual Editor

**As a** designer (P-5), **I want to** author rumble patterns in a data-driven format in the editor,
**so that** haptic design is visual.

## US-6.4.1.7 Verify Rumble on All Controller Types

**As an** engine tester (P-27), **I want to** verify rumble on Xbox, DualSense, and Switch Pro,
**so that** all controllers produce haptic feedback.

## US-6.4.1.8 Feel Horse Gallop Rhythm

**As a** player (P-23), **I want** rhythmic rumble when riding a horse at gallop, **so that** mount
locomotion is felt.

## US-6.4.1.9 Trigger Rumble from Gameplay Events

**As a** designer (P-5), **I want** rumble patterns triggered by gameplay events, **so that** haptic
feedback is data-driven.

## US-6.4.1.10 Test Pattern Priority Interruption

**As an** engine tester (P-27), **I want to** test rumble pattern priority when multiple events fire
simultaneously, **so that** the highest priority wins.

## US-6.4.1.11 Support XInput Dual-Motor Model

**As an** engine developer (P-26), **I want** XInput left (low-freq) and right (high-freq) motors
exposed, **so that** Xbox rumble works natively.

## US-6.4.1.12 Test Rumble Envelope Accuracy

**As a** QA tester (P-19), **I want to** verify rumble envelope (attack, sustain, decay) plays
accurately, **so that** pattern timing is correct.

## US-6.4.2.1 Control DualSense Adaptive Trigger Resistance

**As an** engine developer (P-26), **I want** DualSense L2/R2 adaptive trigger resistance controlled
per-trigger, **so that** bowstring tension and brake feel work.

## US-6.4.2.2 Support Resistance Effect Mode

**As an** engine developer (P-26), **I want** resistance mode for adaptive triggers, **so that**
bowstring draw and brake pedal have progressive resistance.

## US-6.4.2.3 Support Vibration Effect Mode

**As an** engine developer (P-26), **I want** vibration mode on adaptive triggers, **so that**
machine gun recoil is felt through L2/R2.

## US-6.4.2.4 Feel Bowstring Tension When Drawing

**As a** player (P-23), **I want** trigger resistance when drawing a bow, **so that** archery feels
physical.

## US-6.4.2.5 Support Sectioned Resistance for Gears

**As an** engine developer (P-26), **I want** sectioned resistance for gear shifting notches,
**so that** vehicle gear changes are tactile.

## US-6.4.2.6 Degrade Gracefully on Non-DualSense

**As an** engine developer (P-26), **I want** adaptive trigger effects to no-op on controllers
without support, **so that** non-DualSense users are unaffected.

## US-6.4.2.7 Verify DualSense HID Communication

**As an** engine tester (P-27), **I want to** verify trigger control via HID output reports on USB
and Bluetooth, **so that** both connection modes work.

## US-6.4.2.8 Author Trigger Effects in Visual Editor

**As a** designer (P-5), **I want to** author adaptive trigger effects in the visual editor,
**so that** trigger feel is tunable.

## US-6.4.2.9 Feel Fishing Line Tension

**As a** player (P-23), **I want** trigger resistance when reeling in a fish, **so that** fishing
feels physical.

## US-6.4.2.10 Test Graceful Degradation on Xbox

**As an** engine tester (P-27), **I want to** verify effects degrade to no-op on Xbox, **so that**
fallback is clean.

## US-6.4.2.11 Configure Effects Per Gameplay Context

**As a** designer (P-5), **I want** trigger effects configurable per context (combat, driving,
fishing), **so that** each activity has unique feel.

## US-6.4.2.12 Test All Effect Modes

**As a** QA tester (P-19), **I want to** test resistance, vibration, and sectioned modes on
DualSense, **so that** all effect types work.

## US-6.4.3.1 Play HD Haptic Waveforms

**As an** engine developer (P-26), **I want** HD haptic waveform playback on Switch Joy-Con and
DualSense, **so that** fine textures and impacts are felt.

## US-6.4.3.2 Reproduce Surface Textures

**As an** engine developer (P-26), **I want** HD haptics reproducing surface textures (stone, grass,
sand), **so that** footsteps feel different per surface.

## US-6.4.3.3 Feel Different Surfaces Under Feet

**As a** player (P-23), **I want** footsteps to feel different on stone vs grass vs sand,
**so that** surface type is tactile.

## US-6.4.3.4 Define Common Waveform Format

**As an** engine developer (P-26), **I want** a common waveform format with platform-specific
conversion, **so that** HD haptics are cross-platform.

## US-6.4.3.5 Drive Joy-Con via Frequency/Amplitude Pairs

**As an** engine developer (P-26), **I want** Switch HD Rumble driven via frequency/amplitude pairs
at 5ms, **so that** Joy-Con LRA actuators are controlled.

## US-6.4.3.6 Drive DualSense via Arbitrary Waveforms

**As an** engine developer (P-26), **I want** DualSense haptics driven via arbitrary waveform
playback, **so that** voice-coil actuators produce rich feedback.

## US-6.4.3.7 Feel Lockpicking Tumbler Feedback

**As a** player (P-23), **I want** lockpicking to provide tumbler feedback, **so that** the minigame
feels physical.

## US-6.4.3.8 Author Waveforms in Visual Editor

**As a** designer (P-5), **I want to** author haptic waveforms in the visual editor, **so that** HD
haptic assets are tuned visually.

## US-6.4.3.9 Verify HD Haptics on Switch and DualSense

**As an** engine tester (P-27), **I want to** verify HD haptic playback on Switch and DualSense,
**so that** both platforms produce correct output.

## US-6.4.3.10 Feel Rain Tapping on Shield

**As a** player (P-23), **I want** rain tapping on my shield to produce fine haptic feedback,
**so that** environmental effects are tactile.

## US-6.4.3.11 Test Waveform Format Conversion

**As an** engine tester (P-27), **I want to** test common-to-platform waveform conversion,
**so that** output matches on Joy-Con and DualSense.

## US-6.4.3.12 Verify Waveform Playback Timing

**As a** QA tester (P-19), **I want to** verify waveform playback timing matches keyframes,
**so that** haptic rhythm is accurate.

## US-6.4.4.1 Generate Haptics from Audio Signals

**As an** engine developer (P-26), **I want** haptic waveforms generated from audio frequency bands
and amplitude envelopes, **so that** tactile syncs with sound.

## US-6.4.4.2 Extract Low-Frequency Band for Haptics

**As an** engine developer (P-26), **I want** frequency band extraction targeting 20-250 Hz,
**so that** haptic actuators receive perceptible frequencies.

## US-6.4.4.3 Feel Explosion Match Its Sound

**As a** player (P-23), **I want** explosions to produce matching tactile feedback, **so that**
every blast is felt through the controller.

## US-6.4.4.4 Eliminate Per-Sound Haptic Asset Need

**As a** designer (P-5), **I want** audio-driven haptics working automatically, **so that**
hand-authored haptic assets are not needed for every effect.

## US-6.4.4.5 Maintain Sync Under 10ms

**As an** engine developer (P-26), **I want** audio-to-haptic latency under 10ms, **so that**
desynchronization is imperceptible.

## US-6.4.4.6 Verify Audio-Haptic Latency

**As an** engine tester (P-27), **I want to** measure audio-to-haptic latency, **so that** it stays
under 10ms.

## US-6.4.4.7 Access Audio Output for Extraction

**As an** engine developer (P-26), **I want** access to audio mix output for band extraction,
**so that** frequency data is accurate.

## US-6.4.4.8 Configure Band Extraction Parameters

**As a** designer (P-5), **I want** frequency band and amplitude parameters configurable,
**so that** haptic generation is tunable.

## US-6.4.4.9 Feel Ambient Rumble Match Environment

**As a** player (P-23), **I want** environmental rumble (thunder, machinery) to match audio,
**so that** ambient haptics are immersive.

## US-6.4.4.10 Test for Various Sound Types

**As an** engine tester (P-27), **I want to** test audio-driven haptics for explosions, footsteps,
and ambient, **so that** generation works for all types.

## US-6.4.4.11 Verify Amplitude Envelope Match

**As a** QA tester (P-19), **I want to** verify haptic amplitude matches audio envelope, **so that**
sync is accurate.

## US-6.4.4.12 Combine Audio-Driven and Authored Haptics

**As an** engine developer (P-26), **I want** audio-driven haptics blended with authored patterns,
**so that** both sources compose.

## US-6.4.5.1 Define Named Haptic Profiles

**As a** designer (P-5), **I want** named profiles combining rumble, adaptive triggers, and HD
haptics into reusable assets, **so that** haptic design is unified.

## US-6.4.5.2 Trigger Profiles from Gameplay Events

**As a** designer (P-5), **I want** profiles triggered by events with parameter binding, **so that**
haptics are data-driven.

## US-6.4.5.3 Feel Rich Combined Feedback on DualSense

**As a** player (P-23), **I want** rich combined feedback (rumble + triggers + HD haptics) on
DualSense, **so that** feedback is immersive.

## US-6.4.5.4 Degrade Gracefully Per Platform

**As an** engine developer (P-26), **I want** profiles degrading gracefully (full on DualSense,
rumble-only on Xbox), **so that** every controller gets feedback.

## US-6.4.5.5 Configure Fallback Chain

**As a** designer (P-5), **I want** fallback chain configurable in the profile asset, **so that**
degradation is authored.

## US-6.4.5.6 Validate Profiles at Build Time

**As an** engine tester (P-27), **I want** fallback ordering validated at build time, **so that**
every profile produces feedback on every controller class.

## US-6.4.5.7 Support Parameter Binding

**As an** engine developer (P-26), **I want** profile parameters bound to gameplay values,
**so that** intensity scales dynamically.

## US-6.4.5.8 Feel Feedback on Any Controller

**As a** player (P-23), **I want** some haptic feedback on any controller, **so that** switching
does not lose all feedback.

## US-6.4.5.9 Author Profiles in Visual Editor

**As a** designer (P-5), **I want to** author haptic profiles in the editor, **so that** combined
assets are visual.

## US-6.4.5.10 Test on Xbox, DualSense, Switch Pro

**As an** engine tester (P-27), **I want to** test each profile on Xbox, DualSense, and Switch Pro,
**so that** fallback works on all.

## US-6.4.5.11 Verify Generic Rumble Fallback

**As a** QA tester (P-19), **I want to** verify profiles fallback to dual-motor rumble on generic
controllers, **so that** minimum feedback is guaranteed.

## US-6.4.5.12 Bind Impact Force to Intensity

**As an** engine developer (P-26), **I want** impact force parameter bound to rumble intensity,
**so that** stronger hits produce stronger feedback.
