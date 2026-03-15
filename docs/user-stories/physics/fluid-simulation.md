# User Stories — 4.8 Fluid Simulation

## F-4.8.1 SPH Fluid Simulation

**US-4.8.1a** As a gameplay programmer, I want SPH fluid volumes to be ECS entities with
`FluidVolume` and `FluidParticleBuffer` components so that fluid simulation integrates with
the ECS schedule and standard component queries.

**US-4.8.1b** As a player, I want pouring water to splash and flow around obstacles so that
fluid interactions feel physically plausible.

## F-4.8.2 FLIP/PIC Hybrid Simulation

**US-4.8.2a** As a designer, I want to use FLIP/PIC simulation for large-scale flooding and
river flow so that water scenarios beyond SPH's practical scale look visually convincing.

## F-4.8.3 Eulerian Grid Fluid Solver

**US-4.8.3a** As a designer, I want to define bounded water volumes (lakes, harbors, moats)
as Eulerian grid entities so that contained bodies of water simulate stably without
particle scatter.

**US-4.8.3b** As a gameplay programmer, I want the Eulerian solver to produce a
divergence-free velocity field so that water behaves as an incompressible fluid with
correct pressure dynamics.

## F-4.8.4 Fluid Surface Reconstruction

**US-4.8.4a** As a gameplay programmer, I want particle-based fluids to produce a watertight
triangle mesh for rendering so that fluid volumes support refraction, reflection, and foam
effects through the standard rendering pipeline.

**US-4.8.4b** As a player, I want water surfaces to look smooth and cohesive rather than
blobby so that fluid visuals match the quality of the rest of the scene.

## F-4.8.5 Water Surface Simulation

**US-4.8.5a** As a designer, I want to configure FFT and Gerstner wave parameters on ocean,
river, and lake entities so that each body of water has a distinct wave character — calm
lakes, choppy seas, flowing rivers.

**US-4.8.5b** As a player, I want water surfaces to tile seamlessly across streaming zones
so that oceans and rivers look continuous when exploring large open worlds.

## F-4.8.6 Buoyancy and Drag Forces

**US-4.8.6a** As a gameplay programmer, I want buoyancy and drag computed automatically for
any rigid body overlapping a fluid volume so that boats float, debris bobs, and heavy
objects sink without custom force code per object.

**US-4.8.6b** As a player, I want objects thrown into water to float or sink based on their
density so that water interactions feel physically intuitive.

## F-4.8.7 Two-Way Fluid-Rigid Body Coupling

**US-4.8.7a** As a player, I want objects entering water to create splashes and wakes so
that fluid reacts to the world rather than being a static visual layer.

**US-4.8.7b** As a gameplay programmer, I want two-way fluid coupling to degrade gracefully
when the simulation budget is exceeded so that frame rate remains stable during heavy
interaction scenes.
