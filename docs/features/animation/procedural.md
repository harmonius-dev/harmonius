# 9.3 — Procedural Animation

## IK Solvers

### F-9.3.1 Two-Bone IK Solver

Solves two-bone IK chains (upper arm/lower arm, thigh/shin) analytically using the law of
cosines. Supports pole vector targets to control elbow and knee orientation. Runs on the GPU as
a post-process pass over the skinned skeleton, enabling hand placement on weapons, ledge grabs,
and mount grips for hundreds of characters simultaneously.

- **Requirements:** R-9.3.1
- **Dependencies:** F-9.1.1
- **Platform notes:** None

### F-9.3.2 CCD IK Solver

Cyclic Coordinate Descent solver for medium-length chains (3-8 bones) such as tails, spines,
and tentacles. Iteratively rotates each joint toward the target from tip to root. Converges in
few iterations for typical game chains and runs as a GPU compute pass with configurable
iteration limits and angular constraints per joint.

- **Requirements:** R-9.3.2
- **Dependencies:** F-9.1.1
- **Platform notes:** None

### F-9.3.3 FABRIK IK Solver

Forward And Backward Reaching Inverse Kinematics solver for long chains and multi-end-effector
problems. Operates in position space rather than rotation space, making it efficient for chains
with many bones. Supports joint constraints and multiple target priorities for complex setups
like spider legs or branching skeletal structures.

- **Requirements:** R-9.3.3
- **Dependencies:** F-9.1.1
- **Platform notes:** None

## Ragdoll Simulation

### F-9.3.4 Ragdoll Physics (Partial and Full)

Position-based dynamics ragdoll simulation on async compute with capsule and sphere collision
primitives driven by the skeleton. Supports full ragdoll (death, knockback) and partial ragdoll
where only a subset of bones are physics-driven while the rest follow animation. Blend weights
between animation and physics are configurable per bone for seamless transitions.

- **Requirements:** R-9.3.4
- **Dependencies:** F-9.1.1
- **Platform notes:** None

## Look-At and Aim Constraints

### F-9.3.5 Look-At and Aim Constraints

Procedurally rotates head and spine bones to track a world-space target with configurable angle
limits and blend weights. Aim constraints orient weapon-holding arms toward a target point for
aiming poses. Both constraints blend smoothly with underlying animation and respect joint
limits to avoid unnatural contortion, enabling NPCs to visually respond to nearby players.

- **Requirements:** R-9.3.5
- **Dependencies:** F-9.1.1
- **Platform notes:** None

## Motion Matching

### F-9.3.6 Motion Matching

Data-driven animation selection that searches a pose database for the best continuation of the
current pose given a desired trajectory. Replaces hand-authored state machine locomotion with
continuous best-fit selection from motion capture data. The search runs CPU-side with
GPU-evaluated blending, supporting responsive character control and natural transitions for
player characters and high-fidelity NPCs.

- **Requirements:** R-9.3.6
- **Dependencies:** F-9.1.2, F-9.1.3
- **Platform notes:** None

## Foot Placement and Procedural Locomotion

### F-9.3.7 Foot Placement and Procedural Locomotion

Raycasts from foot bone positions to the ground surface and adjusts leg IK targets to plant
feet on uneven terrain, stairs, and slopes. Pelvis height is adjusted to maintain natural leg
extension. Procedural stride adaptation modifies step timing and length based on movement speed
and terrain gradient, preventing foot sliding on inclines and enabling believable locomotion
without per-surface animation authoring.

- **Requirements:** R-9.3.7
- **Dependencies:** F-9.3.1, F-9.1.6
- **Platform notes:** None
