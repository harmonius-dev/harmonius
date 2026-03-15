# User Stories — 4.2 Collision Detection

## F-4.2.1 Broadphase Acceleration Structures

**US-4.2.1a** As a gameplay programmer, I want broadphase to use the shared BVH instead of
a physics-private structure so that memory is not wasted on redundant spatial indices for
rendering, networking, and physics.

## F-4.2.2 Narrowphase Contact Generation

**US-4.2.2a** As a gameplay programmer, I want contact manifolds to report accurate
penetration depths and normals so that impulse resolution produces stable, jitter-free
stacking and resting contacts.

**US-4.2.2b** As a QA engineer, I want to verify that narrowphase results match analytical
solutions for all primitive pairs so that I can trust contact accuracy across shape
combinations.

## F-4.2.3 Primitive and Convex Collision Shapes

**US-4.2.3a** As a designer, I want to choose from box, sphere, capsule, and convex hull
collider shapes so that I can approximate any object's silhouette without needing concave
mesh colliders for simple props.

## F-4.2.4 Triangle Mesh and Heightfield Shapes

**US-4.2.4a** As a designer, I want terrain and static environment meshes to support
per-triangle physics materials so that walking on grass, stone, and mud produces different
friction, sounds, and particle effects.

**US-4.2.4b** As a gameplay programmer, I want heightfield colliders to integrate with the
shared BVH so that terrain collision is efficient even for large open worlds.

## F-4.2.5 Compound Shapes

**US-4.2.5a** As a designer, I want to combine multiple primitive shapes into a single
compound collider so that complex objects like vehicles and furniture have accurate
collision without expensive concave decomposition.

## F-4.2.6 Collision Filtering and Layers

**US-4.2.6a** As a designer, I want to assign entities to collision layers (player, enemy,
projectile, trigger) so that I can control which objects interact without writing filter
code.

**US-4.2.6b** As a gameplay programmer, I want to register a custom filter callback for
team-based or owner-based collision rules so that friendly-fire and self-hit scenarios are
handled cleanly at broadphase.

## F-4.2.7 Collision Events

**US-4.2.7a** As a gameplay programmer, I want `CollisionStarted`, `CollisionPersisted`,
and `CollisionEnded` events with contact points and impulse magnitudes so that I can
trigger damage, sound, and VFX at the correct moment.

**US-4.2.7b** As a designer, I want collision events to fire reliably on the exact frame
of contact so that hit feedback (screen shake, particles, audio) feels instantaneous and
not delayed.

## F-4.2.8 Trigger Volumes

**US-4.2.8a** As a designer, I want to place trigger volumes that detect when a player
enters, stays within, or exits a region so that I can create quest zones, traps, and
loading triggers visually.

**US-4.2.8b** As a gameplay programmer, I want trigger volumes to support one-shot,
persistent, and filtered modes so that I can implement single-use pickups, continuous
area effects, and entity-type-specific triggers.

## F-4.2.9 Physics Material Assets

**US-4.2.9a** As a designer, I want to author physics materials (friction, restitution,
density, surface tag) in the visual editor and assign them per-collider so that surface
properties drive audio, VFX, and gameplay without code.

**US-4.2.9b** As a player, I want walking on ice to feel slippery and landing on rubber to
feel bouncy so that different surfaces have distinct physical character.
