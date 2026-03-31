# 4.3 — Constraints & Joints

## Joint Types

| ID      | Feature |
|---------|---------------------- |
| F-4.3.1 | Core Joint Types |
| F-4.3.2 | Advanced Joint Types |

1. **F-4.3.1** — Each joint is an ECS entity carrying a `Joint` component that stores entity
   references to the two connected body entities, plus a `JointType` component selecting one of:
   `Revolute` (hinge), `Prismatic` (slider), `Fixed` (weld), or `Distance`. The
   `ConstraintSolverSystem` queries `(Joint, JointType)` entities each substep to resolve positional
   and velocity constraints between the linked bodies.
   - **Deps:** F-1.1.1 (Archetype Storage), F-1.1.11 (Entity Lifecycle with Generational Indices),
     F-4.1.1, F-4.1.3
2. **F-4.3.2** — Extend the `JointType` component with `Spring`, `ConeTwist`, and `Generic6DOF`
   variants. The `ConeTwist` variant supports angular limits on all three axes for ragdolls and
   turrets. The `Generic6DOF` variant allows per-axis locking, limiting, and freeing of both linear
   and angular degrees of freedom via a `Dof6Config` component. The `ConstraintSolverSystem`
   dispatches to specialized solve routines based on the `JointType` variant.
   - **Deps:** F-4.3.1, F-1.1.17 (Composable Archetype Queries)

## Motors and Limits

| ID      | Feature |
|---------|------------------------- |
| F-4.3.3 | Joint Motors and Limits |
| F-4.3.4 | Breakable Joints |

1. **F-4.3.3** — Attach optional `JointMotor` and `JointLimits` components to any joint entity.
   `JointMotor` specifies a target position or velocity with a configurable maximum force per axis.
   `JointLimits` defines angular and linear bounds with restitution and softness parameters. The
   `ConstraintSolverSystem` reads these optional components during its query over
   `(Joint, JointType)` entities and applies motor drives and limit clamping as additional
   constraint rows.
   - **Deps:** F-4.3.1, F-1.1.17 (Composable Archetype Queries)
2. **F-4.3.4** — Add an optional `BreakForce` component to any joint entity, specifying maximum
   tolerable force and torque thresholds. The `ConstraintSolverSystem` checks accumulated constraint
   impulses against `BreakForce` each substep. When a threshold is exceeded, the joint entity is
   despawned via a command buffer and a `JointBroken` ECS event is emitted containing the joint
   entity, both body entities, and the breaking force magnitude.
   - **Deps:** F-4.3.1, F-4.2.7, F-1.1.32 (Deferred Structural Changes via Command Buffers), F-1.5.1
     (Event Bus)

## Ragdolls and Chains

| ID      | Feature |
|---------|------------------------ |
| F-4.3.5 | Ragdoll Configuration |
| F-4.3.6 | Joint Chains and Ropes |

1. **F-4.3.5** — A ragdoll is a bundle of joint entities whose `Joint` components reference skeleton
   bone entities carrying `RigidBody`, `Collider`, and `Transform` components. A `RagdollDef` asset
   maps a skeleton hierarchy to joint entity archetypes with anatomically plausible `JointLimits`
   and `JointType` (typically `ConeTwist` and `Revolute`). The `RagdollActivationSystem` spawns
   joint entities from the definition when transitioning from animation-driven to physics-driven
   state, and despawns them when returning to animation control.
   - **Deps:** F-4.3.1, F-4.3.2, F-1.1.11 (Entity Lifecycle with Generational Indices), F-1.1.32
     (Deferred Structural Changes via Command Buffers)
   - **Platform:** Mobile: max 4 simultaneous ragdolls, 8 bones per ragdoll. Switch: max 8 ragdolls,
     12 bones. Desktop: max 32 ragdolls, 20 bones. High-end PC: max 128 ragdolls with full skeleton
     fidelity. Distant ragdolls replaced with animation blend.
2. **F-4.3.6** — Model ropes, cables, and chains as sequences of joint entities whose `Joint` body
   references form a linked chain. Each chain segment is an entity with `RigidBody`, `Collider`, and
   `Transform` components, connected to its neighbors by `Joint` entities carrying `Distance` or
   `Spring` `JointType` variants. A `ChainDef` asset configures segment count and stiffness,
   allowing quality-performance trade-offs for the many rope-like objects in an MMO world.
   - **Deps:** F-4.3.1, F-4.2.2, F-1.1.11 (Entity Lifecycle with Generational Indices)
   - **Platform:** Mobile: max 8 segments per chain, 4 active chains. Switch: max 16 segments, 8
     chains. Desktop: max 64 segments, 32 chains. High-end PC: max 128 segments, unlimited chains.
     Distant chains use verlet fallback without collision.

