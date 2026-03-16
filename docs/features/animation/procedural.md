# 9.3 — Procedural Animation

## IK Solvers

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|-------------|--------------|----------------|
| F-9.3.1 | Two-Bone IK Solver | Solves two-bone IK chains analytically using the law of cosines. Supports pole vector targets to control elbow and knee orientation. Runs on the GPU as a post-process pass over the skinned skeleton. | R-9.3.1 | F-9.1.1 | Two-bone IK is lightweight and runs on all tiers. Active IK chain count per frame: mobile 20-40, Switch 80, desktop 500+. |
| F-9.3.2 | CCD IK Solver | Cyclic Coordinate Descent solver for medium-length chains (3-8 bones) such as tails, spines, and tentacles. Iteratively rotates each joint toward the target from tip to root. Runs as a GPU compute pass with configurable iteration limits and angular constraints per joint. | R-9.3.2 | F-9.1.1 | CCD iteration count scales per tier: mobile 2-4, Switch 6, desktop 8-12. |
| F-9.3.3 | FABRIK IK Solver | Forward And Backward Reaching Inverse Kinematics solver for long chains and multi-end-effector problems. Operates in position space rather than rotation space. Supports joint constraints and multiple target priorities. | R-9.3.3 | F-9.1.1 | FABRIK iteration count scales per tier: mobile 2-3, desktop 6-8. Multi-end-effector problems simplified on mobile. |

## Ragdoll Simulation

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|-------------|--------------|----------------|
| F-9.3.4 | Ragdoll Physics (Partial and Full) | Position-based dynamics ragdoll simulation on async compute with capsule and sphere collision primitives. Supports full ragdoll and partial ragdoll with per-bone blend weights. Ragdoll physical simulation is owned by the physics domain (F-4.3.5). This feature defines the animation integration: per-bone blend weights, partial ragdoll masking, and recovery transitions. | R-9.3.4 | F-9.1.1, F-4.3.5 (Joint-Based Ragdoll) | Ragdoll body count scales per tier: mobile 4-8 capsules, Switch 12, desktop 16-20. Partial ragdoll limited to hero characters on mobile. |

## Look-At and Aim Constraints

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|-------------|--------------|----------------|
| F-9.3.5 | Look-At and Aim Constraints | Procedurally rotates head and spine bones to track a world-space target with configurable angle limits and blend weights. Aim constraints orient weapon-holding arms toward a target point. Both blend smoothly with underlying animation and respect joint limits. | R-9.3.5 | F-9.1.1 | Lightweight on all tiers. Active constraint count limited by per-tier animation instance budget (see F-9.1.5). |

## Motion Matching

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|-------------|--------------|----------------|
| F-9.3.6 | Motion Matching | Data-driven animation selection that searches a pose database for the best continuation of the current pose given a desired trajectory. Replaces hand-authored state machine locomotion with continuous best-fit selection from motion capture data. Search runs CPU-side with GPU-evaluated blending. | R-9.3.6 | F-9.1.2, F-9.1.3 | Pose database search cost scales with database size. Mobile uses smaller databases. Desktop supports full mocap libraries. |

## Foot Placement and Procedural Locomotion

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|-------------|--------------|----------------|
| F-9.3.7 | Foot Placement and Procedural Locomotion | Raycasts from foot bone positions to the ground surface and adjusts leg IK targets to plant feet on uneven terrain, stairs, and slopes. Pelvis height is adjusted to maintain natural leg extension. Procedural stride adaptation modifies step timing and length based on movement speed and terrain gradient. | R-9.3.7 | F-9.3.1, F-9.1.6 | Foot placement raycasts per character: mobile 2 (feet only), desktop 4 (feet + pelvis + slope probe). Disabled for distant characters on mobile. |

## Multi-Skeleton Procedural Locomotion

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|-------------|--------------|----------------|
| F-9.3.8 | Multi-Skeleton Procedural Locomotion | Procedural locomotion that adapts to arbitrary skeleton topologies (biped, quadruped, hexapod, centaur, serpentine, custom). Each skeleton defines a locomotion profile specifying leg groups, phase offsets, gait patterns, stride length curves, and body sway parameters. Fully driven by ECS components (`LocomotionProfile`, `GaitState`, `FootGroup`). | R-9.3.8 | F-9.3.1 (IK Solvers), F-9.3.5 (Foot Placement), F-1.9.4 (Spatial Query) | Supported on all tiers. IK iterations and foot group count reduced on mobile. Hexapod+ topologies may use simplified gait patterns on mobile. |

## Physics-Based Locomotion

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|-------------|--------------|----------------|
| F-9.3.9 | Physics-Based Locomotion | Locomotion driven by physical forces rather than root motion. Each limb applies torques and ground reaction forces through the physics system (F-4.1.1). Configurable muscle strength, joint damping, balance PID gains, and stumble recovery thresholds. Supports ragdoll blending for smooth animated-to-physics transitions. | R-9.3.9 | F-9.3.8, F-4.1.1 (Rigid Body ECS), F-4.3.1 (Joint Entities), F-9.3.4 (Ragdoll) | Limited to hero/player characters on mobile. Desktop supports multiple simultaneous physics-driven characters. |

## Procedural Attachment and Dismemberment

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|-------------|--------------|----------------|
| F-9.3.10 | Procedural Attachment and Dismemberment | Runtime attachment and removal of skeletal sub-hierarchies (monster heads, tails, wings, weapon holsters, armor plates). Socket-based attachment points. Dismemberment severs a bone chain, spawning the detached sub-mesh as an independent physics entity. Remaining skeleton adapts locomotion (F-9.3.8) to compensate. ECS operations via command buffers (F-1.1.32). | R-9.3.10 | F-9.3.8, F-9.1.1, F-4.3.5 (Ragdoll), F-1.1.32 (Command Buffers) | Active dismembered part count capped per tier: mobile 2-4, desktop 8-16. Simplified ragdoll on mobile. |

## Locomotion Diagnostics and Visualization

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|-------------|--------------|----------------|
| F-9.3.11 | Locomotion Diagnostics and Visualization | Debug visualization overlay for procedural locomotion: foot placement targets, IK chain solve states, ground contact normals, gait phase diagrams, balance center-of-mass indicator, muscle force vectors, and skeleton wire-frame. Per-entity and per-feature toggles. Locomotion profiler panel shows per-frame IK iterations, foot plant errors, and physics force magnitudes. | R-9.3.11 | F-9.3.8, F-9.3.9, F-15.5.6 (Stat Overlays) | Development-only; stripped from shipping builds on all platforms. |
