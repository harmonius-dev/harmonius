# R-4.1 — Rigid Body Dynamics Requirements

## R-4.1.1 Deterministic Fixed-Timestep Integration

The engine **SHALL** produce bit-identical rigid body simulation results across successive runs
given the same initial ECS component state, fixed timestep, and entity ordering.

- **Derived from:** [F-4.1.1](../../features/physics/rigid-body-dynamics.md)
- **Rationale:** Server-authoritative simulation and client-side rollback require reproducible
  physics results across frames and machines.
- **Verification:** Unit test — run a 1000-step simulation of 100 rigid bodies with mixed forces
  and torques twice with identical inputs; assert all `Transform`, `Velocity`, and
  `AngularVelocity` component values are bit-equal after each step.

## R-4.1.2 Simulation Substeps

The engine **SHALL** execute integration, broadphase, narrowphase, and constraint-solve ECS
systems the number of times specified by the `PhysicsConfig` substep count within a single
physics tick.

- **Derived from:** [F-4.1.2](../../features/physics/rigid-body-dynamics.md)
- **Rationale:** Higher substep counts improve constraint stability and collision accuracy for
  fast-moving or tightly coupled bodies.
- **Verification:** Integration test — configure substep counts of 1, 4, and 8; instrument each
  sub-system to count invocations per tick; assert invocation count equals the configured substep
  value.

## R-4.1.3 Contact Resolution with Restitution and Friction

The engine **SHALL** resolve contact impulses such that a sphere dropped onto a plane with
restitution 1.0 rebounds to within 1% of its original height, and a box on a slope with static
friction coefficient exceeding the slope tangent remains stationary.

- **Derived from:** [F-4.1.3](../../features/physics/rigid-body-dynamics.md)
- **Rationale:** Correct impulse-based resolution with material properties is fundamental to
  believable physics behavior across all game genres.
- **Verification:** Unit test — drop a unit sphere from height h onto a plane with restitution
  1.0 and verify rebound height is within 1% of h; place a box on a 30-degree slope with static
  friction > tan(30) and verify zero displacement over 500 ticks.

## R-4.1.4 Continuous Collision Detection

The engine **SHALL** prevent tunneling for entities with the `CcdEnabled` marker component such
that a 0.1m-radius sphere traveling at 500 m/s detects collision with a 0.01m-thick wall within
a single physics tick.

- **Derived from:** [F-4.1.4](../../features/physics/rigid-body-dynamics.md)
- **Rationale:** Fast-moving projectiles and small obstacles require swept-volume detection to
  avoid missed collisions that break gameplay.
- **Verification:** Unit test — fire a CCD-enabled sphere at 500 m/s toward a 0.01m-thick
  static wall; assert the sphere does not pass through and a `ContactManifold` is generated.

## R-4.1.5 Simulation Islands

The engine **SHALL** partition interacting rigid body entities into disjoint simulation islands
based on `ContactManifold` and `JointConstraint` connectivity, and solve independent islands in
parallel across CPU cores.

- **Derived from:** [F-4.1.5](../../features/physics/rigid-body-dynamics.md)
- **Rationale:** Island partitioning enables parallel constraint solving and is required for
  scaling to large entity counts without serial bottlenecks.
- **Verification:** Integration test — create two groups of 50 rigid bodies with no contacts or
  joints between groups; assert the `IslandComputeSystem` assigns exactly two distinct `Island`
  component values and that solver work is dispatched to at least two worker threads.

## R-4.1.6 Body Sleeping and Wake-Up

The engine **SHALL** add a `Sleeping` marker component to rigid body entities whose linear and
angular velocities remain below the `SleepConfig` thresholds for the configured duration, and
remove it within one tick when an `ExternalForce`, `ExternalTorque`, or `ContactManifold`
component is added or modified.

- **Derived from:** [F-4.1.6](../../features/physics/rigid-body-dynamics.md)
- **Rationale:** Sleeping inactive bodies eliminates wasted solver and integration work, which
  is critical for scenes with thousands of static or settled objects.
- **Verification:** Unit test — simulate a body coming to rest; assert `Sleeping` is added
  after the configured timer elapses; apply an `ExternalForce` and assert `Sleeping` is removed
  within one tick. Benchmark — compare tick cost with 10,000 sleeping vs. 10,000 active bodies.

## R-4.1.7 Cross-Zone Physics Continuity

The engine **SHALL** migrate rigid body entities across streaming zone boundaries while
preserving momentum (linear and angular velocity within 0.1% of pre-migration values) and
active contact state.

- **Derived from:** [F-4.1.7](../../features/physics/rigid-body-dynamics.md)
- **Rationale:** Seamless open-world streaming requires physics objects to cross zone boundaries
  without visible discontinuities or momentum loss.
- **Verification:** Integration test — move a rigid body at constant velocity across a zone
  boundary; assert `Velocity` and `AngularVelocity` component values differ by less than 0.1%
  before and after migration; verify no `ContactManifold` data is lost during transfer.

