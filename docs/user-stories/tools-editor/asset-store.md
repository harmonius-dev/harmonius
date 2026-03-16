# User Stories: Asset Marketplace

## F-15.17.1 Integrated Asset Store Browser

## US-15.17.1.1 Designer Browses Marketplace In-Editor

**As a** designer (P-5), **I want** to browse and search the asset marketplace directly inside the
editor with category filters, tags, ratings, and semantic search, **so that** I can find gameplay
templates and tools without leaving my workspace.

## US-15.17.1.2 Artist Previews 3D Assets

**As an** artist (P-8), **I want** 3D model viewers (rotate, zoom), material previews on standard
meshes, audio playback, and screenshot galleries in asset listings, **so that** I can evaluate
visual quality before purchasing.

## US-15.17.1.3 Developer Searches Semantically

**As a** developer (P-15), **I want** semantic search beyond keyword matching, **so that** I can
find assets by describing what I need rather than guessing exact tag names.

## US-15.17.1.4 Modder Browses from Launcher

**As a** modder (P-24), **I want** the store accessible from both the editor main menu and the
launcher home screen, **so that** I can discover assets before opening a project.

## F-15.17.2 One-Click Asset Import and Project Integration

## US-15.17.2.1 Developer Imports with One Click

**As a** developer (P-15), **I want** purchased assets to import into my project with a single
click, automatically resolving dependencies (required plugins, material libraries), **so that** I do
not manually track transitive dependencies.

## US-15.17.2.2 Designer Gets Conflict Guidance

**As a** designer (P-5), **I want** version conflicts (asset requires plugin v2.0 but project has
v1.8) flagged with specific upgrade steps, **so that** I can resolve incompatibilities before they
cause runtime errors.

## US-15.17.2.3 Artist Sees Assets Immediately

**As an** artist (P-8), **I want** imported assets to appear in the asset browser immediately after
download, **so that** I can start using new content without restarting the editor.

## US-15.17.2.4 Engine Tester Validates Compatibility Check

**As an** engine tester (P-27), **I want** to verify that the import pipeline validates engine
version compatibility, required plugins, and platform support before importing, **so that**
incompatible assets are caught at import time.

## F-15.17.3 Asset Ratings, Reviews, and Curation

## US-15.17.3.1 Artist Rates and Reviews Assets

**As an** artist (P-8), **I want** to rate assets 1-5 stars with text reviews, **so that** other
artists can make informed purchasing decisions based on community feedback.

## US-15.17.3.2 Designer Browses Curated Collections

**As a** designer (P-5), **I want** staff-curated and community-curated collections organized by
theme (e.g., "Medieval Bundle", "Sci-Fi Starter Kit"), **so that** I can bootstrap projects with
visually consistent asset sets.

## US-15.17.3.3 Creative Director Reviews Featured Assets

**As a** creative director (P-2), **I want** featured assets, staff picks, and themed collections
highlighted in the store, **so that** high-quality content is discoverable.

## US-15.17.3.4 Engine Tester Validates Review Moderation

**As an** engine tester (P-27), **I want** to verify that reviews are moderated for spam and abuse,
**so that** the review system provides trustworthy feedback.

## F-15.17.4 Publisher Account and Dashboard

## US-15.17.4.1 Developer Registers as Publisher

**As a** developer (P-15), **I want** to register a verified publisher account with identity
verification and access a dashboard showing published assets, revenue analytics, and user reviews,
**so that** I can manage my asset business.

## US-15.17.4.2 Tech Artist Sets Regional Pricing

**As a** tech artist (P-13), **I want** to set prices in multiple currencies with regional pricing
overrides, **so that** my assets are competitively priced in each market.

## US-15.17.4.3 DevOps Runs Compatibility Tests

**As a** DevOps engineer (P-16), **I want** compatibility test results per engine version displayed
in the publisher dashboard, **so that** I can track which engine versions my assets support.

## US-15.17.4.4 Engine Tester Validates Dashboard Data

