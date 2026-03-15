# 9.3 — Procedural Animation

## IK Solvers

### F-9.3.1 Two-Bone IK Solver

Solves two-bone IK chains (upper arm/lower arm, thigh/shin) analytically using the law of cosines.
Supports pole vector targets to control elbow and knee orientation. Runs on the GPU as a
post-process pass over the skinned skeleton, enabling hand placement on weapons, ledge grabs, and
mount grips for hundreds of characters simultaneously.

- **Requirements:** R-9.3.1
- **Dependencies:** F-9.1.1
- **Platform notes:** Two-bone IK is lightweight and runs on all tiers. Active IK chain count per
  frame: mobile 20-40, Switch 80, desktop 500+.

### F-9.3.2 CCD IK Solver

Cyclic Coordinate Descent solver for medium-length chains (3-8 bones) such as tails, spines, and
tentacles. Iteratively rotates each joint toward the target from tip to root. Converges in few
iterations for typical game chains and runs as a GPU compute pass with configurable iteration limits
and angular constraints per joint.

- **Requirements:** R-9.3.2
- **Dependencies:** F-9.1.1
- **Platform notes:** CCD iteration count scales per tier: mobile 2-4 iterations, Switch 6, desktop
  8-12. Fewer iterations trade accuracy for speed.

### F-9.3.3 FABRIK IK Solver

Forward And Backward Reaching Inverse Kinematics solver for long chains and multi-end-effector
problems. Operates in position space rather than rotation space, making it efficient for chains with
many bones. Supports joint constraints and multiple target priorities for complex setups like spider
legs or branching skeletal structures.

- **Requirements:** R-9.3.3
- **Dependencies:** F-9.1.1
- **Platform notes:** FABRIK iteration count scales per tier: mobile 2-3 iterations, desktop 6-8.
  Multi-end-effector problems simplified on mobile (fewer targets).

## Ragdoll Simulation

### F-9.3.4 Ragdoll Physics (Partial and Full)

Position-based dynamics ragdoll simulation on async compute with capsule and sphere collision
primitives driven by the skeleton. Supports full ragdoll (death, knockback) and partial ragdoll
where only a subset of bones are physics-driven while the rest follow animation. Blend weights
between animation and physics are configurable per bone for seamless transitions.

Ragdoll physical simulation is owned by the physics domain (F-4.3.5 joint-based ragdoll). This
feature defines the animation integration: per-bone blend weights between animated and ragdoll
poses, partial ragdoll (upper body ragdoll while legs animate), ragdoll-to-animated recovery
transitions, and async compute evaluation of blend weights.

- **Requirements:** R-9.3.4
- **Dependencies:** F-9.1.1, F-4.3.5 (Joint-Based Ragdoll)
- **Platform notes:** Ragdoll body count scales per tier: mobile 4-8 capsules, Switch 12, desktop
  16-20. Partial ragdoll limited to hero characters on mobile.

## Look-At and Aim Constraints

### F-9.3.5 Look-At and Aim Constraints

Procedurally rotates head and spine bones to track a world-space target with configurable angle
limits and blend weights. Aim constraints orient weapon-holding arms toward a target point for
aiming poses. Both constraints blend smoothly with underlying animation and respect joint limits to
avoid unnatural contortion, enabling NPCs to visually respond to nearby players.

- **Requirements:** R-9.3.5
- **Dependencies:** F-9.1.1
- **Platform notes:** Look-at/aim constraints are lightweight on all tiers. Active constraint count
  limited by per-tier animation instance budget (see F-9.1.5).

## Motion Matching

### F-9.3.6 Motion Matching

Data-driven animation selection that searches a pose database for the best continuation of the
current pose given a desired trajectory. Replaces hand-authored state machine locomotion with
continuous best-fit selection from motion capture data. The search runs CPU-side with GPU-evaluated
blending, supporting responsive character control and natural transitions for player characters and
high-fidelity NPCs.

- **Requirements:** R-9.3.6
- **Dependencies:** F-9.1.2, F-9.1.3
- **Platform notes:** Motion matching pose database search cost scales with database size. Mobile
  uses smaller motion databases with fewer poses. Desktop supports full mocap libraries.

## Foot Placement and Procedural Locomotion

### F-9.3.7 Foot Placement and Procedural Locomotion

