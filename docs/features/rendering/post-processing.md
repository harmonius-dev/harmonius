# 2.9 — Post-Processing

### F-2.9.1 Bloom

Bright-source glow simulating camera lens diffraction and scattering. A threshold pass extracts
pixels above a luminance cutoff, followed by a multi-pass Gaussian or dual-kawase downscale/upscale
blur chain. The blurred result is composited additively over the scene.

- **Requirements:** R-2.9.1
- **Dependencies:** None
- **Platform notes:** Mobile: 3-pass dual-kawase at quarter-res; max 3 blur iterations. Switch:
  4-pass Gaussian at half-res. Desktop: full multi-pass chain (6+ iterations). High-end: full
  quality with wide kernel for cinematic bloom.

### F-2.9.2 Depth of Field (Cinematic)

Focal-plane blur driven by real-world camera parameters (aperture, focal length, focus distance). A
gather-based circular bokeh filter produces near and far field blur with proper occlusion handling.
Supports configurable bokeh shape and size.

- **Requirements:** R-2.9.2
- **Dependencies:** None
- **Platform notes:** Mobile: disabled by default; optional lightweight Gaussian DOF at quarter-res
  for cutscenes. Switch: simplified separable DOF at half-res. Desktop: full gather-based circular
  bokeh. High-end: full quality with shaped bokeh and near/far occlusion.

### F-2.9.3 Motion Blur

Per-pixel velocity-based blur using a full-screen velocity buffer written during the geometry pass.
A post-process pass samples along each pixel's motion vector to simulate camera shutter exposure.
Supports both camera and per-object motion contribution.

- **Requirements:** R-2.9.3
- **Dependencies:** None
- **Platform notes:** Mobile: disabled; velocity buffer bandwidth and per-pixel sampling too costly.
  Switch: camera-only motion blur (no per-object), 4 samples, half-res. Desktop: full per-pixel
  blur, 8 samples. High-end: 16 samples with tile-max optimization.

### F-2.9.4 Auto Exposure / Eye Adaptation

Automatic scene brightness adjustment simulating human eye adaptation. A compute pass builds a
luminance histogram from the scene, and a temporal smoothing filter adjusts the exposure value
(EV100) between configurable min/max bounds over time.

- **Requirements:** R-2.9.4
- **Dependencies:** None
- **Platform notes:** Mobile: average luminance (no histogram) using parallel reduction at
  eighth-res. Switch: 32-bin histogram at quarter-res. Desktop: 64-bin histogram. High-end: 128-bin
  histogram at full resolution.

### F-2.9.5 Tonemapping and Color Grading

HDR-to-LDR tone mapping with an ACES-compliant filmic curve, followed by color correction controls:
white balance, lift/gamma/gain for shadows/midtones/highlights, saturation, contrast, and color
lookup table (LUT) application. Supports HDR display output.

- **Requirements:** R-2.9.5
- **Dependencies:** F-2.9.4
- **Platform notes:** Mobile: combined tonemap + color grade in single pass; 16x16x16 LUT. Switch:
  same as mobile; HDR output not available. Desktop: 32x32x32 LUT; HDR10/Dolby Vision output.
  High-end: 64x64x64 LUT; full HDR pipeline.

### F-2.9.6 Film Grain

Procedural or texture-based grain overlay simulating photographic film noise. Grain intensity, size,
and response to luminance are configurable for cinematic texture.

- **Requirements:** R-2.9.6
- **Dependencies:** None
- **Platform notes:** All platforms: full quality. Lightweight single-pass effect with negligible
  cost. Combined into final composite pass on mobile to avoid extra bandwidth.

### F-2.9.7 Chromatic Aberration

Simulates RGB color channel separation at image edges caused by lens imperfections. Per-channel UV
offsets are applied radially from the screen center with configurable intensity and start offset.

- **Requirements:** R-2.9.7
- **Dependencies:** None
- **Platform notes:** All platforms: full quality. Lightweight single-pass effect. Combined into
  final composite pass on mobile to avoid extra bandwidth.