## R-4.1.8 Character Controller

The engine **SHALL** provide a kinematic character controller implemented as ECS components
(`CharacterController`, `GroundState`, `StepHeight`, `SlopeLimit`) that detects ground contact
via shape casts against the shared spatial index (F-1.9.4) and rejects movement up slopes
exceeding the configured `SlopeLimit` angle.

- **Derived from:** [F-4.1.8](../../features/physics/rigid-body-dynamics.md)
- **Rationale:** Nearly every game genre requires a reliable character controller that handles
  slopes, steps, and ground detection without tunneling or jitter.
- **Verification:** Integration test — place a character on a 50-degree slope (limit = 45) and
  assert it slides; place on a 30-degree slope and assert it walks normally; place at a 0.3m
  step (step height = 0.35m) and assert it climbs; place at a 0.4m step and assert it is
  blocked.

## R-4.1.9 Moving Platform System

The engine **SHALL** transfer platform velocity to all entities standing on a `MovingPlatform`
entity such that passengers maintain their relative position on the platform with less than
1 cm of drift per second of platform travel.

- **Derived from:** [F-4.1.9](../../features/physics/rigid-body-dynamics.md)
- **Rationale:** Elevators, conveyor belts, and rotating platforms are common across genres and
  must transport passengers without sliding or jitter.
- **Verification:** Integration test — place a character on a platform moving at 5 m/s for 10
  seconds; measure the character's position relative to the platform at each tick; assert
  maximum drift is less than 1 cm per second of travel.

## R-4.1.10 Surface Smoothing and Ground Conformance

The engine **SHALL** smooth character ground height and normal samples using an exponential
moving average filter such that traversal over a triangle mesh with 5 cm edge seams produces
less than 1 mm of vertical oscillation in the smoothed output.

- **Derived from:** [F-4.1.10](../../features/physics/rigid-body-dynamics.md)
- **Rationale:** Raw triangle-mesh contact produces micro-bouncing and stutter on tessellated
  terrain, degrading visual quality and foot-placement IK accuracy.
- **Verification:** Unit test — move a character at constant speed over a mesh with 5 cm
  triangle-edge height discontinuities; record smoothed ground height per frame; assert
  peak-to-peak vertical oscillation is below 1 mm.

---

## Non-Functional Requirements

### R-4.1.NF1 Rigid Body Simulation Frame Budget

The rigid body simulation (integration, broadphase, narrowphase, constraint solve, island
management, sleeping) **SHALL** complete within 4 ms per frame on the minimum-spec target
hardware for scenes containing up to 2,000 active (non-sleeping) rigid bodies at 4 substeps.

- **Derived from:** R-4.1.1, R-4.1.2, R-4.1.5, R-4.1.6
- **Rationale:** Physics must fit within a 16.67 ms frame budget alongside rendering, audio,
  gameplay, and networking. A 4 ms allocation is a standard industry guideline for physics.
- **Verification:** Benchmark — simulate 2,000 active rigid bodies with mixed collision and
  constraint loads at 4 substeps; measure wall-clock time per tick on minimum-spec hardware;
  assert it does not exceed 4 ms.

### R-4.1.NF2 Rigid Body Memory Budget

Peak memory consumption for rigid body simulation state (components, contact manifolds, island
data, sleep timers) **SHALL NOT** exceed 256 bytes per active rigid body entity, excluding
collider shape data.

- **Derived from:** R-4.1.1, R-4.1.5, R-4.1.6
- **Rationale:** Memory budgets must scale linearly with entity count to support large worlds
  with tens of thousands of physics objects.
- **Verification:** Profile — spawn 10,000 rigid body entities; measure total physics component
  memory; divide by entity count; assert per-entity overhead is at most 256 bytes.

### R-4.1.NF3 Cross-Platform Determinism

Rigid body simulation **SHALL** produce bit-identical results across Windows (x86-64),
macOS (ARM64), and Linux (x86-64) given identical initial state, timestep, and entity ordering.

- **Derived from:** R-4.1.1
- **Rationale:** Server-authoritative simulation with client-side prediction requires
  cross-platform reproducibility; any divergence causes desync and rollback storms.
- **Verification:** Run a 1,000-step benchmark scenario on all three platforms; compare
  serialized component state after each step; assert bit-equality across platforms.

### R-4.1.NF4 Character Controller Latency

Character controller movement (ground detection, slope rejection, step climbing) **SHALL**
complete within 0.1 ms per character entity, supporting at least 200 simultaneous characters
within the 4 ms physics budget.

- **Derived from:** R-4.1.8, R-4.1.9, R-4.1.10
- **Rationale:** Multiplayer and AI-heavy scenes run hundreds of character controllers; each
  must be lightweight enough to avoid starving rigid body and constraint solving.
- **Verification:** Benchmark — simulate 200 character controllers navigating varied terrain;
  measure total character-controller system time per frame; assert it remains below 0.1 ms
  per character.
