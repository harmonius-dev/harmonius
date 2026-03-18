# User Stories -- 11.1 Particle System

## GPU Particle Simulation

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-11.1.1.1 | Engine developer (P-26) | F-11.1.1 | R-11.1.1     |
| US-11.1.1.2 | Effects artist (P-12)   | F-11.1.1 | R-11.1.1     |
| US-11.1.1.3 | Engine tester (P-27)    | F-11.1.1 | R-11.1.1     |
| US-11.1.2.1 | Effects artist (P-12)   | F-11.1.2 | R-11.1.2     |
| US-11.1.2.2 | Player (P-23)           | F-11.1.2 | R-11.1.2     |
| US-11.1.2.3 | Engine tester (P-27)    | F-11.1.2 | R-11.1.2     |

1. **US-11.1.1.1** — I want GPU-driven particle simulation using compute shaders with persistent
   buffers, free-list allocation, and indirect dispatch, so that millions of particles update per
   frame without CPU readback.
   - **Acceptance:** All simulation runs on compute shaders with zero CPU readback; throughput
     exceeds one million particles at 60fps
2. **US-11.1.1.2** — I want skinned mesh emitters that sample vertex buffers each frame, so that
   particles spawn on moving characters (fire aura, blood drip, magic trail) without CPU-side mesh
   readback.
   - **Acceptance:** Particles spawn on animated character surfaces; no CPU-side mesh readback
     required
3. **US-11.1.1.3** — I want to disable async compute and verify that particle simulation falls back
   to the graphics queue without visual errors, so that the system works on GPUs without dedicated
   async compute.
   - **Acceptance:** Fallback to graphics queue produces identical visual results; no errors on GPUs
     without async compute
4. **US-11.1.2.1** — I want composable simulation modules (gravity, curl noise, drag, vortex, color
   over life, collision) compiled into a single compute dispatch per emitter, so that I can build
   complex particle behaviors by combining simple modules.
   - **Acceptance:** All listed modules produce correct per-particle output in a single dispatch;
     modules composable in any combination
5. **US-11.1.2.2** — I want particles to collide with the ground and walls (via depth buffer or SDF)
   with configurable bounce and friction, so that sparks bounce off floors and debris settles on
   surfaces rather than falling through the world.
   - **Acceptance:** Depth-buffer and SDF collision with configurable restitution and friction;
     particles do not pass through scene geometry
6. **US-11.1.2.3** — I want to verify that mobile uses depth-buffer collision only with SDF
   collision and curl noise disabled, so that particle simulation fits within mobile compute
   budgets.
   - **Acceptance:** SDF collision and curl noise disabled on mobile; depth-buffer collision active
     on mobile

## Rendering

| ID          | Persona               | Features | Requirements |
|-------------|-----------------------|----------|--------------|
| US-11.1.3.1 | Effects artist (P-12) | F-11.1.3 | R-11.1.3     |
| US-11.1.3.2 | Effects artist (P-12) | F-11.1.3 | R-11.1.3     |
| US-11.1.3.3 | Engine tester (P-27)  | F-11.1.3 | R-11.1.3     |

1. **US-11.1.3.1** — I want ribbon particle rendering that generates spline-based geometry
   connecting sequential particles, so that sword swings, spell streaks, and projectile trails have
   smooth, continuous visual trails.
   - **Acceptance:** Ribbon geometry connects sequential particles; smooth, continuous trails
     without gaps
2. **US-11.1.3.2** — I want mesh particle rendering using GPU instancing with per-particle
   transforms and full material support, so that debris chunks, rock fragments, and crystal shards
   render as lit 3D objects.
   - **Acceptance:** Mesh particles rendered with full material support; per-particle transforms and
     LOD selection by distance
3. **US-11.1.3.3** — I want to verify that sprite particles use soft-particle depth fade on desktop
   and that the fade is disabled on low-end mobile GPUs, so that particle edge blending scales per
   platform.
   - **Acceptance:** Soft-particle depth fade active on desktop; disabled on low-end mobile GPUs

## LOD and Budget