### F-2.9.8 Lens Flare

Image-based lens flare simulation from bright sources. A threshold pass identifies bright pixels,
then ghost and halo artifacts are generated via scaled and rotated resampling to simulate internal
lens reflections.

- **Requirements:** R-2.9.8
- **Dependencies:** None
- **Platform notes:** Mobile: simplified sprite-based flare (no ghost/halo resampling); max 2 flare
  sources. Switch: 2 ghost layers; max 4 sources. Desktop: full ghost/halo chain. High-end: full
  quality; see also F-2.5.13 for RT lens flare.

### F-2.9.9 Vignette

Radial darkening of frame edges and corners simulating optical lens falloff. Intensity is controlled
by a power curve from screen center to edge.

- **Requirements:** R-2.9.9
- **Dependencies:** None
- **Platform notes:** All platforms: full quality. Negligible cost. Combined into final composite
  pass on mobile.

### F-2.9.10 Post-Process Materials

User-defined fullscreen shader effects executed as post-process passes. Custom material graphs can
read scene textures (depth, stencil, G-buffer channels, velocity) and output arbitrary color
transformations for effects like outlines, distortion, and stylization.

- **Requirements:** R-2.9.10
- **Dependencies:** F-2.4.3
- **Platform notes:** Mobile: max 2 custom post-process passes; depth-only scene input (no G-buffer
  channels). Switch: max 4 passes; depth and normal inputs. Desktop: unlimited passes with all scene
  inputs. High-end: same as desktop.

### F-2.9.11 Local Exposure

Per-region exposure adjustment allowing independent brightness adaptation across the frame. A
downsampled luminance grid drives per-tile exposure compensation, preventing detail loss in
high-contrast scenes with both bright and dark regions.

- **Requirements:** R-2.9.11
- **Dependencies:** F-2.9.4
- **Platform notes:** Mobile: disabled; global auto-exposure only. Switch: 4x4 tile grid (coarse).
  Desktop: 16x16 tile grid. High-end: 32x32 tile grid with smooth blending between adjacent tiles.

### F-2.9.12 Panini Projection

Post-process pixel displacement correcting perspective distortion at wide field-of-view angles.
Pixels are remapped using panini projection math to reduce straight-line warping at screen edges
while preserving center fidelity.

- **Requirements:** R-2.9.12
- **Dependencies:** None
- **Platform notes:** All platforms: full quality. Lightweight single-pass UV remap with negligible
  cost.

### F-2.9.13 Screen-Space Cavity and Curvature

Screen-space surface detail enhancement that darkens concavities and brightens convexities to reveal
micro-surface shape invisible to standard lighting. Inspired by Blender's viewport cavity effect and
the Unity "Screen Space Cavity & Curvature" asset by Jolly Theory. The effect operates in two
complementary modes:

1. **Curvature** (small-scale, pixel-radius): Samples the screen-space normal buffer at small
   offsets (1-4 pixels) in X and Y, computing the first-order normal difference `(N_center -
   N_offset)` per channel. Positive differences indicate ridges (convex edges); negative differences
   indicate valleys (concave edges). Multiple samples at varying offsets are weighted and summed.
   The result is a per-pixel scalar: values above 0.5 represent ridges, below 0.5 represent valleys.
2. **Cavity** (large-scale, world-radius): Uses the same normal-difference technique but samples at
   wider, world-space-scaled offsets (configurable radius in meters, converted to pixel offsets via
   the depth buffer). An optional multi-sample blur (2-4 passes at widths 2, 3, 7) softens the
   result. This produces ambient-occlusion-like darkening in crevices and brightening on exposed
   ridges at a configurable spatial scale.

Both modes read the G-buffer normal texture (or reconstruct normals from the depth buffer via
`dFdx`/`dFdy` partial derivatives when normals are unavailable). Output is a single-channel cavity
map composited multiplicatively over the lit scene color, with separate intensity controls for ridge
brightness and valley darkness. Supports distance-based and height-based fade to avoid artifacts at
extreme distances.

