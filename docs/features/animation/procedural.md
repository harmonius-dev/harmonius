# 9.3 — Procedural Animation

## IK Solvers

| ID      | Feature            |
|---------|--------------------|
| F-9.3.1 | Two-Bone IK Solver |
| F-9.3.2 | CCD IK Solver      |
| F-9.3.3 | FABRIK IK Solver   |

1. **F-9.3.1** — Solves two-bone IK chains analytically using the law of cosines. Supports pole
   vector targets to control elbow and knee orientation. Runs on the GPU as a post-process pass over
   the skinned skeleton.
   - **Deps:** F-9.1.1
   - **Platform:** Two-bone IK is lightweight and runs on all tiers. Active IK chain count per
     frame: mobile 20-40, Switch 80, desktop 500+.
2. **F-9.3.2** — Cyclic Coordinate Descent solver for medium-length chains (3-8 bones) such as
   tails, spines, and tentacles. Iteratively rotates each joint toward the target from tip to root.
   Runs as a GPU compute pass with configurable iteration limits and angular constraints per joint.
   - **Deps:** F-9.1.1
   - **Platform:** CCD iteration count scales per tier: mobile 2-4, Switch 6, desktop 8-12.
3. **F-9.3.3** — Forward And Backward Reaching Inverse Kinematics solver for long chains and
   multi-end-effector problems. Operates in position space rather than rotation space. Supports
   joint constraints and multiple target priorities.
   - **Deps:** F-9.1.1
   - **Platform:** FABRIK iteration count scales per tier: mobile 2-3, desktop 6-8.
     Multi-end-effector problems simplified on mobile.

## Ragdoll Simulation

| ID      | Feature                            |
|---------|------------------------------------|
| F-9.3.4 | Ragdoll Physics (Partial and Full) |

1. **F-9.3.4** — Position-based dynamics ragdoll simulation on async compute with capsule and sphere
   collision primitives. Supports full ragdoll and partial ragdoll with per-bone blend weights.
   Ragdoll physical simulation is owned by the physics domain (F-4.3.5). This feature defines the
   animation integration: per-bone blend weights, partial ragdoll masking, and recovery transitions.
   - **Deps:** F-9.1.1, F-4.3.5 (Joint-Based Ragdoll)
   - **Platform:** Ragdoll body count scales per tier: mobile 4-8 capsules, Switch 12, desktop
     16-20. Partial ragdoll limited to hero characters on mobile.

## Look-At and Aim Constraints

| ID      | Feature                     |
|---------|-----------------------------|
| F-9.3.5 | Look-At and Aim Constraints |

1. **F-9.3.5** — Procedurally rotates head and spine bones to track a world-space target with
   configurable angle limits and blend weights. Aim constraints orient weapon-holding arms toward a
   target point. Both blend smoothly with underlying animation and respect joint limits.
   - **Deps:** F-9.1.1
   - **Platform:** Lightweight on all tiers. Active constraint count limited by per-tier animation
     instance budget (see F-9.1.5).

## Motion Matching

| ID      | Feature         |
|---------|-----------------|
| F-9.3.6 | Motion Matching |

1. **F-9.3.6** — Data-driven animation selection that searches a pose database for the best
   continuation of the current pose given a desired trajectory. Replaces hand-authored state machine
   locomotion with continuous best-fit selection from motion capture data. Search runs CPU-side with
   GPU-evaluated blending.
   - **Deps:** F-9.1.2, F-9.1.3
   - **Platform:** Pose database search cost scales with database size. Mobile uses smaller
     databases. Desktop supports full mocap libraries.

## Foot Placement and Procedural Locomotion

| ID      | Feature                                  |
|---------|------------------------------------------|
| F-9.3.7 | Foot Placement and Procedural Locomotion |

1. **F-9.3.7** — Raycasts from foot bone positions to the ground surface and adjusts leg IK targets
   to plant feet on uneven terrain, stairs, and slopes. Pelvis height is adjusted to maintain
   natural leg extension. Procedural stride adaptation modifies step timing and length based on
   movement speed and terrain gradient.
   - **Deps:** F-9.3.1, F-9.1.6
   - **Platform:** Foot placement raycasts per character: mobile 2 (feet only), desktop 4 (feet +
     pelvis + slope probe). Disabled for distant characters on mobile.

## Multi-Skeleton Procedural Locomotion

| ID      | Feature                              |
|---------|--------------------------------------|
| F-9.3.8 | Multi-Skeleton Procedural Locomotion |

1. **F-9.3.8** — Procedural locomotion that adapts to arbitrary skeleton topologies (biped,
   quadruped, hexapod, centaur, serpentine, custom). Each skeleton defines a locomotion profile
   specifying leg groups, phase offsets, gait patterns, stride length curves, and body sway
   parameters. Fully driven by ECS components (`LocomotionProfile`, `GaitState`, `FootGroup`).
   - **Deps:** F-9.3.1 (IK Solvers), F-9.3.5 (Foot Placement), F-1.9.4 (Spatial Query)
   - **Platform:** Supported on all tiers. IK iterations and foot group count reduced on mobile.
     Hexapod+ topologies may use simplified gait patterns on mobile.