## Solvers

| ID      | Feature |
|---------|-------------------- |
| F-4.3.7 | Constraint Solvers |

1. **F-4.3.7** — The `ConstraintSolverSystem` implements sequential impulse (SI) and Temporal
   Gauss-Seidel (TGS) solvers selectable via a `SolverConfig` ECS resource that also controls
   iteration count. The system queries all `(Joint, JointType)` entities, optionally reading
   `JointMotor`, `JointLimits`, and `BreakForce` components, then writes solved impulses to
   `Velocity` and `AngularVelocity` components on the referenced body entities. The solver is
   deterministic given identical entity ordering, supporting server-authoritative simulation with
   client-side prediction.
   - **Deps:** F-4.3.1, F-4.1.5, F-1.1.20 (Automatic Parallel Iteration), F-1.1.25 (Dependency
     Resolution and Topological Ordering)
   - **Platform:** Mobile: SI solver only, 4 iterations. Switch: SI solver, 6 iterations. Desktop:
     TGS default, 8 iterations. High-end PC: TGS with 12+ iterations for high-fidelity stacking and
     vehicle constraints.

## Limb Severance

| ID      | Feature |
|---------|-------------------------------------- |
| F-4.3.8 | Limb Severance and Joint Destruction |
| F-4.3.9 | Prosthetic and Limb Replacement |

1. **F-4.3.8** — Runtime destruction of skeletal joint connections, detaching limbs, tails, wings,
   heads, and appendages from a parent skeleton. When a joint's accumulated damage exceeds its
   severance threshold (configured per joint in the skeleton asset), the joint connection is
   destroyed: the child bone chain and its associated mesh segments are separated from the parent
   skeleton and spawned as independent physics-simulated entities with ragdoll behavior (F-4.3.5).
   The severed limb retains its mesh, collision shapes, and material — blood/gore VFX (F-11.6.4)
   spawn at the separation point. The remaining skeleton adapts: the physics system removes
   constraints for the severed chain, the animation system (F-9.3.10) adjusts locomotion to
   compensate (three-legged gait for quadrupeds, one-armed combat for humanoids), and gameplay
   effects (F-13.10.3) apply stat penalties (reduced damage, movement speed, or abilities tied to
   the lost limb). Severance is an ECS operation: the `JointSevered` event fires through observers
   (F-1.1.30), enabling gameplay systems to react (death on head severance, disarm on weapon-arm
   loss). Kenshi-style progressive limb damage is supported via per-joint HP tracked in a
   `LimbHealth` component.
   - **Deps:** F-4.3.5 (Ragdoll), F-9.3.10 (Procedural Attachment and Dismemberment), F-13.10.3
     (Gameplay Effects), F-1.1.30 (Observers), F-11.6.4 (Event-Driven VFX)
   - **Platform:** Mobile: max 2 simultaneous severed limbs active as physics objects, simplified
     VFX. Switch: max 4 severed limbs. Desktop: max 16 severed limbs with full ragdoll. High-end PC:
     max 64. Severed limbs share ragdoll budget (F-4.3.5).
2. **F-4.3.9** — After limb severance, replacement limbs (prosthetics, mechanical arms, regrown
   appendages) can be attached to the severed joint socket at runtime. A `ProstheticDef` asset
   defines: the replacement mesh, bone chain, collision shapes, stat modifiers (a mechanical arm may
   be stronger but heavier), and compatible socket types (arm socket, leg socket, tail socket).
   Attachment plays a snap animation (F-9.3.10), re-establishes physics constraints at the socket
   joint, and updates the skeleton's bone hierarchy. The animation system re-evaluates locomotion
   profiles (F-9.3.8) to account for the replacement's properties (a peg leg has different gait
   parameters than a mechanical leg). Prosthetics are inventory items (F-13.9.1) equipped through
   the character equipment system (F-13.9.6).
   - **Deps:** F-4.3.8, F-9.3.10 (Attachment), F-9.3.8 (Multi-Skeleton Locomotion), F-13.9.6
     (Equipment Binding)
