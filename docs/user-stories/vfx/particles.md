# User Stories -- 11.1 Particle System

## US-11.1.1.1 Simulate Millions of Particles on GPU Without CPU Bottlenecks

**As an** engine developer (P-26), **I want** GPU-driven particle simulation using compute
shaders with persistent buffers, free-list allocation, and indirect dispatch, **so that**
millions of particles update per frame without CPU readback.

## US-11.1.1.2 Spawn Particles on Animated Character Surfaces

**As a** effects artist (P-12), **I want** skinned mesh emitters that sample vertex buffers each
frame, **so that** particles spawn on moving characters (fire aura, blood drip, magic trail)
without CPU-side mesh readback.

## US-11.1.1.3 Validate Async Compute Queue Fallback

**As an** engine tester (P-27), **I want** to disable async compute and verify that particle
simulation falls back to the graphics queue without visual errors, **so that** the system works
on GPUs without dedicated async compute.

## US-11.1.2.1 Chain Simulation Modules for Complex Particle Behavior

**As a** effects artist (P-12), **I want** composable simulation modules (gravity, curl noise,
drag, vortex, color over life, collision) compiled into a single compute dispatch per emitter,
**so that** I can build complex particle behaviors by combining simple modules.

## US-11.1.2.2 See Particles Collide With Scene Geometry

**As a** player (P-23), **I want** particles to collide with the ground and walls (via depth
buffer or SDF) with configurable bounce and friction, **so that** sparks bounce off floors and
debris settles on surfaces rather than falling through the world.

## US-11.1.2.3 Validate SDF Collision Disabled on Mobile

**As an** engine tester (P-27), **I want** to verify that mobile uses depth-buffer collision
only with SDF collision and curl noise disabled, **so that** particle simulation fits within
mobile compute budgets.

## US-11.1.3.1 Render Sword Trails as Ribbon Particles

**As a** effects artist (P-12), **I want** ribbon particle rendering that generates spline-based
geometry connecting sequential particles, **so that** sword swings, spell streaks, and
projectile trails have smooth, continuous visual trails.

## US-11.1.3.2 Render Mesh Particles With Full Material Support

**As a** effects artist (P-12), **I want** mesh particle rendering using GPU instancing with
per-particle transforms and full material support, **so that** debris chunks, rock fragments,
and crystal shards render as lit 3D objects.

## US-11.1.3.3 Validate Soft-Particle Depth Fade on All Platforms

**As an** engine tester (P-27), **I want** to verify that sprite particles use soft-particle
depth fade on desktop and that the fade is disabled on low-end mobile GPUs, **so that** particle
edge blending scales per platform.

## US-11.1.4.1 Scale VFX Quality Based on Distance and Budget

**As a** effects artist (P-12), **I want** hierarchical LOD tiers that reduce spawn rate and
rendering complexity for distant effects, with a global budget manager prioritizing player-local
effects, **so that** VFX quality degrades gracefully rather than dropping frames.

## US-11.1.4.2 Maintain Stable Frame Rate During 40-Player Raid Combat

**As a** player (P-23), **I want** the global particle budget to cap total alive particles and
prioritize gameplay-critical effects during raid encounters, **so that** frame rate stays stable
even when dozens of spell effects fire simultaneously.

## US-11.1.4.3 Validate Global Particle Budget Per Platform

**As an** engine tester (P-27), **I want** to verify global particle budgets of 10K on mobile,
50K on Switch, 200K on console, and 500K+ on desktop, **so that** particle counts stay within
per-platform GPU compute and memory limits.

## US-11.1.5.1 Create Cascading Firework Effects With Sub-Emitters

**As a** effects artist (P-12), **I want** event-driven child emitter spawning triggered by
particle birth, death, and collision events, **so that** fireworks spawn bursts of color sparks,
impact effects spawn secondary dust, and spell detonations cascade without monolithic systems.

## US-11.1.5.2 Validate Sub-Emitter Depth Limit on Mobile

**As an** engine tester (P-27), **I want** to verify that mobile limits sub-emitter depth to 1
with capped child particle counts, **so that** cascade-driven sub-emitters do not cause budget
spikes on mobile.

## US-11.1.6.1 Make Fire Particles Emit Dynamic Point Lights

**As a** effects artist (P-12), **I want** particles to spawn dynamic point lights with
configurable color, intensity, and attenuation radius into the clustered light buffer, **so
that** fire, sparks, and magic effects illuminate surrounding surfaces.

## US-11.1.6.2 Validate Particle Light Cap Per Emitter Per Platform

**As an** engine tester (P-27), **I want** to verify that mobile caps particle lights to 4 per
emitter and desktop to 16, with stochastic sampling bounding per-tile lighting cost, **so that**
particle lights do not blow lighting budgets during spell-heavy combat.

## US-11.1.7.1 Render Volumetric Fire and Smoke From GPU Fluid Simulation

**As a** effects artist (P-12), **I want** grid-based Eulerian fluid simulation on GPU with
ray-marched volumetric rendering for fire, smoke, and liquid effects, **so that** large-scale
fire and smoke look physically convincing rather than sprite-based.

## US-11.1.7.2 Validate Fluid Simulation Grid Resolution Per Platform

**As an** engine tester (P-27), **I want** to verify that mobile falls back to sprite particles,
Switch uses 32^3 grids, and desktop uses 128^3, **so that** fluid simulation cost scales with
platform capability.
