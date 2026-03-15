# User Stories -- 2.5 Advanced Rendering (Ray Tracing and GI)

## US-2.5.1.1 Build and Compact Acceleration Structures for RT Scenes

**As an** engine developer (P-26), **I want** to build BLAS from meshlet geometry with post-build
compaction and rebuild TLAS each frame for dynamic objects, **so that** RT features have an
up-to-date acceleration structure with minimal VRAM waste.

## US-2.5.1.2 Validate BLAS Budget Limits on Mobile and Switch

**As an** engine tester (P-27), **I want** to verify that BLAS construction respects per-platform
memory budgets (limited on Apple A17+/M3+ and Switch 2 Ampere), **so that** RT scenes do not exceed
mobile or Switch VRAM allocations.

## US-2.5.2.1 See Reflections of Off-Screen Objects on Glossy Surfaces

**As a** player (P-23), **I want** hybrid reflections that combine screen-space rays with hardware
RT for off-screen content, **so that** mirrors and polished floors show correct reflections even
when the reflected object is not on screen.

## US-2.5.2.2 Profile SSR-to-RT Roughness Threshold Handoff

**As an** engine developer (P-26), **I want** to measure the cost of RT reflection rays at varying
roughness thresholds, **so that** I can tune the SSR-to-RT handoff point to balance quality and GPU
budget per platform.

## US-2.5.2.3 Verify Reflection Probe Fallback When SSR and RT Are Disabled

**As an** engine tester (P-27), **I want** to disable both SSR and RT and confirm that reflection
probes provide correct fallback reflections on all surfaces, **so that** mobile devices without SSR
still show plausible reflections.

## US-2.5.3.1 See Color Bleeding From Colored Walls Onto Nearby Objects

**As a** player (P-23), **I want** one-bounce indirect diffuse lighting that bleeds wall color onto
adjacent surfaces, **so that** environments feel naturally lit with warm and cool tones rather than
flat ambient.

## US-2.5.3.2 Test RT Indirect Lighting Temporal Accumulation

**As an** engine developer (P-26), **I want** to verify that 0.25 spp RT indirect lighting
accumulates over 8-16 frames to produce stable noise-free GI, **so that** indirect lighting
converges quickly enough for real-time use.

## US-2.5.4.1 Preview Dynamic GI That Updates When Lights Move

**As a** environment artist (P-8), **I want** DDGI probes that recast rays and update irradiance
when I reposition lights in the editor, **so that** indirect lighting responds to light changes in
real time without rebaking.

## US-2.5.4.2 Validate DDGI Probe Density Scaling Per Platform

**As an** engine tester (P-27), **I want** to verify that desktop uses 8m probe spacing with 64
rays/probe and high-end uses 4m/128+ rays, **so that** GI quality scales correctly with hardware
capability.

## US-2.5.5.1 Render a Ground-Truth Reference Image for Lighting Comparison

**As a** technical artist (P-13), **I want** a progressive path tracer that renders offline- quality
images with full light transport and all material types, **so that** I can compare real-time
rendering against a reference to identify approximation artifacts.

## US-2.5.5.2 Test Path Tracing Produces Correct Energy Conservation

**As an** engine tester (P-27), **I want** to render a white-furnace test scene with the path tracer
and verify that energy is conserved (no darkening or brightening over bounces), **so that** the
reference renderer produces physically accurate results.

## US-2.5.6.1 See Light Glow Through Character Ears and Fingers

**As a** player (P-23), **I want** ray-traced subsurface transmission through thin body parts like
ears, fingers, and nostrils when backlit, **so that** characters look lifelike in strong directional
lighting.

## US-2.5.6.2 Verify RT SSS Fallback to Preintegrated LUT on Non-RT Hardware

**As an** engine tester (P-27), **I want** to disable RT and confirm that subsurface scattering
falls back to preintegrated LUT on mobile and Switch without visual artifacts, **so that** skin
rendering degrades gracefully.

## US-2.5.7.1 Light Dynamic Scenes Without Prebaked GI

**As a** environment artist (P-8), **I want** surfel-based GI that discretizes scene geometry from
the G-buffer each frame without precomputation, **so that** fully dynamic environments with
destructible objects receive correct indirect lighting.

## US-2.5.7.2 Profile Surfel GI Clipmap Level Scaling

**As an** engine developer (P-26), **I want** to measure per-frame GPU cost of surfel GI at 2
clipmap levels (desktop) versus 4+ levels (high-end), **so that** I can validate that multi-scale
surfel density meets performance targets per tier.

## US-2.5.8.1 Light Scenes With Thousands of Lights Using ReSTIR

**As a** environment artist (P-8), **I want** ReSTIR DI to sample thousands of lights with
physically correct soft shadows from every source, **so that** I can author dense light environments
(concert halls, cityscapes) without per-light shadow cost.

## US-2.5.8.2 Validate ReSTIR Reservoir Memory Usage on Desktop

