# User Stories -- 11.5 Destruction VFX

## Stories

| ID           | Persona                 |
|--------------|-------------------------|
| US-11.5.1.1  | effects artist (P-12)   |
| US-11.5.1.2  | effects artist (P-12)   |
| US-11.5.1.3  | engine developer (P-26) |
| US-11.5.2.1  | effects artist (P-12)   |
| US-11.5.2.2  | effects artist (P-12)   |
| US-11.5.2.3  | engine developer (P-26) |
| US-11.5.3.1  | effects artist (P-12)   |
| US-11.5.3.2  | effects artist (P-12)   |
| US-11.5.3.3  | engine developer (P-26) |
| US-11.5.4.1  | effects artist (P-12)   |
| US-11.5.4.2  | effects artist (P-12)   |
| US-11.5.4.3  | engine developer (P-26) |
| US-11.5.5.1  | effects artist (P-12)   |
| US-11.5.5.2  | engine developer (P-26) |
| US-11.5.6.1  | effects artist (P-12)   |
| US-11.5.6.2  | engine developer (P-26) |
| US-11.5.6.3  | game designer (P-5)     |
| US-11.5.7.1  | effects artist (P-12)   |
| US-11.5.7.2  | effects artist (P-12)   |
| US-11.5.7.3  | engine developer (P-26) |
| US-11.5.8.1  | effects artist (P-12)   |
| US-11.5.8.2  | effects artist (P-12)   |
| US-11.5.9.1  | effects artist (P-12)   |
| US-11.5.9.2  | technical artist (P-13) |
| US-11.5.10.1 | effects artist (P-12)   |
| US-11.5.10.2 | effects artist (P-12)   |
| US-11.5.11.1 | game designer (P-5)     |
| US-11.5.11.2 | game designer (P-5)     |

1. **US-11.5.1.1** — **As a** effects artist (P-12), **I want** debris mesh fragments emitted from
   destruction events with material inherited from the source object, **so that** destroying a stone
   wall produces stone fragments.

2. **US-11.5.1.2** — **As a** effects artist (P-12), **I want** per-object debris tables controlling
   fragment type, count, velocity cone, and material, **so that** each destructible produces
   appropriate debris.

3. **US-11.5.1.3** — **As a** engine developer (P-26), **I want** a global debris budget (32 on
   mobile, 256 on desktop) capping concurrent fragments, **so that** frame time stays bounded during
   large battles.

4. **US-11.5.2.1** — **As a** effects artist (P-12), **I want** dust clouds colored by destroyed
   material (grey for stone, brown for wood) that expand on impact and transition to drifting smoke,
   **so that** destruction kick-up looks physically convincing.

5. **US-11.5.2.2** — **As a** effects artist (P-12), **I want** smoke plumes to drift with the wind
   field, **so that** destruction smoke is coherent with the weather system.

6. **US-11.5.2.3** — **As a** engine developer (P-26), **I want** mobile to skip wind interaction
   and use shorter plume persistence, **so that** smoke fits within mobile particle budgets.

7. **US-11.5.3.1** — **As a** effects artist (P-12), **I want** spark bursts from metallic impacts
   with gravity, bounce, and rapid color fade from white-hot to dark, **so that** hitting metal
   produces convincing spark showers.

8. **US-11.5.3.2** — **As a** effects artist (P-12), **I want** slow drifting ember particles rising
   from burning debris with flickering emissive intensity and light emission, **so that**
   fire-damaged areas glow with residual heat.

9. **US-11.5.3.3** — **As a** engine developer (P-26), **I want** particle lights for sparks and
   embers disabled on mobile with spark count at 25% of desktop, **so that** destruction VFX fit
   mobile budgets.

10. **US-11.5.4.1** — **As a** effects artist (P-12), **I want** animated crack decals spreading
    from impact points with growth speed scaling with accumulated damage, **so that** players get
    visual warning before collapse.

