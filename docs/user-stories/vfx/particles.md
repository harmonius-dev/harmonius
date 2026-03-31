# User Stories -- 11.1 Particle System

## Stories

| ID          | Persona                  |
|-------------|--------------------------|
| US-11.1.1.1 | effects artist (P-12)    |
| US-11.1.1.2 | effects artist (P-12)    |
| US-11.1.1.3 | engine developer (P-26)  |
| US-11.1.2.1 | effects artist (P-12)    |
| US-11.1.2.2 | effects artist (P-12)    |
| US-11.1.2.3 | engine developer (P-26)  |
| US-11.1.3.1 | effects artist (P-12)    |
| US-11.1.3.2 | effects artist (P-12)    |
| US-11.1.3.3 | technical artist (P-13)  |
| US-11.1.4.1 | effects artist (P-12)    |
| US-11.1.4.2 | game designer (P-5)      |
| US-11.1.4.3 | engine developer (P-26)  |
| US-11.1.5.1 | effects artist (P-12)    |
| US-11.1.5.2 | effects artist (P-12)    |
| US-11.1.5.3 | engine developer (P-26)  |
| US-11.1.6.1 | effects artist (P-12)    |
| US-11.1.6.2 | technical artist (P-13)  |
| US-11.1.6.3 | engine developer (P-26)  |
| US-11.1.7.1 | effects artist (P-12)    |
| US-11.1.7.2 | technical artist (P-13)  |
| US-11.1.7.3 | engine developer (P-26)  |

1. **US-11.1.1.1** — **As a** effects artist (P-12), **I want** GPU-driven particle simulation using
   compute shaders with spawn shapes (point, sphere, box, cone, mesh, skinned mesh), **so that** I
   can emit millions of particles per frame without CPU readback.

2. **US-11.1.1.2** — **As a** effects artist (P-12), **I want** skinned mesh emitters that sample
   vertex buffers each frame, **so that** particles spawn on animated characters for fire auras,
   blood drips, and magic trails.

3. **US-11.1.1.3** — **As a** engine developer (P-26), **I want** persistent GPU buffers with
   free-list allocation and indirect dispatch, **so that** variable particle counts require no
   CPU-side management.

4. **US-11.1.2.1** — **As a** effects artist (P-12), **I want** composable simulation modules
   (gravity, curl noise, drag, vortex, color/size over life, collision) compiled into a single
   compute dispatch, **so that** I can build complex behaviors by combining simple modules.

5. **US-11.1.2.2** — **As a** effects artist (P-12), **I want** depth-buffer and SDF collision modes
   with configurable restitution and friction, **so that** sparks bounce off floors and debris
   settles on surfaces.

6. **US-11.1.2.3** — **As a** engine developer (P-26), **I want** SDF collision and curl noise
   disabled on mobile with depth-buffer collision only, **so that** simulation fits within mobile
   compute budgets.

7. **US-11.1.3.1** — **As a** effects artist (P-12), **I want** sprite billboards with atlas
   flipbook animation and blend modes (additive, alpha, premultiplied) with soft-particle depth
   fade, **so that** fire, smoke, and dust look polished.

8. **US-11.1.3.2** — **As a** effects artist (P-12), **I want** ribbon rendering generating
   spline-based geometry connecting sequential particles, **so that** sword swings, spell streaks,
   and projectile trails have smooth visual trails.

9. **US-11.1.3.3** — **As a** technical artist (P-13), **I want** mesh particles using GPU
   instancing with full material support and distance-based LOD, **so that** debris chunks and
   crystal shards render as lit 3D objects.

10. **US-11.1.4.1** — **As a** effects artist (P-12), **I want** hierarchical LOD tiers (full sim,
    reduced spawn, billboard impostor, culled) with hysteresis, **so that** VFX quality degrades
    gracefully without popping.

11. **US-11.1.4.2** — **As a** game designer (P-5), **I want** a global budget manager prioritizing
    player-local and gameplay-critical effects, **so that** frame rate stays stable during large
    encounters.

12. **US-11.1.4.3** — **As a** engine developer (P-26), **I want** per-platform particle budgets
    (mobile 10K, Switch 50K, console 200K, desktop 500K+) enforced at runtime, **so that** GPU
    compute and memory limits are respected.

13. **US-11.1.5.1** — **As a** effects artist (P-12), **I want** event-driven child emitters
    triggered by particle birth, death, and collision, **so that** fireworks spawn color bursts and
    impacts spawn secondary dust.

14. **US-11.1.5.2** — **As a** effects artist (P-12), **I want** sub-emitters to inherit parent
    particle transform and velocity, **so that** cascading effects flow naturally from their source.

15. **US-11.1.5.3** — **As a** engine developer (P-26), **I want** mobile sub-emitter depth limited
    to 1 with capped child counts, **so that** cascade chains do not cause budget spikes.

16. **US-11.1.6.1** — **As a** effects artist (P-12), **I want** particles to spawn dynamic point
    lights into the clustered light buffer, **so that** fire, sparks, and magic illuminate
    surrounding geometry.

17. **US-11.1.6.2** — **As a** technical artist (P-13), **I want** a stochastic sampling strategy
    capping particle lights per tile, **so that** lighting cost stays bounded during spell-heavy
    combat.

18. **US-11.1.6.3** — **As a** engine developer (P-26), **I want** mobile particle lights capped at
    4 per emitter (vs. 16 on desktop) and disabled on low-end GPUs, **so that** lighting budgets are
    respected per tier.

19. **US-11.1.7.1** — **As a** effects artist (P-12), **I want** grid-based Eulerian fluid
    simulation on GPU with ray-marched volumetric rendering, **so that** fire, smoke, and liquid
    effects look physically convincing.

20. **US-11.1.7.2** — **As a** technical artist (P-13), **I want** fluid grid resolution scaled with
    LOD distance, **so that** large open-world deployments keep GPU cost viable.

21. **US-11.1.7.3** — **As a** engine developer (P-26), **I want** mobile to fall back to sprite
    particles, Switch to use 32^3 grids, and desktop to use 128^3, **so that** fluid simulation cost
    matches platform capability.
