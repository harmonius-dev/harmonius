# 15.17 — Asset Marketplace

## Browsing and Discovery

### F-15.17.1 Integrated Asset Store Browser

An in-editor marketplace browser for discovering, previewing, and acquiring assets, plugins, and
tools. The browser displays a searchable, filterable catalog with categories (3D models, materials,
VFX, audio, logic graph templates, full game templates, plugins, tools), tags, ratings, reviews,
download counts, and compatibility badges. Asset previews include 3D model viewers (rotate, zoom),
material previews on standard meshes, audio playback, and screenshot galleries. Search supports
text, tags, and semantic queries. The store is accessible from the editor's main menu and the
launcher's home screen. Consider integration with Epic's FAB marketplace API for cross-engine asset
access where asset formats are compatible.

- **Requirements:** R-15.17.1
- **Dependencies:** F-15.1.1 (Editor Framework), F-12.3.1 (Asset Database)
- **Platform notes:** Store API is REST-based, served by the cloud infrastructure (F-15.12.7).

### F-15.17.2 One-Click Asset Import and Project Integration

Purchase/download an asset and automatically import it into the current project. The import pipeline
validates compatibility (engine version, required plugins, platform support), resolves dependencies
(an asset requiring a specific material library pulls it automatically), and installs to a
designated marketplace assets folder. Imported assets appear in the asset browser (F-12.3.1)
immediately. Version conflicts (asset requires plugin v2.0 but project has v1.8) are flagged with
upgrade guidance. Free assets download without payment flow. Asset licenses are tracked per-project.

- **Requirements:** R-15.17.2
- **Dependencies:** F-15.17.1, F-12.1.1 (Native Asset Ingestion), F-15.15.4 (Project File)
- **Platform notes:** None

### F-15.17.3 Asset Ratings, Reviews, and Curation

Players and developers rate assets 1-5 stars with text reviews. Reviews are moderated for spam and
abuse. An editorial curation team highlights featured assets, staff picks, and themed collections
(e.g., "Medieval Bundle", "Sci-Fi Starter Kit"). Community-driven collections allow users to curate
and share lists of recommended assets. Assets display: average rating, review count, download count,
last-updated date, engine version compatibility, and verified-publisher badge.

- **Requirements:** R-15.17.3
- **Dependencies:** F-15.17.1, F-15.12.7 (Cloud Service)
- **Platform notes:** None

## Publisher Tools

### F-15.17.4 Publisher Account and Dashboard

Asset creators register publisher accounts with identity verification. The publisher dashboard
displays: published assets, revenue analytics (sales, refunds, royalties), download statistics, user
reviews (with response capability), compatibility test results per engine version, and payout
history. Publishers set prices in multiple currencies, configure regional pricing, run time-limited
sales and discount codes, and bundle assets into packs with aggregate pricing.

- **Requirements:** R-15.17.4
- **Dependencies:** F-15.17.1, F-15.12.7 (Cloud Service)
- **Platform notes:** None

### F-15.17.5 Automated Compatibility Testing

When a new engine version is released, the marketplace automatically tests all published assets
against it. The testing pipeline imports each asset into a test project, verifies it loads without
errors, runs any included automated tests, and generates a compatibility report. Assets that pass
receive a compatibility badge for the new version. Publishers receive notifications for assets that
fail, with specific error details for remediation. Compatibility testing runs on CI infrastructure
using the shared build cache (F-15.11.1).

- **Requirements:** R-15.17.5
- **Dependencies:** F-15.17.4, F-15.11.1 (Shared Build Cache), F-15.15.2 (Project Upgrades)
- **Platform notes:** None

### F-15.17.6 Revenue Sharing and Payout

The marketplace takes a configurable commission (default: 12%) on paid asset sales. Publishers
receive monthly payouts via bank transfer, PayPal, or platform credits. Revenue reports detail
per-asset sales, regional breakdown, and commission deductions. Refund processing automatically
reverses publisher revenue for refunded purchases within the refund window (14 days). Tax
documentation (W-9, W-8BEN) is collected from publishers for compliance. Free assets cost nothing to
publish.

- **Requirements:** R-15.17.6
- **Dependencies:** F-15.17.4
- **Platform notes:** Payment processing uses Stripe or equivalent for global payouts.

## Asset Types and Licensing

### F-15.17.7 Asset Type Support

The marketplace supports all asset types producible by the engine: 3D meshes (with LODs, materials,
collision), 2D sprites and sprite sheets, materials and material functions, VFX effect graphs, audio
clips and music tracks, logic graph templates (gameplay systems, AI behaviors, tools), UI widget
templates, animation clips and state machines, terrain brushes and biome presets, procedural
generation graphs, full project templates (genre starters), and Rust plugins (compiled dynamic
libraries per platform). Each type has appropriate preview rendering and metadata.

