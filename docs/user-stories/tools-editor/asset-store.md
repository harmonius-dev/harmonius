# User Stories: Asset Marketplace

## US-15.17.1 Browsing the Marketplace In-Editor

**As a** designer (P-5), **I want** to browse and search the asset marketplace directly inside
the editor with category filters, tags, and semantic search, **so that** I can find gameplay
templates, VFX presets, and UI widgets without leaving my workspace.

## US-15.17.2 Previewing Assets Before Purchase

**As an** environment artist (P-8), **I want** to rotate, zoom, and inspect 3D models and
preview materials on standard meshes inside the store browser, **so that** I can evaluate visual
quality and style fit before committing to a purchase.

## US-15.17.3 One-Click Import with Dependency Resolution

**As a** game developer (P-15), **I want** purchased assets to import into my project with a
single click, automatically pulling any required plugins or material libraries, **so that** I
do not have to manually track and install transitive dependencies.

## US-15.17.4 Version Conflict Guidance on Import

**As a** game developer (P-15), **I want** the import pipeline to flag version conflicts between
an asset's required plugin version and my project's installed version with specific upgrade
steps, **so that** I can resolve incompatibilities before they cause runtime errors.

## US-15.17.5 Free Asset Discovery for Prototyping

**As a** designer (P-5), **I want** to filter the store to show only free assets and download
them without a payment flow, **so that** I can rapidly prototype game mechanics with placeholder
content at no cost.

## US-15.17.6 Rating and Reviewing Assets

**As a** character artist (P-9), **I want** to rate assets 1-5 stars and write a text review
describing quality, usability, and any issues, **so that** other artists can make informed
purchasing decisions based on community feedback.

## US-15.17.7 Curated Collections for Themed Projects

**As a** designer (P-5), **I want** to browse staff-curated and community-curated asset
collections organized by theme (e.g., "Medieval Bundle", "Sci-Fi Starter Kit"), **so that** I
can bootstrap a new project with a cohesive set of visually consistent assets.

## US-15.17.8 Publishing Assets to the Marketplace

**As an** asset store developer (P-25), **I want** to register a verified publisher account,
upload assets with metadata and previews, set pricing with regional overrides, and track
revenue analytics on a dashboard, **so that** I can monetize my reusable content and grow my
asset business.

## US-15.17.9 Responding to User Reviews

**As an** asset store developer (P-25), **I want** to respond to user reviews on my published
assets with clarifications or update announcements, **so that** I can address criticism, build
community trust, and communicate fixes.

## US-15.17.10 Running Sales and Discount Codes

**As an** asset store developer (P-25), **I want** to create time-limited sales, percentage
discounts, and promotional codes for my assets, **so that** I can run marketing campaigns and
increase sales volume during events or engine version launches.

## US-15.17.11 Automated Compatibility Badges

**As an** asset store developer (P-25), **I want** the marketplace to automatically test my
published assets against each new engine version and award a compatibility badge on success,
**so that** buyers have confidence my assets work and I only need to intervene when something
actually breaks.

## US-15.17.12 Compatibility Failure Notifications

**As an** asset store developer (P-25), **I want** to receive a notification with specific error
details when my asset fails automated compatibility testing on a new engine version, **so that**
I can remediate the issue quickly and maintain my compatibility badge.

## US-15.17.13 Revenue Reports and Payout Tracking

**As an** asset store developer (P-25), **I want** monthly revenue reports showing per-asset
sales, regional breakdown, commission deductions, and refund reversals, **so that** I can
track my income accurately and plan my content roadmap based on what sells.

## US-15.17.14 Publishing All Engine Asset Types

**As an** environment artist (P-8), **I want** to publish any asset type the engine supports --
3D meshes with LODs, materials, VFX graphs, terrain brushes, and procedural generation presets
-- with type-appropriate previews, **so that** the marketplace covers my full content workflow.

## US-15.17.15 Publishing Rust Plugins

**As a** game developer (P-15), **I want** to publish compiled Rust plugins to the marketplace
with per-platform binaries and automatic compatibility testing, **so that** I can distribute
engine extensions to other developers without requiring them to compile from source.

## US-15.17.16 Downloading Logic Graph Templates

**As a** designer (P-5), **I want** to download pre-built logic graph templates for common
gameplay systems (inventory, dialogue, AI behavior trees) from the marketplace, **so that** I
can start with proven implementations and customize them rather than building from scratch.

## US-15.17.17 Understanding Asset License Terms

**As an** environment artist (P-8), **I want** to see clear license terms (personal, commercial,
redistribution rights, attribution requirements) on every asset listing and in the asset browser
after import, **so that** I know exactly what I am allowed to do with purchased content.

## US-15.17.18 Offline Use After Import

**As a** game developer (P-15), **I want** imported marketplace assets to work fully offline
with no runtime license checks or DRM, **so that** my game's shipping build never depends on
the marketplace being reachable.

## US-15.17.19 Resumable Downloads on Unreliable Connections

**As a** character artist (P-9), **I want** asset downloads to resume from where they left off
after a network interruption rather than restarting, **so that** I do not waste time and
bandwidth re-downloading large asset packs on unstable connections.
