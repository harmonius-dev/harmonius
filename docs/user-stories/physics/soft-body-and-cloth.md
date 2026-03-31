# User Stories -- 4.7 Soft Body & Cloth

## Simulation Core

| ID         | Persona                 |
|------------|-------------------------|
| US-4.7.1.1 | engine developer (P-26) |
| US-4.7.1.2 | game developer (P-15)   |
| US-4.7.2.1 | game developer (P-15)   |
| US-4.7.2.2 | game designer (P-5)     |

1. **US-4.7.1.1** -- **As a** engine developer (P-26), **I want** an XPBD solver querying
   ClothSimulation components with GPU buffer handles, **so that** soft body constraints resolve
   within the ECS schedule.
2. **US-4.7.1.2** -- **As a** game developer (P-15), **I want** per-entity compliance parameters for
   stiffness control, **so that** each cloth instance has independent material behavior.
3. **US-4.7.2.1** -- **As a** game developer (P-15), **I want** cloth instances attached to skeleton
   bones via ClothAttachment, **so that** capes and banners follow animated characters.
4. **US-4.7.2.2** -- **As a** game designer (P-5), **I want** active cloth instance count capped per
   platform tier, **so that** mobile limits cloth to the player character.

## Collision and Interaction

| ID         | Persona                 |
|------------|-------------------------|
| US-4.7.3.1 | game developer (P-15)   |
| US-4.7.3.2 | engine tester (P-27)    |
| US-4.7.4.1 | engine developer (P-26) |
| US-4.7.4.2 | game developer (P-15)   |

1. **US-4.7.3.1** -- **As a** game developer (P-15), **I want** cloth self-collision via a
   SelfCollisionEnabled marker, **so that** folding cloth does not pass through itself.
2. **US-4.7.3.2** -- **As a** engine tester (P-27), **I want** to verify self-collision is disabled
   by default on mobile, **so that** GPU budget is preserved.
3. **US-4.7.4.1** -- **As a** engine developer (P-26), **I want** the XPBD solver to read nearby
   rigid body colliders and write reaction forces, **so that** cloth drapes over objects with
   two-way coupling.
4. **US-4.7.4.2** -- **As a** game developer (P-15), **I want** rigid bodies to deflect from cloth
   contact, **so that** objects interacting with cloth feel physically connected.

## Environmental Effects

| ID         | Persona                 |
|------------|-------------------------|
| US-4.7.5.1 | game developer (P-15)   |
| US-4.7.5.2 | engine developer (P-26) |
| US-4.7.6.1 | game developer (P-15)   |
| US-4.7.6.2 | engine tester (P-27)    |

1. **US-4.7.5.1** -- **As a** game developer (P-15), **I want** wind sources as ECS entities
   generating a shared 3D wind field texture, **so that** cloth, hair, foliage, and particles all
   sample the same wind.
2. **US-4.7.5.2** -- **As a** engine developer (P-26), **I want** wind field texture resolution
   configurable per platform tier, **so that** mobile uses a coarser wind volume.
3. **US-4.7.6.1** -- **As a** game developer (P-15), **I want** cloth tearing when constraint strain
   exceeds a threshold, **so that** sails and banners rip during combat.
4. **US-4.7.6.2** -- **As a** engine tester (P-27), **I want** to verify tear events per frame are
   capped per platform, **so that** mobile uses pre-authored torn mesh swaps instead.

## Performance

| ID         | Persona                 |
|------------|-------------------------|
| US-4.7.7.1 | game designer (P-5)     |
| US-4.7.7.2 | engine developer (P-26) |

1. **US-4.7.7.1** -- **As a** game designer (P-5), **I want** cloth LOD that reduces particle count
   and update frequency at distance, **so that** only nearby cloth runs full simulation.
2. **US-4.7.7.2** -- **As a** engine developer (P-26), **I want** extreme-distance cloth replaced
   with an animation fallback at zero simulation cost, **so that** distant cloth has no physics
   overhead.
