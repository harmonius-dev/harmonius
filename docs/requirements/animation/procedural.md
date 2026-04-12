# R-9.3 -- Procedural Animation Requirements

## IK Solvers

1. **R-9.3.1** -- The engine **SHALL** provide a two-bone IK solver using the law of cosines with
   pole vector targets, running as a GPU post-process pass.
   - **Rationale:** Two-bone IK is the fundamental solver for limb targeting; GPU execution avoids
     CPU bottlenecks at scale.
   - **Verification:** Set a target for a two-bone arm chain. Verify the end effector reaches the
     target within tolerance. Verify the pole vector controls elbow orientation.

2. **R-9.3.2** -- The engine **SHALL** provide a CCD solver for 3-8 bone chains with configurable
   iteration limits and per-joint angular constraints.
   - **Rationale:** CCD handles medium-length chains like tails and spines where analytical
     solutions are not available.
   - **Verification:** Solve a 5-bone tail chain toward a target. Verify convergence within the
     iteration limit. Verify angular constraints are respected per joint.

3. **R-9.3.3** -- The engine **SHALL** provide a FABRIK solver for long chains and
   multi-end-effector problems with joint constraints.
   - **Rationale:** FABRIK handles complex IK setups like multi-limbed creatures more efficiently
     than CCD for long chains.
   - **Verification:** Solve a 10-bone chain with 2 end effectors. Verify both effectors reach their
     targets within tolerance.

## Ragdoll and Constraints

1. **R-9.3.4** -- The engine **SHALL** support full and partial ragdoll simulation with per-bone
   blend weights on async compute, with recovery transitions back to animation.
   - **Rationale:** Partial ragdoll enables hit reactions on specific body parts while maintaining
     animation control on others.
   - **Verification:** Enable partial ragdoll on the upper body. Verify upper bones respond to
     physics while lower bones follow animation. Verify recovery blends smoothly.

2. **R-9.3.5** -- The engine **SHALL** procedurally rotate head and spine bones to track world-space
   targets with configurable angle limits, blending with underlying animation.
   - **Rationale:** Look-at and aim constraints add liveliness to NPCs and enable weapon alignment
     without per-target authored clips.
   - **Verification:** Set a look-at target and verify head and spine rotate toward it. Verify angle
     limits are respected. Verify blending with base animation produces no discontinuities.

## Motion Matching

1. **R-9.3.6** -- The engine **SHALL** search a pre-built pose database for the best continuation of
   the current pose given a desired trajectory, using weighted feature distance metrics.
   - **Rationale:** Motion matching produces fluid locomotion from mocap data without hand-authored
     state machine transitions.
   - **Verification:** Query a motion database with a known pose and trajectory. Verify the selected
     frame minimizes the weighted distance metric. Verify matching runs CPU-side with GPU-evaluated
     blending.

## Foot Placement and Locomotion

1. **R-9.3.7** -- The engine **SHALL** raycast from foot bone positions to the ground and adjust leg
   IK targets to plant feet on uneven terrain, with pelvis height adjustment and stride adaptation.
   - **Rationale:** Foot placement prevents floating and skating on non-flat surfaces, critical for
     visual quality in open-world games.
   - **Verification:** Walk a character across stairs and verify feet plant on each step. Verify
     pelvis height adjusts to maintain natural leg extension.

2. **R-9.3.8** -- The engine **SHALL** support procedural locomotion for arbitrary skeleton
   topologies (biped, quadruped, hexapod, custom) via data-driven locomotion profiles.
   - **Rationale:** Procedural locomotion adapts to any creature without per-topology authoring,
     enabling diverse fauna.
   - **Verification:** Configure a quadruped locomotion profile. Verify gait pattern, phase offsets,
     and stride length produce natural movement on uneven terrain.

3. **R-9.3.9** -- The engine **SHALL** support physics-based locomotion driven by limb torques and
   ground reaction forces with configurable muscle strength and balance gains.
   - **Rationale:** Physics-driven locomotion produces emergent interaction with the environment for
     characters that need physical presence.
   - **Verification:** Push a physics-driven character and verify it stumbles and recovers. Verify
     locomotion on a slope adapts to the incline.

4. **R-9.3.10** -- The engine **SHALL** support runtime attachment and removal of skeletal
   sub-hierarchies via socket-based attachment points, with dismemberment that spawns detached
   sub-meshes as physics entities.
   - **Rationale:** Dynamic attachment and dismemberment enable equipment systems and combat gore
     without pre-authored mesh variants.
   - **Verification:** Attach a weapon to a socket and verify it follows the bone. Sever a limb and
     verify the detached mesh becomes a physics entity and the remaining skeleton adapts.