- **Requirements:** R-15.17.7
- **Dependencies:** F-15.17.1, F-12.7.1 (Binary Asset Format), F-13.1.10 (Rust Plugin API)
- **Platform notes:** Rust plugins require per-platform compilation; the marketplace hosts
  platform-specific binaries.

### F-15.17.8 License Management and DRM

Assets carry license metadata: usage rights (personal, commercial, non-transferable), redistribution
rights (can mods include this asset?), attribution requirements, and seat count (per-user or
per-project). The engine validates licenses at import time and displays license terms in the asset
browser. No runtime DRM -- once imported, assets work offline without license checks. License
violations (using a personal-license asset in a commercial product) are contractual, not technical.

- **Requirements:** R-15.17.8
- **Dependencies:** F-15.17.2, F-15.15.5 (Account Management)
- **Platform notes:** None

## Open Source Asset Store

### F-15.17.9 Open Source Asset Browser

Browse, search, and filter free and open-source assets hosted on the engine's community Git
repository. The browser integrates into the same store UI as the paid marketplace (F-15.17.1) with a
dedicated "Open Source" tab. Assets are indexed by license type (CC0, CC-BY, Apache 2.0, MIT),
category, tags, and engine version compatibility. Community contributions are accepted via pull
requests to the repository. The repository is cloneable for fully offline access.

- **Requirements:** R-15.17.9
- **Dependencies:** F-15.17.1, F-15.10.1 (Git Integration)
- **Platform notes:** Git-based storage; mirrors served via CDN.

### F-15.17.10 Asset Upload and Publishing

Publish assets to the open source store with mandatory license tagging (CC0, CC-BY, Apache 2.0,
MIT), metadata (description, category, tags), thumbnails, and preview media. Publishing creates a
pull request to the community repository. Automated CI validates the asset (loads in a test project,
passes lint checks, verifies license file presence). Maintainers review and merge. Published assets
appear in the open source browser after merge.

- **Requirements:** R-15.17.10
- **Dependencies:** F-15.17.9, F-15.17.5 (Compat Testing)
- **Platform notes:** None

### F-15.17.11 Asset Rating and Reviews

Community ratings (1-5 stars), text reviews, download counts, and popularity sorting for open source
assets. Reviews are moderated by community maintainers. Assets display: average rating, review
count, download count, last-updated date, contributor count, and license badge. Sorting options
include: most downloaded, highest rated, newest, and recently updated.

- **Requirements:** R-15.17.11
- **Dependencies:** F-15.17.9, F-15.17.3
- **Platform notes:** Review data stored in the community repository as structured metadata files.

### F-15.17.12 Asset Versioning

Semantic versioning for published open source assets. Each version is a tagged Git commit in the
community repository. The import system resolves dependencies between open source assets and
validates engine version compatibility using semver ranges. Older versions remain accessible via Git
history. Dependency resolution follows the same rules as the paid marketplace (F-15.17.2).

- **Requirements:** R-15.17.12
- **Dependencies:** F-15.17.9, F-15.17.2
- **Platform notes:** None

### F-15.17.13 One-Click Import

Download and import open source assets directly into a project from the browser. Auto-configure
materials, prefabs, and animations based on asset metadata. The import pipeline validates engine
version compatibility, resolves transitive dependencies from the open source repository, and places
assets in a designated community assets folder. Imported assets appear in the asset browser
(F-12.3.1) immediately.

- **Requirements:** R-15.17.13
- **Dependencies:** F-15.17.9, F-15.17.2
- **Platform notes:** None

## External Store Integration

### F-15.17.14 FAB (Epic) Integration

Browse and purchase assets from Epic's FAB marketplace within the editor. OAuth 2.0 login
authenticates the user's Epic account. The integration displays FAB listings with prices, ratings,
and previews. Purchased assets are downloaded and auto-imported with format conversion (FBX to
engine format, material remapping). The engine takes no commission on FAB purchases. License terms
from FAB are tracked per-project alongside locally purchased assets.

- **Requirements:** R-15.17.14
- **Dependencies:** F-15.17.1, F-15.17.2, F-12.1.1 (Asset Import)
- **Platform notes:** Requires Epic account and FAB API access.

### F-15.17.15 Synty Store Integration

Browse Synty asset packs from within the editor. Purchase through Synty's storefront via embedded
browser or OAuth flow. Downloaded packs are auto-imported with material conversion to the engine's
PBR pipeline (albedo, normal, roughness, metallic channel remapping). Synty's low-poly art style
metadata is preserved for LOD and rendering optimization. The engine takes no commission.

