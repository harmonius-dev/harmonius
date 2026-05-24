# Constraints and Joints

Angular and positional relationships between bodies maintained by constraint solving.

## What it covers

- Revolute joints: single-axis rotation (doors, wheels).
- Prismatic joints: single-axis translation (pistons, elevators).
- Ball-and-socket joints: 3-DOF rotation (swings, ragdoll shoulders).
- Fixed joints: zero relative motion (welding).
- Spring joints with configurable stiffness and damping.
- Limits: angle ranges and distance bounds on joint motion.
- Motors: powered joints applying torque or force to reach a target velocity.
- Constraint stabilization: Baumgarte correction preventing drift.
- Soft constraints: compliant joints with spring-like behavior.
- Point-to-point constraints for rope and cable simulation.
- Hinge breaks when force exceeds a threshold.

## Concepts

### Joint Types and Degrees of Freedom

Joints constrain the relative motion of two rigid bodies. Revolute joints allow one-axis rotation
(pinned doors). Prismatic joints allow one-axis translation (extending hydraulic arms). Ball joints
allow full 3-DOF rotation (ragdoll shoulder). Fixed joints eliminate all relative motion, effectively
welding bodies together. Each joint type specifies which linear and angular degrees of freedom are
locked or free.

### Constraint Solving

The physics engine solves constraints iteratively: for each constraint, compute the relative motion
violating it, then apply an impulse to bodies to restore compliance. Multiple passes (typically
4–8 iterations) improve stability. Baumgarte correction dampen drift accumulation, preventing
constraint violation over time.

### Joint Limits and Motors

Revolute joints configure angle limits (e.g., door stops at 90 degrees). Prismatic joints configure
distance limits (e.g., piston extends only 2 meters). Motor settings drive joints toward a target
velocity (spinning wheel), or a target angle (servo joint). Motors apply torque or force up to a
specified maximum; feedback control stabilizes around the target.

### Spring Constraints and Soft Bodies

Spring joints connect bodies with stiffness and damping, applying restoring force proportional to
distance from the rest length. This enables rope, cable, and cloth simulation. Soft constraints
deliberately allow small compliance for stability and visual appeal (hanging chains feel better
slightly stretchy than rigid).

### Breakable Constraints

Constraints break when the force to maintain them exceeds a threshold. This enables destructible
joints: ragdoll limbs detach when overstressed, or balconies collapse under load. Breaking triggers
gameplay events and physics state changes.

## How it fits

- See [dynamics.md](./dynamics.md) for velocity integration and impulse responses.
- See [../simulation/grids-and-volumes.md](../simulation/grids-and-volumes.md) for rigid body
  placement in spatial structures.
- Integrates with [../data-systems/attributes-and-effects.md](../data-systems/attributes-and-effects.md)
  for damage and constraint breakage events.
