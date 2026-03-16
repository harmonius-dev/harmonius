# R-4.1 — Rigid Body Dynamics Requirements

## F-4.1.1 Deterministic Fixed-Timestep Integration

### R-4.1.1 Fixed-Timestep Accumulator

The engine **SHALL** decouple physics tick rate from render frame rate using a fixed-timestep
accumulator, executing the `IntegrationSystem` at a constant frequency (default 60 Hz) regardless of
frame timing variation.

- **Derived from:**
  [F-4.1.1](../../features/physics/rigid-body-dynamics.md)
- **Rationale:** Variable frame rates must not affect simulation outcome; a fixed accumulator
  guarantees consistent physics regardless of rendering performance.
- **Verification:** Run a 1000-frame simulation at 30 fps, 60 fps, and 144 fps render rates. Assert
  all `Transform`, `Velocity`, and `AngularVelocity` values are bit-equal across all three runs.

### R-4.1.1a Integration Methods

The engine **SHALL** support symplectic Euler and Verlet integration methods, selectable via the
`PhysicsConfig` ECS resource. The `IntegrationSystem` **SHALL** query
`(RigidBody, &mut Velocity, &mut AngularVelocity, &ExternalForce, &ExternalTorque, &Mass, &Inertia)`
each tick to update velocity and advance position and orientation on the `Transform` component.

- **Derived from:**
  [F-4.1.1](../../features/physics/rigid-body-dynamics.md)
- **Rationale:** Both methods offer different stability and accuracy trade-offs; symplectic Euler is
  cheaper while Verlet provides better energy conservation.
- **Verification:** Unit test: run identical 500-tick scenarios with each method. Assert symplectic
  Euler and Verlet produce distinct but physically plausible trajectories. Verify energy drift is
  below 0.1% per second for Verlet.

### R-4.1.1b Cross-Platform Determinism

The engine **SHALL** produce bit-identical simulation results across Windows (x86-64), macOS
(ARM64), and Linux (x86-64) given identical initial state, timestep, and entity ordering. IEEE 754
strict compliance **SHALL** be enforced with fast-math optimizations disabled on all platforms.

- **Derived from:**
  [F-4.1.1](../../features/physics/rigid-body-dynamics.md)
- **Rationale:** Server-authoritative simulation with client-side prediction requires cross-platform
  reproducibility; any divergence causes desync.
- **Verification:** Run a 1000-step benchmark scenario on all three platforms. Compare serialized
  component state after each step. Assert bit-equality across platforms.

### R-4.1.1c Integration Throughput

The `IntegrationSystem` **SHALL** complete within 1 ms for 2000 active rigid bodies on minimum-spec
hardware.

- **Derived from:**
  [F-4.1.1](../../features/physics/rigid-body-dynamics.md)
- **Rationale:** Integration must consume only a fraction of the 4 ms physics budget to leave room
  for broadphase, narrowphase, and constraint solving.
- **Verification:** Benchmark: simulate 2000 active rigid bodies. Measure `IntegrationSystem`
  wall-clock time. Assert it completes within 1 ms.

### R-4.1.1d ECS-Only Physics Data

All rigid body simulation data (position, velocity, angular velocity, force, torque, mass, inertia)
**SHALL** be stored as ECS components. There **SHALL** be no separate physics world or
physics-private data structures.

- **Derived from:**
  [F-4.1.1](../../features/physics/rigid-body-dynamics.md)
- **Rationale:** A unified ECS data model ensures all systems (rendering, networking, gameplay)
  access physics state through the same query API without synchronization.
- **Verification:** Code review: confirm no physics module allocates a standalone physics world.
  Integration test: verify all physics state is accessible via standard ECS queries from a gameplay
  system.

---

## US-4.1.1.1 Configure Fixed Timestep in Project Settings

**As a** designer (P-5), **I want to** set the physics fixed timestep to 1/60s in the project
settings panel, **so that** physics runs at a consistent 60Hz regardless of frame rate.

## US-4.1.1.2 Select Integration Method

**As a** designer (P-5), **I want to** choose between symplectic Euler and Verlet integration
methods from the project physics settings, **so that** I can pick the method best suited to the
game's stability and accuracy needs.

## US-4.1.1.3 Apply External Forces via ECS Components

**As a** game developer (P-15), **I want to** add `ExternalForce` and `ExternalTorque` components to
any rigid body entity and have the integration system read them each tick, **so that** I can drive
physics behavior through standard ECS composition.

## US-4.1.1.4 Verify Deterministic Simulation Output

**As an** engine tester (P-27), **I want to** run the same 1000-frame simulation with 100 rigid
bodies twice with identical inputs and assert all `Transform`, `Velocity`, and `AngularVelocity`
values are bit-equal, **so that** determinism is confirmed for server-authoritative gameplay.

## US-4.1.1.5 Verify Cross-Platform Determinism

**As an** engine tester (P-27), **I want to** run a 1000-frame physics simulation on Windows, macOS,
and Linux and compare per-frame state snapshots, **so that** cross-platform determinism is verified
for networked gameplay.

## US-4.1.1.6 Experience Consistent Physics at Variable Frame Rates

