# Queries and Vehicles

Raycasting, shape overlap detection, and specialized simulation for wheeled and aerial vehicles.

## What it covers

- Raycasting: tracing rays to find the nearest intersecting shape.
- Shape overlap queries: finding all overlapping shapes in a region.
- Sweep tests: moving a shape along a path to detect collisions.
- Convex shape queries for proximity testing.
- Terrain raycasting for ground detection and footstep response.
- Wheeled vehicle physics: suspension springs, tire friction, and torque distribution.
- Steering and throttle inputs mapped to wheel angular velocity.
- Wheel load and slip detecting drift vs traction.
- Aerodynamic drag scaling with velocity.
- Helicopters and flying vehicles with rotor thrust vectors.
- Tanks and treaded vehicles with independent track force.

## Concepts

### Ray and Shape Queries

Raycasts find the nearest collision along a ray from origin to direction, reporting hit point and
surface normal. Sweep tests slide a shape along a path (e.g., a sphere sliding along the ground),
detecting collision points where the shape would first touch. Shape overlap queries find all shapes
within a region, used for proximity checks and AI perception. All query results include surface
material information for footstep sound and impact effects.

### Terrain Interaction and Foot Placement

Raycasts downward from character feet detect ground height for locomotion. Surface normals guide
foot rotation in IK solvers. Surface material affects footstep sounds and particle effects. Slope
angles determine sliding vs standing. Repeated raycasts per frame track ground contact through
physics frames.

### Vehicle Dynamics

Wheeled vehicles apply suspension springs per wheel, absorbing vertical motion. Tire friction
depends on slip angle and slip ratio; sideways slip (oversteering) reduces friction. Torque applies
to wheel axles from the engine; steer angle rotates the wheel orientation. Load-dependent friction
decreases with light wheels and increases with heavy load. Aerodynamic drag scales quadratically
with velocity.

### Specialized Vehicles

Helicopters and flying vehicles define rotor thrust vectors relative to the fuselage, allowing
pitch, roll, and yaw control via differential rotor speeds. Tanks and treads apply force via each
track, independent of wheels. Hover vehicles apply upward force to counteract gravity at tunable
altitudes.

## How it fits

- See [dynamics.md](./dynamics.md) for velocity and force integration.
- See [../simulation/spatial-awareness.md](../simulation/spatial-awareness.md) for spatial
  acceleration structures enabling fast queries.
- See [../input/devices-and-actions.md](../input/devices-and-actions.md) for vehicle input
  mapping.
