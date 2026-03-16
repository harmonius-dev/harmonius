# VR Editor Mode Test Cases

Companion test cases for [vr-editor.md](vr-editor.md).

## Unit Tests

### VR Mode Manager

#### TC-15.16.1 Enter VR Mode

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `enter_vr()` with HMD connected | `Ok(())`, `state() == VrActive` | R-15.1.9 |
| 2 | `enter_vr()` without HMD connected | `Err(VrError::HmdNotFound)` | R-15.1.9 |
| 3 | `enter_vr()` while already in VR | `Err(VrError::AlreadyInVr)` | R-15.1.9 |

#### TC-15.16.2 Exit VR Mode

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `exit_vr()` while in VR | `Ok(())`, `state() == Desktop` | R-15.1.9 |
| 2 | `exit_vr()` while in desktop mode | `Err(VrError::NotInVr)` | R-15.1.9 |

### VR Input Adapter — Motion Controllers

#### TC-15.16.3 Trigger Select

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Trigger press with ray hitting entity | `EditorAction::Select(entity)` in output | R-15.1.9 |
| 2 | Trigger press with ray hitting nothing | No select action generated | R-15.1.9 |

#### TC-15.16.4 Grip Context Menu

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Grip button pressed | `EditorAction::ContextMenu(position)` in output | R-15.1.9 |

#### TC-15.16.5 Thumbstick Teleport

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Thumbstick pushed forward with ray hitting ground | `EditorAction::Teleport(ground_point)` in output | R-15.1.9 |
| 2 | Thumbstick pushed forward with ray hitting nothing | No teleport action generated | R-15.1.9 |

#### TC-15.16.6 Thumbstick Tool Cycle

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Thumbstick right while tool is Translate | `EditorAction::SetTool(Rotate)` | R-15.1.9 |
| 2 | Thumbstick right while tool is Scale | `EditorAction::SetTool(Translate)` (wraps) | R-15.1.9 |

#### TC-15.16.7 Trigger Drag Gizmo

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Trigger hold + drag on gizmo axis | `EditorAction::GizmoManipulate(delta)` per frame | R-15.1.9 |

#### TC-15.16.8 Button B Undo

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Button B pressed | `EditorAction::Undo` in output | R-15.1.9 |

#### TC-15.16.9 Both Grips Scale World

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Both grips pressed, hands pulled apart | `EditorAction::WorldScale(factor > 1.0)` | R-15.1.9 |
| 2 | Both grips pressed, hands pushed together | `EditorAction::WorldScale(factor < 1.0)` | R-15.1.9 |

### VR Input Adapter — Hand Tracking

#### TC-15.16.10 Pinch Select

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `HandGesture::Pinch` with ray hitting entity | `EditorAction::Select(entity)` | R-15.16.1 |
| 2 | `HandGesture::Pinch` with ray hitting nothing | No select action | R-15.16.1 |

#### TC-15.16.11 Grab Move

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `HandGesture::Grab` with entity selected | `EditorAction::GizmoManipulate(delta)` per frame | R-15.16.1 |

#### TC-15.16.12 Point Raycast

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `HandGesture::Point` | `pointing_ray()` direction matches index finger | R-15.16.1 |

#### TC-15.16.13 Open Palm Menu

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `HandGesture::OpenPalm` (face up) | `EditorAction::OpenMenu(palm_position)` | R-15.16.1 |

### VR Panel System

#### TC-15.16.14 Spawn Panel

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `spawn_panel(inspector, WorldFixed(pos, rot))` | Panel exists at `pos` with orientation `rot` | R-15.16.2 |

#### TC-15.16.15 Gaze Focus

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Head ray intersects panel A | `focused_panel() == Some(panel_a_id)` | R-15.16.2 |
| 2 | Head ray intersects no panel | `focused_panel() == None` | R-15.16.2 |

#### TC-15.16.16 Panel Anchor Change

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `set_anchor(id, HandAttached(Left, offset))` | Panel follows left hand position | R-15.16.2 |

#### TC-15.16.17 Radial Menu Show Dismiss

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `show_radial_menu(pos, items)` | Menu visible at position with items | R-15.16.2 |
| 2 | `dismiss_radial_menu()` | Menu no longer visible | R-15.16.2 |

