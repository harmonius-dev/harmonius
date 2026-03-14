# Post-Processing

## F-2.5.1 Bloom

Bright-source glow simulating camera lens diffraction and scattering. A threshold pass extracts
pixels above a luminance cutoff, followed by a multi-pass Gaussian or dual-kawase downscale/upscale
blur chain. The blurred result is composited additively over the scene.

## F-2.5.2 Depth of Field (Cinematic)

Focal-plane blur driven by real-world camera parameters (aperture, focal length, focus distance). A
gather-based circular bokeh filter produces near and far field blur with proper occlusion handling.
Supports configurable bokeh shape and size.

## F-2.5.3 Motion Blur

Per-pixel velocity-based blur using a full-screen velocity buffer written during the geometry pass. A
post-process pass samples along each pixel's motion vector to simulate camera shutter exposure.
Supports both camera and per-object motion contribution.

## F-2.5.4 Auto Exposure / Eye Adaptation

Automatic scene brightness adjustment simulating human eye adaptation. A compute pass builds a
luminance histogram from the scene, and a temporal smoothing filter adjusts the exposure value
(EV100) between configurable min/max bounds over time.

## F-2.5.5 Tonemapping and Color Grading

HDR-to-LDR tone mapping with an ACES-compliant filmic curve, followed by color correction controls:
white balance, lift/gamma/gain for shadows/midtones/highlights, saturation, contrast, and color
lookup table (LUT) application. Supports HDR display output.

## F-2.5.6 Film Grain

Procedural or texture-based grain overlay simulating photographic film noise. Grain intensity, size,
and response to luminance are configurable for cinematic texture.

## F-2.5.7 Chromatic Aberration

Simulates RGB color channel separation at image edges caused by lens imperfections. Per-channel UV
offsets are applied radially from the screen center with configurable intensity and start offset.

## F-2.5.8 Lens Flare

Image-based lens flare simulation from bright sources. A threshold pass identifies bright pixels,
then ghost and halo artifacts are generated via scaled and rotated resampling to simulate internal
lens reflections.

## F-2.5.9 Vignette

Radial darkening of frame edges and corners simulating optical lens falloff. Intensity is controlled
by a power curve from screen center to edge.

## F-2.5.10 Post-Process Materials

User-defined fullscreen shader effects executed as post-process passes. Custom material graphs can
read scene textures (depth, stencil, G-buffer channels, velocity) and output arbitrary color
transformations for effects like outlines, distortion, and stylization.

## F-2.5.11 Local Exposure

Per-region exposure adjustment allowing independent brightness adaptation across the frame. A
downsampled luminance grid drives per-tile exposure compensation, preventing detail loss in
high-contrast scenes with both bright and dark regions.

## F-2.5.12 Panini Projection

Post-process pixel displacement correcting perspective distortion at wide field-of-view angles.
Pixels are remapped using panini projection math to reduce straight-line warping at screen edges
while preserving center fidelity.
