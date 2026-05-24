# Particles and Effects

Particle systems, emitters, and visual effects generation.

## What it covers

- Emitters: spawn rules for particles per frame.
- Particle properties: age, lifetime, velocity, color, size.
- Emitter shapes: point, sphere, box, mesh surface, volume.
- Spawn rates and curves: controlling particle birth per frame.
- Particle lifetime: age and death tracking.
- Physics application: gravity, drag, wind forces on particles.
- Collision: particles bouncing or dying on contact.
- Size and color over lifetime: curves modulating particle appearance.
- Sorting: depth-sorted particle rendering.
- Billboarding: particles facing camera.

## Concepts

### Emitters and Spawning

Emitters generate particles according to spawn rules: N particles per frame, or pulse emissions. Emitter
shapes define where particles spawn: point emitter creates particles at one location; sphere emitter
scatters particles on sphere surface. Spawn rates vary over emitter lifetime: increasing at start,
fading at end creates smoke clouds forming then dissolving.

### Particle Lifetime and Evolution

Each particle has an age (current time) and lifetime (duration until death). Properties animate over
age: size grows from 0 to max at 50% lifetime, then shrinks to 0 at death. Color shifts: birth color
transitions to fade color. Velocity decays: particles start fast, slowing due to air resistance (drag).

### Physics and Interaction

Particles apply physics: gravity pulls downward, wind forces apply lateral acceleration, collision
bounces particles off geometry or kills them. Collision detection uses simple shapes (spheres) for
speed. Heavy particles (sparks) fall faster than light particles (smoke).

### Rendering and Sorting

Particles render as camera-facing billboards (quads). Depth sorting ensures far particles render
behind near particles. Particles blend additively (fire, light) or normally (smoke, dust). Texture
atlases pack multiple particle types into one texture, reducing state changes.

## How it fits

- See [decals-and-screen.md](./decals-and-screen.md) for screen-space effects and decals.
- See [weather-and-destruction.md](./weather-and-destruction.md) for destruction and weather
  effects.
- See [../physics/advanced.md](../physics/advanced.md) for destruction prefabs triggering
  particles.
