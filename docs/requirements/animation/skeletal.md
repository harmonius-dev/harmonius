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
