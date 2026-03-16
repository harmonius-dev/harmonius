# Gestures, Haptics, and VR Input Test Cases

Companion test cases for [gestures-haptics.md](gestures-haptics.md).

## Unit Tests

### TC-6.3.1.1 Single Tap Recognition

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Touch-down + touch-up within distance and time thresholds | Single-tap fires | R-6.3.1 |

### TC-6.3.1.2 Double Tap Recognition

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Two taps within inter-tap interval | Double-tap fires; single-tap does not | R-6.3.1 |

### TC-6.3.1.3 Long Press Recognition

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Hold touch past configured duration (500 ms) | Long-press fires | R-6.3.1 |

### TC-6.3.1.4 DPI Scaling

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Set DPI to 2x reference | Distance threshold doubles from base value | R-6.3.1 |

### TC-6.3.2.1 Swipe Cardinal

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Inject rightward drag above distance and velocity thresholds | Swipe-right fires with velocity value | R-6.3.2 |

### TC-6.3.2.2 Swipe Diagonal

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Inject 45-degree diagonal drag above thresholds | Correct diagonal direction reported | R-6.3.2 |

### TC-6.3.2.3 Tap-Swipe Filter

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Inject short drag below distance threshold | No swipe fires | R-6.3.2 |

### TC-6.3.3.1 Pinch Scale

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Two fingers spreading apart | Scale factor > 1.0 | R-6.3.3 |

### TC-6.3.3.2 Rotation Angle

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Two fingers rotating 45 degrees clockwise | Angle delta == 45 degrees | R-6.3.3 |

### TC-6.3.3.3 Simultaneous Pinch and Pan

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Two fingers spread + translate simultaneously | Both pinch and pan gestures fire | R-6.3.3 |

### TC-6.3.4.1 Require-Failure Strategy

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Tap configured with require-failure of double-tap | Tap fires only after double-tap timeout expires | R-6.3.4 |

### TC-6.3.4.2 Simultaneous Strategy

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Pan + pinch configured as simultaneous | Both gestures fire concurrently | R-6.3.4 |

### TC-6.3.4.3 Priority Strategy

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Tap priority > long-press; ambiguous input | Tap wins on conflict | R-6.3.4 |

### TC-6.3.4.4 Lifecycle State Transitions

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Complete gesture from start to end | States: Possible -> Began -> Changed -> Ended | R-6.3.4 |

### TC-6.3.4.5 Cancelled State

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Interrupt gesture mid-flight (e.g., system alert) | State transitions to Cancelled | R-6.3.4 |

### TC-6.3.5.1 Custom Circle Gesture

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Author circle gesture; inject circular motion | Gesture fires | R-6.3.5 |
| 2 | Inject linear swipe motion | Gesture does not fire | R-6.3.5 |

### TC-6.3.5.2 Custom Asset Loading

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Load custom gesture as data asset; trigger hot-reload | Asset reloads; gesture still functional | R-6.3.5 |

### TC-6.4.1.1 Dual Motor Independent

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Set low_motor=0.8, high_motor=0.2 | Output low=0.8, high=0.2 | R-6.4.1 |

### TC-6.4.1.2 Envelope Timing

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Attack=100 ms, sustain=200 ms, decay=100 ms | Envelope shape correct within 5 ms tolerance | R-6.4.1 |

### TC-6.4.1.3 Priority Interruption

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Playing priority-5; trigger priority-3 | No interrupt (lower priority) | R-6.4.1 |
| 2 | Playing priority-5; trigger priority-7 | Interrupt occurs (higher priority) | R-6.4.1 |

### TC-6.4.1.4 Rumble Pattern Loop

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Play looping pattern for 3 loops | Continuous output for 3 full loops | R-6.4.1 |

### TC-6.4.2.1 Adaptive Trigger Resistance

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Apply resistance at position 0.5 on DualSense | HID output bytes match resistance at 0.5 | R-6.4.2 |

### TC-6.4.2.2 Adaptive Trigger Degradation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Apply adaptive trigger effect on Xbox controller | No error; no output (graceful degradation) | R-6.4.2 |

### TC-6.4.3.1 Waveform Conversion

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Load 100 Hz sine waveform | Switch: correct freq/amp pairs; DualSense: correct raw output | R-6.4.3 |

### TC-6.4.4.1 Audio Band Extraction

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Feed 100 Hz sine at 0.8 amplitude | Output in 80-120 Hz band | R-6.4.4 |

### TC-6.4.4.2 Audio High Frequency Rejection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Feed 5 kHz signal | Haptic amplitude near zero | R-6.4.4 |

### TC-6.4.5.1 Profile Full DualSense

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Play all-layer profile on DualSense | All 3 layers (rumble, adaptive, HD) activate | R-6.4.5 |

### TC-6.4.5.2 Profile Fallback Xbox

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Play same profile on Xbox | Only rumble layer activates | R-6.4.5 |

### TC-6.4.5.3 Parameter Binding

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Bind impact_force=0.5 to rumble profile | Rumble intensity == 50% of base amplitude | R-6.4.5 |

### TC-6.4.5.4 Fallback Validation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Validate profile missing rumble fallback at build time | Error reported | R-6.4.5 |

## VR Unit Tests

### TC-6.5.1.1 HMD 6DOF Update

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Read HmdPose each frame | Position and orientation update each frame | R-6.5.1 |

### TC-6.5.1.2 Tracking Loss Event

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Simulate tracking loss | Status changes to Lost within 1 frame | R-6.5.1 |

### TC-6.5.1.3 Guardian Boundary

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Move past guardian boundary | Guardian event fires | R-6.5.1 |

### TC-6.5.2.1 Controller 6DOF

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Read ControllerPose each frame | Position, orientation, buttons, triggers all update | R-6.5.2 |

