# Gestures, Haptics, and VR Input Test Cases

Companion test cases for [gestures-haptics.md](gestures-haptics.md).

## Unit Tests

### TC-6.3.1.1 Single Tap Recognition

| # | Requirement |
|---|-------------|
| 1 | R-6.3.1     |

1. **#1** — Touch-down + touch-up within distance and time thresholds
   - **Expected:** Single-tap fires

### TC-6.3.1.2 Double Tap Recognition

| # | Requirement |
|---|-------------|
| 1 | R-6.3.1     |

1. **#1** — Two taps within inter-tap interval
   - **Expected:** Double-tap fires; single-tap does not

### TC-6.3.1.3 Long Press Recognition

| # | Requirement |
|---|-------------|
| 1 | R-6.3.1     |

1. **#1** — Hold touch past configured duration (500 ms)
   - **Expected:** Long-press fires

### TC-6.3.1.4 DPI Scaling

| # | Requirement |
|---|-------------|
| 1 | R-6.3.1     |

1. **#1** — Set DPI to 2x reference
   - **Expected:** Distance threshold doubles from base value

### TC-6.3.2.1 Swipe Cardinal

| # | Requirement |
|---|-------------|
| 1 | R-6.3.2     |

1. **#1** — Inject rightward drag above distance and velocity thresholds
   - **Expected:** Swipe-right fires with velocity value

### TC-6.3.2.2 Swipe Diagonal

| # | Requirement |
|---|-------------|
| 1 | R-6.3.2     |

1. **#1** — Inject 45-degree diagonal drag above thresholds
   - **Expected:** Correct diagonal direction reported

### TC-6.3.2.3 Tap-Swipe Filter

| # | Requirement |
|---|-------------|
| 1 | R-6.3.2     |

1. **#1** — Inject short drag below distance threshold
   - **Expected:** No swipe fires

### TC-6.3.3.1 Pinch Scale

| # | Requirement |
|---|-------------|
| 1 | R-6.3.3     |

1. **#1** — Two fingers spreading apart
   - **Expected:** Scale factor > 1.0

### TC-6.3.3.2 Rotation Angle

| # | Requirement |
|---|-------------|
| 1 | R-6.3.3     |

1. **#1** — Two fingers rotating 45 degrees clockwise
   - **Expected:** Angle delta == 45 degrees

### TC-6.3.3.3 Simultaneous Pinch and Pan

| # | Requirement |
|---|-------------|
| 1 | R-6.3.3     |

1. **#1** — Two fingers spread + translate simultaneously
   - **Expected:** Both pinch and pan gestures fire

### TC-6.3.4.1 Require-Failure Strategy

| # | Requirement |
|---|-------------|
| 1 | R-6.3.4     |

1. **#1** — Tap configured with require-failure of double-tap
   - **Expected:** Tap fires only after double-tap timeout expires

### TC-6.3.4.2 Simultaneous Strategy

| # | Requirement |
|---|-------------|
| 1 | R-6.3.4     |

1. **#1** — Pan + pinch configured as simultaneous
   - **Expected:** Both gestures fire concurrently

### TC-6.3.4.3 Priority Strategy

| # | Requirement |
|---|-------------|
| 1 | R-6.3.4     |

1. **#1** — Tap priority > long-press; ambiguous input
   - **Expected:** Tap wins on conflict

### TC-6.3.4.4 Lifecycle State Transitions

| # | Requirement |
|---|-------------|
| 1 | R-6.3.4     |

1. **#1** — Complete gesture from start to end
   - **Expected:** States: Possible -> Began -> Changed -> Ended

### TC-6.3.4.5 Cancelled State

| # | Requirement |
|---|-------------|
| 1 | R-6.3.4     |

1. **#1** — Interrupt gesture mid-flight (e.g., system alert)
   - **Expected:** State transitions to Cancelled

### TC-6.3.5.1 Custom Circle Gesture

| # | Requirement |
|---|-------------|
| 1 | R-6.3.5     |
| 2 | R-6.3.5     |

