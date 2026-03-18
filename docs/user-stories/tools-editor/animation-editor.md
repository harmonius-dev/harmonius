# User Stories: Animation Editor

## F-15.4.1 Animation Timeline

| ID          | Persona              | Features | Requirements |
|-------------|----------------------|----------|--------------|
| US-15.4.1.1 | artist (P-8)         |          |              |
| US-15.4.1.2 | designer (P-5)       |          |              |
| US-15.4.1.3 | tech artist (P-13)   |          |              |
| US-15.4.1.4 | engine tester (P-27) |          |              |

1. **US-15.4.1.1** — a multi-track timeline displaying keyframes for translation, rotation, scale,
   and custom properties with color-coded differentiation
   - **Acceptance:** I can view and edit all channels of an animation clip at a glance
2. **US-15.4.1.2** — playback controls, scrubbing, frame stepping, looping, and adjustable playback
   speed on the timeline
   - **Acceptance:** I can examine animation behavior at any point in the clip
3. **US-15.4.1.3** — to move, copy, and snap keyframes to frame boundaries with multi-selection
   - **Acceptance:** I can retime animations efficiently across many tracks simultaneously
4. **US-15.4.1.4** — to verify that keyframe placement and scrubbing produce frame-accurate results
   at varying playback speeds
   - **Acceptance:** animators can trust the timeline's temporal precision

## F-15.4.2 Curve Editor

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-15.4.2.1 | artist (P-8)            |          |              |
| US-15.4.2.2 | designer (P-5)          |          |              |
| US-15.4.2.3 | tech artist (P-13)      |          |              |
| US-15.4.2.4 | engine developer (P-26) |          |              |

1. **US-15.4.2.1** — to edit animation curves with Bezier and Hermite tangent modes and per-channel
   visibility
   - **Acceptance:** I can fine-tune easing and timing for each bone independently
2. **US-15.4.2.2** — curve presets (ease-in, ease-out, linear, stepped) applicable with a single
   click
   - **Acceptance:** I can quickly set common timing patterns without manually adjusting tangent
     handles
3. **US-15.4.2.3** — box selection and multi-curve editing for batch adjustments across many bones
   simultaneously
   - **Acceptance:** I can retime complex full-body animations without editing each curve
     individually
4. **US-15.4.2.4** — curve interpolation to produce smooth, mathematically correct results with no
   discontinuities at tangent boundaries
   - **Acceptance:** animation playback matches the curve editor preview exactly

## F-15.4.3 Skeleton Viewer

| ID          | Persona              | Features | Requirements |
|-------------|----------------------|----------|--------------|
| US-15.4.3.1 | artist (P-8)         |          |              |
| US-15.4.3.2 | tech artist (P-13)   |          |              |
| US-15.4.3.3 | designer (P-5)       |          |              |
| US-15.4.3.4 | engine tester (P-27) |          |              |

1. **US-15.4.3.1** — to view the skeleton hierarchy as a 3D bone visualization overlaid on the mesh
   with selectable bones that highlight children, constraints, and IK chains
   - **Acceptance:** I can understand and navigate complex rig structures visually
2. **US-15.4.3.2** — to overlay multiple skeletons for comparison
   - **Acceptance:** I can identify bone naming mismatches and hierarchy differences between source
     and target rigs
3. **US-15.4.3.3** — to toggle between bone display modes (octahedral, stick, axes)
   - **Acceptance:** I can choose the visualization that best suits my current task
4. **US-15.4.3.4** — to verify that bone selection correctly highlights the full child chain and
   associated constraints
   - **Acceptance:** animators always see the complete scope of their edits

## F-15.4.4 Animation Preview

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-15.4.4.1 | artist (P-8)            |          |              |
| US-15.4.4.2 | designer (P-5)          |          |              |
| US-15.4.4.3 | tech artist (P-13)      |          |              |
| US-15.4.4.4 | creative director (P-2) |          |              |

1. **US-15.4.4.1** — to play back animations on a character mesh in a dedicated preview viewport
   with configurable ground plane, lighting, and camera orbit
   - **Acceptance:** I can evaluate animation quality in a clean preview environment
