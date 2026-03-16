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

---

## F-15.17.9 Open Source Asset Browser

### US-15.17.9.1 Environment Artist Finds Free Assets

**As an** environment artist (P-8), **I want** to browse and search the open source asset store
filtered by license type (CC0, CC-BY), category, and engine version, **so that** I can find free,
legally clear assets for my environment scenes without cost.

### US-15.17.9.2 Modder Browses Community Assets

**As a** modder (P-24), **I want** to browse the community asset repository with sorting by
popularity, rating, and recency, **so that** I can discover the best community-contributed content
for my mods.

### US-15.17.9.3 Modder Clones Repository Offline

**As a** modder (P-24), **I want** to clone the entire open source asset repository for fully
offline access, **so that** I can browse and use community assets without network connectivity.

## F-15.17.10 Asset Upload and Publishing

### US-15.17.10.1 Technical Artist Publishes Asset

**As a** technical artist (P-13), **I want** to publish an asset to the open source store with
license tagging (CC0, Apache 2.0), metadata, thumbnails, and preview media via a pull request
workflow, **so that** I can contribute back to the community with proper license attribution.

### US-15.17.10.2 Technical Artist Gets CI Feedback

**As a** technical artist (P-13), **I want** automated CI to validate my submitted asset (load test,
lint, license file check) before maintainers review it, **so that** I get fast feedback on any
issues with my contribution.

### US-15.17.10.3 Environment Artist Updates Asset

**As an** environment artist (P-8), **I want** to publish updated versions of my open source assets
with changelogs, **so that** users of my assets can see what changed and upgrade safely.

## F-15.17.11 Asset Rating and Reviews

### US-15.17.11.1 Environment Artist Rates Assets

**As an** environment artist (P-8), **I want** to rate and review open source assets with 1-5 stars
and text feedback, **so that** the community can identify the highest quality free assets.

## F-15.17.12 Asset Versioning

### US-15.17.12.1 Technical Artist Versions Assets

**As a** technical artist (P-13), **I want** semantic versioning for my published open source assets
with engine version compatibility declarations, **so that** users know which engine versions my
asset supports.

## F-15.17.13 One-Click Import

### US-15.17.13.1 Environment Artist Imports with One Click

**As an** environment artist (P-8), **I want** to import open source assets into my project with one
click, auto-configuring materials, prefabs, and animations, **so that** I can use community assets
immediately without manual setup.

---

## F-15.17.14 FAB (Epic) Integration

### US-15.17.14.1 Level Designer Imports from FAB

**As a** level designer (P-6), **I want** to browse and purchase assets from Epic's FAB marketplace
within the editor, with OAuth login and auto-import including format conversion, **so that** I can
access FAB's large asset catalog without leaving my workspace.

### US-15.17.14.2 Level Designer Previews FAB Assets

**As a** level designer (P-6), **I want** to preview FAB asset listings with prices, ratings, and 3D
previews inside the editor, **so that** I can evaluate assets before purchasing them.

### US-15.17.14.3 Level Designer Gets Auto-Converted Materials

**As a** level designer (P-6), **I want** FAB assets to be auto-imported with FBX-to-engine format
conversion and material remapping, **so that** imported assets render correctly without manual
material fixes.

## F-15.17.15 Synty Store Integration

### US-15.17.15.1 Level Designer Browses Synty Packs

**As a** level designer (P-6), **I want** to browse Synty asset packs within the editor and purchase
them through Synty's storefront, **so that** I can quickly acquire low-poly art packs for
prototyping.

## F-15.17.16 TurboSquid Integration

### US-15.17.16.1 Environment Artist Imports from TurboSquid

**As an** environment artist (P-8), **I want** to search and import 3D models from TurboSquid with
automatic format conversion (FBX, OBJ, glTF), LOD generation, and UV validation, **so that** I can
use high-quality third-party models without manual pipeline steps.

