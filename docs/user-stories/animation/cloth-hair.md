# User Stories -- 9.5 Cloth and Hair Simulation

## Cloth Simulation

| ID          | Persona                    |
|-------------|----------------------------|
| US-9.5.1.1  | character animator (P-11)  |
| US-9.5.1.2  | technical artist (P-13)    |
| US-9.5.1.3  | engine developer (P-26)    |

1. **US-9.5.1.1** -- **As a** character animator (P-11), **I want** GPU cloth simulation with
   distance, bending, and self-collision constraints driven by wind and skeletal animation,
   **so that** garments move realistically on animated characters.

2. **US-9.5.1.2** -- **As a** technical artist (P-13), **I want** to author panel-based collision
   proxies and LOD tiers per garment, **so that** cloth quality scales with platform tier and
   character importance.

3. **US-9.5.1.3** -- **As an** engine developer (P-26), **I want** cloth simulation disabled on
   mobile by default with a baked animation fallback, **so that** garment motion remains visible
   without simulation cost.

## Hair Systems

| ID          | Persona                    |
|-------------|----------------------------|
| US-9.5.2.1  | character animator (P-11)  |
| US-9.5.2.2  | engine developer (P-26)    |
| US-9.5.2.3  | technical artist (P-13)    |
| US-9.5.3.1  | character animator (P-11)  |
| US-9.5.3.2  | technical artist (P-13)    |
| US-9.5.4.1  | engine developer (P-26)    |
| US-9.5.4.2  | technical artist (P-13)    |

1. **US-9.5.2.1** -- **As a** character animator (P-11), **I want** strand-based hair simulation
   using guide curves with stretch, bend, and collision constraints, **so that** hero character hair
   moves with physical fidelity.

2. **US-9.5.2.2** -- **As an** engine developer (P-26), **I want** strand simulation limited to
   desktop with card-based fallback on lower tiers, **so that** each platform uses the most
   appropriate hair method.

3. **US-9.5.2.3** -- **As a** technical artist (P-13), **I want** to configure guide strand count
   and simulation parameters per character, **so that** hair fidelity matches character importance
   and platform budget.

4. **US-9.5.3.1** -- **As a** character animator (P-11), **I want** card-based hair rendering with
   alpha-tested transparency and anisotropic specular shading, **so that** NPC hair looks convincing
   at medium distances and on lower platforms.

5. **US-9.5.3.2** -- **As a** technical artist (P-13), **I want** to configure card count and spring
   physics parameters per character, **so that** hair appearance and cost are tunable per platform
   and character importance.

6. **US-9.5.4.1** -- **As an** engine developer (P-26), **I want** a hair LOD system that
   transitions between strand, cluster, card, and shell representations based on camera distance,
   **so that** hundreds of visible characters have hair within budget.

7. **US-9.5.4.2** -- **As a** technical artist (P-13), **I want** temporal blending on LOD
   transitions, **so that** hair LOD changes do not pop visibly.

## Cloth-Body and Wind

| ID          | Persona                    |
|-------------|----------------------------|
| US-9.5.5.1  | rigger (P-10)              |
| US-9.5.5.2  | engine developer (P-26)    |
| US-9.5.6.1  | character animator (P-11)  |
| US-9.5.6.2  | engine developer (P-26)    |
| US-9.5.6.3  | technical artist (P-13)    |

1. **US-9.5.5.1** -- **As a** rigger (P-10), **I want** cloth-body collision resolved using capsule
   and convex hull proxies attached to skeleton bones, **so that** cloth does not pass through limbs
   during fast movement.

2. **US-9.5.5.2** -- **As an** engine developer (P-26), **I want** collision proxy count to scale
   per platform tier, **so that** collision fidelity matches the available compute budget.

3. **US-9.5.6.1** -- **As a** character animator (P-11), **I want** hair to respond to directional
   and turbulent wind sampled from the shared wind field, **so that** hair motion is visually
   coherent with all other simulated elements.

4. **US-9.5.6.2** -- **As an** engine developer (P-26), **I want** per-particle aerodynamic drag on
   desktop strand simulation with simplified spring displacement on mobile card hair, **so that**
   wind response is present on all platforms.

5. **US-9.5.6.3** -- **As a** technical artist (P-13), **I want** per-strand wind response
   parameters (drag coefficient, turbulence scale) tunable per character, **so that** hair wind
   behavior matches the character's hair style and weight.

## Parent Stories

The 3-segment parent stories below are umbrella rollups for the refined 4-segment sub-stories listed
above. Each parent inherits the persona of its first sub-story and describes the umbrella capability
that the sub-stories refine.

| ID | Persona |
|----|---------|
| US-9.5.1 | character animator (P-11) |
| US-9.5.2 | character animator (P-11) |
| US-9.5.3 | character animator (P-11) |
| US-9.5.4 | engine developer (P-26) |
| US-9.5.5 | rigger (P-10) |
| US-9.5.6 | character animator (P-11) |

1. **US-9.5.1** -- **As a** character animator (P-11), **I want** the capabilities defined in
   sub-stories US-9.5.1.1 through US-9.5.1.3 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

2. **US-9.5.2** -- **As a** character animator (P-11), **I want** the capabilities defined in
   sub-stories US-9.5.2.1 through US-9.5.2.3 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

3. **US-9.5.3** -- **As a** character animator (P-11), **I want** the capabilities defined in
   sub-stories US-9.5.3.1 through US-9.5.3.2 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

4. **US-9.5.4** -- **As a** engine developer (P-26), **I want** the capabilities defined in
   sub-stories US-9.5.4.1 through US-9.5.4.2 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

5. **US-9.5.5** -- **As a** rigger (P-10), **I want** the capabilities defined in sub-stories
   US-9.5.5.1 through US-9.5.5.2 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

6. **US-9.5.6** -- **As a** character animator (P-11), **I want** the capabilities defined in
   sub-stories US-9.5.6.1 through US-9.5.6.3 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.
