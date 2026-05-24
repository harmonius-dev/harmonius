# Anti-Aliasing and Upscaling

Temporal sampling, reconstruction, and upscaling techniques for smooth edges and high-resolution
appearance at lower GPU cost.

## What it covers

- Temporal anti-aliasing (TAA) sampling at sub-pixel offsets across frames.
- Jitter patterns for multi-sample convergence without per-pixel cost.
- Sample reuse across frames to accumulate quality.
- TAA reconstruction filters reducing ghosting and ringing.
- Screen-space upscaling from lower resolution to display resolution.
- Motion vector sampling and velocity history for temporal coherence.
- Feedback clipping and neighborhood exploration reducing temporal artifacts.
- MSAA anti-aliasing at specified sample counts (2×, 4×, 8×).
- Forward-plus rendering for multi-sample transparency.
- Adaptive sample rates per material (e.g., reduced samples on distant foliage).

## Concepts

### Temporal Anti-Aliasing Foundations

Temporal anti-aliasing (TAA) jitters the camera projection by sub-pixel offsets each frame
following a Halton sequence or similar pattern. Over consecutive frames, jitter positions form a
grid at higher effective resolution than the frame buffer. Samples reuse accumulated history,
reconstructing detail beyond native resolution. This technique eliminates geometric aliasing
without per-pixel overhead of multisampling and works well at fixed timesteps when motion vectors
are available.

### Jitter and Reconstruction

Jitter patterns space sample offsets carefully to avoid visible banding or clustering. The
reconstruction filter reads current and historical samples, using motion vectors to align history
temporally. Feedback mechanisms blend new and old color: high feedback (slower response) reduces
flicker but risks ghosting; low feedback (faster response) follows motion well but introduces
temporal artifacts. Neighborhood exploration clips historical samples to the neighborhood of
current samples, further reducing ghosting.

### Motion-Aware Temporal Coherence

Motion vectors sampled per pixel enable proper temporal alignment when objects move. Velocity
history tracks per-pixel motion across frames, allowing the reconstruction filter to account for
accelerating or decelerating movement. Disocclusions (newly visible surfaces after occlusion
removal) require special handling to avoid smearing; these detect via depth discontinuities.

### Upscaling and Reconstruction

Screen-space upscaling reconstructs high-resolution output from lower-resolution rendering (e.g.,
render at 1080p, output at 4K). Edge-aware reconstruction uses depth and normal discontinuities to
avoid blurring edges. Lanczos or similar reconstruction kernels maintain sharpness. Upscaling
trades GPU cost (render resolution) for per-frame performance while accepting reconstruction
artifacts.

### Multisampling and Adaptive Sampling

Traditional multisample anti-aliasing (MSAA) rasterizes at higher sample counts, resolving to
display resolution per pixel. MSAA works naturally with forward rendering but costs memory and
bandwidth. Adaptive sample rates reduce samples on distant geometry or specific materials; nearby
high-detail surfaces sample fully while distant foliage samples sparsely, trading quality for
performance per material.

## How it fits

- See [pipeline.md](./pipeline.md) for jitter application during projection setup.
- See [effects-and-styles.md](./effects-and-styles.md) for post-processing after antialiasing
  and before tone mapping.
- See [../core-runtime/simulation-loop.md](../core-runtime/simulation-loop.md) for frame pacing
  and timestamp consistency.
