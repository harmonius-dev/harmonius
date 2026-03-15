# User Stories — 4.5 Vehicle Physics

## F-4.5.1 Wheel Suspension Model

**US-4.5.1a** As a designer, I want to tune spring stiffness, damping, and travel limits
per wheel so that vehicle suspension behavior matches the intended handling feel — from
soft off-road trucks to stiff sports cars.

**US-4.5.1b** As a player, I want a parked vehicle to settle to a stable rest height
without bouncing indefinitely so that vehicles look grounded and realistic when stationary.

## F-4.5.2 Tire Friction Model

**US-4.5.2a** As a designer, I want to define Pacejka tire curves and per-surface friction
tables so that tires grip differently on asphalt, gravel, mud, and ice without code changes.

**US-4.5.2b** As a player, I want cornering to feel different on wet roads versus dry
asphalt so that surface conditions affect driving strategy.

## F-4.5.3 Drivetrain Simulation

**US-4.5.3a** As a designer, I want to configure engine torque curves, gear ratios,
differential type, and drive layout (FWD, RWD, AWD) so that each vehicle class has a
distinct driving character.

**US-4.5.3b** As a player, I want to feel the engine's torque curve through acceleration
response and hear gear shifts at the right RPM so that driving feels mechanically authentic.

## F-4.5.4 Anti-Roll Bars and Stability Control

**US-4.5.4a** As a designer, I want anti-roll bar and stability control components that I
can enable or tune per vehicle so that arcade vehicles resist rollover while simulation
vehicles can be configured to allow it.

## F-4.5.5 Tracked Vehicle Simulation

**US-4.5.5a** As a designer, I want tracked vehicle steering via differential track speeds
so that tanks and construction vehicles turn realistically with pivot, gradual, and neutral
steering modes.

**US-4.5.5b** As a player, I want driving a tank to feel heavy and distinct from wheeled
vehicles so that tracked vehicles have their own satisfying handling character.

## F-4.5.6 Hover Vehicle Simulation

**US-4.5.6a** As a designer, I want hover vehicles with height-dependent repulsion and tilt
stabilization so that sci-fi hovercraft glide over terrain and water with a floaty,
responsive feel.

**US-4.5.6b** As a player, I want a hover vehicle to bob gently over terrain edges and
maintain stable altitude so that hover driving feels smooth and distinct from ground
vehicles.

## F-4.5.7 Server-Authoritative Vehicle Replication

**US-4.5.7a** As a gameplay programmer, I want vehicle components to replicate via the
standard ECS replication system so that multiplayer vehicles predict locally and reconcile
with the server without custom serialization.

**US-4.5.7b** As a player, I want other players' vehicles to move smoothly without
teleporting or rubber-banding under normal network conditions so that multiplayer driving
feels fair and responsive.
