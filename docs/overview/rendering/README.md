# Rendering

Drawing meshes, lighting, effects, and compositing images.

## Topics

- [pipeline](./pipeline.md) — render graphs, passes, and frame composition.
- [lighting-and-materials](./lighting-and-materials.md) — lights, PBR, materials, and
  shaders.
- [effects-and-styles](./effects-and-styles.md) — post-processing, filters, and stylization.
- [anti-aliasing-and-upscaling](./anti-aliasing-and-upscaling.md) — reducing aliasing and
  rendering at lower resolution then scaling up.
- [environment-and-characters](./environment-and-characters.md) — special rendering for
  terrain, foliage, sky, and character skin.

## Key takeaways

- Decoupled extraction enables CPU parallelism: simulation advances while rendering extracts
  snapshot on a separate thread using immutable queries.
- Render proxies in flat SoA layout with dirty-flag updates reduce per-frame bandwidth from
  O(N) to O(changed).
- Bindless descriptor indexing via per-instance material parameter buffers eliminates state
  switches enabling thousands of material-diverse draws at scale.
- Cook-Torrance PBR with extended BRDF lobes (skin SSS, clearcoat, anisotropy, sheen) covers
  diverse material categories while degrading gracefully on lower tiers.
- Tiled light culling bounds per-fragment cost, enabling hundreds of dynamic lights without
  quadratic frame-time scaling.
- Temporal anti-aliasing jitters projection across frames, reconstructing detail beyond native
  resolution at lower per-pixel cost than MSAA.
- Reflection probes with parallax correction and screen-space reflections combine for efficient
  indirect lighting.

## Integration risks

- Render layer bitmask filtering must align with narrative state: editor overlays and debug views
  use distinct layers; unintended visibility breaks level design intent. See
  [pipeline.md](./pipeline.md) for layer semantics.
- Light culling tile size trades overhead vs per-fragment cost; mistuning causes frame budget
  overflow on mobile or performance cliff on desktop. See [lighting-and-materials.md](./lighting-and-materials.md)
  for tuning guidance.
- Temporal anti-aliasing introduces frame latency (history from N-1 frames); first-person head
  tracking or VR motion sickness sensitive scenarios need lower feedback weights or reduced
  jitter. See [anti-aliasing-and-upscaling.md](./anti-aliasing-and-upscaling.md) for trade-offs.
- Material instances share PSOs; incompatible parameter combinations (e.g., invalid roughness
  ranges) cause compile failures or runtime visual errors. See [lighting-and-materials.md](./lighting-and-materials.md)
  for parameter validation.
- Skin SSS and hair Marschner models require correct normal maps and material flags; incorrect
  setup results in glitchy appearance or dark silhouettes. See [environment-and-characters.md](./environment-and-characters.md)
  for setup checklist.
