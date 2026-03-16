# User Stories — 6.5 VR and Spatial Input

## US-6.5.1.1 Track Head Position and Orientation in 6DOF

**As an** engine developer (P-26), **I want** 6DOF head tracking exposing position and orientation
as HmdPose and HmdVelocity ECS components, **so that** VR view matches head movement.

## US-6.5.1.2 Support Room-Scale and Seated Play Areas

**As a** designer (P-5), **I want** room-scale, seated, and standing play areas with configurable
guardian boundaries, **so that** VR setups are flexible.

## US-6.5.1.3 Look Around in VR Without Sickness

**As a** player (P-23), **I want** low-latency head tracking up to 120Hz with late-latching pose
updates, **so that** motion sickness is minimized.

## US-6.5.1.4 Detect Tracking Loss

**As an** engine developer (P-26), **I want** tracking loss detection triggering a configurable
response (freeze game, warning overlay), **so that** lost tracking is handled gracefully.

## US-6.5.1.5 Support Inside-Out and Outside-In Tracking

**As an** engine developer (P-26), **I want** both inside-out (Quest, WMR) and outside-in
(Lighthouse) tracking through the abstraction layer, **so that** all headsets work.

## US-6.5.1.6 Submit Late-Latching Pose Updates

**As an** engine developer (P-26), **I want** late-latching pose updates submitted close to scanout,
**so that** motion-to-photon latency is minimized.

## US-6.5.1.7 Verify 120Hz Tracking Rate

**As an** engine tester (P-27), **I want to** verify tracking runs at up to 120Hz, **so that**
high-refresh-rate HMDs are supported.

## US-6.5.1.8 Configure Guardian Boundaries

**As a** designer (P-5), **I want** guardian/chaperone boundaries configurable, **so that** play
area safety is enforced.

## US-6.5.1.9 Move Head and See World Respond Instantly

**As a** player (P-23), **I want** the virtual world to respond instantly to head movement,
**so that** VR presence is convincing.

## US-6.5.1.10 Test on Quest, SteamVR, and PSVR2

**As an** engine tester (P-27), **I want to** test head tracking on Quest (OVR), SteamVR (OpenXR),
and PSVR2, **so that** all platforms work.

## US-6.5.1.11 Test Tracking Loss Response

**As an** engine tester (P-27), **I want to** test tracking loss detection and response, **so that**
graceful degradation works.

## US-6.5.1.12 Verify Motion-to-Photon Latency

**As a** QA tester (P-19), **I want to** measure motion-to-photon latency, **so that** it stays
under acceptable VR thresholds.

## US-6.5.2.1 Track Controllers in 6DOF

**As an** engine developer (P-26), **I want** 6DOF hand controller tracking with pose, velocity, and
angular velocity as ECS components, **so that** hand position is known.

## US-6.5.2.2 Expose Controller Buttons and Triggers

**As an** engine developer (P-26), **I want** button states, analog axes, and capacitive touch
sensors exposed per controller, **so that** all inputs are available.

## US-6.5.2.3 Hold and Swing VR Weapons

**As a** player (P-23), **I want** hand controllers tracked in 3D space, **so that** I can hold and
swing weapons naturally.

## US-6.5.2.4 Map Controller Inputs to Game Actions

**As an** engine developer (P-26), **I want** controller inputs mapped to game actions via the
action system, **so that** VR and flat modes share input mapping.

## US-6.5.2.5 Render Controller Models In-Game

**As an** engine developer (P-26), **I want** controller models loaded from the runtime and rendered
in-game, **so that** players see their controllers.

## US-6.5.2.6 Verify OpenXR Interaction Profiles

**As an** engine tester (P-27), **I want to** verify OpenXR interaction profiles for Oculus Touch,
Index, and HP Reverb, **so that** all PC VR controllers work.

## US-6.5.2.7 Configure VR Controller Bindings

**As a** designer (P-5), **I want** VR controller bindings configurable in the action system,
**so that** controls are customizable.

