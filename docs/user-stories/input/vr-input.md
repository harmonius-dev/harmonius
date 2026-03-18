# User Stories — 6.5 VR and Spatial Input

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-6.5.1.1  | engine developer (P-26) |          |              |
| US-6.5.1.2  | designer (P-5)          |          |              |
| US-6.5.1.3  | player (P-23)           |          |              |
| US-6.5.1.4  | engine developer (P-26) |          |              |
| US-6.5.1.5  | engine developer (P-26) |          |              |
| US-6.5.1.6  | engine developer (P-26) |          |              |
| US-6.5.1.7  | engine tester (P-27)    |          |              |
| US-6.5.1.8  | designer (P-5)          |          |              |
| US-6.5.1.9  | player (P-23)           |          |              |
| US-6.5.1.10 | engine tester (P-27)    |          |              |
| US-6.5.1.11 | engine tester (P-27)    |          |              |
| US-6.5.1.12 | QA tester (P-19)        |          |              |
| US-6.5.2.1  | engine developer (P-26) |          |              |
| US-6.5.2.2  | engine developer (P-26) |          |              |
| US-6.5.2.3  | player (P-23)           |          |              |
| US-6.5.2.4  | engine developer (P-26) |          |              |
| US-6.5.2.5  | engine developer (P-26) |          |              |
| US-6.5.2.6  | engine tester (P-27)    |          |              |
| US-6.5.2.7  | designer (P-5)          |          |              |
| US-6.5.2.8  | engine tester (P-27)    |          |              |
| US-6.5.2.9  | player (P-23)           |          |              |
| US-6.5.2.10 | engine developer (P-26) |          |              |
| US-6.5.2.11 | engine tester (P-27)    |          |              |
| US-6.5.2.12 | QA tester (P-19)        |          |              |
| US-6.5.3.1  | engine developer (P-26) |          |              |
| US-6.5.3.2  | engine developer (P-26) |          |              |
| US-6.5.3.3  | player (P-23)           |          |              |
| US-6.5.3.4  | designer (P-5)          |          |              |
| US-6.5.3.5  | engine developer (P-26) |          |              |
| US-6.5.3.6  | engine tester (P-27)    |          |              |
| US-6.5.3.7  | engine tester (P-27)    |          |              |
| US-6.5.3.8  | player (P-23)           |          |              |
| US-6.5.3.9  | designer (P-5)          |          |              |
| US-6.5.3.10 | engine tester (P-27)    |          |              |
| US-6.5.3.11 | QA tester (P-19)        |          |              |
| US-6.5.3.12 | player (P-23)           |          |              |
| US-6.5.4.1  | engine developer (P-26) |          |              |
| US-6.5.4.2  | engine developer (P-26) |          |              |
| US-6.5.4.3  | player (P-23)           |          |              |
| US-6.5.4.4  | engine developer (P-26) |          |              |
| US-6.5.4.5  | designer (P-5)          |          |              |
| US-6.5.4.6  | designer (P-5)          |          |              |
| US-6.5.4.7  | engine developer (P-26) |          |              |
| US-6.5.4.8  | engine tester (P-27)    |          |              |
| US-6.5.4.9  | engine tester (P-27)    |          |              |
| US-6.5.4.10 | engine developer (P-26) |          |              |
| US-6.5.4.11 | engine tester (P-27)    |          |              |
| US-6.5.4.12 | QA tester (P-19)        |          |              |
| US-6.5.5.1  | engine developer (P-26) |          |              |
| US-6.5.5.2  | engine developer (P-26) |          |              |
| US-6.5.5.3  | player (P-23)           |          |              |
| US-6.5.5.4  | engine developer (P-26) |          |              |
| US-6.5.5.5  | engine developer (P-26) |          |              |
| US-6.5.5.6  | designer (P-5)          |          |              |
| US-6.5.5.7  | engine developer (P-26) |          |              |
| US-6.5.5.8  | engine developer (P-26) |          |              |
| US-6.5.5.9  | player (P-23)           |          |              |
| US-6.5.5.10 | engine tester (P-27)    |          |              |
| US-6.5.5.11 | engine tester (P-27)    |          |              |
| US-6.5.5.12 | QA tester (P-19)        |          |              |

