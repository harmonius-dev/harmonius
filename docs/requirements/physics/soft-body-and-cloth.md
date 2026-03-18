# R-4.7 — Soft Body & Cloth Requirements

## F-4.7.1 Position-Based Dynamics Solver

| ID       | Derived From                                             |
|----------|----------------------------------------------------------|
| R-4.7.1  | [F-4.7.1](../../features/physics/soft-body-and-cloth.md) |
| R-4.7.1a | [F-4.7.1](../../features/physics/soft-body-and-cloth.md) |

1. **R-4.7.1** — XPBD Solver System: The engine **SHALL** implement an `XpbdSolverSystem` that
   resolves distance, bending, volume preservation, and shape-matching constraints on GPU buffers
   for entities with `ClothSimulation` components. Constraint compliance **SHALL** be configurable
   per entity for timestep- independent stiffness.
   - **Rationale:** XPBD provides stable soft body simulation with compliance-based stiffness that
     does not depend on iteration count or timestep size.
   - **Verification:** Create a 10x10 cloth grid. Apply gravity for 100 ticks at 10 iterations.
     Assert all distance constraints are within 1% of rest length.
2. **R-4.7.1a** — Single Cloth Performance Budget: A single cloth instance (20x20 particle grid, 10
   solver iterations) **SHALL** complete its simulation step within 0.5 ms on minimum-spec hardware.
   - **Rationale:** Multiple cloth instances must fit within the physics budget; each must be
     individually cheap.
   - **Verification:** Benchmark a 20x20 cloth grid at 10 iterations. Assert completion within 0.5
     ms.

## F-4.7.2 Cloth Simulation

| ID       | Derived From                                             |
|----------|----------------------------------------------------------|
| R-4.7.2  | [F-4.7.2](../../features/physics/soft-body-and-cloth.md) |
| R-4.7.2a | [F-4.7.2](../../features/physics/soft-body-and-cloth.md) |
| R-4.7.2b | [F-4.7.2](../../features/physics/soft-body-and-cloth.md) |

1. **R-4.7.2** — ECS Cloth Entities: Each cloth instance **SHALL** be an ECS entity with a
   `ClothSimulation` component storing particle positions, velocities, constraint buffers, and
   material parameters (stretch, shear, bend resistance).
   - **Rationale:** Standard ECS entities ensure cloth is queryable, serializable, and manageable by
     the same systems as all other physics objects.
   - **Verification:** Create a cloth entity. Assert it is queryable via
     `(ClothSimulation, Transform)`.
2. **R-4.7.2a** — Skeleton Attachment Accuracy: Cloth particles attached to skeleton bones via
   `ClothAttachment` components **SHALL** remain within 0.1 mm of their bone positions during
   simulation.
   - **Rationale:** Detached cloth particles produce visible stretching at attachment points on
     character capes and banners.
   - **Verification:** Attach a 20x20 cloth to two animated bone entities. Simulate 200 ticks.
     Assert attached particles stay within 0.1 mm of bone positions.
3. **R-4.7.2b** — Cloth Memory Budget: Per-cloth memory consumption (particle positions, velocities,
   constraint descriptors, attachment data) **SHALL NOT** exceed 64 KB for a 20x20 particle grid.
   - **Rationale:** Many cloth instances must fit within GPU memory alongside rendering and fluid
     data.
   - **Verification:** Create a 20x20 cloth instance. Measure total buffer size. Assert it does not
     exceed 64 KB.

## F-4.7.3 Cloth Self-Collision

| ID      | Derived From                                             |
|---------|----------------------------------------------------------|
| R-4.7.3 | [F-4.7.3](../../features/physics/soft-body-and-cloth.md) |

1. **R-4.7.3** — Self-Collision Prevention: Entities with both `ClothSimulation` and
   `SelfCollisionEnabled` components **SHALL** prevent non-adjacent particles from interpenetrating
   closer than the configured thickness during simulation.
   - **Rationale:** Cloth folding over itself without self-collision produces visible clipping
     artifacts that break visual fidelity.
   - **Verification:** Drop a 20x20 cloth onto a sphere so it folds. Simulate 500 ticks. Assert no
     non-adjacent particles are closer than the thickness value.

## F-4.7.4 Two-Way Rigid Body Coupling

| ID      | Derived From                                             |
|---------|----------------------------------------------------------|
| R-4.7.4 | [F-4.7.4](../../features/physics/soft-body-and-cloth.md) |

1. **R-4.7.4** — Bidirectional Cloth-Rigid Body Forces: The `XpbdSolverSystem` **SHALL** generate
   contact constraints between cloth particles and nearby `Collider` entities, writing reaction
   forces as `ExternalForce` components on rigid bodies. Cloth **SHALL** drape over rigid bodies and
   push them via reaction forces.
   - **Rationale:** One-way coupling where cloth only receives forces looks unrealistic; two-way
     coupling enables cloth to deflect lightweight objects.
   - **Verification:** Drape a 10 kg cloth over a 1 kg rigid body. Assert the rigid body's
     `ExternalForce` is non-zero and its position decreases.

## F-4.7.5 Wind Interaction

| ID       | Derived From                                             |
|----------|----------------------------------------------------------|
| R-4.7.5  | [F-4.7.5](../../features/physics/soft-body-and-cloth.md) |
| R-4.7.5a | [F-4.7.5](../../features/physics/soft-body-and-cloth.md) |
| R-4.7.5b | [F-4.7.5](../../features/physics/soft-body-and-cloth.md) |