### TC-6.5.2.2 Shared Action Mapping

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Bind Fire to gamepad trigger and VR trigger | Same action fires from either device | R-6.5.2 |

### TC-6.5.2.3 Capacitive Touch

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Query capacitive touch on Oculus Touch | Present | R-6.5.2 |
| 2 | Query capacitive touch on non-supporting controller | Absent | R-6.5.2 |

### TC-6.5.3.1 Hand 26 Joints

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Display tracked hand | All 26 joints have valid positions | R-6.5.3 |

### TC-6.5.3.2 Pinch Gesture

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Perform pinch (thumb + index close) | Pinch action fires | R-6.5.3 |

### TC-6.5.3.3 Custom Hand Gesture

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Author thumbs-up gesture in logic graph; perform gesture | Action fires | R-6.5.3 |

### TC-6.5.3.4 Auto Switch Controller to Hand

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Hold controllers | Controller mode active | R-6.5.3 |
| 2 | Release controllers | Hand tracking mode active | R-6.5.3 |

### TC-6.5.4.1 Gaze Ray Update

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Read GazeRay each frame | Valid direction vector returned | R-6.5.4 |

### TC-6.5.4.2 Fixation Detection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fixate on point for 500 ms | Fixation event fires | R-6.5.4 |

### TC-6.5.4.3 Saccade Detection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Rapid eye movement | Saccade event fires | R-6.5.4 |

### TC-6.5.5.1 VR Haptic Impulse

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Trigger impulse at amplitude=0.8, freq=150 Hz, right hand | Correct haptic output on right controller | R-6.5.5 |

### TC-6.5.5.2 VR Haptic Continuous

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Play continuous pattern for 2 s | Sustained output for full 2 s duration | R-6.5.5 |

### TC-6.5.5.3 VR Spatial Haptic

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Move controller toward object | Haptic intensity increases with proximity | R-6.5.5 |

### TC-6.5.5.4 VR Asymmetric Haptic

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Trigger different haptics per hand | Left and right outputs independent | R-6.5.5 |

## Integration Tests

### TC-6.3.NF1.I1 Gesture Latency Discrete

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Inject tap; measure frames to event | Within 1 frame | R-6.3.NF1 |

### TC-6.3.NF1.I2 Gesture Latency Continuous

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Inject swipe; measure frames from threshold crossing | Within 2 frames | R-6.3.NF1 |

### TC-6.4.NF1.I1 Haptic Output Latency

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Trigger rumble; measure time to HID report | p99 < 5 ms | R-6.4.NF1 |

### TC-6.4.4.I1 Audio-Haptic Sync

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Measure audio-to-haptic timestamp delta | Delta < 10 ms | R-6.4.4 |

### TC-6.5.NF1.I1 Motion-to-Photon Latency

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Measure pose-to-scanout delta at 90 Hz | < 20 ms | R-6.5.NF1 |

### TC-6.5.NF2.I1 Hand Tracking Rate

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Track hand for 10 s; measure update frequency | >= 30 Hz | R-6.5.NF2 |

### TC-6.5.NF2.I2 Hand Tracking Accuracy

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Compare engine joint positions vs SDK reference | < 5 mm RMS error | R-6.5.NF2 |

### TC-6.4.1.I1 Rumble All Controllers

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Test rumble on Xbox, DualSense, Switch Pro | Rumble output on all three | R-6.4.1 |

### TC-6.4.2.I1 Adaptive All Modes

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Test resistance, vibration, sectioned modes on DualSense | All modes produce correct HID output | R-6.4.2 |

### TC-6.5.1.I1 HMD All Headsets

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Test head tracking on PCVR, Quest, PSVR2 | Valid pose data from all headsets | R-6.5.1 |

### TC-6.5.2.I1 Controllers All Platforms

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Test Oculus Touch, Index, PSVR2 Sense, Quest Touch | All controllers report correct inputs | R-6.5.2 |

### TC-6.5.3.I1 Hand Tracking All

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Test hand tracking on Quest and PC OpenXR | Valid 26-joint data from both platforms | R-6.5.3 |

### TC-6.5.4.I1 Eye Tracking All

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Test on PSVR2, Quest Pro, PC Tobii | Valid gaze data from all devices | R-6.5.4 |

### TC-6.4.5.I1 Profiles All Controllers

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Test each haptic profile on each controller class | Correct layer activation per controller capability | R-6.4.5 |

## Benchmarks

### TC-6.3.NF1.B1 Gesture Recognition Per Frame

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Recognize 10 simultaneous touches per frame | Wall time | < 0.1 ms | R-6.3.NF1 |

### TC-6.4.NF1.B1 Haptic Pattern Tick

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Tick single active haptic pattern | Wall time | < 0.05 ms | R-6.4.NF1 |

### TC-6.4.NF1.B2 Profile Fallback Resolution

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Resolve fallback for single trigger | Wall time | < 0.01 ms | R-6.4.NF1 |

### TC-6.4.NF1.B3 HID Report Submission

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Submit HID report to single controller | Wall time | < 1 ms | R-6.4.NF1 |

### TC-6.5.NF1.B1 VR Pose Query

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Query HMD + 2 controller poses | Wall time | < 0.5 ms total | R-6.5.NF1 |

### TC-6.5.NF2.B1 Hand Skeleton Query

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Query 26-joint skeleton per hand | Wall time | < 0.3 ms per hand | R-6.5.NF2 |

### TC-6.5.4.B1 Eye Gaze Query

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Query eye gaze ray | Wall time | < 0.1 ms | R-6.5.4 |

### TC-6.4.4.B1 Audio Band Extraction

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Extract bands from 5 ms audio block | Wall time | < 1 ms | R-6.4.4 |
