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