## Diagnostics

1. **R-9.3.11** -- The engine **SHALL** provide debug visualization for foot placement, IK chains,
   gait phase, balance, and muscle forces, stripped from shipping builds.
   - **Rationale:** Procedural locomotion is difficult to debug without visual feedback on solver
     state and physical forces.
   - **Verification:** Enable the overlay and verify all listed elements are visible. Verify the
     overlay is absent from shipping builds.

## Pose Source Composition and Advanced IK

1. **R-9.3.12** -- The engine **SHALL** provide a Full Body IK (FBIK) solver variant that unifies
   center-of-mass balance, momentum, and prioritized end-effector solving in a single solver pass,
   distinct from per-chain FABRIK.
   - **Rationale:** FBIK produces physically plausible poses when characters reach overhead, lean on
     surfaces, or counterbalance weight, which per-chain solvers cannot represent.
   - **Verification:** Place a character reaching for an overhead target. Assert FBIK shifts the
     center of mass forward and adjusts the opposite arm for balance. Assert FABRIK in isolation
     does not.
2. **R-9.3.13** -- The engine **SHALL** expose pose sources (keyframe clip, blend space, motion
   matching, motion capture, procedural IK, ragdoll, spring bones, procedural gait, learned policy)
   as interchangeable entries in an `AnimationLayerStack`, composable via override, additive, and
   multiply blend modes with bone masks.
   - **Rationale:** Decoupling pose sources from the layer stack lets complex creatures combine
     authored and procedural animation without bespoke glue code.
   - **Verification:** Build a layer stack combining a mocap base layer, a foot IK layer, and a
     spring bone refinement layer. Assert the final pose reflects all three composed in order. Swap
     the base layer to motion matching and assert the remaining layers continue to apply.
3. **R-9.3.14** -- The engine **SHALL** resolve spring bone collisions against character body
   capsule or SDF proxies, preventing capes, tails, and accessory chains from intersecting the
   owning character.
   - **Rationale:** Spring bones without body collision clip through the character mesh during rapid
     movement.
   - **Verification:** Enable spring bone collision on a cape. Rapidly rotate the character. Assert
     the cape does not penetrate the body capsule at any simulation step.
4. **R-9.3.15** -- The engine **SHALL** support composing motion capture base layers with foot IK,
   look-at, and spring bone refinement layers, using inertialization for transitions between mocap
   takes.
   - **Rationale:** Studio mocap workflows require post-process refinement and smooth transitions
     without baking corrections into every take.
   - **Verification:** Play a mocap take with foot IK and look-at refinement. Assert the final pose
     plants feet correctly on terrain and the head tracks the aim target. Transition to a second
     take via inertialization and assert the blend decays smoothly.
5. **R-9.3.16** -- The engine **SHALL** accept trained neural-network locomotion policies as a pose
   source producing joint torques from character movement state, blending through the
   `AnimationLayerStack`. (Research-tier feature; non-shipping.)
   - **Rationale:** Learned locomotion research may produce physically grounded motion that is
     impractical to author by hand; the pipeline must accept the policy as a first-class pose
     source.
   - **Verification:** Load a trained policy. Assert the layer stack executes it as a pose source.
     Assert the output drives joint torques on the target skeleton.

## Non-Functional Requirements

1. **R-9.3.NF1** -- The engine **SHALL** solve 500 two-bone IK chains within 0.5 ms GPU time.
   - **Rationale:** IK must scale to large character counts without exceeding animation frame
     budget.
   - **Verification:** Benchmark 500 two-bone chains. Assert GPU time stays within 0.5 ms.

2. **R-9.3.NF2** -- The engine **SHALL** evaluate foot placement for 100 characters within 0.3 ms
   CPU time.
   - **Rationale:** Foot placement raycasts and IK must stay within CPU budget for MMO-scale scenes.
   - **Verification:** Benchmark 100 characters with foot placement. Assert CPU time stays within
     0.3 ms.

3. **R-9.3.NF3** -- The engine **SHALL** simulate GPU cloth PBD for 1000 vertices within 0.5 ms GPU
   time on desktop.
   - **Rationale:** Cloth simulation must share GPU budget with skinning and rendering.
   - **Verification:** Benchmark a 1000-vertex cloth panel. Assert GPU time stays within 0.5 ms.
