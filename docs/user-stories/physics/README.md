# Physics User Stories

User stories for Domain 4: Physics & Simulation, derived from feature specifications in
[docs/features/physics/](../../features/physics/) and requirements in
[docs/requirements/physics/](../../requirements/physics/).

## Roles

| Role | Description |
|------|-------------|
| Gameplay programmer | Integrates physics systems into game logic via ECS APIs |
| Designer | Configures physics behavior through visual editors and component properties |
| Player | Experiences physics interactions during gameplay |
| QA engineer | Verifies physics correctness, performance, and stability |

## Files

| File | Category | Stories |
|------|----------|--------|
| [rigid-body-dynamics.md](rigid-body-dynamics.md) | Integration, collision response, islands, sleeping, character movement | 19 |
| [collision-detection.md](collision-detection.md) | Broadphase, narrowphase, shapes, filtering, events, triggers, materials | 15 |
| [constraints-and-joints.md](constraints-and-joints.md) | Joint types, motors, limits, ragdolls, chains, solvers, severance, prosthetics | 17 |
| [spatial-queries.md](spatial-queries.md) | Ray casts, shape casts, overlaps, closest point, batching, filtering | 10 |
| [vehicle-physics.md](vehicle-physics.md) | Suspension, tires, drivetrain, stability, tracked, hover, replication | 13 |
| [destruction-and-fracture.md](destruction-and-fracture.md) | Fracture generation, runtime destruction, structural collapse, debris | 13 |
| [soft-body-and-cloth.md](soft-body-and-cloth.md) | XPBD solver, cloth, self-collision, coupling, wind, tearing, LOD | 13 |
| [fluid-simulation.md](fluid-simulation.md) | SPH, FLIP/PIC, Eulerian, surface reconstruction, water, buoyancy | 13 |

**Total: 113 user stories** (1-2 per feature across 62 features)
