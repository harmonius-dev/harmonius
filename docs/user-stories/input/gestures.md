# User Stories — 6.3 Gestures

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-6.3.1.1  | engine developer (P-26) |          |              |
| US-6.3.1.2  | engine developer (P-26) |          |              |
| US-6.3.1.3  | engine developer (P-26) |          |              |
| US-6.3.1.4  | player (P-23)           |          |              |
| US-6.3.1.5  | engine developer (P-26) |          |              |
| US-6.3.1.6  | engine developer (P-26) |          |              |
| US-6.3.1.7  | designer (P-5)          |          |              |
| US-6.3.1.8  | player (P-23)           |          |              |
| US-6.3.1.9  | engine developer (P-26) |          |              |
| US-6.3.1.10 | engine tester (P-27)    |          |              |
| US-6.3.1.11 | player (P-23)           |          |              |
| US-6.3.1.12 | QA tester (P-19)        |          |              |
| US-6.3.2.1  | engine developer (P-26) |          |              |
| US-6.3.2.2  | engine developer (P-26) |          |              |
| US-6.3.2.3  | player (P-23)           |          |              |
| US-6.3.2.4  | designer (P-5)          |          |              |
| US-6.3.2.5  | engine developer (P-26) |          |              |
| US-6.3.2.6  | designer (P-5)          |          |              |
| US-6.3.2.7  | engine tester (P-27)    |          |              |
| US-6.3.2.8  | player (P-23)           |          |              |
| US-6.3.2.9  | engine tester (P-27)    |          |              |
| US-6.3.2.10 | designer (P-5)          |          |              |
| US-6.3.2.11 | engine developer (P-26) |          |              |
| US-6.3.2.12 | QA tester (P-19)        |          |              |
| US-6.3.3.1  | engine developer (P-26) |          |              |
| US-6.3.3.2  | engine developer (P-26) |          |              |
| US-6.3.3.3  | engine developer (P-26) |          |              |
| US-6.3.3.4  | player (P-23)           |          |              |
| US-6.3.3.5  | player (P-23)           |          |              |
| US-6.3.3.6  | engine developer (P-26) |          |              |
| US-6.3.3.7  | engine developer (P-26) |          |              |
| US-6.3.3.8  | engine developer (P-26) |          |              |
| US-6.3.3.9  | player (P-23)           |          |              |
| US-6.3.3.10 | engine tester (P-27)    |          |              |
| US-6.3.3.11 | designer (P-5)          |          |              |
| US-6.3.3.12 | QA tester (P-19)        |          |              |
| US-6.3.4.1  | engine developer (P-26) |          |              |
| US-6.3.4.2  | engine developer (P-26) |          |              |
| US-6.3.4.3  | engine developer (P-26) |          |              |
| US-6.3.4.4  | player (P-23)           |          |              |
| US-6.3.4.5  | engine developer (P-26) |          |              |
| US-6.3.4.6  | designer (P-5)          |          |              |
| US-6.3.4.7  | engine tester (P-27)    |          |              |
| US-6.3.4.8  | engine tester (P-27)    |          |              |
| US-6.3.4.9  | player (P-23)           |          |              |
| US-6.3.4.10 | engine developer (P-26) |          |              |
| US-6.3.4.11 | QA tester (P-19)        |          |              |
| US-6.3.4.12 | engine tester (P-27)    |          |              |
| US-6.3.5.1  | designer (P-5)          |          |              |
| US-6.3.5.2  | designer (P-5)          |          |              |
| US-6.3.5.3  | player (P-23)           |          |              |
| US-6.3.5.4  | engine developer (P-26) |          |              |
| US-6.3.5.5  | designer (P-5)          |          |              |
| US-6.3.5.6  | designer (P-5)          |          |              |
| US-6.3.5.7  | engine tester (P-27)    |          |              |
| US-6.3.5.8  | engine developer (P-26) |          |              |
| US-6.3.5.9  | player (P-23)           |          |              |
| US-6.3.5.10 | engine developer (P-26) |          |              |
| US-6.3.5.11 | QA tester (P-19)        |          |              |
| US-6.3.5.12 | engine tester (P-27)    |          |              |

