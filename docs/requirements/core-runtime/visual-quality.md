# Visual Quality

## R-3.5.1 Temporal Stability

All stochastic and temporally amortized effects (TAA, DDGI, volumetric clouds, RT denoising) SHALL
converge to a stable image within a bounded frame count and SHALL not exhibit visible flickering
under static camera and lighting conditions.

## R-3.5.2 Ray Trace Denoising

Ray-traced effects (reflections, shadows, GI, AO) SHALL apply spatial-temporal denoising to produce
visually acceptable results at the configured sample-per-pixel count. The denoiser SHALL preserve
edge sharpness and avoid over-blurring fine detail.

## R-3.5.3 Color Precision

The rendering pipeline SHALL maintain at least 10 bits of effective color precision per channel from
light accumulation through tone mapping. Banding-prone gradients (sky, fog, large flat surfaces)
SHALL use dithering or higher-precision intermediate targets.

## R-3.5.4 Shadow Bias Control

Shadow mapping SHALL provide per-cascade configurable constant depth bias and slope-scaled bias.
Default bias values SHALL eliminate self-shadowing artifacts (shadow acne) without introducing
visible light leaking (peter-panning) on reference test scenes.

## R-3.5.5 LOD Transition Smoothing

Geometry LOD transitions SHALL use dithered crossfade, screen-door transparency, or vertex morphing
to avoid visible popping. The transition range SHALL be configurable per asset.

- **Features:** [F-3.1.11](../../features/geometry-world/meshlet-pipeline.md) LOD blending and
  transitions

## R-3.5.6 Atmosphere LUT Quality

Precomputed atmosphere lookup tables SHALL be generated at sufficient resolution to avoid visible
banding in sky gradients. Default resolutions SHALL be: transmittance 256x64, multi-scattering
32x32, sky-view 192x108, aerial perspective 32x32x32.
