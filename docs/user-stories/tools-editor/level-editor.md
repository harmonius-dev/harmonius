# User Stories -- 15.2 Level Editor

## Stories

| ID          | Persona                 |
|-------------|-------------------------|
| US-15.2.1.1 | level designer (P-6)    |
| US-15.2.1.2 | game designer (P-5)     |
| US-15.2.2.1 | level designer (P-6)    |
| US-15.2.2.2 | game designer (P-5)     |
| US-15.2.3.1 | level designer (P-6)    |
| US-15.2.3.2 | game designer (P-5)     |
| US-15.2.4.1 | level designer (P-6)    |
| US-15.2.4.2 | game designer (P-5)     |
| US-15.2.5.1 | level designer (P-6)    |
| US-15.2.5.2 | technical artist (P-13) |
| US-15.2.6.1 | level designer (P-6)    |
| US-15.2.6.2 | technical artist (P-13) |
| US-15.2.7.1 | level designer (P-6)    |
| US-15.2.7.2 | game designer (P-5)     |
| US-15.2.8.1 | level designer (P-6)    |

1. **US-15.2.1.1** — **As a** level designer (P-6), **I want** to drag entities from the asset
   browser into the world with grid and surface snapping, **so that** I can populate levels quickly.

2. **US-15.2.1.2** — **As a** game designer (P-5), **I want** vertex snapping for precise geometry
   alignment, **so that** modular pieces connect seamlessly.

3. **US-15.2.2.1** — **As a** level designer (P-6), **I want** reusable Entity Template hierarchies
   with nesting, **so that** a village template can compose house templates containing furniture
   templates.

4. **US-15.2.2.2** — **As a** game designer (P-5), **I want** changes to a parent Entity Template to
   propagate to all instances, **so that** I fix a layout error in one place.

5. **US-15.2.3.1** — **As a** level designer (P-6), **I want** to override individual properties on
   an Entity Template instance while preserving its link, **so that** each instance can have unique
   variation.

6. **US-15.2.3.2** — **As a** game designer (P-5), **I want** overrides visualized as bold labels in
   the inspector, **so that** I can distinguish local changes from inherited values.

7. **US-15.2.4.1** — **As a** level designer (P-6), **I want** additive and subtractive CSG
   primitives for rapid blockout, **so that** I can prototype level layouts before committing to
   final art.

8. **US-15.2.4.2** — **As a** game designer (P-5), **I want** to convert finalized brush geometry to
   static meshes, **so that** blockout transitions to production smoothly.

9. **US-15.2.5.1** — **As a** level designer (P-6), **I want** Bezier and Catmull-Rom spline editing
   in the viewport, **so that** I can author roads, rivers, and rail paths.

10. **US-15.2.5.2** — **As a** technical artist (P-13), **I want** meshes and entities distributed
    along splines with spacing and randomization rules, **so that** fences and power lines follow
    terrain contours automatically.

11. **US-15.2.6.1** — **As a** level designer (P-6), **I want** to paint terrain material layers
    with configurable brush shape and falloff, **so that** I blend grass, rock, and dirt naturally.

12. **US-15.2.6.2** — **As a** technical artist (P-13), **I want** height-based and slope-based
    auto-painting rules, **so that** cliffs receive rock material automatically.

13. **US-15.2.7.1** — **As a** level designer (P-6), **I want** to paint foliage instances with
    density brushes and per-type placement rules, **so that** vegetation distribution looks organic.

14. **US-15.2.7.2** — **As a** game designer (P-5), **I want** foliage painters to respect slope
    limits, altitude ranges, and exclusion zones, **so that** trees do not appear on cliffs or in
    building footprints.

15. **US-15.2.8.1** — **As a** level designer (P-6), **I want** to select a group of placed entities
    in the viewport and save them as a reusable entity template, **so that** I can stamp the same
    pattern across my level without manually re-creating each instance.
