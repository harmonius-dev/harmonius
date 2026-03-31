# R-4.3 -- Constraints & Joints Requirements

1. **R-4.3.1** -- The engine **SHALL** support revolute, prismatic, fixed, and distance joints as
   ECS entities with spring, cone-twist, and generic 6-DOF extensions.
   - **Rationale:** Standard joint types cover most mechanical and ragdoll connections.
   - **Verification:** Create each joint type. Assert correct constraint behavior. Assert 6-DOF
     per-axis locking works.

2. **R-4.3.2** -- The engine **SHALL** support optional JointMotor and JointLimits components with
   configurable target, max force, and limit softness.
   - **Rationale:** Motors and limits enable powered joints and bounded angular ranges.
   - **Verification:** Add a motor to a revolute joint. Assert it reaches target velocity. Assert
     limits clamp angular range.

3. **R-4.3.3** -- The engine **SHALL** support breakable joints with force/torque thresholds,
   emitting JointBroken events when exceeded.
   - **Rationale:** Breakable joints enable destruction and fracture-driven separation.
   - **Verification:** Apply force exceeding the threshold. Assert the joint despawns. Assert the
     JointBroken event carries correct force magnitude.

4. **R-4.3.4** -- The engine **SHALL** support ragdoll configurations from skeleton assets with
   anatomically plausible limits, with per-platform bone count caps.
   - **Rationale:** Ragdolls transition characters to physics on death or knockback.
   - **Verification:** Activate a ragdoll. Assert bones simulate within configured limits. Assert
     mobile uses a reduced bone count.

5. **R-4.3.5** -- The engine **SHALL** model ropes and chains as linked joint entity sequences with
   configurable segment count and stiffness.
   - **Rationale:** Ropes and chains are common environmental objects requiring constraint-based
     simulation.
   - **Verification:** Create a chain. Assert segments hang under gravity. Assert segment count
     scales per platform.

6. **R-4.3.6** -- The engine **SHALL** provide sequential impulse and TGS constraint solvers
   selectable via SolverConfig, deterministic given identical entity ordering.
   - **Rationale:** SI is simpler; TGS provides better convergence for stacking and vehicles.
   - **Verification:** Stack 10 boxes with TGS. Assert stable tower. Assert SI produces the same
     result with more iterations.

7. **R-4.3.7** -- The engine **SHALL** support limb severance via per-joint damage thresholds,
   spawning severed limbs as ragdoll entities and emitting JointSevered events.
   - **Rationale:** Limb severance enables dismemberment gameplay from Kenshi-style damage tracking.
   - **Verification:** Damage a limb joint past threshold. Assert the limb detaches as an
     independent entity. Assert the skeleton adapts.

8. **R-4.3.8** -- The engine **SHALL** support prosthetic and replacement limb attachment to severed
   sockets at runtime, re-establishing physics constraints and updating the skeleton hierarchy.
   - **Rationale:** Limb replacement enables prosthetic gameplay and regrowth mechanics.
   - **Verification:** Attach a prosthetic. Assert joint constraints re-establish. Assert locomotion
     profile updates.