- **Requirements:** R-15.17.15
- **Dependencies:** F-15.17.1, F-15.17.2, F-15.3.1 (Materials)
- **Platform notes:** Material conversion handles Synty's Unity and Unreal material formats.

### F-15.17.16 TurboSquid Integration

Search and import 3D models from TurboSquid within the editor. Supports format conversion for FBX,
OBJ, and glTF imports. The integration displays TurboSquid listings with poly counts, texture
resolutions, and format availability. Auto-generates LODs for imported models. UV unwrapping is
validated and repaired if needed. The engine takes no commission on TurboSquid purchases.

- **Requirements:** R-15.17.16
- **Dependencies:** F-15.17.1, F-15.17.2, F-12.1.1 (Asset Import)
- **Platform notes:** None

### F-15.17.17 Generic Store API

A plugin API for integrating additional third-party asset stores beyond FAB, Synty, and TurboSquid.
The API provides a standardized interface for: browsing catalogs, authenticating users, processing
purchases, downloading assets, and importing with format conversion. Store plugins register with the
asset store browser and appear as additional tabs. The API handles credential storage, download
progress tracking, and license term propagation.

- **Requirements:** R-15.17.17
- **Dependencies:** F-15.17.1, F-15.17.2, F-15.20.1 (Plugin Arch)
- **Platform notes:** None

### F-15.17.18 License Compliance Tracking

Track licenses of all imported assets per-project across all sources (open source store, FAB, Synty,
TurboSquid, other stores). Generate a license compliance report for distribution that lists every
third-party asset, its license type, attribution requirements, and redistribution rights. Warn on
incompatible license combinations (e.g., GPL asset in a proprietary project). The report is
exportable as Markdown, JSON, and PDF.

- **Requirements:** R-15.17.18
- **Dependencies:** F-15.17.8, F-15.17.14, F-15.17.15, F-15.17.16
- **Platform notes:** None

## AI Content Generation

### F-15.17.19 AI Texture Generation

Generate textures from text prompts using diffusion-based models. Supports seamless tiling and PBR
channel generation (albedo, normal, roughness, metallic, ambient occlusion). Generated textures
integrate with the material editor (F-15.3.1) and are tagged with AI provenance metadata (F-15.7.1).
Resolution options from 256x256 to 4096x4096. Batch generation for texture variations. All
generation runs through the local AI inference system (F-15.17.27).

- **Requirements:** R-15.17.19
- **Dependencies:** F-15.3.1, F-15.7.1, F-15.17.27
- **Platform notes:** GPU-accelerated; requires Metal, Vulkan, or D3D12 compute.

### F-15.17.20 AI Mesh Generation

Generate 3D meshes from text prompts or reference images. Output includes clean quad-dominant
topology, automatic UV unwrapping, and auto-generated LOD chain. Meshes are import-ready with
collision geometry. Poly count is controllable via a target budget parameter. Generated meshes
receive AI provenance tags (F-15.7.1) and are editable in DCC tools after export.

- **Requirements:** R-15.17.20
- **Dependencies:** F-12.2.2 (LOD Generation), F-15.7.1, F-15.17.27
- **Platform notes:** GPU-accelerated inference.

### F-15.17.21 AI Animation Generation

