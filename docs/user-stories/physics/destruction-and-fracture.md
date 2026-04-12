# User Stories -- 4.6 Destruction & Fracture

## Fracture Generation

| ID         | Persona                 |
|------------|-------------------------|
| US-4.6.1.1 | game developer (P-15)   |
| US-4.6.1.2 | game designer (P-5)     |
| US-4.6.2.1 | game developer (P-15)   |
| US-4.6.2.2 | engine tester (P-27)    |

1. **US-4.6.1.1** -- **As a** game developer (P-15), **I want** Voronoi fracture patterns generated
   at build time with configurable seeding, **so that** breakable objects have pre-computed fragment
   data.
2. **US-4.6.1.2** -- **As a** game designer (P-5), **I want** fragment count per fracture asset
   capped per platform tier, **so that** destruction scales with device capability.
3. **US-4.6.2.1** -- **As a** game developer (P-15), **I want** pre-fractured meshes from DCC tools
   loaded as fracture assets, **so that** artists author art-directed destruction for hero objects.
4. **US-4.6.2.2** -- **As a** engine tester (P-27), **I want** to verify fracture assets load
   connectivity graphs and break thresholds, **so that** runtime fracture has correct structural
   data.

## Runtime Destruction

| ID         | Persona                 |
|------------|-------------------------|
| US-4.6.3.1 | game developer (P-15)   |
| US-4.6.3.2 | engine developer (P-26) |
| US-4.6.4.1 | game designer (P-5)     |
| US-4.6.4.2 | engine tester (P-27)    |

1. **US-4.6.3.1** -- **As a** game developer (P-15), **I want** fracture triggered when cumulative
   damage exceeds a threshold, spawning fragment entities with physics, **so that** objects break
   apart at runtime.
2. **US-4.6.3.2** -- **As a** engine developer (P-26), **I want** fragment spawning budgeted per
   frame to avoid hitches, **so that** large-scale destruction stays within frame time.
3. **US-4.6.4.1** -- **As a** game designer (P-5), **I want** progressive damage with visual
   cracking stages before full fracture, **so that** players see feedback before an object breaks.
4. **US-4.6.4.2** -- **As a** engine tester (P-27), **I want** to verify DamageHealth tracks
   cumulative damage and triggers fracture at the correct threshold, **so that** destruction timing
   is predictable.

## Structural Analysis

| ID         | Persona                 |
|------------|-------------------------|
| US-4.6.5.1 | game developer (P-15)   |
| US-4.6.5.2 | engine developer (P-26) |

1. **US-4.6.5.1** -- **As a** game developer (P-15), **I want** stress propagation through fragment
   connectivity graphs, **so that** removing a load-bearing piece causes cascading collapse.
2. **US-4.6.5.2** -- **As a** engine developer (P-26), **I want** unsupported fragments to lose
   their joints and fall under gravity, **so that** structural collapse emerges from connectivity
   analysis.

## Debris Management

| ID         | Persona                 |
|------------|-------------------------|
| US-4.6.6.1 | game developer (P-15)   |
| US-4.6.6.2 | engine tester (P-27)    |
| US-4.6.7.1 | engine developer (P-26) |
| US-4.6.7.2 | game designer (P-5)     |

1. **US-4.6.6.1** -- **As a** game developer (P-15), **I want** debris entities with configurable
   time-to-live and maximum count, **so that** fragments clean up automatically after destruction.
2. **US-4.6.6.2** -- **As a** engine tester (P-27), **I want** to verify debris TTL and count caps
   per platform tier, **so that** mobile limits debris to stay within budget.
3. **US-4.6.7.1** -- **As a** engine developer (P-26), **I want** debris entity pooling to reuse
   despawned fragments, **so that** allocation churn is eliminated during destruction.
4. **US-4.6.7.2** -- **As a** game designer (P-5), **I want** distant debris reduced to visual-only
   particles, **so that** far-away fragments have zero simulation cost.

## Voxel and SDF Integration