1. **#1** — Author circle gesture; inject circular motion
   - **Expected:** Gesture fires
2. **#2** — Inject linear swipe motion
   - **Expected:** Gesture does not fire

### TC-6.3.5.2 Custom Asset Loading

| # | Requirement |
|---|-------------|
| 1 | R-6.3.5     |

1. **#1** — Load custom gesture as data asset; trigger hot-reload
   - **Expected:** Asset reloads; gesture still functional

### TC-6.4.1.1 Dual Motor Independent

| # | Requirement |
|---|-------------|
| 1 | R-6.4.1     |

1. **#1** — Set low_motor=0.8, high_motor=0.2
   - **Expected:** Output low=0.8, high=0.2

### TC-6.4.1.2 Envelope Timing

| # | Requirement |
|---|-------------|
| 1 | R-6.4.1     |

1. **#1** — Attack=100 ms, sustain=200 ms, decay=100 ms
   - **Expected:** Envelope shape correct within 5 ms tolerance

### TC-6.4.1.3 Priority Interruption

| # | Requirement |
|---|-------------|
| 1 | R-6.4.1     |
| 2 | R-6.4.1     |

1. **#1** — Playing priority-5; trigger priority-3
   - **Expected:** No interrupt (lower priority)
2. **#2** — Playing priority-5; trigger priority-7
   - **Expected:** Interrupt occurs (higher priority)

### TC-6.4.1.4 Rumble Pattern Loop

| # | Requirement |
|---|-------------|
| 1 | R-6.4.1     |

1. **#1** — Play looping pattern for 3 loops
   - **Expected:** Continuous output for 3 full loops

### TC-6.4.2.1 Adaptive Trigger Resistance

| # | Requirement |
|---|-------------|
| 1 | R-6.4.2     |

1. **#1** — Apply resistance at position 0.5 on DualSense
   - **Expected:** HID output bytes match resistance at 0.5

### TC-6.4.2.2 Adaptive Trigger Degradation

| # | Requirement |
|---|-------------|
| 1 | R-6.4.2     |

1. **#1** — Apply adaptive trigger effect on Xbox controller
   - **Expected:** No error; no output (graceful degradation)

### TC-6.4.3.1 Waveform Conversion

| # | Requirement |
|---|-------------|
| 1 | R-6.4.3     |

1. **#1** — Load 100 Hz sine waveform
   - **Expected:** Switch: correct freq/amp pairs; DualSense: correct raw output

### TC-6.4.4.1 Audio Band Extraction

| # | Requirement |
|---|-------------|
| 1 | R-6.4.4     |

1. **#1** — Feed 100 Hz sine at 0.8 amplitude
   - **Expected:** Output in 80-120 Hz band

### TC-6.4.4.2 Audio High Frequency Rejection

| # | Requirement |
|---|-------------|
| 1 | R-6.4.4     |

1. **#1** — Feed 5 kHz signal
   - **Expected:** Haptic amplitude near zero

### TC-6.4.5.1 Profile Full DualSense

| # | Requirement |
|---|-------------|
| 1 | R-6.4.5     |

1. **#1** — Play all-layer profile on DualSense
   - **Expected:** All 3 layers (rumble, adaptive, HD) activate

### TC-6.4.5.2 Profile Fallback Xbox

| # | Requirement |
|---|-------------|
| 1 | R-6.4.5     |

1. **#1** — Play same profile on Xbox
   - **Expected:** Only rumble layer activates

### TC-6.4.5.3 Parameter Binding

| # | Requirement |
|---|-------------|
| 1 | R-6.4.5     |

1. **#1** — Bind impact_force=0.5 to rumble profile
   - **Expected:** Rumble intensity == 50% of base amplitude

### TC-6.4.5.4 Fallback Validation

| # | Requirement |
|---|-------------|
| 1 | R-6.4.5     |

1. **#1** — Validate profile missing rumble fallback at build time
   - **Expected:** Error reported

