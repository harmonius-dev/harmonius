# Perception

Sensing enemies, objects, and environmental features for decision-making.

## What it covers

- Line-of-sight (LOS) checking: detecting visible entities.
- Sensory ranges: sight, hearing, smell detection radiuses.
- Occlusion: obstacles blocking sight and sound.
- Memory: maintaining records of recently sensed entities.
- Threat assessment: evaluating enemy danger levels.
- Interest points: notable locations like cover, ammo pickups, goals.
- Spatial queries: "are there enemies in this region".
- Tracking: maintaining awareness of moving targets.
- Forgetting: decay of memory over time without updates.
- Stimulus spread: sound alerts propagating through groups.

## Concepts

### Sensory Input

Characters sense their environment via sight, hearing, and smell. Sight traces rays from the
character's eyes to potential targets, checking line-of-sight. Hearing detects sound events (gunfire,
footsteps) within range. Smell works like hearing but with different range and event types. Each
sense has a range (e.g., sight 50 meters, hearing 100 meters) and is blocked by obstacles.

### Occlusion and LOS

Line-of-sight checks whether a target is visible. A raycast from the character's head to the
target's position detects obstacles. Partial occlusion (target partially behind a wall) can reduce
visibility. Some targets hide in shadows or camouflage; the engine adjusts visibility based on
lighting and surface properties.

### Memory and Awareness

When a character senses something, it records the memory: entity, position, time, confidence. Memory
decays if not reinforced: if a target disappears from sight and is not heard or seen again, the
character gradually forgets it. Short-term memory persists briefly; long-term memory stores past
encounters influencing behavior (that location was hostile before).

### Threat Assessment

Threats (enemies) score by danger level: health, weapons, proximity, recent aggression. Assess
threat AI runs frequently, updating danger scores. Immediate threats (shooting at the character)
score high; distant unthreatening entities score low. Threat scores influence decision-making: high
threat triggers defensive behavior or retreat.

### Interest Points and Spatial Queries

Interest points mark notable locations: ammo, cover, objectives. Spatial queries find points
within a region (cover spots near current position). Characters query for cover when under fire,
navigate to ammo points when low, approach objectives. Spatial queries use acceleration structures
for speed.

### Stimulus Propagation

Sound events propagate through groups: one character hears a gunshot, alerts nearby allies. Alert
spreads through the group like waves. This creates coordinated responses without explicit
communication: a guard shouts alarm; nearby guards awaken and investigate.

## How it fits

- See [decision-making.md](./decision-making.md) for using perception to drive decisions.
- See [navigation.md](./navigation.md) for moving toward perceived threats or objects.
- See [../simulation/spatial-awareness.md](../simulation/spatial-awareness.md) for spatial
  queries and occlusion.
