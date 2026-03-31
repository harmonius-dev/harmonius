# 15.17 — Asset Marketplace

## Browsing and Discovery

| ID | Feature |
| ----------- | ------------------------------------------------ |
| F-15.17.1 | Integrated Asset Store Browser |
| F-15.17.2 | One-Click Asset Import and Project Integration |
| F-15.17.3 | Asset Ratings, Reviews, and Curation |

1. **F-15.17.1** — An in-editor marketplace browser for discovering, previewing, and acquiring
   assets, plugins, and tools. The browser displays a searchable, filterable catalog with categories
   (3D models, materials, VFX, audio, logic graph templates, full game templates, plugins, tools),
   tags, ratings, reviews, download counts, and compatibility badges. Asset previews include 3D
   model viewers (rotate, zoom), material previews on standard meshes, audio playback, and
   screenshot galleries. Search supports text, tags, and semantic queries. The store is accessible
   from the editor's main menu and the launcher's home screen. Consider integration with Epic's FAB
   marketplace API for cross-engine asset access where asset formats are compatible.
   - **Deps:** F-15.1.1 (Editor Framework), F-12.3.1 (Asset Database)
   - **Platform:** Store API is REST-based, served by the cloud infrastructure (F-15.12.7).
2. **F-15.17.2** — Purchase/download an asset and automatically import it into the current project.
   The import pipeline validates compatibility (engine version, required plugins, platform support),
   resolves dependencies (an asset requiring a specific material library pulls it automatically),
   and installs to a designated marketplace assets folder. Imported assets appear in the asset
   browser (F-12.3.1) immediately. Version conflicts (asset requires plugin v2.0 but project has
   v1.8) are flagged with upgrade guidance. Free assets download without payment flow. Asset
   licenses are tracked per-project.
   - **Deps:** F-15.17.1, F-12.1.1 (Native Asset Ingestion), F-15.15.4 (Project File)
3. **F-15.17.3** — Players and developers rate assets 1-5 stars with text reviews. Reviews are
   moderated for spam and abuse. An editorial curation team highlights featured assets, staff picks,
   and themed collections (e.g., "Medieval Bundle", "Sci-Fi Starter Kit"). Community-driven
   collections allow users to curate and share lists of recommended assets. Assets display: average
   rating, review count, download count, last-updated date, engine version compatibility, and
   verified-publisher badge.
   - **Deps:** F-15.17.1, F-15.12.7 (Cloud Service)

## Publisher Tools

| ID | Feature |
| ----------- | --------------------------------- |
| F-15.17.4 | Publisher Account and Dashboard |
| F-15.17.5 | Automated Compatibility Testing |
| F-15.17.6 | Revenue Sharing and Payout |

1. **F-15.17.4** — Asset creators register publisher accounts with identity verification. The
   publisher dashboard displays: published assets, revenue analytics (sales, refunds, royalties),
   download statistics, user reviews (with response capability), compatibility test results per
   engine version, and payout history. Publishers set prices in multiple currencies, configure
   regional pricing, run time-limited sales and discount codes, and bundle assets into packs with
   aggregate pricing.
   - **Deps:** F-15.17.1, F-15.12.7 (Cloud Service)
2. **F-15.17.5** — When a new engine version is released, the marketplace automatically tests all
   published assets against it. The testing pipeline imports each asset into a test project,
   verifies it loads without errors, runs any included automated tests, and generates a
   compatibility report. Assets that pass receive a compatibility badge for the new version.
   Publishers receive notifications for assets that fail, with specific error details for
   remediation. Compatibility testing runs on CI infrastructure using the shared build cache
   (F-15.11.1).
   - **Deps:** F-15.17.4, F-15.11.1 (Shared Build Cache), F-15.15.2 (Project Upgrades)
3. **F-15.17.6** — The marketplace takes a configurable commission (default: 12%) on paid asset
   sales. Publishers receive monthly payouts via bank transfer, PayPal, or platform credits. Revenue
   reports detail per-asset sales, regional breakdown, and commission deductions. Refund processing
   automatically reverses publisher revenue for refunded purchases within the refund window (14
   days). Tax documentation (W-9, W-8BEN) is collected from publishers for compliance. Free assets
   cost nothing to publish.
   - **Deps:** F-15.17.4
   - **Platform:** Payment processing uses Stripe or equivalent for global payouts.

