# Advanced Rendering (Ray Tracing and GI)

## F-2.5.1 Acceleration Structure Management (BLAS/TLAS)

Bottom-level acceleration structure (BLAS) building from meshlet geometry with post-build compaction
to reduce memory. Top-level acceleration structure (TLAS) rebuilt or refit each frame for dynamic
scenes.

- **Requirements:** [R-3.2.3](../../requirements/3-nonfunctional/3.2-hardware.md) RT soft-gated,
  [R-3.1.4](../../requirements/3-nonfunctional/3.1-performance.md) feature gates

## F-2.5.2 Ray Traced Reflections (Hybrid SSR + RT)

Hybrid reflection system combining screen-space ray marching (SSR) for nearby surfaces with hardware
ray tracing for off-screen and rough reflections. A roughness threshold determines when RT rays
supplement SSR. Temporal denoising stabilizes the output.

- **Requirements:** [R-3.2.3](../../requirements/3-nonfunctional/3.2-hardware.md) RT soft-gated,
  [R-3.1.9](../../requirements/3-nonfunctional/3.1-performance.md) quality tier fallbacks

## F-2.5.3 Ray Traced Indirect Lighting

Secondary diffuse ray tracing from surface positions to collect one-bounce indirect illumination.
Output feeds the global illumination system.

- **Requirements:** [R-3.2.3](../../requirements/3-nonfunctional/3.2-hardware.md) RT soft-gated

## F-2.5.4 Real-Time Global Illumination (DDGI)

Dynamic Diffuse Global Illumination using a probe grid. Each probe casts rays via the TLAS,
accumulates irradiance and visibility into octahedral atlas textures with temporal hysteresis.
Surfaces sample nearby probes for diffuse indirect lighting, producing color bleeding and ambient
fill.

- **Requirements:** [R-3.2.3](../../requirements/3-nonfunctional/3.2-hardware.md) RT soft-gated,
  [R-3.1.4](../../requirements/3-nonfunctional/3.1-performance.md) budget gates

## F-2.5.5 Path Tracing (Reference Renderer)

Progressive unbiased hardware-accelerated renderer for offline-quality output. Traces full light
transport paths (diffuse, specular, transmission) with configurable samples per pixel and max
bounces. Supports all material types, volumetrics, and sky atmosphere. Used as a ground-truth
reference and for cinematic rendering via the render queue.

- **Requirements:** [R-3.2.3](../../requirements/3-nonfunctional/3.2-hardware.md) RT soft-gated

## F-2.5.6 Ray Traced Subsurface Transmission

Volumetric scattering computation through translucent media during shadow ray evaluation. When a
shadow ray hits a subsurface material, light transport through the material's thickness is evaluated
using the subsurface profile, enabling realistic light transmission through skin, wax, and thin
objects.

- **Requirements:** [R-3.2.3](../../requirements/3-nonfunctional/3.2-hardware.md) RT soft-gated

## F-2.5.7 Surfel-Based Global Illumination

Dynamic surfel-based GI requiring no precomputation. Scene geometry is discretized into surface
elements (surfels) from the G-Buffer each frame. A clipmap probe system with octahedral
representation caches irradiance and depth. Hardware ray tracing traces rays from surfels to compute
incoming radiance with ray guiding and ray binning for efficient integration. Multi-scale mean
estimation provides temporal hysteresis. Separate ray-traced probes handle character lighting. Scales
to environments of arbitrary size with predictable performance.

- **Requirements:** [R-3.2.3](../../requirements/3-nonfunctional/3.2-hardware.md) RT soft-gated,
  [R-3.1.4](../../requirements/3-nonfunctional/3.1-performance.md) budget gates

## F-2.5.8 ReSTIR Sampling Framework

Reservoir-based spatiotemporal importance resampling (ReSTIR) framework for efficient stochastic
light sampling. ReSTIR DI handles direct illumination from thousands of lights simultaneously with
physically correct soft shadows from every source. ReSTIR GI enables screen-space resampling of
secondary surfaces for multi-bounce indirect lighting. Reservoirs are maintained per pixel across
frames, with spatial and temporal reuse for rapid convergence.

