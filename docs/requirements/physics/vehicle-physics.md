# R-4.5 — Vehicle Physics Requirements

## R-4.5.1 Wheel Suspension Model

The engine **SHALL** simulate wheel suspension as child entities with `WheelSuspension`
components that produce spring forces proportional to compression, and the suspension system
shall settle a stationary vehicle to within 1 mm of the analytical rest length within 2 seconds
of simulation time.

- **Derived from:** [F-4.5.1](../../features/physics/vehicle-physics.md)
- **Rationale:** Suspension is the foundation of all wheeled vehicle behavior; it must converge
  to a stable rest state without oscillation or drift.
- **Verification:** Unit test — place a 1500 kg vehicle on flat ground with known spring
  stiffness and damping; simulate for 2 seconds; assert each wheel's compression converges to
  within 1 mm of the analytically computed rest compression.

## R-4.5.2 Tire Friction Model

The engine **SHALL** compute lateral and longitudinal tire forces from slip angle and slip ratio
using the Pacejka Magic Formula, producing force curves that peak within 5% of the reference
Pacejka output for the given coefficients.

- **Derived from:** [F-4.5.2](../../features/physics/vehicle-physics.md)
- **Rationale:** Realistic tire forces govern handling feel; deviations from the expected
  friction curve produce unpredictable or unrealistic cornering and braking behavior.
- **Verification:** Unit test — provide known Pacejka coefficients and sweep slip angle from
  0 to 20 degrees; compare computed lateral force at each degree against the reference formula
  output; assert all values differ by less than 5%.

## R-4.5.3 Drivetrain Simulation

The engine **SHALL** distribute engine torque to driven wheels according to the configured
differential type (open, limited-slip, locked) and gear ratio, such that total torque delivered
to wheels equals engine output torque multiplied by the current gear ratio within 1% error.

- **Derived from:** [F-4.5.3](../../features/physics/vehicle-physics.md)
- **Rationale:** Correct torque distribution determines acceleration, top speed, and traction
  balance, which are core to vehicle gameplay feel.
- **Verification:** Unit test — configure a rear-wheel-drive vehicle with a known torque curve
  and gear ratio; apply full throttle; measure torque written to each driven wheel's
  `WheelSuspension`; assert the sum equals engine torque times gear ratio within 1%.

## R-4.5.4 Anti-Roll Bars and Stability Control

The engine **SHALL** reduce body roll angle by at least 30% when `AntiRollBar` is enabled on a
vehicle cornering at a fixed lateral acceleration, compared to the same vehicle without
anti-roll bars.

- **Derived from:** [F-4.5.4](../../features/physics/vehicle-physics.md)
- **Rationale:** Anti-roll bars and stability control are essential tuning tools for preventing
  rollovers and maintaining predictable handling.
- **Verification:** Benchmark — simulate a vehicle cornering at 0.5g lateral acceleration with
  and without `AntiRollBar` enabled; measure peak chassis roll angle in both cases; assert the
  anti-roll configuration produces at least 30% less roll.

## R-4.5.5 Tracked Vehicle Simulation

The engine **SHALL** simulate tracked vehicle steering by varying left and right track speeds
in the `TrackedVehicle` component, producing a turning radius that is within 10% of the
analytically expected radius for the given track speed differential and vehicle geometry.

- **Derived from:** [F-4.5.5](../../features/physics/vehicle-physics.md)
- **Rationale:** Tanks and construction vehicles steer via differential track speeds; accurate
  turning radius is fundamental to navigation and gameplay.
- **Verification:** Unit test — set left track speed to 5 m/s and right track speed to 3 m/s
  on a vehicle with known track width; simulate for 10 seconds; measure the actual turning
  radius from the trajectory; assert it is within 10% of the expected value.

## R-4.5.6 Hover Vehicle Simulation

The engine **SHALL** maintain a hover vehicle at a stable altitude within 5% of the configured
maximum hover height when stationary over flat terrain, using `HoverRepulsor` components that
cast rays downward and compute height-dependent repulsion forces.

- **Derived from:** [F-4.5.6](../../features/physics/vehicle-physics.md)
- **Rationale:** Hover vehicles must reach a stable equilibrium altitude dictated by repulsor
  configuration without oscillation or ground collision.
- **Verification:** Unit test — place a hover vehicle with 4 repulsors over flat ground with a
  2 m target hover height; simulate for 5 seconds; assert the chassis height stabilizes within
  5% of 2 m and vertical velocity drops below 0.01 m/s.

## R-4.5.7 Server-Authoritative Vehicle Replication

The engine **SHALL** replicate vehicle ECS components (`WheelSuspension`, `Drivetrain`,
`Vehicle`) via the ECS state replication system such that client-predicted vehicle position
diverges by less than 10 cm from the server-authoritative position under 100 ms simulated
network latency.

- **Derived from:** [F-4.5.7](../../features/physics/vehicle-physics.md)
- **Rationale:** Multiplayer vehicles must feel responsive locally while remaining consistent
  with the server-authoritative simulation.
- **Verification:** Integration test — run server and client simulations with 100 ms artificial
  latency; drive a vehicle in a straight line for 5 seconds; measure position divergence at
  each server snapshot; assert maximum divergence is below 10 cm after reconciliation.

---

## Non-Functional Requirements

### R-4.5.NF1 Vehicle Simulation Frame Budget

A single vehicle's full simulation pipeline (suspension, tire forces, drivetrain, stability
control) **SHALL** complete within 0.2 ms on minimum-spec hardware, supporting at least 20
simultaneous active vehicles within the 4 ms physics budget.

- **Derived from:** R-4.5.1, R-4.5.2, R-4.5.3, R-4.5.4
- **Rationale:** Racing games and open-world scenarios with vehicle traffic require many
  simultaneous vehicles without exceeding the physics frame budget.
- **Verification:** Benchmark — simulate 20 vehicles (4 wheels each) driving on terrain;
  measure total vehicle system time per frame; assert it completes within 4 ms.

### R-4.5.NF2 Vehicle Replication Bandwidth

Replicated vehicle state (suspension compression, drivetrain RPM, wheel positions) **SHALL
NOT** exceed 256 bytes per vehicle per network snapshot, after delta compression.

- **Derived from:** R-4.5.7
- **Rationale:** Bandwidth is constrained in multiplayer; vehicle state must compress
  efficiently to support many replicated vehicles without saturating the connection.
- **Verification:** Profile — capture 100 consecutive vehicle snapshots during active driving;
  measure average delta-compressed payload size; assert it does not exceed 256 bytes per
  vehicle per snapshot.