## Asset Types and Licensing

| ID | Feature |
| ----------- | ---------------------------- |
| F-15.17.7 | Asset Type Support |
| F-15.17.8 | License Management and DRM |

1. **F-15.17.7** — The marketplace supports all asset types producible by the engine: 3D meshes
   (with LODs, materials, collision), 2D sprites and sprite sheets, materials and material
   functions, VFX effect graphs, audio clips and music tracks, logic graph templates (gameplay
   systems, AI behaviors, tools), UI widget templates, animation clips and state machines, terrain
   brushes and biome presets, procedural generation graphs, full project templates (genre starters),
   and Rust plugins (compiled dynamic libraries per platform). Each type has appropriate preview
   rendering and metadata. platform-specific binaries.
   - **Deps:** F-15.17.1, F-12.7.1 (Binary Asset Format), F-13.1.10 (Rust Plugin API)
   - **Platform:** Rust plugins require per-platform compilation; the marketplace hosts
2. **F-15.17.8** — Assets carry license metadata: usage rights (personal, commercial,
   non-transferable), redistribution rights (can mods include this asset?), attribution
   requirements, and seat count (per-user or per-project). The engine validates licenses at import
   time and displays license terms in the asset browser. No runtime DRM -- once imported, assets
   work offline without license checks. License violations (using a personal-license asset in a
   commercial product) are contractual, not technical.
   - **Deps:** F-15.17.2, F-15.15.5 (Account Management)

## Open Source Asset Store

| ID | Feature |
| ------------ | ----------------------------- |
| F-15.17.9 | Open Source Asset Browser |
| F-15.17.10 | Asset Upload and Publishing |
| F-15.17.11 | Asset Rating and Reviews |
| F-15.17.12 | Asset Versioning |
| F-15.17.13 | One-Click Import |

1. **F-15.17.9** — Browse, search, and filter free and open-source assets hosted on the engine's
   community Git repository. The browser integrates into the same store UI as the paid marketplace
   (F-15.17.1) with a dedicated "Open Source" tab. Assets are indexed by license type (CC0, CC-BY,
   Apache 2.0, MIT), category, tags, and engine version compatibility. Community contributions are
   accepted via pull requests to the repository. The repository is cloneable for fully offline
   access.
   - **Deps:** F-15.17.1, F-15.10.1 (Git Integration)
   - **Platform:** Git-based storage; mirrors served via CDN.
2. **F-15.17.10** — Publish assets to the open source store with mandatory license tagging (CC0,
   CC-BY, Apache 2.0, MIT), metadata (description, category, tags), thumbnails, and preview media.
   Publishing creates a pull request to the community repository. Automated CI validates the asset
   (loads in a test project, passes lint checks, verifies license file presence). Maintainers review
   and merge. Published assets appear in the open source browser after merge.
   - **Deps:** F-15.17.9, F-15.17.5 (Compat Testing)
3. **F-15.17.11** — Community ratings (1-5 stars), text reviews, download counts, and popularity
   sorting for open source assets. Reviews are moderated by community maintainers. Assets display:
   average rating, review count, download count, last-updated date, contributor count, and license
   badge. Sorting options include: most downloaded, highest rated, newest, and recently updated.
   - **Deps:** F-15.17.9, F-15.17.3
   - **Platform:** Review data stored in the community repository as structured metadata files.
4. **F-15.17.12** — Semantic versioning for published open source assets. Each version is a tagged
   Git commit in the community repository. The import system resolves dependencies between open
   source assets and validates engine version compatibility using semver ranges. Older versions
   remain accessible via Git history. Dependency resolution follows the same rules as the paid
   marketplace (F-15.17.2).
   - **Deps:** F-15.17.9, F-15.17.2
5. **F-15.17.13** — Download and import open source assets directly into a project from the browser.
   Auto-configure materials, prefabs, and animations based on asset metadata. The import pipeline
   validates engine version compatibility, resolves transitive dependencies from the open source
   repository, and places assets in a designated community assets folder. Imported assets appear in
   the asset browser (F-12.3.1) immediately.
   - **Deps:** F-15.17.9, F-15.17.2

## External Store Integration

