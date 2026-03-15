# R-2.3 -- Core Rendering Requirements

## R-2.3.1 Direct Lighting

The engine **SHALL** evaluate point, spot, and directional lights with physically-based attenuation,
writing all light types into a single unified light buffer consumed by both forward and deferred
rendering paths.

- **Derived from:** [F-2.3.1](../../features/rendering/core-rendering.md)
- **Rationale:** A unified light buffer eliminates duplicate light management across rendering paths
  and ensures consistent lighting regardless of the active pipeline.
- **Verification:** Render a scene containing one point light, one spot light, and one directional
  light using both forward and deferred paths. Verify pixel-identical output (within floating-point
  tolerance) between paths. Confirm all three light types read from the same GPU buffer via GPU
  capture inspection.

## R-2.3.2 GPU Frustum Culling

The engine **SHALL** perform meshlet-level frustum culling on the GPU via a compute pass that tests
each meshlet AABB against the six camera frustum planes and excludes meshlets outside the frustum
from the indirect draw buffer.

- **Derived from:** [F-2.3.2](../../features/rendering/core-rendering.md)
- **Rationale:** GPU-side frustum culling avoids CPU-GPU round-trips for culling decisions and
  scales to millions of meshlets without CPU bottlenecks.
- **Verification:** Render a scene with the camera positioned so that at least 50% of meshlets are
  outside the frustum. Verify the indirect draw buffer contains zero entries for off-screen
  meshlets. Compare against a CPU reference culler and confirm identical accept/reject decisions for
  all meshlets.

## R-2.3.3 Backface Culling

The engine **SHALL** perform meshlet-level normal cone culling on the GPU, rejecting meshlets whose
triangles all face away from the camera as determined by the meshlet normal cone test against the
camera position.

- **Derived from:** [F-2.3.3](../../features/rendering/core-rendering.md)
- **Rationale:** Normal cone culling eliminates fully back-facing meshlets before rasterization,
  reducing vertex processing and overdraw.
- **Verification:** Render a closed convex mesh (e.g., sphere) and verify that approximately 50% of
  meshlets are culled. Rotate the camera 180 degrees and verify the culled set inverts. Confirm no
  visible popping or missing geometry in the final image.

## R-2.3.4 Occlusion Culling (HZB Two-Phase)

The engine **SHALL** implement two-phase hierarchical Z-buffer occlusion culling where phase 1 tests
meshlets against the previous frame's HZB (conservative) and phase 2 re-tests phase-1 rejects
against the current frame's HZB to avoid missing newly-revealed geometry. The two-phase HZB
algorithm is shared with the meshlet pipeline (R-3.1.2); this requirement covers the integration
into the scene rendering pipeline.

- **Derived from:** [F-2.3.4](../../features/rendering/core-rendering.md)
- **Rationale:** Two-phase HZB prevents temporal disocclusion artifacts that single-phase occlusion
  culling produces when objects move or the camera translates.
- **Verification:** Place a large occluder in front of 100 meshlets. Verify phase 1 rejects the
  occluded meshlets. Move the camera to reveal the occluded geometry and verify phase 2 recovers
  them in the same frame with no one-frame pop-in. Measure that occlusion culling reduces draw calls
  by at least 30% in a dense urban scene.

## R-2.3.5 Orthographic Projection

The engine **SHALL** support orthographic camera projection producing correct rendering for top-down
views, 2D game rendering, and shadow map generation.

- **Derived from:** [F-2.3.5](../../features/rendering/core-rendering.md)
- **Rationale:** Orthographic projection is required for 2D games, isometric views, and directional
  light shadow map rendering.
- **Verification:** Render two identical cubes at different distances from an orthographic camera.
  Verify both cubes have identical screen-space size (within 1 pixel). Verify shadow map depth
  values are linear.

## R-2.3.6 Perspective Projection (Reverse-Z)

The engine **SHALL** support perspective camera projection with reverse-Z depth mapping and an
optional infinite far plane for maximum depth precision at distance.

- **Derived from:** [F-2.3.6](../../features/rendering/core-rendering.md)
- **Rationale:** Reverse-Z distributes floating-point precision evenly across the depth range,
  eliminating z-fighting at far distances that plagues standard depth mappings.
- **Verification:** Render two coplanar surfaces at 10,000 units from the camera. Verify no
  z-fighting artifacts with reverse-Z enabled. Enable infinite far plane and verify objects at
  extreme distance (1,000,000 units) remain depth-resolvable. Confirm the depth buffer stores 1.0 at
  the near plane and 0.0 at the far plane.

## R-2.3.7 GPU-Driven Instancing

The engine **SHALL** compact surviving opaque meshlet instances after culling into contiguous draw
buffers batched by shared material and material instance, dispatching each material batch via a
single indirect draw call, while rendering transparent objects individually in back-to-front sorted
order without batching.

- **Derived from:** [F-2.3.7](../../features/rendering/core-rendering.md)
- **Rationale:** GPU-side instance compaction minimizes draw call count for opaque geometry while
  preserving correct depth ordering for transparent objects.
- **Verification:** Render a scene with 5,000 opaque instances sharing 10 materials. Verify the
  indirect draw buffer contains exactly 10 draw calls (one per material batch). Add 50 transparent
  objects and verify each produces an individual draw call in back-to-front order. Confirm no visual
  artifacts from incorrect draw ordering.

## R-2.3.8 Render-to-Texture

The engine **SHALL** support rendering any pass to an off-screen texture for consumption by
subsequent passes, with the render graph compiler automatically inserting barriers between write and
read operations.