**As an** engine developer (P-26), **I want** to measure per-pixel reservoir memory footprint during
ReSTIR DI+GI and confirm it fits within a 2 GB render target budget on desktop, **so that** ReSTIR
operates within VRAM constraints.

## US-2.5.9.1 Play With Real-Time Path-Traced Visuals on High-End Hardware

**As a** player (P-23), **I want** multi-bounce real-time path tracing with neural denoising for
reflections, AO, GI, and refractions, **so that** the game looks as close to cinematic quality as
possible during live gameplay.

## US-2.5.9.2 Verify Rasterized GI Fallback After Final Path-Tracing Bounce

**As an** engine tester (P-27), **I want** to confirm that the path tracer falls back to rasterized
GI after the final bounce rather than terminating to black, **so that** path-traced scenes maintain
ambient fill in shadowed areas.

## US-2.5.10.1 Accelerate RT for Alpha-Tested Vegetation With Opacity Micromaps

**As an** engine developer (P-26), **I want** opacity micromaps to annotate per-micro-triangle
opacity for vegetation and hair, **so that** hardware skips any-hit shader invocations for
classified triangles and RT performance improves by 20-40%.

## US-2.5.10.2 Verify OMM Support Detection on Ada and RDNA3+ GPUs

**As an** engine tester (P-27), **I want** to run on Ada Lovelace and RDNA3+ GPUs and confirm that
opacity micromaps are enabled automatically, **so that** supported hardware gets the RT performance
benefit without manual configuration.

## US-2.5.11.1 Improve RT Shader Coherency With Execution Reordering

**As an** engine developer (P-26), **I want** shader execution reordering to group similar RT shader
invocations into coherent wavefronts, **so that** divergent material shaders after ray intersection
run with improved GPU occupancy.

## US-2.5.11.2 Validate SER Activation on Supported Hardware

**As an** engine tester (P-27), **I want** to verify that SER activates on Ada Lovelace+ via
VK_NV_ray_tracing_invocation_reorder and is skipped on unsupported hardware, **so that** the feature
is conditionally enabled without driver errors.

## US-2.5.12.1 Denoise RT Output With Neural Ray Reconstruction

**As a** technical artist (P-13), **I want** a neural denoiser to replace hand-tuned spatiotemporal
filters for RT and path-traced output, **so that** noisy 1-spp rays produce temporally stable,
artifact-free images.

## US-2.5.12.2 Verify NRD Bilateral Fallback When Tensor Cores Unavailable

**As an** engine tester (P-27), **I want** to run on a GPU without Tensor Cores and confirm that the
engine falls back to NRD bilateral/wavelet denoising, **so that** RT denoising works on all desktop
GPUs regardless of neural hardware.

## US-2.5.13.1 See Physically Accurate Lens Flare From Bright Sources

**As a** player (P-23), **I want** ray-traced lens flare that simulates real lens element refraction
to produce ghost and halo artifacts, **so that** looking toward the sun or bright explosion produces
convincing optical effects.

## US-2.5.13.2 Test Image-Based Flare Fallback on Non-RT Platforms

**As an** engine tester (P-27), **I want** to disable RT and verify that lens flare falls back to
the image-based approximation (F-2.9.8), **so that** mobile and Switch still show lens flare
effects.

## US-2.5.14.1 Enable Rasterization-Only GI on Non-RT Hardware

**As a** environment artist (P-8), **I want** voxel-based GI that works without RT hardware by
voxelizing the scene and relighting the voxel cache for time-of-day changes, **so that** Switch and
older desktop GPUs still get convincing indirect lighting.

## US-2.5.14.2 Profile Voxel GI VRAM Usage Across Grid Resolutions

**As an** engine developer (P-26), **I want** to measure 3D texture VRAM for voxel GI at 64^3
(Switch), 128^3 (desktop), and 256^3 (high-end) grid resolutions, **so that** I can verify each tier
fits within its platform VRAM budget.

## US-2.5.15.1 Reduce Path-Traced Noise With Neural Radiance Cache

**As an** engine developer (P-26), **I want** a runtime-trained neural network that predicts
remaining light transport after 2-3 bounces, **so that** path-traced output has less noise with
fewer full-length paths required.

## US-2.5.15.2 Validate Neural Radiance Cache Hardware Requirements

**As an** engine tester (P-27), **I want** to confirm that the neural radiance cache requires Tensor
Core hardware and 12+ GB VRAM, and is disabled by default on all other configurations, **so that**
the feature does not activate on unsupported hardware.

## US-2.5.16.1 See Elongated Specular Reflections on Rough Surfaces

**As a** player (P-23), **I want** stochastic screen-space reflections importance-sampled from the
microfacet BRDF, **so that** rough metal surfaces show elongated, physically correct specular
reflections rather than sharp mirror images.

## US-2.5.16.2 Verify Stochastic SSR Resolution and Filter Quality

**As an** engine tester (P-27), **I want** to compare stochastic SSR at quarter-res (Switch),
half-res (desktop), and half-res with spatial+temporal denoiser (high-end), **so that** each
platform tier produces acceptable reflection quality.
