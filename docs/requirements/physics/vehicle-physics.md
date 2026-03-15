# R-4.5 — Vehicle Physics User Stories

## F-4.5.1 Wheel Suspension Model

## US-4.5.1.1 Set Up Wheel Suspension Components

**As a** game developer (P-15), **I want to** create wheel child entities with
`WheelSuspension` components specifying rest length, stiffness, damping, and travel limits,
**so that** vehicles have configurable suspension through ECS composition.

## US-4.5.1.2 Configure Suspension Parameters in Editor

**As a** designer (P-5), **I want to** tune spring stiffness, damping, rest length, and travel
limits on each wheel in the editor, **so that** vehicle handling is adjusted visually.

## US-4.5.1.3 Verify Suspension Settles to Rest Length

**As an** engine tester (P-27), **I want to** place a 1500 kg vehicle on flat ground and assert
each wheel's compression converges to within 1mm of the analytical rest within 2 seconds,
**so that** suspension convergence is accurate.

## US-4.5.1.4 Benchmark Suspension System Cost

**As an** engine tester (P-27), **I want to** benchmark suspension for 20 vehicles (4 wheels
each) and assert total system time stays within 4ms, **so that** suspension fits within the
vehicle budget.

## US-4.5.1.5 Implement Suspension Ray Cast System

**As an** engine developer (P-26), **I want to** implement `SuspensionSystem` that casts rays
downward from wheel entities, computes spring forces from compression, and writes results to
`WheelSuspension`, **so that** suspension is fully ECS-driven.

## US-4.5.1.6 Experience Vehicles with Responsive Suspension

**As a** player (P-23), **I want** vehicles to bounce over bumps and settle smoothly on flat
ground, **so that** driving feels physically grounded.

---

## F-4.5.2 Tire Friction Model

## US-4.5.2.1 Configure Pacejka Coefficients

**As a** game developer (P-15), **I want to** set Pacejka Magic Formula coefficients on each
wheel's `TireFriction` component, **so that** tire grip behavior matches the desired handling
profile.

## US-4.5.2.2 Configure Surface Friction Tables

**As a** designer (P-5), **I want to** define per-surface friction curves (asphalt, dirt, ice)
in the editor, **so that** tire grip varies by terrain type.

## US-4.5.2.3 Verify Pacejka Force Curve Accuracy

**As an** engine tester (P-27), **I want to** sweep slip angle from 0 to 20 degrees with known
coefficients and assert lateral force values differ from reference by less than 5%, **so that**
Pacejka implementation accuracy is confirmed.

## US-4.5.2.4 Verify Surface-Dependent Friction

**As an** engine tester (P-27), **I want to** drive a vehicle from asphalt onto ice and verify
reduced grip, **so that** surface-dependent friction is functioning.

## US-4.5.2.5 Implement Tire Force Computation System

**As an** engine developer (P-26), **I want to** implement `TireForceSystem` that computes
lateral and longitudinal forces from slip angle and ratio using Pacejka, writing results to
`WheelForceOutput`, **so that** tire physics runs through ECS.

## US-4.5.2.6 Experience Different Grip on Different Surfaces

**As a** player (P-23), **I want** tires to grip well on dry road and slide on wet or icy
surfaces, **so that** driving feels responsive to terrain.

## US-4.5.2.7 Set Up Surface Types for Race Tracks

**As a** level designer (P-6), **I want to** assign surface types (asphalt, gravel, grass) to
track sections, **so that** vehicles have varied grip throughout the course.

---

## F-4.5.3 Drivetrain Simulation

## US-4.5.3.1 Configure Drivetrain Parameters

**As a** game developer (P-15), **I want to** set engine torque curve, gear ratios, differential
type, and drive layout on the `Drivetrain` component, **so that** vehicle powertrain behavior
is data-driven.

## US-4.5.3.2 Configure Gear Ratios and Shift Points

**As a** designer (P-5), **I want to** tune gear ratios and shift points in the editor,
**so that** vehicle acceleration and top speed are balanced per vehicle.

## US-4.5.3.3 Verify Torque Distribution Accuracy

