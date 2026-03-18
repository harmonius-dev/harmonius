# R-15.22 -- VR Editor Mode Requirements

## Immersive Editing

| ID         | Derived From                                          |
|------------|-------------------------------------------------------|
| R-15.22.1  | [F-15.22.1](../../features/tools-editor/vr-editor.md) |
| R-15.22.1a | [F-15.22.1](../../features/tools-editor/vr-editor.md) |
| R-15.22.2  | [F-15.22.2](../../features/tools-editor/vr-editor.md) |
| R-15.22.2a | [F-15.22.2](../../features/tools-editor/vr-editor.md) |
| R-15.22.3  | [F-15.22.3](../../features/tools-editor/vr-editor.md) |
| R-15.22.3a | [F-15.22.3](../../features/tools-editor/vr-editor.md) |
| R-15.22.4  | [F-15.22.4](../../features/tools-editor/vr-editor.md) |
| R-15.22.4a | [F-15.22.4](../../features/tools-editor/vr-editor.md) |
| R-15.22.5  | [F-15.22.5](../../features/tools-editor/vr-editor.md) |
| R-15.22.5a | [F-15.22.5](../../features/tools-editor/vr-editor.md) |
| R-15.22.6  | [F-15.22.6](../../features/tools-editor/vr-editor.md) |
| R-15.22.6a | [F-15.22.6](../../features/tools-editor/vr-editor.md) |

1. **R-15.22.1** — The editor **SHALL** provide a full VR editing mode with stereoscopic rendering,
   head tracking, and 6-DoF motion controller input via OpenXR. The VR editor **SHALL** share world
   state with the desktop editor so changes are visible in both views simultaneously. The editor
   **SHALL** support room-scale and seated configurations and enter/exit VR mode without restart.
   - **Rationale:** VR editing lets designers experience spatial proportions at true player scale
     with natural gestures.
   - **Verification:** Integration test: enter VR mode, place an entity via motion controller, and
     verify it appears at the correct position in the desktop viewport.
2. **R-15.22.1a** — The VR editor **SHALL** maintain a minimum of 90 fps (or the headset's native
   refresh rate) to prevent VR discomfort. UI rendering **SHALL NOT** cause frame drops below the
   headset refresh rate.
   - **Rationale:** Sub-threshold frame rates cause motion sickness in VR.
   - **Verification:** Benchmark: measure frame time during typical editing operations in VR and
     verify 99th percentile stays below the headset's frame budget (11.1 ms at 90 Hz).
3. **R-15.22.2** — The editor **SHALL** support direct manipulation of scene objects using motion
   controllers with single-hand grab for translation and rotation, two-hand grab for scaling, and
   trigger-based precision placement with snap-to-grid. Controller haptic feedback **SHALL** confirm
   grab, release, and snap events. Transform operations **SHALL** integrate with the undo/redo stack
   (F-15.1.3).
   - **Rationale:** Natural hand manipulation in VR is faster and more intuitive than mouse-based
     gizmos for spatial editing.
   - **Verification:** Integration test: grab an object with one controller, move it, release, undo
     the transform, and verify the object returns to its original position.
4. **R-15.22.2a** — Snap-to-grid in VR **SHALL** use the same snap increments as desktop gizmos
   (F-15.1.5) and produce numerically identical results. Snap events **SHALL** trigger haptic pulses
   of at least 20 ms duration.
   - **Rationale:** Consistent snap behavior between VR and desktop prevents placement
     discrepancies.
   - **Verification:** Unit test: snap an object to the same grid in VR and desktop and verify
     positions match within epsilon.
5. **R-15.22.3** — The editor **SHALL** support bare-hand editing on headsets with optical hand
   tracking via the `XR_EXT_hand_tracking` OpenXR extension. Pinch gestures **SHALL** select and
   grab objects. Open palm **SHALL** activate the tool palette. Hand tracking **SHALL** coexist with
   controller input and switch seamlessly when controllers are set down or picked up.
   - **Rationale:** Hand tracking removes the barrier of holding controllers for quick edits.
   - **Verification:** Integration test: pick up an object with a pinch gesture, release it, then
     pick up a controller and verify the input mode switches seamlessly.