- **Requirements:** R-2.9.13
- **Dependencies:** F-2.4.2 (G-Buffer normals) or depth buffer with normal reconstruction
- **Platform notes:** Mobile: curvature-only mode at half-res with 4 samples; cavity disabled.
  Switch: curvature at native res, cavity at half-res with 8 samples. Desktop: both modes at native
  res, 16 samples. High-end: full quality with multi-pass cavity blur.

### F-2.9.14 Post-Process Graph Editor

A visual, node-based graph editor for authoring custom post-process effect chains without writing
shader code. Users compose full-screen image processing pipelines by connecting typed nodes in a
directed acyclic graph. The graph compiles to an optimized sequence of GPU compute dispatches
executed within the post-processing pipeline. Built on the universal logic graph runtime (F-15.8.1)
with a specialized node library for image processing operations.

**Node categories:**

| Category | Nodes | Description |
|----------|-------|-------------|
| Input | Scene Color, Scene Depth, Scene Normals, Velocity, Stencil, G-Buffer Channel, Custom Texture, Time, Resolution | Read render targets and per-frame constants |
| Filter | Gaussian Blur, Box Blur, Kawase Blur, Bilateral Blur, Sharpen, Median, Sobel Edge, Laplacian, Kuwahara | Spatial convolution and filtering operations |
| Color | Brightness/Contrast, Hue/Saturation, Levels, Curves, Channel Mixer, Color Remap, LUT Apply, Threshold, Posterize | Per-pixel color manipulation |
| Math | Add, Subtract, Multiply, Lerp, Clamp, Remap, Step, Smoothstep, Abs, Min, Max, Dot, Cross, Normalize | Scalar and vector arithmetic |
| Blend | Normal, Multiply, Screen, Overlay, Soft Light, Add, Subtract, Difference, Linear Dodge | Porter-Duff and Photoshop-style blend modes |
| Transform | UV Offset, UV Scale, UV Rotate, Radial Warp, Barrel Distortion, Pixelate, Tile | UV coordinate manipulation |
| Mask | Circle, Rectangle, Gradient (Linear/Radial), Depth Threshold, Normal Threshold, Stencil Mask, Screen Position | Procedural mask generation for selective application |
| Sample | Downsample, Upsample, Mip Sample, Custom Kernel, Gather (NxN) | Resolution control and multi-tap sampling |
| Utility | Preview, Split Screen, Side-by-Side Compare, Debug Value Display | Development and debugging aids |
| Output | Final Color, Intermediate Target (named, typed, sized) | Write to output or intermediate render targets |

**Multi-pass support:** Graphs can declare intermediate render targets as named, typed, and sized
resources. An upstream subgraph writes to an intermediate target; downstream nodes read from it. The
graph compiler allocates transient resources via the render graph (F-2.2.3) and inserts barriers
automatically (F-2.2.5).

**Execution model:** The compiled graph produces a sequence of fullscreen compute dispatches
injected into the post-processing pipeline at a configurable insertion point (before tonemapping,
after tonemapping, or at a custom slot). Each dispatch corresponds to one or more fused nodes. The
compiler performs dead-node elimination, constant folding, and pass merging (combining compatible
single-pass nodes into one dispatch).

**Integration with existing systems:**

- Compiles through the shader graph HLSL pipeline (F-15.8.5b)
- Graphs are assets managed by the asset database (F-12.5.1)
- Parameters are exposed as typed slots with per-instance overrides and ECS data binding (same
  parameter model as effect graphs, F-11.6.3)
- Live preview in the editor viewport with hot reload on graph changes (F-12.4.1)

- **Requirements:** R-2.9.14
- **Dependencies:** F-15.8.1 (Logic Graph Runtime), F-15.8.5a (Shader Graph Core), F-2.2.1 (Render
  Graph), F-2.9.10 (Post-Process Materials)
- **Platform notes:** Mobile: max 4 nodes per graph; intermediate targets at half-res; no
  multi-pass. Switch: max 8 nodes; half-res intermediates. Desktop: unlimited nodes and passes.
  High-end: same as desktop with full-res intermediates.
