# 12.1 — Asset Import

## Native Format Ingestion

| ID       | Feature                |
|----------|------------------------|
| F-12.1.1 | Native Asset Ingestion |

1. **F-12.1.1** — Accept asset exports from DCC tool plugins (F-12.6) in the engine's native binary
   format (F-12.7.1). The ingestion pipeline validates the format version and magic bytes, verifies
   content hashes against the embedded BLAKE3 digest, and registers the asset in the asset database
   (F-12.3.2). Duplicate assets are detected via content-addressable storage and deduplicated
   automatically.
   - **Deps:** F-12.7.1 (Universal Binary Asset Format), F-12.3.2 (Asset Metadata Store)

## Texture Source Import

| ID       | Feature               |
|----------|-----------------------|
| F-12.1.2 | Texture Source Import |

1. **F-12.1.2** — Import raw texture source files (PNG, JPEG, EXR, HDR, TIFF) as a convenience
   fallback for textures not exported from DCC plugins. PNG and JPEG are decoded for sRGB color
   data; EXR, HDR, and TIFF are decoded for linear high-dynamic-range environment maps, lightmaps,
   and emissive sources. All decoded textures feed into the texture compression pipeline (F-12.2.1).
   - **Deps:** F-12.2.1 (Texture Compression)

## Audio Source Import

| ID       | Feature             |
|----------|---------------------|
| F-12.1.3 | Audio Source Import |

1. **F-12.1.3** — Import raw audio source files in lossless (WAV, FLAC) and compressed (Ogg Vorbis)
   formats for processing into the engine's runtime audio format. Metadata such as sample rate,
   channel count, bit depth, loop points, and cue markers are extracted and stored alongside the raw
   audio data for downstream encoding (F-12.2.6).
   - **Deps:** F-12.2.6 (Audio Encoding)

## Validation

| ID       | Feature                               |
|----------|---------------------------------------|
| F-12.1.4 | Import Validation and Error Reporting |

1. **F-12.1.4** — Validate all imported assets against schema definitions, format version
   constraints, and content integrity checks. Errors are reported with source file path and byte
   offset, alongside actionable fix suggestions (e.g., "format version 3 expected, got 2 — re-export
   from DCC plugin"). Warnings surface non-fatal issues such as missing optional metadata or
   suboptimal texture dimensions.
   - **Deps:** F-12.1.1 (Native Asset Ingestion)

## Batch Import

| ID       | Feature                             |
|----------|-------------------------------------|
| F-12.1.5 | Batch Import with Progress Tracking |

1. **F-12.1.5** — Import multiple assets in a single operation with a progress bar, estimated time
   remaining, and per-asset status indicators. Processing is parallelized across available CPU cores
   with configurable concurrency limits. Supports cancellation at any point, rolling back partially
   imported assets to maintain database consistency. (Audio Source Import)
   - **Deps:** F-12.1.1 (Native Asset Ingestion), F-12.1.2 (Texture Source Import), F-12.1.3