**As a** player (P-23), **I want** physics objects to behave identically whether I am running at
30fps or 144fps, **so that** gameplay feels fair regardless of my hardware.

## US-4.1.1.7 Implement IEEE 754 Strict Compliance

**As an** engine developer (P-26), **I want to** disable fast-math optimizations and enforce strict
IEEE 754 rounding modes on all platforms, **so that** floating-point determinism is guaranteed
across compilers and architectures.

## US-4.1.1.8 Implement Fixed-Timestep Accumulator

**As an** engine developer (P-26), **I want to** implement a fixed-timestep accumulator that
decouples physics tick rate from render frame rate, **so that** the `IntegrationSystem` runs at a
constant frequency regardless of frame timing variation.

## US-4.1.1.9 Benchmark Integration System Throughput

**As an** engine tester (P-27), **I want to** benchmark the `IntegrationSystem` with 2000 active
rigid bodies and verify it completes within 1ms, **so that** integration performance stays within
the physics frame budget.

---

## F-4.1.2 Simulation Substeps

### R-4.1.2 Configurable Substep Loop

The engine **SHALL** subdivide each physics tick into a configurable number of substeps controlled
by a `PhysicsConfig` ECS resource. Each substep **SHALL** re-execute integration, broadphase,
narrowphase, and constraint-solve systems in sequence.

- **Derived from:**
  [F-4.1.2](../../features/physics/rigid-body-dynamics.md)
- **Rationale:** Higher substep counts improve constraint convergence and stacking stability at the
  cost of CPU time; the count must be tunable per project.
- **Verification:** Instrument each physics sub-system to count invocations per tick at substep
  counts 1, 4, and 8. Assert invocation count equals the configured substep count for each
  sub-system.

### R-4.1.2a Per-Entity Substep Override

The engine **SHALL** support a `SubstepOverride` component on individual entities to increase their
substep count beyond the global default.

- **Derived from:**
  [F-4.1.2](../../features/physics/rigid-body-dynamics.md)
- **Rationale:** Critical objects like player vehicles need higher fidelity without increasing
  global substep cost for all bodies.
- **Verification:** Unit test: set global substeps to 4. Attach `SubstepOverride(8)` to one entity.
  Assert the overridden entity is processed 8 times while others are processed 4 times per tick.

### R-4.1.2b Linear Substep Cost Scaling

Physics tick time **SHALL** scale linearly with substep count within 10% of the expected multiplier.

- **Derived from:**
  [F-4.1.2](../../features/physics/rigid-body-dynamics.md)
- **Rationale:** Predictable scaling enables designers to budget CPU time when adjusting substep
  counts.
- **Verification:** Benchmark: measure tick time at 1, 4, 8, and 16 substeps with 500 active bodies.
  Assert the ratio of tick times matches the substep ratio within 10%.

---

## US-4.1.2.1 Configure Global Substep Count

**As a** designer (P-5), **I want to** set the global substep count in the `PhysicsConfig` resource
via the editor, **so that** I can trade simulation quality for performance project-wide.

## US-4.1.2.2 Override Substep Count Per Entity

**As a** game developer (P-15), **I want to** attach a `SubstepOverride` component to specific
entities to increase their substep count beyond the global default, **so that** critical objects
like player vehicles get higher simulation fidelity.

## US-4.1.2.3 Verify Substep Execution Count

**As an** engine tester (P-27), **I want to** instrument each physics sub-system to count
invocations per tick at substep counts of 1, 4, and 8, and assert the count equals the configured
value, **so that** substep execution correctness is verified.

## US-4.1.2.4 Implement Substep Loop in ECS Schedule

**As an** engine developer (P-26), **I want to** implement the substep loop that re-executes
integration, broadphase, narrowphase, and constraint-solve systems N times per tick, **so that**
higher fidelity simulation is available through configuration.

## US-4.1.2.5 Benchmark Substep Scaling

**As an** engine tester (P-27), **I want to** benchmark physics tick time at 1, 4, 8, and 16
substeps with 500 active bodies and verify linear scaling, **so that** substep cost is predictable
for budget planning.

## US-4.1.2.6 Experience Stable Stacking at High Substep Counts

**As a** player (P-23), **I want** stacked crates and barrels to remain stable without jitter or
collapse, **so that** physics interactions look believable during gameplay.

---

## F-4.1.3 Contact Resolution with Restitution and Friction

### R-4.1.3 Impulse-Based Contact Solver

The engine **SHALL** resolve contacts using an impulse-based `ContactSolverSystem` that reads
`ContactManifold` and `PhysicsMaterial` components and writes solved impulses to `Velocity` and
`AngularVelocity` components.

- **Derived from:**
  [F-4.1.3](../../features/physics/rigid-body-dynamics.md)
- **Rationale:** Impulse-based resolution is efficient and integrates naturally with the ECS query
  model, avoiding external solver state.
- **Verification:** Unit test: create two colliding spheres with known mass and velocity. Assert
  post-solve velocities match the analytical impulse solution within 0.1%.

### R-4.1.3a Material Combination Rules

The engine **SHALL** support material combination modes (average, min, max, multiply) stored in a
`MaterialCombinationRules` ECS resource for resolving effective friction and restitution from two
colliders' `PhysicsMaterial` components.

