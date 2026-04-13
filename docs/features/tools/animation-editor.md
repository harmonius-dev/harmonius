# 15.4 — Animation Editor

## Timeline and Curves

| ID | Feature |
| ---------- | -------------------- |
| F-15.4.1 | Animation Timeline |
| F-15.4.2 | Curve Editor |

1. **F-15.4.1** — Provides a multi-track timeline for viewing and editing animation clips. Tracks
   display keyframes for translation, rotation, scale, and custom properties with color-coded
   differentiation. Supports playback controls, scrubbing, frame stepping, looping, and playback
   speed adjustment. Keyframes can be moved, copied, and snapped to frame boundaries.
   - **Deps:** F-9.1.2, F-15.1.3
   - **Platform:** Desktop only. Not available on mobile or console runtime.
2. **F-15.4.2** — Edits animation curves with Bezier and Hermite tangent modes. Provides per-channel
   curve display with selectable visibility, tangent manipulation handles, auto-tangent computation,
   and curve presets (ease-in, ease-out, linear, stepped). Supports box selection and multi-curve
   editing for batch adjustments across many bones simultaneously.
   - **Deps:** F-15.4.1
   - **Platform:** Desktop only. Not available on mobile or console runtime.

## Skeleton and Preview

| ID | Feature |
| ---------- | ------------------- |
| F-15.4.3 | Skeleton Viewer |
| F-15.4.4 | Animation Preview |

1. **F-15.4.3** — Renders the skeleton hierarchy as a 3D bone visualization overlaid on the mesh.
   Bones are selectable, and selected bones highlight their children, associated constraints, and IK
   chains. Supports toggling between bone display modes (octahedral, stick, axes) and overlaying
   multiple skeletons for comparison.
   - **Deps:** F-9.1.1, F-15.1.4
   - **Platform:** Desktop only. Not available on mobile or console runtime.
2. **F-15.4.4** — Plays back animations on a character mesh in a dedicated preview viewport with
   configurable ground plane, lighting, and camera orbit. Supports previewing blend results, layered
   animations, and root motion trajectories. The preview can render at game quality or with debug
   overlays such as velocity vectors, contact points, and bone trails.
   - **Deps:** F-15.4.1, F-9.1.1, F-15.1.2
   - **Platform:** Desktop only. Not available on mobile or console runtime.

## Blend and State Authoring

| ID | Feature |
| ---------- | ---------------------- |
| F-15.4.5 | Blend Space Editor |
| F-15.4.6 | State Machine Editor |
| F-15.4.7 | Retargeting Setup |

1. **F-15.4.5** — Provides a 1D or 2D blend space editor where animation clips are placed at
   parameter-space positions (e.g., speed vs. direction). The editor shows interpolation regions,
   allows drag-based repositioning of samples, and previews blended output in real time as the user
   moves a crosshair through the parameter space.
   - **Deps:** F-9.1.3, F-15.4.1
   - **Platform:** Desktop only. Not available on mobile or console runtime.
2. **F-15.4.6** — Visual node-graph editor for authoring animation state machines. States represent
   animation clips or blend spaces, and transitions define blend durations, conditions, and
   interruption rules. The editor highlights the active state during preview playback and shows
   transition evaluation in real time, enabling animators to debug complex locomotion graphs.
   - **Deps:** F-9.4.1, F-15.4.5
   - **Platform:** Desktop only. Not available on mobile or console runtime.
3. **F-15.4.7** — Configures skeleton-to-skeleton retargeting mappings so animations authored for
   one character rig can drive a differently proportioned rig. The editor provides a side-by-side
   view of source and target skeletons with bone-pair assignment, limb length ratio adjustment, and
   real-time retargeted preview. Enables sharing animation libraries across humanoid, creature, and
   mount rigs in an MMO roster.
   - **Deps:** F-9.1.1, F-15.4.3
   - **Platform:** Desktop only. Not available on mobile or console runtime.