**As an** engine tester (P-27), **I want to** apply full throttle to a rear-wheel-drive vehicle
and assert driven wheel torque sums to engine torque times gear ratio within 1%, **so that**
torque distribution is accurate.

## US-4.5.3.4 Verify Differential Types

**As an** engine tester (P-27), **I want to** test open, limited-slip, and locked differentials
and verify correct torque split between driven wheels, **so that** each differential type
behaves correctly.

## US-4.5.3.5 Implement Drivetrain System

**As an** engine developer (P-26), **I want to** implement `DrivetrainSystem` that computes
engine RPM, output torque, and distributes torque to driven wheels based on differential type
and gear ratio, **so that** powertrain simulation is complete.

## US-4.5.3.6 Experience Vehicles Accelerating Through Gears

**As a** player (P-23), **I want** vehicles to accelerate through gears with rising engine
pitch and shifting pauses, **so that** driving feels mechanically authentic.

---

## F-4.5.4 Anti-Roll Bars and Stability Control

## US-4.5.4.1 Configure Anti-Roll Bars

**As a** game developer (P-15), **I want to** add `AntiRollBar` components to vehicle entities
linking paired wheels, **so that** body roll during cornering is reduced.

## US-4.5.4.2 Configure Stability Control Parameters

**As a** designer (P-5), **I want to** tune traction control and stability control strength in
the editor, **so that** vehicle handling stability is adjustable.

## US-4.5.4.3 Verify Anti-Roll Bar Effectiveness

**As an** engine tester (P-27), **I want to** corner at 0.5g with and without anti-roll bars
and assert at least 30% less roll with bars enabled, **so that** anti-roll effectiveness is
quantified.

## US-4.5.4.4 Implement Anti-Roll and Stability Systems

**As an** engine developer (P-26), **I want to** implement `AntiRollBarSystem` and
`StabilityControlSystem` that correct roll and apply traction control, **so that** vehicle
stability aids are available.

## US-4.5.4.5 Experience Stable Vehicle Handling

**As a** player (P-23), **I want** vehicles to corner without tipping over unpredictably,
**so that** driving feels controllable.

---

## F-4.5.5 Tracked Vehicle Simulation

## US-4.5.5.1 Set Up Tracked Vehicle Components

**As a** game developer (P-15), **I want to** add a `TrackedVehicle` component to a chassis
entity with track friction, tension, and per-side speed values, **so that** tanks and
construction vehicles steer via differential track speeds.

## US-4.5.5.2 Configure Track Parameters in Editor

**As a** designer (P-5), **I want to** tune track friction and tension in the editor, **so
that** tracked vehicle handling is adjustable.

## US-4.5.5.3 Verify Turning Radius Accuracy

**As an** engine tester (P-27), **I want to** set left track to 5 m/s and right to 3 m/s,
simulate for 10 seconds, and assert the turning radius is within 10% of the expected value,
**so that** tracked steering accuracy is confirmed.

## US-4.5.5.4 Implement Tracked Vehicle System

**As an** engine developer (P-26), **I want to** implement `TrackedVehicleSystem` that computes
ground contact via shape casts and drives forces from differential track speeds, **so that**
tracked vehicle physics is fully ECS-based.

## US-4.5.5.5 Experience Driving Tanks and Treaded Vehicles

**As a** player (P-23), **I want** tanks to pivot-steer by running tracks at different speeds,
**so that** tracked vehicle control feels authentic.

## US-4.5.5.6 Place Tracked Vehicles in Levels

**As a** level designer (P-6), **I want to** place tank and bulldozer entities with configured
track parameters, **so that** tracked vehicles are part of the level without code.

---

## F-4.5.6 Hover Vehicle Simulation

## US-4.5.6.1 Set Up Hover Repulsor Components

**As a** game developer (P-15), **I want to** add `HoverRepulsor` child entities with
configurable hover height, force falloff, and lateral friction, **so that** hover vehicles
float above the ground.

## US-4.5.6.2 Configure Hover Parameters in Editor

**As a** designer (P-5), **I want to** tune hover height, repulsion force, and stabilization
in the editor, **so that** hover vehicle feel is adjustable.

