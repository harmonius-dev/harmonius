# R-4.1 — Rigid Body Dynamics Requirements

## F-4.1.1 Deterministic Fixed-Timestep Integration

| ID       | Derived From                                             |
|----------|----------------------------------------------------------|
| R-4.1.1  | [F-4.1.1](../../features/physics/rigid-body-dynamics.md) |
| R-4.1.1a | [F-4.1.1](../../features/physics/rigid-body-dynamics.md) |
| R-4.1.1b | [F-4.1.1](../../features/physics/rigid-body-dynamics.md) |
| R-4.1.1c | [F-4.1.1](../../features/physics/rigid-body-dynamics.md) |
| R-4.1.1d | [F-4.1.1](../../features/physics/rigid-body-dynamics.md) |

1. **R-4.1.1** — Fixed-Timestep Accumulator: The engine **SHALL** decouple physics tick rate from
   render frame rate using a fixed-timestep accumulator, executing the `IntegrationSystem` at a
   constant frequency (default 60 Hz) regardless of frame timing variation.
   - **Rationale:** Variable frame rates must not affect simulation outcome; a fixed accumulator
     guarantees consistent physics regardless of rendering performance.
   - **Verification:** Run a 1000-frame simulation at 30 fps, 60 fps, and 144 fps render rates.
     Assert all `Transform`, `Velocity`, and `AngularVelocity` values are bit-equal across all three
     runs.
2. **R-4.1.1a** — Integration Methods: The engine **SHALL** support symplectic Euler and Verlet
   integration methods, selectable via the `PhysicsConfig` ECS resource. The `IntegrationSystem`
   **SHALL** query
   `(RigidBody, &mut Velocity, &mut AngularVelocity, &ExternalForce, &ExternalTorque, &Mass, &Inertia)`
   each tick to update velocity and advance position and orientation on the `Transform` component.
   - **Rationale:** Both methods offer different stability and accuracy trade-offs; symplectic Euler
     is cheaper while Verlet provides better energy conservation.
   - **Verification:** Unit test: run identical 500-tick scenarios with each method. Assert
     symplectic Euler and Verlet produce distinct but physically plausible trajectories. Verify
     energy drift is below 0.1% per second for Verlet.
3. **R-4.1.1b** — Cross-Platform Determinism: The engine **SHALL** produce bit-identical simulation
   results across Windows (x86-64), macOS (ARM64), and Linux (x86-64) given identical initial state,
   timestep, and entity ordering. IEEE 754 strict compliance **SHALL** be enforced with fast-math
   optimizations disabled on all platforms.
   - **Rationale:** Server-authoritative simulation with client-side prediction requires
     cross-platform reproducibility; any divergence causes desync.
   - **Verification:** Run a 1000-step benchmark scenario on all three platforms. Compare serialized
     component state after each step. Assert bit-equality across platforms.
4. **R-4.1.1c** — Integration Throughput: The `IntegrationSystem` **SHALL** complete within 1 ms for
   2000 active rigid bodies on minimum-spec hardware.
   - **Rationale:** Integration must consume only a fraction of the 4 ms physics budget to leave
     room for broadphase, narrowphase, and constraint solving.
   - **Verification:** Benchmark: simulate 2000 active rigid bodies. Measure `IntegrationSystem`
     wall-clock time. Assert it completes within 1 ms.
5. **R-4.1.1d** — ECS-Only Physics Data: All rigid body simulation data (position, velocity, angular
   velocity, force, torque, mass, inertia) **SHALL** be stored as ECS components. There **SHALL** be
   no separate physics world or physics-private data structures.
   - **Rationale:** A unified ECS data model ensures all systems (rendering, networking, gameplay)
     access physics state through the same query API without synchronization.
   - **Verification:** Code review: confirm no physics module allocates a standalone physics world.
     Integration test: verify all physics state is accessible via standard ECS queries from a
     gameplay system.

## F-4.1.2 Simulation Substeps

| ID       | Derived From                                             |
|----------|----------------------------------------------------------|
| R-4.1.2  | [F-4.1.2](../../features/physics/rigid-body-dynamics.md) |
| R-4.1.2a | [F-4.1.2](../../features/physics/rigid-body-dynamics.md) |
| R-4.1.2b | [F-4.1.2](../../features/physics/rigid-body-dynamics.md) |

