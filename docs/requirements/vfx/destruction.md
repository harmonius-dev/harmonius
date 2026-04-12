# R-11.5 -- Destruction VFX Requirements

## Debris and Fragments

1. **R-11.5.1** — The engine **SHALL** emit debris mesh fragments from destruction events using the
   object's debris table, inheriting source material and physics-driven trajectories, with a global
   budget capping concurrent fragments.
   - **Rationale:** Debris provides visual feedback; a global budget prevents frame time spikes
     during large battles.
   - **Verification:** Trigger destruction on an object with a 10-entry debris table and verify
     matching material and velocity. Exceed the budget and verify the cap holds.

2. **R-11.5.2** — The engine **SHALL** spawn volumetric dust and smoke at destruction sites with
   color derived from the destroyed material, applying wind field displacement to smoke plumes.
   - **Rationale:** Material-appropriate dust and wind-driven smoke convey the physical reality of
     destruction.
   - **Verification:** Destroy stone and wood objects and verify dust color matches material. Enable
     wind and verify plume drift aligns within 15 degrees.

3. **R-11.5.3** — The engine **SHALL** emit spark particles with gravity, bounce, and color fade
   from white-hot to dark, and ember particles with upward drift and flickering emissive intensity,
   both contributing to the clustered light buffer.
   - **Rationale:** Sparks and embers sell metallic and fire destruction through physically
     motivated motion and emissive lighting.
   - **Verification:** Trigger metallic destruction and verify spark gravity, bounce, and color
     fade. Trigger fire destruction and verify ember drift. Confirm both register in the clustered
     light buffer.

## Surface Damage

4. **R-11.5.4** — The engine **SHALL** project animated crack decals spreading from impact points at
   a rate and branching density proportional to accumulated damage, using material-appropriate crack
   atlas patterns.
   - **Rationale:** Progressive cracks warn of structural failure before full collapse during siege
     events.
   - **Verification:** Apply incremental damage and verify crack growth from impact point. Verify
     speed increases with damage. Confirm atlas matches surface material.

5. **R-11.5.5** — The engine **SHALL** place scorch decals at explosion and fire sites that modify
   albedo, roughness, and normal channels, persist above transient decals in priority, and fade over
   a configurable world-time duration.
   - **Rationale:** Persistent scorch marks communicate battlefield history and maintain
     readability.
   - **Verification:** Trigger an explosion and verify scorch modifies albedo, roughness, and
     normal. Spawn a transient decal and verify scorch renders above it. Advance time and verify
     fade.

## Explosive and Fire Effects

6. **R-11.5.6** — The engine **SHALL** render expanding shockwaves as screen-space radial refraction
   rings, spawn ground-contact dust, and apply distance-scaled camera shake, with additive
   distortion from overlapping shockwaves capped.
   - **Rationale:** Shockwave distortion, dust, and shake sell explosive force; capping prevents
     disorienting warping.
   - **Verification:** Detonate and verify radial refraction, dust ring, and distance-scaled shake.
     Trigger two overlapping and verify cap on composite distortion.

7. **R-11.5.7** — The engine **SHALL** render surface-spreading fire driven by a propagation map
   that spawns flame particles, emits light, and overlays emissive burn textures, with spread rate
   influenced by wind and material flammability.
   - **Rationale:** Fire propagation driven by material and wind creates dynamic, believable fire
     spread.
   - **Verification:** Ignite adjacent wood and stone zones. Verify fire spreads on wood, not stone.
     Enable wind and verify increased downwind spread. Confirm flame particles and light emission.

## Voxel and SDF Integration

8. **R-11.5.8** — The engine **SHALL** emit particles from voxel edit operations with color and
   texture matching the voxel material ID, scaling emission count with edit volume and distributing
   across the SDF edit boundary.
   - **Rationale:** Material-matched particles make terrain excavation visually appropriate per
     material type.
   - **Verification:** Edit dirt, stone, and ice voxels and verify particle color matches the
     material lookup table. Increase edit volume and verify proportional emission.

9. **R-11.5.9** — The engine **SHALL** collide particles with SDF voxel volumes using sphere-traced
   distance queries, pushing particles to the surface along the SDF gradient, with configurable
   restitution and friction per material, working alongside depth-buffer collision (F-11.1.2).
   - **Rationale:** SDF collision handles deformed terrain and runtime voxel edits without mesh
     regeneration.
   - **Verification:** Spawn particles above a carved cave ceiling and verify bounce. Carve terrain
     and verify collision responds on next frame. Confirm SDF and depth-buffer coexist without
     double-collision.

10. **R-11.5.10** — The engine **SHALL** select material-specific VFX presets from the parent
    object's surface type tag, defining fragment meshes, particle colors, dust opacity, and
    secondary effects per material as data assets.
    - **Rationale:** Material-matched debris and VFX give each material a distinct destruction
      character.
    - **Verification:** Destroy wood, stone, metal, glass, and ice objects and verify each produces
      the material-specific preset. Confirm presets are loaded as data assets.

11. **R-11.5.11** — The engine **SHALL** emit paired audio-VFX commands on destruction events
    through a unified event bus, carrying material ID, fragment count, total mass, and impact energy
    for independent interpretation by audio and VFX systems.
    - **Rationale:** Synchronized audio-VFX triggers ensure destruction feels cohesive without tight
      coupling.
    - **Verification:** Trigger destruction and verify audio and VFX fire within the same frame.
      Increase fragment count and verify audio complexity scales proportionally.

## Fire Damage

12. **R-11.5.7a** — The engine **SHALL** apply fire DPS to entities overlapping the fire propagation
    map, with damage scaled by fuel type and coverage.
    - **Rationale:** Fire must deal damage proportional to exposure for gameplay consequences beyond
      visual effects.
    - **Verification:** Place an entity in a fire zone with wood fuel type. Assert DPS matches the
      configured rate. Move the entity to partial coverage. Assert damage scales proportionally.
      Place on stone. Assert reduced or zero damage.
