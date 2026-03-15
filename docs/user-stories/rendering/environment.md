# User Stories -- 2.7 Environment and Atmosphere

## US-2.7.1.1 Preview Time-of-Day Sky Transitions in the Editor

**As a** environment artist (P-8), **I want** a physically-based sky atmosphere with Rayleigh and
Mie scattering that updates in real time as I scrub the sun position slider, **so that** I can
author time-of-day lighting without baking or waiting for precomputation.

## US-2.7.1.2 Validate Sky LUT Recomputation Frequency Per Platform

**As an** engine tester (P-27), **I want** to verify that mobile uses precomputed LUTs with no
runtime recomputation, Switch recomputes only on time-of-day change, and desktop recomputes
continuously, **so that** sky rendering cost matches each platform's compute budget.

## US-2.7.2.1 Add Volumetric Fog to a Dungeon Entrance

**As a** environment artist (P-8), **I want** ray-marched volumetric fog using a froxel grid that
accumulates in-scattered light per cell, **so that** dungeon entrances and swamp areas have
atmospheric depth and light shafts without baked fog volumes.

## US-2.7.2.2 Profile Froxel Grid Resolution Impact on GPU Time

**As an** engine developer (P-26), **I want** to benchmark volumetric fog at 64x36x32 (Switch),
160x90x64 (desktop), and 160x90x128 (high-end) grid resolutions, **so that** I can validate that fog
cost scales predictably across hardware tiers.

## US-2.7.2.3 Verify Analytical Fog Fallback on Mobile

**As an** engine tester (P-27), **I want** to confirm that mobile uses exponential distance/ height
fog instead of volumetric froxels, **so that** fog rendering stays within mobile bandwidth
constraints.

## US-2.7.3.1 Fly Through Volumetric Clouds at Altitude

**As a** player (P-23), **I want** ray-marched volumetric clouds with temporal reprojection that I
can fly through in aircraft gameplay, **so that** clouds look three-dimensional and solid rather
than flat skybox textures.

## US-2.7.3.2 Validate Cloud Temporal Reprojection Frame Count

**As an** engine tester (P-27), **I want** to verify that Switch uses 4-frame temporal reprojection,
desktop uses 2-frame, and high-end uses single-frame with temporal accumulation, **so that** cloud
quality and cost scale correctly per platform.

## US-2.7.4.1 See God Rays Streaming Through Forest Canopy

**As a** player (P-23), **I want** volumetric light shafts from the sun penetrating through tree
canopy and fog, **so that** forests feel atmospheric and dramatically lit.

## US-2.7.4.2 Profile God Ray Cost Between Screen-Space and Volumetric Modes

**As an** engine developer (P-26), **I want** to measure GPU time for screen-space radial blur
versus full froxel-based volumetric god rays, **so that** I can set the correct mode per platform
(screen-space on mobile/Switch, volumetric on desktop+).

## US-2.7.5.1 Add Quick Distance Fog to an Open-World Scene

**As a** environment artist (P-8), **I want** exponential and exponential-squared analytical fog
with height falloff, **so that** I can quickly add atmospheric depth to outdoor scenes without the
cost of full volumetric fog.

## US-2.7.5.2 Test Fog Combined With Volumetric Froxel System

**As an** engine tester (P-27), **I want** to enable analytical fog alongside froxel volumetrics and
verify they composite correctly without double-fogging, **so that** both systems work together as
expected.

## US-2.7.6.1 Author an FFT Ocean With Foam and Underwater Effects

**As a** environment artist (P-8), **I want** an FFT-based ocean surface with compute-generated
normals, Fresnel blending, foam at wave crests, and optional underwater effects, **so that** I can
create a realistic ocean environment with full artistic control.

## US-2.7.6.2 Validate Simplified Gerstner Wave Fallback on Mobile

**As an** engine tester (P-27), **I want** to verify that mobile uses simplified Gerstner wave sums
instead of FFT with no planar reflections, **so that** water rendering stays within mobile GPU
budgets.

## US-2.7.6.3 See Reflections on Water Update With Scene Changes

**As a** player (P-23), **I want** water reflections (SSR or RT) to update when nearby objects move,
**so that** the ocean surface shows current scene content rather than stale reflection data.

## US-2.7.7.1 Import and Render OpenVDB Volumes for Fire and Smoke

**As a** effects artist (P-12), **I want** to import sparse volume data (OpenVDB) and render it with
full lighting, shadows, and volumetric BSDF scattering, **so that** pre-simulated fire, smoke, and
cloud volumes look cinematic in real time.

## US-2.7.7.2 Validate Volume LOD Fallback on Switch

**As an** engine tester (P-27), **I want** to verify that Switch uses billboard/impostor fallback
for distant volumes and simplified 32^3 grids for near volumes, **so that** OpenVDB rendering
degrades gracefully on Switch hardware.

## US-2.7.8.1 Author Production-Quality Clouds With Voxel Representation

**As a** environment artist (P-8), **I want** full 3D voxel volumetric clouds with SDF ray-march
acceleration and fluid-simulation-based modeling, **so that** clouds have realistic dark edges,
inner glow, and support fly-through gameplay.

## US-2.7.8.2 Test Voxel Cloud Fallback to Noise-Based Clouds on Switch

**As an** engine tester (P-27), **I want** to verify that voxel clouds are disabled on Switch and
fall back to noise-based volumetric clouds (F-2.7.3), **so that** Switch still renders clouds
without the voxel system overhead.

## US-2.7.9.1 Control Breaking Wave Shape With Art-Directable Guide Curves

**As a** environment artist (P-8), **I want** to edit guide curves that control wave shape, timing,
and deformation using Houdini-baked 2D deformation textures, **so that** shoreline breaking waves
match the artistic vision for each coastal environment.

## US-2.7.9.2 Validate Breaking Wave Instance Count Per Platform

**As an** engine tester (P-27), **I want** to confirm that mobile disables breaking waves, Switch
supports max 2 active instances with reduced vertex density, and desktop is fully configurable, **so
that** wave complexity scales with hardware capability.

## US-2.7.10.1 Author a Full Weather Cycle (Clear to Rain to Thunderstorm)

**As a** game designer (P-5), **I want** a weather state machine with configurable transition
durations between states (clear, overcast, rain, thunderstorm, snow, dust storm), **so that** I can
script dynamic weather cycles that drive clouds, fog, precipitation, and material wetness.

## US-2.7.10.2 See Puddles Form and Surfaces Darken During Rain

**As a** player (P-23), **I want** rain to darken surfaces, form puddles in terrain concavities, and
increase vegetation animation intensity, **so that** weather feels like it affects the world rather
than being a cosmetic overlay.

## US-2.7.10.3 Validate Weather-Driven Material Wetness on Desktop vs Mobile

**As an** engine tester (P-27), **I want** to trigger a rain state and verify that desktop shows
full puddle rendering and material wetness while mobile only drives fog and lighting changes, **so
that** weather effects scale per platform without visual errors.
