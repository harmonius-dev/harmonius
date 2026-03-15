# R-4.7 — Soft Body & Cloth User Stories

## F-4.7.1 Position-Based Dynamics Solver

## US-4.7.1.1 Configure XPBD Solver Parameters

**As a** game developer (P-15), **I want to** set constraint compliance and iteration count on
`ClothSimulation` components, **so that** solver stiffness is independent of timestep.

## US-4.7.1.2 Configure Solver Iterations in Editor

**As a** designer (P-5), **I want to** set XPBD solver iteration count per cloth instance in the
editor, **so that** cloth stiffness versus performance is tuned visually.

## US-4.7.1.3 Verify Constraint Convergence

**As an** engine tester (P-27), **I want to** create a 10x10 cloth grid, apply gravity for 100 ticks
at 10 iterations, and assert all distance constraints are within 1% of rest length, **so that**
solver convergence is confirmed.

## US-4.7.1.4 Benchmark XPBD Solver Cost

**As an** engine tester (P-27), **I want to** benchmark a 20x20 cloth grid at 10 iterations and
assert completion within 0.5ms, **so that** single-cloth cost meets the budget.

## US-4.7.1.5 Implement XPBD Solver System

**As an** engine developer (P-26), **I want to** implement `XpbdSolverSystem` that resolves
distance, bending, volume preservation, and shape-matching constraints on GPU buffers, **so that**
soft body physics runs within the ECS schedule.

## US-4.7.1.6 Experience Cloth Draping Naturally

**As a** player (P-23), **I want** cloth to drape, fold, and hang realistically on objects and
characters, **so that** soft materials look physically believable.

---

## F-4.7.2 Cloth Simulation

## US-4.7.2.1 Set Up Cloth Entities

**As a** game developer (P-15), **I want to** create cloth entities with `ClothSimulation`
components storing particle positions, velocities, and constraint buffers, **so that** cloth
instances are standard ECS entities.

## US-4.7.2.2 Attach Cloth to Skeleton Bones

**As a** game developer (P-15), **I want to** add `ClothAttachment` components linking cloth
particles to skeleton bone entities, **so that** capes, banners, and sails follow animated
characters.

## US-4.7.2.3 Configure Cloth Material Properties in Editor

**As a** designer (P-5), **I want to** set stretch, shear, and bend resistance parameters per cloth
instance in the editor, **so that** different fabrics feel distinct.

## US-4.7.2.4 Verify Cloth Attachment Accuracy

**As an** engine tester (P-27), **I want to** attach a 20x20 cloth to two animated bone entities,
simulate 200 ticks, and assert attached particles stay within 0.1mm of their bone positions, **so
that** attachment accuracy is verified.

## US-4.7.2.5 Verify Free Particles Respond to Gravity

**As an** engine tester (P-27), **I want to** verify unattached cloth particles fall under gravity
while attached particles remain fixed, **so that** attachment and free simulation coexist correctly.

## US-4.7.2.6 Benchmark Multiple Cloth Instances

**As an** engine tester (P-27), **I want to** simulate 8 cloth instances (20x20 each) and assert
total system time stays within 4ms, **so that** multi-cloth cost meets the budget.

## US-4.7.2.7 Implement Cloth Simulation System

**As an** engine developer (P-26), **I want to** implement `ClothSimulationSystem` that steps
particle-constraint meshes and writes updated positions to GPU buffers each tick, **so that** cloth
simulation runs on the GPU.

## US-4.7.2.8 Experience Character Capes Flowing in Movement

**As a** player (P-23), **I want** character capes and robes to flow and react to movement, **so
that** characters look dynamic and alive.

---

## F-4.7.3 Cloth Self-Collision

## US-4.7.3.1 Enable Self-Collision on Cloth

**As a** game developer (P-15), **I want to** add a `SelfCollisionEnabled` marker component to cloth
entities with configurable thickness, **so that** cloth does not clip through itself.

## US-4.7.3.2 Configure Self-Collision Thickness in Editor

**As a** designer (P-5), **I want to** set the self-collision thickness parameter in the editor,
**so that** garment folding looks correct without excessive cost.

## US-4.7.3.3 Verify Self-Collision Prevents Interpenetration

