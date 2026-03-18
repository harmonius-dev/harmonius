# R-4.6 — Destruction & Fracture Requirements

## F-4.6.1 Voronoi Fracture Generation

| ID       | Derived From                                                  |
|----------|---------------------------------------------------------------|
| R-4.6.1  | [F-4.6.1](../../features/physics/destruction-and-fracture.md) |
| R-4.6.1a | [F-4.6.1](../../features/physics/destruction-and-fracture.md) |

1. **R-4.6.1** — Build-Time Voronoi Decomposition: The engine **SHALL** generate 3D Voronoi fracture
   assets at build time, producing convex hull fragments, connectivity graphs, and joint
   configurations. Fracture point seeding **SHALL** support random, impact-directed, and
   artist-guided modes.
   - **Rationale:** Pre-computed fracture avoids runtime mesh generation cost; multiple seeding
     modes enable both procedural and art-directed destruction.
   - **Verification:** Fracture a unit cube into 20 Voronoi fragments. Assert total fragment volume
     is within 1% of the original.
2. **R-4.6.1a** — Platform-Specific Fragment Counts: Fragment counts **SHALL** be configurable per
   platform (default 8 mobile, 64 desktop) in the build pipeline.
   - **Rationale:** Fragment count directly impacts simulation and rendering cost; platforms have
     different budgets.
   - **Verification:** Build fracture assets for mobile and desktop targets. Assert fragment counts
     match configured values.

## F-4.6.2 Pre-Fractured Mesh Authoring

| ID      | Derived From                                                  |
|---------|---------------------------------------------------------------|
| R-4.6.2 | [F-4.6.2](../../features/physics/destruction-and-fracture.md) |

1. **R-4.6.2** — Pre-Fractured Asset Import: The engine **SHALL** support importing pre-fractured
   meshes authored in DCC tools, storing fragment geometry, connectivity graph, and joint break
   thresholds in a fracture asset. An entity with a `Destructible` component **SHALL** reference
   this asset by handle.
   - **Rationale:** Hero objects like castle walls and bridges require art-directed fracture
     patterns that cannot be generated procedurally.
   - **Verification:** Load a pre-fractured asset with 15 fragments. Trigger fracture. Assert all
     fragment and joint entities spawn within one frame.

## F-4.6.3 Runtime Fracture and Activation

| ID       | Derived From                                                  |
|----------|---------------------------------------------------------------|
| R-4.6.3  | [F-4.6.3](../../features/physics/destruction-and-fracture.md) |
| R-4.6.3a | [F-4.6.3](../../features/physics/destruction-and-fracture.md) |
| R-4.6.3b | [F-4.6.3](../../features/physics/destruction-and-fracture.md) |

1. **R-4.6.3** — Damage-Triggered Fracture Activation: The `FractureActivationSystem` **SHALL**
   trigger fracture when cumulative damage on a `DamageHealth` component exceeds the threshold in
   the `Destructible` component. The intact entity **SHALL** be despawned and all fragment entities
   spawned with `RigidBody`, `Collider`, `DebrisLifetime`, and `Transform` components within the
   same frame.
   - **Rationale:** Fracture must be responsive to feel impactful; same-frame activation prevents
     visible delay between impact and destruction.
   - **Verification:** Apply damage exceeding the threshold. Assert the intact entity is despawned
     and all fragment entities spawn within the same frame.
2. **R-4.6.3a** — Fracture Activation Budget: Fracture activation **SHALL** complete within 2 ms for
   objects with up to 50 fragments.
   - **Rationale:** Destruction during gameplay must not cause frame hitches; the 2 ms budget keeps
     fracture within a small fraction of the frame.
   - **Verification:** Trigger fracture on a 50-fragment object. Measure wall-clock time. Assert
     completion within 2 ms.
3. **R-4.6.3b** — Fragment Position Accuracy: Fragment `Transform` positions **SHALL** match the
   fracture asset layout on spawn, preserving the original object's visual appearance at the moment
   of fracture.
   - **Rationale:** Misaligned fragments create visible gaps or overlaps at the moment of
     destruction.
   - **Verification:** Trigger fracture. Assert all fragment positions match the fracture asset
     layout within 0.1 mm.

## F-4.6.4 Progressive Damage Model

| ID       | Derived From                                                  |
|----------|---------------------------------------------------------------|
| R-4.6.4  | [F-4.6.4](../../features/physics/destruction-and-fracture.md) |
| R-4.6.4a | [F-4.6.4](../../features/physics/destruction-and-fracture.md) |

1. **R-4.6.4** — Impulse-Based Damage Accumulation: The `DamageAccumulationSystem` **SHALL** process
   contact events and subtract damage from `DamageHealth` based on impact impulse magnitude. Visual
   damage stages **SHALL** be driven by configurable thresholds in the `DamageHealth` component.
   - **Rationale:** Progressive visual damage (cracks before fracture) provides gameplay feedback
     and heightens the destruction experience.
   - **Verification:** Apply incremental impulses to a 3-stage `DamageHealth` entity. Assert each
     stage triggers in order with integrity decreasing proportionally within 5%.
2. **R-4.6.4a** — Server-Authoritative Damage State: `DamageHealth` state **SHALL** replicate via
   the ECS state replication system to prevent client-side cheating.
   - **Rationale:** Damage values must be authoritative on the server to prevent clients from
     bypassing destruction.
   - **Verification:** Modify `DamageHealth` on the client. Assert the server rejects the
     modification and re-synchronizes the authoritative value.

## F-4.6.5 Stress Propagation and Structural Collapse