6. **R-15.22.3a** — The gesture recognizer **SHALL** achieve at least 95% recognition accuracy for
   pinch, grab, open palm, and point gestures under normal lighting conditions. False positive rate
   **SHALL** be below 2%.
   - **Rationale:** Unreliable gesture recognition frustrates users and causes accidental edits.
   - **Verification:** Benchmark: perform 100 of each gesture type and measure recognition accuracy
     and false positive rate.
7. **R-15.22.4** — The editor **SHALL** provide a spatial UI system for VR with floating panels
   positioned in world space, radial menus on each controller for quick tool access, gaze-activated
   widgets on headsets with eye tracking, and a virtual keyboard or voice dictation for text input.
   UI elements **SHALL** scale with distance to maintain readability.
   - **Rationale:** Traditional 2D UI does not translate to VR; spatial UI with radial menus
     provides efficient tool access.
   - **Verification:** Integration test: pin a floating panel, move it with a grab gesture, and
     verify it stays at the new position. Verify radial menu tool selection activates the correct
     tool.
8. **R-15.22.4a** — UI text in VR panels **SHALL** be readable at distances from 0.5 to 3.0 meters.
   Font size **SHALL** scale automatically based on panel distance from the user's head. Minimum
   rendered text height **SHALL** be at least 2 mm at the panel's position.
   - **Rationale:** Unreadable text in VR forces users to lean in, breaking workflow.
   - **Verification:** Unit test: render text at 0.5, 1.0, 2.0, and 3.0 meters and verify font size
     scales to maintain the minimum height.
9. **R-15.22.5** — The editor **SHALL** display other users' avatars (head and hand models) and
   cursors in VR space during collaborative sessions. Cursor trails **SHALL** visualize recent edit
   activity. Voice chat audio **SHALL** be spatialized to each user's avatar position. The system
   **SHALL** integrate with the collaboration presence system (F-15.12.3) and voice chat
   (F-15.12.10).
   - **Rationale:** Seeing collaborators in VR space provides spatial awareness during team editing
     sessions.
   - **Verification:** Integration test: connect two users in VR, verify each sees the other's
     avatar at the correct position, and verify spatial audio originates from the avatar.
10. **R-15.22.5a** — Avatar rendering **SHOULD** use simplified meshes (under 5,000 triangles per
    avatar) to maintain VR frame rate. With 8 simultaneous avatars visible, rendering overhead
    **SHALL** stay below 1 ms per frame.
    - **Rationale:** VR frame rate is critical; avatar rendering must not cause dropped frames.
    - **Verification:** Benchmark: render 8 avatars and measure the per-frame rendering cost.
11. **R-15.22.6** — The editor **SHALL** support a follow mode in VR that mirrors another user's or
    AI agent's camera position and orientation with a configurable offset. The followed user's
    actions (selection, placement, tool changes) **SHALL** be highlighted with visual indicators. AI
    agents (F-15.12.12) **SHALL** be followable for observing automated workflows.
    - **Rationale:** Follow mode enables mentoring, review, and AI observation without manual camera
      navigation.
    - **Verification:** Integration test: follow a user, verify the VR camera tracks their position,
      and verify the followed user's tool changes are highlighted.
12. **R-15.22.6a** — Follow mode **SHALL** apply a latency buffer of 1-2 frames to smooth camera
    motion. Sudden movements by the followed user **SHALL** be damped to prevent VR discomfort. The
    user **SHALL** be able to exit follow mode instantly by pressing any controller button.
    - **Rationale:** Abrupt camera motion in VR causes nausea; smoothing is essential.
    - **Verification:** Unit test: simulate a sudden 90-degree rotation by the followed user and
      verify the follower's camera smoothly interpolates over 1-2 frames.

---

## User Story Traceability

User stories for this domain are maintained in
[user-stories/tools-editor/vr-editor.md](../../user-stories/tools-editor/vr-editor.md). Requirements
in this document are derived from those user stories.
