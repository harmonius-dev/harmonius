# R-4.1 -- Rigid Body Dynamics Requirements

1. **R-4.1.1** -- The engine **SHALL** integrate rigid body motion using symplectic Euler with a
   fixed-timestep accumulator, producing bit-identical results across platforms.
   - **Rationale:** Deterministic integration is required for server-authoritative reconciliation
     and rollback.
   - **Verification:** Run the same scenario twice. Assert bit-identical output. Run on two
     platforms. Assert matching results.

2. **R-4.1.2** -- The engine **SHALL** support configurable substep count via a PhysicsConfig
   resource and per-entity SubstepOverride components.
   - **Rationale:** Substeps trade accuracy for performance; per-entity overrides let high-fidelity
     objects use more substeps without global cost.
   - **Verification:** Set global substeps to 4. Override one entity to 8. Assert the entity
     receives 8 substeps.

3. **R-4.1.3** -- The engine **SHALL** resolve contacts using impulse-based resolution with
   PhysicsMaterial restitution and friction, combined via configurable rules.
   - **Rationale:** Material-driven resolution produces correct bouncing and sliding per surface.
   - **Verification:** Drop a sphere onto surfaces with different materials. Assert bounce height
     and slide distance match the configured values.

4. **R-4.1.4** -- The engine **SHALL** detect and resolve tunneling for CCD-flagged entities via
   swept-volume time-of-impact queries.
   - **Rationale:** Fast projectiles must not tunnel through thin geometry.
   - **Verification:** Fire a bullet at maximum speed through a thin wall. Assert a hit is detected.

5. **R-4.1.5** -- The engine **SHALL** partition bodies into simulation islands via union-find and
   solve independent islands in parallel.
   - **Rationale:** Island-parallel solving exploits multiple CPU cores without contention.
   - **Verification:** Create two isolated groups. Assert they are assigned different islands.
     Assert islands solve concurrently.

6. **R-4.1.6** -- The engine **SHALL** sleep bodies at rest and wake them on external force, torque,
   or new contact.
   - **Rationale:** Sleeping eliminates simulation cost for stationary bodies.
   - **Verification:** Let a body settle. Assert the Sleeping marker is added. Apply a force. Assert
     the marker is removed within one frame.

7. **R-4.1.7** -- The engine **SHALL** migrate physics entities across streaming zone boundaries
   preserving momentum and contact state.
   - **Rationale:** Seamless zone transitions prevent visible physics discontinuities in open
     worlds.
   - **Verification:** Push a body across a zone boundary. Assert velocity is preserved. Assert no
     visual hitch.

8. **R-4.1.8** -- The engine **SHALL** provide a kinematic character controller with ground
   detection, slope sliding, step climbing, moving platform attachment, and coyote-time support.
   - **Rationale:** A robust character controller is foundational for all ground-based gameplay.
   - **Verification:** Navigate stairs, slopes, and a moving platform. Assert stable movement with
     no jitter.

9. **R-4.1.9** -- The engine **SHALL** support moving platforms that transport characters and
   physics objects without sliding, with one-way platform filtering.
   - **Rationale:** Elevators, conveyor belts, and platformer mechanics require smooth passenger
     transport.
   - **Verification:** Ride an elevator. Assert no sliding. Jump through a one-way platform. Assert
     passage from below and solid from above.

10. **R-4.1.10** -- The engine **SHALL** smooth character ground conformance via filtered ground
    height and normal sampling, eliminating micro-bouncing on tessellated terrain.
    - **Rationale:** Raw terrain contacts cause vibration on triangle edges and seams.
    - **Verification:** Walk over rough terrain. Assert no vibration. Assert smoothing half-life is
      configurable.

## Advanced Forces and Friction

11. **R-4.1.11** -- The engine **SHALL** apply gyroscopic torque `tau = omega x (I * omega)` during
    integration for rigid bodies with the gyroscopic flag set on `RigidBody`, producing correct
    precession and nutation under external torques for spinning projectiles, wheels, and tops.
    - **Rationale:** Omitting gyroscopic torque causes spinning objects to respond incorrectly to
      external torque and fail to precess, breaking realistic behavior for tops and gyros.
    - **Verification:** Spin a top under gravity with the gyroscopic flag. Assert the top precesses
      around the vertical axis. Disable the flag and assert the top falls without precession.
12. **R-4.1.12** -- The engine **SHALL** apply rolling friction torque opposing angular velocity on
    contacting bodies, scaled by a `rolling_friction` coefficient on `PhysicsMaterial` and the
    contact normal force, bringing spheres and wheels to rest on flat surfaces.
    - **Rationale:** Without rolling friction, spheres and wheels roll indefinitely on flat ground,
      producing unrealistic billiards, vehicle, and marble behavior.
    - **Verification:** Roll a sphere on flat ground with non-zero rolling friction. Assert the
      sphere decelerates to rest within the configured time. Assert zero rolling friction preserves
      motion.
13. **R-4.1.13** -- The engine **SHALL** support directional friction on `PhysicsMaterial` with a
    primary friction axis (`friction_direction`) and a separate `lateral_friction` coefficient,
    enabling ice surfaces, conveyor belts, rails, skis, and directional grip.
    - **Rationale:** Isotropic friction cannot represent surfaces where grip differs between
      travel-aligned and lateral directions.
    - **Verification:** Slide a body along a rail surface. Assert axis-aligned motion preserves
      velocity while lateral motion decelerates at the lateral coefficient rate.

## Gravity Models and Multi-Planet

