# User Stories -- 9.1 Skeletal Animation

## GPU Skinning and Evaluation

| ID          | Persona                    |
|-------------|----------------------------|
| US-9.1.1.1  | engine developer (P-26)    |
| US-9.1.1.2  | character animator (P-11)  |
| US-9.1.2.1  | engine developer (P-26)    |
| US-9.1.2.2  | character animator (P-11)  |
| US-9.1.3.1  | character animator (P-11)  |
| US-9.1.3.2  | engine developer (P-26)    |

1. **US-9.1.1.1** -- **As an** engine developer (P-26), **I want** GPU compute skinning that
   transforms vertices by weighted bone matrices using linear blend and dual-quaternion modes,
   **so that** thousands of skeleton instances are evaluated in a single dispatch without CPU
   bottlenecks.

2. **US-9.1.1.2** -- **As a** character animator (P-11), **I want** to toggle between linear blend
   and dual-quaternion skinning per skeleton in the viewport, **so that** I can fix candy-wrapping
   artifacts at twist joints without a code change.

3. **US-9.1.2.1** -- **As an** engine developer (P-26), **I want** GPU keyframe evaluation using
   Hermite interpolation for translation, rotation, and scale tracks in a single compute dispatch,
   **so that** animation curve sampling requires no CPU-side per-joint work.

4. **US-9.1.2.2** -- **As a** character animator (P-11), **I want** to select loop, clamp, or
   ping-pong playback mode per clip in the animation editor, **so that** cyclic walks and one-shot
   attacks have correct end behavior.

5. **US-9.1.3.1** -- **As a** character animator (P-11), **I want** linear and cubic Hermite
   blending between up to 8 simultaneous clips, **so that** complex locomotion blends transition
   smoothly without popping.

6. **US-9.1.3.2** -- **As an** engine developer (P-26), **I want** blend weights computed CPU-side
   and uploaded as blend descriptors, **so that** the GPU performs blending without per-frame CPU
   intervention.

## Layers and Instancing

| ID          | Persona                    |
|-------------|----------------------------|
| US-9.1.4.1  | character animator (P-11)  |
| US-9.1.4.2  | technical artist (P-13)    |
| US-9.1.5.1  | engine developer (P-26)    |
| US-9.1.5.2  | engine developer (P-26)    |

1. **US-9.1.4.1** -- **As a** character animator (P-11), **I want** animation layers with per-bone
   masks, **so that** upper-body combat animations overlay on lower-body locomotion without
   authoring separate combined clips.

2. **US-9.1.4.2** -- **As a** technical artist (P-13), **I want** additive layers encoding pose
   deltas relative to a reference pose, **so that** hit reactions, breathing, and weapon sway
   composite on any base animation.

3. **US-9.1.5.1** -- **As an** engine developer (P-26), **I want** instanced skeletal animation that
   batch-evaluates thousands of skeleton instances in a single compute dispatch, **so that** MMO
   city hubs render with full animation quality at scale.

4. **US-9.1.5.2** -- **As an** engine developer (P-26), **I want** instances sharing the same clip
   grouped for optimal GPU occupancy, **so that** dispatch overhead is minimized.

## Root Motion and Compression

| ID          | Persona                    |
|-------------|----------------------------|
| US-9.1.6.1  | character animator (P-11)  |
| US-9.1.6.2  | engine developer (P-26)    |
| US-9.1.7.1  | engine developer (P-26)    |
| US-9.1.7.2  | technical artist (P-13)    |

1. **US-9.1.6.1** -- **As a** character animator (P-11), **I want** root motion extraction that
   separates authored locomotion from gameplay movement, **so that** dodges, vaults, and mount
   animations drive the character transform naturally.

2. **US-9.1.6.2** -- **As an** engine developer (P-26), **I want** root motion deltas applied
   through the physics system while maintaining capsule collision, **so that** authored movement
   respects collision.

3. **US-9.1.7.1** -- **As an** engine developer (P-26), **I want** animation compression using
   smallest-three quaternion encoding and range-reduced fixed-point values, **so that** 10:1+
   compression ratios are achieved on humanoid clips.

4. **US-9.1.7.2** -- **As a** technical artist (P-13), **I want** to inspect compression quality per
   clip in the editor, **so that** I can verify no visible artifacts from quantization.

## Retargeting, Events, and LOD

| ID           | Persona                    |
|--------------|----------------------------|
| US-9.1.8.1   | character animator (P-11)  |
| US-9.1.8.2   | rigger (P-10)              |
| US-9.1.9.1   | character animator (P-11)  |
| US-9.1.9.2   | engine developer (P-26)    |
| US-9.1.10.1  | engine developer (P-26)    |
| US-9.1.10.2  | technical artist (P-13)    |

1. **US-9.1.8.1** -- **As a** character animator (P-11), **I want** to play animation clips authored
   on one skeleton on a different skeleton with different proportions, **so that** animation
   libraries are reusable across character types.

2. **US-9.1.8.2** -- **As a** rigger (P-10), **I want** per-bone retargeting modes and manual bone
   remapping for cross-species retargeting, **so that** animations transfer between dissimilar
   skeletons with control over joint correspondence.

3. **US-9.1.9.1** -- **As a** character animator (P-11), **I want** named events embedded at
   specific frames in animation clips, **so that** gameplay systems receive callbacks for footsteps,
   impacts, and VFX triggers during playback.

