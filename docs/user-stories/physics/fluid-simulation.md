# User Stories -- 4.8 Fluid Simulation

## Particle-Based Methods

| ID         | Persona                 |
|------------|-------------------------|
| US-4.8.1.1 | game developer (P-15)   |
| US-4.8.1.2 | engine developer (P-26) |
| US-4.8.2.1 | game developer (P-15)   |
| US-4.8.2.2 | engine tester (P-27)    |

1. **US-4.8.1.1** -- **As a** game developer (P-15), **I want** SPH fluid volumes as ECS entities
   with GPU particle buffers, **so that** localized fluid effects simulate at interactive rates.
2. **US-4.8.1.2** -- **As a** engine developer (P-26), **I want** particle counts bounded per
   FluidVolume, **so that** frame budgets are maintained per platform tier.
3. **US-4.8.2.1** -- **As a** game developer (P-15), **I want** FLIP/PIC hybrid simulation for
   large-scale flooding, **so that** grid stability combines with particle detail preservation.
4. **US-4.8.2.2** -- **As a** engine tester (P-27), **I want** to verify mobile disables
   particle-based fluid by default, **so that** GPU budget is preserved.

## Grid-Based Methods

| ID         | Persona                 |
|------------|-------------------------|
| US-4.8.3.1 | game developer (P-15)   |
| US-4.8.3.2 | game designer (P-5)     |

1. **US-4.8.3.1** -- **As a** game developer (P-15), **I want** Eulerian grid fluid solvers for
   bounded water volumes (lakes, moats), **so that** calm enclosed water bodies simulate stably.
2. **US-4.8.3.2** -- **As a** game designer (P-5), **I want** grid resolution configurable per
   entity, **so that** I trade simulation detail for performance per water body.

## Surface and Rendering Integration

| ID         | Persona                 |
|------------|-------------------------|
| US-4.8.4.1 | engine developer (P-26) |
| US-4.8.4.2 | game developer (P-15)   |
| US-4.8.5.1 | game developer (P-15)   |
| US-4.8.5.2 | engine developer (P-26) |

1. **US-4.8.4.1** -- **As a** engine developer (P-26), **I want** fluid surface reconstruction from
   particle buffers into renderable meshes, **so that** fluid data bridges to the rendering
   pipeline.
2. **US-4.8.4.2** -- **As a** game developer (P-15), **I want** reconstructed surfaces to support
   refraction, reflection, and foam effects, **so that** fluid looks like water.
3. **US-4.8.5.1** -- **As a** game developer (P-15), **I want** ocean, river, and lake entities with
   WaterSurface components, **so that** water bodies have unified wave synthesis and flow fields.
4. **US-4.8.5.2** -- **As a** engine developer (P-26), **I want** WaterSurface to integrate with the
   visual water rendering system (F-3.4.1), **so that** physics and rendering share wave state.

## Rigid Body Interaction

| ID         | Persona                 |
|------------|-------------------------|
| US-4.8.6.1 | game developer (P-15)   |
| US-4.8.6.2 | game designer (P-5)     |
| US-4.8.7.1 | engine developer (P-26) |
| US-4.8.7.2 | engine tester (P-27)    |

1. **US-4.8.6.1** -- **As a** game developer (P-15), **I want** buoyancy and drag forces applied to
   rigid bodies overlapping fluid volumes, **so that** objects float and sink realistically.
2. **US-4.8.6.2** -- **As a** game designer (P-5), **I want** buoyant body count and sample points
   configurable per platform tier, **so that** mobile limits buoyancy cost.
3. **US-4.8.7.1** -- **As a** engine developer (P-26), **I want** two-way fluid-rigid body coupling
   where submerged bodies displace fluid, **so that** splashes and wakes emerge naturally.
4. **US-4.8.7.2** -- **As a** engine tester (P-27), **I want** to verify mobile falls back to
   one-way coupling only, **so that** fluid displacement is disabled on constrained devices.