1. **US-6.5.1.1** — 6DOF head tracking exposing position and orientation as HmdPose and HmdVelocity
   ECS components
   - **Acceptance:** VR view matches head movement
2. **US-6.5.1.2** — room-scale, seated, and standing play areas with configurable guardian
   boundaries
   - **Acceptance:** VR setups are flexible
3. **US-6.5.1.3** — low-latency head tracking up to 120Hz with late-latching pose updates
   - **Acceptance:** motion sickness is minimized
4. **US-6.5.1.4** — tracking loss detection triggering a configurable response (freeze game, warning
   overlay)
   - **Acceptance:** lost tracking is handled gracefully
5. **US-6.5.1.5** — both inside-out (Quest, WMR) and outside-in (Lighthouse) tracking through the
   abstraction layer
   - **Acceptance:** all headsets work
6. **US-6.5.1.6** — late-latching pose updates submitted close to scanout
   - **Acceptance:** motion-to-photon latency is minimized
7. **US-6.5.1.7** — verify tracking runs at up to 120Hz
   - **Acceptance:** high-refresh-rate HMDs are supported
8. **US-6.5.1.8** — guardian/chaperone boundaries configurable
   - **Acceptance:** play area safety is enforced
9. **US-6.5.1.9** — the virtual world to respond instantly to head movement
   - **Acceptance:** VR presence is convincing
10. **US-6.5.1.10** — test head tracking on Quest (OVR), SteamVR (OpenXR), and PSVR2
    - **Acceptance:** all platforms work
11. **US-6.5.1.11** — test tracking loss detection and response
    - **Acceptance:** graceful degradation works
12. **US-6.5.1.12** — measure motion-to-photon latency
    - **Acceptance:** it stays under acceptable VR thresholds
13. **US-6.5.2.1** — 6DOF hand controller tracking with pose, velocity, and angular velocity as ECS
    components
    - **Acceptance:** hand position is known
14. **US-6.5.2.2** — button states, analog axes, and capacitive touch sensors exposed per controller
    - **Acceptance:** all inputs are available
15. **US-6.5.2.3** — hand controllers tracked in 3D space
    - **Acceptance:** I can hold and swing weapons naturally
16. **US-6.5.2.4** — controller inputs mapped to game actions via the action system
    - **Acceptance:** VR and flat modes share input mapping
17. **US-6.5.2.5** — controller models loaded from the runtime and rendered in-game
    - **Acceptance:** players see their controllers
18. **US-6.5.2.6** — verify OpenXR interaction profiles for Oculus Touch, Index, and HP Reverb
    - **Acceptance:** all PC VR controllers work
19. **US-6.5.2.7** — VR controller bindings configurable in the action system
    - **Acceptance:** controls are customizable
20. **US-6.5.2.8** — test PSVR2 Sense controller input
    - **Acceptance:** PlayStation VR is supported
21. **US-6.5.2.9** — reach out and interact with objects using my hands
    - **Acceptance:** VR interaction feels natural
22. **US-6.5.2.10** — shared input mapping between flat and VR modes
    - **Acceptance:** game actions work in both modes
23. **US-6.5.2.11** — test controller velocity reporting for throw mechanics
    - **Acceptance:** throwing objects uses accurate velocity
24. **US-6.5.2.12** — verify all supported VR controller types produce correct input
    - **Acceptance:** compatibility is confirmed
25. **US-6.5.3.1** — camera-based hand tracking providing a 26-joint skeletal hand model as
    HandSkeleton and HandJointPose components
    - **Acceptance:** hand pose is known
26. **US-6.5.3.2** — predefined gestures (pinch, grab, point, thumbs- up, open palm) detected and
    exposed as actions
    - **Acceptance:** gesture input works
27. **US-6.5.3.3** — reach out and grab objects with my hands
    - **Acceptance:** VR interaction is controller-free
28. **US-6.5.3.4** — custom hand gestures authored visually using joint angle thresholds in logic
    graphs
    - **Acceptance:** novel gestures are possible