- **Derived from:**
  [F-4.1.3](../../features/physics/rigid-body-dynamics.md)
- **Rationale:** Different combination rules suit different game styles; designers need control over
  how surfaces interact.
- **Verification:** Unit test: test each combine mode with known friction and restitution values.
  Assert combined values match expected results within floating-point epsilon.

### R-4.1.3b Elastic Rebound Accuracy

A sphere dropped onto a plane with restitution 1.0 **SHALL** rebound to within 1% of its original
height.

- **Derived from:**
  [F-4.1.3](../../features/physics/rigid-body-dynamics.md)
- **Rationale:** Perfect elastic collisions must be accurately simulated to provide physically
  correct behavior.
- **Verification:** Drop a 1 kg sphere from 10 m onto a plane with restitution 1.0. Assert rebound
  height is within 0.1 m of 10 m.

### R-4.1.3c Static Friction Correctness

A rigid body on a slope with static friction coefficient exceeding tan(slope_angle) **SHALL** remain
stationary with zero displacement over 500 ticks.

- **Derived from:**
  [F-4.1.3](../../features/physics/rigid-body-dynamics.md)
- **Rationale:** Static friction must prevent sliding on slopes below the friction threshold to
  produce physically correct resting behavior.
- **Verification:** Place a box on a 30-degree slope with static friction > tan(30). Simulate 500
  ticks. Assert zero displacement.

---

## US-4.1.3.1 Assign Physics Materials to Colliders

**As a** game developer (P-15), **I want to** attach a `PhysicsMaterial` component to any collider
entity specifying restitution and friction coefficients, **so that** surface properties drive
contact response behavior.

## US-4.1.3.2 Configure Material Combination Rules

**As a** designer (P-5), **I want to** choose the material combination mode (average, min, max,
multiply) in the editor, **so that** I can control how different surface materials interact during
collisions.

## US-4.1.3.3 Verify Elastic Rebound Accuracy

**As an** engine tester (P-27), **I want to** drop a sphere onto a plane with restitution 1.0 and
verify it rebounds to within 1% of its original height, **so that** elastic collision accuracy is
confirmed.

## US-4.1.3.4 Verify Static Friction Holds on Slopes

**As an** engine tester (P-27), **I want to** place a box on a 30-degree slope with static friction
exceeding tan(30) and verify zero displacement over 500 ticks, **so that** friction correctness is
validated.

## US-4.1.3.5 Experience Realistic Surface Responses

**As a** player (P-23), **I want** a rubber ball to bounce high off stone and a metal crate to slide
on ice, **so that** different surfaces feel physically distinct during gameplay.

## US-4.1.3.6 Implement Impulse-Based Contact Solver

**As an** engine developer (P-26), **I want to** implement the `ContactSolverSystem` that reads
`ContactManifold` and `PhysicsMaterial` components and writes solved impulses back to `Velocity` and
`AngularVelocity`, **so that** contact resolution runs entirely through ECS queries.

## US-4.1.3.7 Set Up Material Properties in Editor

**As a** level designer (P-6), **I want to** assign physics materials (ice, wood, metal) to floor
and wall colliders in the level editor, **so that** different zones have distinct surface properties
without code changes.

---

## F-4.1.4 Continuous Collision Detection

### R-4.1.4 Swept-Volume CCD

The engine **SHALL** perform swept-volume time-of-impact queries for entities carrying a
`CcdEnabled` marker component, preventing tunneling through thin geometry. The `CcdSystem` **SHALL**
query `(CcdEnabled, &Velocity, &Collider, &Transform)` and insert sub-step corrections into
`Velocity` and `Transform`.

- **Derived from:**
  [F-4.1.4](../../features/physics/rigid-body-dynamics.md)
- **Rationale:** Fast-moving objects (projectiles, vehicles) must not pass through thin walls even
  when their per-frame displacement exceeds the wall thickness.
- **Verification:** Fire a CCD-enabled 0.1 m sphere at 500 m/s toward a 0.01 m-thick wall. Assert a
  `ContactManifold` is generated and the sphere does not penetrate.

### R-4.1.4a CCD Performance Budget

The `CcdSystem` **SHALL** complete within 0.5 ms for up to 256 CCD-enabled entities on desktop
hardware.

- **Derived from:**
  [F-4.1.4](../../features/physics/rigid-body-dynamics.md)
- **Rationale:** CCD is expensive per entity; it must fit within a bounded budget to avoid consuming
  the entire physics frame.
- **Verification:** Benchmark: enable CCD on 256 entities. Measure `CcdSystem` wall-clock time.
  Assert it completes within 0.5 ms.

---

## US-4.1.4.1 Enable CCD on Fast-Moving Entities

**As a** game developer (P-15), **I want to** add a `CcdEnabled` marker component to projectile
entities, **so that** they do not tunnel through thin walls at high velocities.

## US-4.1.4.2 Verify CCD Prevents Tunneling

**As an** engine tester (P-27), **I want to** fire a CCD-enabled 0.1m sphere at 500 m/s toward a
0.01m-thick wall and assert that a `ContactManifold` is generated, **so that** tunneling prevention
is confirmed.

## US-4.1.4.3 Experience Reliable Projectile Hits

