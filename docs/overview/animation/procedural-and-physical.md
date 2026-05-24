# Procedural and Physical

Procedural bone motion, inverse kinematics, and physics-driven animation.

## What it covers

- Inverse kinematics (IK): positioning end effectors by solving for bone rotations.
- IK solvers: FABRIK, analytic two-bone, cyclic coordinate descent.
- Procedural motion: animated bone transforms via code (bouncing, swaying).
- Physics-driven animation: bones as physics bodies with constraints.
- Ragdoll mode: all bones physics-driven for limp bodies and impacts.
- Partial ragdoll: selective bones physics-driven (arms) while others animation-driven (torso).
- Hair and cloth simulation with bone or particle dynamics.
- Jiggle bones: soft-body secondary motion from velocity.
- Root motion extraction: using animation to drive character position.
- Striding and footstep placement from animation curves.

## Concepts

### Inverse Kinematics

Inverse kinematics solves the inverse problem: given an end-effector target position, find bone
rotations reaching it. FABRIK (Forward And Backward Reaching Inverse Kinematics) iteratively pulls
bones toward the target and back to the root, converging quickly. Analytic two-bone IK solves
exactly for elbow-joint angles reaching a wrist target. Cyclic coordinate descent rotates each
bone iteratively toward the target. IK systems constrain rotations to joint limits (elbows don't
bend backward).

### Procedural Motion

Procedural motion animates bones via code: apply sinusoidal motion to spine for breathing, apply
offset to arms for gesturing. Layered on top of clip-based animation, procedural offsets add life
without explicit keyframing. Bounce bones bob up-down; sway bones oscillate side-to-side.

### Physics-Driven and Ragdoll Animation

Physics-driven animation treats bones as rigid bodies connected by constraints (joints). Ragdoll
mode activates when character takes damage, transitioning from animation-driven to physics-driven.
Partial ragdoll keeps torso animation-driven but frees arms and legs to physics, enabling fall-and-catch
animations. Jiggle bones add secondary motion: bone velocity accumulates, dragging children
behind, creating jiggly physics-free motion.

### Root Motion

Root motion extracts character movement from the skeleton's root bone animation. Instead of moving
the character via velocity, animation keyframes move the root bone, and the character follows. This
enables smooth, animation-accurate movement: vaulting animations naturally propel the character
over obstacles.

### Hair, Cloth, and Secondary Motion

Hair and cloth simulate as chains of bones or particles connected by springs. Wind forces apply
lateral acceleration. Each frame integrates velocity and position, then solves distance constraints
keeping hair together. Attachment bones pin hair to the head; the hair then swings freely.

## How it fits

- See [skeletal-and-states.md](./skeletal-and-states.md) for animation clip playback and state
  machines.
- See [matching-and-blending.md](./matching-and-blending.md) for transitioning between animation
  styles.
- See [../physics/dynamics.md](../physics/dynamics.md) for rigid body simulation in ragdoll mode.
