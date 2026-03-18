# R-4.3 — Constraints & Joints Requirements

## F-4.3.1 Core Joint Types

| ID       | Derived From                                                |
|----------|-------------------------------------------------------------|
| R-4.3.1  | [F-4.3.1](../../features/physics/constraints-and-joints.md) |
| R-4.3.1a | [F-4.3.1](../../features/physics/constraints-and-joints.md) |

1. **R-4.3.1** — ECS Joint Entity Model: The engine **SHALL** represent each joint as an ECS entity
   carrying a `Joint` component (storing entity references to two connected bodies) and a
   `JointType` component selecting one of: `Revolute`, `Prismatic`, `Fixed`, or `Distance`.
   - **Rationale:** Joint-as-entity enables standard ECS queries, change detection, and command
     buffer operations on constraints without a separate physics world.
   - **Verification:** Create a joint entity with `Joint` and `JointType` components. Assert it is
     queryable via `(Joint, JointType)` and references two valid body entities.
2. **R-4.3.1a** — Joint Constraint Accuracy: The `ConstraintSolverSystem` **SHALL** maintain
   positional drift at joint anchor points below 1 mm over 500 ticks at 8 solver iterations for all
   four core joint types.
   - **Rationale:** Visible constraint drift breaks mechanical connections and ragdolls;
     sub-millimeter accuracy is required for believable joints.
   - **Verification:** Connect two 1 kg bodies with each core joint type. Apply forces for 500 ticks
     at 8 iterations. Assert positional drift at the anchor is below 1 mm.

## F-4.3.2 Advanced Joint Types

| ID      | Derived From                                                |
|---------|-------------------------------------------------------------|
| R-4.3.2 | [F-4.3.2](../../features/physics/constraints-and-joints.md) |

1. **R-4.3.2** — Extended JointType Variants: The engine **SHALL** extend `JointType` with `Spring`,
   `ConeTwist`, and `Generic6DOF` variants. `ConeTwist` **SHALL** support angular limits on all
   three axes. `Generic6DOF` **SHALL** support per-axis locking, limiting, and freeing via a
   `Dof6Config` component.
   - **Rationale:** Ragdoll shoulders, turrets, and mechanical assemblies require multi-axis angular
     constraints that exceed the four core types.
   - **Verification:** Create a cone-twist joint with a 45-degree limit. Apply torque past the
     limit. Assert the angle does not exceed 45.5 degrees over 1000 ticks.

## F-4.3.3 Joint Motors and Limits

| ID      | Derived From                                                |
|---------|-------------------------------------------------------------|
| R-4.3.3 | [F-4.3.3](../../features/physics/constraints-and-joints.md) |

1. **R-4.3.3** — Motor and Limit Components: The engine **SHALL** support optional `JointMotor` and
   `JointLimits` components on joint entities. `JointMotor` **SHALL** specify target velocity and
   maximum force. `JointLimits` **SHALL** define angular and linear bounds. Both are integrated as
   additional constraint rows in the solver.
   - **Rationale:** Powered doors, turrets, and mechanisms need motor-driven motion with
     configurable limits.
   - **Verification:** Set a revolute joint motor target to 2 rad/s. Assert steady-state velocity is
     within 1% of 2 rad/s. Set angular limits to +/-45 degrees. Apply excess torque. Assert the body
     does not exceed 45.5 degrees.

## F-4.3.4 Breakable Joints

| ID       | Derived From                                                |
|----------|-------------------------------------------------------------|
| R-4.3.4  | [F-4.3.4](../../features/physics/constraints-and-joints.md) |
| R-4.3.4a | [F-4.3.4](../../features/physics/constraints-and-joints.md) |

1. **R-4.3.4** — Break Force Threshold Detection: The engine **SHALL** support a `BreakForce`
   component on joint entities specifying maximum force and torque thresholds. When accumulated
   constraint impulses exceed a threshold, the joint entity **SHALL** be despawned via command
   buffer and a `JointBroken` event **SHALL** be emitted within one substep.
   - **Rationale:** Destructible mechanical connections require responsive break detection to enable
     dynamic destruction sequences.
   - **Verification:** Create a joint with 1000 N threshold. Apply 1500 N. Assert the joint is
     despawned and a `JointBroken` event fires within one substep.
2. **R-4.3.4a** — JointBroken Event Payload: The `JointBroken` event **SHALL** contain both body
   entity IDs and the breaking force magnitude.
   - **Rationale:** Gameplay systems need full context about which bodies separated and the force
     involved to trigger appropriate VFX and gameplay reactions.
   - **Verification:** Trigger a joint break. Assert the event contains both body entity IDs and the
     force magnitude.

## F-4.3.5 Ragdoll Configuration

| ID       | Derived From                                                |
|----------|-------------------------------------------------------------|
| R-4.3.5  | [F-4.3.5](../../features/physics/constraints-and-joints.md) |
| R-4.3.5a | [F-4.3.5](../../features/physics/constraints-and-joints.md) |
| R-4.3.5b | [F-4.3.5](../../features/physics/constraints-and-joints.md) |