**As a** player (P-23), **I want** arrows and bullets to always hit walls and enemies without
passing through them, **so that** combat feels reliable.

## US-4.1.4.4 Implement Swept-Volume Time-of-Impact Queries

**As an** engine developer (P-26), **I want to** implement the `CcdSystem` that performs swept tests
against broadphase candidates and inserts sub-step corrections into `Velocity` and `Transform`,
**so that** fast objects are caught before tunneling.

## US-4.1.4.5 Benchmark CCD Cost at Platform Limits

**As an** engine tester (P-27), **I want to** benchmark CCD with 256 enabled entities on desktop and
verify the system completes within 0.5ms, **so that** CCD cost stays within budget at the platform
entity cap.

## US-4.1.4.6 Configure CCD Entity Budget Per Platform

**As a** designer (P-5), **I want to** see the platform-specific CCD entity limit in the editor and
receive a warning when I exceed it, **so that** I stay within performance budgets.

---

## F-4.1.5 Simulation Islands

### R-4.1.5 Union-Find Island Computation

The engine **SHALL** partition interacting rigid bodies into disjoint simulation islands using a
union-find algorithm over entities linked by `ContactManifold` and `JointConstraint` components.
Each entity **SHALL** receive an `Island` component identifying its island.

- **Derived from:**
  [F-4.1.5](../../features/physics/rigid-body-dynamics.md)
- **Rationale:** Island partitioning enables parallel constraint solving and allows sleeping entire
  groups of resting bodies.
- **Verification:** Create two groups of 50 bodies with no contacts between groups. Assert exactly
  two distinct `Island` component values are assigned.

### R-4.1.5a Parallel Island Solving

Independent islands **SHALL** be dispatched to separate worker threads via the ECS job system for
parallel constraint solving.

- **Derived from:**
  [F-4.1.5](../../features/physics/rigid-body-dynamics.md)
- **Rationale:** Multi-core CPUs must be utilized during constraint solving to meet frame-time
  budgets with many interacting bodies.
- **Verification:** Create 4 independent islands with 50 bodies each. Verify at least 2 worker
  threads are used during solving. Benchmark: verify near-linear speedup (at least 1.8x on 2 cores
  vs 1 core).

---

## US-4.1.5.1 Verify Island Partitioning

**As an** engine tester (P-27), **I want to** create two groups of 50 rigid bodies with no contacts
between groups and assert exactly two distinct `Island` component values, **so that** island
partitioning correctness is confirmed.

## US-4.1.5.2 Verify Parallel Island Solving

**As an** engine tester (P-27), **I want to** verify that independent islands are dispatched to at
least two worker threads during solving, **so that** parallel island solving is confirmed.

## US-4.1.5.3 Implement Union-Find Island Computation

**As an** engine developer (P-26), **I want to** implement the `IslandComputeSystem` using a
union-find over entities linked by `ContactManifold` and `JointConstraint` components, **so that**
interacting body groups are identified each frame.

## US-4.1.5.4 Implement Parallel Island Solve Dispatch

**As an** engine developer (P-26), **I want to** dispatch independent island solves to separate
worker threads via the ECS job system, **so that** multi-core CPUs are fully utilized during
constraint solving.

## US-4.1.5.5 Benchmark Island System Scaling

**As an** engine tester (P-27), **I want to** benchmark island computation with 1024 islands and 256
bodies per island on desktop, **so that** scaling behavior is characterized at platform limits.

## US-4.1.5.6 Experience Smooth Multi-Body Interactions

**As a** player (P-23), **I want** piles of objects in different parts of the world to simulate
independently without affecting each other's performance, **so that** physics remains responsive in
large scenes.

---

## F-4.1.6 Body Sleeping and Wake-Up

### R-4.1.6 Sleep Activation via Marker Component

The engine **SHALL** add a `Sleeping` marker component to rigid bodies whose `Velocity` and
`AngularVelocity` remain below configurable thresholds (stored in `SleepConfig`) for a sustained
period tracked by a `SleepTimer` component. The `SleepSystem` **SHALL** skip `Sleeping` entities
during integration and solving.

- **Derived from:**
  [F-4.1.6](../../features/physics/rigid-body-dynamics.md)
- **Rationale:** Sleeping eliminates simulation cost for resting bodies, which typically outnumber
  active bodies in large scenes.
- **Verification:** Simulate a body coming to rest. Assert `Sleeping` is added after the configured
  timer elapses. Assert the body is excluded from integration queries.

### R-4.1.6a Wake-Up via ECS Change Detection

The engine **SHALL** automatically remove the `Sleeping` marker within one tick when
`ExternalForce`, `ExternalTorque`, or `ContactManifold` components change on or near the sleeping
entity, using ECS change detection.

- **Derived from:**
  [F-4.1.6](../../features/physics/rigid-body-dynamics.md)
- **Rationale:** Bodies must wake immediately when disturbed to avoid visually frozen objects
  ignoring impacts.
- **Verification:** Apply `ExternalForce` to a sleeping body. Assert `Sleeping` is removed within
  one tick. Drop an active body onto a sleeping body. Assert the sleeping body wakes within one
  tick.

### R-4.1.6b Sleep Performance Savings

