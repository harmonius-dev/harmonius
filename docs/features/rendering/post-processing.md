# 2.9 — Post-Processing

### F-2.9.1 Bloom

Bright-source glow simulating camera lens diffraction and scattering. A threshold pass extracts
pixels above a luminance cutoff, followed by a multi-pass Gaussian or dual-kawase downscale/upscale
blur chain. The blurred result is composited additively over the scene.

- **Requirements:** R-2.9.1
- **Dependencies:** None
- **Platform notes:** Mobile: 3-pass dual-kawase at quarter-res; max 3 blur iterations.
  Switch: 4-pass Gaussian at half-res. Desktop: full multi-pass chain (6+ iterations).
  High-end: full quality with wide kernel for cinematic bloom.

### F-2.9.2 Depth of Field (Cinematic)

Focal-plane blur driven by real-world camera parameters (aperture, focal length, focus distance). A
gather-based circular bokeh filter produces near and far field blur with proper occlusion handling.
Supports configurable bokeh shape and size.

- **Requirements:** R-2.9.2
- **Dependencies:** None
- **Platform notes:** Mobile: disabled by default; optional lightweight Gaussian DOF at
  quarter-res for cutscenes. Switch: simplified separable DOF at half-res. Desktop:
  full gather-based circular bokeh. High-end: full quality with shaped bokeh and
  near/far occlusion.

### F-2.9.3 Motion Blur

Per-pixel velocity-based blur using a full-screen velocity buffer written during the geometry pass. A
post-process pass samples along each pixel's motion vector to simulate camera shutter exposure.
Supports both camera and per-object motion contribution.

- **Requirements:** R-2.9.3
- **Dependencies:** None
- **Platform notes:** Mobile: disabled; velocity buffer bandwidth and per-pixel sampling
  too costly. Switch: camera-only motion blur (no per-object), 4 samples, half-res.
  Desktop: full per-pixel blur, 8 samples. High-end: 16 samples with tile-max
  optimization.

### F-2.9.4 Auto Exposure / Eye Adaptation

Automatic scene brightness adjustment simulating human eye adaptation. A compute pass builds a
luminance histogram from the scene, and a temporal smoothing filter adjusts the exposure value
(EV100) between configurable min/max bounds over time.

- **Requirements:** R-2.9.4
- **Dependencies:** None
- **Platform notes:** Mobile: average luminance (no histogram) using parallel reduction
  at eighth-res. Switch: 32-bin histogram at quarter-res. Desktop: 64-bin histogram.
  High-end: 128-bin histogram at full resolution.

### F-2.9.5 Tonemapping and Color Grading

HDR-to-LDR tone mapping with an ACES-compliant filmic curve, followed by color correction controls:
white balance, lift/gamma/gain for shadows/midtones/highlights, saturation, contrast, and color
lookup table (LUT) application. Supports HDR display output.

- **Requirements:** R-2.9.5
- **Dependencies:** F-2.9.4
- **Platform notes:** Mobile: combined tonemap + color grade in single pass; 16x16x16
  LUT. Switch: same as mobile; HDR output not available. Desktop: 32x32x32 LUT;
  HDR10/Dolby Vision output. High-end: 64x64x64 LUT; full HDR pipeline.

### F-2.9.6 Film Grain

Procedural or texture-based grain overlay simulating photographic film noise. Grain intensity, size,
and response to luminance are configurable for cinematic texture.

- **Requirements:** R-2.9.6
- **Dependencies:** None
- **Platform notes:** All platforms: full quality. Lightweight single-pass effect with
  negligible cost. Combined into final composite pass on mobile to avoid extra
  bandwidth.

### F-2.9.7 Chromatic Aberration

Simulates RGB color channel separation at image edges caused by lens imperfections. Per-channel UV
offsets are applied radially from the screen center with configurable intensity and start offset.

- **Requirements:** R-2.9.7
- **Dependencies:** None
- **Platform notes:** All platforms: full quality. Lightweight single-pass effect.
  Combined into final composite pass on mobile to avoid extra bandwidth.

### F-2.9.8 Lens Flare

Image-based lens flare simulation from bright sources. A threshold pass identifies bright pixels,
then ghost and halo artifacts are generated via scaled and rotated resampling to simulate internal
lens reflections.

- **Requirements:** R-2.9.8
- **Dependencies:** None
- **Platform notes:** Mobile: simplified sprite-based flare (no ghost/halo resampling);
  max 2 flare sources. Switch: 2 ghost layers; max 4 sources. Desktop: full
  ghost/halo chain. High-end: full quality; see also F-2.5.13 for RT lens flare.

### F-2.9.9 Vignette

Radial darkening of frame edges and corners simulating optical lens falloff. Intensity is controlled
by a power curve from screen center to edge.

- **Requirements:** R-2.9.9
- **Dependencies:** None
- **Platform notes:** All platforms: full quality. Negligible cost. Combined into final
  composite pass on mobile.

### F-2.9.10 Post-Process Materials

User-defined fullscreen shader effects executed as post-process passes. Custom material graphs can
read scene textures (depth, stencil, G-buffer channels, velocity) and output arbitrary color
transformations for effects like outlines, distortion, and stylization.

- **Requirements:** R-2.9.10
- **Dependencies:** F-2.4.3
- **Platform notes:** Mobile: max 2 custom post-process passes; depth-only scene input
  (no G-buffer channels). Switch: max 4 passes; depth and normal inputs. Desktop:
  unlimited passes with all scene inputs. High-end: same as desktop.

### F-2.9.11 Local Exposure

Per-region exposure adjustment allowing independent brightness adaptation across the frame. A
downsampled luminance grid drives per-tile exposure compensation, preventing detail loss in
high-contrast scenes with both bright and dark regions.

- **Requirements:** R-2.9.11
- **Dependencies:** F-2.9.4
- **Platform notes:** Mobile: disabled; global auto-exposure only. Switch: 4x4 tile
  grid (coarse). Desktop: 16x16 tile grid. High-end: 32x32 tile grid with smooth
  blending between adjacent tiles.

### F-2.9.12 Panini Projection

Post-process pixel displacement correcting perspective distortion at wide field-of-view angles.
Pixels are remapped using panini projection math to reduce straight-line warping at screen edges
while preserving center fidelity.

- **Requirements:** R-2.9.12
- **Dependencies:** None
- **Platform notes:** All platforms: full quality. Lightweight single-pass UV remap with
  negligible cost.
