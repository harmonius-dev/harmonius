# 9.1 — Skeletal Animation

## GPU Skinning

| ID      | Feature              |
|---------|----------------------|
| F-9.1.1 | GPU Compute Skinning |

1. **F-9.1.1** — Transforms vertices by weighted bone matrices on the GPU using compute shaders.
   Supports both linear blend skinning and dual-quaternion skinning to eliminate candy-wrapping
   artifacts at twist joints. Bone palettes are uploaded as world-space matrices each frame.
   - **Platform:** Bone count per skeleton scales per tier: mobile 32-64, Switch 96, desktop 256+.
     Dual-quaternion skinning on desktop; linear blend on mobile.

## Animation Clips (GPU Keyframe Evaluation)

| ID      | Feature                 |
|---------|-------------------------|
| F-9.1.2 | GPU Keyframe Evaluation |

1. **F-9.1.2** — Evaluates animation curves entirely on the GPU using Hermite interpolation.
   Per-joint translation, rotation, and scale tracks are sampled at arbitrary time offsets in a
   single compute dispatch. Supports looped, clamped, and ping-pong playback modes.
   - **Deps:** F-9.1.1
   - **Platform:** GPU keyframe evaluation on all tiers. Track count per clip limited by skeleton
     bone budget (see F-9.1.1).

## Animation Blending

| ID      | Feature                               |
|---------|---------------------------------------|
| F-9.1.3 | Animation Blending (Linear and Cubic) |

1. **F-9.1.3** — Blends multiple animation clips using linear interpolation or cubic Hermite curves
   to produce smooth transitions between poses. Blend weights are computed CPU-side by the state
   machine and uploaded as blend descriptors. Supports blending up to 8 simultaneous clips per
   skeleton.
   - **Deps:** F-9.1.2
   - **Platform:** Simultaneous blend clip count scales per tier: mobile 4, Switch 6, desktop 8.
     Cubic blending falls back to linear on mobile.

## Animation Layers

| ID      | Feature                                |
|---------|----------------------------------------|
| F-9.1.4 | Animation Layers and Additive Blending |

1. **F-9.1.4** — Applies animations on separate layers with per-bone masks, enabling upper-body
   combat overlaid on lower-body locomotion. Additive layers encode pose deltas relative to a
   reference pose, allowing hit reactions, breathing, and weapon sway to composite on top of any
   base animation.
   - **Deps:** F-9.1.3
   - **Platform:** Active animation layer count scales per tier: mobile 2, Switch 3, desktop 4+.
     Additive layers may be skipped on mobile under budget pressure.

## Instanced Skeletal Animation

| ID      | Feature                      |
|---------|------------------------------|
| F-9.1.5 | Instanced Skeletal Animation |

1. **F-9.1.5** — Batch-evaluates animation for thousands of skeleton instances in a single compute
   dispatch using an arena buffer. Enables MMO-scale crowds, armies, and city NPCs without
   per-instance CPU overhead. Instances sharing the same clip are grouped for optimal GPU occupancy.
   - **Deps:** F-9.1.1, F-9.1.2
   - **Platform:** Active instances scale per tier: mobile 20-50, Switch 50, desktop 500+. Distant
     instances use reduced bone sets or VAT playback.

## Root Motion

| ID      | Feature                |
|---------|------------------------|
| F-9.1.6 | Root Motion Extraction |

1. **F-9.1.6** — Extracts translation and rotation deltas from the root bone of animation clips and
   applies them to the character's world transform via the physics system. Separates authored
   locomotion from gameplay movement for dodges, vaults, and mount animations while maintaining
   capsule-based collision.
   - **Deps:** F-9.1.2
   - **Platform:** Lightweight on all platforms. No platform-specific scaling required.

## Animation Compression

| ID      | Feature               |
|---------|-----------------------|
| F-9.1.7 | Animation Compression |

1. **F-9.1.7** — Compresses animation clip data using variable-rate quantization per track: rotation
   tracks use smallest-three quaternion encoding, translation and scale tracks use range-reduced
   fixed-point values. Achieves 10:1+ compression ratios on typical humanoid clips.
   - **Deps:** F-9.1.2
   - **Platform:** Compression ratios identical across platforms. Mobile may use more aggressive
     quantization for additional memory savings.

## Animation Retargeting

| ID      | Feature               |
|---------|-----------------------|
| F-9.1.8 | Animation Retargeting |

1. **F-9.1.8** — Play animation clips authored on one skeleton on a different skeleton with
   different bone counts, proportions, and naming conventions. Uses a shared canonical pose (T-pose
   or A-pose) for bone correspondence. Per-bone modes: direct copy, scaled translation, rotation
   only. Supports cross-species retargeting with manual bone remapping.
   - **Deps:** F-9.1.1 (GPU Skinning), F-9.1.6 (Root Motion)
   - **Platform:** Runs on all tiers. Target skeleton bone count limited by per-tier budget (see
     F-9.1.1).

## Animation Events

| ID      | Feature                       |
|---------|-------------------------------|
| F-9.1.9 | Animation Events and Notifies |

1. **F-9.1.9** — Named events embedded at specific frames within animation clips that fire callbacks
   during playback. Events flow through the ECS observer system (F-1.1.30) as typed event
   components. Supports branching events, window events (active for a frame range), and
   montage-scoped events.
   - **Deps:** F-9.1.1, F-1.1.30 (Observers), F-9.4.7 (Montages)
   - **Platform:** Event dispatch is lightweight on all platforms. No platform-specific scaling
     required.

## Animation Level of Detail

| ID       | Feature                   |
|----------|---------------------------|
| F-9.1.10 | Animation Level of Detail |

1. **F-9.1.10** — Reduces animation cost for distant or occluded entities. LOD tiers: full skeletal
   evaluation (close), reduced bone set (medium), bone-count-halved with lower update rate (far),
   VAT playback (crowd distance). Transitions use the shared spatial index distance (F-1.9.1) and
   blend smoothly. Per-entity LOD bias for hero characters.
   - **Deps:** F-9.1.1, F-9.1.5 (Instanced Skeletal), F-1.9.1 (Shared BVH)
   - **Platform:** LOD thresholds more aggressive on mobile: earlier transitions to reduced bone
     sets, lower update rates, and VAT playback at shorter distances.
