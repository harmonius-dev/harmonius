# R-4.5 — Vehicle Physics Requirements

## F-4.5.1 Wheel Suspension Model

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-4.5.1 | ECS Wheel Suspension Components: The engine **SHALL** support `WheelSuspension` components on wheel child entities with configurable rest length, spring stiffness, damping coefficient, and travel limits. The `SuspensionSystem` **SHALL** cast rays downward from wheel entities to detect ground contact and write compression and contact normal back into `WheelSuspension`. |  [F-4.5.1](../../features/physics/vehicle-physics.md) | Suspension is the foundation of vehicle handling; ECS components enable per-wheel tuning and standard query access. | Place a 1500 kg vehicle on flat ground. Assert each wheel's compression converges to within 1 mm of the analytical rest position within 2 seconds. |

## F-4.5.2 Tire Friction Model

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-4.5.2 | Pacejka Magic Formula Implementation: The engine **SHALL** compute lateral and longitudinal tire forces using the Pacejka Magic Formula via a `TireForceSystem` that reads `TireFriction` and `WheelSuspension` components. Computed forces **SHALL** be written to a `WheelForceOutput` component on each wheel. |  [F-4.5.2](../../features/physics/vehicle-physics.md) | Pacejka provides industry-standard tire force curves that enable realistic handling profiles. | Sweep slip angle from 0 to 20 degrees with known coefficients. Assert lateral force values differ from reference by less than 5%. |
| R-4.5.2a | Surface-Dependent Friction: Tire friction **SHALL** vary by surface type using per- surface friction curve tables stored in the `TireFriction` component. |  [F-4.5.2](../../features/physics/vehicle-physics.md) | Driving from asphalt onto ice must produce noticeably different grip to create varied handling. | Drive a vehicle from asphalt onto ice. Assert reduced lateral grip on the ice surface. |

## F-4.5.3 Drivetrain Simulation

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-4.5.3 | Drivetrain Component: The engine **SHALL** support a `Drivetrain` component storing engine torque curve, gear ratios, current gear, differential type (open, limited-slip, locked), and drive layout (front, rear, all-wheel). The `DrivetrainSystem` **SHALL** compute engine RPM and distribute torque to driven wheels. |  [F-4.5.3](../../features/physics/vehicle-physics.md) | Drivetrain simulation is essential for authentic vehicle acceleration, gear shifting, and torque distribution behavior. | Apply full throttle to a rear-wheel- drive vehicle. Assert driven wheel torque sums to engine torque times gear ratio within 1%. |
| R-4.5.3a | Differential Types: The engine **SHALL** support open, limited-slip, and locked differentials with correct torque split behavior. |  [F-4.5.3](../../features/physics/vehicle-physics.md) | Different differential types produce different handling characteristics; all three are needed for vehicle variety. | Test each differential type. Assert open splits torque equally, limited-slip transfers torque to the gripping wheel, and locked keeps wheels at the same speed. |

## F-4.5.4 Anti-Roll Bars and Stability Control

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-4.5.4 | Anti-Roll Bar System: The engine **SHALL** support `AntiRollBar` components that transfer load between paired wheels to resist body roll during cornering. |  [F-4.5.4](../../features/physics/vehicle-physics.md) | Uncontrolled body roll makes vehicles tip over during cornering; anti-roll bars are standard in vehicle simulation. | Corner at 0.5g with and without anti-roll bars. Assert at least 30% less roll with bars enabled. |
| R-4.5.4a | Stability Control System: The engine **SHALL** support `StabilityControl` components that apply traction control and electronic stability corrections by modifying brake and throttle values on individual wheels. |  [F-4.5.4](../../features/physics/vehicle-physics.md) | Stability aids prevent arcade-style spin- outs and make vehicles controllable for non-expert players. | Enable stability control. Induce a skid by oversteering. Assert the system reduces throttle or applies brake to the appropriate wheels. |

