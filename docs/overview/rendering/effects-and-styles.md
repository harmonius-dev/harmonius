# Effects and Styles

Post-processing, color grading, artistic overrides, and depth-of-field effects applied after
all geometry and lighting.

## What it covers

- Post-processing pipelines compositing effects after lighting.
- Bloom with configurable thresholds and blur radii detecting emissive materials.
- Tone mapping: ACES curve, adjustable exposure, and contrast.
- Color grading via LUT with branching to game-specific curves (gameplay, menu, cutscene).
- Ambient occlusion screen-space approximations.
- Depth-of-field with configurable aperture, focus distance, and layered blur.
- Motion blur temporal accumulation.
- Chromatic aberration and lens distortion.
- Deferred decals rendering into separate targets before lighting.
- Stylized rendering: cell shading, outline highlighting, posterization, and sketch effects.
- Projected and mesh-based decals with normal-mapped contact blending.

## Concepts

### Post-Processing Architecture

Post-processing effects apply after lighting and before the final tone mapping. Each effect runs as
a compute or fragment shader pass consuming intermediate targets (color, normal, depth, motion) and
producing a modified color output. Effects chain sequentially; each can read prior outputs.

### Bloom and Emissive Handling

Bloom extracts pixels above a brightness threshold, typically those with emissive intensity, and
applies separable Gaussian blur with configurable radius. The bloom-on threshold triggers
automatically when materials set emissive intensity; this drives both real-time bloom and optionally
contributes to global illumination on capable platforms. Bloom scales with resolution and platform
capabilities.

### Color and Tone

Tone mapping converts linear color to display color using an industry-standard ACES curve,
maintaining perceptual fidelity. Color grading applies a 3D LUT for artistic control, with lookup
branching to game state (gameplay mode, menu, cutscene) so each gameplay context has distinct color.
This technique avoids per-pixel color calculation overhead.

### Stylization and Outline

Stylized effects (cell shading, posterization, sketch modes) inspect normals and depth for feature
edges. Outlines render scene geometry with inverted normals and slight scale to produce silhouette
highlighting. These effects are optional per-frame and typically gate to specific gameplay modes or
debug visualization.

### Decals and Detail

Projected decals render into a separate deferred decal target before lighting, allowing the lighting
pass to blend decal normals. Mesh-based decals use projected tangent-space UVs for detail-mapped
contact. Both types support normal-mapped contact blending preventing decals from floating.

## How it fits

- See [lighting-and-materials.md](./lighting-and-materials.md) for emissive detection and bloom
  triggers.
- See [anti-aliasing-and-upscaling.md](./anti-aliasing-and-upscaling.md) for pre-tone-map
  antialiasing.
- See [pipeline.md](./pipeline.md) for deferred buffer layout and decal placement.
- Integrates with [../data-systems/attributes-and-effects.md](../data-systems/attributes-and-effects.md)
  for per-entity effect flags.
