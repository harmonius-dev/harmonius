# User Stories -- 9.3 Procedural Animation

## US-9.3.1.1 Place Character Hands on Weapons Using Two-Bone IK

**As a** character animator (P-11), **I want** two-bone IK with pole vector targets to position
hands on weapon grips, ledges, and mount handles, **so that** characters interact physically
with objects without authoring per-object hand-placement clips.

## US-9.3.1.2 Solve Hundreds of IK Chains Simultaneously on GPU

**As an** engine developer (P-26), **I want** two-bone IK running as a GPU post-process pass
over skinned skeletons, **so that** 500+ simultaneous IK chains (desktop) evaluate without CPU
overhead.

## US-9.3.1.3 Validate IK Chain Count Limits Per Platform

**As an** engine tester (P-27), **I want** to verify that mobile supports 20-40 active IK
chains per frame, Switch supports 80, and desktop supports 500+, **so that** IK budget is
enforced per platform.

## US-9.3.2.1 Animate Tails and Tentacles With CCD IK

**As a** character animator (P-11), **I want** CCD IK for medium-length chains (3-8 bones) with
configurable iteration limits and angular constraints per joint, **so that** tails, spines, and
tentacles track targets smoothly.

## US-9.3.2.2 Validate CCD Iteration Count Per Platform

**As an** engine tester (P-27), **I want** to verify that mobile uses 2-4 CCD iterations,
Switch uses 6, and desktop uses 8-12, **so that** CCD accuracy trades correctly against
per-platform compute budgets.

## US-9.3.3.1 Solve Spider Legs With Multi-End-Effector FABRIK

**As a** character animator (P-11), **I want** FABRIK IK for long chains with multiple end-
effector targets and joint constraints, **so that** spider legs and branching skeletal structures
solve correctly with position-space efficiency.

## US-9.3.3.2 Validate FABRIK Simplification on Mobile

**As an** engine tester (P-27), **I want** to verify that mobile uses 2-3 FABRIK iterations with
fewer targets while desktop uses 6-8, **so that** multi-end-effector problems are simplified
appropriately on mobile.

## US-9.3.4.1 Blend Between Animated and Ragdoll Poses Per Bone

**As a** character animator (P-11), **I want** per-bone blend weights between animation and
ragdoll physics, **so that** partial ragdoll (upper body ragdolls on hit while legs animate)
produces smooth transitions.

## US-9.3.4.2 Recover From Ragdoll to Animated Locomotion Smoothly

**As a** game developer (P-15), **I want** ragdoll-to-animated recovery transitions that blend
from physics pose back to animation over a configurable duration, **so that** characters regain
composure naturally after knockback rather than snapping to idle.

## US-9.3.4.3 Validate Ragdoll Body Count Per Platform

**As an** engine tester (P-27), **I want** to verify that mobile supports 4-8 ragdoll capsules,
Switch supports 12, and desktop supports 16-20, **so that** ragdoll complexity scales per
platform.

## US-9.3.5.1 Make NPCs Turn Their Heads to Track Nearby Players

**As a** game designer (P-5), **I want** look-at constraints that procedurally rotate head and
spine bones to track a world-space target with angle limits, **so that** NPCs visually respond
to players walking past without authored look animations.

## US-9.3.5.2 Blend Aim Constraints Smoothly With Underlying Animation

**As a** character animator (P-11), **I want** aim constraints that orient weapon-holding arms
toward a target with smooth blending and joint limits, **so that** aiming poses look natural on
top of any locomotion state.

## US-9.3.6.1 Select Best Animation Continuation From a Pose Database

**As a** character animator (P-11), **I want** motion matching that searches a pose database for
the best continuation given the current pose and desired trajectory, **so that** character
movement feels responsive and natural without hand-authored state machine locomotion.

## US-9.3.6.2 Profile Motion Matching Database Search Cost

**As an** engine developer (P-26), **I want** to benchmark pose database search at varying
database sizes (small on mobile, full mocap library on desktop), **so that** I can set per-
platform database size limits that fit within CPU budgets.

