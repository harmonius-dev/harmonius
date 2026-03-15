# R-15.4 — Animation Editor Requirements

## Timeline and Curves

### R-15.4.1 Animation Timeline

The engine **SHALL** provide a multi-track timeline for viewing and editing animation clips with
color-coded keyframe tracks for translation, rotation, scale, and custom properties, supporting
playback controls, scrubbing, frame stepping, looping, speed adjustment, and keyframe move, copy,
and snap operations.

- **Derived from:** [F-15.4.1](../../features/tools-editor/animation-editor.md)
- **Rationale:** A multi-track timeline is the standard animation editing paradigm, and keyframe
  manipulation with playback controls is essential for authoring and reviewing animation clips.
- **Verification:** Integration test: load an animation clip with translation and rotation tracks.
  Verify keyframes display on correct tracks with distinct colors. Move a keyframe to a new frame,
  scrub to that frame, and verify the pose matches the keyframe value. Copy a keyframe, paste it
  at a different time, and verify the pose is duplicated. Toggle looping and verify playback wraps.

### R-15.4.2 Curve Editor

The engine **SHALL** provide an animation curve editor with Bezier and Hermite tangent modes,
per-channel visibility toggles, tangent manipulation handles, auto-tangent computation, curve
presets (ease-in, ease-out, linear, stepped), and box selection for multi-curve batch editing.

- **Derived from:** [F-15.4.2](../../features/tools-editor/animation-editor.md)
- **Rationale:** Fine-grained curve editing controls interpolation quality between keyframes, which
  directly determines whether animation motion appears natural or mechanical.
- **Verification:** Integration test: create a two-keyframe animation, switch to the curve editor,
  and set Bezier tangent mode. Drag a tangent handle and verify the interpolated value at the
  midpoint changes accordingly. Apply the ease-in preset and verify the curve shape matches the
  expected ease-in profile. Select multiple curves via box selection, apply stepped mode, and
  verify all selected curves switch to stepped interpolation.

## Skeleton and Preview

### R-15.4.3 Skeleton Viewer

The engine **SHALL** render the skeleton hierarchy as a 3D bone visualization overlaid on the mesh,
with selectable bones that highlight children, constraints, and IK chains, supporting toggleable
display modes (octahedral, stick, axes) and multi-skeleton overlay for comparison.

- **Derived from:** [F-15.4.3](../../features/tools-editor/animation-editor.md)
- **Rationale:** Visual bone selection with hierarchy highlighting and constraint display is
  essential for animators to understand and debug complex rigs and IK setups.
- **Verification:** Integration test: load a rigged character, select a bone, and verify its
  children and associated IK chain highlight. Switch between octahedral, stick, and axes display
  modes and verify the bone visualization changes. Load a second skeleton, enable overlay, and
  verify both skeletons render simultaneously with distinguishable colors.

### R-15.4.4 Animation Preview

The engine **SHALL** provide a dedicated animation preview viewport with configurable ground plane,
lighting, and camera orbit, supporting playback of blend results, layered animations, and root
motion trajectories, with both game-quality rendering and debug overlays (velocity vectors,
contact points, bone trails).

- **Derived from:** [F-15.4.4](../../features/tools-editor/animation-editor.md)
- **Rationale:** Previewing animations in isolation with debug overlays enables animators to
  evaluate motion quality, blending artifacts, and root motion paths without running the full game.
- **Verification:** Integration test: play an animation in the preview viewport and verify the
  character mesh animates correctly on the ground plane. Enable root motion visualization and
  verify the trajectory path renders. Layer two animations and verify the blended result plays.
  Toggle debug overlays (velocity vectors, bone trails) and verify they render over the mesh.

## Non-Functional Requirements

### R-15.4.NF1 Animation Playback and Scrubbing Performance

The animation timeline **SHALL** support scrubbing at interactive frame rates (above 30 FPS) for
characters with up to 300 bones. Keyframe insertion, deletion, and move operations **SHALL** complete
within one frame (under 33ms). The curve editor **SHALL** maintain interactive performance with up
to 10,000 keyframes visible simultaneously. Blend space preview **SHALL** update within one frame
when the parameter crosshair is moved.

- **Derived from:** F-15.4.1 through F-15.4.7 (all animation editor features).
- **Rationale:** Animators scrub through timelines continuously while evaluating motion quality.
  Dropped frames during scrub or slow keyframe operations break the evaluation workflow.
- **Verification:** Load a 300-bone character with a 5,000-frame animation. Scrub through the
  timeline and assert frame rate stays above 30 FPS. Insert 100 keyframes sequentially and assert
  each completes within 33ms. Display 10,000 keyframes in the curve editor and assert pan/zoom
  remains interactive.

## Blend and State Authoring

### R-15.4.5 Blend Space Editor

The engine **SHALL** provide a 1D and 2D blend space editor where animation clips are positioned
in parameter space, with drag-based sample repositioning, interpolation region visualization,
and real-time blended output preview driven by a movable parameter-space crosshair.

- **Derived from:** [F-15.4.5](../../features/tools-editor/animation-editor.md)
- **Rationale:** Blend spaces drive locomotion systems (speed vs. direction blending), and visual
  editing with live preview enables animators to tune interpolation without code changes.
- **Verification:** Integration test: create a 2D blend space with four animation samples at corner
  positions. Move the crosshair to the center and verify the preview shows an equal blend of all
  four samples. Move a sample to a new position, move the crosshair near it, and verify that
  sample dominates the blend. Verify interpolation region boundaries update when samples are
  repositioned.

### R-15.4.6 State Machine Editor

The engine **SHALL** provide a visual node-graph editor for authoring animation state machines
with states (clips or blend spaces), transitions (blend durations, conditions, interruption
rules), active-state highlighting during preview playback, and real-time transition evaluation
display.

- **Derived from:** [F-15.4.6](../../features/tools-editor/animation-editor.md)
- **Rationale:** Visual state machine editing with live debugging enables animators to author and
  troubleshoot complex locomotion graphs without inspecting code or log output.
- **Verification:** Integration test: create a state machine with idle and walk states connected by
  a speed-threshold transition. Play the preview, set speed above the threshold, and verify the
  active-state highlight moves from idle to walk. Verify the transition evaluator displays the
  blend duration and condition status in real time. Add an interruption rule and verify a mid-
  transition interrupt triggers correctly.

### R-15.4.7 Retargeting Setup

The engine **SHALL** provide a retargeting editor with side-by-side source and target skeleton
views, bone-pair assignment, limb length ratio adjustment, and real-time retargeted animation
preview, enabling animation sharing across differently proportioned rigs.

- **Derived from:** [F-15.4.7](../../features/tools-editor/animation-editor.md)
- **Rationale:** Retargeting enables sharing animation libraries across humanoid, creature, and
  mount rigs, reducing the animation authoring workload for large character rosters.
- **Verification:** Integration test: load a source and target skeleton with different proportions.
  Assign bone pairs for all major joints. Play an animation on the source and verify the target
  plays a retargeted version with correct joint mapping. Adjust limb length ratios and verify the
  retargeted animation updates in the preview. Verify foot contact positions are preserved within
  a configurable tolerance.
