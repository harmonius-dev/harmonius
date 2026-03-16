# R-15.4 -- Animation Editor Requirements

## Timeline

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.4.1 | The editor **SHALL** provide a multi-track timeline for animation clips with color-coded tracks per property type, playback controls with scrubbing and frame stepping, keyframe move/copy, and frame boundary snapping. | [F-15.4.1](../../features/tools-editor/animation-editor.md) | Frame-accurate keyframe editing is essential for animation authoring. | Benchmark: scrub a 300-bone animation and verify the viewport stays above 30 FPS. |
| R-15.4.2 | The editor **SHALL** provide a curve editor with Bezier and Hermite tangent modes, per-channel visibility toggles, tangent manipulation handles, auto-tangent computation, curve presets (ease-in, ease-out, linear, stepped), and box selection for multi-curve batch editing. | [F-15.4.2](../../features/tools-editor/animation-editor.md) | Precise interpolation control is critical for polished animation quality. | Unit test: apply each curve preset and verify the interpolation profile matches the expected mathematical function within epsilon. |

## Skeleton and Preview

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.4.3 | The editor **SHALL** display a 3D bone visualization overlaid on the mesh with selectable bones that highlight children and IK chains, togglable display modes (octahedral, stick, axes), and multi-skeleton overlay for comparison. | [F-15.4.3](../../features/tools-editor/animation-editor.md) | Skeleton visualization is required for rigging verification, IK setup, and retargeting. | Unit test: select a bone and verify the correct child chain is highlighted. |
| R-15.4.4 | The editor **SHALL** provide a dedicated animation preview viewport with configurable ground plane, adjustable lighting, camera orbit, blend result preview, root motion trajectory visualization, and debug overlays for velocity vectors and bone trails. | [F-15.4.4](../../features/tools-editor/animation-editor.md) | Evaluating animation in isolation from the scene with debug overlays accelerates authoring. | Unit test: play a root motion animation and verify the trajectory visualization matches expected travel distance within tolerance. |

## Blend and State Authoring

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.4.5 | The editor **SHALL** provide 1D and 2D blend space editors with drag-based sample repositioning, interpolation region visualization, and real-time blended output preview via a parameter-space crosshair. | [F-15.4.5](../../features/tools-editor/animation-editor.md) | Visual blend space authoring is required for no-code locomotion tuning. | Unit test: position the crosshair at center of a 2D blend space and verify equal blend of corner samples. |
| R-15.4.6 | The editor **SHALL** provide a visual node-graph editor for animation state machines with states representing clips or blend spaces, configurable transition blend durations and conditions, active-state highlighting during preview, and interruption rules for mid-transition overrides. | [F-15.4.6](../../features/tools-editor/animation-editor.md) | No-code animation state machine authoring is essential for the no-code engine constraint. | Unit test: set a transition condition, trigger it, and verify the state change occurs at the correct parameter value. |

## Retargeting

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.4.7 | The editor **SHALL** provide a side-by-side skeleton comparison view with bone-pair assignment, limb length ratio adjustment, and real-time retargeted animation preview across humanoid, creature, and mount rigs. | [F-15.4.7](../../features/tools-editor/animation-editor.md) | Sharing animation libraries across rigs saves production time; real-time preview catches issues before commit. | Unit test: retarget a walk animation and verify foot contact positions are preserved within configurable tolerance. |

---

## User Story Traceability

User stories for this domain are maintained in
[user-stories/tools-editor/animation-editor.md](../../user-stories/tools-editor/animation-editor.md).
Requirements in this document are derived from those user stories.
