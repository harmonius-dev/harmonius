# R-9.1 -- Skeletal Animation Requirements

## GPU Skinning and Evaluation

1. **R-9.1.1** -- The engine **SHALL** transform vertices by weighted bone matrices on the GPU using
   compute shaders, supporting both linear blend and dual-quaternion skinning.
   - **Rationale:** GPU compute skinning offloads per-vertex deformation and enables skinning
     thousands of characters without CPU bottlenecks.
   - **Verification:** Skin a mesh with a 180-degree forearm twist. Verify dual-quaternion mode
     produces no candy-wrapping. Confirm all bone matrices are read from a single GPU buffer.

2. **R-9.1.2** -- The engine **SHALL** evaluate animation curves entirely on the GPU using Hermite
   interpolation for translation, rotation, and scale tracks in a single compute dispatch.
   - **Rationale:** GPU-side keyframe evaluation eliminates CPU-GPU transfer for pose computation
     and scales to thousands of animated entities.
   - **Verification:** Sample a clip at frame 15.5 and verify positional error is below 0.001 units
     vs. a CPU reference. Verify loop, clamp, and ping-pong modes wrap correctly.

3. **R-9.1.3** -- The engine **SHALL** blend up to 8 simultaneous animation clips per skeleton using
   linear interpolation or cubic Hermite curves.
   - **Rationale:** Multi-clip blending enables smooth state transitions and layered animation
     without unique authored clips for every combination.
   - **Verification:** Blend 8 clips with equal weights and verify the pose matches a CPU reference
     within 0.001 units per joint.

## Layers and Instancing

1. **R-9.1.4** -- The engine **SHALL** apply animations on separate layers with per-bone masks and
   support additive layers encoding pose deltas relative to a reference pose.
   - **Rationale:** Layered animation separates upper/lower body and enables composable reactions
     without combinatorial clip explosion.
   - **Verification:** Apply upper-body combat with a bone mask over lower-body locomotion. Verify
     masked bones follow combat exclusively. Apply additive breathing and verify the delta sums
     correctly.

2. **R-9.1.5** -- The engine **SHALL** batch-evaluate animation for at least 1000 skeleton instances
   in a single compute dispatch using an arena buffer.
   - **Rationale:** Instanced evaluation eliminates per-instance CPU overhead for MMO-scale crowds
     and armies.
   - **Verification:** Spawn 1000 instances and verify correct poses by sampling 50 against
     single-instance evaluation. Verify dispatch count is 1 and frame time is under 2 ms.

## Root Motion, Compression, and Retargeting

1. **R-9.1.6** -- The engine **SHALL** extract root bone translation and rotation deltas from clips
   and apply them to the character's world transform via the physics system.
   - **Rationale:** Root motion separates authored locomotion from gameplay movement for dodges,
     vaults, and mounts while maintaining collision.
   - **Verification:** Play a dodge clip and verify the character moves by the authored distance
     while respecting collision bounds.

2. **R-9.1.7** -- The engine **SHALL** compress animation clips using smallest-three quaternion
   encoding and range-reduced fixed-point values, achieving 10:1+ compression on humanoid clips.
   - **Rationale:** Compressed clips reduce memory and streaming bandwidth with negligible visual
     quality loss.
   - **Verification:** Compress and decompress a reference clip. Verify positional error per joint
     is below the perceptual threshold. Measure compression ratio.

3. **R-9.1.8** -- The engine **SHALL** retarget animation clips between skeletons with different
   bone counts and proportions using a shared canonical pose.
   - **Rationale:** Animation library reuse across character types eliminates redundant authoring.
   - **Verification:** Retarget a walk cycle from a tall skeleton to a short one. Verify foot
     contact timing is preserved and bone correspondence is correct.

## Events and LOD

1. **R-9.1.9** -- The engine **SHALL** fire named events embedded at specific frames during clip
   playback through the ECS observer system, supporting branching and window events.
   - **Rationale:** Animation events synchronize gameplay effects like footstep sounds and VFX with
     animation playback.
   - **Verification:** Embed an event at frame 15 and verify it fires when playback crosses that
     frame. Verify window events remain active for the specified range.

