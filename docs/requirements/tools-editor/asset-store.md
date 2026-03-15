# R-15.17 -- Asset Marketplace Requirements

## R-15.17.1 Integrated Asset Store Browser
The engine **SHALL** provide an in-editor marketplace browser accessible from the editor main menu
and the launcher home screen. The browser SHALL display a searchable, filterable catalog organized
by categories (3D models, materials, VFX, audio, logic graph templates, game templates, plugins,
tools) with tags, star ratings, reviews, download counts, and engine-version compatibility badges.
Asset previews SHALL include interactive 3D model viewers with rotation and zoom, material previews
on standard meshes, audio playback, and screenshot galleries. Search SHALL support text, tag
filtering, and semantic queries.
- **Derived from:** [F-15.17.1](../../features/tools-editor/asset-store.md)
- **Rationale:** An integrated store reduces context-switching between editor and external web
  browsers, enables rich in-engine asset previews that external storefronts cannot provide, and
  increases asset discoverability for all engine users.
- **Verification:** Open the store browser from the editor menu and the launcher; confirm the
  catalog loads with categories and metadata; search by text, tags, and semantic query and confirm
  relevant results; preview a 3D model, material, and audio asset and confirm each preview type
  functions correctly; confirm compatibility badges reflect the current engine version.

## R-15.17.2 One-Click Asset Import and Project Integration
The engine **SHALL** import purchased or free marketplace assets into the current project with a
single action. The import pipeline SHALL validate engine-version compatibility and required plugin
dependencies, automatically download transitive dependencies, install assets to a designated
marketplace directory, and surface the imported assets in the asset browser immediately. Version
conflicts between asset requirements and the project's installed plugins SHALL be flagged with
specific upgrade guidance. Asset licenses SHALL be tracked per-project.
- **Derived from:** [F-15.17.2](../../features/tools-editor/asset-store.md)
- **Rationale:** One-click import with automatic dependency resolution eliminates manual asset
  wrangling, prevents dependency version mismatches, and keeps marketplace content isolated from
  hand-authored project assets.
- **Verification:** Purchase a paid asset and a free asset; confirm both import into the project
  without additional steps; confirm transitive dependencies are downloaded automatically; confirm
  the asset appears in the asset browser; import an asset requiring a newer plugin version and
  confirm a version-conflict warning with upgrade guidance appears; confirm license metadata is
  recorded in the project file.

## R-15.17.3 Asset Ratings, Reviews, and Curation
The engine **SHALL** allow authenticated users to rate assets from 1 to 5 stars and submit text
reviews. Reviews SHALL be moderated for spam and abuse. The store SHALL support editorial curation
(featured assets, staff picks, themed collections) and community-curated collections. Each asset
listing SHALL display: average rating, review count, download count, last-updated date, engine
version compatibility range, and verified-publisher badge status.
- **Derived from:** [F-15.17.3](../../features/tools-editor/asset-store.md)
- **Rationale:** Ratings, reviews, and curation provide social proof and editorial guidance that
  help users identify high-quality assets and avoid low-quality or abandoned content.
- **Verification:** Submit a rating and review for an asset; confirm both appear on the listing;
  submit a review containing flagged content and confirm moderation intercepts it; confirm
  featured collections appear on the store home page; create a community collection and confirm
  other users can view it; confirm all metadata fields display on the asset detail page.

## R-15.17.4 Publisher Account and Dashboard
The engine **SHALL** provide a publisher registration flow with identity verification and a
publisher dashboard displaying: published assets, revenue analytics (sales, refunds, royalties),
download statistics, user reviews with response capability, per-engine-version compatibility test
results, and payout history. Publishers SHALL set prices in multiple currencies, configure regional
pricing, create time-limited sales and discount codes, and bundle assets into packs with aggregate
pricing.
- **Derived from:** [F-15.17.4](../../features/tools-editor/asset-store.md)
- **Rationale:** A self-service publisher dashboard with analytics, pricing tools, and review
  management enables asset creators to operate independently and optimize revenue without
  marketplace administrator intervention.
- **Verification:** Register a publisher account with identity verification; publish an asset and
  confirm it appears in the catalog; confirm revenue analytics display sales, refunds, and
  royalties; set regional pricing and confirm the correct price displays per region; create a
  discount code and confirm it applies at checkout; respond to a user review and confirm the
  response is visible on the listing.

## R-15.17.5 Automated Compatibility Testing
The marketplace **SHALL** automatically test all published assets against each new engine version
release. The testing pipeline SHALL import each asset into a test project, verify it loads without
errors, execute included automated tests, and generate a compatibility report. Assets that pass
SHALL receive a compatibility badge for the new engine version. Publishers SHALL receive
notifications for failing assets with specific error details. Testing SHALL run on CI
infrastructure using the shared build cache (F-15.11.1).
- **Derived from:** [F-15.17.5](../../features/tools-editor/asset-store.md)
- **Rationale:** Automated compatibility testing surfaces breakage early, gives publishers
  actionable remediation details, and gives buyers confidence that badged assets work on their
  engine version.
