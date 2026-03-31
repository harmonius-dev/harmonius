# User Stories -- 12.1 Asset Import

## Native Format Ingestion

| ID         | Persona                    |
|------------|----------------------------|
| US-12.1.1  | technical artist (P-13)    |
| US-12.1.2  | engine developer (P-26)    |
| US-12.1.3  | build engineer (P-16)      |

1. **US-12.1.1** — **As a** technical artist (P-13), **I want** assets exported from DCC plugins to
   be validated and registered automatically on ingestion, **so that** I know immediately if an
   export is corrupt or incompatible.
2. **US-12.1.2** — **As an** engine developer (P-26), **I want** native binary imports to verify
   BLAKE3 content hashes against embedded digests, **so that** data integrity is guaranteed before
   any asset enters the database.
3. **US-12.1.3** — **As a** build engineer (P-16), **I want** duplicate assets detected via
   content-addressable storage during ingestion, **so that** redundant data never inflates build
   sizes.

## Texture Source Import

| ID         | Persona                    |
|------------|----------------------------|
| US-12.1.4  | environment artist (P-8)   |
| US-12.1.5  | technical artist (P-13)    |

1. **US-12.1.4** — **As an** environment artist (P-8), **I want** to import PNG, JPEG, EXR, HDR, and
   TIFF textures directly without a DCC plugin round-trip, **so that** I can quickly bring in
   reference or sourced textures.
2. **US-12.1.5** — **As a** technical artist (P-13), **I want** sRGB and linear HDR color spaces
   decoded correctly based on file format, **so that** textures feed into the compression pipeline
   with accurate color data.

## Audio Source Import

| ID         | Persona                    |
|------------|----------------------------|
| US-12.1.6  | audio designer (P-14)      |
| US-12.1.7  | audio designer (P-14)      |

1. **US-12.1.6** — **As an** audio designer (P-14), **I want** to import WAV, FLAC, and Ogg Vorbis
   files with metadata extracted automatically, **so that** I can supply audio assets in standard
   formats without extra tooling.
2. **US-12.1.7** — **As an** audio designer (P-14), **I want** loop points and cue markers preserved
   during import, **so that** downstream encoding respects my authoring intent.

## Validation

| ID         | Persona                    |
|------------|----------------------------|
| US-12.1.8  | technical artist (P-13)    |
| US-12.1.9  | environment artist (P-8)   |

1. **US-12.1.8** — **As a** technical artist (P-13), **I want** import errors to include the source
   file path, byte offset, and an actionable fix suggestion, **so that** I can resolve issues
   without guessing.
2. **US-12.1.9** — **As an** environment artist (P-8), **I want** warnings for non-fatal issues like
   suboptimal texture dimensions, **so that** I can improve assets without being blocked from
   importing.

## Batch Import

| ID          | Persona                    |
|-------------|----------------------------|
| US-12.1.10  | build engineer (P-16)      |
| US-12.1.11  | technical artist (P-13)    |
| US-12.1.12  | environment artist (P-8)   |

1. **US-12.1.10** — **As a** build engineer (P-16), **I want** batch imports parallelized across
   available CPU cores with configurable concurrency, **so that** large asset sets import as fast as
   the hardware allows.
2. **US-12.1.11** — **As a** technical artist (P-13), **I want** a progress bar with per-asset
   status and estimated time remaining during batch imports, **so that** I can plan my work around
   long imports.
3. **US-12.1.12** — **As an** environment artist (P-8), **I want** to cancel a batch import and have
   partially imported assets rolled back, **so that** the database stays consistent if I realize I
   selected the wrong files.
