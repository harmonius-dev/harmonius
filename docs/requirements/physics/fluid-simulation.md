# R-4.8 -- Fluid Simulation Requirements

1. **R-4.8.1** -- The engine **SHALL** simulate SPH fluids as ECS entities with GPU particle
   buffers, bounded particle counts per FluidVolume per platform tier.
   - **Rationale:** SPH provides localized interactive fluid effects with budgeted particle counts.
   - **Verification:** Create an SPH volume. Assert particles simulate. Assert mobile disables SPH
     by default.

2. **R-4.8.2** -- The engine **SHALL** support FLIP/PIC hybrid simulation combining grid pressure
   projection with particle velocity preservation.
   - **Rationale:** FLIP/PIC combines grid stability with particle detail for large-scale flooding.
   - **Verification:** Simulate a flood scenario. Assert grid pressure projection stabilizes. Assert
     particles preserve fine detail.

3. **R-4.8.3** -- The engine **SHALL** support Eulerian grid fluid solvers for bounded water volumes
   with configurable per-entity grid resolution.
   - **Rationale:** Grid solvers produce stable results for enclosed calm water bodies.
   - **Verification:** Simulate a lake. Assert velocity field converges. Assert grid resolution
     matches the entity configuration.

4. **R-4.8.4** -- The engine **SHALL** reconstruct fluid surfaces from particle buffers into
   renderable meshes supporting refraction, reflection, and foam effects.
   - **Rationale:** Surface reconstruction bridges physics data to the rendering pipeline.
   - **Verification:** Render a fluid surface. Assert watertight mesh. Assert refraction and
     reflection are visible.

5. **R-4.8.5** -- The engine **SHALL** provide WaterSurface ECS components with wave synthesis, flow
   fields, and integration with the visual water rendering system (F-3.4.1).
   - **Rationale:** Unified water surface components bridge physics and rendering for all water body
     types.
   - **Verification:** Create a river entity. Assert flow fields drive visual UV animation. Assert
     physics and rendering share wave state.

6. **R-4.8.6** -- The engine **SHALL** apply buoyancy and drag forces to rigid bodies overlapping
   fluid volumes, with per-platform buoyant body count and sample point caps.
   - **Rationale:** Buoyancy is fundamental for ships, floating objects, and swimming.
   - **Verification:** Drop a rigid body into water. Assert it floats at the correct depth. Assert
     mobile limits buoyant body count.

7. **R-4.8.7** -- The engine **SHALL** support two-way fluid-rigid body coupling where submerged
   bodies displace fluid, producing splashes and wakes, with mobile falling back to one-way
   coupling.
   - **Rationale:** Two-way coupling produces emergent splash and wake effects.
   - **Verification:** Drop a large body into fluid. Assert fluid displaces. Assert mobile omits
     fluid displacement.