| ID | Feature |
| ------------ | ----------------------------- |
| F-15.17.14 | FAB (Epic) Integration |
| F-15.17.15 | Synty Store Integration |
| F-15.17.16 | TurboSquid Integration |
| F-15.17.17 | Generic Store API |
| F-15.17.18 | License Compliance Tracking |

1. **F-15.17.14** — Browse and purchase assets from Epic's FAB marketplace within the editor. OAuth
   2.0 login authenticates the user's Epic account. The integration displays FAB listings with
   prices, ratings, and previews. Purchased assets are downloaded and auto-imported with format
   conversion (FBX to engine format, material remapping). The engine takes no commission on FAB
   purchases. License terms from FAB are tracked per-project alongside locally purchased assets.
   - **Deps:** F-15.17.1, F-15.17.2, F-12.1.1 (Asset Import)
   - **Platform:** Requires Epic account and FAB API access.
2. **F-15.17.15** — Browse Synty asset packs from within the editor. Purchase through Synty's
   storefront via embedded browser or OAuth flow. Downloaded packs are auto-imported with material
   conversion to the engine's PBR pipeline (albedo, normal, roughness, metallic channel remapping).
   Synty's low-poly art style metadata is preserved for LOD and rendering optimization. The engine
   takes no commission.
   - **Deps:** F-15.17.1, F-15.17.2, F-15.3.1 (Materials)
   - **Platform:** Material conversion handles Synty's Unity and Unreal material formats.
3. **F-15.17.16** — Search and import 3D models from TurboSquid within the editor. Supports format
   conversion for FBX, OBJ, and glTF imports. The integration displays TurboSquid listings with poly
   counts, texture resolutions, and format availability. Auto-generates LODs for imported models. UV
   unwrapping is validated and repaired if needed. The engine takes no commission on TurboSquid
   purchases.
   - **Deps:** F-15.17.1, F-15.17.2, F-12.1.1 (Asset Import)
4. **F-15.17.17** — A plugin API for integrating additional third-party asset stores beyond FAB,
   Synty, and TurboSquid. The API provides a standardized interface for: browsing catalogs,
   authenticating users, processing purchases, downloading assets, and importing with format
   conversion. Store plugins register with the asset store browser and appear as additional tabs.
   The API handles credential storage, download progress tracking, and license term propagation.
   - **Deps:** F-15.17.1, F-15.17.2, F-15.20.1 (Plugin Arch)
5. **F-15.17.18** — Track licenses of all imported assets per-project across all sources (open
   source store, FAB, Synty, TurboSquid, other stores). Generate a license compliance report for
   distribution that lists every third-party asset, its license type, attribution requirements, and
   redistribution rights. Warn on incompatible license combinations (e.g., GPL asset in a
   proprietary project). The report is exportable as Markdown, JSON, and PDF.
   - **Deps:** F-15.17.8, F-15.17.14, F-15.17.15, F-15.17.16

## AI Content Generation

| ID | Feature |
| ------------ | ------------------------- |
| F-15.17.19 | AI Texture Generation |
| F-15.17.20 | AI Mesh Generation |
| F-15.17.21 | AI Animation Generation |
| F-15.17.22 | AI Audio Generation |
| F-15.17.23 | AI Level Layout |
| F-15.17.24 | AI Material Generation |
| F-15.17.25 | AI VFX Generation |
| F-15.17.26 | AI Content Iteration |
| F-15.17.27 | Local AI Inference |
| F-15.17.28 | AI Content Governance |

1. **F-15.17.19** — Generate textures from text prompts using diffusion-based models. Supports
   seamless tiling and PBR channel generation (albedo, normal, roughness, metallic, ambient
   occlusion). Generated textures integrate with the material editor (F-15.3.1) and are tagged with
   AI provenance metadata (F-15.7.1). Resolution options from 256x256 to 4096x4096. Batch generation
   for texture variations. All generation runs through the local AI inference system (F-15.17.27).
   - **Deps:** F-15.3.1, F-15.7.1, F-15.17.27
   - **Platform:** GPU-accelerated; requires Metal, Vulkan, or D3D12 compute.
2. **F-15.17.20** — Generate 3D meshes from text prompts or reference images. Output includes clean
   quad-dominant topology, automatic UV unwrapping, and auto-generated LOD chain. Meshes are
   import-ready with collision geometry. Poly count is controllable via a target budget parameter.
   Generated meshes receive AI provenance tags (F-15.7.1) and are editable in DCC tools after
   export.
   - **Deps:** F-12.2.2 (LOD Generation), F-15.7.1, F-15.17.27
   - **Platform:** GPU-accelerated inference.