Raycasts from foot bone positions to the ground surface and adjusts leg IK targets to plant feet on
uneven terrain, stairs, and slopes. Pelvis height is adjusted to maintain natural leg extension.
Procedural stride adaptation modifies step timing and length based on movement speed and terrain
gradient, preventing foot sliding on inclines and enabling believable locomotion without per-surface
animation authoring.

- **Requirements:** R-9.3.7
- **Dependencies:** F-9.3.1, F-9.1.6
- **Platform notes:** Foot placement raycasts per character: mobile 2 (feet only), desktop 4 (feet +
  pelvis adjust + slope probe). Disabled for distant characters on mobile.

## Multi-Skeleton Procedural Locomotion

### F-9.3.8 Multi-Skeleton Procedural Locomotion

Procedural locomotion system that adapts to arbitrary skeleton topologies — humanoid (biped),
quadruped, hexapod (insects), centaur (hybrid), serpentine (snakes, dragons), and custom
configurations. Each skeleton defines a locomotion profile specifying: leg groups with phase
offsets, foot contact points, gait patterns (walk, trot, canter, gallop), stride length curves, and
body sway parameters. The system procedurally generates foot placement cycles, body oscillation, and
weight shifting based on velocity, terrain slope, and turn rate. Fully driven by ECS components
(`LocomotionProfile`, `GaitState`, `FootGroup`).

- **Requirements:** R-9.3.8
- **Dependencies:** F-9.3.1 (IK Solvers), F-9.3.5 (Foot Placement), F-1.9.4 (Spatial Query)
- **Platform notes:** Multi-skeleton locomotion supported on all tiers. IK iterations and foot group
  count per skeleton reduced on mobile. Hexapod+ topologies may use simplified gait patterns on
  mobile.

## Physics-Based Locomotion

### F-9.3.9 Physics-Based Locomotion

Locomotion driven by physical forces rather than root motion, producing emergent movement that
responds naturally to slopes, obstacles, and external forces. Each limb applies torques and ground
reaction forces through the physics system (F-4.1.1). Configurable parameters include muscle
strength, joint damping, balance PID gains, and stumble recovery thresholds. Supports ragdoll
blending — smoothly transition from animated to physics-driven locomotion when hit by impacts, and
recover to animated locomotion after regaining balance. Integrates with the hitbox system for
limb-specific damage responses.

- **Requirements:** R-9.3.9
- **Dependencies:** F-9.3.8, F-4.1.1 (Rigid Body ECS), F-4.3.1 (Joint Entities), F-9.3.4 (Ragdoll)
- **Platform notes:** Physics-based locomotion limited to hero/player characters on mobile due to
  per-limb force computation cost. Desktop supports multiple simultaneous physics-driven characters.

## Procedural Attachment and Dismemberment

### F-9.3.10 Procedural Attachment and Dismemberment

Runtime attachment and removal of skeletal sub-hierarchies — monster heads, tails, wings, weapon
holsters, and armor plates. Each attachment point is a socket defined on the parent skeleton with
transform offset and allowed attachment types. Dismemberment severs a bone chain from the skeleton,
spawning the detached sub-mesh as an independent physics-simulated entity with its own ragdoll. The
remaining skeleton adapts locomotion (F-9.3.8) to compensate — a quadruped losing a leg switches to
a three-legged gait. Attachment and detachment are ECS operations using command buffers (F-1.1.32).

- **Requirements:** R-9.3.10
- **Dependencies:** F-9.3.8, F-9.1.1 (GPU Skinning), F-4.3.5 (Ragdoll), F-1.1.32 (Command Buffers)
- **Platform notes:** Dismemberment spawns physics entities — active dismembered part count capped
  per tier: mobile 2-4, desktop 8-16. Detached sub-meshes use simplified ragdoll on mobile.

## Locomotion Diagnostics and Visualization

### F-9.3.11 Locomotion Diagnostics and Visualization

Debug visualization overlay for procedural locomotion: foot placement targets (predicted vs actual),
IK chain solve states, ground contact normals, gait phase diagrams, balance center-of-mass
indicator, muscle force vectors, and skeleton wire-frame with bone axes. Visualization toggles
per-entity and per-feature. A locomotion profiler panel shows per-frame IK iteration counts, foot
plant errors, and physics force magnitudes. Accessible from the editor's animation preview viewport
and the runtime debug overlay.

- **Requirements:** R-9.3.11
- **Dependencies:** F-9.3.8, F-9.3.9, F-15.5.6 (Stat Overlays)
- **Platform notes:** Debug visualization is development-only and stripped from shipping builds on
  all platforms. No runtime platform scaling required.