1. **R-4.1.2** — Configurable Substep Loop: The engine **SHALL** subdivide each physics tick into a
   configurable number of substeps controlled by a `PhysicsConfig` ECS resource. Each substep
   **SHALL** re-execute integration, broadphase, narrowphase, and constraint-solve systems in
   sequence.
   - **Rationale:** Higher substep counts improve constraint convergence and stacking stability at
     the cost of CPU time; the count must be tunable per project.
   - **Verification:** Instrument each physics sub-system to count invocations per tick at substep
     counts 1, 4, and 8. Assert invocation count equals the configured substep count for each
     sub-system.
2. **R-4.1.2a** — Per-Entity Substep Override: The engine **SHALL** support a `SubstepOverride`
   component on individual entities to increase their substep count beyond the global default.
   - **Rationale:** Critical objects like player vehicles need higher fidelity without increasing
     global substep cost for all bodies.
   - **Verification:** Unit test: set global substeps to 4. Attach `SubstepOverride(8)` to one
     entity. Assert the overridden entity is processed 8 times while others are processed 4 times
     per tick.
3. **R-4.1.2b** — Linear Substep Cost Scaling: Physics tick time **SHALL** scale linearly with
   substep count within 10% of the expected multiplier.
   - **Rationale:** Predictable scaling enables designers to budget CPU time when adjusting substep
     counts.
   - **Verification:** Benchmark: measure tick time at 1, 4, 8, and 16 substeps with 500 active
     bodies. Assert the ratio of tick times matches the substep ratio within 10%.

## F-4.1.3 Contact Resolution with Restitution and Friction

| ID       | Derived From                                             |
|----------|----------------------------------------------------------|
| R-4.1.3  | [F-4.1.3](../../features/physics/rigid-body-dynamics.md) |
| R-4.1.3a | [F-4.1.3](../../features/physics/rigid-body-dynamics.md) |
| R-4.1.3b | [F-4.1.3](../../features/physics/rigid-body-dynamics.md) |
| R-4.1.3c | [F-4.1.3](../../features/physics/rigid-body-dynamics.md) |

1. **R-4.1.3** — Impulse-Based Contact Solver: The engine **SHALL** resolve contacts using an
   impulse-based `ContactSolverSystem` that reads `ContactManifold` and `PhysicsMaterial` components
   and writes solved impulses to `Velocity` and `AngularVelocity` components.
   - **Rationale:** Impulse-based resolution is efficient and integrates naturally with the ECS
     query model, avoiding external solver state.
   - **Verification:** Unit test: create two colliding spheres with known mass and velocity. Assert
     post-solve velocities match the analytical impulse solution within 0.1%.
2. **R-4.1.3a** — Material Combination Rules: The engine **SHALL** support material combination
   modes (average, min, max, multiply) stored in a `MaterialCombinationRules` ECS resource for
   resolving effective friction and restitution from two colliders' `PhysicsMaterial` components.
   - **Rationale:** Different combination rules suit different game styles; designers need control
     over how surfaces interact.
   - **Verification:** Unit test: test each combine mode with known friction and restitution values.
     Assert combined values match expected results within floating-point epsilon.
3. **R-4.1.3b** — Elastic Rebound Accuracy: A sphere dropped onto a plane with restitution 1.0
   **SHALL** rebound to within 1% of its original height.
   - **Rationale:** Perfect elastic collisions must be accurately simulated to provide physically
     correct behavior.
   - **Verification:** Drop a 1 kg sphere from 10 m onto a plane with restitution 1.0. Assert
     rebound height is within 0.1 m of 10 m.
4. **R-4.1.3c** — Static Friction Correctness: A rigid body on a slope with static friction
   coefficient exceeding tan(slope_angle) **SHALL** remain stationary with zero displacement over
   500 ticks.
   - **Rationale:** Static friction must prevent sliding on slopes below the friction threshold to
     produce physically correct resting behavior.
   - **Verification:** Place a box on a 30-degree slope with static friction > tan(30). Simulate 500
     ticks. Assert zero displacement.

