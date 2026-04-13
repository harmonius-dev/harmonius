# R-15.4 -- Animation Editor Requirements

## Requirements

1. **R-15.4.1** — The engine **SHALL** provide a multi-track animation timeline with keyframe
   editing, playback controls, scrubbing, and color-coded channels for transform and custom
   properties.
   - **Rationale:** A timeline is the fundamental tool for reviewing and editing keyframed animation
     data.
   - **Verification:** Create keyframes on three channels, scrub the timeline, and verify the
     preview updates in real time.

2. **R-15.4.2** — The engine **SHALL** provide a curve editor with Bezier and Hermite tangent modes,
   auto-tangent computation, preset curves, and multi-curve batch editing.
   - **Rationale:** Precise curve control determines animation quality and timing feel.
   - **Verification:** Apply an ease-in preset, verify the tangent handles match the expected curve
     shape.

3. **R-15.4.3** — The engine **SHALL** render a skeleton viewer with selectable bones, constraint
   visualization, IK chain highlights, and multiple display modes.
   - **Rationale:** Rig inspection is required for debugging skeletal issues and retargeting setup.
   - **Verification:** Select a bone and verify its IK chain and constraints highlight in the
     viewer.

4. **R-15.4.4** — The engine **SHALL** provide an animation preview viewport with configurable
   ground plane, lighting, orbit camera, and debug overlays for velocity, contacts, and bone trails.
   - **Rationale:** Preview with debug data enables diagnosing foot sliding and animation-gameplay
     desync.
   - **Verification:** Play an animation and verify velocity vectors render on each bone in the
     overlay.

5. **R-15.4.5** — The engine **SHALL** provide a 1D/2D blend space editor with real-time blended
   output preview and drag-based sample repositioning.
   - **Rationale:** Blend spaces control locomotion quality; real-time preview enables fast tuning.
   - **Verification:** Move the blend crosshair between two samples and verify the preview shows
     interpolated output.

6. **R-15.4.6** — The engine **SHALL** provide a visual state machine editor with state nodes,
   transition edges with condition expressions, and active-state highlighting during preview.
   - **Rationale:** State machines are the primary way designers author animation logic without
     code.
   - **Verification:** Create a two-state machine, trigger a transition, and verify the editor
     highlights the active state.

7. **R-15.4.7** — The engine **SHALL** provide a retargeting setup editor with side-by-side skeleton
   comparison, bone-pair assignment, limb length ratio adjustment, and real-time retargeted preview.
   - **Rationale:** Retargeting enables sharing animation libraries across differently proportioned
     rigs.
   - **Verification:** Retarget a walk animation to a differently proportioned skeleton and verify
     foot contact positions are correct.