14. **R-4.1.14** -- The engine **SHALL** select per-world gravity via a `GravityMode` ECS resource
    supporting `Uniform` direction, `Radial { g }` toward world origin, and user-supplied custom
    gravity functions, with physics simulation continuing in flat planet-local Cartesian space
    regardless of mode.
    - **Rationale:** Planet-surface games need radial gravity while flat-world games need uniform
      gravity; all collision math must remain Cartesian to avoid curved-space physics.
    - **Verification:** Set `GravityMode::Radial { g: 9.81 }`. Drop bodies at four points around the
      origin. Assert each accelerates toward the origin. Switch to `Uniform` and assert parallel
      fall.
15. **R-4.1.15** -- The engine **SHALL** support multiple planets as separate ECS worlds, each with
    its own physics BVH and gravity mode, with inter-planetary entity migration that preserves
    velocity through universe-level Euclidean transforms and breaks joints that span the boundary
    unless both bodies migrate together.
    - **Rationale:** Per-planet worlds isolate physics solves and avoid f32 precision loss across
      interstellar distances.
    - **Verification:** Create two planet worlds. Migrate a body from planet A to planet B. Assert
      velocity transforms through universe space correctly. Assert joints spanning worlds emit
      `JointBroken` events.

## 2D Physics

16. **R-4.1.16** -- The engine **SHALL** provide a 2D rigid body simulation mode with scalar moment
    of inertia, 2D collider shapes (`Circle`, `Rectangle`, `Capsule2D`, `ConvexPolygon`, `Edge`,
    `Chain`), a separate 2D physics BVH, and a 2D constraint solver operating on 2 linear plus 1
    angular degrees of freedom, coexisting with 3D physics in the same ECS world.
    - **Rationale:** 2D and 2.5D games need specialized scalar-inertia physics while sharing the
      island, sleeping, CCD, and solver infrastructure with 3D.
    - **Verification:** Spawn a 2D circle and a 3D box in the same world. Assert the 2D body solves
      against 2D colliders only. Assert the 3D body ignores the 2D BVH. Verify 2D constraint solver
      uses 2 linear + 1 angular DoF.

## Character Controller Extensions

17. **R-4.1.17** -- The engine **SHALL** support character controller wall sliding where contact
    with steep surfaces decomposes velocity into wall-parallel and wall-normal components with
    configurable wall friction, wall-angle threshold, and corner handling.
    - **Rationale:** Wall sliding produces smooth traversal in tight corridors and against
      near-vertical geometry without abrupt stops.
    - **Verification:** Move a character into a wall at 45 degrees. Assert velocity decomposes into
      sliding motion along the wall. Assert the wall friction coefficient attenuates parallel
      velocity.
18. **R-4.1.18** -- The engine **SHALL** support character controller multi-jump with configurable
    max jump count, wall jump with configurable horizontal and vertical force and cling window, and
    pre-land jump buffer with configurable duration.
    - **Rationale:** Platformer movement feel requires tunable jump counts, wall jumps, and buffered
      input without code changes per game.
    - **Verification:** Configure max jump count 2 and verify double jump succeeds. Configure wall
      jump and verify a wall-held character jumps away from the wall. Configure jump buffer 150 ms
      and verify a pre-land jump press is consumed on the next ground contact.
19. **R-4.1.19** -- The engine **SHALL** support character controller crouching via configurable
    height scale and speed scale, with a ceiling clearance shape cast before uncrouching to prevent
    clipping into geometry.
    - **Rationale:** Uncrouching under a low ceiling would otherwise pop the character into solid
      geometry.
    - **Verification:** Crouch a character under a low ceiling. Attempt to uncrouch. Assert the
      controller remains crouched until the ceiling is clear.
20. **R-4.1.20** -- The engine **SHALL** apply push forces from character controllers to dynamic
    rigid bodies on contact, with force proportional to controller mass and velocity and a
    configurable push strength per controller.
    - **Rationale:** Characters must visibly interact with loose dynamic props rather than treating
      them as immovable.
    - **Verification:** Walk a character into a lightweight dynamic box. Assert the box accelerates
      away from the controller proportional to the configured push strength.

## Non-Functional Requirements

11. **R-4.1.NF1** -- The engine **SHALL** simulate 2000 active rigid bodies with 4 substeps within 4
    ms on minimum-spec hardware.
    - **Rationale:** Frame budget headroom is required for gameplay, rendering, and audio.
    - **Verification:** Benchmark 2000 active bodies with 4 substeps. Assert total physics time
      stays within 4 ms on min-spec hardware.

12. **R-4.1.NF2** -- The engine **SHALL** use at most 256 bytes per active rigid body excluding
    collider shape data.
    - **Rationale:** Memory-efficient components maximize the number of active bodies per cache
      line.
    - **Verification:** Profile memory layout. Assert RigidBody + Velocity + AngularVelocity + Mass
      - Inertia + ExternalForce + ExternalTorque fits within 256 bytes.

13. **R-4.1.NF3** -- The engine **SHALL** produce bit-identical simulation results across Windows,
    macOS, and Linux for the same entity ordering and timestep.
    - **Rationale:** Cross-platform determinism is required for server-authoritative netcode and
      replay systems.
    - **Verification:** Run a 1000-step scenario on all three platforms. Assert bit-identical
      output.

14. **R-4.1.NF4** -- The engine **SHALL** evaluate a single character controller within 0.1 ms,
    supporting 200 simultaneous controllers.
    - **Rationale:** MMO-scale character counts require sub-millisecond per-controller cost.
    - **Verification:** Benchmark 200 character controllers on varied terrain. Assert per-controller
      cost stays within 0.1 ms.