3. **F-15.17.21** — Generate character animations from text descriptions (e.g., "tired walk cycle",
   "sword slash combo"). Output is retargetable to any character skeleton via the retargeting system
   (F-9.1.8). Supports motion matching integration for blending AI-generated clips with
   hand-authored animations. Generated clips include root motion data and animation events.
   - **Deps:** F-9.1.8, F-9.4.1, F-15.7.1, F-15.17.27
   - **Platform:** GPU-accelerated inference.
4. **F-15.17.22** — Generate sound effects from text prompts (e.g., "metallic clang", "forest
   ambience at night"). Generate music variations from a seed track or style description. Output is
   in the engine's native audio format with metadata for spatial audio integration. Supports batch
   generation of sound effect variations for auditory variety. Generated audio receives AI
   provenance tags (F-15.7.1).
   - **Deps:** F-5.1.3, F-15.7.1, F-15.17.27
   - **Platform:** GPU-accelerated inference.
5. **F-15.17.23** — Generate level layouts from designer-specified constraints: genre, size,
   difficulty curve, theme, and gameplay flow. The generator places props, sets lighting, sculpts
   terrain, and paints materials. Output is a fully editable level in the editor (F-15.2.1) that
   designers can refine. Constraints are defined via a visual interface, not code. Supports
   iterative regeneration of specific level regions.
   - **Deps:** F-15.2.1, F-15.6.1, F-15.7.1, F-15.17.27
   - **Platform:** GPU-accelerated inference.
6. **F-15.17.24** — Generate PBR materials from text descriptions or photo references. Output is a
   material graph in the material editor (F-15.3.1) with auto-generated nodes for albedo, normal,
   roughness, metallic, and height channels. Supports material variations (weathered, clean,
   damaged). Generated materials are fully editable in the material graph and tagged with AI
   provenance (F-15.7.1).
   - **Deps:** F-15.3.1, F-15.7.1, F-15.17.27
   - **Platform:** GPU-accelerated inference.
7. **F-15.17.25** — Generate particle effects and VFX graphs from text descriptions (e.g., "campfire
   with sparks", "magical portal swirl", "explosion with debris"). Output is a VFX effect graph
   (F-11.6.1) with configured emitters, forces, and rendering parameters. Generated effects are
   fully editable in the VFX editor. Supports style transfer from reference VFX to new descriptions.
   - **Deps:** F-11.6.1, F-15.7.1, F-15.17.27
   - **Platform:** GPU-accelerated inference.
8. **F-15.17.26** — Refine AI-generated content with follow-up prompts (e.g., "make the texture more
   weathered", "increase the walk cycle speed"). Supports undo/redo for all AI operations integrated
   with the editor's command history (F-15.1.3). Blend AI-generated output with hand-authored
   content using masking and layering. Iteration history is preserved in the AI provenance trail
   (F-15.7.6).
   - **Deps:** F-15.1.3, F-15.7.1, F-15.7.6, F-15.17.27
9. **F-15.17.27** — Run AI models locally without cloud connectivity. GPU-accelerated inference
   using Metal (macOS), Vulkan (Linux), or D3D12 (Windows) compute shaders. Model download and
   caching via the shared cache system (F-15.11.1). Supports ONNX and safetensors model formats.
   Memory management respects GPU VRAM budgets and falls back to CPU inference when GPU memory is
   insufficient. No telemetry or data collection from local inference. default.
   - **Deps:** F-15.11.1, F-2.1.1 (GPU Backend)
   - **Platform:** Metal on macOS, Vulkan on Linux, D3D12 on Windows. No cloud fallback by
10. **F-15.17.28** — Provenance tracking for all AI-generated content using the engine's AI
    governance system (F-15.7.1). Opt-out lists for artists who do not consent to their work being
    used in training data. License- safe training data verification that audits model training
    sources against known license databases. Integration with the AI content review workflow
    (F-15.7.7) for mandatory review of AI-generated assets before production use.
    - **Deps:** F-15.7.1, F-15.7.6, F-15.7.7

## Feature Summary

| ID | Feature |
| ---- | --------- |
