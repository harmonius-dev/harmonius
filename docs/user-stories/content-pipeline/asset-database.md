# User Stories -- 12.3 Asset Database

## US-12.3.1 Store Assets with Automatic Deduplication

**As a** designer (P-5), **I want** all processed assets stored in a content-addressable store keyed
by BLAKE3 hash with automatic deduplication, **so that** identical assets across expansion packs and
live-ops updates do not waste storage.

## US-12.3.2 Find Assets by Metadata and Tags

**As a** designer (P-5), **I want** full-text and tag-based faceted search by type, tag,
modification date, file size, and dependency count, **so that** I can locate assets quickly in
libraries containing millions of entries.

## US-12.3.3 Find Assets by Natural Language Description

**As a** designer (P-5), **I want** natural language queries ("rusted metal door with broken
hinges") against a vector index of asset embeddings, **so that** I can find assets by intent rather
than memorized naming conventions.

## US-12.3.4 See Thumbnails for All Asset Types

**As a** designer (P-5), **I want** auto-generated thumbnails (mesh orbit preview, texture
downscale, material sphere, audio waveform) displayed instantly in the asset browser, **so that** I
can visually browse assets without opening each one individually.

## US-12.3.5 Receive Smart Asset Recommendations

**As a** designer (P-5), **I want** the system to recommend matching assets (trim, decals, rubble
meshes) when I select a stone wall texture, **so that** I discover related assets and find
near-duplicates that waste storage.

## US-12.3.6 Skip Unchanged Assets During Builds

**As a** DevOps engineer (P-16), **I want** hash-based import caching that skips all import and
processing stages for unchanged assets, **so that** full rebuilds of hundred-thousand-asset projects
complete in minutes instead of hours.

## US-12.3.7 Rebuild Only Changed Assets and Their Dependents

**As a** DevOps engineer (P-16), **I want** incremental builds that walk the dependency graph
bottom-up and rebuild only assets whose source, settings, or transitive dependencies changed, **so
that** CI build times are proportional to the number of changes.

## US-12.3.8 Roll Back Assets to Any Previous Version

**As a** designer (P-5), **I want** full revision history for each asset with the ability to restore
any previous version by content hash, **so that** I can quickly undo accidental changes or roll back
regressions during live-ops.

## US-12.3.9 Auto-Categorize Imported Assets with AI

**As a** designer (P-5), **I want** an LLM-based classifier that auto-assigns categories, tags, and
quality ratings to imported assets based on visual content and metadata, **so that** the manual
curation overhead for large asset libraries is reduced.

## US-12.3.10 Maintain a Single Source of Truth for All Asset Metadata

**As an** engine developer (P-26), **I want** a persistent metadata store mapping asset IDs to
source paths, content hashes, import settings, dependencies, tags, and per-platform build outputs,
**so that** the content pipeline and editor asset browser always operate on consistent,
authoritative data.

## US-12.3.11 Monitor Build Cache Hit Rates in CI

**As a** DevOps engineer (P-16), **I want** metrics on import cache hit rate and incremental build
coverage, **so that** I can detect cache invalidation regressions and optimize build configurations.

## US-12.3.12 Verify Content-Addressable Deduplication Correctness

**As an** engine tester (P-27), **I want** tests that import identical assets from different paths
and verify only one copy is stored with correct metadata references, **so that** deduplication works
correctly and does not create orphaned entries.

## US-12.3.13 Verify Incremental Build Produces Identical Output to Full Build

**As an** engine tester (P-27), **I want** CI tests that compare incremental build output against a
full rebuild for the same inputs and verify byte-identical results, **so that** incremental builds
never produce different artifacts than clean builds.

## US-12.3.14 Verify Asset Version Restore Produces Correct Content

**As an** engine tester (P-27), **I want** tests that create multiple asset versions, restore an
earlier version, and verify the content matches the original by hash, **so that** version rollback
is reliable for live-ops workflows.

## US-12.3.15 Verify Semantic Search Returns Relevant Results

**As an** engine tester (P-27), **I want** tests that index a known asset corpus and verify natural
language queries return expected assets within the top results, **so that** semantic search quality
is validated and regressions are caught.

## US-12.3.16 Benchmark Search Performance on Million-Entry Databases

**As an** engine tester (P-27), **I want** benchmarks that measure search response time on databases
with one million asset entries, **so that** search performance stays within interactive latency
targets (< 100 ms) as the database grows.

## US-12.3.17 Generate Thumbnails Asynchronously During Import

**As an** engine developer (P-26), **I want** thumbnail rendering for meshes, textures, materials,
and audio to run asynchronously during import and be stored in the metadata database, **so that**
the asset browser displays previews instantly without blocking import.

## US-12.3.18 Verify AI Categorization Accuracy on Reference Assets

**As an** engine tester (P-27), **I want** tests that run the LLM classifier on a labeled reference
corpus and verify categorization accuracy exceeds a threshold (e.g., 85%), **so that**
auto-categorization quality is validated before deployment.

## US-12.3.19 Review AI-Assigned Tags Before Acceptance

**As a** designer (P-5), **I want** to review and override AI-assigned categories and tags before
they are committed to the metadata store, **so that** I maintain control over asset organization and
can correct misclassifications.

## US-12.3.20 Find Near-Duplicate Assets to Reduce Storage Waste

**As a** DevOps engineer (P-16), **I want** smart collections that flag near-duplicate imports and
underused assets, **so that** I can identify storage waste and consolidate redundant content across
expansion packs.
