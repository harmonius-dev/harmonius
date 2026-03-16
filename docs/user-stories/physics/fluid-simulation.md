# User Stories — 4.8 Fluid Simulation

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-4.8.1.1 | game developer (P-15) | As a game developer (P-15), I want to create entities with FluidVolume (solver type SPH) and FluidParticleBuffer components, so that SPH fluid instances are ECS-based. | — | — | — |
| US-4.8.1.2 | engine developer (P-26) | As an engine developer (P-26), I want to implement SPHSystem that evaluates density and pressure kernels and writes updated particle state, so that particle-based fluid simulation runs. | — | — | — |
| US-4.8.1.3 | designer (P-5) | As a designer (P-5), I want to configure viscosity and surface tension per FluidVolume, so that different fluid types (water, oil, lava) behave distinctly. | — | — | — |
| US-4.8.1.4 | player (P-23) | As a player (P-23), I want water to splash when objects fall in, so that fluid interactions feel dynamic. | — | — | — |
| US-4.8.1.5 | engine tester (P-27) | As an engine tester (P-27), I want to confirm SPH is disabled by default on mobile, so that GPU budget is preserved. | — | — | — |
| US-4.8.1.6 | engine developer (P-26) | As an engine developer (P-26), I want particle counts bounded per FluidVolume, so that frame budgets are maintained. | — | — | — |
| US-4.8.1.7 | designer (P-5) | As a designer (P-5), I want to set domain bounds per FluidVolume, so that fluid stays within defined spatial regions. | — | — | — |
| US-4.8.1.8 | engine tester (P-27) | As an engine tester (P-27), I want to test SPH with 256K particles on high-end PC, so that maximum particle counts are validated. | — | — | — |
| US-4.8.1.9 | level designer (P-6) | As a level designer (P-6), I want SPH fluid volumes for fountains and water features, so that environments have interactive water. | — | — | — |
| US-4.8.1.10 | engine developer (P-26) | As an engine developer (P-26), I want FluidParticleBuffer to store particle data as GPU buffer resources, so that GPU compute acceleration is possible. | — | — | — |
| US-4.8.1.11 | player (P-23) | As a player (P-23), I want fluid to pour from containers and pool on surfaces, so that liquid behavior looks realistic. | — | — | — |
| US-4.8.1.12 | engine tester (P-27) | As an engine tester (P-27), I want to verify Switch caps at 1 SPH volume with 1K particles, so that budget is respected. | — | — | — |
| US-4.8.2.1 | game developer (P-15) | As a game developer (P-15), I want to create entities with FluidVolume (solver type FLIP/PIC), FluidParticleBuffer, and FluidGrid, so that hybrid fluid simulation runs. | — | — | — |
| US-4.8.2.2 | engine developer (P-26) | As an engine developer (P-26), I want to implement FLIPSystem that transfers particle velocities to grid, projects pressure, and updates particles, so that FLIP/PIC hybrid simulation works. | — | — | — |
| US-4.8.2.3 | engine developer (P-26) | As an engine developer (P-26), I want FLIP/PIC to combine grid pressure stability with particle detail preservation, so that large-scale fluid looks good. | — | — | — |
| US-4.8.2.4 | player (P-23) | As a player (P-23), I want large-scale flooding and river flow simulated, so that water feels expansive and powerful. | — | — | — |
| US-4.8.2.5 | engine tester (P-27) | As an engine tester (P-27), I want to confirm FLIP/PIC is disabled on mobile, so that GPU compute requirement is not violated. | — | — | — |
| US-4.8.2.6 | designer (P-5) | As a designer (P-5), I want to set grid resolution per FluidGrid component, so that fluid detail is tunable per volume. | — | — | — |
| US-4.8.2.7 | engine tester (P-27) | As an engine tester (P-27), I want to test FLIP on Switch with 32x32x16 grid and 2K particles, so that Switch budget is validated. | — | — | — |
| US-4.8.2.8 | level designer (P-6) | As a level designer (P-6), I want FLIP/PIC fluid for river flow and dam breaks, so that water features are interactive. | — | — | — |
| US-4.8.2.9 | engine developer (P-26) | As an engine developer (P-26), I want FluidGrid to store velocity, pressure, and boundary data for pressure projection, so that incompressibility is enforced. | — | — | — |
| US-4.8.2.10 | player (P-23) | As a player (P-23), I want river water to flow around rocks and obstacles, so that fluid interaction with environment looks natural. | — | — | — |
| US-4.8.2.11 | engine developer (P-26) | As an engine developer (P-26), I want high-end PC to support 128K particles with 128x128x64 grid, so that cinematic fluid is possible. | — | — | — |
| US-4.8.2.12 | engine tester (P-27) | As an engine tester (P-27), I want to verify grid-to-particle velocity transfer preserves momentum, so that energy conservation is correct. | — | — | — |
| US-4.8.3.1 | game developer (P-15) | As a game developer (P-15), I want to create entities with FluidVolume (solver type Eulerian) and FluidGrid, so that bounded water volumes are simulated. | — | — | — |
| US-4.8.3.2 | engine developer (P-26) | As an engine developer (P-26), I want to implement EulerianSystem that computes velocity advection, pressure projection, and boundary enforcement, so that grid-based fluid works. | — | — | — |
| US-4.8.3.3 | designer (P-5) | As a designer (P-5), I want to configure grid resolution per FluidGrid, so that fluid detail matches the volume's importance. | — | — | — |
| US-4.8.3.4 | player (P-23) | As a player (P-23), I want lake water to ripple and react when objects enter, so that bounded water bodies feel interactive. | — | — | — |
| US-4.8.3.5 | engine tester (P-27) | As an engine tester (P-27), I want to confirm Eulerian solver is disabled on mobile, so that memory budget is preserved. | — | — | — |
| US-4.8.3.6 | engine developer (P-26) | As an engine developer (P-26), I want boundary conditions enforced on the FluidGrid edges, so that fluid stays within its domain. | — | — | — |
| US-4.8.3.7 | engine developer (P-26) | As an engine developer (P-26), I want high-end PC to support adaptive grid refinement at 256x256, so that detail concentrates where needed. | — | — | — |
| US-4.8.3.8 | engine tester (P-27) | As an engine tester (P-27), I want to test Eulerian solver at each platform's grid cap (32x32 Switch, 128x128 desktop, 256x256 high-end), so that scaling is validated. | — | — | — |
| US-4.8.3.9 | level designer (P-6) | As a level designer (P-6), I want bounded Eulerian fluid volumes for lakes, harbors, and moats, so that water features are interactive. | — | — | — |
| US-4.8.3.10 | engine developer (P-26) | As an engine developer (P-26), I want velocity advection computed on the uniform grid, so that fluid motion propagates correctly. | — | — | — |
| US-4.8.3.11 | player (P-23) | As a player (P-23), I want harbor water to respond to ships entering and leaving, so that nautical environments feel alive. | — | — | — |
| US-4.8.3.12 | engine tester (P-27) | As an engine tester (P-27), I want to verify GPU compute is required for grids above 64x64, so that CPU is not bottlenecked on large grids. | — | — | — |
| US-4.8.4.1 | engine developer (P-26) | As an engine developer (P-26), I want to implement SurfaceReconstructionSystem that reconstructs renderable triangle meshes from FluidParticleBuffer, so that fluid is visible. | — | — | — |
| US-4.8.4.2 | engine developer (P-26) | As an engine developer (P-26), I want both marching cubes and screen-space surface reconstruction methods, so that quality scales with platform. | — | — | — |
| US-4.8.4.3 | engine developer (P-26) | As an engine developer (P-26), I want reconstructed mesh data written to a FluidRenderer component, so that the rendering pipeline has surface data. | — | — | — |
| US-4.8.4.4 | player (P-23) | As a player (P-23), I want fluid to have a smooth, continuous surface, so that water looks like a cohesive liquid. | — | — | — |
| US-4.8.4.5 | engine tester (P-27) | As an engine tester (P-27), I want to confirm mobile uses screen-space approach at half resolution, so that GPU budget is met. | — | — | — |
| US-4.8.4.6 | engine developer (P-26) | As an engine developer (P-26), I want reconstructed meshes to be watertight with smooth normals, so that rendering produces correct refraction and reflection. | — | — | — |
| US-4.8.4.7 | designer (P-5) | As a designer (P-5), I want FluidRenderer to bridge to the rendering pipeline for refraction, reflection, and foam effects, so that water looks beautiful. | — | — | — |
| US-4.8.4.8 | engine tester (P-27) | As an engine tester (P-27), I want to verify surface reconstruction runs at interactive frame rates, so that fluid rendering does not cause frame drops. | — | — | — |
| US-4.8.4.9 | level designer (P-6) | As a level designer (P-6), I want fluid surfaces with refraction and reflection, so that water features look stunning. | — | — | — |
| US-4.8.4.10 | engine developer (P-26) | As an engine developer (P-26), I want desktop to use marching cubes at full resolution, so that surface quality is maximized. | — | — | — |
| US-4.8.4.11 | player (P-23) | As a player (P-23), I want churning water to produce foam, so that agitated fluid looks realistic. | — | — | — |
| US-4.8.4.12 | engine tester (P-27) | As an engine tester (P-27), I want to verify marching cubes with adaptive refinement on high-end PC, so that maximum surface quality is achieved. | — | — | — |
| US-4.8.5.1 | game developer (P-15) | As a game developer (P-15), I want to create entities with WaterSurface and WaveConfig components, so that ocean, river, and lake surfaces are ECS-based. | — | — | — |
| US-4.8.5.2 | engine developer (P-26) | As an engine developer (P-26), I want to implement WaterSurfaceSystem that evaluates wave synthesis from WaveConfig plus localized displacement from nearby FluidVolume entities, so that water surfaces animate. | — | — | — |
| US-4.8.5.3 | engine developer (P-26) | As an engine developer (P-26), I want WaveConfig to store both FFT parameters and Gerstner wave descriptors, so that both wave methods are available. | — | — | — |
| US-4.8.5.4 | player (P-23) | As a player (P-23), I want ocean waves to roll in with natural motion, so that the sea looks vast and alive. | — | — | — |
| US-4.8.5.5 | engine tester (P-27) | As an engine tester (P-27), I want to confirm mobile uses Gerstner waves only with max 4 layers and no flow maps, so that mobile budget is met. | — | — | — |
| US-4.8.5.6 | designer (P-5) | As a designer (P-5), I want to configure wave layer count and flow map references in WaveConfig, so that water look and behavior is tunable. | — | — | — |
| US-4.8.5.7 | engine developer (P-26) | As an engine developer (P-26), I want WaterSurface to tile seamlessly across streaming zones, so that ocean water has no visible seams. | — | — | — |
| US-4.8.5.8 | engine developer (P-26) | As an engine developer (P-26), I want WaveConfig to support river flow fields, so that rivers have directional current. | — | — | — |
| US-4.8.5.9 | player (P-23) | As a player (P-23), I want waves to break at shorelines, so that coastal environments look natural. | — | — | — |
| US-4.8.5.10 | level designer (P-6) | As a level designer (P-6), I want configurable water surfaces for oceans, rivers, and lakes, so that water environments are varied. | — | — | — |
| US-4.8.5.11 | engine tester (P-27) | As an engine tester (P-27), I want to test FFT ocean at 512x512 resolution with shoreline breaking on high-end PC, so that maximum quality is validated. | — | — | — |
| US-4.8.5.12 | designer (P-5) | As a designer (P-5), I want to set Gerstner wave layer count for mobile, so that wave complexity matches device budget. | — | — | — |
| US-4.8.6.1 | engine developer (P-26) | As an engine developer (P-26), I want BuoyancySystem to compute buoyancy from submerged volume approximation using sample points, so that objects float correctly. | — | — | — |
| US-4.8.6.2 | engine developer (P-26) | As an engine developer (P-26), I want drag computed from entity velocity relative to fluid, so that submerged objects slow down realistically. | — | — | — |
| US-4.8.6.3 | engine developer (P-26) | As an engine developer (P-26), I want buoyancy and drag forces written as ExternalForce components, so that the physics integrator applies them automatically. | — | — | — |
| US-4.8.6.4 | player (P-23) | As a player (P-23), I want wooden crates to float and metal objects to sink, so that buoyancy feels physically correct. | — | — | — |
| US-4.8.6.5 | engine tester (P-27) | As an engine tester (P-27), I want to confirm mobile caps at 4 buoyant bodies with 4 sample points, so that performance is controlled. | — | — | — |
| US-4.8.6.6 | designer (P-5) | As a designer (P-5), I want to set sample point count per collider for buoyancy calculation, so that buoyancy precision is tunable. | — | — | — |
| US-4.8.6.7 | engine tester (P-27) | As an engine tester (P-27), I want to test buoyancy with objects of varying density, so that heavy objects sink and light objects float correctly. | — | — | — |
| US-4.8.6.8 | player (P-23) | As a player (P-23), I want ships to rock naturally in waves, so that sailing feels dynamic. | — | — | — |
| US-4.8.6.9 | level designer (P-6) | As a level designer (P-6), I want buoyancy on physics objects, so that water-based puzzles (float platforms, sinking obstacles) are possible. | — | — | — |
| US-4.8.6.10 | engine developer (P-26) | As an engine developer (P-26), I want BuoyancySystem to test rigid body bounds against FluidVolume domains, so that only relevant fluid-body pairs are processed. | — | — | — |
| US-4.8.6.11 | engine developer (P-26) | As an engine developer (P-26), I want high-end PC to support 256 buoyant bodies with 32 sample points, so that large naval scenes work. | — | — | — |
| US-4.8.6.12 | engine tester (P-27) | As an engine tester (P-27), I want to test drag in still water vs flowing river, so that drag forces account for fluid velocity. | — | — | — |
| US-4.8.7.1 | engine developer (P-26) | As an engine developer (P-26), I want fluid systems to read RigidBody, Velocity, and Collider on nearby entities and apply displacement forces into the fluid, so that objects push fluid aside. | — | — | — |
| US-4.8.7.2 | engine developer (P-26) | As an engine developer (P-26), I want object splashes to emerge from bidirectional coupling, so that falling objects create splashes without scripted effects. | — | — | — |
| US-4.8.7.3 | engine developer (P-26) | As an engine developer (P-26), I want ship wakes to emerge from two-way coupling, so that moving vessels leave visible trails in the water. | — | — | — |
| US-4.8.7.4 | player (P-23) | As a player (P-23), I want water to splash when I jump in, so that water entry feels impactful. | — | — | — |
| US-4.8.7.5 | engine tester (P-27) | As an engine tester (P-27), I want to confirm mobile uses one-way coupling only (buoyancy without fluid displacement), so that budget is respected. | — | — | — |
| US-4.8.7.6 | game developer (P-15) | As a game developer (P-15), I want debris to be carried by fluid currents, so that river debris flows downstream naturally. | — | — | — |
| US-4.8.7.7 | engine developer (P-26) | As an engine developer (P-26), I want coupling to degrade gracefully when budget is exceeded by reducing query radius, so that frame rate is maintained. | — | — | — |
| US-4.8.7.8 | engine developer (P-26) | As an engine developer (P-26), I want FluidRenderer components to bridge simulation state to rendering, so that visual feedback matches physical behavior. | — | — | — |
| US-4.8.7.9 | engine tester (P-27) | As an engine tester (P-27), I want to test two-way coupling with multiple bodies in fluid simultaneously, so that the simulation remains stable. | — | — | — |
| US-4.8.7.10 | player (P-23) | As a player (P-23), I want a visible wake trailing behind my ship, so that sailing looks realistic. | — | — | — |
| US-4.8.7.11 | engine developer (P-26) | As an engine developer (P-26), I want high-end PC to support full two-way coupling with 50m query radius and wake effects, so that cinematic naval scenes work. | — | — | — |
| US-4.8.7.12 | engine tester (P-27) | As an engine tester (P-27), I want to test coupling with reduced query radius under budget pressure, so that graceful degradation preserves frame rate. | — | — | — |