2. **US-15.4.4.2** — to preview blend results, layered animations, and root motion trajectories
   - **Acceptance:** I can verify that animations blend smoothly before integrating them into the
     state machine
3. **US-15.4.4.3** — debug overlays showing velocity vectors, contact points, and bone trails in the
   preview
   - **Acceptance:** I can diagnose foot sliding, contact misalignment, and motion artifacts
4. **US-15.4.4.4** — the preview to render at full game quality
   - **Acceptance:** I can evaluate animation fidelity during review sessions without launching the
     game

## F-15.4.5 Blend Space Editor

| ID          | Persona              | Features | Requirements |
|-------------|----------------------|----------|--------------|
| US-15.4.5.1 | designer (P-5)       |          |              |
| US-15.4.5.2 | artist (P-8)         |          |              |
| US-15.4.5.3 | tech artist (P-13)   |          |              |
| US-15.4.5.4 | engine tester (P-27) |          |              |

1. **US-15.4.5.1** — a 1D or 2D blend space editor where animation clips are placed at
   parameter-space positions (e.g., speed vs. direction)
   - **Acceptance:** I can define smooth transitions between locomotion states
2. **US-15.4.5.2** — drag-based repositioning of samples in the blend space with visible
   interpolation regions
   - **Acceptance:** I can adjust blend weights visually and see the impact immediately
3. **US-15.4.5.3** — to preview the blended animation output in real time as I move a crosshair
   through the parameter space
   - **Acceptance:** I can identify dead zones and unnatural transitions
4. **US-15.4.5.4** — to verify that blend space interpolation produces smooth, artifact-free results
   at all parameter positions
   - **Acceptance:** locomotion blending works correctly across the full range of player input

## F-15.4.6 State Machine Editor

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-15.4.6.1 | designer (P-5)          |          |              |
| US-15.4.6.2 | artist (P-8)            |          |              |
| US-15.4.6.3 | tech artist (P-13)      |          |              |
| US-15.4.6.4 | engine developer (P-26) |          |              |

1. **US-15.4.6.1** — a visual node-graph editor for authoring animation state machines with states,
   transitions, blend durations, and conditions
   - **Acceptance:** I can define complex locomotion logic without writing code
2. **US-15.4.6.2** — the editor to highlight the active state during preview playback and show
   transition evaluation in real time
   - **Acceptance:** I can debug which state is active and why specific transitions fire
3. **US-15.4.6.3** — to set interruption rules on transitions
   - **Acceptance:** high-priority actions (dodge, hit reaction) can interrupt lower-priority
     animations at the correct blend point
4. **US-15.4.6.4** — the state machine editor to validate that all states are reachable and no
   transitions create infinite loops
   - **Acceptance:** authored state machines are always well-formed

## F-15.4.7 Retargeting Setup

| ID          | Persona              | Features | Requirements |
|-------------|----------------------|----------|--------------|
| US-15.4.7.1 | artist (P-8)         |          |              |
| US-15.4.7.2 | designer (P-5)       |          |              |
| US-15.4.7.3 | tech artist (P-13)   |          |              |
| US-15.4.7.4 | engine tester (P-27) |          |              |

1. **US-15.4.7.1** — to configure skeleton-to-skeleton retargeting mappings with bone-pair
   assignment and limb length ratio adjustment
   - **Acceptance:** animations from one character rig can drive a differently proportioned rig
2. **US-15.4.7.2** — a side-by-side view of source and target skeletons with real-time retargeted
   preview
   - **Acceptance:** I can verify that shared animations look correct on each character type before
     shipping
3. **US-15.4.7.3** — retargeting to enable sharing animation libraries across humanoid, creature,
   and mount rigs
   - **Acceptance:** a single animation investment covers the full character roster
4. **US-15.4.7.4** — to verify that retargeted animations preserve contacts, avoid
   self-intersection, and maintain proportional correctness
   - **Acceptance:** retargeting produces production-quality results without manual fixup