**As an** engine tester (P-27), **I want to** drop a 20x20 cloth onto a sphere so it folds, simulate
500 ticks, and assert no non-adjacent particles are closer than the thickness, **so that**
self-collision prevents clipping.

## US-4.7.3.4 Implement Cloth Self-Collision System

**As an** engine developer (P-26), **I want to** implement `ClothSelfCollisionSystem` using spatial
hashing or BVH-based continuous collision detection on cloth particles, **so that**
self-intersection is prevented.

## US-4.7.3.5 Experience Cloth Folding Without Clipping

**As a** player (P-23), **I want** draped garments to fold over themselves without visibly clipping,
**so that** cloth looks physically correct.

---

## F-4.7.4 Two-Way Rigid Body Coupling

## US-4.7.4.1 Set Up Cloth-Rigid Body Interaction

**As a** game developer (P-15), **I want** cloth to drape over rigid bodies and push them via
reaction forces, **so that** two-way physical coupling is supported between cloth and rigid bodies.

## US-4.7.4.2 Verify Two-Way Coupling Forces

**As an** engine tester (P-27), **I want to** drape a 10 kg cloth over a 1 kg rigid body and assert
the rigid body's `ExternalForce` is non-zero and its position decreases, **so that** two-way
coupling is confirmed.

## US-4.7.4.3 Implement Two-Way Coupling in XPBD Solver

**As an** engine developer (P-26), **I want to** implement contact constraints between cloth
particles and nearby collider entities, writing reaction forces as `ExternalForce` on rigid bodies,
**so that** cloth and rigid bodies interact bidirectionally.

## US-4.7.4.4 Experience Cloth Pushing Objects

**As a** player (P-23), **I want** heavy curtains to push lightweight objects and drape over
furniture realistically, **so that** cloth interacts with the physical world.

---

## F-4.7.5 Wind Interaction

## US-4.7.5.1 Place Wind Source Entities

**As a** game developer (P-15), **I want to** create wind source entities with `WindSource`
components (directional, point, vortex) that are sampled into a shared 3D wind field texture, **so
that** wind affects all consumers consistently.

## US-4.7.5.2 Configure Wind Parameters in Editor

**As a** designer (P-5), **I want to** set wind direction, strength, radius, and turbulence per wind
source in the editor, **so that** environmental wind is tuned visually.

## US-4.7.5.3 Place Wind Sources for Environmental Effects

**As a** level designer (P-6), **I want to** place directional and vortex wind sources near flags,
banners, and trees in the level, **so that** environmental wind brings the world to life.

## US-4.7.5.4 Verify Wind Affects Cloth Proportionally

**As an** engine tester (P-27), **I want to** place a stationary cloth with a directional wind at
strength S, simulate 100 ticks, double S, and assert displacement increases by at least 50%, **so
that** wind response scales with strength.

## US-4.7.5.5 Verify Cloth Reads Shared Wind Field

**As an** engine tester (P-27), **I want to** verify cloth reads wind forces from the shared 3D wind
field texture rather than directly from `WindSource` entities, **so that** all consumers use a
consistent wind representation.

## US-4.7.5.6 Benchmark Wind Field Update Cost

**As an** engine tester (P-27), **I want to** create 16 active wind sources and assert wind field
generation completes within 0.2ms, **so that** wind field cost meets the budget.

## US-4.7.5.7 Implement Wind Field Generation System

**As an** engine developer (P-26), **I want to** implement `WindFieldGenerationSystem` that samples
all active `WindSource` entities into a shared 3D wind texture, and `ClothWindSystem` that applies
sampled forces to cloth entities, **so that** wind is a unified system.

## US-4.7.5.8 Experience Banners and Flags Blowing in Wind

**As a** player (P-23), **I want** banners, flags, and sails to flutter and ripple in the wind, **so
that** the environment feels dynamic and alive.

---

## F-4.7.6 Cloth Tearing

## US-4.7.6.1 Configure Tear Strain Thresholds

**As a** game developer (P-15), **I want to** set a tear strain threshold on `ClothSimulation`
components, **so that** cloth tears when stretched beyond its limit.

## US-4.7.6.2 Set Up Destructible Sails and Banners

**As a** designer (P-5), **I want to** configure tear thresholds on sail and banner cloth in the
editor, **so that** battle damage tears cloth during gameplay.

## US-4.7.6.3 Verify Cloth Splitting on Tear

