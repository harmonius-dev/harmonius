# 9.1 — Skeletal Animation

## GPU Skinning

### F-9.1.1 GPU Compute Skinning

Transforms vertices by weighted bone matrices on the GPU using compute shaders. Supports both linear
blend skinning and dual-quaternion skinning to eliminate candy-wrapping artifacts at twist joints.
Bone palettes are uploaded as world-space matrices each frame.

- **Requirements:** R-9.1.1
- **Dependencies:** None
- **Platform notes:** Bone count per skeleton scales per tier: mobile 32-64, Switch 96, desktop
  256+. Dual-quaternion skinning on desktop; linear blend on mobile.

## Animation Clips (GPU Keyframe Evaluation)

### F-9.1.2 GPU Keyframe Evaluation

Evaluates animation curves entirely on the GPU using Hermite interpolation. Per-joint translation,
rotation, and scale tracks are sampled at arbitrary time offsets in a single compute dispatch.
Supports looped, clamped, and ping-pong playback modes.

- **Requirements:** R-9.1.2
- **Dependencies:** F-9.1.1
- **Platform notes:** GPU keyframe evaluation on all tiers. Track count per clip limited by skeleton
  bone budget (see F-9.1.1).

## Animation Blending

### F-9.1.3 Animation Blending (Linear and Cubic)

Blends multiple animation clips using linear interpolation or cubic Hermite curves to produce smooth
transitions between poses. Blend weights are computed CPU-side by the state machine and uploaded as
blend descriptors. Supports blending up to 8 simultaneous clips per skeleton.

- **Requirements:** R-9.1.3
- **Dependencies:** F-9.1.2
- **Platform notes:** Simultaneous blend clip count scales per tier: mobile 4, Switch 6, desktop 8.
  Cubic blending falls back to linear on mobile.

## Animation Layers

### F-9.1.4 Animation Layers and Additive Blending

Applies animations on separate layers with per-bone masks, enabling upper-body combat animations
overlaid on lower-body locomotion. Additive layers encode pose deltas relative to a reference pose,
allowing hit reactions, breathing, and weapon sway to composite on top of any base animation without
authoring per-combination clips.

- **Requirements:** R-9.1.4
- **Dependencies:** F-9.1.3
- **Platform notes:** Active animation layer count scales per tier: mobile 2, Switch 3, desktop 4+.
  Additive layers may be skipped on mobile under budget pressure.

## Instanced Skeletal Animation

### F-9.1.5 Instanced Skeletal Animation

Batch-evaluates animation for thousands of skeleton instances in a single compute dispatch using an
arena buffer. Enables MMO-scale crowds, armies, and city NPCs without per-instance CPU overhead.
Instances sharing the same animation clip are grouped for optimal GPU occupancy.

- **Requirements:** R-9.1.5
- **Dependencies:** F-9.1.1, F-9.1.2
- **Platform notes:** Active animation instances scale per tier: mobile 20-50, Switch 50, desktop
  500+. Distant instances use reduced bone sets or VAT playback.

## Root Motion

### F-9.1.6 Root Motion Extraction

Extracts translation and rotation deltas from the root bone of animation clips and applies them to
the character's world transform via the physics system. Separates authored locomotion from gameplay
movement, enabling animation-driven traversal for dodges, vaults, and mount animations while
maintaining capsule-based collision.

- **Requirements:** R-9.1.6
- **Dependencies:** F-9.1.2
- **Platform notes:** Root motion extraction is lightweight on all platforms. No platform-specific
  scaling required.

## Animation Compression

### F-9.1.7 Animation Compression

Compresses animation clip data using variable-rate quantization per track: rotation tracks use
smallest-three quaternion encoding, translation and scale tracks use range-reduced fixed-point
values. Achieves 10:1+ compression ratios on typical humanoid clips, critical for streaming
thousands of unique emote, combat, and mount animations in an MMO world.

- **Requirements:** R-9.1.7
- **Dependencies:** F-9.1.2
- **Platform notes:** Compression ratios identical across platforms. Mobile may use more aggressive
  quantization for additional memory savings.

## Animation Retargeting

### F-9.1.8 Animation Retargeting

Play animation clips authored on one skeleton on a different skeleton with different bone counts,
proportions, and naming conventions. Retargeting uses a shared canonical pose (T-pose or A-pose) to
establish bone correspondence between source and target skeletons. Per-bone retargeting modes:
direct copy (fingers), scaled translation (root, pelvis), rotation only (spine, limbs). A
retargeting asset defines the bone mapping and per-bone mode, authored visually in the animation
editor. Supports cross-species retargeting (human mocap on a fantasy creature) with manual bone
remapping.

- **Requirements:** R-9.1.8
- **Dependencies:** F-9.1.1 (GPU Skinning), F-9.1.6 (Root Motion)
- **Platform notes:** Retargeting runs on all tiers. Target skeleton bone count limited by per-tier
  budget (see F-9.1.1).

## Animation Events

### F-9.1.9 Animation Events and Notifies

Named events embedded at specific frames within animation clips that fire callbacks during playback.
Events are authored visually on the timeline in the animation editor — designers place markers for
footstep sounds, hit detection windows, VFX spawn points, weapon trail start/end, and gameplay
triggers. Events flow through the ECS observer system (F-1.1.30) as typed event components. Supports
branching events (fire only if a condition is met), window events (active for a frame range rather
than a single frame), and montage-scoped events that only fire during montage playback.

- **Requirements:** R-9.1.9
- **Dependencies:** F-9.1.1, F-1.1.30 (Observers), F-9.4.7 (Montages)
- **Platform notes:** Event dispatch is lightweight on all platforms. No platform-specific scaling
  required.

## Animation Level of Detail

### F-9.1.10 Animation Level of Detail

Reduce animation processing cost for distant or occluded entities. LOD tiers: full skeletal
evaluation (close), reduced bone set with fewer IK passes (medium), bone-count-halved skeleton with
lower update rate (far), vertex animation texture playback (crowd distance). LOD transitions use the
shared spatial index distance (F-1.9.1) and blend smoothly over configurable transition ranges to
avoid popping. Per-entity LOD bias allows hero characters to maintain full fidelity regardless of
distance. The system automatically manages the LOD population budget to maintain target frame time.

- **Requirements:** R-9.1.10
- **Dependencies:** F-9.1.1, F-9.1.5 (Instanced Skeletal), F-1.9.1 (Shared BVH)
- **Platform notes:** LOD tier thresholds more aggressive on mobile — earlier transitions to reduced
  bone sets, lower update rates, and VAT playback at shorter distances.