1. **R-4.3.5** — Ragdoll Definition and Activation: The engine **SHALL** support `RagdollDef` assets
   that map skeleton hierarchies to joint entity archetypes with anatomically plausible limits and
   joint types. The `RagdollActivationSystem` **SHALL** spawn all joint entities from the definition
   within one frame when transitioning from animation to physics.
   - **Rationale:** Ragdolls are high-value gameplay moments; activation must be instant and
     complete to avoid partial ragdoll states.
   - **Verification:** Trigger ragdoll activation on a 20-bone skeleton. Assert all joint entities
     spawn with correct types and limits within one frame.
2. **R-4.3.5a** — Ragdoll Constraint Stability: An active ragdoll **SHALL** maintain constraint
   violations below 5 mm over 100 ticks of simulation.
   - **Rationale:** Visible joint separation in ragdolls breaks the illusion of a connected
     skeleton.
   - **Verification:** Simulate an active ragdoll for 100 ticks. Assert no constraint violation
     exceeds 5 mm.
3. **R-4.3.5b** — Ragdoll Activation Latency: Ragdoll activation **SHALL** complete within 0.5 ms
   per ragdoll, supporting at least 8 simultaneous activations per frame within the 4 ms physics
   budget.
   - **Rationale:** Mass-casualty events (explosions) can activate multiple ragdolls in one frame;
     each must be fast enough to avoid hitches.
   - **Verification:** Activate 8 ragdolls (20 bones each) in one frame. Assert total time stays
     within 4 ms.

## F-4.3.6 Joint Chains and Ropes

| ID       | Derived From                                                |
|----------|-------------------------------------------------------------|
| R-4.3.6  | [F-4.3.6](../../features/physics/constraints-and-joints.md) |
| R-4.3.6a | [F-4.3.6](../../features/physics/constraints-and-joints.md) |

1. **R-4.3.6** — Chain Definition Assets: The engine **SHALL** support `ChainDef` assets configuring
   segment count, stiffness, and joint type. Chain spawning **SHALL** create a sequence of rigid
   body entities connected by joint entities from the definition.
   - **Rationale:** Ropes, cables, and chains are common in game environments; data-driven
     definitions enable reuse.
   - **Verification:** Spawn a chain from a `ChainDef` with 16 segments. Assert all body and joint
     entities are created with correct connectivity.
2. **R-4.3.6a** — Chain Stability: A joint chain of up to 32 segments **SHALL** remain stable (no
   segment separation exceeding 1 mm, no energy gain exceeding 1% per second) for at least 60
   seconds of continuous simulation at 4 substeps.
   - **Rationale:** Long-running rope and chain simulations must not accumulate drift or energy that
     causes visual artifacts or instability.
   - **Verification:** Simulate a 32-segment pendulum chain for 60 seconds at 4 substeps. Measure
     energy and separation each second. Assert energy gain below 1% and separation below 1 mm
     throughout.

## F-4.3.7 Constraint Solvers

| ID       | Derived From                                                |
|----------|-------------------------------------------------------------|
| R-4.3.7  | [F-4.3.7](../../features/physics/constraints-and-joints.md) |
| R-4.3.7a | [F-4.3.7](../../features/physics/constraints-and-joints.md) |
| R-4.3.7b | [F-4.3.7](../../features/physics/constraints-and-joints.md) |

1. **R-4.3.7** — SI and TGS Solver Selection: The engine **SHALL** implement both Sequential Impulse
   (SI) and Temporal Gauss-Seidel (TGS) constraint solvers, selectable via a `SolverConfig` ECS
   resource that also controls iteration count.
   - **Rationale:** SI is simpler and cheaper; TGS converges faster for complex constraint
     configurations. Projects need to choose based on their requirements.
   - **Verification:** Run a 10-body joint chain for 1000 ticks with both SI and TGS at 8
     iterations. Assert TGS drift is at least 30% lower than SI.
2. **R-4.3.7a** — Solver Throughput: The constraint solver **SHALL** process at least 5,000
   constraint rows per millisecond on minimum-spec hardware, supporting 500 active joints at 8
   iterations within the 4 ms physics budget.
   - **Rationale:** Complex scenes (ragdolls, vehicles, destruction chains) accumulate thousands of
     constraint rows per frame.
   - **Verification:** Benchmark: create 500 active joints at 8 iterations. Measure solver
     wall-clock time. Assert completion within 4 ms.
3. **R-4.3.7b** — Solver Determinism: The constraint solver **SHALL** produce bit-identical results
   given identical entity ordering and inputs.
   - **Rationale:** Client-side prediction requires deterministic solving to avoid desync with the
     authoritative server.
   - **Verification:** Run the same constraint scenario twice. Assert bit-identical velocity
     outputs.

## F-4.3.8 Limb Severance and Joint Destruction

| ID       | Derived From                                                |
|----------|-------------------------------------------------------------|
| R-4.3.8  | [F-4.3.8](../../features/physics/constraints-and-joints.md) |
| R-4.3.8a | [F-4.3.8](../../features/physics/constraints-and-joints.md) |
| R-4.3.8b | [F-4.3.8](../../features/physics/constraints-and-joints.md) |

