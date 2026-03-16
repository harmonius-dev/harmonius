# User Stories — 4.5 Vehicle Physics

## US-4.5.1.1 Configure Suspension Parameters Per Wheel

**As a** designer (P-5), **I want to** configure spring stiffness, damping, and travel limits on
each WheelSuspension component, **so that** vehicle ride quality is tunable per wheel.

## US-4.5.1.2 Implement SuspensionSystem with Downward Raycasts

**As an** engine developer (P-26), **I want to** implement SuspensionSystem that casts a ray
downward from each wheel to detect ground contact, **so that** suspension compression is calculated
from terrain contact.

## US-4.5.1.3 Link Wheels as Child Entities of Chassis

**As a** game developer (P-15), **I want** each wheel to be a child entity of the chassis with a
WheelSuspension component, **so that** vehicle hierarchy uses ECS parent-child relationships.

## US-4.5.1.4 See Vehicle Bounce Over Bumps

**As a** player (P-23), **I want** vehicles to bounce realistically over bumps, **so that**
suspension feels dynamic.

## US-4.5.1.5 Verify Suspension Rest Length Behavior

**As an** engine tester (P-27), **I want to** verify suspension at rest holds the chassis at the
configured rest length, **so that** ride height is correct.

## US-4.5.1.6 Write Compression and Normal to Component

**As an** engine developer (P-26), **I want** SuspensionSystem to write compression and contact
normal back to WheelSuspension, **so that** downstream systems consume accurate data.

## US-4.5.1.7 Set Rest Length for Vehicle Height

**As a** designer (P-5), **I want to** set suspension rest length per wheel, **so that** I can
configure vehicle ride height visually.

## US-4.5.1.8 Test Suspension Under Extreme Loads

**As an** engine tester (P-27), **I want to** test suspension behavior under extreme loads (heavy
cargo, jumps), **so that** travel limits prevent unrealistic compression.

## US-4.5.1.9 Place Varied Vehicles in Levels

**As a** level designer (P-6), **I want** vehicles with different suspension setups to work
correctly on varied terrain, **so that** I can populate levels with diverse vehicles.

## US-4.5.1.10 Mark Chassis with Vehicle Component

**As a** game developer (P-15), **I want to** add a Vehicle marker component to the chassis entity,
**so that** systems identify vehicle entities.

## US-4.5.1.11 Experience Smooth Ride on Even Terrain

**As a** player (P-23), **I want** vehicles to ride smoothly on flat roads, **so that** normal
driving feels comfortable.

## US-4.5.1.12 Verify Damping Prevents Oscillation

**As an** engine tester (P-27), **I want to** verify that proper damping prevents suspension
oscillation after bumps, **so that** vehicles settle quickly.

## US-4.5.2.1 Configure Pacejka Tire Model Coefficients

**As a** designer (P-5), **I want to** configure Pacejka Magic Formula coefficients per tire,
**so that** tire grip characteristics are tunable.

## US-4.5.2.2 Implement TireForceSystem

**As an** engine developer (P-26), **I want to** implement TireForceSystem that computes lateral and
longitudinal forces from slip angle and slip ratio, **so that** tire physics is accurate.

## US-4.5.2.3 Read Surface Type from Contacted Collider

**As an** engine developer (P-26), **I want** TireForceSystem to read surface type from the
contacted entity's PhysicsMaterial, **so that** grip varies by surface.

## US-4.5.2.4 Feel Different Grip on Road vs Gravel

**As a** player (P-23), **I want** tires to grip differently on asphalt versus gravel, **so that**
surface type affects driving.

## US-4.5.2.5 Configure Surface Friction Curve Table

**As a** designer (P-5), **I want to** define surface-type friction curve tables, **so that** each
surface type has distinct tire behavior.

## US-4.5.2.6 Write Forces to WheelForceOutput

**As an** engine developer (P-26), **I want** computed forces written to a WheelForceOutput
component, **so that** downstream drivetrain and stability systems consume them.

## US-4.5.2.7 Verify Simplified Model on Mobile

**As an** engine tester (P-27), **I want to** confirm mobile uses linear approximation instead of
Pacejka, **so that** CPU budget is respected.

## US-4.5.2.8 Test Tire Behavior at Slip Limit

**As an** engine tester (P-27), **I want to** test tire forces at extreme slip angles, **so that**
behavior is stable at the grip limit.

## US-4.5.2.9 Experience Realistic Oversteer and Understeer

**As a** player (P-23), **I want** cars to oversteer and understeer based on speed and steering,
**so that** driving feels realistic.