## US-6.5.2.8 Test PSVR2 Sense Controllers

**As an** engine tester (P-27), **I want to** test PSVR2 Sense controller input, **so that**
PlayStation VR is supported.

## US-6.5.2.9 Interact Naturally with Hands in VR

**As a** player (P-23), **I want to** reach out and interact with objects using my hands,
**so that** VR interaction feels natural.

## US-6.5.2.10 Share Input Mapping Between Flat and VR

**As an** engine developer (P-26), **I want** shared input mapping between flat and VR modes,
**so that** game actions work in both modes.

## US-6.5.2.11 Test Controller Velocity for Throws

**As an** engine tester (P-27), **I want to** test controller velocity reporting for throw
mechanics, **so that** throwing objects uses accurate velocity.

## US-6.5.2.12 Verify All Controller Types

**As a** QA tester (P-19), **I want to** verify all supported VR controller types produce correct
input, **so that** compatibility is confirmed.

## US-6.5.3.1 Track Full 26-Joint Hand Skeleton

**As an** engine developer (P-26), **I want** camera-based hand tracking providing a 26-joint
skeletal hand model as HandSkeleton and HandJointPose components, **so that** hand pose is known.

## US-6.5.3.2 Detect Predefined Hand Gestures

**As an** engine developer (P-26), **I want** predefined gestures (pinch, grab, point, thumbs- up,
open palm) detected and exposed as actions, **so that** gesture input works.

## US-6.5.3.3 Grab Objects with Hands in VR

**As a** player (P-23), **I want to** reach out and grab objects with my hands, **so that** VR
interaction is controller-free.

## US-6.5.3.4 Author Custom Hand Gestures via Logic Graphs

**As a** designer (P-5), **I want** custom hand gestures authored visually using joint angle
thresholds in logic graphs, **so that** novel gestures are possible.

## US-6.5.3.5 Auto-Switch Between Controllers and Hands

**As an** engine developer (P-26), **I want** automatic switching between controllers and hand
tracking based on controller presence, **so that** transitions are seamless.

## US-6.5.3.6 Verify OpenXR Hand Tracking

**As an** engine tester (P-27), **I want to** verify hand tracking via OpenXR XR_EXT_hand_tracking,
**so that** PC VR hand tracking works.

## US-6.5.3.7 Test Quest Hand Tracking

**As an** engine tester (P-27), **I want to** test Quest hand tracking via Meta SDK, **so that**
standalone hand tracking works.

## US-6.5.3.8 Point at Objects to Select

**As a** player (P-23), **I want to** point at objects to select them, **so that** hand tracking
enables natural interaction.

## US-6.5.3.9 Configure Gesture Thresholds

**As a** designer (P-5), **I want** gesture recognition thresholds configurable, **so that**
sensitivity is tunable per game.

## US-6.5.3.10 Verify PSVR2 Fallback

**As an** engine tester (P-27), **I want to** verify PSVR2 gracefully handles absent hand tracking,
**so that** missing features are handled.

## US-6.5.3.11 Test Gesture Recognition Accuracy

**As a** QA tester (P-19), **I want to** test predefined gesture accuracy, **so that** pinch, grab,
and point are reliable.

## US-6.5.3.12 Cast Spells with Custom Hand Gestures

**As a** player (P-23), **I want** custom hand gestures to cast spells in VR, **so that** magic
feels immersive and physical.

## US-6.5.4.1 Track Gaze Direction Per Eye

**As an** engine developer (P-26), **I want** eye tracking providing gaze direction per eye as a
GazeRay ECS component, **so that** gaze input is available.

## US-6.5.4.2 Detect Fixation and Saccades

**As an** engine developer (P-26), **I want** saccade and fixation detection, **so that** gaze
behavior is classified for analytics and adaptive difficulty.

## US-6.5.4.3 Look at Objects to Interact

**As a** player (P-23), **I want to** look at objects to select or interact with them, **so that**
gaze-based UI works naturally.

## US-6.5.4.4 Inform Foveated Rendering from Gaze

