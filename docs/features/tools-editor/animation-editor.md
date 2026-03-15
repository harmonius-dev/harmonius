# 15.4 — Animation Editor

## Timeline and Curves

### F-15.4.1 Animation Timeline

Provides a multi-track timeline for viewing and editing animation clips. Tracks display
keyframes for translation, rotation, scale, and custom properties with color-coded
differentiation. Supports playback controls, scrubbing, frame stepping, looping, and playback
speed adjustment. Keyframes can be moved, copied, and snapped to frame boundaries.

- **Requirements:** R-15.4.1
- **Dependencies:** F-9.1.2, F-15.1.3
- **Platform notes:** Desktop only. Not available on mobile or console runtime.

### F-15.4.2 Curve Editor

Edits animation curves with Bezier and Hermite tangent modes. Provides per-channel curve
display with selectable visibility, tangent manipulation handles, auto-tangent computation,
and curve presets (ease-in, ease-out, linear, stepped). Supports box selection and multi-curve
editing for batch adjustments across many bones simultaneously.

- **Requirements:** R-15.4.2
- **Dependencies:** F-15.4.1
- **Platform notes:** Desktop only. Not available on mobile or console runtime.

## Skeleton and Preview

### F-15.4.3 Skeleton Viewer

Renders the skeleton hierarchy as a 3D bone visualization overlaid on the mesh. Bones are
selectable, and selected bones highlight their children, associated constraints, and IK chains.
Supports toggling between bone display modes (octahedral, stick, axes) and overlaying multiple
skeletons for comparison.

- **Requirements:** R-15.4.3
- **Dependencies:** F-9.1.1, F-15.1.4
- **Platform notes:** Desktop only. Not available on mobile or console runtime.

### F-15.4.4 Animation Preview

Plays back animations on a character mesh in a dedicated preview viewport with configurable
ground plane, lighting, and camera orbit. Supports previewing blend results, layered animations,
and root motion trajectories. The preview can render at game quality or with debug overlays
such as velocity vectors, contact points, and bone trails.

- **Requirements:** R-15.4.4
- **Dependencies:** F-15.4.1, F-9.1.1, F-15.1.2
- **Platform notes:** Desktop only. Not available on mobile or console runtime.

## Blend and State Authoring

### F-15.4.5 Blend Space Editor

Provides a 1D or 2D blend space editor where animation clips are placed at parameter-space
positions (e.g., speed vs. direction). The editor shows interpolation regions, allows
drag-based repositioning of samples, and previews blended output in real time as the user
moves a crosshair through the parameter space.

- **Requirements:** R-15.4.5
- **Dependencies:** F-9.1.3, F-15.4.1
- **Platform notes:** Desktop only. Not available on mobile or console runtime.

### F-15.4.6 State Machine Editor

Visual node-graph editor for authoring animation state machines. States represent animation
clips or blend spaces, and transitions define blend durations, conditions, and interruption
rules. The editor highlights the active state during preview playback and shows transition
evaluation in real time, enabling animators to debug complex locomotion graphs.

- **Requirements:** R-15.4.6
- **Dependencies:** F-9.4.1, F-15.4.5
- **Platform notes:** Desktop only. Not available on mobile or console runtime.

### F-15.4.7 Retargeting Setup

Configures skeleton-to-skeleton retargeting mappings so animations authored for one character
rig can drive a differently proportioned rig. The editor provides a side-by-side view of
source and target skeletons with bone-pair assignment, limb length ratio adjustment, and
real-time retargeted preview. Enables sharing animation libraries across humanoid, creature,
and mount rigs in an MMO roster.

- **Requirements:** R-15.4.7
- **Dependencies:** F-9.1.1, F-15.4.3
- **Platform notes:** Desktop only. Not available on mobile or console runtime.