## US-4.5.2.10 Design Racing Tracks with Surface Variety

**As a** level designer (P-6), **I want** tracks with asphalt, gravel, grass, and mud surfaces,
**so that** tire grip variation creates interesting driving challenges.

## US-4.5.2.11 Test Reduced Curve Table on Switch

**As an** engine tester (P-27), **I want to** verify Switch uses Pacejka with 4 surface types,
**so that** memory and CPU are within budget.

## US-4.5.2.12 Store TireFriction on Each Wheel Entity

**As a** game developer (P-15), **I want** each wheel entity to carry a TireFriction component,
**so that** per-wheel tire properties are configurable.

## US-4.5.3.1 Configure Engine Torque Curve and Gear Ratios

**As a** designer (P-5), **I want to** configure engine torque curve, gear ratios, and differential
type in the Drivetrain component, **so that** vehicle power delivery is tunable.

## US-4.5.3.2 Implement DrivetrainSystem

**As an** engine developer (P-26), **I want to** implement DrivetrainSystem that reads throttle and
brake input, computes RPM and torque, and distributes to driven wheels, **so that** the powertrain
simulation works.

## US-4.5.3.3 Distribute Torque by Drive Layout

**As an** engine developer (P-26), **I want** torque distributed to driven wheels by drive layout
(front, rear, AWD), **so that** different drivetrain configurations behave correctly.

## US-4.5.3.4 Shift Gears Based on RPM Thresholds

**As an** engine developer (P-26), **I want** gear shifting logic within DrivetrainSystem based on
configurable shift points, **so that** automatic shifting works smoothly.

## US-4.5.3.5 Feel Power Difference Between Vehicles

**As a** player (P-23), **I want** different vehicles to have distinct acceleration and top speed,
**so that** vehicle variety feels meaningful.

## US-4.5.3.6 Verify Drivetrain Limits on Mobile

**As an** engine tester (P-27), **I want to** confirm mobile caps at 4 simultaneous drivetrain
simulations with open differential only, **so that** CPU budget is respected.

## US-4.5.3.7 Configure Differential Type

**As a** designer (P-5), **I want to** choose differential type (open, limited-slip, locked) per
vehicle, **so that** power distribution suits each vehicle class.

## US-4.5.3.8 Hear Engine RPM Correspond to Speed

**As a** player (P-23), **I want** engine sound to correspond to RPM and gear, **so that** the audio
matches driving physics.

## US-4.5.3.9 Test Gear Shifting Under Load

**As an** engine tester (P-27), **I want to** test gear shifting under uphill and downhill
conditions, **so that** shifting logic handles varied loads.

## US-4.5.3.10 Design Vehicles with Different Powertrains

**As a** level designer (P-6), **I want** different vehicles (trucks, sports cars, buggies) to have
distinct drivetrain configurations, **so that** vehicle diversity is meaningful.

## US-4.5.3.11 Support 256 Drivetrains on High-End PC

**As an** engine developer (P-26), **I want** high-end PC to support 256 simultaneous drivetrain
simulations, **so that** large vehicle populations are feasible.

## US-4.5.3.12 Test Full Differential Models on Desktop

**As an** engine tester (P-27), **I want to** test open, limited-slip, and locked differentials on
desktop, **so that** all differential types are validated.

## US-4.5.4.1 Configure Anti-Roll Bar Stiffness

**As a** designer (P-5), **I want to** configure anti-roll bar stiffness per axle, **so that** body
roll is controlled during cornering.

## US-4.5.4.2 Implement AntiRollBarSystem

**As an** engine developer (P-26), **I want to** implement AntiRollBarSystem that reads paired wheel
suspension compression and applies corrective forces, **so that** body roll is resisted.

## US-4.5.4.3 Implement StabilityControlSystem

**As an** engine developer (P-26), **I want to** implement StabilityControlSystem applying traction
control and electronic stability corrections, **so that** vehicles remain drivable.

## US-4.5.4.4 Corner Without Tipping Over

**As a** player (P-23), **I want** vehicles to corner without tipping over, **so that** driving at
speed feels controllable.

## US-4.5.4.5 Verify Stability Only on Player Vehicle on Mobile

**As an** engine tester (P-27), **I want to** confirm mobile enables stability control only on the
player vehicle, **so that** CPU budget is respected.

## US-4.5.4.6 Disable Stability Aids Per Vehicle

**As a** designer (P-5), **I want to** disable anti-roll bars and stability control per entity,
**so that** vehicles can have deliberately loose handling.

