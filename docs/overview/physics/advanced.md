# Advanced Physics

Specialized simulation for ropes, cloth, fluids, destruction, and character-environment
interaction.

## What it covers

- Rope and cable simulation: point-mass chains with constraints and damping.
- Cloth simulation: grid-based or particles with wind forces and collision constraints.
- Fluid simulation: shallow water equations or vortex particle methods.
- Destruction and fracture: prefab blueprints triggered by impact.
- Decal spawning on impact surfaces.
- Particle effects and sound triggers on collision.
- Penetration recovery: pushing overlapping bodies apart.
- Contact manifold generation for stable stacking.
- Friction models: static, kinetic, and rolling friction.
- Bounce and restitution curves parameterized by impact speed.

## Concepts

### Rope and Cloth

Ropes simulate as point-mass chains with distance constraints between consecutive points. Wind
forces apply lateral acceleration. Rope endpoints attach to rigid bodies or move along prescribed
paths. Cloth simulates similarly: a 2D grid of points with constraints connecting neighbors,
colliding against geometry. Wind drives cloth motion; pin constraints hold corners or edges to fixed
points. Cloth reads wind zones from the environment.

### Fluid and Particle Methods

Shallow water equations simulate standing water or rivers, storing per-cell height and flow velocity.
Vortex particle methods simulate swirling fluid zones for visceral smoke and water effects. Both
exchange energy with floating bodies (buoyancy) and accept sources (fountains, broken pipes).

### Destruction and Fracture

Impact triggers destruction prefabs, breaking the object into debris chunks and spawning decals on
impact surfaces. Decal placement uses hit point and surface normal. Particle effects like dust clouds
and spark streams trigger on collision; optional audio cues play impact sounds. Destruction blueprints
specify breakage patterns: some objects shatter into many pieces, others simply split at a plane.

### Contact Stability and Friction

Contact manifold generation produces multiple contact points per pair, stabilizing stacking of many
bodies. Friction curves provide smooth transitions: static friction (stiction) is higher than
kinetic friction; rolling friction applies to wheels. Bounce and restitution curves map impact
speed to rebound intensity (soft impacts bounce less, hard impacts rebound more). Penetration
recovery gently moves overlapping bodies apart to prevent tunnel-through.

## How it fits

- See [constraints-and-joints.md](./constraints-and-joints.md) for constraint formulations.
- See [../vfx/particles-and-effects.md](../vfx/particles-and-effects.md) for destruction effects
  and debris.
- See [../data-systems/attributes-and-effects.md](../data-systems/attributes-and-effects.md)
  for destruction prefabs and blueprints.