- **Requirements:** [R-3.2.3](../../requirements/3-nonfunctional/3.2-hardware.md) RT soft-gated

## F-2.5.9 Real-Time Production Path Tracing

Tiered real-time path tracing for production game rendering, distinct from the offline reference
renderer. Supports configurable quality tiers from direct-RT-only through multi-bounce full path
tracing with RT reflections, AO, GI, and refractions. After the final path-tracing bounce, the
engine falls back to rasterized GI rather than terminating to black. Paired with neural denoising and
temporal upscaling for real-time frame rates.

- **Requirements:** [R-3.2.3](../../requirements/3-nonfunctional/3.2-hardware.md) RT soft-gated,
  [R-3.1.9](../../requirements/3-nonfunctional/3.1-performance.md) quality tier fallbacks

## F-2.5.10 Opacity Micromaps

Acceleration structure annotation encoding per-micro-triangle opacity classification (opaque,
transparent, or unknown) for alpha-tested geometry. The hardware skips any-hit shader invocations for
micro-triangles classified as fully opaque or fully transparent, significantly accelerating ray
intersection testing for vegetation and hair.

- **Requirements:** [R-3.2.3](../../requirements/3-nonfunctional/3.2-hardware.md) RT soft-gated

## F-2.5.11 Shader Execution Reordering

Hardware-assisted reordering of ray tracing shader invocations to improve execution coherency. After
intersection testing scatters rays to diverse material shaders, SER groups similar shader invocations
into coherent wavefronts before execution, reducing divergence and improving GPU occupancy.

- **Requirements:** [R-3.2.3](../../requirements/3-nonfunctional/3.2-hardware.md) RT soft-gated

## F-2.5.12 Neural Denoising (Ray Reconstruction)

AI-trained neural network denoiser that replaces hand-tuned spatiotemporal denoisers for ray-traced
and path-traced output. Operates on noisy per-pixel radiance with motion vectors and depth, producing
temporally stable results with fewer artifacts than traditional bilateral/wavelet filters. Supports
fallback to conventional NRD denoiser algorithms when neural hardware is unavailable.

- **Requirements:** [R-3.2.3](../../requirements/3-nonfunctional/3.2-hardware.md) RT soft-gated

## F-2.5.13 RT Lens Flare

Physically accurate ray-traced lens flare simulation. Lens elements are modeled as refractive
surfaces; rays from bright scene sources are traced through the lens system to produce ghost and halo
artifacts matching real optical behavior, replacing the image-based approximation with ground-truth
results.

- **Requirements:** [R-3.2.3](../../requirements/3-nonfunctional/3.2-hardware.md) RT soft-gated

## F-2.5.14 Voxel-Based Global Illumination

Rasterization-only GI system that voxelizes the world, storing albedo, normals, and translucency
into a 3D voxel cache. The cache is re-lit for varying times of day without re-voxelization. Combined
with irradiance volumes blended across time-of-day transitions and improved cubemap reflections, this
system provides convincing indirect lighting entirely through rasterization without hardware ray
tracing.

## F-2.5.15 Neural Radiance Cache

Neural network trained at runtime to estimate indirect radiance at arbitrary scene points. The
network terminates the majority of path-traced paths early (after 2-3 bounces) and predicts the
remaining light transport, using a small percentage of full-length paths for ongoing training.
Produces less noise than regular path tracing and supports entirely dynamic scenes.

- **Requirements:** [R-3.2.3](../../requirements/3-nonfunctional/3.2-hardware.md) RT soft-gated

## F-2.5.16 Stochastic Screen-Space Reflections

Monte Carlo importance-sampled screen-space reflections with spatiotemporal filtering. Rays are
importance-sampled from the microfacet BRDF to reproduce specular elongation and spatially-varying
roughness. Traced at half resolution with adjacent pixel ray reuse and dedicated spatial and temporal
filters for denoising.
