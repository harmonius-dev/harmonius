# 4.3 — Constraints & Joints

## Joint Types

### F-4.3.1 Core Joint Types

Each joint is an ECS entity carrying a `Joint` component that stores entity references to the two
connected body entities, plus a `JointType` component selecting one of: `Revolute` (hinge),
`Prismatic` (slider), `Fixed` (weld), or `Distance`. The `ConstraintSolverSystem` queries
`(Joint, JointType)` entities each substep to resolve positional and velocity constraints between
the linked bodies.

- **Requirements:** R-4.3.1
- **Dependencies:** F-1.1.1 (Archetype Storage), F-1.1.4 (Entity Lifecycle), F-4.1.1, F-4.1.3
- **Platform notes:** None

### F-4.3.2 Advanced Joint Types

Extend the `JointType` component with `Spring`, `ConeTwist`, and `Generic6DOF` variants. The
`ConeTwist` variant supports angular limits on all three axes for ragdolls and turrets. The
`Generic6DOF` variant allows per-axis locking, limiting, and freeing of both linear and angular
degrees of freedom via a `Dof6Config` component. The `ConstraintSolverSystem` dispatches to
specialized solve routines based on the `JointType` variant.

- **Requirements:** R-4.3.2
- **Dependencies:** F-4.3.1, F-1.1.5 (Composable Archetype Queries)
- **Platform notes:** None

## Motors and Limits

### F-4.3.3 Joint Motors and Limits

Attach optional `JointMotor` and `JointLimits` components to any joint entity. `JointMotor`
specifies a target position or velocity with a configurable maximum force per axis. `JointLimits`
defines angular and linear bounds with restitution and softness parameters. The
`ConstraintSolverSystem` reads these optional components during its query over `(Joint, JointType)`
entities and applies motor drives and limit clamping as additional constraint rows.

- **Requirements:** R-4.3.3
- **Dependencies:** F-4.3.1, F-1.1.5 (Composable Archetype Queries)
- **Platform notes:** None

### F-4.3.4 Breakable Joints

Add an optional `BreakForce` component to any joint entity, specifying maximum tolerable force and
torque thresholds. The `ConstraintSolverSystem` checks accumulated constraint impulses against
`BreakForce` each substep. When a threshold is exceeded, the joint entity is despawned via a
command buffer and a `JointBroken` ECS event is emitted containing the joint entity, both body
entities, and the breaking force magnitude.

- **Requirements:** R-4.3.4
- **Dependencies:** F-4.3.1, F-4.2.7, F-1.1.11 (Command Buffers), F-1.5.1 (Event Bus)
- **Platform notes:** None

## Ragdolls and Chains

### F-4.3.5 Ragdoll Configuration

A ragdoll is a bundle of joint entities whose `Joint` components reference skeleton bone entities
carrying `RigidBody`, `Collider`, and `Transform` components. A `RagdollDef` asset maps a
skeleton hierarchy to joint entity archetypes with anatomically plausible `JointLimits` and
`JointType` (typically `ConeTwist` and `Revolute`). The `RagdollActivationSystem` spawns joint
entities from the definition when transitioning from animation-driven to physics-driven state,
and despawns them when returning to animation control.

- **Requirements:** R-4.3.5
- **Dependencies:** F-4.3.1, F-4.3.2, F-1.1.4 (Entity Lifecycle), F-1.1.11 (Command Buffers)
- **Platform notes:** None

### F-4.3.6 Joint Chains and Ropes

Model ropes, cables, and chains as sequences of joint entities whose `Joint` body references form
a linked chain. Each chain segment is an entity with `RigidBody`, `Collider`, and `Transform`
components, connected to its neighbors by `Joint` entities carrying `Distance` or `Spring`
`JointType` variants. A `ChainDef` asset configures segment count and stiffness, allowing
quality-performance trade-offs for the many rope-like objects in an MMO world.

- **Requirements:** R-4.3.6
- **Dependencies:** F-4.3.1, F-4.2.2, F-1.1.4 (Entity Lifecycle)
- **Platform notes:** None

## Solvers

### F-4.3.7 Constraint Solvers

The `ConstraintSolverSystem` implements sequential impulse (SI) and Temporal Gauss-Seidel (TGS)
solvers selectable via a `SolverConfig` ECS resource that also controls iteration count. The
system queries all `(Joint, JointType)` entities, optionally reading `JointMotor`, `JointLimits`,
and `BreakForce` components, then writes solved impulses to `Velocity` and `AngularVelocity`
components on the referenced body entities. The solver is deterministic given identical entity
ordering, supporting server-authoritative simulation with client-side prediction.

- **Requirements:** R-4.3.7
- **Dependencies:** F-4.3.1, F-4.1.5, F-1.1.6 (Parallel Iteration), F-1.1.9 (System Scheduling)
- **Platform notes:** None
