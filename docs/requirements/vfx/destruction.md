# R-11.5 — Destruction VFX Requirements

## R-11.5.1 Debris Spawning

The engine **SHALL** emit debris mesh fragments from destruction events using the destroyed
object's debris table, inheriting source material and physics-driven trajectories, while
enforcing a global debris budget that caps concurrent fragments.

- **Derived from:** [F-11.5.1](../../features/vfx/destruction.md)
- **Rationale:** Debris provides visual feedback for destruction; a global budget prevents
  frame time spikes during large-scale battles with many simultaneous destruction events.
- **Verification:** Integration test — trigger destruction on an object with a 10-entry debris
  table; assert fragments spawn with matching material and velocity within the configured cone;
  trigger enough destructions to exceed the global budget; assert total concurrent fragments
  never exceeds the cap.

## R-11.5.2 Dust Clouds and Smoke Plumes

The engine **SHALL** spawn volumetric dust and smoke particles at destruction sites with color
derived from the destroyed material type, and apply wind field displacement to smoke plumes.

- **Derived from:** [F-11.5.2](../../features/vfx/destruction.md)
- **Rationale:** Material-appropriate dust and wind-driven smoke convey the physical reality of
  destruction and maintain visual consistency across different surface types.
- **Verification:** Integration test — destroy a stone object and a wood object; assert dust
  particle color matches the configured material palette for each; enable a wind field and
  assert smoke plume drift direction aligns with the wind vector within 15 degrees.

## R-11.5.3 Sparks and Embers

The engine **SHALL** emit spark particles with gravity, bounce collision, and color fade from
white-hot to dark, and emit ember particles with upward drift and flickering emissive
intensity, both contributing to the clustered light buffer.

- **Derived from:** [F-11.5.3](../../features/vfx/destruction.md)
- **Rationale:** Sparks and embers sell metallic and fire destruction through physically
  motivated motion and emissive lighting that integrates with the scene.
- **Verification:** Integration test — trigger a metallic destruction event; assert spark
  particles apply gravity and bounce on collision; verify color interpolates from white to
  orange to dark over lifetime; trigger a fire destruction event; assert ember particles drift
  upward; verify both particle types register light contributions in the clustered light buffer.

## R-11.5.4 Structural Cracking Overlays

The engine **SHALL** project animated crack decals that spread outward from impact points at a
rate and branching density proportional to accumulated damage, using material-appropriate crack
atlas patterns.

- **Derived from:** [F-11.5.4](../../features/vfx/destruction.md)
- **Rationale:** Progressive crack visuals provide early warning of structural failure, giving
  players time to react before full collapse during siege events.
- **Verification:** Integration test — apply incremental damage to a destructible surface;
  assert crack decals appear at the impact point and grow outward; verify growth speed
  increases with accumulated damage; assert the selected crack atlas matches the surface
  material type.

## R-11.5.5 Persistent Scorch Marks

The engine **SHALL** place scorch decals at explosion and fire damage sites that modify albedo,
roughness, and normal channels of the underlying surface, persist above transient combat decals
in priority order, and fade over a configurable world-time duration.

- **Derived from:** [F-11.5.5](../../features/vfx/destruction.md)
- **Rationale:** Persistent scorch marks communicate battlefield history and maintain
  readability of prior engagements across extended gameplay sessions.
- **Verification:** Integration test — trigger an explosion on a surface; assert the scorch
  decal modifies albedo (darkened), roughness (increased), and normal (flattened) channels;
  spawn a transient combat decal at the same location; assert the scorch mark renders above it;
  advance world time past the configured fade duration; assert the scorch mark is removed.

## R-11.5.6 Explosion Shockwaves

The engine **SHALL** render expanding spherical shockwaves as screen-space radial refraction
rings from the detonation point, spawn a dust particle ring at ground contact, and apply
distance-scaled camera shake, with additive distortion from overlapping shockwaves capped to a
maximum displacement.

- **Derived from:** [F-11.5.6](../../features/vfx/destruction.md)
- **Rationale:** Shockwave distortion, dust rings, and camera shake sell the force of
  explosions; capping composite distortion prevents disorienting screen warping.
- **Verification:** Integration test — detonate an explosion; assert a radial refraction ring
  expands outward in screen space; assert dust particles spawn at the ground plane intersection;
  assert camera shake magnitude decreases with distance; trigger two overlapping explosions;
  assert combined distortion does not exceed the configured maximum displacement.

## R-11.5.7 Fire Propagation Visuals

The engine **SHALL** render surface-spreading fire driven by a propagation map that spawns
flame particles, emits light, and overlays emissive burn textures on active fire regions, with
spread rate influenced by wind and material flammability.

- **Derived from:** [F-11.5.7](../../features/vfx/destruction.md)
- **Rationale:** Fire propagation visuals driven by material and wind create dynamic,
  believable fire spread across wooden structures and vegetation during siege warfare.
- **Verification:** Integration test — ignite a surface with two adjacent material zones (wood
  and stone); assert fire spreads across the wood zone at a rate greater than zero; assert fire
  does not spread onto the stone zone; enable a wind field and assert spread rate increases in
  the downwind direction; verify flame particles spawn and light is emitted in active burn
  regions.