| ID       | Derived From                                                  |
|----------|---------------------------------------------------------------|
| R-4.6.5  | [F-4.6.5](../../features/physics/destruction-and-fracture.md) |
| R-4.6.5a | [F-4.6.5](../../features/physics/destruction-and-fracture.md) |

1. **R-4.6.5** — Connectivity Graph Traversal: The `StructuralAnalysisSystem` **SHALL** traverse the
   joint connectivity graph to identify fragments without a path to a grounded anchor. Unsupported
   fragments **SHALL** have their joint entities despawned and fall as independent rigid bodies
   under gravity.
   - **Rationale:** Cascading structural collapse emerges from connectivity analysis; removing key
     supports must trigger chain-reaction failure.
   - **Verification:** Build a 3-column arch. Break the keystone joint. Assert unsupported fragments
     have joints despawned within one frame and fall under gravity.
2. **R-4.6.5a** — Structural Analysis Scalability: Graph traversal **SHALL** complete within 0.5 ms
   for connectivity graphs containing up to 200 fragment nodes.
   - **Rationale:** Structural analysis runs every frame a load-bearing joint breaks; it must avoid
     frame spikes during cascade events.
   - **Verification:** Benchmark: build a 200-node fragment graph. Break a central node. Assert
     traversal completes within 0.5 ms.

## F-4.6.6 Debris Simulation and Lifecycle

| ID      | Derived From                                                  |
|---------|---------------------------------------------------------------|
| R-4.6.6 | [F-4.6.6](../../features/physics/destruction-and-fracture.md) |

1. **R-4.6.6** — Debris Lifetime and Cap Enforcement: The engine **SHALL** manage debris via
   `DebrisLifetime` components with configurable time-to-live values. The `DebrisLifetimeSystem`
   **SHALL** despawn expired debris and enforce a configurable global debris count cap by despawning
   the oldest entities first when the cap is exceeded.
   - **Rationale:** Unbounded debris accumulation degrades performance over time; automatic cleanup
     prevents resource exhaustion.
   - **Verification:** Spawn 500 debris with a cap of 200. Assert active count never exceeds 200
     with oldest despawned first. Verify all debris despawns within 1 frame of lifetime expiration.

## F-4.6.7 Debris Pooling and LOD

| ID       | Derived From                                                  |
|----------|---------------------------------------------------------------|
| R-4.6.7  | [F-4.6.7](../../features/physics/destruction-and-fracture.md) |
| R-4.6.7a | [F-4.6.7](../../features/physics/destruction-and-fracture.md) |

1. **R-4.6.7** — Debris Entity Pooling: The engine **SHALL** recycle despawned debris entities from
   a pool by resetting their components with new fragment data, reducing allocation churn during
   destruction events by at least 80% compared to unpooled spawning.
   - **Rationale:** Frequent entity creation and destruction during sustained battles causes
     allocation pressure; pooling eliminates this overhead.
   - **Verification:** Trigger 10 destruction events with pooling enabled vs disabled. Assert
     pooling reduces allocation count by at least 80%.
2. **R-4.6.7a** — Distance-Based Debris LOD: The `DebrisLodSystem` **SHALL** reduce collision shape
   complexity for distant debris and remove `RigidBody` and `Collider` components entirely beyond a
   configurable maximum LOD distance, resulting in zero simulation cost.
   - **Rationale:** Distant debris is not visually important enough to justify simulation cost;
     removing physics components eliminates their CPU impact.
   - **Verification:** Place debris beyond the max LOD distance. Assert no `RigidBody` or `Collider`
     components are present. Assert zero solver invocations for that entity.

## Non-Functional Requirements

| ID        | Derived From     |
|-----------|------------------|
| R-4.6.NF1 | R-4.6.3, R-4.6.2 |
| R-4.6.NF2 | R-4.6.6, R-4.6.7 |
| R-4.6.NF3 | R-4.6.5          |

1. **R-4.6.NF1** — Fracture Activation Frame Budget: Fracture activation (despawning the intact
   entity and spawning all fragment entities with physics components) **SHALL** complete within 2 ms
   for objects with up to 50 fragments, and budget spawning across frames for objects exceeding 50
   fragments.
   - **Rationale:** Destruction events during gameplay must not cause visible frame hitches; large
     fractures must be amortized across frames to stay within budget.
   - **Verification:** Benchmark -- trigger fracture on an object with 50 fragments; measure
     wall-clock time for the activation frame; assert it completes within 2 ms.
2. **R-4.6.NF2** — Maximum Active Debris Count: The engine **SHALL** enforce a configurable global
   debris cap (default 500) and maintain stable frame rates when the cap is reached during sustained
   destruction sequences.
   - **Rationale:** Sustained battles can generate thousands of fragments; an enforced cap prevents
     unbounded memory and simulation cost growth.
   - **Verification:** Stress test -- trigger continuous destruction generating 2,000 fragments with
     a cap of 500; assert the active debris count never exceeds 500; measure frame time and assert
     it remains within the physics budget.
3. **R-4.6.NF3** — Structural Analysis Scalability: Stress propagation graph traversal **SHALL**
   complete within 0.5 ms for connectivity graphs containing up to 200 fragment nodes.
   - **Rationale:** Structural collapse analysis runs every frame a load-bearing joint breaks; it
     must be fast enough to avoid frame spikes during cascade events.
   - **Verification:** Benchmark -- build a 200-node fragment connectivity graph; break a central
     load-bearing node; measure graph traversal time; assert it completes within 0.5 ms.
