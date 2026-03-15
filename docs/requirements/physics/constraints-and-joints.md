# R-4.3 — Constraints & Joints User Stories

## F-4.3.1 Core Joint Types

## US-4.3.1.1 Create Revolute Joints

**As a** game developer (P-15), **I want to** create a revolute (hinge) joint entity with `Joint`
and `JointType` components referencing two body entities, **so that** doors and hinged panels rotate
around a single axis.

## US-4.3.1.2 Create Prismatic Joints

**As a** game developer (P-15), **I want to** create a prismatic (slider) joint, **so that**
drawbridges and sliding mechanisms move along a single axis.

## US-4.3.1.3 Create Fixed Joints

**As a** game developer (P-15), **I want to** create a fixed (weld) joint, **so that** two bodies
are rigidly locked together as a single unit.

## US-4.3.1.4 Create Distance Joints

**As a** game developer (P-15), **I want to** create a distance joint, **so that** two bodies
maintain a fixed separation like a tether or chain link.

## US-4.3.1.5 Verify Core Joint Constraint Accuracy

**As an** engine tester (P-27), **I want to** connect two 1 kg bodies with each core joint type,
apply forces for 500 ticks at 8 iterations, and assert positional drift at the anchor is below 1 mm,
**so that** all four joint types maintain constraint accuracy.

## US-4.3.1.6 Implement Constraint Solver for Core Joints

**As an** engine developer (P-26), **I want to** implement the `ConstraintSolverSystem` that queries
`(Joint, JointType)` entities and resolves positional and velocity constraints, **so that** joint
physics runs through ECS queries.

## US-4.3.1.7 Set Up Joints Between Level Objects

**As a** level designer (P-6), **I want to** connect doors, drawbridges, and mechanisms using joints
in the editor by selecting two entities and choosing a joint type, **so that** mechanical
connections are authored visually.

## US-4.3.1.8 Configure Joint Anchor Points in Editor

**As a** designer (P-5), **I want to** position joint anchor points on connected bodies in the
editor with visual handles, **so that** rotation and slide axes are placed precisely.

## US-4.3.1.9 Experience Doors Swinging on Hinges

**As a** player (P-23), **I want** doors to swing open and closed on hinges realistically, **so
that** interacting with the environment feels physically grounded.

---

## F-4.3.2 Advanced Joint Types

## US-4.3.2.1 Create Spring Joints

**As a** game developer (P-15), **I want to** create a spring joint with configurable stiffness and
damping, **so that** elastic connections like bungee cords and suspension links are possible.

## US-4.3.2.2 Create Cone-Twist Joints

**As a** game developer (P-15), **I want to** create a cone-twist joint with angular limits on all
three axes, **so that** ragdoll shoulders and turrets have anatomically plausible motion ranges.

## US-4.3.2.3 Create Generic 6DOF Joints

**As a** game developer (P-15), **I want to** create a generic 6DOF joint with per-axis locking,
limiting, and freeing via a `Dof6Config` component, **so that** arbitrary constraint configurations
are possible.

## US-4.3.2.4 Verify Cone-Twist Angular Limit Accuracy

**As an** engine tester (P-27), **I want to** create a cone-twist joint with a 45-degree limit,
apply torque past the limit, and assert the angle does not exceed 45.5 degrees over 1000 ticks, **so
that** angular limit enforcement is accurate.

## US-4.3.2.5 Implement Advanced Joint Solve Routines

**As an** engine developer (P-26), **I want to** implement specialized solve routines for `Spring`,
`ConeTwist`, and `Generic6DOF` joint variants, **so that** each type converges correctly within the
constraint solver.

## US-4.3.2.6 Configure 6DOF Joint Axes in Editor

**As a** designer (P-5), **I want to** lock, limit, or free each axis of a 6DOF joint in the editor
with visual indicators showing the allowed range, **so that** complex constraints are intuitive to
configure.

---

## F-4.3.3 Joint Motors and Limits

## US-4.3.3.1 Attach Motor to Joint

**As a** game developer (P-15), **I want to** add a `JointMotor` component to a joint entity with
target velocity and maximum force, **so that** powered doors and turrets are driven mechanically.

## US-4.3.3.2 Attach Limits to Joint

**As a** game developer (P-15), **I want to** add a `JointLimits` component with angular and linear
bounds, **so that** joints stop at defined positions.

## US-4.3.3.3 Configure Motor Speed and Force in Editor

**As a** designer (P-5), **I want to** set motor target velocity and maximum force on joint entities
in the editor, **so that** powered mechanisms are tuned without code.