1. **R-4.7.5** — Shared 3D Wind Field: The engine **SHALL** support `WindSource` entities
   (directional, point, vortex) that are sampled into a shared 3D wind field texture. All wind
   consumers (cloth, hair, foliage, particles) **SHALL** read wind forces from this shared texture
   rather than querying `WindSource` entities directly.
   - **Rationale:** A shared wind field ensures consistent wind behavior across all systems and
     avoids per-consumer sampling of individual wind sources.
   - **Verification:** Verify cloth reads wind from the shared texture. Verify changing a
     `WindSource` affects cloth, foliage, and particles equally.
2. **R-4.7.5a** — Wind Force Proportionality: Cloth displacement **SHALL** increase by at least 50%
   when wind strength is doubled, demonstrating proportional response to wind magnitude.
   - **Rationale:** Wind response must scale with strength to produce visually meaningful variation
     between light and heavy wind.
   - **Verification:** Place stationary cloth with wind at strength S. Simulate 100 ticks. Double S.
     Assert displacement increases by at least 50%.
3. **R-4.7.5b** — Wind Field Update Cost: Wind field generation **SHALL** complete within 0.2 ms per
   frame for up to 16 active wind sources.
   - **Rationale:** Wind affects many systems; its update must be cheap to avoid compounding cost
     across all consumers.
   - **Verification:** Create 16 active wind sources. Measure wind field generation time. Assert
     completion within 0.2 ms.

## F-4.7.6 Cloth Tearing

| ID      | Derived From                                             |
|---------|----------------------------------------------------------|
| R-4.7.6 | [F-4.7.6](../../features/physics/soft-body-and-cloth.md) |

1. **R-4.7.6** — Strain-Based Cloth Tearing: The `ClothTearingSystem` **SHALL** check constraint
   strain against a configurable threshold on the `ClothSimulation` component. When strain exceeds
   the limit, the system **SHALL** split mesh topology, update constraint buffers, and spawn new
   cloth entities for separated patches within one frame with total particle count preserved.
   - **Rationale:** Cloth tearing enables destructible sails, banners, and garments for combat and
     environmental storytelling.
   - **Verification:** Anchor cloth at two corners. Apply force exceeding the tear threshold. Assert
     the cloth splits into separate entities within one frame with total particle count preserved.

## F-4.7.7 Cloth Level of Detail

| ID       | Derived From                                             |
|----------|----------------------------------------------------------|
| R-4.7.7  | [F-4.7.7](../../features/physics/soft-body-and-cloth.md) |
| R-4.7.7a | [F-4.7.7](../../features/physics/soft-body-and-cloth.md) |
| R-4.7.7b | [F-4.7.7](../../features/physics/soft-body-and-cloth.md) |

1. **R-4.7.7** — Distance-Based Cloth LOD: The `ClothLodSystem` **SHALL** adjust simulation fidelity
   (particle count, constraint iterations, update frequency) based on camera distance, controlled by
   a `ClothLod` component on cloth entities. At the maximum LOD distance, the system **SHALL**
   replace physics with an animation- driven fallback at zero simulation cost.
   - **Rationale:** Distant cloth does not need full simulation fidelity; LOD enables many cloth
     instances in a scene without exhausting the physics budget.
   - **Verification:** Place 100 cloth entities at varying distances with LOD enabled vs disabled.
     Assert at least 50% simulation time reduction with LOD enabled.
2. **R-4.7.7a** — Maximum LOD Disables Simulation: Cloth beyond the maximum LOD distance **SHALL**
   have zero solver invocations per frame.
   - **Rationale:** Zero simulation cost at extreme distances ensures cloth does not consume
     resources when invisible.
   - **Verification:** Place cloth beyond the maximum LOD distance. Assert zero solver invocations
     per frame.
3. **R-4.7.7b** — Smooth LOD Transitions: LOD transitions **SHALL** interpolate particle positions
   without visible popping.
   - **Rationale:** Abrupt LOD transitions are visually distracting; interpolation hides the
     transition.
   - **Verification:** Move the camera through LOD transition boundaries. Assert no visible particle
     popping.

## Non-Functional Requirements

| ID        | Derived From     |
|-----------|------------------|
| R-4.7.NF1 | R-4.7.1, R-4.7.2 |
| R-4.7.NF2 | R-4.7.1, R-4.7.2 |
| R-4.7.NF3 | R-4.7.5          |

1. **R-4.7.NF1** — Cloth Simulation Frame Budget: A single cloth instance (20x20 particle grid, 10
   solver iterations) **SHALL** complete its simulation step within 0.5 ms on minimum-spec hardware,
   supporting at least 8 simultaneous active cloth instances within the 4 ms physics budget.
   - **Rationale:** Characters with capes, banners on walls, and sails on ships must simulate
     simultaneously without exhausting the physics frame budget.
   - **Verification:** Benchmark -- simulate 8 cloth instances (20x20 grid, 10 iterations each);
     measure total cloth system time per frame; assert it completes within 4 ms.
2. **R-4.7.NF2** — Cloth Memory Budget: Per-cloth memory consumption (particle positions,
   velocities, constraint descriptors, attachment data) **SHALL NOT** exceed 64 KB for a 20x20
   particle grid.
   - **Rationale:** Many cloth instances in a scene (flags, banners, garments) must fit within GPU
     memory budgets alongside rendering and fluid data.
   - **Verification:** Profile -- create a 20x20 cloth instance; measure total allocated buffer size
     for particles, velocities, and constraints; assert it does not exceed 64 KB.
3. **R-4.7.NF3** — Wind Field Update Cost: Wind field generation (sampling all active `WindSource`
   entities into the shared 3D wind texture) **SHALL** complete within 0.2 ms per frame for up to 16
   active wind sources.
   - **Rationale:** Wind affects cloth, hair, foliage, and particles; the shared wind field must
     update cheaply each frame to avoid compounding cost across all consumers.
   - **Verification:** Benchmark -- create 16 active wind sources of mixed types; measure wind field
     generation time per frame; assert it completes within 0.2 ms.
