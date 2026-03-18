# User Stories — 6.4 Haptics & Feedback

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-6.4.1.1  | engine developer (P-26) |          |              |
| US-6.4.1.2  | designer (P-5)          |          |              |
| US-6.4.1.3  | player (P-23)           |          |              |
| US-6.4.1.4  | engine developer (P-26) |          |              |
| US-6.4.1.5  | engine developer (P-26) |          |              |
| US-6.4.1.6  | designer (P-5)          |          |              |
| US-6.4.1.7  | engine tester (P-27)    |          |              |
| US-6.4.1.8  | player (P-23)           |          |              |
| US-6.4.1.9  | designer (P-5)          |          |              |
| US-6.4.1.10 | engine tester (P-27)    |          |              |
| US-6.4.1.11 | engine developer (P-26) |          |              |
| US-6.4.1.12 | QA tester (P-19)        |          |              |
| US-6.4.2.1  | engine developer (P-26) |          |              |
| US-6.4.2.2  | engine developer (P-26) |          |              |
| US-6.4.2.3  | engine developer (P-26) |          |              |
| US-6.4.2.4  | player (P-23)           |          |              |
| US-6.4.2.5  | engine developer (P-26) |          |              |
| US-6.4.2.6  | engine developer (P-26) |          |              |
| US-6.4.2.7  | engine tester (P-27)    |          |              |
| US-6.4.2.8  | designer (P-5)          |          |              |
| US-6.4.2.9  | player (P-23)           |          |              |
| US-6.4.2.10 | engine tester (P-27)    |          |              |
| US-6.4.2.11 | designer (P-5)          |          |              |
| US-6.4.2.12 | QA tester (P-19)        |          |              |
| US-6.4.3.1  | engine developer (P-26) |          |              |
| US-6.4.3.2  | engine developer (P-26) |          |              |
| US-6.4.3.3  | player (P-23)           |          |              |
| US-6.4.3.4  | engine developer (P-26) |          |              |
| US-6.4.3.5  | engine developer (P-26) |          |              |
| US-6.4.3.6  | engine developer (P-26) |          |              |
| US-6.4.3.7  | player (P-23)           |          |              |
| US-6.4.3.8  | designer (P-5)          |          |              |
| US-6.4.3.9  | engine tester (P-27)    |          |              |
| US-6.4.3.10 | player (P-23)           |          |              |
| US-6.4.3.11 | engine tester (P-27)    |          |              |
| US-6.4.3.12 | QA tester (P-19)        |          |              |
| US-6.4.4.1  | engine developer (P-26) |          |              |
| US-6.4.4.2  | engine developer (P-26) |          |              |
| US-6.4.4.3  | player (P-23)           |          |              |
| US-6.4.4.4  | designer (P-5)          |          |              |
| US-6.4.4.5  | engine developer (P-26) |          |              |
| US-6.4.4.6  | engine tester (P-27)    |          |              |
| US-6.4.4.7  | engine developer (P-26) |          |              |
| US-6.4.4.8  | designer (P-5)          |          |              |
| US-6.4.4.9  | player (P-23)           |          |              |
| US-6.4.4.10 | engine tester (P-27)    |          |              |
| US-6.4.4.11 | QA tester (P-19)        |          |              |
| US-6.4.4.12 | engine developer (P-26) |          |              |
| US-6.4.5.1  | designer (P-5)          |          |              |
| US-6.4.5.2  | designer (P-5)          |          |              |
| US-6.4.5.3  | player (P-23)           |          |              |
| US-6.4.5.4  | engine developer (P-26) |          |              |
| US-6.4.5.5  | designer (P-5)          |          |              |
| US-6.4.5.6  | engine tester (P-27)    |          |              |
| US-6.4.5.7  | engine developer (P-26) |          |              |
| US-6.4.5.8  | player (P-23)           |          |              |
| US-6.4.5.9  | designer (P-5)          |          |              |
| US-6.4.5.10 | engine tester (P-27)    |          |              |
| US-6.4.5.11 | QA tester (P-19)        |          |              |
| US-6.4.5.12 | engine developer (P-26) |          |              |

