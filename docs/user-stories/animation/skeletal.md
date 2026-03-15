# User Stories -- 9.1 Skeletal Animation

## US-9.1.1.1 Skin Thousands of Characters With GPU Compute Skinning

**As an** engine developer (P-26), **I want** GPU compute skinning that transforms vertices by
weighted bone matrices using linear blend and dual-quaternion modes, **so that** thousands of
skeleton instances are evaluated in a single dispatch without CPU bottlenecks.

## US-9.1.1.2 Preview Dual-Quaternion Skinning to Fix Candy-Wrapping

**As a** character animator (P-11), **I want** to toggle between linear blend and dual-quaternion
skinning per skeleton and see results in the viewport, **so that** I can fix candy-wrapping
artifacts at twist joints without a code change.

## US-9.1.1.3 Validate Bone Count Limits Per Platform

**As an** engine tester (P-27), **I want** to confirm that mobile supports 32-64 bones, Switch
supports 96, and desktop supports 256+ bones per skeleton, **so that** skeleton complexity
respects per-platform GPU budgets.

## US-9.1.2.1 Evaluate Animation Curves Entirely on GPU

**As an** engine developer (P-26), **I want** GPU keyframe evaluation using Hermite interpolation
for translation, rotation, and scale tracks in a single compute dispatch, **so that** animation
curve sampling does not require CPU-side per-joint evaluation.

## US-9.1.2.2 Preview Looped, Clamped, and Ping-Pong Playback Modes

**As a** character animator (P-11), **I want** to select loop, clamp, or ping-pong playback mode
per clip and preview the result in the animation editor, **so that** I can author cyclic walk
loops and one-shot attack clips with correct end behavior.

## US-9.1.3.1 Blend Up to 8 Clips Smoothly per Skeleton

**As a** character animator (P-11), **I want** linear and cubic Hermite blending between up to
8 simultaneous clips, **so that** complex locomotion blends (walk + strafe + lean) transition
smoothly without popping.

## US-9.1.3.2 Validate Blend Clip Count Scaling Per Platform

**As an** engine tester (P-27), **I want** to verify that mobile supports 4 simultaneous blend
clips, Switch supports 6, and desktop supports 8, **so that** blend complexity scales within
per-platform GPU budgets.

## US-9.1.4.1 Overlay Upper-Body Combat on Lower-Body Locomotion

**As a** character animator (P-11), **I want** animation layers with per-bone masks so that
upper-body combat animations overlay on lower-body locomotion, **so that** a character can swing
a sword while running without authoring separate combo-run clips.

## US-9.1.4.2 Add Hit Reactions as Additive Layers

**As a** game developer (P-15), **I want** additive animation layers that encode pose deltas
relative to a reference pose, **so that** hit reactions, breathing, and weapon sway composite on
top of any base animation without per-combination authoring.

## US-9.1.4.3 Validate Layer Count Limits Per Platform

**As an** engine tester (P-27), **I want** to verify that mobile supports 2 active animation
layers, Switch supports 3, and desktop supports 4+, **so that** additive layers do not exceed
per-platform evaluation budgets.

## US-9.1.5.1 Animate MMO Crowds Without Per-Instance CPU Overhead

**As a** game developer (P-15), **I want** instanced skeletal animation that batch-evaluates
thousands of skeleton instances in a single compute dispatch using an arena buffer, **so that**
MMO city hubs render with full animation quality at scale.

## US-9.1.5.2 Profile Instanced Animation at 500+ Characters

**As an** engine developer (P-26), **I want** to benchmark instanced skeletal animation at 50
(mobile), 200 (Switch), and 500+ (desktop) simultaneous instances, **so that** I can confirm
GPU compute time stays within 1ms per platform tier.

## US-9.1.6.1 Drive Character Movement From Root Motion

**As a** character animator (P-11), **I want** root motion extraction to apply root bone
translation and rotation deltas to the character's world transform via physics, **so that**
authored locomotion clips (dodges, vaults, mounts) drive gameplay movement while maintaining
capsule collision.

## US-9.1.6.2 Test Root Motion Separation From Gameplay Movement

**As an** engine tester (P-27), **I want** to play a root-motion vault clip and verify that the
character's capsule follows the root bone path while physics collision is maintained, **so that**
root motion and gameplay movement integrate correctly.

## US-9.1.7.1 Stream Thousands of Unique Animation Clips Efficiently

**As a** game developer (P-15), **I want** animation compression achieving 10:1+ ratios using
smallest-three quaternion encoding and range-reduced fixed-point values, **so that** thousands
of unique emote, combat, and mount clips can be streamed without exhausting memory.

## US-9.1.7.2 Validate Compression Quality Across Different Clip Types

**As an** engine tester (P-27), **I want** to compare compressed versus uncompressed animation
playback for locomotion, facial, and combat clips, verifying that visual quality loss is
imperceptible, **so that** compression does not introduce visible artifacts.

## US-9.1.8.1 Retarget Human Mocap Onto Fantasy Creatures

**As a** character animator (P-11), **I want** to retarget animation clips between skeletons
with different bone counts and proportions using a shared canonical pose and per-bone mode
(direct copy, scaled translation, rotation only), **so that** human mocap data works on
non-human characters.

## US-9.1.8.2 Author Retargeting Maps Visually in the Animation Editor

**As a** character animator (P-11), **I want** to define bone correspondence and per-bone
retargeting modes visually in the editor rather than through code, **so that** cross-skeleton
retargeting is accessible to animators without programmer support.

## US-9.1.8.3 Validate Cross-Species Retargeting With Manual Bone Remapping

**As an** engine tester (P-27), **I want** to retarget a human walk cycle onto a quadruped
with manual bone remapping and verify that the output is free of NaN transforms and visible
deformation errors, **so that** cross-species retargeting produces usable results.

## US-9.1.9.1 Place Footstep and VFX Events on the Animation Timeline

**As a** character animator (P-11), **I want** to place named events at specific frames in the
animation editor timeline for footstep sounds, hit windows, VFX spawns, and weapon trails,
**so that** gameplay systems react to animation timing without hardcoded frame numbers.

## US-9.1.9.2 Fire Events Through ECS Observers

**As a** game developer (P-15), **I want** animation events to dispatch as typed event
components through the ECS observer system, **so that** gameplay systems can subscribe to
animation timing events with type-safe handler functions.

## US-9.1.9.3 Test Window Events Active for a Frame Range

**As an** engine tester (P-27), **I want** to verify that window events (active for a frame
range) fire correctly during the specified range and do not fire outside it, **so that**
hit detection windows and weapon trail durations are frame-accurate.

## US-9.1.10.1 Reduce Animation Cost for Distant Characters Automatically

**As a** game developer (P-15), **I want** animation LOD to automatically transition distant
characters through reduced bone sets, lower update rates, and VAT playback, **so that** MMO
scenes with hundreds of visible characters maintain target frame time.

## US-9.1.10.2 Override LOD for Hero Characters Regardless of Distance

**As a** character animator (P-11), **I want** a per-entity LOD bias that keeps hero characters
at full animation fidelity regardless of distance, **so that** the player character and key NPCs
never show reduced animation quality.

## US-9.1.10.3 Validate Animation LOD Threshold Distances Per Platform

**As an** engine tester (P-27), **I want** to verify that mobile uses more aggressive LOD
thresholds (earlier transitions, shorter VAT distances) than desktop, **so that** animation cost
scales with platform capability.