1. **R-4.3.8** — Per-Joint Damage and Severance: The engine **SHALL** support a `LimbHealth`
   component per joint that tracks accumulated damage against a configurable severance threshold.
   When the threshold is reached, the joint connection **SHALL** be destroyed and the child bone
   chain spawned as an independent ragdoll entity within one frame.
   - **Rationale:** Limb severance creates high-impact combat moments; it must be responsive and
     driven by per-joint durability data.
   - **Verification:** Deal damage to a limb until the severance threshold is reached. Assert the
     limb detaches as an independent ragdoll entity within one frame.
2. **R-4.3.8a** — JointSevered Event: The engine **SHALL** emit a `JointSevered` event containing
   the severed joint ID, parent entity, and spawned limb entity.
   - **Rationale:** Gameplay systems need severance details to trigger death on head severance,
     disarm on arm loss, etc.
   - **Verification:** Sever a limb. Assert the event fires with the correct joint ID, parent
     entity, and spawned limb entity.
3. **R-4.3.8b** — Parent Skeleton Adaptation: After limb severance, the parent skeleton **SHALL**
   adapt its locomotion (e.g., three-legged gait) and gameplay stat penalties **SHALL** be applied
   via the gameplay effect system.
   - **Rationale:** Severance must have mechanical consequences to be meaningful gameplay rather
     than just visual.
   - **Verification:** Sever a leg from a quadruped. Assert locomotion adapts to three-legged gait.
     Assert movement speed penalty is applied.

## F-4.3.9 Prosthetic and Limb Replacement

| ID       | Derived From                                                |
|----------|-------------------------------------------------------------|
| R-4.3.9  | [F-4.3.9](../../features/physics/constraints-and-joints.md) |
| R-4.3.9a | [F-4.3.9](../../features/physics/constraints-and-joints.md) |

1. **R-4.3.9** — Prosthetic Definition and Attachment: The engine **SHALL** support `ProstheticDef`
   assets defining replacement mesh, bone chain, collision shapes, stat modifiers, and compatible
   socket types. Attachment **SHALL** re-establish physics constraints at the socket joint and
   update the skeleton's bone hierarchy.
   - **Rationale:** Limb replacement extends the severance system into gameplay progression,
     enabling prosthetic equipment.
   - **Verification:** Sever a limb. Attach a prosthetic. Assert physics constraints are restored
     and locomotion adapts to the prosthetic's parameters.
2. **R-4.3.9a** — Prosthetic Stat Modifiers: Prosthetic stat modifiers (strength, weight, speed)
   **SHALL** be active through the gameplay effect system after attachment.
   - **Rationale:** Prosthetics must have gameplay impact (stronger arm, heavier leg) to be
     meaningful equipment.
   - **Verification:** Attach a prosthetic with a strength modifier. Assert the modifier is active
     in the gameplay effect system.

## Non-Functional Requirements

| ID        | Derived From     |
|-----------|------------------|
| R-4.3.NF1 | R-4.3.1, R-4.3.7 |
| R-4.3.NF2 | R-4.3.5          |
| R-4.3.NF3 | R-4.3.6, R-4.3.7 |

1. **R-4.3.NF1** — Constraint Solver Throughput: The constraint solver **SHALL** process at least
   5,000 constraint rows per millisecond on minimum-spec hardware, supporting scenes with up to 500
   active joints at 8 solver iterations within the 4 ms physics budget.
   - **Rationale:** Complex scenes (ragdolls, vehicles, destruction chains) can accumulate thousands
     of constraint rows; the solver must handle peak loads without frame drops.
   - **Verification:** Benchmark -- create 500 active joints with 8 solver iterations; measure
     constraint solver wall-clock time; assert it completes within 4 ms.
2. **R-4.3.NF2** — Ragdoll Activation Latency: Ragdoll activation (spawning all joint and bone
   entities from a `RagdollDef` asset) **SHALL** complete within 0.5 ms per ragdoll, supporting at
   least 8 simultaneous ragdoll activations per frame without exceeding the physics budget.
   - **Rationale:** Mass-casualty events (explosions, area attacks) can activate multiple ragdolls
     in a single frame; activation must be fast enough to avoid hitches.
   - **Verification:** Benchmark -- activate 8 ragdolls (20 bones each) in a single frame; measure
     total activation time; assert it completes within 4 ms.
3. **R-4.3.NF3** — Joint Chain Stability: A joint chain of up to 32 segments **SHALL** remain stable
   (no segment separation exceeding 1 mm, no energy gain exceeding 1% per second) for at least 60
   seconds of continuous simulation at the minimum substep count of 4.
   - **Rationale:** Long-running rope and chain simulations must not accumulate drift or energy that
     causes visual artifacts or instability over time.
   - **Verification:** Stress test -- simulate a 32-segment pendulum chain for 60 seconds at 4
     substeps; measure total system energy and maximum segment separation each second; assert energy
     gain is below 1% and separation is below 1 mm throughout.
