# R-4.3 — Constraints & Joints Requirements

## R-4.3.1 Core Joint Types

The engine **SHALL** support `Revolute`, `Prismatic`, `Fixed`, and `Distance` joint types as
ECS entities with `Joint` and `JointType` components, and maintain positional constraint error
below 1 mm after 100 solver iterations for each type.

- **Derived from:** [F-4.3.1](../../features/physics/constraints-and-joints.md)
- **Rationale:** These four joint types cover the majority of gameplay constraint needs (doors,
  drawbridges, sliding mechanisms, tethers).
- **Verification:** Unit test — for each joint type, connect two 1 kg bodies with the joint and
  apply external forces for 500 ticks at 8 solver iterations; measure positional drift at the
  anchor point; assert error is below 1 mm.

## R-4.3.2 Advanced Joint Types

The engine **SHALL** support `Spring`, `ConeTwist`, and `Generic6DOF` joint type variants, with
`ConeTwist` enforcing angular limits on all three axes within 0.5 degrees of the configured
bounds.

- **Derived from:** [F-4.3.2](../../features/physics/constraints-and-joints.md)
- **Rationale:** Ragdolls require cone-twist joints with accurate angular limits; spring joints
  enable suspension and elastic connections; 6DOF joints support arbitrary constraint authoring.
- **Verification:** Unit test — create a `ConeTwist` joint with a 45-degree cone limit; apply
  torque to drive the body past the limit; assert the resulting angle does not exceed 45.5
  degrees across 1000 ticks.

## R-4.3.3 Joint Motors and Limits

The engine **SHALL** drive a `JointMotor` to within 1% of its target velocity when the maximum
motor force is sufficient, and clamp joint motion to `JointLimits` bounds within 0.5 degrees
(angular) or 1 mm (linear) of the configured limits.

- **Derived from:** [F-4.3.3](../../features/physics/constraints-and-joints.md)
- **Rationale:** Motorized joints power gameplay mechanics (powered doors, turrets, crane arms)
  and limits prevent unnatural over-rotation.
- **Verification:** Unit test — set a revolute joint motor target to 2 rad/s with sufficient
  max force; measure steady-state angular velocity; assert it is within 1% of 2 rad/s. Set
  angular limits to +/-45 degrees; apply excess torque; assert the body does not exceed 45.5
  degrees.

## R-4.3.4 Breakable Joints

The engine **SHALL** despawn a joint entity and emit a `JointBroken` ECS event within one
substep of the accumulated constraint impulse exceeding the `BreakForce` threshold.

- **Derived from:** [F-4.3.4](../../features/physics/constraints-and-joints.md)
- **Rationale:** Breakable joints enable destruction chains, detachable vehicle parts, and
  gameplay-driven structural failure.
- **Verification:** Unit test — create a breakable joint with a force threshold of 1000 N;
  apply 1500 N of force to one body; assert the joint entity is despawned and a `JointBroken`
  event containing both body entity IDs and the breaking force magnitude is emitted within the
  same substep.

## R-4.3.5 Ragdoll Configuration

The engine **SHALL** spawn a complete set of joint entities from a `RagdollDef` asset that maps
skeleton bone entities to joint archetypes, with all joints active and connected within one
frame of activation.

- **Derived from:** [F-4.3.5](../../features/physics/constraints-and-joints.md)
- **Rationale:** Ragdolls are the primary death and hit-reaction animation across all genres
  with humanoid characters; they must activate reliably and instantly.
- **Verification:** Integration test — trigger ragdoll activation on a 20-bone skeleton; assert
  all expected joint entities are spawned with correct `JointType` and `JointLimits` components;
  verify all `Joint` body references point to valid bone entities; simulate 100 ticks and assert
  no constraint violations exceed 5 mm.

## R-4.3.6 Joint Chains and Ropes

The engine **SHALL** simulate a chain of N joint-connected rigid body segments with stable
behavior (no segment separation exceeding 1 mm) at substep counts of 4 or higher for chains
up to 32 segments.

- **Derived from:** [F-4.3.6](../../features/physics/constraints-and-joints.md)
- **Rationale:** Ropes, cables, and chains are common environmental and gameplay objects that
  must remain visually connected under dynamic forces.
- **Verification:** Integration test — create a 32-segment chain anchored at one end with
  gravity; simulate 2000 ticks at 4 substeps; measure the distance between each adjacent
  segment pair; assert no gap exceeds 1 mm.

## R-4.3.7 Constraint Solvers

