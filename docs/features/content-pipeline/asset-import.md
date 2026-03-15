# 12.1 — Asset Import

## Native Format Ingestion

### F-12.1.1 Native Asset Ingestion

Accept asset exports from DCC tool plugins (F-12.6) in the engine's native binary format (F-12.7.1).
The ingestion pipeline validates the format version and magic bytes, verifies content hashes against
the embedded BLAKE3 digest, and registers the asset in the asset database (F-12.3.2). Duplicate
assets are detected via content-addressable storage and deduplicated automatically.

- **Requirements:** R-12.1.1
- **Dependencies:** F-12.7.1 (Universal Binary Asset Format), F-12.3.2 (Asset Metadata Store)
- **Platform notes:** None

## Texture Source Import

### F-12.1.2 Texture Source Import

Import raw texture source files (PNG, JPEG, EXR, HDR, TIFF) as a convenience fallback for textures
not exported from DCC plugins. PNG and JPEG are decoded for sRGB color data; EXR, HDR, and TIFF are
decoded for linear high-dynamic-range environment maps, lightmaps, and emissive sources. All decoded
textures feed into the texture compression pipeline (F-12.2.1).

- **Requirements:** R-12.1.2
- **Dependencies:** F-12.2.1 (Texture Compression)
- **Platform notes:** None

## Audio Source Import

### F-12.1.3 Audio Source Import

Import raw audio source files in lossless (WAV, FLAC) and compressed (Ogg Vorbis) formats for
processing into the engine's runtime audio format. Metadata such as sample rate, channel count, bit
depth, loop points, and cue markers are extracted and stored alongside the raw audio data for
downstream encoding (F-12.2.6).

- **Requirements:** R-12.1.3
- **Dependencies:** F-12.2.6 (Audio Encoding)
- **Platform notes:** None

## Validation

### F-12.1.4 Import Validation and Error Reporting

Validate all imported assets against schema definitions, format version constraints, and content
integrity checks. Errors are reported with source file path and byte offset, alongside actionable
fix suggestions (e.g., "format version 3 expected, got 2 — re-export from DCC plugin"). Warnings
surface non-fatal issues such as missing optional metadata or suboptimal texture dimensions.

- **Requirements:** R-12.1.4
- **Dependencies:** F-12.1.1 (Native Asset Ingestion)
- **Platform notes:** None

## Batch Import

### F-12.1.5 Batch Import with Progress Tracking

Import multiple assets in a single operation with a progress bar, estimated time remaining, and
per-asset status indicators. Processing is parallelized across available CPU cores with configurable
concurrency limits. Supports cancellation at any point, rolling back partially imported assets to
maintain database consistency.

- **Requirements:** R-12.1.5
- **Dependencies:** F-12.1.1 (Native Asset Ingestion), F-12.1.2 (Texture Source Import), F-12.1.3
  (Audio Source Import)
- **Platform notes:** None