**As an** engine developer (P-26), **I want** gaze direction usable for foveated rendering,
**so that** GPU allocates shading detail where I look.

## US-6.5.4.5 Support Gaze-Based UI Interaction

**As a** designer (P-5), **I want** gaze-based look-and-select UI, **so that** eye tracking enhances
menu navigation.

## US-6.5.4.6 Design Enemies That React to Gaze

**As a** designer (P-5), **I want** gameplay mechanics where enemies react to being looked at,
**so that** eye tracking adds depth.

## US-6.5.4.7 Report Pupil Dilation and Openness

**As an** engine developer (P-26), **I want** pupil dilation and eye openness reported, **so that**
analytics and emotion detection are possible.

## US-6.5.4.8 Verify OpenXR Eye Gaze Extension

**As an** engine tester (P-27), **I want to** verify eye tracking via OpenXR eye gaze extension,
**so that** PC VR eye tracking works.

## US-6.5.4.9 Test PSVR2 Eye Tracking

**As an** engine tester (P-27), **I want to** test PSVR2 native eye tracking, **so that**
PlayStation VR gaze works.

## US-6.5.4.10 Use Platform Calibration

**As an** engine developer (P-26), **I want** eye tracking calibration via platform's native setup,
**so that** per-user calibration is external.

## US-6.5.4.11 Test Quest Pro Eye Tracking

**As an** engine tester (P-27), **I want to** test Quest Pro eye tracking, **so that** Meta eye
tracking works.

## US-6.5.4.12 Verify Gaze Accuracy

**As a** QA tester (P-19), **I want to** verify gaze direction against known fixation targets,
**so that** eye tracking is reliable.

## US-6.5.5.1 Control Haptic Amplitude and Frequency

**As an** engine developer (P-26), **I want** VR controller haptics with amplitude, frequency, and
duration control, **so that** tactile feedback is tunable.

## US-6.5.5.2 Support Continuous Vibration Patterns

**As an** engine developer (P-26), **I want** continuous vibration patterns (engine rumble, wind),
**so that** sustained environmental haptics work.

## US-6.5.5.3 Feel Weapon Impacts Per Hand

**As a** player (P-23), **I want** weapon impacts to vibrate the hand holding the weapon,
**so that** combat feedback is directional.

## US-6.5.5.4 Support Impulse Events

**As an** engine developer (P-26), **I want** impulse haptic events (weapon fire, impacts),
**so that** discrete impacts are felt.

## US-6.5.5.5 Vary Vibration with Proximity

**As an** engine developer (P-26), **I want** vibration intensity varying with proximity to objects,
**so that** spatial awareness is tactile.

## US-6.5.5.6 Author Haptic Patterns as Assets

**As a** designer (P-5), **I want** VR haptic patterns authored as assets in the editor, **so that**
haptic design is data-driven.

## US-6.5.5.7 Trigger from Actions or Events

**As an** engine developer (P-26), **I want** haptics triggered through input actions or gameplay
events, **so that** feedback is integrated.

## US-6.5.5.8 Enable Per-Hand Channels

**As an** engine developer (P-26), **I want** per-hand haptic channels, **so that** asymmetric
feedback (sword right, shield left) works.

## US-6.5.5.9 Feel Shield Block in Left Hand

**As a** player (P-23), **I want** shield blocks to vibrate my left hand, **so that** blocking has
distinct feedback from attacking.

## US-6.5.5.10 Verify OpenXR Haptic Output

**As an** engine tester (P-27), **I want to** verify haptics via OpenXR haptic output, **so that**
PC VR haptics work.

## US-6.5.5.11 Test PSVR2 Sense Haptics

**As an** engine tester (P-27), **I want to** test PSVR2 Sense controller haptics, **so that**
PlayStation VR haptics work.

## US-6.5.5.12 Verify Haptic Timing Accuracy

**As a** QA tester (P-19), **I want to** verify haptic event timing matches trigger event,
**so that** feedback is not delayed.
