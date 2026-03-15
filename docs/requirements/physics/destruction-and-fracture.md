# R-4.6 — Destruction & Fracture User Stories

## F-4.6.1 Voronoi Fracture Generation

## US-4.6.1.1 Generate Voronoi Fracture Assets

**As a** game developer (P-15), **I want to** generate 3D Voronoi fracture assets at build
time with configurable fragment counts, **so that** destructible objects have pre-computed
fracture patterns.

## US-4.6.1.2 Configure Fracture Point Seeding

**As a** designer (P-5), **I want to** choose between random, impact-directed, and artist-
guided fracture point seeding in the editor, **so that** fracture patterns match design intent.

## US-4.6.1.3 Configure Fragment Count Per Platform

**As a** designer (P-5), **I want to** set platform-specific fragment counts (8 mobile, 64
desktop) in the build pipeline, **so that** fracture complexity matches platform budgets.

## US-4.6.1.4 Verify Volume Preservation

**As an** engine tester (P-27), **I want to** fracture a unit cube into 20 Voronoi fragments
and assert total fragment volume is within 1% of the original, **so that** volume preservation
is confirmed.

## US-4.6.1.5 Implement Voronoi Decomposition Pipeline

**As an** engine developer (P-26), **I want to** implement the 3D Voronoi decomposition
pipeline that produces convex hull fragments, connectivity graphs, and joint configurations,
**so that** fracture assets are generated automatically from meshes.

## US-4.6.1.6 Experience Satisfying Destruction Patterns

**As a** player (P-23), **I want** destroyed objects to break into visually interesting
fragment patterns, **so that** destruction looks dynamic rather than pre-canned.

---

## F-4.6.2 Pre-Fractured Mesh Authoring

## US-4.6.2.1 Import Pre-Fractured DCC Assets

**As a** game developer (P-15), **I want to** import fracture assets authored in DCC tools,
**so that** art-directed destruction for hero objects uses designer-authored fracture patterns.

## US-4.6.2.2 Author Fracture Patterns for Hero Objects

**As a** designer (P-5), **I want to** author custom fracture patterns for castle walls and
bridges in the DCC tool, **so that** important destruction moments are art-directed.

## US-4.6.2.3 Verify Pre-Fractured Asset Loading and Spawning

**As an** engine tester (P-27), **I want to** load a pre-fractured asset with 15 fragments,
trigger fracture, and assert all fragment and joint entities spawn within one frame, **so that**
asset loading and spawning work correctly.

## US-4.6.2.4 Implement Fracture Asset Loading

**As an** engine developer (P-26), **I want to** implement loading of fracture assets into the
`Destructible` component, storing fragment geometry, connectivity, and joint thresholds,
**so that** pre-authored destruction is supported.

## US-4.6.2.5 Place Pre-Fractured Destructible Objects in Levels

**As a** level designer (P-6), **I want to** place pre-fractured destructible objects (walls,
pillars, barricades) in the level editor, **so that** destruction is part of level design.

---

## F-4.6.3 Runtime Fracture and Activation

## US-4.6.3.1 Set Up Destructible Entities

**As a** game developer (P-15), **I want to** add `Destructible` and `DamageHealth` components
to entities referencing a fracture asset, **so that** objects break when damaged enough.

## US-4.6.3.2 Configure Damage Thresholds in Editor

**As a** designer (P-5), **I want to** set the damage threshold for fracture activation in the
editor, **so that** destruction sensitivity is tuned per object.

## US-4.6.3.3 Verify Fracture Triggers Within One Frame

**As an** engine tester (P-27), **I want to** apply damage exceeding the threshold and assert
the intact entity is despawned and all fragment entities spawn within the same frame, **so
that** fracture responsiveness is confirmed.

## US-4.6.3.4 Verify Fragment Position Accuracy

**As an** engine tester (P-27), **I want to** verify fragment `Transform` positions match the
fracture asset layout on spawn, **so that** fragments appear in the correct locations.

## US-4.6.3.5 Benchmark Fracture Activation

**As an** engine tester (P-27), **I want to** trigger fracture on a 50-fragment object and
assert activation completes within 2ms, **so that** fracture does not cause frame hitches.

## US-4.6.3.6 Implement Fracture Activation System

