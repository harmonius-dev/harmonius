# R-4.6 -- Destruction & Fracture Requirements

1. **R-4.6.1** -- The engine **SHALL** generate Voronoi fracture patterns at build time with convex
   hull fragments, connectivity graphs, and per-platform fragment count caps.
   - **Rationale:** Pre-computed fracture avoids runtime mesh generation while capping per-platform
     cost.
   - **Verification:** Build a fracture asset. Assert fragment count matches the platform cap.
     Assert connectivity graph is valid.

2. **R-4.6.2** -- The engine **SHALL** load pre-fractured meshes from DCC tools as fracture assets
   with fragment geometry and break thresholds.
   - **Rationale:** Art-directed fracture enables hero destruction for castle walls and bridges.
   - **Verification:** Load a pre-fractured mesh. Assert fragments, connectivity, and thresholds
     load correctly.

3. **R-4.6.3** -- The engine **SHALL** trigger fracture when cumulative damage exceeds the
   Destructible threshold, spawning fragment entities with physics, budgeted per frame to avoid
   hitches.
   - **Rationale:** Runtime fracture activates destruction while frame-budgeting prevents spikes.
   - **Verification:** Damage an object past threshold. Assert fragments spawn. Assert spawning is
     distributed across frames on mobile.

4. **R-4.6.4** -- The engine **SHALL** track progressive damage via DamageHealth with visual
   cracking stages before full fracture.
   - **Rationale:** Progressive damage gives players feedback before objects break.
   - **Verification:** Apply damage incrementally. Assert cracking stages display at configured
     thresholds.

5. **R-4.6.5** -- The engine **SHALL** propagate stress through fragment connectivity graphs,
   causing cascading collapse when load-bearing fragments are destroyed.
   - **Rationale:** Structural analysis produces emergent building collapse from graph connectivity.
   - **Verification:** Destroy a load-bearing fragment. Assert unsupported fragments fall under
     gravity.

6. **R-4.6.6** -- The engine **SHALL** manage debris lifecycle with configurable TTL, max count
   caps, and entity pooling to eliminate allocation churn.
   - **Rationale:** Debris cleanup prevents entity count from growing unbounded during sustained
     destruction.
   - **Verification:** Fracture multiple objects. Assert debris count stays within cap. Assert
     TTL-expired debris despawns.

7. **R-4.6.7** -- The engine **SHALL** reduce distant debris to visual-only particles with no
   simulation cost and simplify collision shapes for mid-range debris.
   - **Rationale:** Debris LOD bounds simulation cost for distant fragments.
   - **Verification:** Observe debris at distance. Assert particle fallback activates. Assert no
     physics cost for distant debris.

8. **R-4.6.8** -- The engine **SHALL** auto-generate colliders from voxel SDF meshes with
   incremental updates on terrain modification.
   - **Rationale:** Voxel terrain edits must produce correct physics collision immediately.
   - **Verification:** Edit voxel terrain. Assert collider regenerates for modified chunks only.

9. **R-4.6.9** -- The engine **SHALL** distribute fragment mass proportionally to volume and inherit
   parent velocity with impact-directed impulse.
   - **Rationale:** Mass distribution and directed impulse produce realistic debris trajectories.
   - **Verification:** Fracture an object with impact. Assert fragments fly from the impact point.
     Assert heavier fragments move slower.

10. **R-4.6.10** -- The engine **SHALL** support SDF sphere-tracing for raycasts against voxel
    volumes as fast pre-checks before mesh narrowphase.
    - **Rationale:** SDF pre-checks skip expensive mesh collision when no hit exists.
    - **Verification:** Raycast into empty voxel space. Assert SDF reports no hit. Assert mesh
      narrowphase is skipped.

11. **R-4.6.11** -- The engine **SHALL** trigger voxel SDF subtraction from physics damage events
    with structural integrity checks on disconnected islands.
    - **Rationale:** Explosions carving terrain and floating islands falling are core destruction
      features.
    - **Verification:** Detonate an explosion on voxel terrain. Assert a crater forms. Assert
      disconnected islands fall.