| ID          | Persona                 |
|-------------|-------------------------|
| US-4.6.8.1  | engine developer (P-26) |
| US-4.6.9.1  | game developer (P-15)   |
| US-4.6.10.1 | engine developer (P-26) |
| US-4.6.11.1 | game developer (P-15)   |
| US-4.6.11.2 | engine tester (P-27)    |

1. **US-4.6.8.1** -- **As a** engine developer (P-26), **I want** colliders auto-generated from
   voxel SDF meshes with incremental updates, **so that** runtime terrain edits produce correct
   physics.
2. **US-4.6.9.1** -- **As a** game developer (P-15), **I want** fracture fragments to inherit parent
   velocity with impact-directed impulse, **so that** debris flies away from the impact point
   realistically.
3. **US-4.6.10.1** -- **As a** engine developer (P-26), **I want** SDF sphere-tracing for raycasts
   against voxel volumes, **so that** SDF queries serve as fast pre-checks before mesh narrowphase.
4. **US-4.6.11.1** -- **As a** game developer (P-15), **I want** physics damage events to trigger
   voxel SDF subtraction on terrain, **so that** explosions carve craters in voxel terrain.
5. **US-4.6.11.2** -- **As a** engine tester (P-27), **I want** to verify structural integrity
   checks on disconnected voxel islands, **so that** floating terrain fragments fall under gravity.

## Parent Stories

The 3-segment parent stories below are umbrella rollups for the refined 4-segment sub-stories listed
above. Each parent inherits the persona of its first sub-story and describes the umbrella capability
that the sub-stories refine.

| ID | Persona |
|----|---------|
| US-4.6.1 | game developer (P-15) |
| US-4.6.10 | engine developer (P-26) |
| US-4.6.11 | game developer (P-15) |
| US-4.6.2 | game developer (P-15) |
| US-4.6.3 | game developer (P-15) |
| US-4.6.4 | game designer (P-5) |
| US-4.6.5 | game developer (P-15) |
| US-4.6.6 | game developer (P-15) |
| US-4.6.7 | engine developer (P-26) |
| US-4.6.8 | engine developer (P-26) |
| US-4.6.9 | game developer (P-15) |

1. **US-4.6.1** -- **As a** game developer (P-15), **I want** the capabilities defined in
   sub-stories US-4.6.1.1 through US-4.6.1.2 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

2. **US-4.6.10** -- **As a** engine developer (P-26), **I want** the capabilities defined in
   sub-stories US-4.6.10.1 through US-4.6.10.1 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

3. **US-4.6.11** -- **As a** game developer (P-15), **I want** the capabilities defined in
   sub-stories US-4.6.11.1 through US-4.6.11.2 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

4. **US-4.6.2** -- **As a** game developer (P-15), **I want** the capabilities defined in
   sub-stories US-4.6.2.1 through US-4.6.2.2 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

5. **US-4.6.3** -- **As a** game developer (P-15), **I want** the capabilities defined in
   sub-stories US-4.6.3.1 through US-4.6.3.2 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

6. **US-4.6.4** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-4.6.4.1 through US-4.6.4.2 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

7. **US-4.6.5** -- **As a** game developer (P-15), **I want** the capabilities defined in
   sub-stories US-4.6.5.1 through US-4.6.5.2 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

8. **US-4.6.6** -- **As a** game developer (P-15), **I want** the capabilities defined in
   sub-stories US-4.6.6.1 through US-4.6.6.2 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

9. **US-4.6.7** -- **As a** engine developer (P-26), **I want** the capabilities defined in
   sub-stories US-4.6.7.1 through US-4.6.7.2 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

10. **US-4.6.8** -- **As a** engine developer (P-26), **I want** the capabilities defined in
    sub-stories
US-4.6.8.1 through US-4.6.8.1 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.

11. **US-4.6.9** -- **As a** game developer (P-15), **I want** the capabilities defined in
    sub-stories
US-4.6.9.1 through US-4.6.9.1 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.
