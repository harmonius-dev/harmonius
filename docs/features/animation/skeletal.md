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

## Skeleton Types and Rigs

| ID       | Feature                                 |
|----------|-----------------------------------------|
| F-9.1.11 | Canonical Humanoid Rig                  |
| F-9.1.12 | Quadruped, Bird, and Custom Rigs        |
| F-9.1.13 | Named Bone Chains                       |
| F-9.1.14 | Complex Mechanical and Segmented Rigs   |

1. **F-9.1.11** — A canonical `HumanoidRig` defines standard named slots (`hips`, `spine`, `chest`,
   `neck`, `head`, `left_arm`/`right_arm` limbs, `left_leg`/`right_leg` limbs, optional finger
   chains) used as the intermediate representation for automatic retargeting between any two
   humanoid skeletons. The rig mapping is authored once per skeleton asset and stored alongside it.
   Source animation is converted through the canonical rig to reach the target skeleton without
   manual bone correspondence.
   - **Deps:** F-9.1.8 (Retargeting)
2. **F-9.1.12** — `SkeletonType` supports `Quadruped`, `Bird`, and `Custom(Vec<NamedBoneChain>)`
   variants in addition to `Humanoid`. Quadruped and bird rigs define anatomically plausible slots
   (pelvis, spine, neck, head, four limbs, tail, wings, optional ears). Retargeting within the same
   type is automatic via canonical rig lookup; retargeting across types requires a user-authored
   retarget map. `Custom` handles any topology through user-defined named chains.
   - **Deps:** F-9.1.11, F-9.1.13
3. **F-9.1.13** — Named bone chains group sequential bones into semantic units (`Spine`, `Limb`,
   `Digits`, `Tail`, `Custom`) used for per-chain IK targets, bone masks, LOD pruning, and ragdoll
   scoping. Chains are assigned at skeleton import time or manually in the editor. `BoneMask` can be
   constructed from a chain set: `BoneMask::from_chains(&[spine, left_arm])`, composing blend masks
   from semantic groups rather than raw bone indices.
   - **Deps:** F-9.1.11
4. **F-9.1.14** — Arbitrary topologies (8-leg spider rigs, multi-wheel vehicle rigs, segmented
   centipede rigs, mechanical piston/gear rigs) are supported via the `Custom` skeleton variant plus
   specialized joint descriptors. Spider rigs run 8 simultaneous IK solves with alternating gait
   groups; vehicle rigs rotate wheels from drivetrain speed and drive suspension bones from physics
   spring compression; segmented rigs propagate a sinusoidal body wave along chained segments;
   mechanical rigs expose revolute, prismatic, and piston joint types with hard limits.
   - **Deps:** F-9.1.13, F-9.1.1

## Motion Capture and FABRIK

| ID       | Feature                             |
|----------|-------------------------------------|
| F-9.1.15 | Motion Capture Ingest               |
| F-9.1.16 | FABRIK Chain Solver                 |

1. **F-9.1.15** — Accept live and recorded mocap data via the input system, flowing as `MocapEvent`
   components into the animation pipeline. Supports streaming body capture (OptiTrack, Vicon, Live
   Link equivalent), ARKit-style 52-blendshape face capture from iOS devices, and offline FBX/BVH
   import for recorded takes. Incoming poses are retargeted to the canonical humanoid rig (F-9.1.11)
   and feed the animation layer stack as a high-priority override layer.
   - **Deps:** F-9.1.8 (Retargeting), F-9.1.11 (Humanoid Rig)
2. **F-9.1.16** — FABRIK (Forward And Backward Reaching Inverse Kinematics) solves multi-bone IK
   chains with configurable iteration count for spines, tails, tentacles, and other organic chains
   where analytical two-bone IK is insufficient. Runs on GPU for batched chain evaluation across
   many entities. Per-chain angle limits and pole targets constrain the solution. FABRIK is the
   default solver for chains longer than two bones.
   - **Deps:** F-9.1.13 (Named Bone Chains)

## Ragdoll and Animation Blending

| ID       | Feature                                 |
|----------|-----------------------------------------|
| F-9.1.17 | Animated-to-Ragdoll and Get-Up Blending |
| F-9.1.18 | Partial Ragdoll with Per-Chain Masks    |

1. **F-9.1.17** — On trigger, ragdoll bodies are spawned at the current world transform of each
   bone, joint constraints are installed from the `RagdollDef` asset (F-4.3.5), and velocity is
   inherited from the previous frame's animation delta. For get-up animations, a
   `RagdollBlendWeight` component lerps each bone between the physics pose and the animation pose,
   smoothly transitioning weight from 1.0 to 0.0 over a configurable duration before despawning the
   ragdoll bodies.
   - **Deps:** F-4.3.5 (Ragdoll Configuration), F-9.1.3 (Animation Blending)
2. **F-9.1.18** — Partial ragdoll drives a subset of bones from physics while the remainder stays
   animated — e.g., spine and arms ragdoll while legs continue walking during a hit reaction. A bone
   mask partitions which chains are physics-driven; a joint at the partition point constrains the
   ragdoll region to the animated region so the two halves remain coherent.
   - **Deps:** F-9.1.17, F-9.1.13 (Named Bone Chains), F-9.1.4 (Animation Layers)

## Skinning Extensions

| ID       | Feature                                 |
|----------|-----------------------------------------|
| F-9.1.19 | Variable Bone Influence Count Per Mesh  |
| F-9.1.20 | Bone Sockets for Entity Attachment      |
| F-9.1.21 | Modular Character Customization         |

1. **F-9.1.19** — Support 1, 2, 4, or 8 bone influences per vertex, selectable per mesh. Rigid props
   and weapons use 1 influence, simple characters use 2, standard characters use 4 (default), and
   faces, muscle simulation, and cloth-bone hybrids use 8. The mesh shader reads the influence count
   from meshlet metadata and unrolls accordingly. The asset pipeline normalizes weights and reduces
   influence count (8→4→2) for LOD meshes to save GPU bandwidth.
   - **Deps:** F-9.1.1 (GPU Compute Skinning)
2. **F-9.1.20** — Bone sockets are named attachment points on a skeleton bone carrying a local
   offset transform. Entities with an `AttachedToSocket` component inherit the bone's world
   transform times the socket offset each frame via a `SocketTransformSystem` that runs after the
   bone palette is computed. Multiple sockets can reference the same bone with different offsets.
   Used for weapons, helmets, backpacks, muzzle VFX, and eye glow effects.
   - **Deps:** F-9.1.1 (GPU Compute Skinning)
3. **F-9.1.21** — Modular character customization via multiple mesh entities sharing one
   `SkeletonRef`. Body parts (head, torso, arms, legs, hair, cape, accessories) are independent mesh
   assets all authored against the same canonical skeleton and bind pose. Each part weights only the
   bones it uses. Parts can be merged at load time into a single combined mesh for draw call
   reduction and re-merged when equipment changes. Body proportions are driven by additive morph
   targets applied before skinning.
   - **Deps:** F-9.1.1 (GPU Compute Skinning), F-9.1.11 (Canonical Humanoid Rig)