## VR Unit Tests

### TC-6.5.1.1 HMD 6DOF Update

| # | Requirement |
|---|-------------|
| 1 | R-6.5.1     |

1. **#1** — Read HmdPose each frame
   - **Expected:** Position and orientation update each frame

### TC-6.5.1.2 Tracking Loss Event

| # | Requirement |
|---|-------------|
| 1 | R-6.5.1     |

1. **#1** — Simulate tracking loss
   - **Expected:** Status changes to Lost within 1 frame

### TC-6.5.1.3 Guardian Boundary

| # | Requirement |
|---|-------------|
| 1 | R-6.5.1     |

1. **#1** — Move past guardian boundary
   - **Expected:** Guardian event fires

### TC-6.5.2.1 Controller 6DOF

| # | Requirement |
|---|-------------|
| 1 | R-6.5.2     |

1. **#1** — Read ControllerPose each frame
   - **Expected:** Position, orientation, buttons, triggers all update

### TC-6.5.2.2 Shared Action Mapping

| # | Requirement |
|---|-------------|
| 1 | R-6.5.2     |

1. **#1** — Bind Fire to gamepad trigger and VR trigger
   - **Expected:** Same action fires from either device

### TC-6.5.2.3 Capacitive Touch

| # | Requirement |
|---|-------------|
| 1 | R-6.5.2     |
| 2 | R-6.5.2     |

1. **#1** — Query capacitive touch on Oculus Touch
   - **Expected:** Present
2. **#2** — Query capacitive touch on non-supporting controller
   - **Expected:** Absent

### TC-6.5.3.1 Hand 26 Joints

| # | Requirement |
|---|-------------|
| 1 | R-6.5.3     |

1. **#1** — Display tracked hand
   - **Expected:** All 26 joints have valid positions

### TC-6.5.3.2 Pinch Gesture

| # | Requirement |
|---|-------------|
| 1 | R-6.5.3     |

1. **#1** — Perform pinch (thumb + index close)
   - **Expected:** Pinch action fires

### TC-6.5.3.3 Custom Hand Gesture

| # | Requirement |
|---|-------------|
| 1 | R-6.5.3     |

1. **#1** — Author thumbs-up gesture in logic graph; perform gesture
   - **Expected:** Action fires

### TC-6.5.3.4 Auto Switch Controller to Hand

| # | Requirement |
|---|-------------|
| 1 | R-6.5.3     |
| 2 | R-6.5.3     |

1. **#1** — Hold controllers
   - **Expected:** Controller mode active
2. **#2** — Release controllers
   - **Expected:** Hand tracking mode active

### TC-6.5.4.1 Gaze Ray Update

| # | Requirement |
|---|-------------|
| 1 | R-6.5.4     |

1. **#1** — Read GazeRay each frame
   - **Expected:** Valid direction vector returned

### TC-6.5.4.2 Fixation Detection

| # | Requirement |
|---|-------------|
| 1 | R-6.5.4     |

1. **#1** — Fixate on point for 500 ms
   - **Expected:** Fixation event fires

### TC-6.5.4.3 Saccade Detection

| # | Requirement |
|---|-------------|
| 1 | R-6.5.4     |

1. **#1** — Rapid eye movement
   - **Expected:** Saccade event fires

### TC-6.5.5.1 VR Haptic Impulse

| # | Requirement |
|---|-------------|
| 1 | R-6.5.5     |

1. **#1** — Trigger impulse at amplitude=0.8, freq=150 Hz, right hand
   - **Expected:** Correct haptic output on right controller

### TC-6.5.5.2 VR Haptic Continuous

| # | Requirement |
|---|-------------|
| 1 | R-6.5.5     |

1. **#1** — Play continuous pattern for 2 s
   - **Expected:** Sustained output for full 2 s duration

### TC-6.5.5.3 VR Spatial Haptic

| # | Requirement |
|---|-------------|
| 1 | R-6.5.5     |