The engine **SHALL** provide both Sequential Impulse (SI) and Temporal Gauss-Seidel (TGS)
solvers selectable via `SolverConfig`, with TGS producing at least 30% less constraint drift
than SI at the same iteration count for a 10-body joint chain benchmark.

- **Derived from:** [F-4.3.7](../../features/physics/constraints-and-joints.md)
- **Rationale:** TGS converges faster for complex constraint graphs, while SI is simpler and
  cheaper for light workloads; supporting both lets developers choose per-project.
- **Verification:** Benchmark — run a 10-body joint chain for 1000 ticks with both SI and TGS
  at 8 iterations; measure cumulative constraint drift; assert TGS drift is at least 30% lower
  than SI drift.

## R-4.3.8 Limb Severance and Joint Destruction

The engine **SHALL** support runtime severance of skeletal joints when accumulated damage
exceeds a per-joint configurable threshold. Severance **SHALL** spawn the detached bone chain
as an independent ragdoll entity within one frame of the threshold being crossed. The parent
skeleton **SHALL** continue functioning with adapted locomotion and gameplay stat penalties
applied via the effect system. A `JointSevered` event **SHALL** fire through the observer
system with the severed joint ID, parent entity, and spawned limb entity as payload.

- **Derived from:** [F-4.3.8](../../features/physics/constraints-and-joints.md)
- **Rationale:** Limb severance enables Kenshi/Monster Hunter-style progressive body damage that
  affects gameplay mechanics, not just visuals.
- **Verification:** Deal damage to a limb until the severance threshold; verify the limb detaches
  as a ragdoll entity within 1 frame; verify the parent skeleton's locomotion adapts; verify the
  JointSevered event fires with correct payload.

## R-4.3.9 Prosthetic and Limb Replacement

The engine **SHALL** support runtime attachment of replacement limbs to severed joint sockets.
Attachment **SHALL** re-establish physics constraints, update the bone hierarchy, and trigger
locomotion profile re-evaluation. Prosthetic stat modifiers **SHALL** be applied through the
gameplay effect system. Attachment and detachment **SHALL** complete within one frame.

- **Derived from:** [F-4.3.9](../../features/physics/constraints-and-joints.md)
- **Rationale:** Prosthetic replacement enables deep character customization and recovery mechanics
  where limb loss is not permanent.
- **Verification:** Sever a limb; attach a prosthetic; verify physics constraints are restored,
  locomotion adapts to the prosthetic's parameters, and stat modifiers are active.

---

## Non-Functional Requirements

### R-4.3.NF1 Constraint Solver Throughput

The constraint solver **SHALL** process at least 5,000 constraint rows per millisecond on
minimum-spec hardware, supporting scenes with up to 500 active joints at 8 solver iterations
within the 4 ms physics budget.

- **Derived from:** R-4.3.1, R-4.3.7
- **Rationale:** Complex scenes (ragdolls, vehicles, destruction chains) can accumulate
  thousands of constraint rows; the solver must handle peak loads without frame drops.
- **Verification:** Benchmark — create 500 active joints with 8 solver iterations; measure
  constraint solver wall-clock time; assert it completes within 4 ms.

### R-4.3.NF2 Ragdoll Activation Latency

Ragdoll activation (spawning all joint and bone entities from a `RagdollDef` asset) **SHALL**
complete within 0.5 ms per ragdoll, supporting at least 8 simultaneous ragdoll activations
per frame without exceeding the physics budget.

- **Derived from:** R-4.3.5
- **Rationale:** Mass-casualty events (explosions, area attacks) can activate multiple ragdolls
  in a single frame; activation must be fast enough to avoid hitches.
- **Verification:** Benchmark — activate 8 ragdolls (20 bones each) in a single frame; measure
  total activation time; assert it completes within 4 ms.

### R-4.3.NF3 Joint Chain Stability

A joint chain of up to 32 segments **SHALL** remain stable (no segment separation exceeding
1 mm, no energy gain exceeding 1% per second) for at least 60 seconds of continuous simulation
at the minimum substep count of 4.

- **Derived from:** R-4.3.6, R-4.3.7
- **Rationale:** Long-running rope and chain simulations must not accumulate drift or energy
  that causes visual artifacts or instability over time.
- **Verification:** Stress test — simulate a 32-segment pendulum chain for 60 seconds at 4
  substeps; measure total system energy and maximum segment separation each second; assert
  energy gain is below 1% and separation is below 1 mm throughout.
