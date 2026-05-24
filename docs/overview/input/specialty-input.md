# Specialty Input

Specialized input devices and patterns.

## What it covers

- Motion and accelerometer input: device orientation, tilt, and shake detection.
- Gyroscope input: rotational velocity for aiming assist.
- Analog triggers with dynamic sensitivity.
- Voice input: speech recognition and voice command dispatch.
- Eye tracking: gaze direction for accessibility or attention-based rendering.
- VR controller input: hand tracking and pose estimation.
- Kinect and depth sensors: skeletal tracking for full-body input.
- MIDI input: connecting musical instruments to the engine.
- Custom input protocols: mapping custom hardware via input extension points.

## Concepts

### Motion and Orientation Input

Motion sensors (accelerometer, gyroscope) report device orientation and movement. Accelerometer
measures acceleration in three axes, used to detect device tilt and shake gestures. Gyroscope
measures rotation velocity, enabling motion aiming: rotating the device tilts camera aim. These
inputs are noisy; the engine applies filtering and smoothing to reduce jitter.

### Voice and Speech Recognition

Voice input transcribes speech to text or dispatches recognized commands directly. The engine
interfaces with platform-specific speech APIs (Windows Speech Recognition, iOS Siri, Google Voice).
Commands are discrete actions (jump, attack) or continuous parameters (command volume). Voice
recognition introduces latency; the engine buffers recent recognized commands for playback syncing.

### Eye and Hand Tracking

Eye tracking reports gaze direction relative to the screen. Games apply gaze-contingent rendering:
rendering high detail where eyes look, low detail elsewhere. Hand tracking from VR controllers or
depth sensors reports finger and joint positions, enabling gesture recognition. Accessibility
applications use gaze as a mouse alternative.

### Motion Capture and Skeletal Tracking

Depth sensors (Kinect, RealSense) detect human skeleton: joints (shoulder, elbow, hip, knee) and
bones connecting them. The engine maps skeleton joints to character rig bones, retargeting
real-world motion to avatar. Hand tracking via VR controllers or depth sensors reports hand
skeleton and individual finger poses.

### MIDI and Musical Input

MIDI (Musical Instrument Digital Interface) connects keyboards, drums, and other instruments. MIDI
events report note on/off (pitch, velocity) and continuous controllers (knobs, sliders). Game
soundscapes respond to MIDI input: playing notes triggers sounds, turning knobs modifies effects.

## How it fits

- See [devices-and-actions.md](./devices-and-actions.md) for input abstraction and binding.
- See [../animation/character-and-first-person.md](../animation/character-and-first-person.md)
  for skeletal retargeting from motion capture.
- See [../game-framework/camera-and-controls.md](../game-framework/camera-and-controls.md)
  for camera control mapping.
