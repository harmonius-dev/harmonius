# R-12.1 Asset Import

## Native Format Ingestion

1. **R-12.1.1** — The engine **SHALL** accept asset exports produced by DCC tool plugins in the
   engine's native binary format, validate format version and magic bytes, verify content hashes
   against the embedded BLAKE3 digest, and register each asset in the asset database with automatic
   content-addressable deduplication.
   - **Rationale:** A single validated ingestion path for plugin-exported assets ensures data
     integrity and eliminates redundant storage.
   - **Verification:** Import a valid native-format export and confirm registration; import a file
     with a corrupted hash and confirm rejection; import a duplicate and confirm deduplication
     produces no new storage entry.

## Texture Source Import

2. **R-12.1.2** — The engine **SHALL** import raw texture source files in PNG, JPEG, EXR, HDR, and
   TIFF formats, decode sRGB color data from PNG and JPEG and linear HDR data from EXR, HDR, and
   TIFF, and feed all decoded textures into the texture compression pipeline.
   - **Rationale:** Importing standard image formats lets artists bring in textures directly without
     an intermediate export step.
   - **Verification:** Import one file of each format and confirm correct color-space decoding (sRGB
     vs. linear) and successful handoff to texture compression.

## Audio Source Import

3. **R-12.1.3** — The engine **SHALL** import raw audio source files in WAV, FLAC, and Ogg Vorbis
   formats, extract metadata (sample rate, channel count, bit depth, loop points, cue markers), and
   store raw audio data alongside metadata for downstream encoding.
   - **Rationale:** Accepting standard lossless and compressed audio formats allows sound designers
     to supply assets directly using standard audio formats.
   - **Verification:** Import WAV, FLAC, and Ogg Vorbis files and confirm extracted metadata matches
     the source; confirm raw audio data is passed to the audio encoding stage.

## Validation

4. **R-12.1.4** — The engine **SHALL** validate all imported assets against schema definitions,
   format version constraints, and content integrity checks, reporting errors with source file path,
   byte offset, and actionable fix suggestions.
   - **Rationale:** Precise error reporting with fix suggestions reduces artist iteration time and
     prevents corrupt assets from entering the pipeline.
   - **Verification:** Import an asset with an incorrect format version and confirm the error
     includes file path, byte offset, and a fix suggestion; import an asset with missing optional
     metadata and confirm a non-fatal warning is emitted.

## Batch Import

5. **R-12.1.5** — The engine **SHALL** import multiple assets in a single operation with
   parallelized processing across available CPU cores, provide progress reporting with per-asset
   status, and support cancellation that rolls back partially imported assets to maintain database
   consistency.
   - **Rationale:** Parallelized batch import with rollback-safe cancellation is essential for
     large-scale asset libraries where sequential import would be prohibitively slow.
   - **Verification:** Batch-import 100+ assets and confirm all complete with correct status; cancel
     mid-batch and confirm no partially imported assets remain; vary concurrency limits and confirm
     they are respected.