2. **R-9.1.10** -- The engine **SHALL** reduce animation cost for distant entities through LOD tiers
   that decrease bone count and update rate, with per-entity LOD bias.
   - **Rationale:** Animation LOD ensures cost scales with screen importance, critical for MMO-scale
     scenes.
   - **Verification:** Place a character at LOD 2 distance and verify reduced bone set. Verify a
     hero character with LOD bias maintains full quality at the same distance.

## Skeleton Types and Rigs

1. **R-9.1.11** -- The engine **SHALL** define a canonical `HumanoidRig` with standard named slots
   (hips, spine, chest, neck, head, left and right arms with twist bones, left and right legs with
   twist bones, hands, feet, optional finger chains) used as the intermediate representation for
   automatic retargeting between humanoid skeletons.
   - **Rationale:** A single canonical rig eliminates manual bone correspondence when retargeting
     between arbitrary humanoid skeletons.
   - **Verification:** Import two humanoid skeletons. Assign the canonical rig mapping. Retarget a
     walk clip from one to the other. Assert foot contact timing and limb alignment are preserved.
2. **R-9.1.12** -- The engine **SHALL** support `SkeletonType` variants `Humanoid`, `Quadruped`,
   `Bird`, and `Custom(Vec<NamedBoneChain>)`, each exposing predefined bone chain templates for IK
   solvers, retargeting, and LOD.
   - **Rationale:** First-class skeleton types eliminate per-rig template reinvention for common
     non-humanoid topologies.
   - **Verification:** Create a quadruped skeleton. Assert the predefined leg and spine chains are
     populated. Create a custom skeleton and assert user-defined chains are accepted.
3. **R-9.1.13** -- The engine **SHALL** support named bone chains (`Spine`, `Limb`, `Digits`,
   `Tail`, `Custom`) that can be referenced by IK solvers, bone masks, LOD systems, and physics
   attachment, with `BoneMask::from_chains(&[...])` composing masks from semantic groups rather than
   raw bone indices.
   - **Rationale:** Named chains decouple tools and rigging conventions from raw bone indices.
   - **Verification:** Define a spine chain. Build a BoneMask from the chain. Assert the mask bits
     match the chain's bone indices. Assert the IK solver accepts the chain as a target.
4. **R-9.1.14** -- The engine **SHALL** support arbitrary mechanical and segmented rig topologies
   via the `Custom` skeleton variant, including multi-leg rigs with alternating gait groups, vehicle
   rigs with wheel and suspension bones, segmented serpentine rigs, and mechanical rigs exposing
   revolute, prismatic, and piston joint types with hard limits.
   - **Rationale:** Non-organic characters (spiders, vehicles, centipedes, mechanical props) need
     the same animation pipeline as humanoids without bespoke code paths.
   - **Verification:** Build an 8-leg spider rig. Assert 8 simultaneous IK solves with alternating
     gait groups. Build a vehicle rig. Assert wheel bones rotate from drivetrain speed and
     suspension bones displace from spring compression.
5. **R-9.1.15** -- The engine **SHALL** accept live motion capture streams (OptiTrack, Vicon,
   Rokoko, ARKit face capture) and offline BVH/FBX mocap clips, retargeting the source rig through
   the canonical humanoid rig to target skeletons as a high-priority override animation layer.
   - **Rationale:** Studio workflows require both live and offline mocap ingest directly into the
     animation pipeline without external baking.
   - **Verification:** Stream OptiTrack data into the engine. Assert `MocapEvent` components are
     produced. Assert the override layer drives the target skeleton. Import an FBX mocap take and
     verify playback matches the authored clip.