| ID          | Persona               | Features | Requirements |
|-------------|-----------------------|----------|--------------|
| US-11.1.4.1 | Effects artist (P-12) | F-11.1.4 | R-11.1.4     |
| US-11.1.4.2 | Player (P-23)         | F-11.1.4 | R-11.1.4     |
| US-11.1.4.3 | Engine tester (P-27)  | F-11.1.4 | R-11.1.4     |

1. **US-11.1.4.1** — I want hierarchical LOD tiers that reduce spawn rate and rendering complexity
   for distant effects, with a global budget manager prioritizing player-local effects, so that VFX
   quality degrades gracefully rather than dropping frames.
   - **Acceptance:** LOD tiers transition at configured distances without popping; global budget
     caps total alive particles by priority
2. **US-11.1.4.2** — I want the global particle budget to cap total alive particles and prioritize
   gameplay-critical effects during raid encounters, so that frame rate stays stable even when
   dozens of spell effects fire simultaneously.
   - **Acceptance:** Lower-priority emitters culled first when budget exceeded; gameplay-critical
     effects retain full fidelity
3. **US-11.1.4.3** — I want to verify global particle budgets of 10K on mobile, 50K on Switch, 200K
   on console, and 500K+ on desktop, so that particle counts stay within per-platform GPU compute
   and memory limits.
   - **Acceptance:** Per-platform budgets enforced; particle counts never exceed configured limits

## Advanced Features

| ID          | Persona               | Features | Requirements |
|-------------|-----------------------|----------|--------------|
| US-11.1.5.1 | Effects artist (P-12) | F-11.1.5 | R-11.1.5     |
| US-11.1.5.2 | Engine tester (P-27)  | F-11.1.5 | R-11.1.5     |
| US-11.1.6.1 | Effects artist (P-12) | F-11.1.6 | R-11.1.6     |
| US-11.1.6.2 | Engine tester (P-27)  | F-11.1.6 | R-11.1.6     |
| US-11.1.7.1 | Effects artist (P-12) | F-11.1.7 | R-11.1.7     |
| US-11.1.7.2 | Engine tester (P-27)  | F-11.1.7 | R-11.1.7     |

1. **US-11.1.5.1** — I want event-driven child emitter spawning triggered by particle birth, death,
   and collision events, so that fireworks spawn bursts of color sparks, impact effects spawn
   secondary dust, and spell detonations cascade without monolithic systems.
   - **Acceptance:** Child particles spawn at correct parent position and velocity for each event
     type; cascading chains function correctly
2. **US-11.1.5.2** — I want to verify that mobile limits sub-emitter depth to 1 with capped child
   particle counts, so that cascade-driven sub-emitters do not cause budget spikes on mobile.
   - **Acceptance:** Mobile sub-emitter depth limited to 1; child particle count capped on mobile
3. **US-11.1.6.1** — I want particles to spawn dynamic point lights with configurable color,
   intensity, and attenuation radius into the clustered light buffer, so that fire, sparks, and
   magic effects illuminate surrounding surfaces.
   - **Acceptance:** Particle lights appear in clustered light buffer with correct color and
     attenuation; surrounding geometry illuminated
4. **US-11.1.6.2** — I want to verify that mobile caps particle lights to 4 per emitter and desktop
   to 16, with stochastic sampling bounding per-tile lighting cost, so that particle lights do not
   blow lighting budgets during spell-heavy combat.
   - **Acceptance:** Mobile caps at 4 per emitter; desktop caps at 16; per-tile light cap enforced
5. **US-11.1.7.1** — I want grid-based Eulerian fluid simulation on GPU with ray-marched volumetric
   rendering for fire, smoke, and liquid effects, so that large-scale fire and smoke look physically
   convincing rather than sprite-based.
   - **Acceptance:** Advection, emission, absorption, and scattering produce physically plausible
     results; volumetric rendering with ray marching
6. **US-11.1.7.2** — I want to verify that mobile falls back to sprite particles, Switch uses 32^3
   grids, and desktop uses 128^3, so that fluid simulation cost scales with platform capability.
   - **Acceptance:** Mobile falls back to sprite particles; Switch uses 32^3 grid; desktop uses
     128^3 grid