## F-15.17.17 Generic Store API

### US-15.17.17.1 Technical Artist Integrates New Store

**As a** technical artist (P-13), **I want** a generic store plugin API that allows integrating
additional third-party asset stores as new tabs in the store browser, **so that** our team can
access assets from niche stores specific to our art style.

## F-15.17.18 License Compliance Tracking

### US-15.17.18.1 DevOps Checks License Compliance

**As a** DevOps engineer (P-16), **I want** to generate a license compliance report for all imported
assets in my project listing each asset's license type, attribution requirements, and redistribution
rights, **so that** I can verify legal compliance before distributing the game.

### US-15.17.18.2 DevOps Gets Incompatibility Warnings

**As a** DevOps engineer (P-16), **I want** warnings when imported assets have incompatible license
combinations (e.g., GPL in a proprietary project), **so that** license conflicts are caught before
distribution.

### US-15.17.18.3 DevOps Exports Report

**As a** DevOps engineer (P-16), **I want** to export the license compliance report as Markdown,
JSON, or PDF, **so that** I can share it with legal review and include it in release documentation.

---

## F-15.17.19 AI Texture Generation

### US-15.17.19.1 Character Artist Generates AI Textures

**As a** character artist (P-9), **I want** to generate PBR textures from text prompts with seamless
tiling and automatic albedo, normal, roughness, and metallic channel output, **so that** I can
rapidly create material textures for character models without painting from scratch.

### US-15.17.19.2 Character Artist Generates Variations

**As a** character artist (P-9), **I want** to batch-generate texture variations (weathered, clean,
damaged) from a single prompt, **so that** I have multiple options to choose from without running
each variation manually.

### US-15.17.19.3 Character Artist Controls Resolution

**As a** character artist (P-9), **I want** to choose texture resolution from 256x256 to 4096x4096,
**so that** I can balance quality and memory budget for different use cases.

## F-15.17.20 AI Mesh Generation

### US-15.17.20.1 Environment Artist Generates Props

**As an** environment artist (P-8), **I want** to generate 3D meshes from text prompts with clean
topology, auto UV unwrapping, LOD chain, and collision geometry, **so that** I can quickly populate
scenes with varied props during level design.

### US-15.17.20.2 Environment Artist Controls Poly Budget

**As an** environment artist (P-8), **I want** to set a target poly count for AI-generated meshes,
**so that** the output fits my project's performance budget.

## F-15.17.21 AI Animation Generation

### US-15.17.21.1 Animator Generates Walk Cycle

**As a** character animator (P-11), **I want** to generate character animations from text
descriptions (e.g., "tired walk cycle") that are retargetable to any skeleton, **so that** I can
quickly create animation drafts for iteration.

### US-15.17.21.2 Animator Blends AI with Hand-Authored

**As a** character animator (P-11), **I want** to blend AI-generated animation clips with
hand-authored animations via motion matching, **so that** AI output can supplement my library
without replacing hand-crafted quality.

### US-15.17.21.3 Animator Gets Root Motion Data

**As a** character animator (P-11), **I want** AI-generated animations to include root motion data
and animation events, **so that** they integrate with gameplay systems without manual event tagging.

## F-15.17.22 AI Audio Generation

### US-15.17.22.1 Audio Designer Generates Sound Effects

**As an** audio designer (P-14), **I want** to generate sound effects from text prompts (e.g.,
"metallic clang", "forest ambience") in the engine's native audio format, **so that** I can rapidly
fill out the audio palette during prototyping.

### US-15.17.22.2 Audio Designer Generates Variations

**As an** audio designer (P-14), **I want** to batch-generate sound effect variations from a single
prompt, **so that** I have auditory variety for repeated gameplay events (footsteps, impacts).

### US-15.17.22.3 Audio Designer Generates Music Variations

**As an** audio designer (P-14), **I want** to generate music variations from a seed track or style
description, **so that** I can create adaptive music layers without composing each variation.

