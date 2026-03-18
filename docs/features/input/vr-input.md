# 6.5 — VR and Spatial Input

## Head and Hand Tracking

| ID      | Feature                          | Requirements |
|---------|----------------------------------|--------------|
| F-6.5.1 | Head-Mounted Display Tracking    | R-6.5.1      |
| F-6.5.2 | Motion Controller Input          | R-6.5.2      |
| F-6.5.3 | Hand Tracking and Skeletal Input | R-6.5.3      |
| F-6.5.4 | Eye Tracking and Gaze Input      | R-6.5.4      |

1. **F-6.5.1** — 6DOF head tracking from VR headsets, exposing position and orientation as ECS
   components (`HmdPose`, `HmdVelocity`). The tracking system supports room-scale, seated, and
   standing play areas with configurable guardian/chaperone boundaries. Tracking loss detection
   triggers a configurable response (freeze game, show warning overlay). Supports both inside-out
   (Quest, WMR) and outside-in (Lighthouse) tracking systems through the platform abstraction layer.
   Refresh rates up to 120Hz are supported with late-latching pose updates submitted as close to
   scanout as possible. PlayStation VR SDK. Not supported on phones or Switch.
   - **Deps:** F-6.1.1 (Keyboard and Mouse Abstraction)
   - **Platform:** PC VR via OpenXR (SteamVR, Oculus, WMR). Quest via OVR native. PSVR2 via
2. **F-6.5.2** — 6DOF hand controller tracking with button, trigger, thumbstick, and touchpad input.
   Each controller exposes pose (position + orientation), velocity, angular velocity, button states,
   analog axes, and capacitive touch sensors as ECS components. Controller models are loaded from
   the runtime and rendered in-game. The input action system (F-6.2.1) maps controller inputs to
   game actions identically to gamepad inputs, enabling shared input mapping between flat and VR
   modes. PSVR2 Sense controllers. Quest Touch. Not available on phones or Switch.
   - **Deps:** F-6.1.3 (Gamepad Abstraction), F-6.2.1 (Input Actions)
   - **Platform:** PC via OpenXR interaction profiles (Oculus Touch, Valve Index, HP Reverb).
3. **F-6.5.3** — Camera-based hand tracking providing a full 26-joint skeletal hand model per hand.
   Joint positions, orientations, and radii are exposed as ECS components (`HandSkeleton`,
   `HandJointPose`). The system detects predefined gestures (pinch, grab, point, thumbs-up, open
   palm) and exposes them as input actions through the action system (F-6.2.1). Custom gesture
   recognition is authored visually in the logic graph (F-15.8.4) using joint angle and distance
   thresholds. Hand tracking and controller tracking coexist -- the system automatically switches
   based on whether controllers are held. does not support hand tracking. Not available on phones or
   Switch.
   - **Deps:** F-6.5.2, F-6.2.1 (Input Actions), F-15.8.4 (Logic Graphs)
   - **Platform:** PC via OpenXR XR_EXT_hand_tracking. Quest via Meta hand tracking SDK. PSVR2
4. **F-6.5.4** — Eye tracking providing gaze direction, fixation point, pupil dilation, and eye
   openness per eye. Gaze direction is exposed as an ECS component (`GazeRay`) usable for foveated
   rendering (informing the GPU where to allocate shading detail), gaze-based UI interaction
   (look-and-select), and gameplay mechanics (enemies react to being looked at). Saccade and
   fixation detection classify gaze behavior for analytics and adaptive difficulty. Calibration is
   performed through the platform's native eye tracking setup. tracking. Quest Pro eye tracking. Not
   available on phones or Switch.
   - **Deps:** F-6.5.1 (HMD Tracking)
   - **Platform:** PC via OpenXR XR_EXT_eye_gaze_interaction and Tobii SDK. PSVR2 native eye

## VR Haptics

| ID      | Feature               | Requirements |
|---------|-----------------------|--------------|
| F-6.5.5 | VR Controller Haptics | R-6.5.5      |

1. **F-6.5.5** — Haptic feedback on VR hand controllers with amplitude, frequency, and duration
   control. Supports continuous vibration patterns (engine rumble, wind), impulse events (weapon
   fire, impacts), and spatially-driven haptics (vibration intensity varies with proximity to
   objects). Haptic patterns are authored as assets in the visual editor and triggered through the
   input action system or gameplay events. Per-hand haptic channels enable asymmetric feedback
   (sword impact in right hand, shield block in left). Touch haptics. Not available on phones or
   Switch.
   - **Deps:** F-6.4.1 (Rumble Patterns), F-6.5.2 (Motion Controllers)
   - **Platform:** PC via OpenXR haptic output. PSVR2 Sense adaptive triggers and haptics. Quest
