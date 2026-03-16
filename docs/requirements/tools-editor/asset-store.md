# R-15.17 -- Asset Marketplace Requirements

## Browsing and Discovery

### R-15.17.1 Integrated Asset Store Browser

The editor **SHALL** provide an in-editor marketplace
browser with searchable, filterable catalog, interactive
3D model previews, material previews, audio playback,
engine-version compatibility badges, ratings, reviews,
and download counts.

- **Derived from:**
  [F-15.17.1](../../features/tools-editor/asset-store.md)
- **Rationale:** In-editor asset discovery eliminates
  context switching and accelerates content acquisition.
- **Verification:** Unit test: search by keyword and verify
  results match query terms.

### R-15.17.2 One-Click Asset Import

The editor **SHALL** support one-click import of
marketplace assets with automatic dependency resolution,
engine-version compatibility validation at import, and
per-project license tracking.

- **Derived from:**
  [F-15.17.2](../../features/tools-editor/asset-store.md)
- **Rationale:** Frictionless import with dependency
  resolution ensures purchased assets work immediately.
- **Verification:** Unit test: import an asset with
  transitive dependencies and verify all are downloaded
  automatically.

## Ratings and Curation

### R-15.17.3 Asset Ratings, Reviews, and Curation

The marketplace **SHALL** support 1-5 star ratings with
text reviews, editorial curation with featured assets and
staff picks, community-curated collections, and review
moderation for spam and abuse.

- **Derived from:**
  [F-15.17.3](../../features/tools-editor/asset-store.md)
- **Rationale:** Quality signals help buyers evaluate
  assets before purchase.
- **Verification:** Unit test: flag content for moderation
  and verify it is intercepted.

## Publisher Tools

### R-15.17.4 Publisher Account and Dashboard

The marketplace **SHALL** provide publisher accounts with
identity verification, revenue analytics, multi-currency
pricing with regional pricing, review response capability,
and time-limited sales and discount codes.

- **Derived from:**
  [F-15.17.4](../../features/tools-editor/asset-store.md)
- **Rationale:** Publishers need business tools to manage
  and promote their assets.
- **Verification:** Unit test: apply a discount code at
  checkout and verify the discount is applied.

### R-15.17.5 Automated Compatibility Testing

The marketplace **SHALL** automatically test all published
assets against new engine versions, awarding compatibility
badges to passing assets and notifying publishers of
failures with specific error details.

- **Derived from:**
  [F-15.17.5](../../features/tools-editor/asset-store.md)
- **Rationale:** Automated testing keeps compatibility
  badges current without manual publisher effort.
- **Verification:** Unit test: verify passing assets
  receive the new version badge.

### R-15.17.6 Revenue Sharing and Payout

The marketplace **SHALL** take a configurable commission
(default 12%) with monthly payouts via bank transfer or
PayPal, per-asset revenue reports with regional breakdown,
and automatic refund revenue reversal within 14 days.

- **Derived from:**
  [F-15.17.6](../../features/tools-editor/asset-store.md)
- **Rationale:** Fair, transparent revenue sharing
  incentivizes publisher participation.
- **Verification:** Unit test: process a refund within 14
  days and verify publisher revenue is reversed.

## Asset Types and Licensing

### R-15.17.7 Asset Type Support

The marketplace **SHALL** support all engine-producible
asset types (3D meshes, materials, VFX, audio, logic graph
templates, full project templates, Rust plugins) with
per-platform compiled binaries for plugins.

- **Derived from:**
  [F-15.17.7](../../features/tools-editor/asset-store.md)
- **Rationale:** Complete asset type coverage maximizes
  marketplace utility.
- **Verification:** Unit test: verify each supported asset
  type renders a correct preview in the store listing.

### R-15.17.8 License Management

The marketplace **SHALL** support per-asset usage rights
(personal, commercial), license terms displayed at import
time, and no runtime DRM on imported assets.

- **Derived from:**
  [F-15.17.8](../../features/tools-editor/asset-store.md)
- **Rationale:** Clear licensing and DRM-free imports
  ensure assets work offline without restrictions.
- **Verification:** Unit test: import an asset and verify
  it functions offline without network connectivity.

---

## User Stories

## US-15.17.1 Integrated Asset Store Browser

### US-15.17.1.1
As a **designer (P-5)**, I want an in-editor marketplace browser so that I can discover assets
without leaving the editor.

### US-15.17.1.2
As a **designer (P-5)**, I want searchable, filterable catalog with categories so that I can find
assets by type, tag, or text query.

### US-15.17.1.3
As a **artist (P-8)**, I want interactive 3D model previews with rotate and zoom so that I can
evaluate mesh quality before purchasing.

### US-15.17.1.4
As a **artist (P-8)**, I want material previews on standard meshes so that I can evaluate materials
in a familiar rendering context.

### US-15.17.1.5
As a **artist (P-8)**, I want audio playback for audio assets so that I can listen to sound effects
before downloading.

### US-15.17.1.6
As a **developer (P-15)**, I want engine-version compatibility badges so that I only see assets
compatible with my current version.

### US-15.17.1.7
As a **creative director (P-2)**, I want ratings, reviews, and download counts so that I can assess
asset quality from community feedback.

