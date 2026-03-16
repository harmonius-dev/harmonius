# User Stories — 6.3 Gestures

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-6.3.1.1 | engine developer (P-26) | single-tap gesture recognition with configurable distance threshold | target selection works on touch |  |  |
| US-6.3.1.2 | engine developer (P-26) | double-tap recognition with configurable timing | quick actions (auto-run) work |  |  |
| US-6.3.1.3 | engine developer (P-26) | long press recognition | context menus (invite, inspect, trade) activate on sustained touch |  |  |
| US-6.3.1.4 | player (P-23) | tap enemies to select them | target selection is intuitive on touch |  |  |
| US-6.3.1.5 | engine developer (P-26) | tap distance tolerance scaled with display DPI | mobile vs desktop touchscreens are accommodated |  |  |
| US-6.3.1.6 | engine developer (P-26) | haptic feedback at long-press threshold on supported platforms | the player knows the gesture was recognized |  |  |
| US-6.3.1.7 | designer (P-5) | tap timing and distance thresholds configurable in the editor | gesture sensitivity is tunable |  |  |
| US-6.3.1.8 | player (P-23) | long press to open a context menu on objects | interact options are accessible |  |  |
| US-6.3.1.9 | engine developer (P-26) | triple-tap recognition | text selection and special actions work |  |  |
| US-6.3.1.10 | engine tester (P-27) | verify tap, double-tap, and long press timing thresholds | recognition is accurate |  |  |
| US-6.3.1.11 | player (P-23) | long press to initiate drag for inventory rearrangement | item management works on touch |  |  |
| US-6.3.1.12 | QA tester (P-19) | test gesture recognition on screens of different DPI | tolerance scaling is validated |  |  |
| US-6.3.2.1 | engine developer (P-26) | linear swipe recognition in cardinal and diagonal directions | directional input works on touch |  |  |
| US-6.3.2.2 | engine developer (P-26) | swipe velocity and distance reported | gesture intensity is available |  |  |
| US-6.3.2.3 | player (P-23) | swiping to trigger dodge in the swipe direction | touch combat feels responsive |  |  |
| US-6.3.2.4 | designer (P-5) | directional swipe on virtual pad to cast quick-slot abilities | ability activation is fast |  |  |
| US-6.3.2.5 | engine developer (P-26) | swipe detection to filter out incidental movement during tap and long press | gestures do not conflict |  |  |
| US-6.3.2.6 | designer (P-5) | swipe velocity thresholds tunable per platform | different touch surfaces are accommodated |  |  |
| US-6.3.2.7 | engine tester (P-27) | verify swipe direction detection is accurate for all 8 directions | directional input is reliable |  |  |
| US-6.3.2.8 | player (P-23) | swiping to dismiss UI panels | panel management is quick |  |  |
| US-6.3.2.9 | engine tester (P-27) | test swipe vs tap disambiguation | gestures do not misfire |  |  |
| US-6.3.2.10 | designer (P-5) | swipe for navigation shortcuts | mobile UI flow is efficient |  |  |
| US-6.3.2.11 | engine developer (P-26) | diagonal swipe detection | 8- directional input is available |  |  |
| US-6.3.2.12 | QA tester (P-19) | test swipe recognition on phone, tablet, and trackpad | all surfaces work correctly |  |  |
| US-6.3.3.1 | engine developer (P-26) | two-finger pinch tracked with scale factor | camera zoom on mobile works |  |  |
| US-6.3.3.2 | engine developer (P-26) | two-finger rotation tracked with angle delta | camera orbit and map rotation work |  |  |
| US-6.3.3.3 | engine developer (P-26) | single/multi-finger pan tracked with position delta and velocity | camera orbiting and scrolling work |  |  |
| US-6.3.3.4 | player (P-23) | pinching to zoom the camera on mobile | view distance is controllable |  |  |
| US-6.3.3.5 | player (P-23) | two-finger rotation to rotate the map | map orientation is adjustable |  |  |
| US-6.3.3.6 | engine developer (P-26) | pinch and pan to run simultaneously | camera zoom-and-orbit is a single gesture |  |  |
| US-6.3.3.7 | engine developer (P-26) | macOS trackpad gestures via NSEvent magnification and rotation | trackpad input is native |  |  |
| US-6.3.3.8 | engine developer (P-26) | pinch and rotate disambiguated when finger arcs are ambiguous | gestures do not conflict |  |  |
| US-6.3.3.9 | player (P-23) | panning to drag inventory items on touch | item management is intuitive |  |  |
| US-6.3.3.10 | engine tester (P-27) | verify pinch scale factor accuracy | zoom corresponds to finger spread |  |  |
| US-6.3.3.11 | designer (P-5) | pan sensitivity configurable | camera orbit speed is tunable |  |  |
| US-6.3.3.12 | QA tester (P-19) | test pinch, rotate, and pan on phone, tablet, and trackpad | all devices work correctly |  |  |
| US-6.3.4.1 | engine developer (P-26) | gesture recognition as composable state machines with lifecycle states (possible, began, changed, ended, cancelled, failed) | gesture lifecycle is well-defined |  |  |
| US-6.3.4.2 | engine developer (P-26) | require-failure strategy (tap waits for double- tap timeout) | ambiguous gestures are resolved |  |  |
| US-6.3.4.3 | engine developer (P-26) | simultaneous recognition (pan and pinch together) | combined gestures work |  |  |
| US-6.3.4.4 | player (P-23) | touch input to work without gestures eating each other | controls are reliable |  |  |
| US-6.3.4.5 | engine developer (P-26) | priority ordering between conflicting recognizers | higher-priority gestures win |  |  |
| US-6.3.4.6 | designer (P-5) | conflict resolution strategies configurable in the editor | gesture behavior is tunable |  |  |
| US-6.3.4.7 | engine tester (P-27) | verify all lifecycle states fire correctly | recognizer state machines are accurate |  |  |
| US-6.3.4.8 | engine tester (P-27) | test require-failure with various timings | tap vs double-tap resolves correctly |  |  |
| US-6.3.4.9 | player (P-23) | simultaneous pinch-and-pan to work reliably | camera control feels natural |  |  |
| US-6.3.4.10 | engine developer (P-26) | gesture resolution consistent across platforms | behavior is predictable everywhere |  |  |
| US-6.3.4.11 | QA tester (P-19) | test all conflicting gesture pairs and verify resolution | no input is swallowed incorrectly |  |  |
| US-6.3.4.12 | engine tester (P-27) | test gesture cancellation and failure paths | error states are handled cleanly |  |  |
| US-6.3.5.1 | designer (P-5) | define custom gesture recognizers in the visual gesture editor | novel gestures are authored without code |  |  |
| US-6.3.5.2 | designer (P-5) | compose custom gestures from tap, swipe, hold, and pinch primitives | complex patterns are built from simple parts |  |  |
| US-6.3.5.3 | player (P-23) | draw rune shapes on screen to cast spells | gesture-based magic feels immersive |  |  |
| US-6.3.5.4 | engine developer (P-26) | custom gestures evaluated at runtime through the logic graph system | recognition is data-driven |  |  |
| US-6.3.5.5 | designer (P-5) | distance, angle, and timing thresholds set via property panels | custom gesture sensitivity is tunable |  |  |
| US-6.3.5.6 | designer (P-5) | circular swipe gestures for area-of-effect targeting | spatial abilities use touch input |  |  |
| US-6.3.5.7 | engine tester (P-27) | test custom gesture recognition accuracy | authored patterns are detected reliably |  |  |
| US-6.3.5.8 | engine developer (P-26) | primitive recognizers connected with sequencing and branching nodes | multi-step gestures are composable |  |  |
| US-6.3.5.9 | player (P-23) | signature emote gestures on touch | social expression uses custom input |  |  |
| US-6.3.5.10 | engine developer (P-26) | custom gestures supported on desktop touchscreen and macOS trackpad | non-mobile touch is covered |  |  |
| US-6.3.5.11 | QA tester (P-19) | test custom gestures on phone, tablet, and trackpad | all touch devices work |  |  |
| US-6.3.5.12 | engine tester (P-27) | verify custom gesture evaluation through logic graphs | data-driven recognition works correctly |  |  |