1. **US-6.4.1.1** — low-frequency and high-frequency rumble motors driven independently with
   intensity and duration
   - **Acceptance:** tactile feedback is detailed
2. **US-6.4.1.2** — reusable rumble patterns as keyframe sequences with attack, sustain, and decay
   - **Acceptance:** haptic assets are authored visually
3. **US-6.4.1.3** — distinct rumble for combat hits, ability impacts, and mount locomotion
   - **Acceptance:** actions have tactile presence
4. **US-6.4.1.4** — rumble patterns supporting looping, blending, and priority-based interruption
   - **Acceptance:** multiple haptic events compose correctly
5. **US-6.4.1.5** — motor intensity normalized to 0.0-1.0 across all backends
   - **Acceptance:** rumble is consistent across controllers
6. **US-6.4.1.6** — author rumble patterns in a data-driven format in the editor
   - **Acceptance:** haptic design is visual
7. **US-6.4.1.7** — verify rumble on Xbox, DualSense, and Switch Pro
   - **Acceptance:** all controllers produce haptic feedback
8. **US-6.4.1.8** — rhythmic rumble when riding a horse at gallop
   - **Acceptance:** mount locomotion is felt
9. **US-6.4.1.9** — rumble patterns triggered by gameplay events
   - **Acceptance:** haptic feedback is data-driven
10. **US-6.4.1.10** — test rumble pattern priority when multiple events fire simultaneously
    - **Acceptance:** the highest priority wins
11. **US-6.4.1.11** — XInput left (low-freq) and right (high-freq) motors exposed
    - **Acceptance:** Xbox rumble works natively
12. **US-6.4.1.12** — verify rumble envelope (attack, sustain, decay) plays accurately
    - **Acceptance:** pattern timing is correct
13. **US-6.4.2.1** — DualSense L2/R2 adaptive trigger resistance controlled per-trigger
    - **Acceptance:** bowstring tension and brake feel work
14. **US-6.4.2.2** — resistance mode for adaptive triggers
    - **Acceptance:** bowstring draw and brake pedal have progressive resistance
15. **US-6.4.2.3** — vibration mode on adaptive triggers
    - **Acceptance:** machine gun recoil is felt through L2/R2
16. **US-6.4.2.4** — trigger resistance when drawing a bow
    - **Acceptance:** archery feels physical
17. **US-6.4.2.5** — sectioned resistance for gear shifting notches
    - **Acceptance:** vehicle gear changes are tactile
18. **US-6.4.2.6** — adaptive trigger effects to no-op on controllers without support
    - **Acceptance:** non-DualSense users are unaffected
19. **US-6.4.2.7** — verify trigger control via HID output reports on USB and Bluetooth
    - **Acceptance:** both connection modes work
20. **US-6.4.2.8** — author adaptive trigger effects in the visual editor
    - **Acceptance:** trigger feel is tunable
21. **US-6.4.2.9** — trigger resistance when reeling in a fish
    - **Acceptance:** fishing feels physical
22. **US-6.4.2.10** — verify effects degrade to no-op on Xbox
    - **Acceptance:** fallback is clean
23. **US-6.4.2.11** — trigger effects configurable per context (combat, driving, fishing)
    - **Acceptance:** each activity has unique feel
24. **US-6.4.2.12** — test resistance, vibration, and sectioned modes on DualSense
    - **Acceptance:** all effect types work
25. **US-6.4.3.1** — HD haptic waveform playback on Switch Joy-Con and DualSense
    - **Acceptance:** fine textures and impacts are felt
26. **US-6.4.3.2** — HD haptics reproducing surface textures (stone, grass, sand)
    - **Acceptance:** footsteps feel different per surface
27. **US-6.4.3.3** — footsteps to feel different on stone vs grass vs sand
    - **Acceptance:** surface type is tactile
28. **US-6.4.3.4** — a common waveform format with platform-specific conversion
    - **Acceptance:** HD haptics are cross-platform
29. **US-6.4.3.5** — Switch HD Rumble driven via frequency/amplitude pairs at 5ms
    - **Acceptance:** Joy-Con LRA actuators are controlled
