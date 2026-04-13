# R-15.22 -- VR Editor Mode Requirements

## Requirements

1. **R-15.22.1** — The engine **SHALL** provide a VR editor mode with stereoscopic rendering, head
   tracking, and 6-DoF motion controller input sharing world state with the desktop editor.
   - **Rationale:** VR editing enables spatial level design at true player scale.
   - **Verification:** Enter VR mode, place an entity, exit, and verify the entity appears on the
     desktop viewport.

2. **R-15.22.2** — The engine **SHALL** support direct object manipulation with motion controllers
   including grab, move, rotate, scale, and snap-to-grid with haptic feedback.
   - **Rationale:** Natural spatial manipulation is VR's primary advantage over desktop editing.
   - **Verification:** Grab an object with two hands, scale it, and verify the scale value matches
     the gesture.

3. **R-15.22.3** — The engine **SHALL** support optical hand tracking with pinch, grab, and point
   gestures on supported headsets, with seamless controller fallback.
   - **Rationale:** Hand tracking enables controller-free editing.
   - **Verification:** Pinch-select an object with bare hands, pick up a controller, and verify
     seamless transition.

4. **R-15.22.4** — The engine **SHALL** provide a spatial VR UI with floating panels, radial menus,
   gaze-activated widgets, and text input via virtual keyboard or voice dictation.
   - **Rationale:** VR requires spatial UI adapted from desktop panels.
   - **Verification:** Open a radial menu, select a tool, and verify the tool activates.

5. **R-15.22.5** — The engine **SHALL** display collaborator avatars and cursor positions in VR with
   spatialized voice chat positioned at each avatar.
   - **Rationale:** Spatial presence and audio enhance collaborative VR editing.
   - **Verification:** Connect two VR users and verify each sees the other's avatar and spatialized
     audio.

6. **R-15.22.6** — The engine **SHALL** support follow mode for observing another user's or AI
   agent's viewpoint with camera motion buffering to prevent VR discomfort.
   - **Rationale:** Follow mode enables mentoring, review, and observing AI agent work.
   - **Verification:** Follow a user making rapid movements and verify camera smoothing prevents
     judder.