## F-4.1.4 Continuous Collision Detection

| ID       | Derived From                                             |
|----------|----------------------------------------------------------|
| R-4.1.4  | [F-4.1.4](../../features/physics/rigid-body-dynamics.md) |
| R-4.1.4a | [F-4.1.4](../../features/physics/rigid-body-dynamics.md) |

1. **R-4.1.4** — Swept-Volume CCD: The engine **SHALL** perform swept-volume time-of-impact queries
   for entities carrying a `CcdEnabled` marker component, preventing tunneling through thin
   geometry. The `CcdSystem` **SHALL** query `(CcdEnabled, &Velocity, &Collider, &Transform)` and
   insert sub-step corrections into `Velocity` and `Transform`.
   - **Rationale:** Fast-moving objects (projectiles, vehicles) must not pass through thin walls
     even when their per-frame displacement exceeds the wall thickness.
   - **Verification:** Fire a CCD-enabled 0.1 m sphere at 500 m/s toward a 0.01 m-thick wall. Assert
     a `ContactManifold` is generated and the sphere does not penetrate.
2. **R-4.1.4a** — CCD Performance Budget: The `CcdSystem` **SHALL** complete within 0.5 ms for up to
   256 CCD-enabled entities on desktop hardware.
   - **Rationale:** CCD is expensive per entity; it must fit within a bounded budget to avoid
     consuming the entire physics frame.
   - **Verification:** Benchmark: enable CCD on 256 entities. Measure `CcdSystem` wall-clock time.
     Assert it completes within 0.5 ms.

## F-4.1.5 Simulation Islands

| ID       | Derived From                                             |
|----------|----------------------------------------------------------|
| R-4.1.5  | [F-4.1.5](../../features/physics/rigid-body-dynamics.md) |
| R-4.1.5a | [F-4.1.5](../../features/physics/rigid-body-dynamics.md) |

1. **R-4.1.5** — Union-Find Island Computation: The engine **SHALL** partition interacting rigid
   bodies into disjoint simulation islands using a union-find algorithm over entities linked by
   `ContactManifold` and `JointConstraint` components. Each entity **SHALL** receive an `Island`
   component identifying its island.
   - **Rationale:** Island partitioning enables parallel constraint solving and allows sleeping
     entire groups of resting bodies.
   - **Verification:** Create two groups of 50 bodies with no contacts between groups. Assert
     exactly two distinct `Island` component values are assigned.
2. **R-4.1.5a** — Parallel Island Solving: Independent islands **SHALL** be dispatched to separate
   worker threads via the ECS job system for parallel constraint solving.
   - **Rationale:** Multi-core CPUs must be utilized during constraint solving to meet frame-time
     budgets with many interacting bodies.
   - **Verification:** Create 4 independent islands with 50 bodies each. Verify at least 2 worker
     threads are used during solving. Benchmark: verify near-linear speedup (at least 1.8x on 2
     cores vs 1 core).

## F-4.1.6 Body Sleeping and Wake-Up

| ID       | Derived From                                             |
|----------|----------------------------------------------------------|
| R-4.1.6  | [F-4.1.6](../../features/physics/rigid-body-dynamics.md) |
| R-4.1.6a | [F-4.1.6](../../features/physics/rigid-body-dynamics.md) |
| R-4.1.6b | [F-4.1.6](../../features/physics/rigid-body-dynamics.md) |

1. **R-4.1.6** — Sleep Activation via Marker Component: The engine **SHALL** add a `Sleeping` marker
   component to rigid bodies whose `Velocity` and `AngularVelocity` remain below configurable
   thresholds (stored in `SleepConfig`) for a sustained period tracked by a `SleepTimer` component.
   The `SleepSystem` **SHALL** skip `Sleeping` entities during integration and solving.
   - **Rationale:** Sleeping eliminates simulation cost for resting bodies, which typically
     outnumber active bodies in large scenes.
   - **Verification:** Simulate a body coming to rest. Assert `Sleeping` is added after the
     configured timer elapses. Assert the body is excluded from integration queries.
