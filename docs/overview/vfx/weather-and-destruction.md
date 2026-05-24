# Weather and Destruction

Environmental effects, large-scale destruction, and dynamic simulation.

## What it covers

- Rain and snow: particle systems with wind and accumulation.
- Lightning: dynamic light flashes synchronized to thunder audio.
- Wind zones: parameterized wind affecting particles, cloth, vegetation.
- Destruction systems: breaking objects into debris.
- Destruction prefabs: blueprints defining destruction outcomes.
- Rubble: persistent destruction debris with physics.
- Dust clouds: particle effects on impact and destruction.
- Sound effects: destruction audio (crash, bang, rumble).
- Decals on destruction: scorch marks, cracks, impact sites.
- Environmental hazards: acid pools, lava, steam vents.

## Concepts

### Weather Effects

Rain renders as falling particles (billboards) with wind drift. Accumulated rain pools on surfaces
(pre-baked in level design). Snow behaves similarly but accumulates thicker. Lightning strikes
dynamically light scenes and trigger thunder audio. Wind zones parameterize wind in regions;
particle systems, cloth, and foliage sample wind and apply forces. Wind varies spatially and
temporally (gusts).

### Destruction Prefabs and Outcomes

Destruction occurs when impact force exceeds a threshold or player destroys an object explicitly.
Destruction prefabs define outcomes: a crate breaks into 5 splinter pieces; a brick wall shatters
into rubble chunks. Prefabs spawn at the destruction point; each piece gets initial velocity from
impact direction. Decals mark destruction locations (burn marks, impact scars).

### Persistent Destruction State

Destruction persists in the world: rubble piles accumulate. Physics rules rubble stability: stacked
pieces tumble if base pieces are removed. Rubble eventually despawns (cleaned up) after a duration.
This creates a sense of destruction history: areas show battle scars.

### Environmental Hazards

Toxic pools and lava zones deal damage on contact. Steam vents eject particles upward. Fire zones
spread to flammable objects. These hazards trigger particle effects and audio. Passing through a
hazard applies status effects (burning, poisoned).

## How it fits

- See [particles-and-effects.md](./particles-and-effects.md) for visual effects systems.
- See [decals-and-screen.md](./decals-and-screen.md) for impact decals and feedback.
- See [../physics/advanced.md](../physics/advanced.md) for destruction and fracture physics.