**As an** engine developer (P-26), **I want to** implement `FractureActivationSystem` that
despawns the intact entity and spawns fragment entities with physics components when damage
exceeds the threshold, **so that** runtime destruction is automated.

## US-4.6.3.7 Experience Objects Breaking Under Impact

**As a** player (P-23), **I want** walls, crates, and barrels to shatter into fragments when
hit hard enough, **so that** destruction feels immediate and satisfying.

## US-4.6.3.8 Place Destructible Cover in Combat Areas

**As a** level designer (P-6), **I want to** place destructible cover objects in combat areas,
**so that** players can destroy cover during fights.

---

## F-4.6.4 Progressive Damage Model

## US-4.6.4.1 Set Up Progressive Damage Stages

**As a** game developer (P-15), **I want to** configure multiple visual damage stages in the
`DamageHealth` component with threshold values, **so that** objects show cracks before full
fracture.

## US-4.6.4.2 Configure Damage Stages in Editor

**As a** designer (P-5), **I want to** set damage stage thresholds and their visual crack
levels in the editor, **so that** progressive damage feedback is tuned visually.

## US-4.6.4.3 Verify Progressive Damage Stages

**As an** engine tester (P-27), **I want to** apply incremental impulses to a 3-stage
`DamageHealth` entity and assert each stage triggers in order with integrity decreasing
proportionally within 5%, **so that** progressive damage is accurate.

## US-4.6.4.4 Verify Server-Authoritative Damage State

**As an** engine tester (P-27), **I want to** verify that `DamageHealth` state replicates via
the ECS state replication system, **so that** clients cannot cheat damage values.

## US-4.6.4.5 Implement Damage Accumulation System

**As an** engine developer (P-26), **I want to** implement `DamageAccumulationSystem` that
processes contact events and subtracts damage based on impulse magnitude, **so that** impact-
driven damage is automated.

## US-4.6.4.6 Experience Visual Cracking Before Destruction

**As a** player (P-23), **I want** objects to show cracks and deformation before they fully
break, **so that** I can judge structural integrity during gameplay.

---

## F-4.6.5 Stress Propagation and Structural Collapse

## US-4.6.5.1 Set Up Grounded Anchors

**As a** game developer (P-15), **I want to** mark certain fragments as grounded anchors in the
connectivity graph, **so that** structural analysis knows which fragments support the
structure.

## US-4.6.5.2 Configure Load-Bearing Connections

**As a** designer (P-5), **I want to** designate load-bearing joints in the editor, **so that**
breaking key connections triggers cascading collapse.

## US-4.6.5.3 Verify Cascading Collapse

**As an** engine tester (P-27), **I want to** build a 3-column arch, break the keystone joint,
and assert unsupported fragments have joints despawned within one frame and fall under gravity,
**so that** structural collapse is correct.

## US-4.6.5.4 Benchmark Structural Analysis Scalability

**As an** engine tester (P-27), **I want to** benchmark graph traversal on a 200-node
connectivity graph and assert completion within 0.5ms, **so that** analysis scales to complex
structures.

## US-4.6.5.5 Implement Structural Analysis System

**As an** engine developer (P-26), **I want to** implement `StructuralAnalysisSystem` that
traverses the joint connectivity graph to find unsupported fragments and releases them,
**so that** cascading structural collapse emerges from connectivity analysis.

## US-4.6.5.6 Experience Buildings Collapsing Realistically

**As a** player (P-23), **I want** buildings and fortifications to collapse in cascading
sequences when key supports are destroyed, **so that** siege and combat feel physically
dramatic.

## US-4.6.5.7 Design Structures with Deliberate Weak Points

**As a** level designer (P-6), **I want to** design structures with specific weak points that
trigger dramatic collapses when destroyed, **so that** destruction is a core level design tool.

---

## F-4.6.6 Debris Simulation and Lifecycle

## US-4.6.6.1 Configure Debris Lifetime

**As a** designer (P-5), **I want to** set the time-to-live value on `DebrisLifetime`
components and the global debris cap in the editor, **so that** debris cleanup is automatic.

## US-4.6.6.2 Configure Platform-Specific Debris Budgets

**As a** designer (P-5), **I want to** set per-platform debris caps (32 mobile, 512 desktop)
and TTL values, **so that** debris density matches platform capability.

## US-4.6.6.3 Verify Debris Count Cap Enforcement

