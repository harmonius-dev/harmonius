# User Stories — 12.1 Asset Import

## US-12.1.1 Import Assets Safely and Quickly Into the Pipeline
**As an** artist, **I want** native format ingestion with hash-based deduplication, texture
and audio source import, validation with actionable error messages, and parallelized batch
import with cancellation, **so that** I can bring assets into the engine reliably without
worrying about duplicates or corrupt data.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Native binary format validated with BLAKE3 hash | F-12.1.1 | R-12.1.1 |
| PNG, JPEG, EXR, HDR, TIFF texture import | F-12.1.2 | R-12.1.2 |
| WAV, FLAC, Ogg Vorbis audio import with metadata | F-12.1.3 | R-12.1.3 |
| Validation errors with file path and fix suggestions | F-12.1.4 | R-12.1.4 |
| Parallelized batch import with rollback on cancel | F-12.1.5 | R-12.1.5 |
| 100+ assets/min throughput on 16-core machine | F-12.1.5 | R-X.14.1 |
| Fallback assets on load failure (no crash) | F-12.1.4 | R-X.3.3 |