4. **US-9.1.9.2** -- **As an** engine developer (P-26), **I want** animation events to flow through
   the ECS observer system as typed event components, **so that** event handling integrates with the
   entity event pipeline.

5. **US-9.1.10.1** -- **As an** engine developer (P-26), **I want** animation LOD that reduces bone
   count and update rate for distant entities, **so that** animation cost scales with screen
   importance.

6. **US-9.1.10.2** -- **As a** technical artist (P-13), **I want** per-entity LOD bias for hero
   characters, **so that** important characters maintain full animation quality regardless of
   distance.

## Skeleton Types and Rigs

| ID           | Persona                    |
|--------------|----------------------------|
| US-9.1.11.1  | rigger (P-10)              |
| US-9.1.12.1  | rigger (P-10)              |
| US-9.1.13.1  | technical artist (P-13)    |
| US-9.1.14.1  | rigger (P-10)              |

1. **US-9.1.11.1** -- **As a** rigger (P-10), **I want** a canonical humanoid rig with standard
   named slots for hips, spine, arms, legs, and optional fingers, **so that** any two humanoid
   skeletons can share animation through automatic retargeting.
2. **US-9.1.12.1** -- **As a** rigger (P-10), **I want** predefined quadruped and bird rigs plus a
   custom chain variant for arbitrary topologies, **so that** I can author skeletons for any
   creature without reinventing the rig template.
3. **US-9.1.13.1** -- **As a** technical artist (P-13), **I want** named bone chains that build bone
   masks by semantic group, **so that** I compose per-chain IK, blend masks, and ragdoll scopes
   without referencing raw bone indices.
4. **US-9.1.14.1** -- **As a** rigger (P-10), **I want** the custom skeleton variant to support
   spider, vehicle, segmented, and mechanical rigs with specialized joint types, **so that**
   non-organic characters work on the same animation pipeline as humanoids.

## Motion Capture and IK Chains

| ID           | Persona                    |
|--------------|----------------------------|
| US-9.1.15.1  | character animator (P-11)  |
| US-9.1.15.2  | character animator (P-11)  |
| US-9.1.16.1  | technical artist (P-13)    |

1. **US-9.1.15.1** -- **As a** character animator (P-11), **I want** streaming body mocap data
   ingested via MocapEvent components and retargeted to the canonical humanoid rig, **so that** live
   performance capture drives in-engine characters in real time.
2. **US-9.1.15.2** -- **As a** character animator (P-11), **I want** ARKit-style 52-blendshape face
   capture from an iOS device routed to the face morph targets, **so that** facial performances can
   be recorded without a dedicated capture stage.
3. **US-9.1.16.1** -- **As a** technical artist (P-13), **I want** FABRIK solving multi-bone chains
   on GPU with pole targets and angle limits, **so that** spines, tails, and tentacles deform
   organically where two-bone IK is insufficient.

## Ragdoll Blending

| ID           | Persona                    |
|--------------|----------------------------|
| US-9.1.17.1  | game developer (P-15)      |
| US-9.1.17.2  | character animator (P-11)  |
| US-9.1.18.1  | character animator (P-11)  |

1. **US-9.1.17.1** -- **As a** game developer (P-15), **I want** on-trigger conversion from animated
   pose to ragdoll with velocity inherited from the last frame's animation delta, **so that** deaths
   and knockbacks look physically continuous.
2. **US-9.1.17.2** -- **As a** character animator (P-11), **I want** a ragdoll blend weight that
   smoothly lerps bones between physics and animation over a configurable duration, **so that**
   get-up animations transition seamlessly out of ragdoll.
3. **US-9.1.18.1** -- **As a** character animator (P-11), **I want** partial ragdoll that drives
   only the upper body from physics while the lower body keeps walking, **so that** hit reactions
   look dynamic without interrupting locomotion.

## Skinning Extensions

| ID           | Persona                    |
|--------------|----------------------------|
| US-9.1.19.1  | technical artist (P-13)    |
| US-9.1.20.1  | character animator (P-11)  |
| US-9.1.20.2  | game developer (P-15)      |
| US-9.1.21.1  | character artist (P-9)     |
| US-9.1.21.2  | game developer (P-15)      |

1. **US-9.1.19.1** -- **As a** technical artist (P-13), **I want** to select 1, 2, 4, or 8 bone
   influences per mesh with auto-reduction on LOD meshes, **so that** GPU skinning bandwidth matches
   the mesh's deformation complexity.
2. **US-9.1.20.1** -- **As a** character animator (P-11), **I want** to define named bone sockets on
   skeleton assets by clicking a bone and naming the socket, **so that** weapons, helmets, and VFX
   emitters attach to characters without writing code.
3. **US-9.1.20.2** -- **As a** game developer (P-15), **I want** attached entities to inherit the
   socket bone's world transform times the socket's local offset, **so that** weapon meshes follow
   the hand through every animation including ragdoll.
4. **US-9.1.21.1** -- **As a** character artist (P-9), **I want** modular body parts (head, torso,
   arms, legs, hair, cape) authored against the same canonical skeleton, **so that** characters can
   swap parts for customization without rerigging.
5. **US-9.1.21.2** -- **As a** game developer (P-15), **I want** modular parts optionally merged
   into a single combined mesh at load time, **so that** heavily customized characters still issue
   one draw call per material.