Physics tick cost for 10,000 sleeping bodies **SHALL** be at least 80% lower than for 10,000 active
bodies.

- **Derived from:**
  [F-4.1.6](../../features/physics/rigid-body-dynamics.md)
- **Rationale:** Sleep must deliver substantial cost reduction to justify its complexity; 80%
  savings reflects skipping integration, broadphase, and solving.
- **Verification:** Benchmark: compare tick cost with 10,000 sleeping vs 10,000 active bodies.
  Assert at least 80% cost reduction.

---

## US-4.1.6.1 Configure Sleep Thresholds

**As a** designer (P-5), **I want to** adjust the velocity and energy sleep thresholds in the
`SleepConfig` resource via the editor, **so that** I can tune how quickly objects go to sleep.

## US-4.1.6.2 Verify Sleep Activation

**As an** engine tester (P-27), **I want to** simulate a body coming to rest and assert that the
`Sleeping` marker is added after the configured timer elapses, **so that** sleep activation timing
is correct.

## US-4.1.6.3 Verify Wake-Up on Force Application

**As an** engine tester (P-27), **I want to** apply an `ExternalForce` to a sleeping body and assert
that `Sleeping` is removed within one tick, **so that** wake-up responsiveness is verified.

## US-4.1.6.4 Verify Wake-Up on Contact

**As an** engine tester (P-27), **I want to** drop an active body onto a sleeping body and assert
the sleeping body wakes within one tick via `ContactManifold` change detection, **so that**
contact-triggered wake-up works correctly.

## US-4.1.6.5 Benchmark Sleep Performance Savings

**As an** engine tester (P-27), **I want to** compare tick cost with 10000 sleeping versus 10000
active bodies and verify at least 80% cost reduction, **so that** sleep optimization effectiveness
is quantified.

## US-4.1.6.6 Implement Sleep System with ECS Change Detection

**As an** engine developer (P-26), **I want to** implement the `SleepSystem` that skips sleeping
entities during integration and uses ECS change detection on force and contact components for
wake-up, **so that** sleep is driven purely by component state.

## US-4.1.6.7 Experience Objects at Rest Not Jittering

**As a** player (P-23), **I want** settled objects to remain perfectly still without visual jitter,
**so that** the physics world looks stable and believable.

## US-4.1.6.8 Set Aggressive Sleep Thresholds on Mobile

**As a** designer (P-5), **I want** mobile platforms to use higher default sleep thresholds than
desktop, **so that** more bodies sleep and CPU budget is preserved on constrained devices.

---

## F-4.1.7 Cross-Zone Physics Continuity

### R-4.1.7 Zone Migration with Physics Preservation

The engine **SHALL** transfer entities and all physics components (`RigidBody`, `Velocity`,
`AngularVelocity`, `Mass`, `Inertia`, `Collider`) to the destination world's ECS when `Transform`
crosses a zone boundary. Velocity values **SHALL** differ by less than 0.1% before and after
migration.

- **Derived from:**
  [F-4.1.7](../../features/physics/rigid-body-dynamics.md)
- **Rationale:** Seamless open worlds require physics continuity across streaming boundaries;
  visible discontinuities break immersion.
- **Verification:** Move a rigid body at constant velocity across a zone boundary. Assert velocity
  values differ by less than 0.1% before and after migration.

### R-4.1.7a Contact State Preservation

No `ContactManifold` data **SHALL** be lost when an entity migrates between zones.

- **Derived from:**
  [F-4.1.7](../../features/physics/rigid-body-dynamics.md)
- **Rationale:** Losing contact state causes objects in contact to briefly separate or
  interpenetrate during zone transitions.
- **Verification:** Migrate a pair of contacting bodies across a zone boundary. Assert
  `ContactManifold` data is identical before and after migration.

---

## US-4.1.7.1 Verify Momentum Preservation During Migration

**As an** engine tester (P-27), **I want to** move a rigid body at constant velocity across a zone
boundary and assert velocity values differ by less than 0.1% before and after migration, **so that**
momentum preservation is confirmed.

## US-4.1.7.2 Verify Contact State Preservation

**As an** engine tester (P-27), **I want to** verify that no `ContactManifold` data is lost when an
entity migrates between zones, **so that** contact continuity is maintained.

## US-4.1.7.3 Implement Zone Migration System

**As an** engine developer (P-26), **I want to** implement the `ZoneMigrationSystem` that transfers
entities and all physics components to the destination world's ECS when `Transform` crosses a zone
boundary, **so that** streaming and physics are integrated seamlessly.

## US-4.1.7.4 Experience Seamless Physics Across Zone Boundaries

**As a** player (P-23), **I want** rolling boulders and thrown objects to cross zone boundaries
without visible pauses or direction changes, **so that** the open world feels continuous.

## US-4.1.7.5 Place Objects Near Zone Boundaries Without Issues

**As a** level designer (P-6), **I want to** place physics objects near streaming zone boundaries
and know they will migrate correctly, **so that** I do not need to avoid boundary regions during
level layout.

---

## F-4.1.8 Character Controller

### R-4.1.8 Kinematic Character Controller Components