## US-4.3.3.4 Verify Motor Target Velocity Accuracy

**As an** engine tester (P-27), **I want to** set a revolute joint motor target to 2 rad/s and
assert steady-state velocity is within 1% of 2 rad/s, **so that** motor drive accuracy is confirmed.

## US-4.3.3.5 Verify Joint Limit Clamping

**As an** engine tester (P-27), **I want to** set angular limits to +/-45 degrees, apply excess
torque, and assert the body does not exceed 45.5 degrees, **so that** limit enforcement is accurate.

## US-4.3.3.6 Implement Motor and Limit Constraint Rows

**As an** engine developer (P-26), **I want to** implement motor drives and limit clamping as
additional constraint rows in the solver, **so that** motors and limits integrate into the existing
solve pipeline.

## US-4.3.3.7 Experience Powered Mechanisms

**As a** player (P-23), **I want** powered doors, gates, and turrets to move at consistent speeds
and stop at their end positions, **so that** mechanical objects behave predictably.

---

## F-4.3.4 Breakable Joints

## US-4.3.4.1 Configure Break Force Thresholds

**As a** game developer (P-15), **I want to** add a `BreakForce` component to a joint entity
specifying maximum force and torque thresholds, **so that** the joint breaks under sufficient
stress.

## US-4.3.4.2 Set Up Breakable Connections in Editor

**As a** designer (P-5), **I want to** set break force thresholds on joints in the editor, **so
that** destruction sequences are tuned visually.

## US-4.3.4.3 Verify Joint Breaking

**As an** engine tester (P-27), **I want to** create a breakable joint with a 1000N threshold, apply
1500N, and assert the joint is despawned and a `JointBroken` event fires within one substep, **so
that** break detection is responsive.

## US-4.3.4.4 Verify JointBroken Event Payload

**As an** engine tester (P-27), **I want to** verify the `JointBroken` event contains both body
entity IDs and the breaking force magnitude, **so that** gameplay systems can react to breaks with
full context.

## US-4.3.4.5 Implement Break Force Check in Solver

**As an** engine developer (P-26), **I want to** check accumulated constraint impulses against
`BreakForce` each substep and despawn the joint entity via command buffer when exceeded, **so that**
breakable joints are handled within the solver.

## US-4.3.4.6 Experience Destructible Mechanical Connections

**As a** player (P-23), **I want** chains, beams, and connections to snap under heavy impacts, **so
that** destruction feels dynamic and reactive.

## US-4.3.4.7 Place Breakable Connections in Levels

**As a** level designer (P-6), **I want to** mark specific joints as breakable with tuned
thresholds, **so that** environmental destruction points are designed intentionally.

---

## F-4.3.5 Ragdoll Configuration

## US-4.3.5.1 Define Ragdoll Assets

**As a** game developer (P-15), **I want to** create a `RagdollDef` asset that maps a skeleton
hierarchy to joint archetypes with plausible limits and types, **so that** ragdolls activate from a
reusable data definition.

## US-4.3.5.2 Configure Ragdoll Joint Limits Per Bone

**As a** designer (P-5), **I want to** tune angular limits per joint in the ragdoll definition via
the editor, **so that** body part ranges of motion are anatomically plausible.

## US-4.3.5.3 Verify Ragdoll Activation Completeness

**As an** engine tester (P-27), **I want to** trigger ragdoll activation on a 20-bone skeleton and
assert all joint entities spawn with correct types and limits within one frame, **so that**
activation reliability is confirmed.

## US-4.3.5.4 Verify Ragdoll Constraint Stability

**As an** engine tester (P-27), **I want to** simulate an active ragdoll for 100 ticks and assert no
constraint violations exceed 5mm, **so that** ragdoll stability is verified.

## US-4.3.5.5 Benchmark Ragdoll Activation Latency

**As an** engine tester (P-27), **I want to** activate 8 ragdolls (20 bones each) in a single frame
and assert total time stays within 4ms, **so that** mass-casualty events do not cause hitches.

## US-4.3.5.6 Implement Ragdoll Activation System

**As an** engine developer (P-26), **I want to** implement the `RagdollActivationSystem` that spawns
joint entities from the definition when transitioning from animation to physics, **so that** ragdoll
activation is automatic.

## US-4.3.5.7 Experience Realistic Death Ragdolls

**As a** player (P-23), **I want** characters to collapse into believable ragdoll poses on death,
**so that** combat outcomes are visually impactful.

---

## F-4.3.6 Joint Chains and Ropes

## US-4.3.6.1 Create Chain Definitions

