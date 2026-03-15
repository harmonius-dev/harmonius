# User Stories -- 9.4 Animation State Machine

## US-9.4.1.1 Author Character Animation Graphs in the Visual Editor

**As a** character animator (P-11), **I want** a declarative state graph where each node is a pose
source (clip, blend tree, sub-state machine, montage) evaluated CPU-side with GPU-uploaded blend
descriptors, **so that** I can build complex animation behavior visually without writing code.

## US-9.4.1.2 Share Graph Definitions Across Thousands of Character Instances

**As a** game developer (P-15), **I want** per-character graph instances that share graph
definitions, **so that** thousands of MMO characters use minimal memory overhead per instance.

## US-9.4.1.3 Validate State Graph Instance Count at MMO Scale

**As an** engine tester (P-27), **I want** to spawn 1,000 characters each with active state graph
instances and verify that CPU evaluation time stays within 1ms, **so that** state graph overhead is
acceptable at MMO density.

## US-9.4.2.1 Blend Between States With Per-Bone Profiles and Sync Markers

**As a** character animator (P-11), **I want** transitions with configurable blend duration, blend
curve, and per-bone blend profiles that allow different body parts to transition at different rates,
**so that** locomotion transitions keep feet planted via sync marker alignment.

## US-9.4.2.2 Validate Sync Marker Foot Alignment Prevents Sliding

**As an** engine tester (P-27), **I want** to transition between walk and run cycles with sync
markers and measure foot sliding distance, verifying it stays below 1cm, **so that** locomotion
transitions produce grounded foot contacts.

## US-9.4.3.1 Encapsulate Combat States as a Reusable Sub-State Machine

**As a** character animator (P-11), **I want** to package a combat state machine (idle, attack,
dodge, stagger) as a reusable sub-graph with entry and exit points, **so that** multiple character
archetypes share the combat sub-graph without duplication.

## US-9.4.3.2 Nest Sub-State Machines Hierarchically

**As a** game developer (P-15), **I want** to nest a combat sub-state machine inside a top-level
locomotion graph, **so that** complex character behavior is decomposed into manageable, composable
modules.

## US-9.4.4.1 Run Upper-Body and Lower-Body State Machines in Parallel

**As a** character animator (P-11), **I want** multiple state machine layers running in parallel
with per-bone masks and blend modes (override, additive), **so that** upper-body aiming operates
independently from lower-body locomotion.

## US-9.4.4.2 Adjust Layer Weights at Runtime for Smooth Activation

**As a** game developer (P-15), **I want** to dynamically adjust layer weights at runtime to
smoothly activate and deactivate overlay behaviors (facial expressions, hit reactions), **so that**
layer transitions feel natural rather than snapping on/off.

## US-9.4.4.3 Validate Parallel Layer Count Per Platform

**As an** engine tester (P-27), **I want** to verify that mobile supports 2 parallel layers, Switch
3, and desktop 4+, **so that** layer complexity respects per-platform animation budgets.

## US-9.4.5.1 Drive Transitions From Gameplay Parameters Without Code

**As a** game designer (P-5), **I want** named parameters (booleans, floats, triggers) that I set
from logic graphs to drive state transitions via boolean expressions, **so that** I can tune
animation behavior without programmer support.

## US-9.4.5.2 Verify Trigger Parameters Auto-Reset After Consumption

**As an** engine tester (P-27), **I want** to fire a trigger parameter (jump, attack) and verify it
auto-resets after the transition consumes it, **so that** one-shot events fire exactly once without
manual reset.

## US-9.4.6.1 Keep Walk and Run Cycles Phase-Synchronized

**As a** character animator (P-11), **I want** sync groups that advance multiple clips by normalized
time with aligned sync markers, **so that** blending between walk and run speeds keeps feet in
consistent contact timing without sliding.

## US-9.4.6.2 Validate Sync Group Phase Alignment at Speed Transitions

**As an** engine tester (P-27), **I want** to sweep blend space speed from walk to sprint and verify
that foot contact markers stay aligned within 1 frame tolerance, **so that** locomotion blending is
temporally correct.

## US-9.4.7.1 Play Emote and Ability Montages That Override State Machine

**As a** character animator (P-11), **I want** montages that temporarily override state machine
output on specific bone groups with branching sections and blend curves, **so that** emotes, combat
abilities, and interaction animations play cleanly on top of locomotion.

## US-9.4.7.2 Trigger Hundreds of Simultaneous Montages in a Raid

**As a** game developer (P-15), **I want** concurrent montage playback to scale to hundreds of
simultaneous players triggering unique ability montages, **so that** MMO raid encounters run without
animation system bottlenecks.

## US-9.4.7.3 Validate Montage Notify Events Fire Correctly

**As an** engine tester (P-27), **I want** to play a montage with embedded notify events and verify
that montage-scoped notifies fire only during montage playback and not during state machine
evaluation, **so that** event scoping is correct.

## US-9.4.8.1 Author Locomotion Blend Spaces With Speed and Direction

**As a** character animator (P-11), **I want** 1D and 2D blend spaces where I place animation
samples in parameter space (speed, direction) and the runtime interpolates using barycentric
weights, **so that** locomotion covers all movement angles smoothly.

## US-9.4.8.2 Preview Blend Space Triangulation in the Editor

**As a** character animator (P-11), **I want** to see the triangulated sample layout in the blend
space editor and adjust sample positions visually, **so that** I can optimize clip placement for
smooth interpolation across the full parameter range.

## US-9.4.8.3 Validate Blend Space Sample Count Per Platform

**As an** engine tester (P-27), **I want** to verify that mobile supports 6-8 blend space samples,
Switch 12, and desktop 16+, **so that** blend space complexity scales per platform.

## US-9.4.9.1 Aim Weapons With Additive Pitch/Yaw Offset Layers

**As a** character animator (P-11), **I want** aim offsets as 2D blend spaces parameterized by pitch
and yaw that produce additive bone rotations on top of locomotion, **so that** characters aim
weapons toward a target while locomotion continues independently.

## US-9.4.9.2 Integrate Aim Offsets With IK for Precise Weapon Alignment

**As a** game developer (P-15), **I want** aim offsets integrated with the IK system for precise
weapon-to-target alignment, **so that** the gun barrel points accurately at the aim target in
third-person shooters.

## US-9.4.10.1 Drive AI NPC Animations From Behavior Trees

**As a** game designer (P-5), **I want** behavior trees and GOAP planners to set blackboard
variables that drive blend space parameters and trigger state transitions, **so that** AI agents
animate responsively based on their decision-making without hardcoded animation calls.

## US-9.4.10.2 Query Animation State From AI Decision-Making

**As a** game developer (P-15), **I want** AI systems to query current animation state, remaining
clip time, and root motion displacement, **so that** AI decisions synchronize with animation timing
(wait for attack wind-up, dodge after stagger recovery).

## US-9.4.10.3 Validate AI-Driven Animation at 500 Concurrent Agents

**As an** engine tester (P-27), **I want** to run 500 AI agents each driving animation state via
behavior trees and verify that combined CPU cost for AI + animation evaluation stays within 2ms,
**so that** AI animation scales to MMO population density.