## US-4.5.4.7 Test Anti-Roll Bar on Uneven Terrain

**As an** engine tester (P-27), **I want to** test anti-roll bar behavior on uneven terrain,
**so that** load transfer between paired wheels works correctly.

## US-4.5.4.8 Design Vehicles with Varied Stability

**As a** level designer (P-6), **I want** some vehicles to have high stability (tanks) and others
low (motorcycles), **so that** handling variety adds gameplay depth.

## US-4.5.4.9 Experience Traction Control on Ice

**As a** player (P-23), **I want** traction control to prevent wheel spin on icy surfaces,
**so that** vehicles remain drivable on slippery terrain.

## US-4.5.4.10 Modify Brake and Throttle Per Wheel

**As an** engine developer (P-26), **I want** StabilityControlSystem to modify brake and throttle
values on individual wheel entities, **so that** stability corrections are precise.

## US-4.5.4.11 Support Full Stability for All Vehicles on Desktop

**As an** engine developer (P-26), **I want** desktop to run full stability simulation for all
active vehicles, **so that** AI vehicles also have realistic handling.

## US-4.5.4.12 Test Stability Under Emergency Maneuvers

**As an** engine tester (P-27), **I want to** test stability control during emergency swerves and
braking, **so that** corrections prevent spin-outs.

## US-4.5.5.1 Create Tracked Vehicle Entities

**As a** game developer (P-15), **I want to** add a TrackedVehicle component to chassis entities
instead of wheel children, **so that** tanks and bulldozers work.

## US-4.5.5.2 Implement TrackedVehicleSystem

**As an** engine developer (P-26), **I want to** implement TrackedVehicleSystem that computes ground
contact via shape casts along each track span, **so that** tracked vehicles have accurate ground
interaction.

## US-4.5.5.3 Steer by Varying Track Speeds

**As a** game developer (P-15), **I want** steering achieved by varying left and right track speeds,
**so that** tracked vehicle turning works like real tanks.

## US-4.5.5.4 Drive Tank with Dual-Stick Controls

**As a** player (P-23), **I want to** steer a tank using differential track speeds, **so that**
driving feels authentic.

## US-4.5.5.5 Verify Tracked Vehicle Limits on Mobile

**As an** engine tester (P-27), **I want to** confirm mobile caps at 2 tracked vehicles with 4 shape
casts per track, **so that** performance is controlled.

## US-4.5.5.6 Configure Track Friction and Tension

**As a** designer (P-5), **I want to** configure track friction coefficients and tension per
vehicle, **so that** tracked vehicle feel is tunable.

## US-4.5.5.7 Design Levels with Tracked Vehicles

**As a** level designer (P-6), **I want** tracked vehicles (tanks, excavators) to traverse terrain
correctly, **so that** I can populate levels with heavy machinery.

## US-4.5.5.8 Test Tracked Vehicle on Slopes

**As an** engine tester (P-27), **I want to** test tracked vehicles on slopes and rough terrain,
**so that** ground contact and steering remain stable.

## US-4.5.5.9 Compute Drive Forces from Differential Speeds

**As an** engine developer (P-26), **I want** drive forces calculated from differential track
speeds, **so that** linear and angular forces on the chassis are correct.

## US-4.5.5.10 See Tank Tracks Grip Terrain

**As a** player (P-23), **I want** tank tracks to grip terrain convincingly, **so that** heavy
vehicles feel grounded.

## US-4.5.5.11 Support 32 Tracked Vehicles on Desktop

**As an** engine developer (P-26), **I want** desktop to support 32 tracked vehicles with 8 casts
per track, **so that** large military scenarios work.

## US-4.5.5.12 Test Ground Sampling Density Per Platform

**As an** engine tester (P-27), **I want to** verify ground sampling density scales per platform,
**so that** cast count matches device capability.

## US-4.5.6.1 Create Hover Vehicle with Repulsor Points

**As a** game developer (P-15), **I want to** create hover vehicles with HoverRepulsor components on
child entities, **so that** hovering mechanics work.

## US-4.5.6.2 Implement HoverRepulsorSystem

**As an** engine developer (P-26), **I want to** implement HoverRepulsorSystem that casts rays
downward from each repulsor and computes repulsion force, **so that** hover vehicles float.

## US-4.5.6.3 Implement HoverStabilizationSystem

**As an** engine developer (P-26), **I want to** implement HoverStabilizationSystem that applies
tilt correction torques, **so that** hover vehicles stay level over terrain edges.

