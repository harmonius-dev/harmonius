# Physics

How rigid bodies, cloth, fluids, and vehicles move and collide.

## Topics

- [dynamics](./dynamics.md) — mass, forces, gravity, and rigid body integration.
- [constraints-and-joints](./constraints-and-joints.md) — hinges, balls, ropes, and
  motorized movement.
- [queries-and-vehicles](./queries-and-vehicles.md) — raycasts, sweeps, wheels, and suspension.
- [advanced](./advanced.md) — soft bodies, cloth, fluids, destruction, and fracture.

## Key takeaways

- Semi-implicit Euler integration with impulse-based collision response ensures momentum
  conservation and numerical stability over naive explicit schemes.
- Sleeping and waking reduce CPU by orders of magnitude for stationary bodies while maintaining
  instantaneous response on disturbance.
- Constraint solving iteratively (4–8 passes) via impulse methods with Baumgarte correction
  stabilizes stacking, joint limits, and powered motors without drift.
- Continuous collision detection for fast bodies prevents tunneling through thin geometry while
  remaining affordable vs discrete detection.
- Raycasts, sweeps, and shape overlap queries use acceleration structures (spatial grids, trees)
  reducing O(N) queries to O(log N).

## Integration risks

- Joint motor power tuning affects whole-game feel (vehicle response, door speed); over-tuning
  causes oscillation and instability. See [constraints-and-joints.md](./constraints-and-joints.md)
  for motor parameter scaling.
- Sleeping thresholds must account for game pacing: if threshold too high, objects stop
  prematurely; too low, CPU wasted on tiny wobbles. See [dynamics.md](./dynamics.md) for
  per-object config.
- Constraint iteration count (solver passes) directly impacts per-frame CPU; too low causes
  visible jitter and joint stretching; too high wastes performance. See [constraints-and-joints.md](./constraints-and-joints.md)
  for iteration guidance.
- Destruction triggering ragdoll must smoothly transition animation to physics to prevent
  glitchy limb snapping. See [advanced.md](./advanced.md) for ragdoll initialization.
- Buoyancy zones interact with player locomotion in water; misconfigured buoyancy makes swimming
  feel uncontrolled. See [dynamics.md](./dynamics.md) for tuning underwater feel.
