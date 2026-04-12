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

## Advanced Forces and Friction

| ID           | Persona                 |
|--------------|-------------------------|
| US-4.1.11.1  | game developer (P-15)   |
| US-4.1.12.1  | game designer (P-5)     |
| US-4.1.13.1  | game designer (P-5)     |

1. **US-4.1.11.1** -- **As a** game developer (P-15), **I want** optional gyroscopic torque applied
   during integration for flagged rigid bodies, **so that** spinning tops, wheels, and projectiles
   precess correctly under external torque without paying the cost on every body.
2. **US-4.1.12.1** -- **As a** game designer (P-5), **I want** a rolling friction coefficient on
   physics materials that brings spheres and wheels to rest on flat ground, **so that** billiards,
   vehicles, and marble gameplay behave realistically without infinite rolling.
3. **US-4.1.13.1** -- **As a** game designer (P-5), **I want** directional friction on physics
   materials with a separate lateral coefficient, **so that** ice surfaces, rails, and conveyor
   belts have grip along one axis and slip along the other.

## Gravity Models and Multi-Planet

| ID           | Persona                 |
|--------------|-------------------------|
| US-4.1.14.1  | game designer (P-5)     |
| US-4.1.14.2  | engine developer (P-26) |
| US-4.1.15.1  | game developer (P-15)   |
| US-4.1.15.2  | engine tester (P-27)    |

1. **US-4.1.14.1** -- **As a** game designer (P-5), **I want** to select uniform, radial, or custom
   gravity per world via an ECS resource, **so that** I can switch between flat-world and planet
   physics without rewriting the integration system.
2. **US-4.1.14.2** -- **As an** engine developer (P-26), **I want** all collision detection and
   constraint solving to run in flat planet-local Cartesian space regardless of gravity mode,
   **so that** curved-space physics math is never required.
3. **US-4.1.15.1** -- **As a** game developer (P-15), **I want** each planet to be a separate ECS
   world with its own physics BVH and gravity, **so that** space exploration games with multiple
   planets simulate each one independently.
4. **US-4.1.15.2** -- **As an** engine tester (P-27), **I want** to verify that inter-planetary
   entity migration preserves velocity through the universe-level transform, **so that** bodies
   crossing planet boundaries remain physically consistent.

## 2D Physics

| ID           | Persona                 |
|--------------|-------------------------|
| US-4.1.16.1  | game developer (P-15)   |
| US-4.1.16.2  | game developer (P-15)   |

1. **US-4.1.16.1** -- **As a** game developer (P-15), **I want** a 2D rigid body mode with scalar
   moment of inertia and 2D collider shapes, **so that** I can build 2D and 2.5D games on the same
   island, sleeping, CCD, and solver infrastructure as 3D physics.
2. **US-4.1.16.2** -- **As a** game developer (P-15), **I want** 2D and 3D physics to coexist in the
   same ECS world, **so that** 2.5D games can mix 2D gameplay physics with 3D debris and particle
   effects.

## Character Controller Extensions

| ID           | Persona                 |
|--------------|-------------------------|
| US-4.1.17.1  | game designer (P-5)     |
| US-4.1.18.1  | game designer (P-5)     |
| US-4.1.19.1  | game developer (P-15)   |
| US-4.1.20.1  | game developer (P-15)   |

1. **US-4.1.17.1** -- **As a** game designer (P-5), **I want** characters to slide along walls with
   configurable wall friction and corner handling, **so that** traversal feels smooth in tight
   corridors and around steep geometry without abrupt stops.
2. **US-4.1.18.1** -- **As a** game designer (P-5), **I want** multi-jump, wall jump, and jump
   buffer parameters tunable per character, **so that** platformer movement feel can be dialed in
   without code changes.
3. **US-4.1.19.1** -- **As a** game developer (P-15), **I want** crouching to shape-cast for ceiling
   clearance before uncrouching, **so that** characters cannot pop into solid geometry under low
   overhangs.
4. **US-4.1.20.1** -- **As a** game developer (P-15), **I want** character controllers to push
   dynamic rigid bodies on contact with a configurable push strength, **so that** players visibly
   interact with loose props rather than treating them as immovable.

## Parent Stories