## US-9.3.7.1 Plant Feet on Uneven Terrain Without Foot Sliding

**As a** player (P-23), **I want** characters' feet to plant correctly on stairs, slopes, and
uneven ground with IK-adjusted pelvis height, **so that** locomotion looks grounded and
believable on any surface.

## US-9.3.7.2 Validate Foot Placement Raycast Count Per Platform

**As an** engine tester (P-27), **I want** to verify that mobile uses 2 raycasts (feet only)
and desktop uses 4 (feet + pelvis + slope probe), and that foot placement is disabled for
distant characters on mobile, **so that** raycast cost scales per platform.

## US-9.3.8.1 Animate Quadrupeds and Hexapods With Procedural Gaits

**As a** character animator (P-11), **I want** procedural locomotion that adapts to arbitrary
skeleton topologies with configurable gait patterns (walk, trot, canter, gallop), **so that**
bipeds, quadrupeds, hexapods, and serpentine creatures locomote correctly from a single system.

## US-9.3.8.2 Configure Locomotion Profiles via ECS Components

**As a** game developer (P-15), **I want** locomotion profiles defined as ECS components
(LocomotionProfile, GaitState, FootGroup) specifying leg groups, phase offsets, and stride
curves, **so that** creature locomotion is data-driven and configurable per entity.

## US-9.3.8.3 Validate Hexapod Gait Simplification on Mobile

**As an** engine tester (P-27), **I want** to verify that hexapod+ topologies use simplified
gait patterns on mobile with reduced IK iterations, **so that** complex creatures remain
animatable on lower-tier hardware.

## US-9.3.9.1 Stumble and Recover Using Physics-Based Locomotion

**As a** player (P-23), **I want** characters to stumble when hit by impacts and recover
balance using physical forces, **so that** combat feels physically grounded with emergent
reactions rather than canned hit animations.

## US-9.3.9.2 Profile Physics-Based Locomotion Per-Limb Force Cost

**As an** engine developer (P-26), **I want** to measure per-limb force computation cost for
physics-based locomotion, **so that** I can confirm it is limited to hero/player characters on
mobile and scales to multiple characters on desktop.

## US-9.3.10.1 Sever a Dragon's Wing and Watch It Ragdoll Away

**As a** player (P-23), **I want** dismemberment to sever a bone chain from a skeleton, spawning
the detached sub-mesh as a physics-simulated entity while the remaining skeleton adapts
locomotion, **so that** combat dismemberment feels visceral and consequential.

## US-9.3.10.2 Attach and Detach Equipment at Runtime via ECS Commands

**As a** game developer (P-15), **I want** runtime attachment and removal of skeletal
sub-hierarchies (weapons, armor plates, holsters) via ECS command buffers, **so that** equipment
changes are immediate and data-driven.

## US-9.3.10.3 Validate Dismembered Part Count Per Platform

**As an** engine tester (P-27), **I want** to verify that mobile caps active dismembered parts
at 2-4 with simplified ragdoll and desktop supports 8-16, **so that** physics entity count
from dismemberment stays within platform budgets.

## US-9.3.11.1 Visualize Foot Placement and IK Solve States

**As a** character animator (P-11), **I want** debug overlays showing foot placement targets,
IK chain states, ground normals, gait phase diagrams, and balance center-of-mass, **so that** I
can diagnose foot sliding and IK solve failures in the animation preview.

## US-9.3.11.2 Profile Per-Frame IK Iteration Counts and Foot Plant Errors

**As an** engine developer (P-26), **I want** a locomotion profiler panel showing IK iterations,
foot plant errors, and physics force magnitudes per frame, **so that** I can identify animation
performance regressions and solve quality issues.

## US-9.3.11.3 Verify Debug Visualization Is Stripped From Shipping Builds

**As an** engine tester (P-27), **I want** to confirm that locomotion debug visualization is
compiled out of shipping builds on all platforms, **so that** debug rendering has zero cost in
production.
