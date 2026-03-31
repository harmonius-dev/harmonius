# User Stories -- 4.5 Vehicle Physics

## Suspension and Tires

| ID         | Persona                 |
|------------|-------------------------|
| US-4.5.1.1 | game developer (P-15)   |
| US-4.5.1.2 | game designer (P-5)     |
| US-4.5.2.1 | game developer (P-15)   |
| US-4.5.2.2 | engine tester (P-27)    |

1. **US-4.5.1.1** -- **As a** game developer (P-15), **I want** wheel entities with configurable
   suspension stiffness, damping, and travel limits, **so that** vehicles respond to terrain
   realistically.
2. **US-4.5.1.2** -- **As a** game designer (P-5), **I want** suspension parameters exposed in the
   visual editor, **so that** I tune ride feel without code.
3. **US-4.5.2.1** -- **As a** game developer (P-15), **I want** Pacejka tire friction with
   surface-type friction curves, **so that** tire grip varies by road surface.
4. **US-4.5.2.2** -- **As a** engine tester (P-27), **I want** to verify mobile uses a simplified
   linear friction model, **so that** mobile frame budgets hold.

## Drivetrain

| ID         | Persona                 |
|------------|-------------------------|
| US-4.5.3.1 | game developer (P-15)   |
| US-4.5.3.2 | game designer (P-5)     |
| US-4.5.3.3 | engine tester (P-27)    |

1. **US-4.5.3.1** -- **As a** game developer (P-15), **I want** drivetrain simulation with engine
   torque curves, gear ratios, and differential types, **so that** vehicle acceleration and speed
   feel authentic.
2. **US-4.5.3.2** -- **As a** game designer (P-5), **I want** to configure drive layout (FWD, RWD,
   AWD) and gear shift points, **so that** I control vehicle handling characteristics.
3. **US-4.5.3.3** -- **As a** engine tester (P-27), **I want** to verify torque distribution matches
   the configured differential, **so that** FWD and RWD behave distinctly.

## Stability

| ID         | Persona                 |
|------------|-------------------------|
| US-4.5.4.1 | game developer (P-15)   |
| US-4.5.4.2 | game designer (P-5)     |

1. **US-4.5.4.1** -- **As a** game developer (P-15), **I want** anti-roll bars and stability control
   as optional components, **so that** vehicles resist body roll and maintain traction.
2. **US-4.5.4.2** -- **As a** game designer (P-5), **I want** stability assists disableable per
   entity, **so that** realistic and arcade handling coexist.

## Specialized Vehicles

| ID         | Persona                 |
|------------|-------------------------|
| US-4.5.5.1 | game developer (P-15)   |
| US-4.5.5.2 | engine tester (P-27)    |
| US-4.5.6.1 | game developer (P-15)   |
| US-4.5.6.2 | game designer (P-5)     |
| US-4.5.7.1 | engine developer (P-26) |
| US-4.5.7.2 | engine tester (P-27)    |

1. **US-4.5.5.1** -- **As a** game developer (P-15), **I want** tracked vehicle simulation with
   differential track steering, **so that** tanks and bulldozers steer by varying left and right
   track speeds.
2. **US-4.5.5.2** -- **As a** engine tester (P-27), **I want** to verify tracked vehicle shape casts
   per track scale per platform, **so that** mobile limits ground sampling cost.
3. **US-4.5.6.1** -- **As a** game developer (P-15), **I want** hover vehicles with configurable
   repulsor height and lateral friction, **so that** hovercraft and speeders float over terrain.
4. **US-4.5.6.2** -- **As a** game designer (P-5), **I want** hover stabilization keeping the
   vehicle level over terrain edges, **so that** hover vehicles feel controllable.
5. **US-4.5.7.1** -- **As a** engine developer (P-26), **I want** vehicle state replicated via the
   ECS state replication system, **so that** multiplayer vehicles synchronize without custom
   serialization.
6. **US-4.5.7.2** -- **As a** engine tester (P-27), **I want** to verify client prediction
   reconciles correctly on server snapshots, **so that** multiplayer vehicle movement is smooth.