2. **R-4.1.6a** — Wake-Up via ECS Change Detection: The engine **SHALL** automatically remove the
   `Sleeping` marker within one tick when `ExternalForce`, `ExternalTorque`, or `ContactManifold`
   components change on or near the sleeping entity, using ECS change detection.
   - **Rationale:** Bodies must wake immediately when disturbed to avoid visually frozen objects
     ignoring impacts.
   - **Verification:** Apply `ExternalForce` to a sleeping body. Assert `Sleeping` is removed within
     one tick. Drop an active body onto a sleeping body. Assert the sleeping body wakes within one
     tick.
3. **R-4.1.6b** — Sleep Performance Savings: Physics tick cost for 10,000 sleeping bodies **SHALL**
   be at least 80% lower than for 10,000 active bodies.
   - **Rationale:** Sleep must deliver substantial cost reduction to justify its complexity; 80%
     savings reflects skipping integration, broadphase, and solving.
   - **Verification:** Benchmark: compare tick cost with 10,000 sleeping vs 10,000 active bodies.
     Assert at least 80% cost reduction.

## F-4.1.7 Cross-Zone Physics Continuity

| ID       | Derived From                                             |
|----------|----------------------------------------------------------|
| R-4.1.7  | [F-4.1.7](../../features/physics/rigid-body-dynamics.md) |
| R-4.1.7a | [F-4.1.7](../../features/physics/rigid-body-dynamics.md) |

1. **R-4.1.7** — Zone Migration with Physics Preservation: The engine **SHALL** transfer entities
   and all physics components (`RigidBody`, `Velocity`, `AngularVelocity`, `Mass`, `Inertia`,
   `Collider`) to the destination world's ECS when `Transform` crosses a zone boundary. Velocity
   values **SHALL** differ by less than 0.1% before and after migration.
   - **Rationale:** Seamless open worlds require physics continuity across streaming boundaries;
     visible discontinuities break immersion.
   - **Verification:** Move a rigid body at constant velocity across a zone boundary. Assert
     velocity values differ by less than 0.1% before and after migration.
2. **R-4.1.7a** — Contact State Preservation: No `ContactManifold` data **SHALL** be lost when an
   entity migrates between zones.
   - **Rationale:** Losing contact state causes objects in contact to briefly separate or
     interpenetrate during zone transitions.
   - **Verification:** Migrate a pair of contacting bodies across a zone boundary. Assert
     `ContactManifold` data is identical before and after migration.

## F-4.1.8 Character Controller

| ID       | Derived From                                             |
|----------|----------------------------------------------------------|
| R-4.1.8  | [F-4.1.8](../../features/physics/rigid-body-dynamics.md) |
| R-4.1.8a | [F-4.1.8](../../features/physics/rigid-body-dynamics.md) |
| R-4.1.8b | [F-4.1.8](../../features/physics/rigid-body-dynamics.md) |
| R-4.1.8c | [F-4.1.8](../../features/physics/rigid-body-dynamics.md) |
| R-4.1.8d | [F-4.1.8](../../features/physics/rigid-body-dynamics.md) |

1. **R-4.1.8** — Kinematic Character Controller Components: The engine **SHALL** provide a kinematic
   character controller composed of `CharacterController`, `GroundState`, `StepHeight`, and
   `SlopeLimit` ECS components with a dedicated movement system that queries the shared spatial
   index for ground detection.
   - **Rationale:** Character movement is the most common physics interaction; it must be composable
     from standard ECS components.
   - **Verification:** Attach all four components to an entity. Verify the movement system produces
     a `DesiredVelocity` component based on ground detection results.
2. **R-4.1.8a** — Slope Rejection: The character controller **SHALL** reject slopes exceeding the
   configured `SlopeLimit` angle, causing the character to slide on surfaces steeper than the limit.
   - **Rationale:** Characters must not walk up sheer cliffs; slope limits enforce traversable
     terrain boundaries.
   - **Verification:** Place a character on a 50-degree slope with a 45-degree limit. Assert the
     character slides. Place on a 40-degree slope. Assert the character holds.