### VR Follow Mode

#### TC-15.16.18 Follow User

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `follow(User(user_42))` with user in presence | `Ok(())`, `state() == TransitioningTo` | R-15.16.4 |
| 2 | `follow(User(user_99))` not in presence | `Err(VrError::FollowTargetNotFound)` | R-15.16.4 |

#### TC-15.16.19 Follow AI Agent

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `follow(AiAgent(agent_1))` with agent active | `Ok(())`, `state() == TransitioningTo` | R-15.16.4 |

#### TC-15.16.20 Break Follow on Input

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Following user, controller trigger pressed | `state()` transitions to `Independent` | R-15.16.4 |
| 2 | Following AI, hand gesture detected | `state()` transitions to `Independent` | R-15.16.4 |

#### TC-15.16.21 Follow Target Disconnects

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Following user, user disconnects from session | `state() == Independent`, `target() == None` | R-15.16.4 |

#### TC-15.16.22 Follow Camera Interpolation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `follow(target)`, call `update()` 60 times | Camera position approaches target position monotonically | R-15.16.4 |

### VR Performance Budget

#### TC-15.16.23 Quality Downgrade on Frame Drop

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Feed `adjust_quality(15.0)` (above 11.1 ms budget) | `quality()` transitions from `Full` to `Reduced` | R-15.16.5 |
| 2 | Feed `adjust_quality(15.0)` again while `Reduced` | `quality()` transitions to `Minimal` | R-15.16.5 |

#### TC-15.16.24 Quality Upgrade on Headroom

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Quality at `Reduced`, feed `adjust_quality(7.0)` for 60 frames | `quality()` transitions to `Full` | R-15.16.5 |

#### TC-15.16.25 Frame Stats Tracking

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Feed 100 frame times, `frame_stats()` | `avg_frame_ms` matches average, `p99_frame_ms` matches 99th percentile | R-15.16.5 |

### VR Avatar System

#### TC-15.16.26 Avatar Update from Presence

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 3 remote users in presence, `update_avatars()` | `avatars().len() == 3`, positions match presence | R-15.16.3 |

#### TC-15.16.27 Avatar Cursor Trail

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Remote user moves cursor through 10 positions | `cursor_trails[0].positions.len() == 10` | R-15.16.3 |
| 2 | Trail duration set to 0.5s, wait 1s | Trail positions cleared | R-15.16.3 |

#### TC-15.16.28 Avatar Name Tag Visibility

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `set_name_tags_visible(false)` | Name tags not rendered for any avatar | R-15.16.3 |
| 2 | `set_name_tags_visible(true)` | Name tags rendered above each avatar | R-15.16.3 |

## Integration Tests

### TC-15.16.30 VR Mode Full Lifecycle

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `enter_vr()`, verify stereo rendering, select entity, translate gizmo, undo, `exit_vr()` | All operations succeed, desktop mode restored, undo history intact | R-15.1.9 |

### TC-15.16.31 VR Input to Gizmo

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Trigger press on translate gizmo X axis, drag along X | Entity moves along X, `TransformCommand` on undo stack | R-15.1.9 |

### TC-15.16.32 VR Panel Interaction

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Spawn inspector panel in VR, point ray at slider, trigger press + drag | Slider value changes, undo command created | R-15.16.2 |

### TC-15.16.33 VR Collaboration Avatars

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Two VR editors in same session | Each sees the other as avatar with correct head/hand positions | R-15.16.3 |

### TC-15.16.34 VR Follow Mode Full Flow

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | User A follows user B, B moves, A's camera tracks, A presses trigger, follow breaks | Camera interpolation correct, break immediate | R-15.16.4 |

### TC-15.16.35 VR Performance Scaling

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Load heavy scene in VR, verify quality downgrades, unload, verify quality upgrades | Frame rate stays above 90 fps throughout | R-15.16.5 |

### TC-15.16.36 VR Mode on macOS

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `enter_vr()` on macOS | `Err(VrError::HmdNotFound)`, editor unaffected | R-15.1.9 |

### TC-15.16.37 VR Editor World Consistency

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Edit entity in VR, exit VR, verify entity state in desktop mode | Entity state matches VR edits | R-15.1.9 |

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
