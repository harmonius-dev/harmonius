# R-9.3 -- Procedural Animation Requirements

## R-9.3.1 Two-Bone IK Solver

The engine **SHALL** solve two-bone IK chains analytically using the law of cosines with pole vector
targets, running on the GPU as a post-process pass over the skinned skeleton.

- **Derived from:** [F-9.3.1](../../features/animation/procedural.md)
- **Rationale:** Analytic two-bone IK is the most common IK configuration for humanoid limbs and
  must run on the GPU to scale to hundreds of characters simultaneously.
- **Verification:** Place a hand IK target at a known world position and solve. Verify the
  end-effector reaches the target within 0.01 units. Rotate the pole vector 90 degrees and verify
  the elbow orientation changes accordingly. Solve for 500 characters simultaneously and verify all
  solutions are correct within tolerance in a single GPU dispatch.

## R-9.3.2 CCD IK Solver

The engine **SHALL** solve medium-length IK chains (3-8 bones) using Cyclic Coordinate Descent with
configurable iteration limits and per-joint angular constraints, running as a GPU compute pass.

- **Derived from:** [F-9.3.2](../../features/animation/procedural.md)
- **Rationale:** CCD handles medium-length chains (tails, spines) that analytic two-bone IK cannot
  solve, and convergence in few iterations makes it GPU-friendly.
- **Verification:** Configure a 6-bone tail chain with angular constraints of 30 degrees per joint.
  Place the target within reachable range and verify the end-effector converges within 0.05 units in
  10 or fewer iterations. Place the target outside reachable range and verify the chain extends
  toward it without violating per-joint angular limits.

## R-9.3.3 FABRIK IK Solver

The engine **SHALL** solve long IK chains and multi-end-effector problems using Forward And Backward
Reaching Inverse Kinematics in position space, supporting joint constraints and multiple target
priorities.

- **Derived from:** [F-9.3.3](../../features/animation/procedural.md)
- **Rationale:** FABRIK is efficient for many-bone chains and multi-end-effector setups (spider
  legs, branching skeletons) where rotation-space solvers are costly.
- **Verification:** Configure an 8-leg spider skeleton with 4 bones per leg and 8 end-effector
  targets. Verify all 8 legs reach their targets within 0.05 units after convergence. Assign
  different priorities to two conflicting targets and verify the higher-priority target is reached
  more accurately.

## R-9.3.4 Ragdoll Physics

The engine **SHALL** simulate ragdoll physics using position-based dynamics on async compute with
capsule and sphere collision primitives, supporting both full ragdoll and partial ragdoll with
per-bone blend weights between animation and physics. Ragdoll joint simulation **SHALL** delegate to
the physics constraint solver (R-4.3.5). The animation system **SHALL** provide blend weights,
partial ragdoll masking, and recovery transitions.

- **Derived from:** [F-9.3.4](../../features/animation/procedural.md)
- **Rationale:** Ragdoll simulation produces physically plausible death and impact responses, and
  partial ragdoll enables smooth animation-to-physics transitions for hit reactions. Delegating
  joint simulation to the physics domain avoids duplicate solvers and ensures consistent physical
  behavior.
- **Verification:** Enable full ragdoll on a character and apply an impulse. Verify all bones
  respond to physics and no bone penetrates the ground plane. Enable partial ragdoll on the upper
  body only, play a walk animation on the lower body, and verify upper-body bones follow physics
  while lower-body bones follow animation. Transition blend weights from 0 to 1 over 0.5 seconds and
  verify no visible discontinuity in the resulting pose.

## R-9.3.5 Look-At and Aim Constraints

The engine **SHALL** procedurally rotate head and spine bones to track a world-space target with
configurable angle limits and blend weights, and orient weapon-holding arms toward aim targets while
respecting joint limits.

- **Derived from:** [F-9.3.5](../../features/animation/procedural.md)
- **Rationale:** Look-at and aim constraints enable characters to visually respond to nearby
  entities without requiring authored per-target animations.
- **Verification:** Set a look-at target 45 degrees to the right and verify the head rotates to face
  it within 1 degree. Set the target 120 degrees to the right (beyond the configured 90 degree
  limit) and verify the head clamps at 90 degrees. Set an aim target and verify the weapon direction
  vector aligns with the target within 2 degrees while the lower body maintains its locomotion pose.

## R-9.3.6 Motion Matching

The engine **SHALL** select the best animation continuation from a pose database by matching the
current pose and desired trajectory, with CPU-side search and GPU-evaluated blending.

- **Derived from:** [F-9.3.6](../../features/animation/procedural.md)
- **Rationale:** Motion matching replaces hand-authored state machine locomotion with data-driven
  selection, producing more natural transitions from motion capture data.