29. **US-6.5.3.5** — automatic switching between controllers and hand tracking based on controller
    presence
    - **Acceptance:** transitions are seamless
30. **US-6.5.3.6** — verify hand tracking via OpenXR XR_EXT_hand_tracking
    - **Acceptance:** PC VR hand tracking works
31. **US-6.5.3.7** — test Quest hand tracking via Meta SDK
    - **Acceptance:** standalone hand tracking works
32. **US-6.5.3.8** — point at objects to select them
    - **Acceptance:** hand tracking enables natural interaction
33. **US-6.5.3.9** — gesture recognition thresholds configurable
    - **Acceptance:** sensitivity is tunable per game
34. **US-6.5.3.10** — verify PSVR2 gracefully handles absent hand tracking
    - **Acceptance:** missing features are handled
35. **US-6.5.3.11** — test predefined gesture accuracy
    - **Acceptance:** pinch, grab, and point are reliable
36. **US-6.5.3.12** — custom hand gestures to cast spells in VR
    - **Acceptance:** magic feels immersive and physical
37. **US-6.5.4.1** — eye tracking providing gaze direction per eye as a GazeRay ECS component
    - **Acceptance:** gaze input is available
38. **US-6.5.4.2** — saccade and fixation detection
    - **Acceptance:** gaze behavior is classified for analytics and adaptive difficulty
39. **US-6.5.4.3** — look at objects to select or interact with them
    - **Acceptance:** gaze-based UI works naturally
40. **US-6.5.4.4** — gaze direction usable for foveated rendering
    - **Acceptance:** GPU allocates shading detail where I look
41. **US-6.5.4.5** — gaze-based look-and-select UI
    - **Acceptance:** eye tracking enhances menu navigation
42. **US-6.5.4.6** — gameplay mechanics where enemies react to being looked at
    - **Acceptance:** eye tracking adds depth
43. **US-6.5.4.7** — pupil dilation and eye openness reported
    - **Acceptance:** analytics and emotion detection are possible
44. **US-6.5.4.8** — verify eye tracking via OpenXR eye gaze extension
    - **Acceptance:** PC VR eye tracking works
45. **US-6.5.4.9** — test PSVR2 native eye tracking
    - **Acceptance:** PlayStation VR gaze works
46. **US-6.5.4.10** — eye tracking calibration via platform's native setup
    - **Acceptance:** per-user calibration is external
47. **US-6.5.4.11** — test Quest Pro eye tracking
    - **Acceptance:** Meta eye tracking works
48. **US-6.5.4.12** — verify gaze direction against known fixation targets
    - **Acceptance:** eye tracking is reliable
49. **US-6.5.5.1** — VR controller haptics with amplitude, frequency, and duration control
    - **Acceptance:** tactile feedback is tunable
50. **US-6.5.5.2** — continuous vibration patterns (engine rumble, wind)
    - **Acceptance:** sustained environmental haptics work
51. **US-6.5.5.3** — weapon impacts to vibrate the hand holding the weapon
    - **Acceptance:** combat feedback is directional
52. **US-6.5.5.4** — impulse haptic events (weapon fire, impacts)
    - **Acceptance:** discrete impacts are felt
53. **US-6.5.5.5** — vibration intensity varying with proximity to objects
    - **Acceptance:** spatial awareness is tactile
54. **US-6.5.5.6** — VR haptic patterns authored as assets in the editor
    - **Acceptance:** haptic design is data-driven
55. **US-6.5.5.7** — haptics triggered through input actions or gameplay events
    - **Acceptance:** feedback is integrated
56. **US-6.5.5.8** — per-hand haptic channels
    - **Acceptance:** asymmetric feedback (sword right, shield left) works
57. **US-6.5.5.9** — shield blocks to vibrate my left hand
    - **Acceptance:** blocking has distinct feedback from attacking
58. **US-6.5.5.10** — verify haptics via OpenXR haptic output
    - **Acceptance:** PC VR haptics work
59. **US-6.5.5.11** — test PSVR2 Sense controller haptics
    - **Acceptance:** PlayStation VR haptics work
60. **US-6.5.5.12** — verify haptic event timing matches trigger event
    - **Acceptance:** feedback is not delayed
