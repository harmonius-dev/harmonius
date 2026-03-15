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
- **Dependencies:** F-4.1.1, F-1.1.4 (Static and Dynamic Component Registration)
- **Platform notes:** Mobile: max 4 substeps, per-entity override disabled. Switch: max 8
  substeps. Desktop: configurable, default 16 substeps. High-end PC: up to 32 substeps
  for high-fidelity vehicle and ragdoll simulation.

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
- **Platform notes:** Mobile: max 16 CCD-enabled entities, simplified sweep (sphere only).
  Switch: max 32 CCD entities. Desktop: max 256 CCD entities with full convex sweeps.
  High-end PC: max 1024 CCD entities.

## Islands and Sleeping

### F-4.1.5 Simulation Islands

Partition interacting bodies into disjoint islands computed from ECS entity connectivity. The
`IslandComputeSystem` builds a union-find over entities linked by `ContactManifold` and
`JointConstraint` components, assigning an `Island` component to each entity. Independent
islands are solved in parallel across CPU cores. Island merging and splitting occurs each frame
based on current collision and constraint component state.

- **Requirements:** R-4.1.5
- **Dependencies:** F-4.1.1, F-4.2.2, F-1.1.2
- **Platform notes:** Mobile: max 64 active islands, 32 bodies per island. Switch: max 128
  islands, 64 bodies per island. Desktop: max 1024 islands, 256 bodies per island.
  High-end PC: unlimited islands with parallel solve across all cores.

### F-4.1.6 Body Sleeping and Wake-Up

Deactivate bodies at rest by adding a `Sleeping` marker component when `Velocity` and
`AngularVelocity` remain below configurable thresholds (stored in a `SleepConfig` ECS
resource) for a sustained period tracked by a `SleepTimer` component. The `SleepSystem` skips
`Sleeping` entities during integration and solving. Wake-up is triggered automatically via ECS
change detection on `ExternalForce`, `ExternalTorque`, or `ContactManifold` components,
removing the `Sleeping` marker.

- **Requirements:** R-4.1.6
- **Dependencies:** F-4.1.5, F-1.1.2
- **Platform notes:** Mobile: aggressive sleep thresholds (higher velocity/energy floor)
  to maximize sleeping ratio. Switch: moderately aggressive thresholds. Desktop: standard
  thresholds. All platforms benefit from sleep; critical on mobile to stay within CPU
  budget.

## Streaming and Scale

### F-4.1.7 Cross-Zone Physics Continuity

Support seamless rigid body simulation across streaming zone boundaries using ECS entity
migration between worlds. The `ZoneMigrationSystem` detects entities whose `Transform` crosses
zone boundaries and transfers them — along with all physics components (`RigidBody`, `Velocity`,
`AngularVelocity`, `Mass`, `Inertia`, `Collider`) — to the destination world's ECS. Momentum
and contact state are preserved during migration to prevent visible discontinuities at zone
borders.

- **Requirements:** R-4.1.7
- **Dependencies:** F-4.1.5, F-4.1.1, F-1.1.11 (Entity Lifecycle with Generational Indices)
- **Platform notes:** None

## Character Movement

### F-4.1.8 Character Controller

A kinematic-based character controller implemented as ECS components (`CharacterController`,
`GroundState`, `StepHeight`, `SlopeLimit`) with a dedicated movement system. Handles ground
detection via shape casts, slope sliding with configurable angle limits, step climbing for
stairs and curbs, moving platform attachment (inherits platform velocity), and coyote-time
grace periods for platformer-style games. The controller queries the shared spatial index
(F-1.9.4) for environment collision and produces a `DesiredVelocity` component consumed by
the physics integration system.

- **Requirements:** R-4.1.8
- **Dependencies:** F-4.1.1, F-4.4.1, F-1.9.4 (Unified Spatial Query)
- **Platform notes:** Mobile: max 16 active character controllers, 2 ground casts per
  frame. Switch: max 32 controllers, 3 ground casts. Desktop: max 256 controllers,
  4 ground casts. High-end PC: max 1024 controllers for MMO crowd scenes.

### F-4.1.9 Moving Platform System

Kinematic platforms (elevators, conveyor belts, rotating platforms, swinging bridges, rail carts)
that transport characters and physics objects riding on them. A `MovingPlatform` component marks
kinematic rigid body entities whose motion is driven by spline paths, animation curves, or logic
graphs (F-15.8.4). The system detects entities standing on the platform via ground-contact queries
and parents their velocity to the platform's velocity — characters inherit translational and
rotational motion without sliding or jitter. Acceleration and deceleration phases apply smoothed
velocity changes to prevent passengers from being launched. Conveyor belt surfaces apply a
continuous surface velocity to entities in contact. One-way platforms (pass through from below,
solid from above) use directional collision filtering. Platform state (position along path, paused,
direction) is replicated for multiplayer (F-8.2.1).

- **Requirements:** R-4.1.9
- **Dependencies:** F-4.1.8 (Character Controller), F-4.1.1, F-8.2.1 (Property Replication)
- **Platform notes:** Mobile: max 8 active moving platforms with passenger detection.
  Switch: max 16 platforms. Desktop: max 128 platforms. Passenger query radius reduced
  on mobile to limit broadphase cost.

### F-4.1.10 Surface Smoothing and Ground Conformance

Smooth character movement over irregular, jagged, or tessellated terrain without stuttering or
micro-bouncing. The system casts a shape (capsule bottom or sphere) downward each frame to sample
the ground normal and height, then applies a smoothing filter (exponential moving average with
configurable half-life) to both the ground height and normal. This prevents the character from
vibrating on triangle edges, seams between mesh tiles, and high-frequency terrain heightmap noise.
Configurable parameters: smoothing half-life (lower = more responsive, higher = smoother),
maximum step-down distance (how far the character snaps downward to stay grounded on descents),
and slope alignment speed (how quickly the character's up-vector aligns to the filtered ground
normal for visual tilting on slopes). The smoothed ground state feeds into the foot placement IK
system (F-9.3.5) for natural-looking locomotion on uneven surfaces.

- **Requirements:** R-4.1.10
- **Dependencies:** F-4.1.8 (Character Controller), F-4.4.1 (Ray Casts), F-9.3.5 (Foot Placement)
- **Platform notes:** None
