# VR Editor Mode Test Cases

Companion test cases for [vr-editor.md](vr-editor.md).

## Unit Tests

### VR Mode Manager

#### TC-15.16.1 Enter VR Mode

| # | Requirement |
|---|-------------|
| 1 | R-15.1.9    |
| 2 | R-15.1.9    |
| 3 | R-15.1.9    |

1. **#1** — `enter_vr()` with HMD connected
   - **Expected:** `Ok(())`, `state() == VrActive`
2. **#2** — `enter_vr()` without HMD connected
   - **Expected:** `Err(VrError::HmdNotFound)`
3. **#3** — `enter_vr()` while already in VR
   - **Expected:** `Err(VrError::AlreadyInVr)`

#### TC-15.16.2 Exit VR Mode

| # | Requirement |
|---|-------------|
| 1 | R-15.1.9    |
| 2 | R-15.1.9    |

1. **#1** — `exit_vr()` while in VR
   - **Expected:** `Ok(())`, `state() == Desktop`
2. **#2** — `exit_vr()` while in desktop mode
   - **Expected:** `Err(VrError::NotInVr)`

### VR Input Adapter — Motion Controllers

#### TC-15.16.3 Trigger Select

| # | Requirement |
|---|-------------|
| 1 | R-15.1.9    |
| 2 | R-15.1.9    |

1. **#1** — Trigger press with ray hitting entity
   - **Expected:** `EditorAction::Select(entity)` in output
2. **#2** — Trigger press with ray hitting nothing
   - **Expected:** No select action generated

#### TC-15.16.4 Grip Context Menu

| # | Requirement |
|---|-------------|
| 1 | R-15.1.9    |

1. **#1** — Grip button pressed
   - **Expected:** `EditorAction::ContextMenu(position)` in output

#### TC-15.16.5 Thumbstick Teleport

| # | Requirement |
|---|-------------|
| 1 | R-15.1.9    |
| 2 | R-15.1.9    |

1. **#1** — Thumbstick pushed forward with ray hitting ground
   - **Expected:** `EditorAction::Teleport(ground_point)` in output
2. **#2** — Thumbstick pushed forward with ray hitting nothing
   - **Expected:** No teleport action generated

#### TC-15.16.6 Thumbstick Tool Cycle

| # | Requirement |
|---|-------------|
| 1 | R-15.1.9    |
| 2 | R-15.1.9    |

1. **#1** — Thumbstick right while tool is Translate
   - **Expected:** `EditorAction::SetTool(Rotate)`
2. **#2** — Thumbstick right while tool is Scale
   - **Expected:** `EditorAction::SetTool(Translate)` (wraps)

#### TC-15.16.7 Trigger Drag Gizmo

| # | Requirement |
|---|-------------|
| 1 | R-15.1.9    |

1. **#1** — Trigger hold + drag on gizmo axis
   - **Expected:** `EditorAction::GizmoManipulate(delta)` per frame

#### TC-15.16.8 Button B Undo

| # | Requirement |
|---|-------------|
| 1 | R-15.1.9    |

1. **#1** — Button B pressed
   - **Expected:** `EditorAction::Undo` in output

#### TC-15.16.9 Both Grips Scale World

| # | Requirement |
|---|-------------|
| 1 | R-15.1.9    |
| 2 | R-15.1.9    |

1. **#1** — Both grips pressed, hands pulled apart
   - **Expected:** `EditorAction::WorldScale(factor > 1.0)`
2. **#2** — Both grips pressed, hands pushed together
   - **Expected:** `EditorAction::WorldScale(factor < 1.0)`

### VR Input Adapter — Hand Tracking

#### TC-15.16.10 Pinch Select

| # | Requirement |
|---|-------------|
| 1 | R-15.16.1   |
| 2 | R-15.16.1   |

1. **#1** — `HandGesture::Pinch` with ray hitting entity
   - **Expected:** `EditorAction::Select(entity)`
2. **#2** — `HandGesture::Pinch` with ray hitting nothing
   - **Expected:** No select action

#### TC-15.16.11 Grab Move

| # | Requirement |
|---|-------------|
| 1 | R-15.16.1   |

1. **#1** — `HandGesture::Grab` with entity selected
   - **Expected:** `EditorAction::GizmoManipulate(delta)` per frame

#### TC-15.16.12 Point Raycast