## US-4.5.6.4 Fly Hover Vehicle Over Terrain

**As a** player (P-23), **I want** hover vehicles to glide smoothly over terrain, **so that** flying
feels fluid and responsive.

## US-4.5.6.5 Configure Hover Height and Force Falloff

**As a** designer (P-5), **I want to** set maximum hover height, force falloff, and lateral friction
per repulsor, **so that** hover feel is tunable.

## US-4.5.6.6 Verify Hover Vehicle Limits on Mobile

**As an** engine tester (P-27), **I want to** confirm mobile caps at 2 hover vehicles with 4
repulsor points each, **so that** ray cast count stays within budget.

## US-4.5.6.7 Design Hover Vehicle Races

**As a** level designer (P-6), **I want** hover vehicles to work correctly over water and varied
terrain, **so that** racing tracks are possible.

## US-4.5.6.8 Test Hover Over Terrain Edges

**As an** engine tester (P-27), **I want to** test hover vehicles over terrain edges and cliffs,
**so that** stabilization prevents flipping.

## US-4.5.6.9 Support 32 Hover Vehicles on Desktop

**As an** engine developer (P-26), **I want** desktop to support 32 hover vehicles with 8 repulsor
points each, **so that** hover vehicle fleets are feasible.

## US-4.5.6.10 Configure Repulsor Layout Per Vehicle

**As a** game developer (P-15), **I want to** position repulsor child entities to define the hover
vehicle's float geometry, **so that** each vehicle has unique flight characteristics.

## US-4.5.6.11 Experience Stable Hovering Over Water

**As a** player (P-23), **I want** hover vehicles to float stably over water surfaces, **so that**
amphibious gameplay works.

## US-4.5.6.12 Test Tilt Correction Under Wind Forces

**As an** engine tester (P-27), **I want to** test hover stabilization under wind forces,
**so that** hover vehicles remain level in stormy conditions.

## US-4.5.7.1 Replicate Vehicle State via ECS Components

**As a** game developer (P-15), **I want** vehicle components (WheelSuspension, Drivetrain, Vehicle)
replicated via ECS state replication, **so that** multiplayer vehicles are synchronized.

## US-4.5.7.2 Run Authoritative Vehicle Systems on Server

**As an** engine developer (P-26), **I want** the server to run authoritative SuspensionSystem,
TireForceSystem, and DrivetrainSystem, **so that** vehicle physics is server-authoritative.

## US-4.5.7.3 Predict Locally and Reconcile on Server Snapshots

**As an** engine developer (P-26), **I want** clients to predict locally using the same ECS systems
and reconcile on server snapshots, **so that** vehicle control feels responsive.

## US-4.5.7.4 See Other Players' Vehicles Move Smoothly

**As a** player (P-23), **I want** other players' vehicles to move smoothly without teleporting,
**so that** multiplayer racing feels fair.

## US-4.5.7.5 Verify Standard Component Serialization

**As an** engine tester (P-27), **I want to** verify that wheel positions, suspension compression,
and RPM use standard ECS component serialization, **so that** no special serialization is needed.

## US-4.5.7.6 Apply Input Compression for Vehicle Controls

**As an** engine developer (P-26), **I want** vehicle input compression to minimize bandwidth,
**so that** steering, throttle, and brake transmit efficiently.

## US-4.5.7.7 Apply Jitter Buffering to Vehicle State

**As an** engine developer (P-26), **I want** jitter buffering on replicated vehicle state,
**so that** network jitter does not cause vehicle stutter.

## US-4.5.7.8 Test Vehicle Prediction Under Packet Loss

**As an** engine tester (P-27), **I want to** test vehicle prediction under 10% packet loss,
**so that** reconciliation handles network degradation gracefully.

## US-4.5.7.9 Configure Vehicle Replication Priority

**As a** designer (P-5), **I want to** set vehicle replication priority, **so that** player vehicles
have higher priority than distant AI vehicles.

## US-4.5.7.10 Race Against Other Players Online

**As a** player (P-23), **I want to** race against other players with consistent vehicle positions,
**so that** multiplayer racing is competitive and fair.

## US-4.5.7.11 Apply Snapshot Interpolation for Smooth Motion

**As an** engine developer (P-26), **I want** snapshot interpolation on vehicle components,
**so that** remote vehicles display smoothly between server updates.

## US-4.5.7.12 Verify Reconciliation After Large Desync

**As an** engine tester (P-27), **I want to** test reconciliation after a large prediction desync,
**so that** vehicles snap back correctly without visible artifacts.