- **Verification:** Publish an asset on engine version N; release engine version N+1 and confirm
  the compatibility pipeline runs; confirm a passing asset receives the N+1 compatibility badge;
  introduce a breaking change and confirm the publisher is notified with the specific error;
  confirm test results appear on the publisher dashboard.

## R-15.17.6 Revenue Sharing and Payout
The marketplace **SHALL** apply a configurable commission rate (default 12%) on paid asset sales
and process monthly publisher payouts via bank transfer, PayPal, or platform credits. Revenue
reports SHALL detail per-asset sales, regional breakdown, and commission deductions. Refund
processing SHALL automatically reverse publisher revenue for purchases refunded within 14 days.
Tax documentation (W-9, W-8BEN) SHALL be collected from publishers before first payout. Free asset
publication SHALL incur no fees.
- **Derived from:** [F-15.17.6](../../features/tools-editor/asset-store.md)
- **Rationale:** Transparent revenue sharing with competitive commission rates attracts
  high-quality publishers, and automated refund reversal and tax compliance reduce administrative
  overhead.
- **Verification:** Sell an asset and confirm the publisher revenue report shows the sale minus
  the 12% commission; process a refund within 14 days and confirm the revenue is reversed;
  process a refund after 14 days and confirm the revenue is not reversed; confirm payout is
  blocked until tax documentation is submitted; confirm free asset publishing incurs no charges.

## R-15.17.7 Asset Type Support
The marketplace **SHALL** support all asset types producible by the engine: 3D meshes (with LODs,
materials, collision), 2D sprites and sprite sheets, materials and material functions, VFX effect
graphs, audio clips and music tracks, logic graph templates, UI widget templates, animation clips
and state machines, terrain brushes and biome presets, procedural generation graphs, full project
templates, and Rust plugins (per-platform compiled binaries). Each asset type SHALL have
type-appropriate preview rendering and metadata fields in the store listing.
- **Derived from:** [F-15.17.7](../../features/tools-editor/asset-store.md)
- **Rationale:** Supporting the full range of engine-producible asset types makes the marketplace
  the single destination for all reusable content, maximizing ecosystem value.
- **Verification:** Upload and publish one asset of each supported type; confirm each appears in
  the catalog with type-appropriate preview rendering; confirm a Rust plugin has per-platform
  binaries available; confirm metadata fields are correct for each asset type.

## R-15.17.8 License Management and DRM
Assets **SHALL** carry license metadata specifying: usage rights (personal, commercial,
non-transferable), redistribution rights, attribution requirements, and seat count (per-user or
per-project). The engine SHALL validate license terms at import time and display them in the asset
browser. The engine SHALL NOT enforce runtime DRM -- imported assets SHALL function offline without
license checks. License compliance beyond the import check SHALL be contractual.
- **Derived from:** [F-15.17.8](../../features/tools-editor/asset-store.md)
- **Rationale:** Import-time license display and validation informs users of their obligations
  without degrading runtime performance or requiring online connectivity, while contractual
  enforcement respects the trust-based creator economy.
- **Verification:** Import an asset and confirm license terms are displayed; confirm the asset
  works offline after import with no network connectivity; confirm license metadata is recorded in
  the project's asset manifest; confirm a personal-license asset displays its restriction in the
  asset browser detail panel.

## Non-Functional Requirements

### R-15.17.NF1 Store Search Latency and Download Resumption
Store catalog search results **SHALL** return within 500 ms for 95th-percentile queries against a
catalog of up to 100,000 assets. Asset downloads **SHALL** support resumption after network
interruption -- a partially downloaded asset SHALL resume from the last completed chunk rather than
restarting. Downloaded chunks SHALL be verified via BLAKE3 content hashes before assembly.
- **Derived from:** F-15.17.1, F-15.17.2
- **Rationale:** Sub-500 ms search keeps the browsing experience responsive, and resumable
  downloads prevent wasted bandwidth and user frustration on unreliable connections.
- **Verification:** Execute 1,000 search queries against a 100,000-asset catalog and confirm the
  95th-percentile latency is under 500 ms. Start a large asset download, interrupt the network
  connection, restore connectivity, and confirm the download resumes from the interruption point
  without re-downloading completed chunks. Corrupt a downloaded chunk and confirm the hash check
  rejects it and re-downloads the chunk.

### R-15.17.NF2 Marketplace Availability
The marketplace API and download infrastructure **SHALL** maintain 99.9% uptime measured monthly
(less than 43.8 minutes of unplanned downtime per month). Planned maintenance windows SHALL be
communicated 72 hours in advance. The editor SHALL gracefully degrade when the store is
unreachable -- previously downloaded assets SHALL remain functional, and the store browser SHALL
display a connectivity warning rather than crashing.
- **Derived from:** F-15.17.1 through F-15.17.8 (all marketplace features)
- **Rationale:** Asset store downtime blocks purchasing and discovery, and a 99.9% SLA aligns
  with industry expectations for developer-facing cloud services.
- **Verification:** Monitor uptime over a 30-day period and confirm it meets 99.9%. Simulate a
  store outage and confirm the editor continues to function normally with a connectivity warning
  in the store browser; confirm previously imported assets remain usable; confirm no editor crash
  or hang occurs.