1. **#1** — Move controller toward object
   - **Expected:** Haptic intensity increases with proximity

### TC-6.5.5.4 VR Asymmetric Haptic

| # | Requirement |
|---|-------------|
| 1 | R-6.5.5     |

1. **#1** — Trigger different haptics per hand
   - **Expected:** Left and right outputs independent

## Integration Tests

### TC-6.3.NF1.I1 Gesture Latency Discrete

| # | Requirement |
|---|-------------|
| 1 | R-6.3.NF1   |

1. **#1** — Inject tap; measure frames to event
   - **Expected:** Within 1 frame

### TC-6.3.NF1.I2 Gesture Latency Continuous

| # | Requirement |
|---|-------------|
| 1 | R-6.3.NF1   |

1. **#1** — Inject swipe; measure frames from threshold crossing
   - **Expected:** Within 2 frames

### TC-6.4.NF1.I1 Haptic Output Latency

| # | Requirement |
|---|-------------|
| 1 | R-6.4.NF1   |

1. **#1** — Trigger rumble; measure time to HID report
   - **Expected:** p99 < 5 ms

### TC-6.4.4.I1 Audio-Haptic Sync

| # | Requirement |
|---|-------------|
| 1 | R-6.4.4     |

1. **#1** — Measure audio-to-haptic timestamp delta
   - **Expected:** Delta < 10 ms

### TC-6.5.NF1.I1 Motion-to-Photon Latency

| # | Requirement |
|---|-------------|
| 1 | R-6.5.NF1   |

1. **#1** — Measure pose-to-scanout delta at 90 Hz
   - **Expected:** < 20 ms

### TC-6.5.NF2.I1 Hand Tracking Rate

| # | Requirement |
|---|-------------|
| 1 | R-6.5.NF2   |

1. **#1** — Track hand for 10 s; measure update frequency
   - **Expected:** >= 30 Hz

### TC-6.5.NF2.I2 Hand Tracking Accuracy

| # | Requirement |
|---|-------------|
| 1 | R-6.5.NF2   |

1. **#1** — Compare engine joint positions vs SDK reference
   - **Expected:** < 5 mm RMS error

### TC-6.4.1.I1 Rumble All Controllers

| # | Requirement |
|---|-------------|
| 1 | R-6.4.1     |

1. **#1** — Test rumble on Xbox, DualSense, Switch Pro
   - **Expected:** Rumble output on all three

### TC-6.4.2.I1 Adaptive All Modes

| # | Requirement |
|---|-------------|
| 1 | R-6.4.2     |

1. **#1** — Test resistance, vibration, sectioned modes on DualSense
   - **Expected:** All modes produce correct HID output

### TC-6.5.1.I1 HMD All Headsets

| # | Requirement |
|---|-------------|
| 1 | R-6.5.1     |

1. **#1** — Test head tracking on PCVR, Quest, PSVR2
   - **Expected:** Valid pose data from all headsets

### TC-6.5.2.I1 Controllers All Platforms

| # | Requirement |
|---|-------------|
| 1 | R-6.5.2     |

1. **#1** — Test Oculus Touch, Index, PSVR2 Sense, Quest Touch
   - **Expected:** All controllers report correct inputs

### TC-6.5.3.I1 Hand Tracking All

| # | Requirement |
|---|-------------|
| 1 | R-6.5.3     |

1. **#1** — Test hand tracking on Quest and PC OpenXR
   - **Expected:** Valid 26-joint data from both platforms

### TC-6.5.4.I1 Eye Tracking All

| # | Requirement |
|---|-------------|
| 1 | R-6.5.4     |

1. **#1** — Test on PSVR2, Quest Pro, PC Tobii
   - **Expected:** Valid gaze data from all devices

### TC-6.4.5.I1 Profiles All Controllers

| # | Requirement |
|---|-------------|
| 1 | R-6.4.5     |

1. **#1** — Test each haptic profile on each controller class
   - **Expected:** Correct layer activation per controller capability

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
