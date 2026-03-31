# 6.5 — VR and Spatial Input

## Head and Hand Tracking

| ID      | Feature                          |
|---------|----------------------------------|
| F-6.5.1 | Head-Mounted Display Tracking    |
| F-6.5.2 | Motion Controller Input          |
| F-6.5.3 | Hand Tracking and Skeletal Input |
| F-6.5.4 | Eye Tracking and Gaze Input      |

1. **F-6.5.1** — 6DOF head tracking from VR headsets, exposing position and orientation as ECS
   components (`HmdPose`, `HmdVelocity`). The tracking system supports room-scale, seated, and
   standing play areas with configurable guardian/chaperone boundaries. Tracking loss detection
   triggers a configurable response. Refresh rates up to 120Hz are supported with late-latching pose
   updates.
   - **Deps:** F-6.1.1
   - **Platform:** PC VR via OpenXR (SteamVR, Oculus, WMR). Quest via OVR native. PSVR2 via
     PlayStation VR SDK.
2. **F-6.5.2** — 6DOF hand controller tracking with button, trigger, thumbstick, and touchpad input.
   Each controller exposes pose, velocity, angular velocity, button states, analog axes, and
   capacitive touch sensors as ECS components. Controller models are loaded from the runtime and
   rendered in-game. The input action system (F-6.2.1) maps controller inputs to game actions
   identically to gamepad inputs.
   - **Deps:** F-6.1.3, F-6.2.1
   - **Platform:** PC via OpenXR interaction profiles. PSVR2 Sense controllers. Quest Touch.
3. **F-6.5.3** — Camera-based hand tracking providing a full 26-joint skeletal hand model per hand.
   Joint positions, orientations, and radii are exposed as ECS components. The system detects
   predefined gestures (pinch, grab, point, thumbs-up, open palm) and exposes them as input actions.
   Custom gesture recognition is authored in the logic graph (F-15.8.4). Hand tracking and
   controller tracking coexist with automatic switching.
   - **Deps:** F-6.5.2, F-6.2.1, F-15.8.4
   - **Platform:** PC via OpenXR XR_EXT_hand_tracking. Quest via Meta hand tracking SDK. PSVR2 does
     not support hand tracking.
4. **F-6.5.4** — Eye tracking providing gaze direction, fixation point, pupil dilation, and eye
   openness per eye. Gaze direction is exposed as an ECS component (`GazeRay`) usable for foveated
   rendering, gaze-based UI interaction, and gameplay mechanics. Saccade and fixation detection
   classify gaze behavior.
   - **Deps:** F-6.5.1
   - **Platform:** PC via OpenXR XR_EXT_eye_gaze_interaction and Tobii SDK. PSVR2 native eye
     tracking. Quest Pro eye tracking.

## VR Haptics

| ID      | Feature               |
|---------|-----------------------|
| F-6.5.5 | VR Controller Haptics |

1. **F-6.5.5** — Haptic feedback on VR hand controllers with amplitude, frequency, and duration
   control. Supports continuous vibration patterns, impulse events, and spatially-driven haptics
   (vibration intensity varies with proximity to objects). Per-hand haptic channels enable
   asymmetric feedback.
   - **Deps:** F-6.4.1, F-6.5.2
   - **Platform:** PC via OpenXR haptic output. PSVR2 Sense adaptive triggers and haptics. Quest
     Touch haptics.