3. **R-4.1.8b** — Step Climbing: The character controller **SHALL** automatically climb steps up to
   the height configured in the `StepHeight` component and block movement at steps exceeding that
   height.
   - **Rationale:** Stairs and curbs must be traversable without special level markup or designer
     intervention.
   - **Verification:** Place a character at a 0.3 m step with `StepHeight` of 0.35 m. Assert it
     climbs. Place at a 0.4 m step. Assert it is blocked.
4. **R-4.1.8c** — Shared Spatial Index Queries: Character controller ground detection **SHALL** use
   shape casts against the shared BVH spatial index, not a physics-private acceleration structure.
   - **Rationale:** The shared spatial index eliminates redundant data structures across physics,
     rendering, and networking.
   - **Verification:** Verify ground detection queries use the shared BVH resource. Assert no
     physics-private spatial structure is allocated for character queries.
5. **R-4.1.8d** — Character Controller Throughput: Character controller movement (ground detection,
   slope rejection, step climbing) **SHALL** complete within 0.1 ms per character entity, supporting
   at least 200 simultaneous controllers within the 4 ms physics budget.
   - **Rationale:** Multiplayer and AI-heavy scenes run hundreds of controllers; each must be
     lightweight.
   - **Verification:** Benchmark: simulate 200 character controllers on varied terrain. Assert total
     system time stays below 20 ms (0.1 ms per controller).

## F-4.1.9 Moving Platform System

| ID       | Derived From                                             |
|----------|----------------------------------------------------------|
| R-4.1.9  | [F-4.1.9](../../features/physics/rigid-body-dynamics.md) |
| R-4.1.9a | [F-4.1.9](../../features/physics/rigid-body-dynamics.md) |
| R-4.1.9b | [F-4.1.9](../../features/physics/rigid-body-dynamics.md) |
| R-4.1.9c | [F-4.1.9](../../features/physics/rigid-body-dynamics.md) |

1. **R-4.1.9** — Platform Velocity Transfer: The engine **SHALL** detect entities standing on a
   `MovingPlatform` via ground-contact queries and parent their velocity to the platform's velocity,
   inheriting both translational and rotational motion without sliding.
   - **Rationale:** Characters and objects on moving platforms must move with the platform
     automatically to avoid visually sliding off.
   - **Verification:** Place a character on a platform moving at 5 m/s for 10 seconds. Assert
     maximum drift is less than 1 cm per second.
2. **R-4.1.9a** — One-Way Platform Filtering: The engine **SHALL** support one-way platforms that
   are passable from below and solid from above using directional collision filtering.
   - **Rationale:** Platformer-style games require jump-through floors that block downward motion
     only.
   - **Verification:** Move a character upward through a one-way platform. Assert no collision
     occurs. Move downward onto it. Assert the platform is solid.
3. **R-4.1.9b** — Conveyor Belt Surface Velocity: The engine **SHALL** support `MovingPlatform`
   entities with a configurable surface velocity that pushes entities in contact along the belt
   direction.
   - **Rationale:** Conveyor belts are common in puzzle and industrial game environments.
   - **Verification:** Place a rigid body on a conveyor with surface velocity 2 m/s. Assert the body
     accelerates in the belt direction.
4. **R-4.1.9c** — Platform State Replication: Platform position, direction, and pause state
   **SHALL** replicate correctly via the ECS replication system for multiplayer.
   - **Rationale:** All clients must see synchronized platform positions to prevent desync during
     traversal.
   - **Verification:** Run server and client with a moving platform. Assert client platform position
     matches server within interpolation tolerance.

## F-4.1.10 Surface Smoothing and Ground Conformance

| ID        | Derived From                                              |
|-----------|-----------------------------------------------------------|
| R-4.1.10  | [F-4.1.10](../../features/physics/rigid-body-dynamics.md) |
| R-4.1.10a | [F-4.1.10](../../features/physics/rigid-body-dynamics.md) |
| R-4.1.10b | [F-4.1.10](../../features/physics/rigid-body-dynamics.md) |