- **Derived from:** [F-2.3.8](../../features/rendering/core-rendering.md)
- **Rationale:** Render-to-texture enables multi-pass rendering techniques including shadow maps,
  reflection probes, and post-processing chains.
- **Verification:** Render a scene to an off-screen texture, then sample that texture in a
  subsequent fullscreen pass. Verify the final output matches a reference image. Inspect the
  compiled render graph and verify barrier insertion between the write and read passes.

## R-2.3.9 Cubemap Rendering

The engine **SHALL** support static and dynamic cubemap rendering for environment maps, reflection
probes, and IBL prefiltering, with dynamic cubemaps re-rendering specified faces per frame.

- **Derived from:** [F-2.3.9](../../features/rendering/core-rendering.md)
- **Rationale:** Cubemaps are the foundation for environment reflections, sky lighting, and
  image-based lighting precomputation.
- **Verification:** Render a dynamic cubemap and verify all six faces contain correct scene geometry
  from the probe position. Update only two faces per frame and verify the remaining four retain
  their previous content. Verify seamless edge filtering across face boundaries.

## R-2.3.10 Scene Capture

The engine **SHALL** render the scene from an arbitrary camera into a texture target supporting both
2D planar capture and omnidirectional cubemap capture for use cases including security cameras,
mirrors, portals, and minimap rendering.

- **Derived from:** [F-2.3.10](../../features/rendering/core-rendering.md)
- **Rationale:** Scene capture enables gameplay features that require rendering the scene from
  viewpoints other than the main camera.
- **Verification:** Configure a planar scene capture rendering a mirror. Verify the reflected image
  matches a reference rendering from the mirror's virtual camera position. Configure a cubemap
  capture and verify all six faces contain correct geometry. Verify the capture texture is usable as
  a material input in the same frame.

## R-2.3.11 Dynamic Resolution

The engine **SHALL** adjust the internal render resolution at runtime to maintain a target frame
budget, driven by a GPU timing feedback loop that scales the screen percentage between configurable
minimum and maximum bounds.

- **Derived from:** [F-2.3.11](../../features/rendering/core-rendering.md)
- **Rationale:** Dynamic resolution maintains frame rate targets under variable GPU load without
  visible quality loss when paired with temporal upscaling.
- **Verification:** Render a scene that exceeds the frame budget at native resolution. Verify the
  resolution scale decreases within 5 frames. Reduce scene complexity below budget and verify the
  resolution scale increases back toward 100%. Verify the scale never exceeds the configured maximum
  or falls below the configured minimum.

## R-2.3.12 Subsurface Scattering

The engine **SHALL** provide screen-space and ray-traced subsurface scattering driven by
per-material SSS profiles defining scatter radius and extinction coefficients, applicable to skin,
wax, marble, and other translucent materials.

- **Derived from:** [F-2.3.12](../../features/rendering/core-rendering.md)
- **Rationale:** Subsurface scattering is essential for realistic rendering of organic and
  translucent materials where light penetrates and diffuses beneath the surface.
- **Verification:** Render a sphere with a skin SSS profile lit by a directional light. Verify
  visible light transmission on the shadow terminator (red/orange hue shift). Compare screen-space
  and ray-traced paths against a reference and verify both produce scatter radii within 10% of the
  profile specification.

## R-2.3.13 Alpha Mask Cutouts

The engine **SHALL** support alpha-tested geometry with a configurable per-material alpha test
threshold, where cutout geometry participates in shadow map rendering.

- **Derived from:** [F-2.3.13](../../features/rendering/core-rendering.md)
- **Rationale:** Alpha mask cutouts enable efficient rendering of vegetation, fences, and decals
  without the cost of transparency sorting.
- **Verification:** Render a quad with an alpha mask texture (checkerboard pattern). Verify pixels
  below the threshold are discarded (not drawn). Adjust the threshold and verify the visible area
  changes accordingly. Verify the cutout geometry casts correct shadows in the shadow map matching
  the alpha mask shape.

## Non-Functional Requirements

### NFR-2.3.1 Culling Pipeline Latency

The combined GPU culling pipeline (frustum + backface + HZB occlusion) **SHALL** complete in under
1.0 ms at 1080p for a scene with 1,000,000 meshlets on target hardware.

- **Rationale:** The culling pipeline runs every frame before draw submission; exceeding 1 ms would
  consume a significant portion of the frame budget.
- **Verification:** Profile the culling pipeline on a scene with 1,000,000 meshlets and verify total
  GPU time is below 1.0 ms.

### NFR-2.3.2 Dynamic Resolution Response Time

The dynamic resolution system **SHALL** converge to a stable resolution scale within 5 frames of a
sustained GPU load change, with no oscillation exceeding 5% of the target scale once converged.

- **Rationale:** Slow convergence causes visible judder; oscillation causes distracting resolution
  flicker.
- **Verification:** Introduce a step change in GPU load and measure convergence time and
  post-convergence oscillation amplitude.

### NFR-2.3.3 GPU Instancing Draw Call Reduction

GPU-driven instancing **SHALL** reduce the number of draw calls to at most one per unique material
batch for opaque geometry, achieving at least a 100x reduction compared to per-instance draw calls
in scenes with 10,000+ instances sharing 10 materials.

- **Rationale:** Draw call count is a primary CPU bottleneck; GPU-driven batching must compress
  draws to the theoretical minimum (one per material).
- **Verification:** Render 10,000 instances across 10 materials and verify the indirect draw buffer
  contains exactly 10 entries.