The 3-segment parent stories below are umbrella rollups for the refined 4-segment sub-stories listed
above. Each parent inherits the persona of its first sub-story and describes the umbrella capability
that the sub-stories refine.

| ID | Persona |
|----|---------|
| US-4.1.1 | game developer (P-15) |
| US-4.1.10 | game developer (P-15) |
| US-4.1.11 | game developer (P-15) |
| US-4.1.12 | game designer (P-5) |
| US-4.1.13 | game designer (P-5) |
| US-4.1.14 | game designer (P-5) |
| US-4.1.15 | game developer (P-15) |
| US-4.1.16 | game developer (P-15) |
| US-4.1.17 | game designer (P-5) |
| US-4.1.18 | game designer (P-5) |
| US-4.1.19 | game developer (P-15) |
| US-4.1.2 | game designer (P-5) |
| US-4.1.20 | game developer (P-15) |
| US-4.1.3 | game developer (P-15) |
| US-4.1.4 | game developer (P-15) |
| US-4.1.5 | engine developer (P-26) |
| US-4.1.6 | engine developer (P-26) |
| US-4.1.7 | engine developer (P-26) |
| US-4.1.8 | game developer (P-15) |
| US-4.1.9 | game developer (P-15) |

1. **US-4.1.1** -- **As a** game developer (P-15), **I want** the capabilities defined in
   sub-stories US-4.1.1.1 through US-4.1.1.3 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

2. **US-4.1.10** -- **As a** game developer (P-15), **I want** the capabilities defined in
   sub-stories US-4.1.10.1 through US-4.1.10.2 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

3. **US-4.1.11** -- **As a** game developer (P-15), **I want** the capabilities defined in
   sub-stories US-4.1.11.1 through US-4.1.11.1 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

4. **US-4.1.12** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-4.1.12.1 through US-4.1.12.1 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

5. **US-4.1.13** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-4.1.13.1 through US-4.1.13.1 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

6. **US-4.1.14** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-4.1.14.1 through US-4.1.14.2 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

7. **US-4.1.15** -- **As a** game developer (P-15), **I want** the capabilities defined in
   sub-stories US-4.1.15.1 through US-4.1.15.2 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

8. **US-4.1.16** -- **As a** game developer (P-15), **I want** the capabilities defined in
   sub-stories US-4.1.16.1 through US-4.1.16.2 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

9. **US-4.1.17** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-4.1.17.1 through US-4.1.17.1 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

10. **US-4.1.18** -- **As a** game designer (P-5), **I want** the capabilities defined in
    sub-stories
US-4.1.18.1 through US-4.1.18.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

11. **US-4.1.19** -- **As a** game developer (P-15), **I want** the capabilities defined in
    sub-stories
US-4.1.19.1 through US-4.1.19.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

12. **US-4.1.2** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
US-4.1.2.1 through US-4.1.2.2 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.

13. **US-4.1.20** -- **As a** game developer (P-15), **I want** the capabilities defined in
    sub-stories
US-4.1.20.1 through US-4.1.20.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

14. **US-4.1.3** -- **As a** game developer (P-15), **I want** the capabilities defined in
    sub-stories
US-4.1.3.1 through US-4.1.3.2 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.

15. **US-4.1.4** -- **As a** game developer (P-15), **I want** the capabilities defined in
    sub-stories
US-4.1.4.1 through US-4.1.4.2 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.

16. **US-4.1.5** -- **As a** engine developer (P-26), **I want** the capabilities defined in
    sub-stories
US-4.1.5.1 through US-4.1.5.2 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.

17. **US-4.1.6** -- **As a** engine developer (P-26), **I want** the capabilities defined in
    sub-stories
US-4.1.6.1 through US-4.1.6.2 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.

18. **US-4.1.7** -- **As a** engine developer (P-26), **I want** the capabilities defined in
    sub-stories
US-4.1.7.1 through US-4.1.7.2 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.

19. **US-4.1.8** -- **As a** game developer (P-15), **I want** the capabilities defined in
    sub-stories
US-4.1.8.1 through US-4.1.8.2 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.

20. **US-4.1.9** -- **As a** game developer (P-15), **I want** the capabilities defined in
    sub-stories
US-4.1.9.1 through US-4.1.9.2 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.
