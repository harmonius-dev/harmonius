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