6. **R-9.1.16** -- The engine **SHALL** provide a FABRIK solver for multi-bone chains with
   configurable iteration count, per-chain angle limits, pole targets, and multiple end-effector
   priorities, operating as the default solver for chains longer than two bones.
   - **Rationale:** FABRIK handles long and multi-end-effector chains where analytical two-bone IK
     is insufficient.
   - **Verification:** Solve a 10-bone chain with 2 end effectors. Assert both effectors reach their
     targets within the iteration limit. Assert angle limits are respected.
7. **R-9.1.17** -- The engine **SHALL** support on-trigger animated-to-ragdoll conversion that
   spawns ragdoll bodies at current bone transforms, inherits velocity from the previous frame's
   animation delta, and supports get-up blending via a `RagdollBlendWeight` component lerping bones
   between physics and animation poses over a configurable duration.
   - **Rationale:** Seamless death, knockback, and get-up transitions require velocity-preserving
     ragdoll entry and smooth exit back to animation.
   - **Verification:** Trigger ragdoll mid-run. Assert bodies inherit the previous frame's velocity.
     Animate get-up. Assert the blend weight smoothly returns bones to animation.
8. **R-9.1.18** -- The engine **SHALL** support partial ragdoll where a bone mask partitions the
   skeleton into physics-driven and animated regions, with a constraining joint at the partition
   point keeping the two regions coherent.
   - **Rationale:** Hit reactions need upper-body physics response while lower-body locomotion
     continues without interruption.
   - **Verification:** Configure partial ragdoll with upper-body mask. Apply an impulse to the
     chest. Assert upper bones respond to physics while leg bones continue the walk cycle.
9. **R-9.1.19** -- The engine **SHALL** support 1, 2, 4, or 8 bone influences per vertex per mesh,
   with the asset pipeline normalizing weights and reducing influence counts for LOD meshes to save
   GPU bandwidth.
   - **Rationale:** Rigid props need one influence; faces and muscle simulation need eight;
     defaulting everything to four wastes bandwidth.
   - **Verification:** Import a mesh with 1, 2, 4, and 8 influence variants. Assert the skinning
     shader reads the correct influence count from meshlet metadata. Assert LOD meshes reduce
     influence count correctly.
10. **R-9.1.20** -- The engine **SHALL** support named bone sockets that expose a local-offset
    transform derived from a specific bone, with entities carrying an `AttachedToSocket` component
    inheriting the socket's world transform each frame after the bone palette is computed.
    - **Rationale:** Weapons, helmets, backpacks, and VFX emitters must follow the skeleton without
      per-item animation code.
    - **Verification:** Define a right-hand weapon socket. Attach a weapon entity. Assert the weapon
      world transform equals the hand bone world transform times the socket offset. Animate the
      character and assert the weapon follows through every frame including ragdoll.
11. **R-9.1.21** -- The engine **SHALL** support modular character customization where multiple mesh
    entities share a single skeleton and bone palette, with parts authored against the canonical
    skeleton and optional load-time merging into a combined mesh to reduce draw calls.
    - **Rationale:** Customization systems cannot afford unique meshes per combination of head,
      torso, arms, legs, hair, and accessories.
    - **Verification:** Load three body parts on one skeleton. Assert all parts share a single bone
      palette. Enable load-time merging and assert the combined mesh renders as a single draw call
      per material.

## Non-Functional Requirements

1. **R-9.1.NF1** -- The engine **SHALL** batch-evaluate 1000 skeleton instances in a single GPU
   compute dispatch with total frame time under 2 ms.
   - **Rationale:** Instanced evaluation must scale to MMO-scale crowds without per-instance CPU
     overhead.
   - **Verification:** Spawn 1000 instances. Assert dispatch count is 1. Assert frame time under 2
     ms.

2. **R-9.1.NF2** -- The engine **SHALL** achieve 10:1 or greater compression on humanoid animation
   clips with positional error per joint below the perceptual threshold.
   - **Rationale:** Compressed clips reduce memory and streaming bandwidth without visible quality
     loss.
   - **Verification:** Compress and decompress a reference clip. Assert ratio exceeds 10:1. Assert
     per-joint error below 0.001 units.