## F-4.5.5 Tracked Vehicle Simulation

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-4.5.5 | TrackedVehicle Component: The engine **SHALL** support a `TrackedVehicle` component storing track friction, tension, and per-side speed values. The `TrackedVehicleSystem` **SHALL** compute ground contact via shape casts and drive forces from differential track speeds. |  [F-4.5.5](../../features/physics/vehicle-physics.md) | Tanks, bulldozers, and construction vehicles steer by varying track speeds; this model is distinct from wheeled suspension. | Set left track to 5 m/s and right to 3 m/s. Simulate 10 seconds. Assert the turning radius is within 10% of the expected value. |

## F-4.5.6 Hover Vehicle Simulation

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-4.5.6 | Hover Repulsor System: The engine **SHALL** support `HoverRepulsor` child entities with configurable hover height, force falloff, and lateral friction. The `HoverRepulsorSystem` **SHALL** cast rays downward and compute height-dependent repulsion forces applied to the parent chassis entity. |  [F-4.5.6](../../features/physics/vehicle-physics.md) | Hover vehicles are a distinct vehicle archetype requiring ground-proximity force computation rather than wheel contact. | Place a hover vehicle with 4 repulsors at 2 m target height. Simulate 5 seconds. Assert height stabilizes within 5% of 2 m with vertical velocity below 0.01 m/s. |
| R-4.5.6a | Hover Stabilization: The engine **SHALL** apply tilt correction torques via a `HoverStabilizationSystem` to keep hover vehicles level over terrain edges and water surfaces. |  [F-4.5.6](../../features/physics/vehicle-physics.md) | Without tilt correction, hover vehicles flip on terrain edges, making them unusable. | Drive a hover vehicle over a terrain edge. Assert the vehicle remains within 15 degrees of level throughout the transition. |

## F-4.5.7 Server-Authoritative Vehicle Replication

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-4.5.7 | Vehicle State Replication: `WheelSuspension`, `Drivetrain`, and `Vehicle` components **SHALL** be replicable via the ECS state replication system. Clients **SHALL** predict locally and reconcile on server snapshots. |  [F-4.5.7](../../features/physics/vehicle-physics.md) | Multiplayer vehicles must synchronize state via standard ECS replication without custom serialization. | Run server and client with 100 ms latency. Assert position divergence stays below 10 cm after reconciliation. |
| R-4.5.7a | Vehicle Replication Bandwidth: Replicated vehicle state (suspension, drivetrain, wheel positions) **SHALL NOT** exceed 256 bytes per vehicle per network snapshot after delta compression. |  [F-4.5.7](../../features/physics/vehicle-physics.md) | Bandwidth is constrained in multiplayer; vehicle state must compress efficiently. | Capture 100 consecutive vehicle snapshots during active driving. Assert average delta-compressed payload is below 256 bytes per vehicle. |

## Non-Functional Requirements

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-4.5.NF1 | Vehicle Simulation Frame Budget: A single vehicle's full simulation pipeline (suspension, tire forces, drivetrain, stability control) **SHALL** complete within 0.2 ms on minimum-spec hardware, supporting at least 20 simultaneous active vehicles within the 4 ms physics budget. | R-4.5.1, R-4.5.2, R-4.5.3, R-4.5.4 | Racing games and open-world scenarios with vehicle traffic require many simultaneous vehicles without exceeding the physics frame budget. | Benchmark -- simulate 20 vehicles (4 wheels each) driving on terrain; measure total vehicle system time per frame; assert it completes within 4 ms. |
| R-4.5.NF2 | Vehicle Replication Bandwidth: Replicated vehicle state (suspension compression, drivetrain RPM, wheel positions) **SHALL NOT** exceed 256 bytes per vehicle per network snapshot, after delta compression. | R-4.5.7 | Bandwidth is constrained in multiplayer; vehicle state must compress efficiently to support many replicated vehicles without saturating the connection. | Profile -- capture 100 consecutive vehicle snapshots during active driving; measure average delta-compressed payload size; assert it does not exceed 256 bytes per vehicle per snapshot. |