The engine **SHALL** provide a kinematic character controller composed of `CharacterController`,
`GroundState`, `StepHeight`, and `SlopeLimit` ECS components with a dedicated movement system that
queries the shared spatial index for ground detection.

- **Derived from:**
  [F-4.1.8](../../features/physics/rigid-body-dynamics.md)
- **Rationale:** Character movement is the most common physics interaction; it must be composable
  from standard ECS components.
- **Verification:** Attach all four components to an entity. Verify the movement system produces a
  `DesiredVelocity` component based on ground detection results.

### R-4.1.8a Slope Rejection

The character controller **SHALL** reject slopes exceeding the configured `SlopeLimit` angle,
causing the character to slide on surfaces steeper than the limit.

- **Derived from:**
  [F-4.1.8](../../features/physics/rigid-body-dynamics.md)
- **Rationale:** Characters must not walk up sheer cliffs; slope limits enforce traversable terrain
  boundaries.
- **Verification:** Place a character on a 50-degree slope with a 45-degree limit. Assert the
  character slides. Place on a 40-degree slope. Assert the character holds.

### R-4.1.8b Step Climbing

The character controller **SHALL** automatically climb steps up to the height configured in the
`StepHeight` component and block movement at steps exceeding that height.

- **Derived from:**
  [F-4.1.8](../../features/physics/rigid-body-dynamics.md)
- **Rationale:** Stairs and curbs must be traversable without special level markup or designer
  intervention.
- **Verification:** Place a character at a 0.3 m step with `StepHeight` of 0.35 m. Assert it climbs.
  Place at a 0.4 m step. Assert it is blocked.

### R-4.1.8c Shared Spatial Index Queries

Character controller ground detection **SHALL** use shape casts against the shared BVH spatial
index, not a physics-private acceleration structure.

- **Derived from:**
  [F-4.1.8](../../features/physics/rigid-body-dynamics.md)
- **Rationale:** The shared spatial index eliminates redundant data structures across physics,
  rendering, and networking.
- **Verification:** Verify ground detection queries use the shared BVH resource. Assert no
  physics-private spatial structure is allocated for character queries.

### R-4.1.8d Character Controller Throughput

Character controller movement (ground detection, slope rejection, step climbing) **SHALL** complete
within 0.1 ms per character entity, supporting at least 200 simultaneous controllers within the 4 ms
physics budget.

- **Derived from:**
  [F-4.1.8](../../features/physics/rigid-body-dynamics.md)
- **Rationale:** Multiplayer and AI-heavy scenes run hundreds of controllers; each must be
  lightweight.
- **Verification:** Benchmark: simulate 200 character controllers on varied terrain. Assert total
  system time stays below 20 ms (0.1 ms per controller).

---

## US-4.1.8.1 Set Up Character Controller Components

**As a** game developer (P-15), **I want to** add `CharacterController`, `GroundState`,
`StepHeight`, and `SlopeLimit` components to a character entity, **so that** the character has a
complete kinematic movement setup through ECS composition.

## US-4.1.8.2 Configure Slope Limit in Editor

**As a** designer (P-5), **I want to** set the maximum walkable slope angle on the `SlopeLimit`
component via the editor, **so that** characters slide on steep surfaces.

## US-4.1.8.3 Configure Step Height in Editor

**As a** designer (P-5), **I want to** set the maximum step-up height on the `StepHeight` component
via the editor, **so that** characters can climb stairs and curbs smoothly.

## US-4.1.8.4 Verify Slope Rejection

**As an** engine tester (P-27), **I want to** place a character on a 50-degree slope with a
45-degree limit and verify it slides, **so that** slope rejection is functioning correctly.

## US-4.1.8.5 Verify Step Climbing

**As an** engine tester (P-27), **I want to** place a character at a 0.3m step with 0.35m step
height and verify it climbs, and at a 0.4m step and verify it is blocked, **so that** step climbing
thresholds are accurate.

## US-4.1.8.6 Verify Ground Detection via Shape Casts

**As an** engine tester (P-27), **I want to** verify that ground detection uses shape casts against
the shared spatial index, **so that** the character controller shares the same spatial data as
rendering and networking.

## US-4.1.8.7 Experience Responsive Character Movement

**As a** player (P-23), **I want** my character to move responsively over stairs, slopes, and uneven
terrain without getting stuck or jittering, **so that** navigation feels smooth.

## US-4.1.8.8 Implement Character Controller Movement System

**As an** engine developer (P-26), **I want to** implement the character controller movement system
that queries the shared spatial index for ground detection and produces a `DesiredVelocity`
component, **so that** character movement runs through the ECS physics pipeline.

## US-4.1.8.9 Set Up Moving Platform Attachment

**As a** game developer (P-15), **I want** the character controller to detect when the character is
standing on a moving platform and inherit the platform's velocity, **so that** characters ride
elevators and conveyor belts without sliding.

## US-4.1.8.10 Configure Coyote Time for Platformer Games

**As a** designer (P-5), **I want to** set a coyote-time grace period on the character controller,
**so that** jumping after walking off a ledge still works for a brief window.

## US-4.1.8.11 Benchmark Character Controller Throughput

**As an** engine tester (P-27), **I want to** benchmark 200 character controllers on varied terrain
and verify total system time stays below 0.1ms per character, **so that** controller cost is
confirmed within budget.

