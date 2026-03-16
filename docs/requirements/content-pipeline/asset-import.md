# R-12.1 Asset Import

## R-12.1.1 Native Asset Ingestion

The engine **SHALL** accept asset exports produced by DCC tool plugins in the engine's native binary
format, validate format version and magic bytes, verify content hashes against the embedded BLAKE3
digest, and register each asset in the asset database with automatic content-addressable
deduplication.

- **Derived from:** [F-12.1.1](../../features/content-pipeline/asset-import.md)
- **Rationale:** A single validated ingestion path for plugin-exported assets ensures data integrity
  and eliminates redundant storage through deduplication.
- **Verification:** Import a valid native-format export and confirm it is registered in the asset
  database; import a file with a corrupted hash and confirm rejection; import a duplicate asset and
  confirm deduplication produces no new storage entry.

## R-12.1.2 Texture Source Import

The engine **SHALL** import raw texture source files in PNG, JPEG, EXR, HDR, and TIFF formats,
decode sRGB color data from PNG and JPEG and linear HDR data from EXR, HDR, and TIFF, and feed all
decoded textures into the texture compression pipeline.

- **Derived from:** [F-12.1.2](../../features/content-pipeline/asset-import.md)
- **Rationale:** A convenience fallback for textures not exported through DCC plugins ensures
  artists can import standard image formats without a round-trip through the plugin workflow.
- **Verification:** Import one file of each supported format and confirm correct color-space
  decoding (sRGB vs. linear) and successful handoff to texture compression.

## R-12.1.3 Audio Source Import

The engine **SHALL** import raw audio source files in WAV, FLAC, and Ogg Vorbis formats, extract
metadata (sample rate, channel count, bit depth, loop points, cue markers), and store the raw audio
data alongside metadata for downstream encoding.

- **Derived from:** [F-12.1.3](../../features/content-pipeline/asset-import.md)
- **Rationale:** Accepting standard lossless and compressed audio formats allows sound designers to
  supply assets directly without requiring a DCC plugin export step.
- **Verification:** Import WAV, FLAC, and Ogg Vorbis files and confirm extracted metadata matches
  the source; confirm raw audio data is passed to the audio encoding stage.

## R-12.1.4 Import Validation and Error Reporting

The engine **SHALL** validate all imported assets against schema definitions, format version
constraints, and content integrity checks, and report errors with source file path, byte offset, and
actionable fix suggestions.

- **Derived from:** [F-12.1.4](../../features/content-pipeline/asset-import.md)
- **Rationale:** Precise error reporting with fix suggestions reduces artist iteration time and
  prevents corrupt or incompatible assets from entering the pipeline.
- **Verification:** Import an asset with an incorrect format version and confirm the error message
  includes the file path, byte offset, and a suggestion to re-export; import an asset with missing
  optional metadata and confirm a non-fatal warning is emitted.

## R-12.1.5 Batch Import with Progress Tracking

The engine **SHALL** import multiple assets in a single operation with parallelized processing
across available CPU cores, provide progress reporting with per-asset status, and support
cancellation that rolls back partially imported assets to maintain database consistency.

- **Derived from:** [F-12.1.5](../../features/content-pipeline/asset-import.md)
- **Rationale:** Parallelized batch import with rollback-safe cancellation is essential for
  large-scale asset libraries where importing thousands of assets sequentially would be
  prohibitively slow.
- **Verification:** Batch-import 100+ assets and confirm all complete with correct status; cancel
  mid-batch and confirm no partially imported assets remain in the database; vary concurrency limits
  and confirm they are respected.
