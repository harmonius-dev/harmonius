# 4.1 — Rigid Body Dynamics

## Integration Methods

### F-4.1.1 Deterministic Fixed-Timestep Integration

Integrate rigid body motion using symplectic Euler and Verlet methods with a fixed-timestep
accumulator. The `IntegrationSystem` queries `(RigidBody, &mut Velocity, &mut AngularVelocity,
&ExternalForce, &ExternalTorque, &Mass, &Inertia)` each tick, reading force and torque
components to update velocity, then advancing position and orientation on the `Transform`
component. Determinism is essential for server-authoritative MMO reconciliation and rollback.

- **Requirements:** R-4.1.1
- **Dependencies:** F-1.1.1, F-1.1.2
- **Platform notes:** Floating-point determinism requires strict IEEE 754 compliance; disable
  fast-math optimizations on all platforms. Cross-platform reproducibility demands identical
  compiler flags and rounding modes.

### F-4.1.2 Simulation Substeps

Subdivide the physics tick into configurable substeps by repeating `IntegrationSystem` and
solver system execution within the ECS schedule. A `PhysicsConfig` ECS resource controls the
global substep count, while an optional `SubstepOverride` component on individual entities
allows per-entity tuning. Each substep re-executes integration, broadphase, narrowphase, and
constraint-solve systems in sequence.

- **Requirements:** R-4.1.2
- **Dependencies:** F-4.1.1, F-1.1.3
- **Platform notes:** None

## Collision Response

### F-4.1.3 Contact Resolution with Restitution and Friction

Resolve collisions using impulse-based contact resolution via a `ContactSolverSystem` that
queries `ContactManifold` components and reads `PhysicsMaterial` components for restitution and
friction coefficients (static and dynamic). Material pairs are combined via user-specified
rules (average, min, max, multiply) stored in a `MaterialCombinationRules` ECS resource. The
solver writes impulse results back to `Velocity` and `AngularVelocity` components.

- **Requirements:** R-4.1.3
- **Dependencies:** F-4.1.1, F-4.2.2, F-1.1.2
- **Platform notes:** None

### F-4.1.4 Continuous Collision Detection

Detect and resolve tunneling for fast-moving objects using swept-volume time-of-impact queries.
A `CcdEnabled` marker component flags entities for CCD processing. The `CcdSystem` queries
`(CcdEnabled, &Velocity, &Collider, &Transform)` and performs swept tests against broadphase
candidates, inserting sub-step corrections into `Velocity` and `Transform` before the next
integration pass.

- **Requirements:** R-4.1.4
- **Dependencies:** F-4.1.1, F-4.2.1, F-4.2.2
- **Platform notes:** None

## Islands and Sleeping

### F-4.1.5 Simulation Islands

Partition interacting bodies into disjoint islands computed from ECS entity connectivity. The
`IslandComputeSystem` builds a union-find over entities linked by `ContactManifold` and
`JointConstraint` components, assigning an `Island` component to each entity. Independent
islands are solved in parallel across CPU cores. Island merging and splitting occurs each frame
based on current collision and constraint component state.

- **Requirements:** R-4.1.5
- **Dependencies:** F-4.1.1, F-4.2.2, F-1.1.2
- **Platform notes:** None

### F-4.1.6 Body Sleeping and Wake-Up

Deactivate bodies at rest by adding a `Sleeping` marker component when `Velocity` and
`AngularVelocity` remain below configurable thresholds (stored in a `SleepConfig` ECS
resource) for a sustained period tracked by a `SleepTimer` component. The `SleepSystem` skips
`Sleeping` entities during integration and solving. Wake-up is triggered automatically via ECS
change detection on `ExternalForce`, `ExternalTorque`, or `ContactManifold` components,
removing the `Sleeping` marker.

- **Requirements:** R-4.1.6
- **Dependencies:** F-4.1.5, F-1.1.2
- **Platform notes:** None

## Streaming and Scale

### F-4.1.7 Cross-Zone Physics Continuity

Support seamless rigid body simulation across streaming zone boundaries using ECS entity
migration between worlds. The `ZoneMigrationSystem` detects entities whose `Transform` crosses
zone boundaries and transfers them — along with all physics components (`RigidBody`, `Velocity`,
`AngularVelocity`, `Mass`, `Inertia`, `Collider`) — to the destination world's ECS. Momentum
and contact state are preserved during migration to prevent visible discontinuities at zone
borders.

- **Requirements:** R-4.1.7
- **Dependencies:** F-4.1.5, F-4.1.1, F-1.1.4
- **Platform notes:** None
