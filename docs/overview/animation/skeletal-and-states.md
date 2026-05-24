# Skeletal and States

Bone hierarchies, animation playback, state machines, and skeletal blending.

## What it covers

- Skeleton: bones and hierarchical relationships (parent/child).
- Bone transforms: local position, rotation, scale per bone.
- Forward kinematics: computing world transforms from bone hierarchy.
- Skeleton instances: multiple characters sharing bone structure.
- Animation clips: keyframed bone poses over time.
- Playback control: play, pause, stop, loop, speed, playback position.
- Blending: crossfading between clips with configurable duration.
- State machines: animation states (idle, walk, run, jump) and transitions with conditions.
- Transition layering: overlaying animations on top of base animations.
- Additive blending: stacking directional animations on motion animations.

## Concepts

### Skeleton Representation

A skeleton stores named bones organized hierarchically. Each bone has local position, rotation, and
scale relative to its parent. Forward kinematics computes world transforms by walking the hierarchy:
root bone's world transform equals local; child's world transform = parent's world × child's local.
This hierarchy enables limb IK solvers: moving a hand-end effector, the IK solver rotates parent
bones to reach the target.

### Animation Clips and Keyframing

Animation clips store keyframes: at time T, bone B has position P and rotation R. The engine
interpolates between keyframes. Linear interpolation (LERP) suffices for position; spherical linear
interpolation (SLERP) works for rotation. Hermite or Catmull-Rom curves provide smoother results
at higher computational cost.

### Playback and Blending

Playback advances a time cursor through the clip. Looping restarts from time 0 when reaching clip
end. Crossfading between clips blends their poses: old_clip × (1 – t) + new_clip × t over time t ∈
[0, 1]. Layering overlays animations: base animation (walk) plus directional layer (strafe left)
produces strafe-walk. Additive blending allows stacking: base pose + offset pose, useful for
recoil or flinch.

### State Machines

State machines organize animations into states (idle, walk, run) connected by transitions. Transitions
have conditions: "speed > 0" triggers walk from idle. Transitions can have durations for blending
rather than cuts. Hierarchical state machines allow nested states: locomotion (idle, walk, run,
jump) as a parent state, each with sub-states.

### Advanced Blending Techniques

Weight-based blending combines multiple clips with independent weights (e.g., 60% walk, 40%
strafe-left). Phase-aligned blending synchronizes clips to the same phase (e.g., both at midstride)
before blending, preventing foot sliding. Motion matching finds the closest pose in a database of
recorded motion, enabling natural transitions between any states.

## How it fits

- See [procedural-and-physical.md](./procedural-and-physical.md) for procedural bone motion and
  physics-based animation.
- See [matching-and-blending.md](./matching-and-blending.md) for advanced blending techniques.
- See [character-and-first-person.md](./character-and-first-person.md) for character animation
  specifics.