1. **US-6.3.1.1** — single-tap gesture recognition with configurable distance threshold
   - **Acceptance:** target selection works on touch
2. **US-6.3.1.2** — double-tap recognition with configurable timing
   - **Acceptance:** quick actions (auto-run) work
3. **US-6.3.1.3** — long press recognition
   - **Acceptance:** context menus (invite, inspect, trade) activate on sustained touch
4. **US-6.3.1.4** — tap enemies to select them
   - **Acceptance:** target selection is intuitive on touch
5. **US-6.3.1.5** — tap distance tolerance scaled with display DPI
   - **Acceptance:** mobile vs desktop touchscreens are accommodated
6. **US-6.3.1.6** — haptic feedback at long-press threshold on supported platforms
   - **Acceptance:** the player knows the gesture was recognized
7. **US-6.3.1.7** — tap timing and distance thresholds configurable in the editor
   - **Acceptance:** gesture sensitivity is tunable
8. **US-6.3.1.8** — long press to open a context menu on objects
   - **Acceptance:** interact options are accessible
9. **US-6.3.1.9** — triple-tap recognition
   - **Acceptance:** text selection and special actions work
10. **US-6.3.1.10** — verify tap, double-tap, and long press timing thresholds
    - **Acceptance:** recognition is accurate
11. **US-6.3.1.11** — long press to initiate drag for inventory rearrangement
    - **Acceptance:** item management works on touch
12. **US-6.3.1.12** — test gesture recognition on screens of different DPI
    - **Acceptance:** tolerance scaling is validated
13. **US-6.3.2.1** — linear swipe recognition in cardinal and diagonal directions
    - **Acceptance:** directional input works on touch
14. **US-6.3.2.2** — swipe velocity and distance reported
    - **Acceptance:** gesture intensity is available
15. **US-6.3.2.3** — swiping to trigger dodge in the swipe direction
    - **Acceptance:** touch combat feels responsive
16. **US-6.3.2.4** — directional swipe on virtual pad to cast quick-slot abilities
    - **Acceptance:** ability activation is fast
17. **US-6.3.2.5** — swipe detection to filter out incidental movement during tap and long press
    - **Acceptance:** gestures do not conflict
18. **US-6.3.2.6** — swipe velocity thresholds tunable per platform
    - **Acceptance:** different touch surfaces are accommodated
19. **US-6.3.2.7** — verify swipe direction detection is accurate for all 8 directions
    - **Acceptance:** directional input is reliable
20. **US-6.3.2.8** — swiping to dismiss UI panels
    - **Acceptance:** panel management is quick
21. **US-6.3.2.9** — test swipe vs tap disambiguation
    - **Acceptance:** gestures do not misfire
22. **US-6.3.2.10** — swipe for navigation shortcuts
    - **Acceptance:** mobile UI flow is efficient
23. **US-6.3.2.11** — diagonal swipe detection
    - **Acceptance:** 8- directional input is available
24. **US-6.3.2.12** — test swipe recognition on phone, tablet, and trackpad
    - **Acceptance:** all surfaces work correctly
25. **US-6.3.3.1** — two-finger pinch tracked with scale factor
    - **Acceptance:** camera zoom on mobile works
26. **US-6.3.3.2** — two-finger rotation tracked with angle delta
    - **Acceptance:** camera orbit and map rotation work
27. **US-6.3.3.3** — single/multi-finger pan tracked with position delta and velocity
    - **Acceptance:** camera orbiting and scrolling work
28. **US-6.3.3.4** — pinching to zoom the camera on mobile
    - **Acceptance:** view distance is controllable
29. **US-6.3.3.5** — two-finger rotation to rotate the map
    - **Acceptance:** map orientation is adjustable
30. **US-6.3.3.6** — pinch and pan to run simultaneously
    - **Acceptance:** camera zoom-and-orbit is a single gesture