## US-4.1.8.12 Place Stairs and Ramps in Levels

**As a** level designer (P-6), **I want to** place stairs and ramps knowing the character controller
handles them automatically without special markup, **so that** level geometry authoring is
straightforward.

---

## F-4.1.9 Moving Platform System

### R-4.1.9 Platform Velocity Transfer

The engine **SHALL** detect entities standing on a `MovingPlatform` via ground-contact queries and
parent their velocity to the platform's velocity, inheriting both translational and rotational
motion without sliding.

- **Derived from:**
  [F-4.1.9](../../features/physics/rigid-body-dynamics.md)
- **Rationale:** Characters and objects on moving platforms must move with the platform
  automatically to avoid visually sliding off.
- **Verification:** Place a character on a platform moving at 5 m/s for 10 seconds. Assert maximum
  drift is less than 1 cm per second.

### R-4.1.9a One-Way Platform Filtering

The engine **SHALL** support one-way platforms that are passable from below and solid from above
using directional collision filtering.

- **Derived from:**
  [F-4.1.9](../../features/physics/rigid-body-dynamics.md)
- **Rationale:** Platformer-style games require jump-through floors that block downward motion only.
- **Verification:** Move a character upward through a one-way platform. Assert no collision occurs.
  Move downward onto it. Assert the platform is solid.

### R-4.1.9b Conveyor Belt Surface Velocity

The engine **SHALL** support `MovingPlatform` entities with a configurable surface velocity that
pushes entities in contact along the belt direction.

- **Derived from:**
  [F-4.1.9](../../features/physics/rigid-body-dynamics.md)
- **Rationale:** Conveyor belts are common in puzzle and industrial game environments.
- **Verification:** Place a rigid body on a conveyor with surface velocity 2 m/s. Assert the body
  accelerates in the belt direction.

### R-4.1.9c Platform State Replication

Platform position, direction, and pause state **SHALL** replicate correctly via the ECS replication
system for multiplayer.

- **Derived from:**
  [F-4.1.9](../../features/physics/rigid-body-dynamics.md)
- **Rationale:** All clients must see synchronized platform positions to prevent desync during
  traversal.
- **Verification:** Run server and client with a moving platform. Assert client platform position
  matches server within interpolation tolerance.

---

## US-4.1.9.1 Set Up Moving Platforms with Spline Paths

**As a** game developer (P-15), **I want to** attach a `MovingPlatform` component to a kinematic
entity and configure its motion via spline paths, **so that** elevators and rail carts follow
authored trajectories.

## US-4.1.9.2 Configure Conveyor Belt Surface Velocity

**As a** designer (P-5), **I want to** set a surface velocity on a conveyor belt platform,
**so that** entities in contact are pushed along the belt direction.

## US-4.1.9.3 Configure One-Way Platforms

**As a** designer (P-5), **I want to** mark a platform as one-way (pass through from below, solid
from above), **so that** platformer-style level design is supported.

## US-4.1.9.4 Verify Passenger Drift Tolerance

**As an** engine tester (P-27), **I want to** place a character on a platform moving at 5 m/s for 10
seconds and assert maximum drift is less than 1 cm per second, **so that** passenger stability is
verified.

## US-4.1.9.5 Verify Rotating Platform Passenger Stability

**As an** engine tester (P-27), **I want to** verify that characters on a rotating platform inherit
rotational motion without sliding, **so that** rotating platform behavior is correct.

## US-4.1.9.6 Experience Smooth Elevator Rides

**As a** player (P-23), **I want** elevators and moving platforms to carry me smoothly without
sliding or jitter, **so that** traversal feels polished.

## US-4.1.9.7 Implement Platform Velocity Transfer System

**As an** engine developer (P-26), **I want to** implement the system that detects entities on
platforms via ground-contact queries and parents their velocity to the platform, **so that**
passengers move with the platform automatically.

## US-4.1.9.8 Place Elevators and Conveyor Belts in Levels

**As a** level designer (P-6), **I want to** place elevator and conveyor belt entities along spline
paths in the editor, **so that** moving platforms are easy to author visually.

## US-4.1.9.9 Verify Platform State Replication

**As an** engine tester (P-27), **I want to** verify that platform position, direction, and pause
state replicate correctly for multiplayer, **so that** all clients see the same platform positions.

---

## F-4.1.10 Surface Smoothing and Ground Conformance

### R-4.1.10 Ground Smoothing Filter

The engine **SHALL** apply an exponential moving average filter to ground height and normal sampled
by shape casts, with a configurable smoothing half-life, to eliminate character jitter on triangle
edges and mesh seams.

- **Derived from:**
  [F-4.1.10](../../features/physics/rigid-body-dynamics.md)
- **Rationale:** Raw triangle-mesh surfaces cause micro-bouncing on seams; smoothing produces
  visually stable character movement.
- **Verification:** Move a character at constant speed over a mesh with 5 cm triangle-edge height
  discontinuities. Assert peak-to-peak vertical oscillation is below 1 mm.

### R-4.1.10a Step-Down Snapping

The engine **SHALL** snap the character downward by up to the configured maximum step-down distance
to stay grounded on descents without floating.