## Physics-Based Locomotion

| ID      | Feature                  |
|---------|--------------------------|
| F-9.3.9 | Physics-Based Locomotion |

1. **F-9.3.9** — Locomotion driven by physical forces rather than root motion. Each limb applies
   torques and ground reaction forces through the physics system (F-4.1.1). Configurable muscle
   strength, joint damping, balance PID gains, and stumble recovery thresholds. Supports ragdoll
   blending for smooth animated-to-physics transitions.
   - **Deps:** F-9.3.8, F-4.1.1 (Rigid Body ECS), F-4.3.1 (Joint Entities), F-9.3.4 (Ragdoll)
   - **Platform:** Limited to hero/player characters on mobile. Desktop supports multiple
     simultaneous physics-driven characters.

## Procedural Attachment and Dismemberment

| ID       | Feature                                 |
|----------|-----------------------------------------|
| F-9.3.10 | Procedural Attachment and Dismemberment |

1. **F-9.3.10** — Runtime attachment and removal of skeletal sub-hierarchies (monster heads, tails,
   wings, weapon holsters, armor plates). Socket-based attachment points. Dismemberment severs a
   bone chain, spawning the detached sub-mesh as an independent physics entity. Remaining skeleton
   adapts locomotion (F-9.3.8) to compensate. ECS operations via command buffers (F-1.1.32).
   - **Deps:** F-9.3.8, F-9.1.1, F-4.3.5 (Ragdoll), F-1.1.32 (Command Buffers)
   - **Platform:** Active dismembered part count capped per tier: mobile 2-4, desktop 8-16.
     Simplified ragdoll on mobile.

## Locomotion Diagnostics and Visualization

| ID       | Feature                                  |
|----------|------------------------------------------|
| F-9.3.11 | Locomotion Diagnostics and Visualization |

1. **F-9.3.11** — Debug visualization overlay for procedural locomotion: foot placement targets, IK
   chain solve states, ground contact normals, gait phase diagrams, balance center-of-mass
   indicator, muscle force vectors, and skeleton wire-frame. Per-entity and per-feature toggles.
   Locomotion profiler panel shows per-frame IK iterations, foot plant errors, and physics force
   magnitudes.
   - **Deps:** F-9.3.8, F-9.3.9, F-15.5.6 (Stat Overlays)
   - **Platform:** Development-only; stripped from shipping builds on all platforms.

## Pose Source Composition and Advanced IK

| ID       | Feature |
|----------|----------------------------------- |
| F-9.3.12 | Full Body IK Solver (FBIK) |
| F-9.3.13 | Composable Animation Pose Sources |
| F-9.3.14 | Spring Bone Collision with Character Body |
| F-9.3.15 | Learned Locomotion Policy Pose Source |

1. **F-9.3.12** — A full-body IK solver variant that unifies center-of-mass balance, momentum
   tracking, and prioritized end-effector targets in a single solve pass. Distinct from per-chain
   FABRIK, FBIK produces physically plausible reaching, leaning, and counterbalance poses from a
   sparse set of end-effector targets plus a center-of-mass target.
   - **Deps:** F-9.3.1 (Two-Bone IK), F-9.3.3 (FABRIK)
   - **Platform:** FBIK enabled for hero characters on all tiers. Mobile falls back to FABRIK for
     background characters.
2. **F-9.3.13** — `AnimationLayerStack` accepts pose sources (`KeyframeClip`, `BlendSpace`,
   `MotionMatching`, `MotionCapture`, `ProceduralIk`, `Ragdoll`, `SpringBones`, `ProceduralGait`,
   `LearnedPolicy`) as interchangeable entries composable via override, additive, and multiply blend
   modes with bone masks. The stack evaluates sources in order, feeding the accumulated pose into
   the next layer.
   - **Deps:** F-9.1.4 (Animation Layers), F-9.3.4 (Ragdoll), F-9.3.6 (Motion Matching)
3. **F-9.3.14** — Spring bones carry a `CollideWith` reference to a character body capsule or signed
   distance field proxy. Each simulation substep projects spring bone positions out of the proxy
   volume, preventing capes, tails, and hair chains from clipping into the owning character.
   Collision cost scales with spring bone count, not mesh triangle count.
   - **Deps:** F-9.3.13
   - **Platform:** Collision resolution enabled on all tiers for hero characters; disabled for
     background characters on mobile.
4. **F-9.3.15** — A `LearnedPolicy` pose source loads a trained neural-network locomotion model and
   evaluates it per frame against character movement state, producing joint torques that drive a
   physics-based locomotion layer. Research-tier feature shipped disabled by default; enables
   experimentation with learned motion without forking the animation pipeline.
   - **Deps:** F-9.3.9 (Physics-Based Locomotion), F-9.3.13
   - **Platform:** Desktop and high-end PC only. Mobile and Switch not supported.