### US-15.17.1.8
As an **engine tester (P-27)**, I want to verify search results match query terms so that store
search accuracy is regression-tested.

---

## US-15.17.2 One-Click Asset Import and Project Integration

### US-15.17.2.1
As a **designer (P-5)**, I want one-click import of marketplace assets so that purchased content is
added to my project instantly.

### US-15.17.2.2
As a **designer (P-5)**, I want automatic dependency resolution during import so that required
companion assets are pulled automatically.

### US-15.17.2.3
As a **developer (P-15)**, I want engine-version compatibility validation at import so that
incompatible assets are flagged with upgrade guidance.

### US-15.17.2.4
As a **developer (P-15)**, I want asset licenses tracked per-project so that license compliance is
managed automatically.

### US-15.17.2.5
As an **engine tester (P-27)**, I want to verify transitive dependencies are downloaded
automatically so that dependency resolution is regression-tested.

---

## US-15.17.3 Asset Ratings, Reviews, and Curation

### US-15.17.3.1
As a **designer (P-5)**, I want to rate assets 1-5 stars with text reviews so that I can share my
evaluation with the community.

### US-15.17.3.2
As a **creative director (P-2)**, I want editorial curation with featured assets and staff picks so
that high-quality content is prominently highlighted.

### US-15.17.3.3
As a **designer (P-5)**, I want community-curated asset collections so that I can browse themed
bundles curated by other developers.

### US-15.17.3.4
As a **server admin (P-22)**, I want review moderation for spam and abuse so that store reviews
maintain quality and trustworthiness.

### US-15.17.3.5
As an **engine tester (P-27)**, I want to verify moderation intercepts flagged content so that
review moderation is regression-tested.

---

## US-15.17.4 Publisher Account and Dashboard

### US-15.17.4.1
As a **modder (P-24)**, I want to register a publisher account with identity verification so that I
can sell assets on the marketplace.

### US-15.17.4.2
As a **modder (P-24)**, I want revenue analytics showing sales, refunds, and royalties so that I can
track my asset business performance.

### US-15.17.4.3
As a **modder (P-24)**, I want to set prices in multiple currencies with regional pricing so that my
assets are priced appropriately per market.

### US-15.17.4.4
As a **modder (P-24)**, I want to respond to user reviews so that I can address feedback and provide
support.

### US-15.17.4.5
As a **modder (P-24)**, I want to create time-limited sales and discount codes so that I can promote
my assets with marketing campaigns.

### US-15.17.4.6
As an **engine tester (P-27)**, I want to verify discount codes apply at checkout so that pricing
functionality is regression-tested.

---

## US-15.17.5 Automated Compatibility Testing

### US-15.17.5.1
As a **DevOps (P-16)**, I want automatic testing of all assets against new engine versions so that
compatibility badges are updated without manual testing.

### US-15.17.5.2
As a **modder (P-24)**, I want notifications when my assets fail compatibility testing so that I can
fix issues for the new engine version.

### US-15.17.5.3
As a **modder (P-24)**, I want compatibility badges displayed on my asset listing so that buyers
know which engine versions my asset supports.

### US-15.17.5.4
As an **engine tester (P-27)**, I want to verify passing assets receive the new version badge so
that compatibility badge assignment is regression-tested.

---

## US-15.17.6 Revenue Sharing and Payout

### US-15.17.6.1
As a **modder (P-24)**, I want monthly payouts via bank transfer or PayPal so that I receive revenue
from my asset sales.

### US-15.17.6.2
As a **modder (P-24)**, I want per-asset revenue reports with regional breakdown so that I
understand my sales distribution.

### US-15.17.6.3
As a **server admin (P-22)**, I want configurable commission rate (default 12%) so that marketplace
economics are adjustable.

### US-15.17.6.4
As a **server admin (P-22)**, I want automatic refund revenue reversal within 14 days so that
refunded purchases are handled correctly.

### US-15.17.6.5
As an **engine tester (P-27)**, I want to verify refund within 14 days reverses publisher revenue so
that refund processing is regression-tested.

---

## US-15.17.7 Asset Type Support

### US-15.17.7.1
As a **modder (P-24)**, I want to publish 3D meshes, materials, VFX, audio, and logic graph
templates so that all engine-producible asset types are marketplace-ready.

### US-15.17.7.2
As a **modder (P-24)**, I want to publish full project templates and Rust plugins so that complete
game starters and engine extensions are distributable.

### US-15.17.7.3
As a **developer (P-15)**, I want per-platform compiled binaries for Rust plugins so that plugin
assets work on all target platforms.

### US-15.17.7.4
As an **engine tester (P-27)**, I want to verify each supported asset type renders correct preview
in the store listing so that asset type preview is regression-tested.

---

## US-15.17.8 License Management and DRM

### US-15.17.8.1
As a **modder (P-24)**, I want to specify usage rights (personal, commercial) per asset so that I
control how my assets are used.

### US-15.17.8.2
As a **developer (P-15)**, I want license terms displayed at import time so that I am informed of
obligations before using an asset.

### US-15.17.8.3
As a **developer (P-15)**, I want no runtime DRM on imported assets so that assets work offline
without license checks.

### US-15.17.8.4
As an **engine tester (P-27)**, I want to verify imported assets function offline without network
connectivity so that DRM-free behavior is regression-tested.