- **Derived from:**
  [F-4.1.10](../../features/physics/rigid-body-dynamics.md)
- **Rationale:** Characters must track the ground on descending slopes and stairs without visible
  air gaps.
- **Verification:** Walk a character down a 30-degree ramp. Assert the character remains grounded
  with no floating frames. Assert the character does not snap beyond the configured maximum
  distance.

### R-4.1.10b Slope Alignment

The engine **SHALL** align the character's up-vector to the filtered ground normal at a configurable
alignment speed.

- **Derived from:**
  [F-4.1.10](../../features/physics/rigid-body-dynamics.md)
- **Rationale:** Visual tilting on slopes makes characters look naturally planted on uneven terrain.
- **Verification:** Place a character on a 20-degree slope. Assert the up-vector converges to the
  slope normal within the configured alignment time. Assert no overshoot.

---

## US-4.1.10.1 Configure Smoothing Half-Life

**As a** designer (P-5), **I want to** adjust the ground smoothing half-life on the character
controller, **so that** I can balance responsiveness against smoothness on rough terrain.

## US-4.1.10.2 Configure Maximum Step-Down Distance

**As a** designer (P-5), **I want to** set the maximum step-down distance, **so that** characters
snap downward to stay grounded on descents without floating.

## US-4.1.10.3 Configure Slope Alignment Speed

**As a** designer (P-5), **I want to** set how quickly the character's up-vector aligns to the
filtered ground normal, **so that** visual tilting on slopes looks natural.

## US-4.1.10.4 Verify Smooth Traversal Over Triangle Seams

**As an** engine tester (P-27), **I want to** move a character at constant speed over a mesh with 5
cm triangle-edge height discontinuities and assert peak-to-peak vertical oscillation is below 1 mm,
**so that** smoothing effectiveness is confirmed.

## US-4.1.10.5 Experience Smooth Movement Over Rough Terrain

**As a** player (P-23), **I want** my character to move smoothly over jagged terrain without
stuttering or micro-bouncing, **so that** traversal is visually polished.

## US-4.1.10.6 Implement Ground Smoothing Filter

**As an** engine developer (P-26), **I want to** implement the exponential moving average filter for
ground height and normal, feeding the smoothed result into foot placement IK, **so that** characters
conform naturally to uneven surfaces.

## US-4.1.10.7 Author Terrain Without Worrying About Triangle Artifacts

**As a** level designer (P-6), **I want** the surface smoothing system to eliminate visible
character jitter on tessellated terrain, **so that** I do not need to hand-smooth every terrain
surface.

---

## Non-Functional Requirements

### R-4.1.NF1 Rigid Body Simulation Frame Budget

The rigid body simulation (integration, broadphase, narrowphase, constraint solve, island
management, sleeping) **SHALL** complete within 4 ms per frame on the minimum-spec target hardware
for scenes containing up to 2,000 active (non-sleeping) rigid bodies at 4 substeps.

- **Derived from:** R-4.1.1, R-4.1.2, R-4.1.5, R-4.1.6
- **Rationale:** Physics must fit within a 16.67 ms frame budget alongside rendering, audio,
  gameplay, and networking. A 4 ms allocation is a standard industry guideline for physics.
- **Verification:** Benchmark -- simulate 2,000 active rigid bodies with mixed collision and
  constraint loads at 4 substeps; measure wall-clock time per tick on minimum-spec hardware; assert
  it does not exceed 4 ms.

### R-4.1.NF2 Rigid Body Memory Budget

Peak memory consumption for rigid body simulation state (components, contact manifolds, island data,
sleep timers) **SHALL NOT** exceed 256 bytes per active rigid body entity, excluding collider shape
data.

- **Derived from:** R-4.1.1, R-4.1.5, R-4.1.6
- **Rationale:** Memory budgets must scale linearly with entity count to support large worlds with
  tens of thousands of physics objects.
- **Verification:** Profile -- spawn 10,000 rigid body entities; measure total physics component
  memory; divide by entity count; assert per-entity overhead is at most 256 bytes.

### R-4.1.NF3 Cross-Platform Determinism

Rigid body simulation **SHALL** produce bit-identical results across Windows (x86-64), macOS
(ARM64), and Linux (x86-64) given identical initial state, timestep, and entity ordering.

- **Derived from:** R-4.1.1
- **Rationale:** Server-authoritative simulation with client-side prediction requires cross-platform
  reproducibility; any divergence causes desync and rollback storms.
- **Verification:** Run a 1,000-step benchmark scenario on all three platforms; compare serialized
  component state after each step; assert bit-equality across platforms.

### R-4.1.NF4 Character Controller Latency

Character controller movement (ground detection, slope rejection, step climbing) **SHALL** complete
within 0.1 ms per character entity, supporting at least 200 simultaneous characters within the 4 ms
physics budget.

- **Derived from:** R-4.1.8, R-4.1.9, R-4.1.10
- **Rationale:** Multiplayer and AI-heavy scenes run hundreds of character controllers; each must be
  lightweight enough to avoid starving rigid body and constraint solving.
- **Verification:** Benchmark -- simulate 200 character controllers navigating varied terrain;
  measure total character-controller system time per frame; assert it remains below 0.1 ms per
  character.