- **Verification:** Build a pose database from 100 locomotion clips. Issue a direction change from
  forward to 90-degree strafe and verify motion matching selects a transition clip within 2 frames
  that produces no visible foot sliding. Measure search time and verify it completes within 0.5 ms
  per character on reference hardware.

## R-9.3.7 Foot Placement and Procedural Locomotion

The engine **SHALL** raycast from foot bone positions to the ground surface and adjust leg IK
targets to plant feet on uneven terrain, adjusting pelvis height to maintain natural leg extension
and adapting stride timing based on movement speed and terrain gradient.

- **Derived from:** [F-9.3.7](../../features/animation/procedural.md)
- **Rationale:** Foot placement prevents floating feet and ground penetration on uneven surfaces
  without requiring per-terrain animation authoring.
- **Verification:** Walk a character across a staircase with 20 cm steps and verify both feet
  contact the stair surfaces with no penetration exceeding 1 cm and no floating exceeding 2 cm. Walk
  across a 30-degree slope and verify stride timing adapts (shorter strides uphill, longer strides
  downhill) with no visible foot sliding.

## R-9.3.8 Multi-Skeleton Procedural Locomotion

The engine **SHALL** generate procedural locomotion for arbitrary skeleton topologies (biped,
quadruped, hexapod, centaur, serpentine) using configurable locomotion profiles that define leg
groups, phase offsets, gait patterns, stride length curves, and body sway parameters, all driven by
ECS components.

- **Derived from:** [F-9.3.8](../../features/animation/procedural.md)
- **Rationale:** Multi-skeleton locomotion enables diverse creature types to move believably without
  authoring per-species animation sets for every terrain and speed combination.
- **Verification:** Configure locomotion profiles for a biped, quadruped, and hexapod. Walk all
  three across the same uneven terrain at the same speed. Verify each uses its correct gait pattern
  (biped alternating, quadruped trotting, hexapod tripod gait). Increase speed and verify gait
  transitions occur (quadruped trot to gallop). Verify all locomotion state is stored in ECS
  components by querying the component store directly.

## R-9.3.9 Physics-Based Locomotion

The engine **SHALL** drive locomotion through physical forces (torques and ground reaction forces)
via the physics system, with configurable muscle strength, joint damping, and balance PID gains,
supporting smooth transitions between animated and physics-driven locomotion.

- **Derived from:** [F-9.3.9](../../features/animation/procedural.md)
- **Rationale:** Physics-based locomotion produces emergent responses to slopes, obstacles, and
  impacts that animation-driven locomotion cannot replicate.
- **Verification:** Enable physics-based locomotion on a biped and walk up a 20-degree slope. Verify
  the character leans forward and maintains balance without falling. Apply a lateral impulse and
  verify the character stumbles and recovers. Transition from animated to physics-driven locomotion
  over 0.3 seconds and verify no visible discontinuity in the pose.

## R-9.3.10 Procedural Attachment and Dismemberment

The engine **SHALL** support runtime attachment and removal of skeletal sub-hierarchies at defined
socket points, spawning detached sub-meshes as independent physics-simulated entities and adapting
the remaining skeleton's locomotion to compensate for missing limbs.

- **Derived from:** [F-9.3.10](../../features/animation/procedural.md)
- **Rationale:** Runtime attachment and dismemberment enable dynamic character customization and
  combat feedback without pre-authored per-configuration assets.
- **Verification:** Attach a weapon to a hand socket and verify it tracks the hand transform within
  0.01 units each frame. Sever a quadruped's leg and verify the detached leg spawns as an
  independent ragdoll entity. Verify the remaining skeleton switches to a three-legged gait within 1
  second of dismemberment. Confirm attachment and detachment operations use ECS command buffers by
  tracing the command buffer log.

## R-9.3.11 Locomotion Diagnostics and Visualization

The engine **SHALL** provide a debug visualization overlay for procedural locomotion displaying foot
placement targets, IK chain states, ground contact normals, gait phase diagrams, balance indicators,
and force vectors, with per-entity and per-feature toggles.

- **Derived from:** [F-9.3.11](../../features/animation/procedural.md)
- **Rationale:** Debug visualization is essential for artists and engineers to diagnose foot
  placement errors, IK failures, and physics force imbalances during locomotion tuning.
- **Verification:** Enable foot placement visualization and verify predicted and actual foot
  positions render as distinct markers. Enable IK chain visualization and verify bone axes and joint
  limits render correctly for a 6-bone chain. Toggle visualization off for a specific entity and
  verify its overlays disappear while other entities retain theirs.