## F-15.17.23 AI Level Layout

### US-15.17.23.1 Designer Uses AI Level Layout

**As a** game designer (P-5), **I want** to generate level layouts by specifying constraints (genre,
size, difficulty, theme) via a visual interface, **so that** I get a playable starting point for
level design without manually placing every prop.

### US-15.17.23.2 Designer Iterates on Regions

**As a** game designer (P-5), **I want** to regenerate specific regions of an AI-generated level
while keeping the rest intact, **so that** I can iteratively refine layouts without starting over.

### US-15.17.23.3 Designer Edits AI Output

**As a** game designer (P-5), **I want** AI-generated levels to be fully editable in the level
editor with all entities selectable and modifiable, **so that** AI output is a starting point, not a
locked result.

## F-15.17.24 AI Material Generation

### US-15.17.24.1 Environment Artist Generates Materials

**As an** environment artist (P-8), **I want** to generate PBR materials from text descriptions or
photo references as editable material graphs, **so that** I can create and tweak materials faster
than building graphs from scratch.

## F-15.17.25 AI VFX Generation

### US-15.17.25.1 VFX Artist Generates Particle Effects

**As a** VFX artist (P-12), **I want** to generate particle effects from text descriptions (e.g.,
"campfire with sparks", "magical portal swirl") as editable VFX effect graphs, **so that** I can
rapidly prototype visual effects and refine them.

### US-15.17.25.2 VFX Artist Transfers Style

**As a** VFX artist (P-12), **I want** to apply style transfer from a reference VFX to a new text
description, **so that** I can maintain visual consistency across generated effects.

### US-15.17.25.3 VFX Artist Edits Generated Effects

**As a** VFX artist (P-12), **I want** AI-generated VFX graphs to be fully editable in the VFX
editor with all emitters, forces, and parameters exposed, **so that** I can hand-tune the output.

## F-15.17.26 AI Content Iteration

### US-15.17.26.1 Character Artist Refines AI Output

**As a** character artist (P-9), **I want** to refine AI-generated content with follow-up prompts
(e.g., "make the texture more weathered") and undo/redo each AI operation, **so that** I can
iteratively improve output without losing previous states.

### US-15.17.26.2 Animator Blends AI and Manual Work

**As a** character animator (P-11), **I want** to blend AI-generated content with hand-authored
content using masking and layering, **so that** I can combine AI efficiency with manual precision.

## F-15.17.27 Local AI Inference

### US-15.17.27.1 Environment Artist Runs AI Offline

**As an** environment artist (P-8), **I want** all AI generation to run locally on my GPU without
cloud connectivity, **so that** I can use AI features on air-gapped workstations and protect project
IP.

### US-15.17.27.2 Technical Artist Manages Model Cache

**As a** technical artist (P-13), **I want** AI models to be downloaded and cached via the shared
cache system, **so that** the team shares one download and I do not re-download models on every
workstation.

## F-15.17.28 AI Content Governance

### US-15.17.28.1 DevOps Audits AI Content Provenance

**As a** DevOps engineer (P-16), **I want** every AI-generated asset to carry provenance metadata
(model, timestamp, prompt hash) tracked by the governance system, **so that** I can audit which
assets in a build are AI-generated for regulatory compliance.

### US-15.17.28.2 Character Artist Opts Out of Training

**As a** character artist (P-9), **I want** an opt-out list that prevents my published artwork from
being used in AI training data, **so that** my creative rights are respected.

### US-15.17.28.3 DevOps Verifies Training Data Licenses

**As a** DevOps engineer (P-16), **I want** license-safe training data verification that audits
model training sources against known license databases, **so that** AI-generated assets in our
project are defensible.

---

## Story Summary

