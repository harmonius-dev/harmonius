# User Stories -- 4.1 Rigid Body Dynamics

## Integration Methods

| ID          | Persona                 |
|-------------|-------------------------|
| US-4.1.1.1  | game developer (P-15)   |
| US-4.1.1.2  | engine developer (P-26) |
| US-4.1.1.3  | engine tester (P-27)    |
| US-4.1.2.1  | game designer (P-5)     |
| US-4.1.2.2  | engine developer (P-26) |

1. **US-4.1.1.1** -- **As a** game developer (P-15), **I want** rigid bodies integrated with
   symplectic Euler at a fixed timestep, **so that** physics simulation is deterministic across
   runs.
2. **US-4.1.1.2** -- **As a** engine developer (P-26), **I want** the IntegrationSystem to read
   force and torque components and write updated velocity and transform, **so that** the integration
   loop is a pure ECS system.
3. **US-4.1.1.3** -- **As a** engine tester (P-27), **I want** to verify bit-identical simulation
   output across platforms, **so that** server-authoritative reconciliation is reliable.
4. **US-4.1.2.1** -- **As a** game designer (P-5), **I want** configurable substep count via a
   PhysicsConfig resource, **so that** I can trade accuracy for performance per project.
5. **US-4.1.2.2** -- **As a** engine developer (P-26), **I want** per-entity SubstepOverride
   components, **so that** high-fidelity objects use more substeps without increasing the global
   count.

## Collision Response

| ID          | Persona                 |
|-------------|-------------------------|
| US-4.1.3.1  | game developer (P-15)   |
| US-4.1.3.2  | game designer (P-5)     |
| US-4.1.4.1  | game developer (P-15)   |
| US-4.1.4.2  | engine tester (P-27)    |

1. **US-4.1.3.1** -- **As a** game developer (P-15), **I want** impulse-based contact resolution
   using PhysicsMaterial restitution and friction, **so that** bouncing and sliding match surface
   properties.
2. **US-4.1.3.2** -- **As a** game designer (P-5), **I want** material combination rules (average,
   min, max, multiply) stored in an ECS resource, **so that** I configure collision behavior without
   code.
3. **US-4.1.4.1** -- **As a** game developer (P-15), **I want** CCD via swept-volume queries on
   flagged entities, **so that** fast-moving projectiles do not tunnel through thin walls.
4. **US-4.1.4.2** -- **As a** engine tester (P-27), **I want** to verify that CCD prevents tunneling
   for bullets at maximum speed, **so that** no projectile passes through geometry.

## Islands and Sleeping

| ID          | Persona                 |
|-------------|-------------------------|
| US-4.1.5.1  | engine developer (P-26) |
| US-4.1.5.2  | engine tester (P-27)    |
| US-4.1.6.1  | engine developer (P-26) |
| US-4.1.6.2  | game developer (P-15)   |

1. **US-4.1.5.1** -- **As a** engine developer (P-26), **I want** interacting bodies partitioned
   into simulation islands via union-find, **so that** independent islands solve in parallel.
2. **US-4.1.5.2** -- **As a** engine tester (P-27), **I want** to verify island merging and
   splitting each frame, **so that** newly contacting bodies join the same island.
3. **US-4.1.6.1** -- **As a** engine developer (P-26), **I want** bodies at rest to receive a
   Sleeping marker that skips integration, **so that** CPU budget is spent only on active bodies.
4. **US-4.1.6.2** -- **As a** game developer (P-15), **I want** sleeping bodies to wake
   automatically on external force or new contact, **so that** gameplay interactions always trigger
   physics response.

## Streaming and Scale

| ID          | Persona                 |
|-------------|-------------------------|
| US-4.1.7.1  | engine developer (P-26) |
| US-4.1.7.2  | game developer (P-15)   |

1. **US-4.1.7.1** -- **As a** engine developer (P-26), **I want** physics entities migrated between
   streaming zones with momentum preserved, **so that** bodies crossing zone boundaries simulate
   continuously.
2. **US-4.1.7.2** -- **As a** game developer (P-15), **I want** no visible discontinuity when a
   rigid body crosses a zone border, **so that** open-world physics feels seamless.

## Character Movement

| ID           | Persona                 |
|--------------|-------------------------|
| US-4.1.8.1   | game developer (P-15)   |
| US-4.1.8.2   | game designer (P-5)     |
| US-4.1.9.1   | game developer (P-15)   |
| US-4.1.9.2   | game designer (P-5)     |
| US-4.1.10.1  | game developer (P-15)   |
| US-4.1.10.2  | engine tester (P-27)    |

1. **US-4.1.8.1** -- **As a** game developer (P-15), **I want** a kinematic character controller
   with ground detection, slope sliding, and step climbing, **so that** characters navigate terrain
   reliably.
2. **US-4.1.8.2** -- **As a** game designer (P-5), **I want** configurable slope limits and
   coyote-time grace periods, **so that** I tune movement feel per game genre.
3. **US-4.1.9.1** -- **As a** game developer (P-15), **I want** moving platforms that transport
   characters without sliding or jitter, **so that** elevators and conveyor belts work correctly.
4. **US-4.1.9.2** -- **As a** game designer (P-5), **I want** one-way platforms and directional
   filtering, **so that** platformer mechanics are possible.
5. **US-4.1.10.1** -- **As a** game developer (P-15), **I want** smoothed ground conformance that
   filters micro-bouncing on rough terrain, **so that** characters walk smoothly over tessellated
   surfaces.
6. **US-4.1.10.2** -- **As a** engine tester (P-27), **I want** to verify surface smoothing
   eliminates vibration on triangle edges and tile seams, **so that** character movement is stable.