**As a** game developer (P-15), **I want to** create a `ChainDef` asset configuring segment count,
stiffness, and joint type, **so that** ropes and chains are spawned from reusable definitions.

## US-4.3.6.2 Configure Chain Segment Count and Stiffness

**As a** designer (P-5), **I want to** set the segment count and stiffness per chain asset in the
editor, **so that** rope quality and performance are balanced per use case.

## US-4.3.6.3 Verify Chain Stability

**As an** engine tester (P-27), **I want to** simulate a 32-segment chain anchored at one end under
gravity for 2000 ticks at 4 substeps and assert no gap exceeds 1mm, **so that** chain stability is
confirmed.

## US-4.3.6.4 Stress Test Long-Running Chain Simulation

**As an** engine tester (P-27), **I want to** simulate a 32-segment chain for 60 seconds and assert
energy gain stays below 1% per second and segment separation stays below 1mm, **so that** long-term
stability is verified.

## US-4.3.6.5 Implement Chain Spawning System

**As an** engine developer (P-26), **I want to** implement chain spawning that creates a sequence of
rigid body entities connected by joint entities from a `ChainDef` asset, **so that** rope-like
structures are supported.

## US-4.3.6.6 Place Ropes and Chains in Levels

**As a** level designer (P-6), **I want to** place rope and chain objects between anchor points in
the editor, **so that** hanging ropes, drawbridge chains, and swinging elements are part of the
level design.

## US-4.3.6.7 Experience Ropes and Chains Swinging

**As a** player (P-23), **I want** ropes and chains to swing and sway naturally when disturbed, **so
that** the environment feels dynamic.

---

## F-4.3.7 Constraint Solvers

## US-4.3.7.1 Select Solver Type

**As a** game developer (P-15), **I want to** choose between Sequential Impulse (SI) and Temporal
Gauss-Seidel (TGS) solvers via the `SolverConfig` resource, **so that** I can pick the solver best
suited to my project.

## US-4.3.7.2 Configure Solver Iteration Count

**As a** designer (P-5), **I want to** set the solver iteration count in the project settings, **so
that** constraint accuracy and performance are balanced.

## US-4.3.7.3 Verify TGS Drift Reduction

**As an** engine tester (P-27), **I want to** run a 10-body joint chain for 1000 ticks with both SI
and TGS at 8 iterations and assert TGS drift is at least 30% lower, **so that** TGS convergence
superiority is quantified.

## US-4.3.7.4 Benchmark Constraint Solver Throughput

**As an** engine tester (P-27), **I want to** process 5000 constraint rows per millisecond on
minimum-spec hardware and assert completion within 4ms for 500 joints at 8 iterations, **so that**
solver throughput meets requirements.

## US-4.3.7.5 Verify Solver Determinism

**As an** engine tester (P-27), **I want to** run the same constraint scenario twice and assert
bit-identical results, **so that** solver determinism is confirmed for client-side prediction.

## US-4.3.7.6 Implement SI and TGS Solvers

**As an** engine developer (P-26), **I want to** implement both SI and TGS solvers within the
`ConstraintSolverSystem`, **so that** the project can select the appropriate solver.

## US-4.3.7.7 Experience Stable Stacking and Joints

**As a** player (P-23), **I want** stacked objects and jointed mechanisms to remain stable without
visible jitter, **so that** physics interactions look solid.

---

## F-4.3.8 Limb Severance and Joint Destruction

## US-4.3.8.1 Configure Per-Joint Severance Thresholds

**As a** game developer (P-15), **I want to** configure a severance damage threshold per joint in
the skeleton asset via a `LimbHealth` component, **so that** individual limbs have independent
durability.

## US-4.3.8.2 Set Up Limb Damage in Editor

**As a** designer (P-5), **I want to** set per-limb HP and severance thresholds in the skeleton
editor, **so that** body damage is tuned visually.

## US-4.3.8.3 Verify Limb Detachment on Threshold

**As an** engine tester (P-27), **I want to** deal damage to a limb until the severance threshold is
reached and verify the limb detaches as an independent ragdoll entity within one frame, **so that**
severance responsiveness is confirmed.

## US-4.3.8.4 Verify JointSevered Event Fires

**As an** engine tester (P-27), **I want to** verify the `JointSevered` event fires with the severed
joint ID, parent entity, and spawned limb entity, **so that** gameplay systems can react to
severance.

## US-4.3.8.5 Verify Parent Skeleton Continues Functioning