## US-4.5.6.3 Verify Stable Hover Altitude

**As an** engine tester (P-27), **I want to** place a hover vehicle with 4 repulsors over flat
ground at 2m target height, simulate 5 seconds, and assert height stabilizes within 5% of 2m
with vertical velocity below 0.01 m/s, **so that** hover equilibrium is confirmed.

## US-4.5.6.4 Implement Hover Repulsor System

**As an** engine developer (P-26), **I want to** implement `HoverRepulsorSystem` that casts
rays from repulsors and computes height-dependent forces applied to the chassis, **so that**
hover physics is ECS-driven.

## US-4.5.6.5 Implement Hover Stabilization

**As an** engine developer (P-26), **I want to** implement `HoverStabilizationSystem` that
applies tilt correction torques to keep the vehicle level over terrain edges, **so that**
hover vehicles remain stable on uneven ground.

## US-4.5.6.6 Experience Hovering Over Terrain

**As a** player (P-23), **I want** hover vehicles to float smoothly above the ground and
stabilize over terrain edges, **so that** hover driving feels unique and fun.

---

## F-4.5.7 Server-Authoritative Vehicle Replication

## US-4.5.7.1 Mark Vehicle Components as Replicated

**As a** game developer (P-15), **I want to** mark `WheelSuspension`, `Drivetrain`, and
`Vehicle` components as replicated, **so that** vehicle state synchronizes automatically
via the ECS replication system.

## US-4.5.7.2 Verify Client Prediction Accuracy

**As an** engine tester (P-27), **I want to** run server and client simulations with 100ms
latency and assert position divergence stays below 10cm after reconciliation, **so that**
vehicle prediction accuracy meets requirements.

## US-4.5.7.3 Verify Replication Bandwidth

**As an** engine tester (P-27), **I want to** capture 100 consecutive vehicle snapshots during
active driving and assert average delta-compressed payload is below 256 bytes per vehicle,
**so that** bandwidth usage meets the requirement.

## US-4.5.7.4 Implement Vehicle State Replication

**As an** engine developer (P-26), **I want to** implement vehicle state replication using ECS
component replication with delta compression for suspension, drivetrain, and wheel data,
**so that** multiplayer vehicles use the standard replication pipeline.

## US-4.5.7.5 Experience Smooth Multiplayer Vehicle Driving

**As a** player (P-23), **I want** other players' vehicles to move smoothly without
teleporting, **so that** multiplayer vehicle gameplay feels seamless.

## US-4.5.7.6 Configure Replication Priority for Vehicles

**As a** designer (P-5), **I want to** set replication priority on vehicle entities, **so
that** nearby player vehicles get more frequent updates than distant ones.

---

## Non-Functional Requirements

### R-4.5.NF1 Vehicle Simulation Frame Budget

A single vehicle's full simulation pipeline (suspension, tire forces, drivetrain, stability
control) **SHALL** complete within 0.2 ms on minimum-spec hardware, supporting at least 20
simultaneous active vehicles within the 4 ms physics budget.

- **Derived from:** R-4.5.1, R-4.5.2, R-4.5.3, R-4.5.4
- **Rationale:** Racing games and open-world scenarios with vehicle traffic require many
  simultaneous vehicles without exceeding the physics frame budget.
- **Verification:** Benchmark -- simulate 20 vehicles (4 wheels each) driving on terrain;
  measure total vehicle system time per frame; assert it completes within 4 ms.

### R-4.5.NF2 Vehicle Replication Bandwidth

Replicated vehicle state (suspension compression, drivetrain RPM, wheel positions) **SHALL
NOT** exceed 256 bytes per vehicle per network snapshot, after delta compression.

- **Derived from:** R-4.5.7
- **Rationale:** Bandwidth is constrained in multiplayer; vehicle state must compress
  efficiently to support many replicated vehicles without saturating the connection.
- **Verification:** Profile -- capture 100 consecutive vehicle snapshots during active
  driving; measure average delta-compressed payload size; assert it does not exceed 256
  bytes per vehicle per snapshot.
