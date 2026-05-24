# Dynamics

Motion, forces, rigid body simulation, and velocity updates each frame.

## What it covers

- Rigid body simulation: position, rotation, linear and angular velocity.
- Forces and impulses: gravity, applied forces, collision impulses.
- Velocity integration: explicit Euler, semi-implicit Euler, and Verlet variants.
- Sleeping and waking: CPU cost reduction for stationary bodies.
- Continuous collision detection for fast-moving bodies preventing tunneling.
- Restitution: elastic and inelastic collision response.
- Damping: linear and angular velocity decay.
- Per-body shape hierarchy: multiple shapes per body for compound geometry.
- Buoyancy and fluid simulation for water environments.
- Ragdoll dynamics activated on character damage or death.

## Concepts

### Rigid Body Representation

Each body stores position, rotation, linear and angular velocity, mass, and inertia tensor. The
physics engine integrates velocity to update position and rotation each frame. Multiple collision
shapes attach to a single body for compound geometry; the engine computes inertia by combining all
shapes. Bodies transition between dynamic (simulated), kinematic (moved by code), and static
(immovable) states.

### Forces and Integration

Applied forces accumulate each frame: gravity, user-applied forces, and impulses from collisions.
The engine integrates velocity using semi-implicit Euler (momentum conservation) or Verlet (energy
stability). Fast-moving bodies enable continuous collision detection to prevent tunneling through
thin barriers; collision impulses apply instantly, avoiding numerical drift.

### Sleeping and Waking

When a body's velocity falls below a threshold for several frames, it enters a sleep state,
removing it from per-frame integration. Collision with an awake body wakes sleeping bodies. Sleeping
dramatically reduces CPU cost for dormant ragdolls, debris, and static environments.

### Advanced Motion

Buoyancy applies upward force proportional to submerged volume in defined fluid zones. Ragdoll
dynamics activate when ragdoll components (head, arms, legs) become free-floating after damage. The
engine then simulates each limb independently, constrained by joints to maintain skeleton coherence.

## How it fits

- See [constraints-and-joints.md](./constraints-and-joints.md) for angular constraints between
  bodies.
- See [queries-and-vehicles.md](./queries-and-vehicles.md) for shape raycasting and proximity
  queries.
- See [../simulation/spatial-awareness.md](../simulation/spatial-awareness.md) for broad-phase
  collision detection.