| # | Requirement |
|---|-------------|
| 1 | R-15.16.1   |

1. **#1** — `HandGesture::Point`
   - **Expected:** `pointing_ray()` direction matches index finger

#### TC-15.16.13 Open Palm Menu

| # | Requirement |
|---|-------------|
| 1 | R-15.16.1   |

1. **#1** — `HandGesture::OpenPalm` (face up)
   - **Expected:** `EditorAction::OpenMenu(palm_position)`

### VR Panel System

#### TC-15.16.14 Spawn Panel

| # | Requirement |
|---|-------------|
| 1 | R-15.16.2   |

1. **#1** — `spawn_panel(inspector, WorldFixed(pos, rot))`
   - **Expected:** Panel exists at `pos` with orientation `rot`

#### TC-15.16.15 Gaze Focus

| # | Requirement |
|---|-------------|
| 1 | R-15.16.2   |
| 2 | R-15.16.2   |

1. **#1** — Head ray intersects panel A
   - **Expected:** `focused_panel() == Some(panel_a_id)`
2. **#2** — Head ray intersects no panel
   - **Expected:** `focused_panel() == None`

#### TC-15.16.16 Panel Anchor Change

| # | Requirement |
|---|-------------|
| 1 | R-15.16.2   |

1. **#1** — `set_anchor(id, HandAttached(Left, offset))`
   - **Expected:** Panel follows left hand position

#### TC-15.16.17 Radial Menu Show Dismiss

| # | Requirement |
|---|-------------|
| 1 | R-15.16.2   |
| 2 | R-15.16.2   |

1. **#1** — `show_radial_menu(pos, items)`
   - **Expected:** Menu visible at position with items
2. **#2** — `dismiss_radial_menu()`
   - **Expected:** Menu no longer visible

### VR Follow Mode

#### TC-15.16.18 Follow User

| # | Requirement |
|---|-------------|
| 1 | R-15.16.4   |
| 2 | R-15.16.4   |

1. **#1** — `follow(User(user_42))` with user in presence
   - **Expected:** `Ok(())`, `state() == TransitioningTo`
2. **#2** — `follow(User(user_99))` not in presence
   - **Expected:** `Err(VrError::FollowTargetNotFound)`

#### TC-15.16.19 Follow AI Agent

| # | Requirement |
|---|-------------|
| 1 | R-15.16.4   |

1. **#1** — `follow(AiAgent(agent_1))` with agent active
   - **Expected:** `Ok(())`, `state() == TransitioningTo`

#### TC-15.16.20 Break Follow on Input

| # | Requirement |
|---|-------------|
| 1 | R-15.16.4   |
| 2 | R-15.16.4   |

1. **#1** — Following user, controller trigger pressed
   - **Expected:** `state()` transitions to `Independent`
2. **#2** — Following AI, hand gesture detected
   - **Expected:** `state()` transitions to `Independent`

#### TC-15.16.21 Follow Target Disconnects

| # | Requirement |
|---|-------------|
| 1 | R-15.16.4   |

1. **#1** — Following user, user disconnects from session
   - **Expected:** `state() == Independent`, `target() == None`

#### TC-15.16.22 Follow Camera Interpolation

| # | Requirement |
|---|-------------|
| 1 | R-15.16.4   |

1. **#1** — `follow(target)`, call `update()` 60 times
   - **Expected:** Camera position approaches target position monotonically

### VR Performance Budget

#### TC-15.16.23 Quality Downgrade on Frame Drop

| # | Requirement |
|---|-------------|
| 1 | R-15.16.5   |
| 2 | R-15.16.5   |

1. **#1** — Feed `adjust_quality(15.0)` (above 11.1 ms budget)
   - **Expected:** `quality()` transitions from `Full` to `Reduced`
2. **#2** — Feed `adjust_quality(15.0)` again while `Reduced`
   - **Expected:** `quality()` transitions to `Minimal`

#### TC-15.16.24 Quality Upgrade on Headroom

| # | Requirement |
|---|-------------|
| 1 | R-15.16.5   |

1. **#1** — Quality at `Reduced`, feed `adjust_quality(7.0)` for 60 frames
   - **Expected:** `quality()` transitions to `Full`

#### TC-15.16.25 Frame Stats Tracking

| # | Requirement |
|---|-------------|
| 1 | R-15.16.5   |