11. **US-11.5.4.2** — **As a** effects artist (P-12), **I want** pre-authored directional crack
    atlases selected by surface material (stone, concrete, wood), **so that** crack patterns look
    appropriate for each material.

12. **US-11.5.4.3** — **As a** engine developer (P-26), **I want** mobile to use static crack decals
    with no animated growth, **so that** crack rendering is cheap on mobile.

13. **US-11.5.5.1** — **As a** effects artist (P-12), **I want** long-lived scorch decals at
    explosion and fire sites that darken albedo, increase roughness, and flatten normals,
    **so that** the battlefield shows lasting evidence of combat.

14. **US-11.5.5.2** — **As a** engine developer (P-26), **I want** mobile scorch marks to modify
    albedo only with shorter persistence, **so that** scorch decals reduce G-buffer writes on
    mobile.

15. **US-11.5.6.1** — **As a** effects artist (P-12), **I want** expanding spherical shockwaves with
    screen-space radial refraction, ground-contact dust rings, and distance-scaled camera shake,
    **so that** explosions have dramatic visual impact.

16. **US-11.5.6.2** — **As a** engine developer (P-26), **I want** mobile to skip screen-space
    distortion and cap concurrent shockwaves at 1, **so that** explosion effects stay within mobile
    GPU budgets.

17. **US-11.5.6.3** — **As a** game designer (P-5), **I want** overlapping shockwave distortion
    capped to a maximum displacement, **so that** screen warping does not become disorienting.

18. **US-11.5.7.1** — **As a** effects artist (P-12), **I want** fire propagating across flammable
    surfaces with flame particles, emissive overlays, and light emission, **so that** setting a
    wooden tower on fire produces spreading flames.

19. **US-11.5.7.2** — **As a** effects artist (P-12), **I want** per-texel propagation maps tracking
    burn state with spread rate influenced by wind and material type, **so that** fire follows
    flammable paths realistically.

20. **US-11.5.7.3** — **As a** engine developer (P-26), **I want** mobile to use quarter texel
    density maps with no wind spread and max 2 fire light sources, **so that** fire propagation cost
    scales per platform.

21. **US-11.5.8.1** — **As a** effects artist (P-12), **I want** voxel edit operations to emit
    particles matching the voxel material (brown dust for dirt, grey chips for stone, translucent
    shards for ice), **so that** terrain excavation produces visually appropriate debris.

22. **US-11.5.8.2** — **As a** effects artist (P-12), **I want** larger excavations to produce
    proportionally more particles distributed across the SDF edit boundary, **so that** big blasts
    feel more impactful than small digs.

23. **US-11.5.9.1** — **As a** effects artist (P-12), **I want** particles to collide with SDF voxel
    volumes using sphere-traced queries, **so that** particles bounce off deformed terrain and
    carved caves.

24. **US-11.5.9.2** — **As a** technical artist (P-13), **I want** SDF and depth-buffer collision to
    coexist without double-collision artifacts, **so that** particles interact correctly with all
    scene geometry.

25. **US-11.5.10.1** — **As a** effects artist (P-12), **I want** destruction debris and VFX to
    inherit material properties from the parent object's surface type, **so that** wood produces
    splinters and sawdust while metal produces sparks and shards.

26. **US-11.5.10.2** — **As a** effects artist (P-12), **I want** material-specific VFX presets
    authored as data assets with fragment meshes, particle colors, and secondary effects,
    **so that** each material has a distinct destruction character.

27. **US-11.5.11.1** — **As a** game designer (P-5), **I want** destruction audio and VFX to trigger
    within the same frame via a unified event bus, **so that** visual and auditory impact are
    perfectly synchronized.

28. **US-11.5.11.2** — **As a** game designer (P-5), **I want** destruction events with more
    fragments to produce more complex layered audio, **so that** large-scale destruction sounds more
    impactful than small breakages.
