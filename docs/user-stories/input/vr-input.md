# User Stories — 6.5 VR and Spatial Input

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-6.5.1.1 | engine developer (P-26) | 6DOF head tracking exposing position and orientation as HmdPose and HmdVelocity ECS components | VR view matches head movement |  |  |
| US-6.5.1.2 | designer (P-5) | room-scale, seated, and standing play areas with configurable guardian boundaries | VR setups are flexible |  |  |
| US-6.5.1.3 | player (P-23) | low-latency head tracking up to 120Hz with late-latching pose updates | motion sickness is minimized |  |  |
| US-6.5.1.4 | engine developer (P-26) | tracking loss detection triggering a configurable response (freeze game, warning overlay) | lost tracking is handled gracefully |  |  |
| US-6.5.1.5 | engine developer (P-26) | both inside-out (Quest, WMR) and outside-in (Lighthouse) tracking through the abstraction layer | all headsets work |  |  |
| US-6.5.1.6 | engine developer (P-26) | late-latching pose updates submitted close to scanout | motion-to-photon latency is minimized |  |  |
| US-6.5.1.7 | engine tester (P-27) | verify tracking runs at up to 120Hz | high-refresh-rate HMDs are supported |  |  |
| US-6.5.1.8 | designer (P-5) | guardian/chaperone boundaries configurable | play area safety is enforced |  |  |
| US-6.5.1.9 | player (P-23) | the virtual world to respond instantly to head movement | VR presence is convincing |  |  |
| US-6.5.1.10 | engine tester (P-27) | test head tracking on Quest (OVR), SteamVR (OpenXR), and PSVR2 | all platforms work |  |  |
| US-6.5.1.11 | engine tester (P-27) | test tracking loss detection and response | graceful degradation works |  |  |
| US-6.5.1.12 | QA tester (P-19) | measure motion-to-photon latency | it stays under acceptable VR thresholds |  |  |
| US-6.5.2.1 | engine developer (P-26) | 6DOF hand controller tracking with pose, velocity, and angular velocity as ECS components | hand position is known |  |  |
| US-6.5.2.2 | engine developer (P-26) | button states, analog axes, and capacitive touch sensors exposed per controller | all inputs are available |  |  |
| US-6.5.2.3 | player (P-23) | hand controllers tracked in 3D space | I can hold and swing weapons naturally |  |  |
| US-6.5.2.4 | engine developer (P-26) | controller inputs mapped to game actions via the action system | VR and flat modes share input mapping |  |  |
| US-6.5.2.5 | engine developer (P-26) | controller models loaded from the runtime and rendered in-game | players see their controllers |  |  |
| US-6.5.2.6 | engine tester (P-27) | verify OpenXR interaction profiles for Oculus Touch, Index, and HP Reverb | all PC VR controllers work |  |  |
| US-6.5.2.7 | designer (P-5) | VR controller bindings configurable in the action system | controls are customizable |  |  |
| US-6.5.2.8 | engine tester (P-27) | test PSVR2 Sense controller input | PlayStation VR is supported |  |  |
| US-6.5.2.9 | player (P-23) | reach out and interact with objects using my hands | VR interaction feels natural |  |  |
| US-6.5.2.10 | engine developer (P-26) | shared input mapping between flat and VR modes | game actions work in both modes |  |  |
| US-6.5.2.11 | engine tester (P-27) | test controller velocity reporting for throw mechanics | throwing objects uses accurate velocity |  |  |
| US-6.5.2.12 | QA tester (P-19) | verify all supported VR controller types produce correct input | compatibility is confirmed |  |  |
| US-6.5.3.1 | engine developer (P-26) | camera-based hand tracking providing a 26-joint skeletal hand model as HandSkeleton and HandJointPose components | hand pose is known |  |  |
| US-6.5.3.2 | engine developer (P-26) | predefined gestures (pinch, grab, point, thumbs- up, open palm) detected and exposed as actions | gesture input works |  |  |
| US-6.5.3.3 | player (P-23) | reach out and grab objects with my hands | VR interaction is controller-free |  |  |
| US-6.5.3.4 | designer (P-5) | custom hand gestures authored visually using joint angle thresholds in logic graphs | novel gestures are possible |  |  |
| US-6.5.3.5 | engine developer (P-26) | automatic switching between controllers and hand tracking based on controller presence | transitions are seamless |  |  |
| US-6.5.3.6 | engine tester (P-27) | verify hand tracking via OpenXR XR_EXT_hand_tracking | PC VR hand tracking works |  |  |
| US-6.5.3.7 | engine tester (P-27) | test Quest hand tracking via Meta SDK | standalone hand tracking works |  |  |
| US-6.5.3.8 | player (P-23) | point at objects to select them | hand tracking enables natural interaction |  |  |
| US-6.5.3.9 | designer (P-5) | gesture recognition thresholds configurable | sensitivity is tunable per game |  |  |
| US-6.5.3.10 | engine tester (P-27) | verify PSVR2 gracefully handles absent hand tracking | missing features are handled |  |  |
| US-6.5.3.11 | QA tester (P-19) | test predefined gesture accuracy | pinch, grab, and point are reliable |  |  |
| US-6.5.3.12 | player (P-23) | custom hand gestures to cast spells in VR | magic feels immersive and physical |  |  |
| US-6.5.4.1 | engine developer (P-26) | eye tracking providing gaze direction per eye as a GazeRay ECS component | gaze input is available |  |  |
| US-6.5.4.2 | engine developer (P-26) | saccade and fixation detection | gaze behavior is classified for analytics and adaptive difficulty |  |  |
| US-6.5.4.3 | player (P-23) | look at objects to select or interact with them | gaze-based UI works naturally |  |  |
| US-6.5.4.4 | engine developer (P-26) | gaze direction usable for foveated rendering | GPU allocates shading detail where I look |  |  |
| US-6.5.4.5 | designer (P-5) | gaze-based look-and-select UI | eye tracking enhances menu navigation |  |  |
| US-6.5.4.6 | designer (P-5) | gameplay mechanics where enemies react to being looked at | eye tracking adds depth |  |  |
| US-6.5.4.7 | engine developer (P-26) | pupil dilation and eye openness reported | analytics and emotion detection are possible |  |  |
| US-6.5.4.8 | engine tester (P-27) | verify eye tracking via OpenXR eye gaze extension | PC VR eye tracking works |  |  |
| US-6.5.4.9 | engine tester (P-27) | test PSVR2 native eye tracking | PlayStation VR gaze works |  |  |
| US-6.5.4.10 | engine developer (P-26) | eye tracking calibration via platform's native setup | per-user calibration is external |  |  |
| US-6.5.4.11 | engine tester (P-27) | test Quest Pro eye tracking | Meta eye tracking works |  |  |
| US-6.5.4.12 | QA tester (P-19) | verify gaze direction against known fixation targets | eye tracking is reliable |  |  |
| US-6.5.5.1 | engine developer (P-26) | VR controller haptics with amplitude, frequency, and duration control | tactile feedback is tunable |  |  |
| US-6.5.5.2 | engine developer (P-26) | continuous vibration patterns (engine rumble, wind) | sustained environmental haptics work |  |  |
| US-6.5.5.3 | player (P-23) | weapon impacts to vibrate the hand holding the weapon | combat feedback is directional |  |  |
| US-6.5.5.4 | engine developer (P-26) | impulse haptic events (weapon fire, impacts) | discrete impacts are felt |  |  |
| US-6.5.5.5 | engine developer (P-26) | vibration intensity varying with proximity to objects | spatial awareness is tactile |  |  |
| US-6.5.5.6 | designer (P-5) | VR haptic patterns authored as assets in the editor | haptic design is data-driven |  |  |
| US-6.5.5.7 | engine developer (P-26) | haptics triggered through input actions or gameplay events | feedback is integrated |  |  |
| US-6.5.5.8 | engine developer (P-26) | per-hand haptic channels | asymmetric feedback (sword right, shield left) works |  |  |
| US-6.5.5.9 | player (P-23) | shield blocks to vibrate my left hand | blocking has distinct feedback from attacking |  |  |
| US-6.5.5.10 | engine tester (P-27) | verify haptics via OpenXR haptic output | PC VR haptics work |  |  |
| US-6.5.5.11 | engine tester (P-27) | test PSVR2 Sense controller haptics | PlayStation VR haptics work |  |  |
| US-6.5.5.12 | QA tester (P-19) | verify haptic event timing matches trigger event | feedback is not delayed |  |  |
