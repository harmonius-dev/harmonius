# 12.1 — Asset Import

## Model Import

### F-12.1.1 glTF 2.0 Model Import

Parse glTF 2.0 binary and text files using a low-level C library (cgltf). Extracts mesh geometry,
skeleton hierarchies, morph targets, materials, and scene graphs into engine-native intermediate
representations. glTF serves as the primary interchange format for all 3D content.

- **Requirements:** R-12.1.1
- **Dependencies:** None
- **Platform notes:** None

### F-12.1.2 FBX Model Import via C Wrapper

Import FBX files through a minimal C wrapper around the Autodesk FBX SDK, bridged into Rust via FFI.
Supports static meshes, skinned meshes, blend shapes, embedded materials, and scene hierarchies.
Required for legacy asset pipelines and DCC tools that export FBX natively.

- **Requirements:** R-12.1.2
- **Dependencies:** None
- **Platform notes:** FBX SDK ships platform-specific shared libraries; the C wrapper must be built
  per-target.

## Texture Import

### F-12.1.3 Raster Texture Import (PNG, JPEG, HDR, EXR)

Import standard raster image formats for use as source textures. PNG and JPEG are decoded for sRGB
color data; HDR and EXR are decoded for linear high-dynamic-range environment maps, lightmaps, and
emissive sources. All decoded textures feed into the processing pipeline for compression.

- **Requirements:** R-12.1.3
- **Dependencies:** F-12.2.1 (texture compression)
- **Platform notes:** None

### F-12.1.4 GPU-Ready Texture Import (KTX2, DDS)

Import pre-compressed GPU-ready textures in KTX2 and DDS container formats. These bypass the
compression stage and are validated, mip-checked, and packed directly into asset bundles. Essential
for artist-controlled compression and third-party texture libraries.

- **Requirements:** R-12.1.4
- **Dependencies:** None
- **Platform notes:** DDS textures with BC formats require transcoding on platforms that only
  support ASTC.

## Audio Import

### F-12.1.5 Audio Import (WAV, OGG, FLAC)

Import audio source files in lossless (WAV, FLAC) and compressed (OGG Vorbis) formats. Metadata
such as sample rate, channel count, loop points, and cue markers are extracted and stored alongside
the raw audio data for downstream encoding.

- **Requirements:** R-12.1.5
- **Dependencies:** F-12.2.5 (audio encoding)
- **Platform notes:** None

## Animation and Scene Import

### F-12.1.6 Animation and Scene Import

Import skeletal animations, morph target animations, and camera/light animation curves from glTF and
FBX sources. Scene-level imports reconstruct full level layouts including entity placement, light
probes, reflection volumes, and trigger regions from DCC scene exports.

- **Requirements:** R-12.1.6
- **Dependencies:** F-12.1.1, F-12.1.2
- **Platform notes:** None

## Import Configuration

### F-12.1.7 Import Settings and Presets

Per-asset and per-directory import settings controlling format options, compression quality, LOD
policies, and platform overrides. Named presets (e.g., "character_diffuse", "terrain_splat",
"environment_hdr") enable consistent import behavior across thousands of assets in an MMO-scale
content library without manual per-file configuration.

- **Requirements:** R-12.1.7
- **Dependencies:** None
- **Platform notes:** None