31. **US-6.3.3.7** — macOS trackpad gestures via NSEvent magnification and rotation
    - **Acceptance:** trackpad input is native
32. **US-6.3.3.8** — pinch and rotate disambiguated when finger arcs are ambiguous
    - **Acceptance:** gestures do not conflict
33. **US-6.3.3.9** — panning to drag inventory items on touch
    - **Acceptance:** item management is intuitive
34. **US-6.3.3.10** — verify pinch scale factor accuracy
    - **Acceptance:** zoom corresponds to finger spread
35. **US-6.3.3.11** — pan sensitivity configurable
    - **Acceptance:** camera orbit speed is tunable
36. **US-6.3.3.12** — test pinch, rotate, and pan on phone, tablet, and trackpad
    - **Acceptance:** all devices work correctly
37. **US-6.3.4.1** — gesture recognition as composable state machines with lifecycle states
    (possible, began, changed, ended, cancelled, failed)
    - **Acceptance:** gesture lifecycle is well-defined
38. **US-6.3.4.2** — require-failure strategy (tap waits for double- tap timeout)
    - **Acceptance:** ambiguous gestures are resolved
39. **US-6.3.4.3** — simultaneous recognition (pan and pinch together)
    - **Acceptance:** combined gestures work
40. **US-6.3.4.4** — touch input to work without gestures eating each other
    - **Acceptance:** controls are reliable
41. **US-6.3.4.5** — priority ordering between conflicting recognizers
    - **Acceptance:** higher-priority gestures win
42. **US-6.3.4.6** — conflict resolution strategies configurable in the editor
    - **Acceptance:** gesture behavior is tunable
43. **US-6.3.4.7** — verify all lifecycle states fire correctly
    - **Acceptance:** recognizer state machines are accurate
44. **US-6.3.4.8** — test require-failure with various timings
    - **Acceptance:** tap vs double-tap resolves correctly
45. **US-6.3.4.9** — simultaneous pinch-and-pan to work reliably
    - **Acceptance:** camera control feels natural
46. **US-6.3.4.10** — gesture resolution consistent across platforms
    - **Acceptance:** behavior is predictable everywhere
47. **US-6.3.4.11** — test all conflicting gesture pairs and verify resolution
    - **Acceptance:** no input is swallowed incorrectly
48. **US-6.3.4.12** — test gesture cancellation and failure paths
    - **Acceptance:** error states are handled cleanly
49. **US-6.3.5.1** — define custom gesture recognizers in the visual gesture editor
    - **Acceptance:** novel gestures are authored without code
50. **US-6.3.5.2** — compose custom gestures from tap, swipe, hold, and pinch primitives
    - **Acceptance:** complex patterns are built from simple parts
51. **US-6.3.5.3** — draw rune shapes on screen to cast spells
    - **Acceptance:** gesture-based magic feels immersive
52. **US-6.3.5.4** — custom gestures evaluated at runtime through the logic graph system
    - **Acceptance:** recognition is data-driven
53. **US-6.3.5.5** — distance, angle, and timing thresholds set via property panels
    - **Acceptance:** custom gesture sensitivity is tunable
54. **US-6.3.5.6** — circular swipe gestures for area-of-effect targeting
    - **Acceptance:** spatial abilities use touch input
55. **US-6.3.5.7** — test custom gesture recognition accuracy
    - **Acceptance:** authored patterns are detected reliably
56. **US-6.3.5.8** — primitive recognizers connected with sequencing and branching nodes
    - **Acceptance:** multi-step gestures are composable
57. **US-6.3.5.9** — signature emote gestures on touch
    - **Acceptance:** social expression uses custom input
58. **US-6.3.5.10** — custom gestures supported on desktop touchscreen and macOS trackpad
    - **Acceptance:** non-mobile touch is covered
59. **US-6.3.5.11** — test custom gestures on phone, tablet, and trackpad
    - **Acceptance:** all touch devices work
60. **US-6.3.5.12** — verify custom gesture evaluation through logic graphs
    - **Acceptance:** data-driven recognition works correctly