**As an** engine tester (P-27), **I want** to verify that revenue analytics, download statistics,
and review data are accurate and up-to-date, **so that** publishers can trust their dashboard
metrics.

## F-15.17.5 Automated Compatibility Testing

## US-15.17.5.1 DevOps Automates Asset Testing

**As a** DevOps engineer (P-16), **I want** the marketplace to automatically test all published
assets against each new engine version and award compatibility badges, **so that** buyers have
confidence assets work without publisher intervention.

## US-15.17.5.2 Developer Gets Failure Notifications

**As a** developer (P-15), **I want** to receive notifications with specific error details when my
asset fails compatibility testing, **so that** I can remediate quickly and maintain my badge.

## US-15.17.5.3 Designer Filters by Compatibility

**As a** designer (P-5), **I want** to filter store listings by compatibility badge for the current
engine version, **so that** I only see assets confirmed to work with my project.

## US-15.17.5.4 Engine Tester Validates Test Pipeline

**As an** engine tester (P-27), **I want** to verify that the compatibility testing pipeline imports
each asset, verifies loading, and runs included tests correctly, **so that** badges are meaningful.

## F-15.17.6 Revenue Sharing and Payout

## US-15.17.6.1 Developer Tracks Revenue

**As a** developer (P-15), **I want** monthly revenue reports showing per-asset sales, regional
breakdown, commission deductions (12% default), and refund reversals, **so that** I can track income
accurately.

## US-15.17.6.2 Tech Artist Publishes Free Assets

**As a** tech artist (P-13), **I want** free assets to cost nothing to publish, **so that** I can
share community tools and resources without financial barriers.

## US-15.17.6.3 Server Admin Manages Tax Compliance

**As a** server admin (P-22), **I want** tax documentation (W-9, W-8BEN) collected from publishers
for compliance, **so that** the marketplace meets tax reporting requirements.

## US-15.17.6.4 Engine Tester Validates Refund Processing

**As an** engine tester (P-27), **I want** to verify that refunds within the 14-day window
automatically reverse publisher revenue, **so that** financial data stays accurate.

## F-15.17.7 Asset Type Support

## US-15.17.7.1 Artist Publishes Any Asset Type

**As an** artist (P-8), **I want** to publish all engine asset types (3D meshes, materials, VFX
graphs, audio, terrain brushes, procedural generation graphs), **so that** the marketplace covers my
full content workflow.

## US-15.17.7.2 Developer Publishes Rust Plugins

**As a** developer (P-15), **I want** to publish compiled Rust plugins with per-platform binaries,
**so that** I can distribute engine extensions without requiring source compilation.

## US-15.17.7.3 Designer Downloads Logic Graph Templates

**As a** designer (P-5), **I want** to download pre-built logic graph templates for common gameplay
systems (inventory, dialogue, AI behavior), **so that** I start with proven implementations.

## US-15.17.7.4 Engine Tester Validates Type-Specific Previews

**As an** engine tester (P-27), **I want** to verify that each asset type has appropriate preview
rendering and metadata in store listings, **so that** all types are presented meaningfully.

## F-15.17.8 License Management and DRM

## US-15.17.8.1 Artist Understands License Terms

**As an** artist (P-8), **I want** clear license metadata (personal, commercial, redistribution
rights, attribution, seat count) on every asset listing and in the asset browser after import,
**so that** I know exactly what I am allowed to do.

## US-15.17.8.2 Developer Uses Assets Offline

**As a** developer (P-15), **I want** imported assets to work fully offline with no runtime license
checks or DRM, **so that** shipped builds never depend on the marketplace being reachable.

## US-15.17.8.3 Modder Checks Redistribution Rights

**As a** modder (P-24), **I want** to see whether an asset's license permits redistribution in mods,
**so that** I know which marketplace assets I can include in my mod packages.

## US-15.17.8.4 Engine Tester Validates License Validation

**As an** engine tester (P-27), **I want** to verify that license terms are validated at import time
and displayed in the asset browser, **so that** users are informed of restrictions before using
purchased content.
