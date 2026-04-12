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
