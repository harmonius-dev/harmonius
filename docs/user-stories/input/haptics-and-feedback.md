# User Stories — 6.4 Haptics & Feedback

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-6.4.1.1 | engine developer (P-26) | low-frequency and high-frequency rumble motors driven independently with intensity and duration | tactile feedback is detailed |  |  |
| US-6.4.1.2 | designer (P-5) | reusable rumble patterns as keyframe sequences with attack, sustain, and decay | haptic assets are authored visually |  |  |
| US-6.4.1.3 | player (P-23) | distinct rumble for combat hits, ability impacts, and mount locomotion | actions have tactile presence |  |  |
| US-6.4.1.4 | engine developer (P-26) | rumble patterns supporting looping, blending, and priority-based interruption | multiple haptic events compose correctly |  |  |
| US-6.4.1.5 | engine developer (P-26) | motor intensity normalized to 0.0-1.0 across all backends | rumble is consistent across controllers |  |  |
| US-6.4.1.6 | designer (P-5) | author rumble patterns in a data-driven format in the editor | haptic design is visual |  |  |
| US-6.4.1.7 | engine tester (P-27) | verify rumble on Xbox, DualSense, and Switch Pro | all controllers produce haptic feedback |  |  |
| US-6.4.1.8 | player (P-23) | rhythmic rumble when riding a horse at gallop | mount locomotion is felt |  |  |
| US-6.4.1.9 | designer (P-5) | rumble patterns triggered by gameplay events | haptic feedback is data-driven |  |  |
| US-6.4.1.10 | engine tester (P-27) | test rumble pattern priority when multiple events fire simultaneously | the highest priority wins |  |  |
| US-6.4.1.11 | engine developer (P-26) | XInput left (low-freq) and right (high-freq) motors exposed | Xbox rumble works natively |  |  |
| US-6.4.1.12 | QA tester (P-19) | verify rumble envelope (attack, sustain, decay) plays accurately | pattern timing is correct |  |  |
| US-6.4.2.1 | engine developer (P-26) | DualSense L2/R2 adaptive trigger resistance controlled per-trigger | bowstring tension and brake feel work |  |  |
| US-6.4.2.2 | engine developer (P-26) | resistance mode for adaptive triggers | bowstring draw and brake pedal have progressive resistance |  |  |
| US-6.4.2.3 | engine developer (P-26) | vibration mode on adaptive triggers | machine gun recoil is felt through L2/R2 |  |  |
| US-6.4.2.4 | player (P-23) | trigger resistance when drawing a bow | archery feels physical |  |  |
| US-6.4.2.5 | engine developer (P-26) | sectioned resistance for gear shifting notches | vehicle gear changes are tactile |  |  |
| US-6.4.2.6 | engine developer (P-26) | adaptive trigger effects to no-op on controllers without support | non-DualSense users are unaffected |  |  |
| US-6.4.2.7 | engine tester (P-27) | verify trigger control via HID output reports on USB and Bluetooth | both connection modes work |  |  |
| US-6.4.2.8 | designer (P-5) | author adaptive trigger effects in the visual editor | trigger feel is tunable |  |  |
| US-6.4.2.9 | player (P-23) | trigger resistance when reeling in a fish | fishing feels physical |  |  |
| US-6.4.2.10 | engine tester (P-27) | verify effects degrade to no-op on Xbox | fallback is clean |  |  |
| US-6.4.2.11 | designer (P-5) | trigger effects configurable per context (combat, driving, fishing) | each activity has unique feel |  |  |
| US-6.4.2.12 | QA tester (P-19) | test resistance, vibration, and sectioned modes on DualSense | all effect types work |  |  |
| US-6.4.3.1 | engine developer (P-26) | HD haptic waveform playback on Switch Joy-Con and DualSense | fine textures and impacts are felt |  |  |
| US-6.4.3.2 | engine developer (P-26) | HD haptics reproducing surface textures (stone, grass, sand) | footsteps feel different per surface |  |  |
| US-6.4.3.3 | player (P-23) | footsteps to feel different on stone vs grass vs sand | surface type is tactile |  |  |
| US-6.4.3.4 | engine developer (P-26) | a common waveform format with platform-specific conversion | HD haptics are cross-platform |  |  |
| US-6.4.3.5 | engine developer (P-26) | Switch HD Rumble driven via frequency/amplitude pairs at 5ms | Joy-Con LRA actuators are controlled |  |  |
| US-6.4.3.6 | engine developer (P-26) | DualSense haptics driven via arbitrary waveform playback | voice-coil actuators produce rich feedback |  |  |
| US-6.4.3.7 | player (P-23) | lockpicking to provide tumbler feedback | the minigame feels physical |  |  |
| US-6.4.3.8 | designer (P-5) | author haptic waveforms in the visual editor | HD haptic assets are tuned visually |  |  |
| US-6.4.3.9 | engine tester (P-27) | verify HD haptic playback on Switch and DualSense | both platforms produce correct output |  |  |
| US-6.4.3.10 | player (P-23) | rain tapping on my shield to produce fine haptic feedback | environmental effects are tactile |  |  |
| US-6.4.3.11 | engine tester (P-27) | test common-to-platform waveform conversion | output matches on Joy-Con and DualSense |  |  |
| US-6.4.3.12 | QA tester (P-19) | verify waveform playback timing matches keyframes | haptic rhythm is accurate |  |  |
| US-6.4.4.1 | engine developer (P-26) | haptic waveforms generated from audio frequency bands and amplitude envelopes | tactile syncs with sound |  |  |
| US-6.4.4.2 | engine developer (P-26) | frequency band extraction targeting 20-250 Hz | haptic actuators receive perceptible frequencies |  |  |
| US-6.4.4.3 | player (P-23) | explosions to produce matching tactile feedback | every blast is felt through the controller |  |  |
| US-6.4.4.4 | designer (P-5) | audio-driven haptics working automatically | hand-authored haptic assets are not needed for every effect |  |  |
| US-6.4.4.5 | engine developer (P-26) | audio-to-haptic latency under 10ms | desynchronization is imperceptible |  |  |
| US-6.4.4.6 | engine tester (P-27) | measure audio-to-haptic latency | it stays under 10ms |  |  |
| US-6.4.4.7 | engine developer (P-26) | access to audio mix output for band extraction | frequency data is accurate |  |  |
| US-6.4.4.8 | designer (P-5) | frequency band and amplitude parameters configurable | haptic generation is tunable |  |  |
| US-6.4.4.9 | player (P-23) | environmental rumble (thunder, machinery) to match audio | ambient haptics are immersive |  |  |
| US-6.4.4.10 | engine tester (P-27) | test audio-driven haptics for explosions, footsteps, and ambient | generation works for all types |  |  |
| US-6.4.4.11 | QA tester (P-19) | verify haptic amplitude matches audio envelope | sync is accurate |  |  |
| US-6.4.4.12 | engine developer (P-26) | audio-driven haptics blended with authored patterns | both sources compose |  |  |
| US-6.4.5.1 | designer (P-5) | named profiles combining rumble, adaptive triggers, and HD haptics into reusable assets | haptic design is unified |  |  |
| US-6.4.5.2 | designer (P-5) | profiles triggered by events with parameter binding | haptics are data-driven |  |  |
| US-6.4.5.3 | player (P-23) | rich combined feedback (rumble + triggers + HD haptics) on DualSense | feedback is immersive |  |  |
| US-6.4.5.4 | engine developer (P-26) | profiles degrading gracefully (full on DualSense, rumble-only on Xbox) | every controller gets feedback |  |  |
| US-6.4.5.5 | designer (P-5) | fallback chain configurable in the profile asset | degradation is authored |  |  |
| US-6.4.5.6 | engine tester (P-27) | fallback ordering validated at build time | every profile produces feedback on every controller class |  |  |
| US-6.4.5.7 | engine developer (P-26) | profile parameters bound to gameplay values | intensity scales dynamically |  |  |
| US-6.4.5.8 | player (P-23) | some haptic feedback on any controller | switching does not lose all feedback |  |  |
| US-6.4.5.9 | designer (P-5) | author haptic profiles in the editor | combined assets are visual |  |  |
| US-6.4.5.10 | engine tester (P-27) | test each profile on Xbox, DualSense, and Switch Pro | fallback works on all |  |  |
| US-6.4.5.11 | QA tester (P-19) | verify profiles fallback to dual-motor rumble on generic controllers | minimum feedback is guaranteed |  |  |
| US-6.4.5.12 | engine developer (P-26) | impact force parameter bound to rumble intensity | stronger hits produce stronger feedback |  |  |