**As an** engine tester (P-27), **I want to** anchor a cloth at two corners, apply force exceeding
the tear threshold, and assert the cloth splits into separate entities within one frame with total
particle count preserved, **so that** tearing mechanics are correct.

## US-4.7.6.4 Implement Cloth Tearing System

**As an** engine developer (P-26), **I want to** implement `ClothTearingSystem` that splits mesh
topology, updates constraint buffers, and spawns new cloth entities for separated patches, **so
that** dynamic cloth tearing is supported.

## US-4.7.6.5 Experience Sails Tearing in Battle

**As a** player (P-23), **I want** sails to tear under heavy wind or weapon impacts, **so that**
naval and siege battles have visible cloth destruction.

---

## F-4.7.7 Cloth Level of Detail

## US-4.7.7.1 Configure Cloth LOD Tiers

**As a** designer (P-5), **I want to** set LOD tier distances and parameters (particle count,
iterations, update frequency) per cloth in the editor, **so that** distant cloth is cheap.

## US-4.7.7.2 Configure Platform-Specific LOD Distances

**As a** designer (P-5), **I want to** set per-platform LOD transition distances (5m mobile, 30m
desktop), **so that** cloth cost scales with platform capability.

## US-4.7.7.3 Verify LOD Reduces Simulation Cost

**As an** engine tester (P-27), **I want to** place 100 cloth entities at varying distances with LOD
enabled vs disabled and assert at least 50% simulation time reduction, **so that** LOD effectiveness
is quantified.

## US-4.7.7.4 Verify Maximum LOD Disables Simulation

**As an** engine tester (P-27), **I want to** verify cloth beyond the maximum LOD distance has zero
solver invocations per frame, **so that** distant cloth has zero simulation cost.

## US-4.7.7.5 Verify LOD Transitions Are Smooth

**As an** engine tester (P-27), **I want to** verify LOD transitions interpolate particle positions
without visible popping, **so that** transitions are not noticeable.

## US-4.7.7.6 Implement Cloth LOD System

**As an** engine developer (P-26), **I want to** implement `ClothLodSystem` that adjusts simulation
fidelity based on camera distance and replaces physics with animation fallback at extreme distances,
**so that** cloth scales efficiently.

## US-4.7.7.7 Experience Distant Cloth Without Frame Drops

**As a** player (P-23), **I want** scenes with many cloth objects (flags, banners, capes) to run
smoothly, **so that** cloth does not degrade overall performance.

---

## Non-Functional Requirements

### R-4.7.NF1 Cloth Simulation Frame Budget

A single cloth instance (20x20 particle grid, 10 solver iterations) **SHALL** complete its
simulation step within 0.5 ms on minimum-spec hardware, supporting at least 8 simultaneous active
cloth instances within the 4 ms physics budget.

- **Derived from:** R-4.7.1, R-4.7.2
- **Rationale:** Characters with capes, banners on walls, and sails on ships must simulate
  simultaneously without exhausting the physics frame budget.
- **Verification:** Benchmark -- simulate 8 cloth instances (20x20 grid, 10 iterations each);
  measure total cloth system time per frame; assert it completes within 4 ms.

### R-4.7.NF2 Cloth Memory Budget

Per-cloth memory consumption (particle positions, velocities, constraint descriptors, attachment
data) **SHALL NOT** exceed 64 KB for a 20x20 particle grid.

- **Derived from:** R-4.7.1, R-4.7.2
- **Rationale:** Many cloth instances in a scene (flags, banners, garments) must fit within GPU
  memory budgets alongside rendering and fluid data.
- **Verification:** Profile -- create a 20x20 cloth instance; measure total allocated buffer size
  for particles, velocities, and constraints; assert it does not exceed 64 KB.

### R-4.7.NF3 Wind Field Update Cost

Wind field generation (sampling all active `WindSource` entities into the shared 3D wind texture)
**SHALL** complete within 0.2 ms per frame for up to 16 active wind sources.

- **Derived from:** R-4.7.5
- **Rationale:** Wind affects cloth, hair, foliage, and particles; the shared wind field must update
  cheaply each frame to avoid compounding cost across all consumers.
- **Verification:** Benchmark -- create 16 active wind sources of mixed types; measure wind field
  generation time per frame; assert it completes within 0.2 ms.