**As an** engine tester (P-27), **I want to** verify that the parent skeleton adapts its locomotion
after limb severance (e.g., three-legged gait) and gameplay stat penalties are applied, **so that**
severance has mechanical consequences.

## US-4.3.8.6 Implement Limb Severance System

**As an** engine developer (P-26), **I want to** implement the severance system that destroys the
joint connection, spawns the detached bone chain as a ragdoll, spawns VFX at the separation point,
and fires the `JointSevered` event, **so that** limb severance is a complete ECS operation.

## US-4.3.8.7 Experience Limb Damage and Severance

**As a** player (P-23), **I want** progressive limb damage to visually weaken body parts before they
detach, with severed limbs falling as ragdolls, **so that** combat has visceral physical
consequences.

## US-4.3.8.8 Place Destructible Enemies with Severable Limbs

**As a** level designer (P-6), **I want to** place enemy entities with configured severable limbs,
**so that** combat encounters feature body damage without custom scripting.

---

## F-4.3.9 Prosthetic and Limb Replacement

## US-4.3.9.1 Define Prosthetic Assets

**As a** game developer (P-15), **I want to** create `ProstheticDef` assets defining replacement
mesh, bone chain, collision shapes, stat modifiers, and compatible socket types, **so that**
prosthetics are reusable data-driven assets.

## US-4.3.9.2 Configure Prosthetic Properties in Editor

**As a** designer (P-5), **I want to** set prosthetic stat modifiers (strength, weight, speed) and
compatible socket types in the editor, **so that** replacement limbs have gameplay balance.

## US-4.3.9.3 Verify Prosthetic Attachment

**As an** engine tester (P-27), **I want to** sever a limb, attach a prosthetic, and verify physics
constraints are restored and locomotion adapts to the prosthetic's parameters, **so that**
attachment correctness is confirmed.

## US-4.3.9.4 Verify Prosthetic Stat Modifiers Apply

**As an** engine tester (P-27), **I want to** verify that prosthetic stat modifiers (e.g., stronger
mechanical arm) are active through the gameplay effect system after attachment, **so that**
prosthetic gameplay impact is verified.

## US-4.3.9.5 Implement Prosthetic Attachment System

**As an** engine developer (P-26), **I want to** implement the attachment system that re-
establishes physics constraints at the socket joint, updates the bone hierarchy, and triggers
locomotion re-evaluation, **so that** prosthetic attachment is seamless.

## US-4.3.9.6 Experience Replacing Lost Limbs

**As a** player (P-23), **I want** to find and equip replacement limbs that change my character's
abilities and movement, **so that** limb loss creates interesting gameplay progression.

---

## Non-Functional Requirements

### R-4.3.NF1 Constraint Solver Throughput

The constraint solver **SHALL** process at least 5,000 constraint rows per millisecond on
minimum-spec hardware, supporting scenes with up to 500 active joints at 8 solver iterations within
the 4 ms physics budget.

- **Derived from:** R-4.3.1, R-4.3.7
- **Rationale:** Complex scenes (ragdolls, vehicles, destruction chains) can accumulate thousands of
  constraint rows; the solver must handle peak loads without frame drops.
- **Verification:** Benchmark -- create 500 active joints with 8 solver iterations; measure
  constraint solver wall-clock time; assert it completes within 4 ms.

### R-4.3.NF2 Ragdoll Activation Latency

Ragdoll activation (spawning all joint and bone entities from a `RagdollDef` asset) **SHALL**
complete within 0.5 ms per ragdoll, supporting at least 8 simultaneous ragdoll activations per frame
without exceeding the physics budget.

- **Derived from:** R-4.3.5
- **Rationale:** Mass-casualty events (explosions, area attacks) can activate multiple ragdolls in a
  single frame; activation must be fast enough to avoid hitches.
- **Verification:** Benchmark -- activate 8 ragdolls (20 bones each) in a single frame; measure
  total activation time; assert it completes within 4 ms.

### R-4.3.NF3 Joint Chain Stability

A joint chain of up to 32 segments **SHALL** remain stable (no segment separation exceeding 1 mm, no
energy gain exceeding 1% per second) for at least 60 seconds of continuous simulation at the minimum
substep count of 4.

- **Derived from:** R-4.3.6, R-4.3.7
- **Rationale:** Long-running rope and chain simulations must not accumulate drift or energy that
  causes visual artifacts or instability over time.
- **Verification:** Stress test -- simulate a 32-segment pendulum chain for 60 seconds at 4
  substeps; measure total system energy and maximum segment separation each second; assert energy
  gain is below 1% and separation is below 1 mm throughout.