30. **US-6.4.3.6** — DualSense haptics driven via arbitrary waveform playback
    - **Acceptance:** voice-coil actuators produce rich feedback
31. **US-6.4.3.7** — lockpicking to provide tumbler feedback
    - **Acceptance:** the minigame feels physical
32. **US-6.4.3.8** — author haptic waveforms in the visual editor
    - **Acceptance:** HD haptic assets are tuned visually
33. **US-6.4.3.9** — verify HD haptic playback on Switch and DualSense
    - **Acceptance:** both platforms produce correct output
34. **US-6.4.3.10** — rain tapping on my shield to produce fine haptic feedback
    - **Acceptance:** environmental effects are tactile
35. **US-6.4.3.11** — test common-to-platform waveform conversion
    - **Acceptance:** output matches on Joy-Con and DualSense
36. **US-6.4.3.12** — verify waveform playback timing matches keyframes
    - **Acceptance:** haptic rhythm is accurate
37. **US-6.4.4.1** — haptic waveforms generated from audio frequency bands and amplitude envelopes
    - **Acceptance:** tactile syncs with sound
38. **US-6.4.4.2** — frequency band extraction targeting 20-250 Hz
    - **Acceptance:** haptic actuators receive perceptible frequencies
39. **US-6.4.4.3** — explosions to produce matching tactile feedback
    - **Acceptance:** every blast is felt through the controller
40. **US-6.4.4.4** — audio-driven haptics working automatically
    - **Acceptance:** hand-authored haptic assets are not needed for every effect
41. **US-6.4.4.5** — audio-to-haptic latency under 10ms
    - **Acceptance:** desynchronization is imperceptible
42. **US-6.4.4.6** — measure audio-to-haptic latency
    - **Acceptance:** it stays under 10ms
43. **US-6.4.4.7** — access to audio mix output for band extraction
    - **Acceptance:** frequency data is accurate
44. **US-6.4.4.8** — frequency band and amplitude parameters configurable
    - **Acceptance:** haptic generation is tunable
45. **US-6.4.4.9** — environmental rumble (thunder, machinery) to match audio
    - **Acceptance:** ambient haptics are immersive
46. **US-6.4.4.10** — test audio-driven haptics for explosions, footsteps, and ambient
    - **Acceptance:** generation works for all types
47. **US-6.4.4.11** — verify haptic amplitude matches audio envelope
    - **Acceptance:** sync is accurate
48. **US-6.4.4.12** — audio-driven haptics blended with authored patterns
    - **Acceptance:** both sources compose
49. **US-6.4.5.1** — named profiles combining rumble, adaptive triggers, and HD haptics into
    reusable assets
    - **Acceptance:** haptic design is unified
50. **US-6.4.5.2** — profiles triggered by events with parameter binding
    - **Acceptance:** haptics are data-driven
51. **US-6.4.5.3** — rich combined feedback (rumble + triggers + HD haptics) on DualSense
    - **Acceptance:** feedback is immersive
52. **US-6.4.5.4** — profiles degrading gracefully (full on DualSense, rumble-only on Xbox)
    - **Acceptance:** every controller gets feedback
53. **US-6.4.5.5** — fallback chain configurable in the profile asset
    - **Acceptance:** degradation is authored
54. **US-6.4.5.6** — fallback ordering validated at build time
    - **Acceptance:** every profile produces feedback on every controller class
55. **US-6.4.5.7** — profile parameters bound to gameplay values
    - **Acceptance:** intensity scales dynamically
56. **US-6.4.5.8** — some haptic feedback on any controller
    - **Acceptance:** switching does not lose all feedback
57. **US-6.4.5.9** — author haptic profiles in the editor
    - **Acceptance:** combined assets are visual
58. **US-6.4.5.10** — test each profile on Xbox, DualSense, and Switch Pro
    - **Acceptance:** fallback works on all
59. **US-6.4.5.11** — verify profiles fallback to dual-motor rumble on generic controllers
    - **Acceptance:** minimum feedback is guaranteed
60. **US-6.4.5.12** — impact force parameter bound to rumble intensity
    - **Acceptance:** stronger hits produce stronger feedback