Generate character animations from text descriptions (e.g., "tired walk cycle", "sword slash
combo"). Output is retargetable to any character skeleton via the retargeting system (F-9.1.8).
Supports motion matching integration for blending AI-generated clips with hand-authored animations.
Generated clips include root motion data and animation events.

- **Requirements:** R-15.17.21
- **Dependencies:** F-9.1.8, F-9.4.1, F-15.7.1, F-15.17.27
- **Platform notes:** GPU-accelerated inference.

### F-15.17.22 AI Audio Generation

Generate sound effects from text prompts (e.g., "metallic clang", "forest ambience at night").
Generate music variations from a seed track or style description. Output is in the engine's native
audio format with metadata for spatial audio integration. Supports batch generation of sound effect
variations for auditory variety. Generated audio receives AI provenance tags (F-15.7.1).

- **Requirements:** R-15.17.22
- **Dependencies:** F-5.1.3, F-15.7.1, F-15.17.27
- **Platform notes:** GPU-accelerated inference.

### F-15.17.23 AI Level Layout

Generate level layouts from designer-specified constraints: genre, size, difficulty curve, theme,
and gameplay flow. The generator places props, sets lighting, sculpts terrain, and paints materials.
Output is a fully editable level in the editor (F-15.2.1) that designers can refine. Constraints are
defined via a visual interface, not code. Supports iterative regeneration of specific level regions.

- **Requirements:** R-15.17.23
- **Dependencies:** F-15.2.1, F-15.6.1, F-15.7.1, F-15.17.27
- **Platform notes:** GPU-accelerated inference.

### F-15.17.24 AI Material Generation

Generate PBR materials from text descriptions or photo references. Output is a material graph in the
material editor (F-15.3.1) with auto-generated nodes for albedo, normal, roughness, metallic, and
height channels. Supports material variations (weathered, clean, damaged). Generated materials are
fully editable in the material graph and tagged with AI provenance (F-15.7.1).

- **Requirements:** R-15.17.24
- **Dependencies:** F-15.3.1, F-15.7.1, F-15.17.27
- **Platform notes:** GPU-accelerated inference.

### F-15.17.25 AI VFX Generation

Generate particle effects and VFX graphs from text descriptions (e.g., "campfire with sparks",
"magical portal swirl", "explosion with debris"). Output is a VFX effect graph (F-11.6.1) with
configured emitters, forces, and rendering parameters. Generated effects are fully editable in the
VFX editor. Supports style transfer from reference VFX to new descriptions.

- **Requirements:** R-15.17.25
- **Dependencies:** F-11.6.1, F-15.7.1, F-15.17.27
- **Platform notes:** GPU-accelerated inference.

### F-15.17.26 AI Content Iteration

Refine AI-generated content with follow-up prompts (e.g., "make the texture more weathered",
"increase the walk cycle speed"). Supports undo/redo for all AI operations integrated with the
editor's command history (F-15.1.3). Blend AI-generated output with hand-authored content using
masking and layering. Iteration history is preserved in the AI provenance trail (F-15.7.6).

- **Requirements:** R-15.17.26
- **Dependencies:** F-15.1.3, F-15.7.1, F-15.7.6, F-15.17.27
- **Platform notes:** None

### F-15.17.27 Local AI Inference

Run AI models locally without cloud connectivity. GPU-accelerated inference using Metal (macOS),
Vulkan (Linux), or D3D12 (Windows) compute shaders. Model download and caching via the shared cache
system (F-15.11.1). Supports ONNX and safetensors model formats. Memory management respects GPU VRAM
budgets and falls back to CPU inference when GPU memory is insufficient. No telemetry or data
collection from local inference.

- **Requirements:** R-15.17.27
- **Dependencies:** F-15.11.1, F-2.1.1 (GPU Backend)
- **Platform notes:** Metal on macOS, Vulkan on Linux, D3D12 on Windows. No cloud fallback by
  default.

### F-15.17.28 AI Content Governance

Provenance tracking for all AI-generated content using the engine's AI governance system (F-15.7.1).
Opt-out lists for artists who do not consent to their work being used in training data. License-
safe training data verification that audits model training sources against known license databases.
Integration with the AI content review workflow (F-15.7.7) for mandatory review of AI-generated
assets before production use.

- **Requirements:** R-15.17.28
- **Dependencies:** F-15.7.1, F-15.7.6, F-15.7.7
- **Platform notes:** None

## Feature Summary

| ID | Name | Category |
|----|------|----------|
| F-15.17.1 | Integrated Asset Store Browser | Browsing |
| F-15.17.2 | One-Click Asset Import | Browsing |
| F-15.17.3 | Asset Ratings, Reviews, Curation | Browsing |
| F-15.17.4 | Publisher Account and Dashboard | Publisher |
| F-15.17.5 | Automated Compatibility Testing | Publisher |
| F-15.17.6 | Revenue Sharing and Payout | Publisher |
| F-15.17.7 | Asset Type Support | Licensing |
| F-15.17.8 | License Management and DRM | Licensing |
| F-15.17.9 | Open Source Asset Browser | OS Store |
| F-15.17.10 | Asset Upload and Publishing | OS Store |
| F-15.17.11 | Asset Rating and Reviews | OS Store |
| F-15.17.12 | Asset Versioning | OS Store |
| F-15.17.13 | One-Click Import | OS Store |
| F-15.17.14 | FAB (Epic) Integration | External |
| F-15.17.15 | Synty Store Integration | External |
| F-15.17.16 | TurboSquid Integration | External |
| F-15.17.17 | Generic Store API | External |
| F-15.17.18 | License Compliance Tracking | External |
| F-15.17.19 | AI Texture Generation | AI Gen |
| F-15.17.20 | AI Mesh Generation | AI Gen |
| F-15.17.21 | AI Animation Generation | AI Gen |
| F-15.17.22 | AI Audio Generation | AI Gen |
| F-15.17.23 | AI Level Layout | AI Gen |
| F-15.17.24 | AI Material Generation | AI Gen |
| F-15.17.25 | AI VFX Generation | AI Gen |
| F-15.17.26 | AI Content Iteration | AI Gen |
| F-15.17.27 | Local AI Inference | AI Gen |
| F-15.17.28 | AI Content Governance | AI Gen |
