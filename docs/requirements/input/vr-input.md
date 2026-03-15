# R-6.5 — VR and Spatial Input Requirements

## Head and Hand Tracking

### R-6.5.1 Head-Mounted Display Tracking

The engine **SHALL** provide 6DOF head tracking from VR headsets as ECS components (`HmdPose`,
`HmdVelocity`) at refresh rates up to 120Hz with late-latching pose updates, support room-scale,
seated, and standing play areas with configurable guardian boundaries, and detect tracking loss
within one frame.

- **Derived from:** [F-6.5.1](../../features/input/vr-input.md)
- **Rationale:** Low-latency 6DOF head tracking is foundational for VR presence; late-latching
  minimizes motion-to-photon latency, and tracking loss detection prevents disorienting experiences.
- **Verification:** Integration test on a VR headset: read the `HmdPose` component each frame;
  assert position and orientation values update at the headset's native refresh rate. Measure
  the timestamp delta between the last pose update and scanout; assert it is within one frame
  interval. Occlude tracking; assert a tracking-loss event fires within one frame. Verify
  room-scale boundaries trigger a guardian event when the headset position exceeds the configured
  play area.

### R-6.5.2 Motion Controller Input

The engine **SHALL** provide 6DOF hand controller tracking with pose (position + orientation),
velocity, angular velocity, button states, analog axes, and capacitive touch sensors as ECS
components, and **SHALL** map controller inputs to game actions through the input action system
identically to gamepad inputs.

- **Derived from:** [F-6.5.2](../../features/input/vr-input.md)
- **Rationale:** Unified action mapping between flat-screen and VR modes enables shared input
  configurations and reduces per-mode authoring burden.
- **Verification:** Integration test: connect VR motion controllers; verify `ControllerPose`,
  button, trigger, and thumbstick ECS components update each frame. Bind a "Fire" action to
  both a gamepad trigger and a VR controller trigger; assert the same action fires from either
  device. Verify capacitive touch sensor data is exposed for controllers that support it (e.g.,
  Oculus Touch) and absent for controllers that do not.

### R-6.5.3 Hand Tracking and Skeletal Input

The engine **SHALL** provide camera-based hand tracking with a 26-joint skeletal model per hand
as ECS components (`HandSkeleton`, `HandJointPose`), detect predefined gestures (pinch, grab,
point, thumbs-up, open palm) as input actions, and support custom gesture recognition authored
visually in the logic graph without code.

- **Derived from:** [F-6.5.3](../../features/input/vr-input.md)
- **Rationale:** Skeletal hand tracking enables natural VR interaction; predefined and custom
  gestures provide input without physical controllers, supporting the no-code engine constraint.
- **Verification:** Integration test on a hand-tracking-capable headset: display a hand; verify
  all 26 joints of `HandSkeleton` update each frame with valid positions and orientations. Perform
  a pinch gesture; assert the pinch input action fires. Author a custom "thumbs-up" gesture in the
  logic graph using joint angle thresholds; perform the gesture and assert the custom action fires.
  Hold controllers; assert the system switches from hand tracking to controller tracking
  automatically.

### R-6.5.4 Eye Tracking and Gaze Input

The engine **SHALL** expose eye tracking data as ECS components (`GazeRay`, per-eye openness,
pupil dilation) providing gaze direction usable for foveated rendering and gaze-based UI
interaction, with saccade and fixation classification for analytics.

- **Derived from:** [F-6.5.4](../../features/input/vr-input.md)
- **Rationale:** Gaze direction drives foveated rendering for GPU optimization and enables
  look-and-select interaction patterns; fixation detection supports adaptive difficulty systems.
- **Verification:** Integration test on an eye-tracking-capable headset: read the `GazeRay`
  component; assert it updates each frame with a valid direction vector. Look at a UI element;
  assert gaze-based selection highlights the element. Fixate on a point for 500ms; assert a
  fixation event fires. Perform a rapid eye movement; assert a saccade event fires. Verify
  `GazeRay` data is consumable by the foveated rendering system.

## VR Haptics

### R-6.5.5 VR Controller Haptics

The engine **SHALL** provide per-hand haptic feedback on VR controllers with amplitude, frequency,
and duration control, supporting continuous vibration patterns, impulse events, and spatially-driven
haptics where vibration intensity varies with proximity to objects, with haptic patterns authored as
visual editor assets.

- **Derived from:** [F-6.5.5](../../features/input/vr-input.md)
- **Rationale:** Per-hand haptic channels enable asymmetric feedback (sword impact in right hand,
  shield block in left), and spatial haptics increase immersion through proximity-based tactile cues.
- **Verification:** Integration test: trigger an impulse haptic on the right controller at 0.8
  amplitude and 150Hz; verify the output matches the requested parameters. Trigger a continuous
  pattern on the left controller; verify it sustains for the configured duration. Place a virtual
  object at 1m distance; verify haptic intensity increases as the controller approaches and
  decreases as it moves away, scaling linearly with inverse distance. Verify haptic pattern
  assets load and play correctly from the visual editor.

---

## Non-Functional Requirements

### R-6.5.NF1 Motion-to-Photon Latency

The engine **SHALL** achieve motion-to-photon latency (head movement to updated frame on
display) not exceeding 20 ms at the headset's native refresh rate, using late-latching pose
updates submitted after the main simulation tick.

- **Derived from:** [F-6.5.1](../../features/input/vr-input.md),
  [R-X.1.1](../cross-cutting.md) (Frame Time Budget)
- **Rationale:** Motion-to-photon latency above 20 ms causes motion sickness in VR. Late-
  latching minimizes the gap between the last pose sample and scanout.
- **Verification:** Integration test: on a VR headset with motion tracking, measure the
  timestamp delta between the pose used for rendering and the display scanout. Assert the
  delta does not exceed 20 ms at 90 Hz and 120 Hz refresh rates.

### R-6.5.NF2 Hand Tracking Joint Update Rate

The engine **SHALL** update the 26-joint skeletal hand model at a minimum of 30 Hz with
per-joint position accuracy within 5 mm of the platform SDK's reported values.

- **Derived from:** [F-6.5.3](../../features/input/vr-input.md)
- **Rationale:** Hand tracking below 30 Hz causes visible jitter in hand-rendered models
  and gesture recognition failures. Positional accuracy within 5 mm ensures gestures are
  recognized reliably.
- **Verification:** Integration test: track a hand performing a pinch gesture over 10
  seconds. Assert joint updates arrive at >= 30 Hz. Compare engine-reported joint positions
  against the platform SDK's raw values; assert < 5 mm RMS error per joint.