1. **#1** — Feed 100 frame times, `frame_stats()`
   - **Expected:** `avg_frame_ms` matches average, `p99_frame_ms` matches 99th percentile

### VR Avatar System

#### TC-15.16.26 Avatar Update from Presence

| # | Requirement |
|---|-------------|
| 1 | R-15.16.3   |

1. **#1** — 3 remote users in presence, `update_avatars()`
   - **Expected:** `avatars().len() == 3`, positions match presence

#### TC-15.16.27 Avatar Cursor Trail

| # | Requirement |
|---|-------------|
| 1 | R-15.16.3   |
| 2 | R-15.16.3   |

1. **#1** — Remote user moves cursor through 10 positions
   - **Expected:** `cursor_trails[0].positions.len() == 10`
2. **#2** — Trail duration set to 0.5s, wait 1s
   - **Expected:** Trail positions cleared

#### TC-15.16.28 Avatar Name Tag Visibility

| # | Requirement |
|---|-------------|
| 1 | R-15.16.3   |
| 2 | R-15.16.3   |

1. **#1** — `set_name_tags_visible(false)`
   - **Expected:** Name tags not rendered for any avatar
2. **#2** — `set_name_tags_visible(true)`
   - **Expected:** Name tags rendered above each avatar

## Integration Tests

### TC-15.16.30 VR Mode Full Lifecycle

| # | Requirement |
|---|-------------|
| 1 | R-15.1.9    |

1. **#1** — `enter_vr()`, verify stereo rendering, select entity, translate gizmo, undo, `exit_vr()`
   - **Expected:** All operations succeed, desktop mode restored, undo history intact

### TC-15.16.31 VR Input to Gizmo

| # | Requirement |
|---|-------------|
| 1 | R-15.1.9    |

1. **#1** — Trigger press on translate gizmo X axis, drag along X
   - **Expected:** Entity moves along X, `TransformCommand` on undo stack

### TC-15.16.32 VR Panel Interaction

| # | Requirement |
|---|-------------|
| 1 | R-15.16.2   |

1. **#1** — Spawn inspector panel in VR, point ray at slider, trigger press + drag
   - **Expected:** Slider value changes, undo command created

### TC-15.16.33 VR Collaboration Avatars

| # | Requirement |
|---|-------------|
| 1 | R-15.16.3   |

1. **#1** — Two VR editors in same session
   - **Expected:** Each sees the other as avatar with correct head/hand positions

### TC-15.16.34 VR Follow Mode Full Flow

| # | Requirement |
|---|-------------|
| 1 | R-15.16.4   |

1. **#1** — User A follows user B, B moves, A's camera tracks, A presses trigger, follow breaks
   - **Expected:** Camera interpolation correct, break immediate

### TC-15.16.35 VR Performance Scaling

| # | Requirement |
|---|-------------|
| 1 | R-15.16.5   |

1. **#1** — Load heavy scene in VR, verify quality downgrades, unload, verify quality upgrades
   - **Expected:** Frame rate stays above 90 fps throughout

### TC-15.16.36 VR Mode on macOS

| # | Requirement |
|---|-------------|
| 1 | R-15.1.9    |

1. **#1** — `enter_vr()` on macOS
   - **Expected:** `Err(VrError::HmdNotFound)`, editor unaffected

### TC-15.16.37 VR Editor World Consistency

| # | Requirement |
|---|-------------|
| 1 | R-15.1.9    |

1. **#1** — Edit entity in VR, exit VR, verify entity state in desktop mode
   - **Expected:** Entity state matches VR edits

## Benchmarks

### TC-15.16.50 VR Frame Time Budget

| Benchmark | Target | Requirement |
|-----------|--------|-------------|
| Full VR frame (input + render + present) | < 11.1 ms (90 fps) | US-15.16.5.1 |

### TC-15.16.51 VR Input Latency

| Benchmark | Target | Requirement |
|-----------|--------|-------------|
| Controller input to gizmo response | < 20 ms | US-15.16.5.2 |

### TC-15.16.52 Avatar Rendering Overhead

| Benchmark | Target | Requirement |
|-----------|--------|-------------|
| Render 10 avatars with name tags and trails | < 1 ms | US-15.16.3.3 |

### TC-15.16.53 VR Panel Render Budget

| Benchmark | Target | Requirement |
|-----------|--------|-------------|
| Render 5 floating panels in VR | < 2 ms total | US-15.16.5.3 |