**As an** engine tester (P-27), **I want to** spawn 500 debris with a cap of 200 and assert
active count never exceeds 200 with oldest despawned first, **so that** the cap is enforced
correctly.

## US-4.6.6.4 Verify Debris Lifetime Expiration

**As an** engine tester (P-27), **I want to** verify all debris entities are despawned within
1 frame of their lifetime expiring, **so that** cleanup is immediate.

## US-4.6.6.5 Stress Test Debris Under Sustained Destruction

**As an** engine tester (P-27), **I want to** trigger continuous destruction generating 2000
fragments with a cap of 500 and assert stable frame rates, **so that** the system handles
sustained destruction.

## US-4.6.6.6 Implement Debris Lifecycle System

**As an** engine developer (P-26), **I want to** implement `DebrisLifetimeSystem` that
decrements timers, despawns expired debris, and enforces the global count cap, **so that**
debris management is automatic.

## US-4.6.6.7 Experience Debris Fading Away After Destruction

**As a** player (P-23), **I want** debris fragments to eventually disappear after settling,
**so that** the world does not get cluttered with old fragments.

---

## F-4.6.7 Debris Pooling and LOD

## US-4.6.7.1 Configure Debris Pool Size

**As a** designer (P-5), **I want to** set the debris entity pool size per platform in the
editor, **so that** allocation churn is minimized during destruction.

## US-4.6.7.2 Configure Debris LOD Distances

**As a** designer (P-5), **I want to** set LOD transition and particle fallback distances in
the editor, **so that** distant debris is cheap to simulate.

## US-4.6.7.3 Verify Pooling Reduces Allocation Churn

**As an** engine tester (P-27), **I want to** trigger 10 destruction events with pooling
enabled vs disabled and assert pooling reduces allocation count by at least 80%, **so that**
pooling effectiveness is quantified.

## US-4.6.7.4 Verify Debris LOD Component Removal

**As an** engine tester (P-27), **I want to** verify debris beyond the max LOD distance has no
`RigidBody` or `Collider` components, **so that** distant debris has zero simulation cost.

## US-4.6.7.5 Implement Debris Pooling and LOD System

**As an** engine developer (P-26), **I want to** implement entity recycling for debris and
`DebrisLodSystem` that reduces collision complexity and removes physics for distant fragments,
**so that** debris scales efficiently with distance.

## US-4.6.7.6 Experience Smooth Frame Rates During Destruction

**As a** player (P-23), **I want** large-scale destruction to run smoothly without frame rate
drops, **so that** epic battles and explosions are enjoyable.

---

## Non-Functional Requirements

### R-4.6.NF1 Fracture Activation Frame Budget

Fracture activation (despawning the intact entity and spawning all fragment entities with
physics components) **SHALL** complete within 2 ms for objects with up to 50 fragments, and
budget spawning across frames for objects exceeding 50 fragments.

- **Derived from:** R-4.6.3, R-4.6.2
- **Rationale:** Destruction events during gameplay must not cause visible frame hitches; large
  fractures must be amortized across frames to stay within budget.
- **Verification:** Benchmark -- trigger fracture on an object with 50 fragments; measure
  wall-clock time for the activation frame; assert it completes within 2 ms.

### R-4.6.NF2 Maximum Active Debris Count

The engine **SHALL** enforce a configurable global debris cap (default 500) and maintain
stable frame rates when the cap is reached during sustained destruction sequences.

- **Derived from:** R-4.6.6, R-4.6.7
- **Rationale:** Sustained battles can generate thousands of fragments; an enforced cap
  prevents unbounded memory and simulation cost growth.
- **Verification:** Stress test -- trigger continuous destruction generating 2,000 fragments
  with a cap of 500; assert the active debris count never exceeds 500; measure frame time
  and assert it remains within the physics budget.

### R-4.6.NF3 Structural Analysis Scalability

Stress propagation graph traversal **SHALL** complete within 0.5 ms for connectivity graphs
containing up to 200 fragment nodes.

- **Derived from:** R-4.6.5
- **Rationale:** Structural collapse analysis runs every frame a load-bearing joint breaks;
  it must be fast enough to avoid frame spikes during cascade events.
- **Verification:** Benchmark -- build a 200-node fragment connectivity graph; break a central
  load-bearing node; measure graph traversal time; assert it completes within 0.5 ms.
