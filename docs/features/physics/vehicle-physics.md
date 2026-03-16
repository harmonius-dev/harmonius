# 4.5 — Vehicle Physics

## Suspension and Tires

### F-4.5.1 Wheel Suspension Model

Each wheel is a child entity of the chassis, carrying a `WheelSuspension` component with
configurable rest length, spring stiffness, damping coefficient, and travel limits.
`SuspensionSystem` queries all `(WheelSuspension, Transform)` pairs, casts a ray downward from each
wheel entity to detect ground contact, and writes suspension compression and contact normal back
into the `WheelSuspension` component. The chassis entity carries a `Vehicle` marker component that
links it to its wheel children via the ECS parent-child hierarchy.

- **Requirements:** R-4.5.1
- **Dependencies:** F-1.1.1, F-4.1.1, F-4.4.1
- **Platform notes:** None

### F-4.5.2 Tire Friction Model

Each wheel entity carries a `TireFriction` component storing Pacejka Magic Formula coefficients and
a surface-type friction curve table. `TireForceSystem` queries all
`(TireFriction, WheelSuspension, Transform)` tuples, reads the contact normal and suspension
compression from `WheelSuspension`, and computes lateral and longitudinal forces from slip angle and
slip ratio. Surface type is determined from the collider material on the contacted entity. Computed
forces are written to a `WheelForceOutput` component on each wheel entity for downstream systems to
consume.

- **Requirements:** R-4.5.2
- **Dependencies:** F-1.1.1, F-4.5.1, F-4.1.3
- **Platform notes:** Mobile: simplified friction model (linear approximation instead of Pacejka).
  Switch: Pacejka with reduced curve table (4 surface types). Desktop: full Pacejka model with
  configurable surface tables.

## Drivetrain

### F-4.5.3 Drivetrain Simulation

The chassis entity carries a `Drivetrain` component storing engine torque curve, gear ratios,
current gear, differential type (open, limited-slip, locked), and drive layout (front, rear,
all-wheel). `DrivetrainSystem` queries all `(Drivetrain, Vehicle)` pairs, reads throttle and brake
input from the chassis entity, computes engine RPM and output torque, then distributes torque to
driven wheel entities by writing to their `WheelSuspension` drive-torque field. Gear shifting logic
runs within the same system based on configurable shift points in the `Drivetrain` component.

- **Requirements:** R-4.5.3
- **Dependencies:** F-1.1.1, F-4.5.1, F-4.5.2
- **Platform notes:** Mobile: max 4 simultaneous drivetrain simulations, simplified differential
  (open only). Switch: max 8 drivetrains. Desktop: max 64 drivetrains with full differential models.
  High-end PC: max 256 drivetrains.

## Stability

### F-4.5.4 Anti-Roll Bars and Stability Control

The chassis entity carries `AntiRollBar` and `StabilityControl` components. `AntiRollBarSystem`
queries `(AntiRollBar, Vehicle)`, iterates paired wheel entities via the parent-child hierarchy,
reads their `WheelSuspension` compression values, and applies a corrective force that transfers load
between paired wheels to resist body roll. `StabilityControlSystem` queries
`(StabilityControl, Drivetrain, Vehicle)` and applies traction control and electronic stability
corrections by modifying brake and throttle values on individual wheel entities. Both components
expose tuning parameters and can be disabled per entity.

- **Requirements:** R-4.5.4
- **Dependencies:** F-1.1.1, F-4.5.1, F-4.5.3
- **Platform notes:** Mobile: stability control only on player vehicle, anti-roll bars disabled for
  AI vehicles. Switch: full stability for up to 4 vehicles. Desktop: full stability for all active
  vehicles.

## Specialized Vehicles

### F-4.5.5 Tracked Vehicle Simulation

Tracked vehicles use a `TrackedVehicle` component on the chassis entity instead of wheel child
entities. The `TrackedVehicle` component stores track friction coefficients, track tension, and
per-side speed values. `TrackedVehicleSystem` queries `(TrackedVehicle, Transform)` and computes
ground contact via shape casts along each track span, calculates drive forces from differential
track speeds, and writes resulting linear and angular forces to the chassis entity. Steering is
achieved by varying left and right track speeds within the `TrackedVehicle` component.

- **Requirements:** R-4.5.5
- **Dependencies:** F-1.1.1, F-4.5.2, F-4.1.3
- **Platform notes:** Mobile: max 2 tracked vehicles, 4 shape casts per track. Switch: max 4 tracked
  vehicles, 6 casts. Desktop: max 32 tracked vehicles, 8 casts per track. Ground sampling density
  reduced on mobile to lower per-frame cast count.

### F-4.5.6 Hover Vehicle Simulation

Hover vehicles use `HoverRepulsor` components on child entities positioned at repulsion points
beneath the chassis. Each `HoverRepulsor` stores height-dependent force falloff, maximum hover
height, and lateral friction coefficients. `HoverRepulsorSystem` queries all
`(HoverRepulsor, Transform)` pairs, casts a ray downward from each repulsor entity, and computes a
repulsion force that is applied to the parent chassis entity. A `HoverStabilizationSystem` queries
the chassis `(Vehicle, Transform)` and applies tilt correction torques to keep the vehicle level
over terrain edges and water surfaces.

- **Requirements:** R-4.5.6
- **Dependencies:** F-1.1.1, F-4.4.1, F-4.1.1
- **Platform notes:** Mobile: max 2 hover vehicles, 4 repulsor points each. Switch: max 4 vehicles,
  6 repulsor points. Desktop: max 32 vehicles, 8 repulsor points. Fewer repulsors reduce per-frame
  ray cast count on constrained platforms.

### F-4.5.7 Server-Authoritative Vehicle Replication

Vehicle state replication uses the ECS state replication system to synchronize vehicle components.
The server runs authoritative `SuspensionSystem`, `TireForceSystem`, and `DrivetrainSystem` and
marks `WheelSuspension`, `Drivetrain`, and `Vehicle` components as replicated. Clients predict
locally using the same ECS systems and reconcile on server snapshots. Input compression, snapshot
interpolation, and jitter buffering operate on the replicated component data. Wheel positions,
suspension compression, and drivetrain RPM are all standard ECS component fields, requiring no
special serialization.

- **Requirements:** R-4.5.7
- **Dependencies:** F-1.1.1, F-4.5.1, F-4.5.3, F-4.1.1
- **Platform notes:** None
