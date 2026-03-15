# 2.6 — Anti-Aliasing and Upscaling

### F-2.6.1 Temporal Anti-Aliasing (TAA)

Multi-frame jittered sample accumulation that blends the current frame with reprojected history
using motion vectors. Reduces geometric aliasing and specular shimmer. Temporal weighting clamps
reject disoccluded pixels to reduce ghosting.

- **Requirements:** R-2.6.1
- **Dependencies:** None
- **Platform notes:** Mobile: available but costly due to history buffer bandwidth;
  prefer FXAA as default. Switch: TAA at native resolution. Desktop/High-end: TAA
  as default AA method; full quality.

### F-2.6.2 Temporal Super Resolution (TSR)

Temporal upscaler rendering high-resolution output from a lower internal resolution (e.g., 1080p to
4K). Combines sub-pixel jitter accumulation with a learned or heuristic detail reconstruction
filter. A platform-agnostic compute shader implementation with quality/performance scalability
controls.

- **Requirements:** R-2.6.2
- **Dependencies:** F-2.6.1
- **Platform notes:** Mobile: heuristic TSR from 50% to native; quality mode only.
  Switch: TSR from 720p to 1080p docked; 540p to 720p handheld. Desktop: TSR from
  1080p to 4K; quality/balanced/performance modes. High-end: TSR to 8K output.

### F-2.6.3 FXAA (Fast Approximate Anti-Aliasing)

Single-pass spatial anti-aliasing using high-contrast edge detection and directional pixel blending.
Lightweight post-process with minimal performance cost and no temporal history dependency.

- **Requirements:** R-2.6.3
- **Dependencies:** None
- **Platform notes:** Mobile: default AA method; FXAA 3.11 quality preset 10 (low).
  Switch: FXAA quality preset 20 (medium). Desktop: optional fallback; quality preset
  29 (high). High-end: typically superseded by TAA or TSR.

### F-2.6.4 MSAA (Multi-Sample Anti-Aliasing)

Hardware multi-sample anti-aliasing sampling multiple sub-pixel locations per fragment during
rasterization. Available only in the forward rendering path since deferred G-buffer layout is
incompatible with per-sample storage. Supports 2x and 4x sample counts.

- **Requirements:** R-2.6.4
- **Dependencies:** F-2.4.1
- **Platform notes:** Mobile: 2x MSAA with on-chip tile resolve (efficient on
  tile-based GPUs); 4x only on high-end iOS devices. Switch: 2x MSAA. Desktop:
  2x or 4x MSAA. High-end: 4x MSAA (rarely used; TAA/TSR preferred).

### F-2.6.5 Checkerboard Rendering

Half-resolution rendering in a checkerboard pattern with temporal reconstruction to full resolution.
Each frame renders 50% of pixels at alternating positions, and a resolve pass reconstructs missing
samples from the current and previous frame's data. Supports both pixel-center and pixel-corner
sampling strategies with a dedicated resolve filter for diagonal edges.

- **Requirements:** R-2.6.5
- **Dependencies:** None
- **Platform notes:** Mobile: effective bandwidth saver on tile-based GPUs; recommended
  for 1080p+ output targets. Switch: used in docked mode to maintain 1080p output.
  Desktop: optional for 4K rendering. High-end: typically superseded by TSR/DLSS.

### F-2.6.6 Third-Party Upscaler Integration

Abstraction layer for integrating vendor-specific temporal upscalers (NVIDIA DLSS, AMD FSR, Intel
XeSS). Each upscaler receives motion vectors, depth, and the low-resolution color buffer and
produces an upscaled output. The integration layer manages per-vendor initialization, quality mode
selection, and graceful fallback to the built-in TSR when no vendor SDK is available.

- **Requirements:** R-2.6.6
- **Dependencies:** F-2.6.2
- **Platform notes:** Mobile: FSR 1.0 spatial-only upscaling available; no DLSS/XeSS.
  Switch 2: DLSS via Tensor Cores (fat/light models). Desktop: DLSS, FSR 2+, and
  XeSS all available based on GPU vendor. High-end: DLSS 4 with multi-frame
  generation.

### F-2.6.7 Frame Generation

AI-driven frame interpolation that synthesizes intermediate frames between rendered frames to
multiply effective frame rate. Consumes motion vectors, depth, and two or more history frames to
produce interpolated output. Supports multi-frame generation on capable hardware with latency
compensation via input-to-display pipeline optimization.

- **Requirements:** R-2.6.7
- **Dependencies:** F-2.6.6
- **Platform notes:** Mobile: disabled; insufficient compute and memory for frame
  synthesis. Switch 2: single-frame DLSS frame gen at 30-to-60 fps target.
  Desktop: single-frame generation (DLSS 3 / FSR 3). High-end: multi-frame
  generation (DLSS 4) producing 3-4x effective frame rate.

### F-2.6.8 Latency Reduction

End-to-end render pipeline latency optimization that minimizes the time between user input and
displayed frame. Synchronizes CPU submission timing with GPU back-pressure to reduce the render
queue depth, paired with frame generation to maintain high frame rates without increasing input lag.

- **Requirements:** R-2.6.8
- **Dependencies:** F-2.6.7
- **Platform notes:** Mobile: basic frame pacing only (no reflex-style pipeline).
  Switch: frame pacing with v-sync alignment. Desktop: NVIDIA Reflex / AMD Anti-Lag
  integration for sub-frame input sampling. High-end: full Reflex + frame gen
  latency compensation.
