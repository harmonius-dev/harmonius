# User Stories -- 12.1 Asset Import

## US-12.1.1 Import Native Format Assets Quickly and Safely

**As a** designer (P-5), **I want** native format ingestion with BLAKE3 hash validation and
automatic deduplication, **so that** I can bring assets into the engine reliably without worrying
about corrupt data or wasting storage on duplicates.

## US-12.1.2 Import Textures from Common Source Formats

**As a** designer (P-5), **I want** PNG, JPEG, EXR, HDR, and TIFF texture import as a fallback for
textures not exported from DCC plugins, **so that** I can use textures from any source without
converting them manually before import.

## US-12.1.3 Import Audio from Lossless and Compressed Formats

**As a** designer (P-5), **I want** WAV, FLAC, and Ogg Vorbis audio import with metadata extraction
(sample rate, loop points, cue markers), **so that** sound assets enter the pipeline with all their
authoring metadata preserved for downstream encoding.

## US-12.1.4 Get Actionable Error Messages When Import Fails

**As a** designer (P-5), **I want** import validation errors that include the source file path, byte
offset, and fix suggestions (e.g., "re-export from DCC plugin"), **so that** I can resolve import
problems without guessing what went wrong.

## US-12.1.5 Import Hundreds of Assets in a Single Operation

**As a** designer (P-5), **I want** parallelized batch import with progress bar, per-asset status,
estimated time remaining, and cancellation that rolls back partial imports, **so that** I can import
large batches efficiently without leaving the database in an inconsistent state.

## US-12.1.6 Validate Imports Automatically in CI

**As a** DevOps engineer (P-16), **I want** import validation to run as a CI step that rejects
builds containing assets with schema violations or version mismatches, **so that** invalid assets
never reach production builds.

## US-12.1.7 Extend the Import Pipeline for New Formats

**As an** engine developer (P-26), **I want** the import pipeline to accept pluggable format
handlers with schema definitions and validation rules, **so that** new asset types can be added
without modifying the core import infrastructure.

## US-12.1.8 Verify Import Handles Corrupt and Edge-Case Files

**As an** engine tester (P-27), **I want** automated tests that import truncated, zero-length,
wrong-version, and malformed asset files, verifying graceful error reporting without crashes,
**so that** the import pipeline is robust against all file corruption scenarios.

## US-12.1.9 Verify Batch Import Rollback on Cancellation

**As an** engine tester (P-27), **I want** tests that cancel a batch import mid-progress and verify
the asset database contains no partially imported entries, **so that** database consistency is
maintained after cancellation.

## US-12.1.10 Benchmark Import Throughput on Multi-Core Machines

**As an** engine tester (P-27), **I want** performance benchmarks that measure import throughput
(assets per minute) on 8-core and 16-core machines, **so that** I can verify parallelized import
scales with available cores and meets the 100+ assets/min target.
