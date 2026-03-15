# R-2.6 — Anti-Aliasing and Upscaling Requirements

## R-2.6.1 Temporal Anti-Aliasing (TAA)

The engine **SHALL** provide temporal anti-aliasing that accumulates jittered sub-pixel samples
across frames using motion-vector-based reprojection, reducing geometric aliasing and specular
shimmer with history rejection clamping that limits ghosting to at most 1 frame of visible
trailing on disoccluded surfaces.

- **Derived from:** [F-2.6.1](../../features/rendering/anti-aliasing-upscaling.md)
- **Rationale:** TAA is the baseline temporal accumulation pass required by TSR and other
  temporal effects; ghosting control is critical for visual quality.
- **Verification:** Render a moving high-frequency geometric pattern (e.g., chain-link fence)
  and verify aliasing is reduced compared to no-AA; move the camera to disocclude surfaces
  and confirm ghosting artifacts resolve within 1 frame.

## R-2.6.2 Temporal Super Resolution (TSR)

The engine **SHALL** provide a platform-agnostic temporal upscaler implemented as compute
shaders that reconstructs output at 2x or higher resolution from a lower internal render
resolution while maintaining perceptual image quality within 1 dB PSNR of native resolution
on standard test scenes.

- **Derived from:** [F-2.6.2](../../features/rendering/anti-aliasing-upscaling.md)
- **Rationale:** Temporal upscaling is essential for achieving high output resolution on
  performance-constrained hardware without vendor SDK lock-in.
- **Verification:** Render a reference scene at native 4K and at 1080p with TSR upscaling
  to 4K; measure PSNR of the upscaled output against the native reference and confirm
  it is within 1 dB; verify quality/performance scalability controls produce measurable
  frame time differences.

## R-2.6.3 FXAA (Fast Approximate Anti-Aliasing)

The engine **SHALL** provide a single-pass spatial anti-aliasing post-process using
high-contrast edge detection and directional blending that completes within 0.3 ms at 1080p
on target hardware and requires no temporal history buffers.

- **Derived from:** [F-2.6.3](../../features/rendering/anti-aliasing-upscaling.md)
- **Rationale:** FXAA serves as a lightweight fallback for platforms or scenes where
  temporal methods are unsuitable.
- **Verification:** Enable FXAA on a scene with aliased edges (thin geometry, high-contrast
  boundaries) and confirm visible edge smoothing; profile GPU time and verify the pass
  completes within the 0.3 ms budget at 1080p.

## R-2.6.4 MSAA (Multi-Sample Anti-Aliasing)

The engine **SHALL** support hardware multi-sample anti-aliasing at 2x and 4x sample counts
exclusively in the forward rendering path, with per-sample evaluation during rasterization.

- **Derived from:** [F-2.6.4](../../features/rendering/anti-aliasing-upscaling.md)
- **Rationale:** MSAA provides high-quality geometric edge smoothing in forward rendering
  scenarios; it is incompatible with deferred G-buffer layouts.
- **Verification:** Enable 2x and 4x MSAA in forward rendering mode and confirm sub-pixel
  edge quality improvement on geometric silhouettes; verify that MSAA is not available when
  deferred rendering is active.

## R-2.6.5 Checkerboard Rendering

The engine **SHALL** support checkerboard rendering that rasterizes 50% of pixels per frame
in an alternating pattern and reconstructs the full-resolution image from current and
previous frame data, with a dedicated resolve filter that handles diagonal edges without
visible seam artifacts.

- **Derived from:** [F-2.6.5](../../features/rendering/anti-aliasing-upscaling.md)
- **Rationale:** Checkerboard rendering halves shading cost while maintaining near-native
  resolution, providing a scalable performance option.
- **Verification:** Render a test scene with checkerboard rendering enabled and compare
  against full-resolution rendering; verify no visible checkerboard or seam artifacts on
  diagonal edges; confirm shading workload is reduced by approximately 50%.

## R-2.6.6 Third-Party Upscaler Integration

The engine **SHALL** provide an abstraction layer that integrates vendor temporal upscalers
(NVIDIA DLSS, AMD FSR, Intel XeSS), accepting motion vectors, depth, and low-resolution
color as inputs and producing upscaled output, with automatic fallback to the built-in TSR
(R-2.6.2) when no vendor SDK is available.

- **Derived from:** [F-2.6.6](../../features/rendering/anti-aliasing-upscaling.md)
- **Rationale:** Vendor upscalers provide hardware-optimized quality and performance; the
  fallback ensures consistent behavior across all platforms.
- **Verification:** Test each vendor upscaler integration with its quality mode selection;
  remove vendor SDKs and confirm the engine falls back to TSR without errors or visual
  regression.

## R-2.6.7 Frame Generation

The engine **SHALL** support AI-driven frame interpolation that synthesizes intermediate
frames from motion vectors, depth, and two or more history frames to multiply the effective
frame rate, with support for multi-frame generation on capable hardware.

- **Derived from:** [F-2.6.7](../../features/rendering/anti-aliasing-upscaling.md)
- **Rationale:** Frame generation multiplies perceived smoothness without increasing
  rendering workload per real frame.
- **Verification:** Enable frame generation and measure displayed frame rate versus rendered
  frame rate; confirm the effective frame rate is at least 2x the rendered frame rate;
  inspect interpolated frames for disocclusion artifacts and verify they are below a
  perceptual threshold.

## R-2.6.8 Latency Reduction

The engine **SHALL** minimize end-to-end render pipeline latency by synchronizing CPU
submission timing with GPU back-pressure to reduce render queue depth, paired with frame
generation to maintain high frame rates without increasing input-to-display latency beyond
the non-frame-generation baseline.

- **Derived from:** [F-2.6.8](../../features/rendering/anti-aliasing-upscaling.md)
- **Rationale:** Frame generation introduces additional pipeline latency; active latency
  compensation is required to preserve input responsiveness.
- **Verification:** Measure input-to-photon latency with frame generation enabled and
  latency reduction active; confirm it does not exceed the latency measured without frame
  generation; verify render queue depth is reduced compared to unmanaged submission.

## Non-Functional Requirements

### NFR-2.6.1 Upscaler Image Quality

The built-in TSR upscaler **SHALL** produce output within 1 dB PSNR of native resolution on
standard test scenes when upscaling from half resolution (e.g., 1080p to 4K), and vendor upscalers
(DLSS, FSR, XeSS) **SHALL** meet or exceed this quality.

- **Rationale:** Upscaling quality must be nearly indistinguishable from native rendering to justify
  the performance gain.
- **Verification:** Render reference scenes at native 4K and at 1080p with TSR upscaling. Measure
  PSNR and verify it is within 1 dB of native.

### NFR-2.6.2 Anti-Aliasing Pass Cost

FXAA **SHALL** complete in under 0.3 ms and TAA **SHALL** complete in under 0.5 ms at 1080p on
target hardware.

- **Rationale:** Anti-aliasing is a mandatory per-frame pass; its cost must be bounded to preserve
  frame budget headroom.
- **Verification:** Profile FXAA and TAA individually at 1080p and verify they complete within their
  respective budgets.

### NFR-2.6.3 Frame Generation Latency

Frame generation **SHALL NOT** increase input-to-display latency beyond the baseline (no frame
generation) when latency reduction is active, and **SHALL** multiply the effective displayed frame
rate by at least 2x.

- **Rationale:** Frame generation must provide smoothness benefits without degrading input
  responsiveness.
- **Verification:** Measure input-to-photon latency with and without frame generation plus latency
  reduction. Verify the frame generation path does not exceed baseline latency. Verify displayed
  FPS is at least 2x rendered FPS.