1. **R-4.1.10** — Ground Smoothing Filter: The engine **SHALL** apply an exponential moving average
   filter to ground height and normal sampled by shape casts, with a configurable smoothing
   half-life, to eliminate character jitter on triangle edges and mesh seams.
   - **Rationale:** Raw triangle-mesh surfaces cause micro-bouncing on seams; smoothing produces
     visually stable character movement.
   - **Verification:** Move a character at constant speed over a mesh with 5 cm triangle-edge height
     discontinuities. Assert peak-to-peak vertical oscillation is below 1 mm.
2. **R-4.1.10a** — Step-Down Snapping: The engine **SHALL** snap the character downward by up to the
   configured maximum step-down distance to stay grounded on descents without floating.
   - **Rationale:** Characters must track the ground on descending slopes and stairs without visible
     air gaps.
   - **Verification:** Walk a character down a 30-degree ramp. Assert the character remains grounded
     with no floating frames. Assert the character does not snap beyond the configured maximum
     distance.
3. **R-4.1.10b** — Slope Alignment: The engine **SHALL** align the character's up-vector to the
   filtered ground normal at a configurable alignment speed.
   - **Rationale:** Visual tilting on slopes makes characters look naturally planted on uneven
     terrain.
   - **Verification:** Place a character on a 20-degree slope. Assert the up-vector converges to the
     slope normal within the configured alignment time. Assert no overshoot.

## Non-Functional Requirements

| ID        | Derived From                       |
|-----------|------------------------------------|
| R-4.1.NF1 | R-4.1.1, R-4.1.2, R-4.1.5, R-4.1.6 |
| R-4.1.NF2 | R-4.1.1, R-4.1.5, R-4.1.6          |
| R-4.1.NF3 | R-4.1.1                            |
| R-4.1.NF4 | R-4.1.8, R-4.1.9, R-4.1.10         |

1. **R-4.1.NF1** — Rigid Body Simulation Frame Budget: The rigid body simulation (integration,
   broadphase, narrowphase, constraint solve, island management, sleeping) **SHALL** complete within
   4 ms per frame on the minimum-spec target hardware for scenes containing up to 2,000 active
   (non-sleeping) rigid bodies at 4 substeps.
   - **Rationale:** Physics must fit within a 16.67 ms frame budget alongside rendering, audio,
     gameplay, and networking. A 4 ms allocation is a standard industry guideline for physics.
   - **Verification:** Benchmark -- simulate 2,000 active rigid bodies with mixed collision and
     constraint loads at 4 substeps; measure wall-clock time per tick on minimum-spec hardware;
     assert it does not exceed 4 ms.
2. **R-4.1.NF2** — Rigid Body Memory Budget: Peak memory consumption for rigid body simulation state
   (components, contact manifolds, island data, sleep timers) **SHALL NOT** exceed 256 bytes per
   active rigid body entity, excluding collider shape data.
   - **Rationale:** Memory budgets must scale linearly with entity count to support large worlds
     with tens of thousands of physics objects.
   - **Verification:** Profile -- spawn 10,000 rigid body entities; measure total physics component
     memory; divide by entity count; assert per-entity overhead is at most 256 bytes.
3. **R-4.1.NF3** — Cross-Platform Determinism: Rigid body simulation **SHALL** produce bit-identical
   results across Windows (x86-64), macOS (ARM64), and Linux (x86-64) given identical initial state,
   timestep, and entity ordering.
   - **Rationale:** Server-authoritative simulation with client-side prediction requires
     cross-platform reproducibility; any divergence causes desync and rollback storms.
   - **Verification:** Run a 1,000-step benchmark scenario on all three platforms; compare
     serialized component state after each step; assert bit-equality across platforms.
4. **R-4.1.NF4** — Character Controller Latency: Character controller movement (ground detection,
   slope rejection, step climbing) **SHALL** complete within 0.1 ms per character entity, supporting
   at least 200 simultaneous characters within the 4 ms physics budget.
   - **Rationale:** Multiplayer and AI-heavy scenes run hundreds of character controllers; each must
     be lightweight enough to avoid starving rigid body and constraint solving.
   - **Verification:** Benchmark -- simulate 200 character controllers navigating varied terrain;
     measure total character-controller system time per frame; assert it remains below 0.1 ms per
     character.
