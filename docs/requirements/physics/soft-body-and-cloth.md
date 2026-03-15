# R-4.7 — Soft Body & Cloth Requirements

## R-4.7.1 Position-Based Dynamics Solver

The engine **SHALL** solve XPBD constraints on `ClothSimulation` entities within the ECS
schedule (no separate physics world), converging distance constraints to within 1% of rest
length after the configured iteration count.

- **Derived from:** [F-4.7.1](../../features/physics/soft-body-and-cloth.md)
- **Rationale:** The XPBD solver is the core of all soft body and cloth simulation; constraint
  convergence determines visual plausibility and stability.
- **Verification:** Unit test — create a 10x10 cloth grid with distance constraints; apply
  gravity for 100 ticks at 10 solver iterations; measure each constraint's length vs. rest
  length; assert all constraints are within 1% of rest length.

## R-4.7.2 Cloth Simulation

The engine **SHALL** simulate cloth entities with `ClothSimulation` components that store
particle and constraint data in GPU buffers, supporting attachment to skeleton bone entities via
`ClothAttachment` components.

- **Derived from:** [F-4.7.2](../../features/physics/soft-body-and-cloth.md)
- **Rationale:** Cloth simulation for capes, banners, and sails is a core visual feature that
  must integrate with the skeletal animation system.
- **Verification:** Integration test — attach a 20x20 cloth to two bone entities of an animated
  skeleton; simulate 200 ticks; assert attached particles remain within 0.1 mm of their target
  bone positions; verify free particles respond to gravity.

## R-4.7.3 Cloth Self-Collision

The engine **SHALL** prevent cloth particle interpenetration for entities with the
`SelfCollisionEnabled` marker component, maintaining a minimum separation distance equal to the
configured thickness parameter.

- **Derived from:** [F-4.7.3](../../features/physics/soft-body-and-cloth.md)
- **Rationale:** Self-collision prevents cloth from clipping through itself, which is visually
  distracting for draped garments and folded fabric.
- **Verification:** Unit test — drop a 20x20 cloth sheet onto a sphere so it folds over itself;
  simulate 500 ticks with self-collision enabled; assert no two non-adjacent particles are
  closer than the configured thickness value.

## R-4.7.4 Two-Way Rigid Body Coupling

The engine **SHALL** generate contact constraints between cloth particles and nearby `Collider`
entities, and write reaction `ExternalForce` components on rigid body entities such that a
10 kg cloth draped over a 1 kg rigid body displaces the rigid body downward.

- **Derived from:** [F-4.7.4](../../features/physics/soft-body-and-cloth.md)
- **Rationale:** One-way collision (cloth conforms to rigid bodies) is insufficient; two-way
  coupling enables physically plausible interactions like heavy curtains pushing objects.
- **Verification:** Integration test — drape a heavy cloth over a light rigid body resting on a
  surface; simulate 200 ticks; assert the rigid body's `ExternalForce` component is non-zero
  and its vertical position decreases compared to the no-cloth baseline.

## R-4.7.5 Wind Interaction

The engine **SHALL** model wind as ECS entities with `WindSource` components (position,
direction, strength, radius, turbulence). A wind field generation system **SHALL** sample all
active `WindSource` entities into a shared 3D wind field texture each frame. The cloth
simulation **SHALL** sample wind forces from this shared wind field texture.

- **Derived from:** [F-4.7.5](../../features/physics/soft-body-and-cloth.md)
- **Rationale:** WindSource ECS entities enable localized and composable wind effects, while
  the shared wind field texture ensures all consumers (cloth, hair, foliage, particles) sample
  a consistent wind representation each frame.
- **Verification:** Integration test — place a stationary 10x10 cloth and a directional
  `WindSource` entity at strength S; simulate 100 ticks; measure average particle displacement
  in the wind direction; double S and assert displacement increases by at least 50%. Verify
  that the cloth reads wind forces from the shared 3D wind field texture, not directly from
  `WindSource` entities.

## R-4.7.6 Cloth Tearing

The engine **SHALL** split the cloth mesh topology when constraint strain exceeds the configured
threshold, spawning new cloth entities for separated patches within one frame of the tear
event.

- **Derived from:** [F-4.7.6](../../features/physics/soft-body-and-cloth.md)
- **Rationale:** Destructible sails and battle-damaged banners require runtime topology changes
  driven by physical forces.
- **Verification:** Integration test — anchor a cloth at two corners and apply a downward force
  exceeding the tear threshold at the center; assert the cloth splits into two or more separate
  `ClothSimulation` entities within one frame; verify total particle count is preserved across
  the new entities.

## R-4.7.7 Cloth Level of Detail

The engine **SHALL** reduce cloth simulation fidelity (particle count, constraint iterations,
update frequency) based on camera distance as configured in the `ClothLod` component, and
disable physics simulation entirely for cloth beyond the maximum LOD distance.

- **Derived from:** [F-4.7.7](../../features/physics/soft-body-and-cloth.md)
- **Rationale:** Distant cloth does not benefit from high-fidelity simulation; LOD saves
  compute budget for nearby cloth that the player can see in detail.
- **Verification:** Benchmark — place 100 cloth entities at varying distances from the camera;
  measure total cloth simulation time with LOD enabled vs. disabled; assert LOD reduces total
  simulation time by at least 50%. Integration test — verify cloth beyond the maximum LOD
  distance has zero solver invocations per frame.

---

## Non-Functional Requirements

### R-4.7.NF1 Cloth Simulation Frame Budget

A single cloth instance (20x20 particle grid, 10 solver iterations) **SHALL** complete its
simulation step within 0.5 ms on minimum-spec hardware, supporting at least 8 simultaneous
active cloth instances within the 4 ms physics budget.

- **Derived from:** R-4.7.1, R-4.7.2
- **Rationale:** Characters with capes, banners on walls, and sails on ships must simulate
  simultaneously without exhausting the physics frame budget.
- **Verification:** Benchmark — simulate 8 cloth instances (20x20 grid, 10 iterations each);
  measure total cloth system time per frame; assert it completes within 4 ms.

### R-4.7.NF2 Cloth Memory Budget

Per-cloth memory consumption (particle positions, velocities, constraint descriptors,
attachment data) **SHALL NOT** exceed 64 KB for a 20x20 particle grid.

- **Derived from:** R-4.7.1, R-4.7.2
- **Rationale:** Many cloth instances in a scene (flags, banners, garments) must fit within
  GPU memory budgets alongside rendering and fluid data.
- **Verification:** Profile — create a 20x20 cloth instance; measure total allocated buffer
  size for particles, velocities, and constraints; assert it does not exceed 64 KB.

### R-4.7.NF3 Wind Field Update Cost

Wind field generation (sampling all active `WindSource` entities into the shared 3D wind
texture) **SHALL** complete within 0.2 ms per frame for up to 16 active wind sources.

- **Derived from:** R-4.7.5
- **Rationale:** Wind affects cloth, hair, foliage, and particles; the shared wind field must
  update cheaply each frame to avoid compounding cost across all consumers.
- **Verification:** Benchmark — create 16 active wind sources of mixed types; measure wind
  field generation time per frame; assert it completes within 0.2 ms.
