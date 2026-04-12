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

9. **R-4.3.9** -- The engine **SHALL** cache accumulated impulses from the previous substep in
   WarmStartData components and apply them at the start of each solve pass to accelerate
   convergence.
   - **Rationale:** Warm starting reduces the iteration count needed for stable stacking and complex
     joint chains.
   - **Verification:** Stack 10 boxes with and without warm starting. Assert warm-started solve
     converges in fewer iterations. Assert WarmStartData persists across substeps.

## LOD and Fallbacks

10. **R-4.3.10** -- The engine **SHALL** scale the cached `WarmStartData` impulses by a configurable
    per-platform warm-start factor at the start of each solve pass, with the factor tunable per
    `SolverConfig`.
    - **Rationale:** Tunable warm-start scaling trades convergence speed against oscillation risk
      differently per platform tier.
    - **Verification:** Vary the warm-start factor from 0.5 to 1.0 on a ragdoll stack. Assert
      convergence iterations decrease as the factor increases. Assert no oscillation at the
      configured default.
11. **R-4.3.11** -- The engine **SHALL** reduce ragdoll simulation fidelity based on camera distance
    via `RagdollLod` tiers, running full joint solve at near distance, reduced bone counts at mid
    distance, and replacing distant ragdolls with animation-driven blend at zero physics cost.
    - **Rationale:** Mass-casualty scenes must keep per-ragdoll cost within the physics budget
      without visibly popping distant ragdolls.
    - **Verification:** Place 32 ragdolls at varied distances. Assert near ragdolls run full solve,
      mid-distance ragdolls use the reduced bone set, and far ragdolls pay zero physics cost.
12. **R-4.3.12** -- The engine **SHALL** fall back to position-based Verlet integration without
    collision for chain joints beyond a configurable distance threshold, skipping the full
    constraint solver.
    - **Rationale:** Decorative environmental chains at distance should not consume solver time when
      Verlet approximation is visually sufficient.
    - **Verification:** Place a chain at twice the Verlet fallback distance. Assert the chain uses
      Verlet integration (no solver rows). Move the camera closer and assert the full solver
      resumes.

## Non-Functional Requirements

13. **R-4.3.NF1** -- The engine **SHALL** achieve constraint solver throughput of at least 5000 rows
    per millisecond, solving 500 joints within 4 ms.
    - **Rationale:** Joint-heavy scenes (ragdolls, vehicles, chains) must stay within the physics
      frame budget.
    - **Verification:** Benchmark 500 joints across mixed types. Assert total solve time stays
      within 4 ms.

14. **R-4.3.NF2** -- The engine **SHALL** activate a ragdoll from animation within 0.5 ms per
    ragdoll, supporting 8 simultaneous activations per frame.
    - **Rationale:** Mass-casualty events must not cause frame hitches.
    - **Verification:** Activate 8 ragdolls in a single frame. Assert per-ragdoll activation cost
      stays within 0.5 ms.

15. **R-4.3.NF3** -- The engine **SHALL** maintain chain stability for 32-segment chains over 60
    seconds with positional drift below 1 mm.
    - **Rationale:** Visible chain stretching breaks immersion for ropes, cables, and bridges.
    - **Verification:** Simulate a 32-segment chain under gravity for 60 s. Assert endpoint drift
      stays below 1 mm.