| ID | Persona | Feature | Category |
|----|---------|---------|----------|
| US-15.17.9.1 | P-8 Environment Artist | F-15.17.9 | OS Store |
| US-15.17.9.2 | P-24 Modder | F-15.17.9 | OS Store |
| US-15.17.9.3 | P-24 Modder | F-15.17.9 | OS Store |
| US-15.17.10.1 | P-13 Technical Artist | F-15.17.10 | OS Store |
| US-15.17.10.2 | P-13 Technical Artist | F-15.17.10 | OS Store |
| US-15.17.10.3 | P-8 Environment Artist | F-15.17.10 | OS Store |
| US-15.17.11.1 | P-8 Environment Artist | F-15.17.11 | OS Store |
| US-15.17.12.1 | P-13 Technical Artist | F-15.17.12 | OS Store |
| US-15.17.13.1 | P-8 Environment Artist | F-15.17.13 | OS Store |
| US-15.17.14.1 | P-6 Level Designer | F-15.17.14 | External |
| US-15.17.14.2 | P-6 Level Designer | F-15.17.14 | External |
| US-15.17.14.3 | P-6 Level Designer | F-15.17.14 | External |
| US-15.17.15.1 | P-6 Level Designer | F-15.17.15 | External |
| US-15.17.16.1 | P-8 Environment Artist | F-15.17.16 | External |
| US-15.17.17.1 | P-13 Technical Artist | F-15.17.17 | External |
| US-15.17.18.1 | P-16 DevOps | F-15.17.18 | External |
| US-15.17.18.2 | P-16 DevOps | F-15.17.18 | External |
| US-15.17.18.3 | P-16 DevOps | F-15.17.18 | External |
| US-15.17.19.1 | P-9 Character Artist | F-15.17.19 | AI Gen |
| US-15.17.19.2 | P-9 Character Artist | F-15.17.19 | AI Gen |
| US-15.17.19.3 | P-9 Character Artist | F-15.17.19 | AI Gen |
| US-15.17.20.1 | P-8 Environment Artist | F-15.17.20 | AI Gen |
| US-15.17.20.2 | P-8 Environment Artist | F-15.17.20 | AI Gen |
| US-15.17.21.1 | P-11 Animator | F-15.17.21 | AI Gen |
| US-15.17.21.2 | P-11 Animator | F-15.17.21 | AI Gen |
| US-15.17.21.3 | P-11 Animator | F-15.17.21 | AI Gen |
| US-15.17.22.1 | P-14 Audio Designer | F-15.17.22 | AI Gen |
| US-15.17.22.2 | P-14 Audio Designer | F-15.17.22 | AI Gen |
| US-15.17.22.3 | P-14 Audio Designer | F-15.17.22 | AI Gen |
| US-15.17.23.1 | P-5 Designer | F-15.17.23 | AI Gen |
| US-15.17.23.2 | P-5 Designer | F-15.17.23 | AI Gen |
| US-15.17.23.3 | P-5 Designer | F-15.17.23 | AI Gen |
| US-15.17.24.1 | P-8 Environment Artist | F-15.17.24 | AI Gen |
| US-15.17.25.1 | P-12 VFX Artist | F-15.17.25 | AI Gen |
| US-15.17.25.2 | P-12 VFX Artist | F-15.17.25 | AI Gen |
| US-15.17.25.3 | P-12 VFX Artist | F-15.17.25 | AI Gen |
| US-15.17.26.1 | P-9 Character Artist | F-15.17.26 | AI Gen |
| US-15.17.26.2 | P-11 Animator | F-15.17.26 | AI Gen |
| US-15.17.27.1 | P-8 Environment Artist | F-15.17.27 | AI Gen |
| US-15.17.27.2 | P-13 Technical Artist | F-15.17.27 | AI Gen |
| US-15.17.28.1 | P-16 DevOps | F-15.17.28 | AI Gov |
| US-15.17.28.2 | P-9 Character Artist | F-15.17.28 | AI Gov |
| US-15.17.28.3 | P-16 DevOps | F-15.17.28 | AI Gov |
