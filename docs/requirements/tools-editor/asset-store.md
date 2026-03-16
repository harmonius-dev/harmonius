# R-15.17 -- Asset Marketplace Requirements

## Browsing and Discovery

### R-15.17.1 Integrated Asset Store Browser

The editor **SHALL** provide an in-editor marketplace browser with searchable, filterable catalog,
interactive 3D model previews, material previews, audio playback, engine-version compatibility
badges, ratings, reviews, and download counts.

- **Derived from:**
  [F-15.17.1](../../features/tools-editor/asset-store.md)
- **Rationale:** In-editor asset discovery eliminates context switching and accelerates content
  acquisition.
- **Verification:** Unit test: search by keyword and verify results match query terms.

### R-15.17.2 One-Click Asset Import

The editor **SHALL** support one-click import of marketplace assets with automatic dependency
resolution, engine-version compatibility validation at import, and per-project license tracking.

- **Derived from:**
  [F-15.17.2](../../features/tools-editor/asset-store.md)
- **Rationale:** Frictionless import with dependency resolution ensures purchased assets work
  immediately.
- **Verification:** Unit test: import an asset with transitive dependencies and verify all are
  downloaded automatically.

## Ratings and Curation

### R-15.17.3 Asset Ratings, Reviews, and Curation

The marketplace **SHALL** support 1-5 star ratings with text reviews, editorial curation with
featured assets and staff picks, community-curated collections, and review moderation for spam and
abuse.

- **Derived from:**
  [F-15.17.3](../../features/tools-editor/asset-store.md)
- **Rationale:** Quality signals help buyers evaluate assets before purchase.
- **Verification:** Unit test: flag content for moderation and verify it is intercepted.

## Publisher Tools

### R-15.17.4 Publisher Account and Dashboard

The marketplace **SHALL** provide publisher accounts with identity verification, revenue analytics,
multi-currency pricing with regional pricing, review response capability, and time-limited sales and
discount codes.

- **Derived from:**
  [F-15.17.4](../../features/tools-editor/asset-store.md)
- **Rationale:** Publishers need business tools to manage and promote their assets.
- **Verification:** Unit test: apply a discount code at checkout and verify the discount is applied.

### R-15.17.5 Automated Compatibility Testing

The marketplace **SHALL** automatically test all published assets against new engine versions,
awarding compatibility badges to passing assets and notifying publishers of failures with specific
error details.

- **Derived from:**
  [F-15.17.5](../../features/tools-editor/asset-store.md)
- **Rationale:** Automated testing keeps compatibility badges current without manual publisher
  effort.
- **Verification:** Unit test: verify passing assets receive the new version badge.

### R-15.17.6 Revenue Sharing and Payout

The marketplace **SHALL** take a configurable commission (default 12%) with monthly payouts via bank
transfer or PayPal, per-asset revenue reports with regional breakdown, and automatic refund revenue
reversal within 14 days.

- **Derived from:**
  [F-15.17.6](../../features/tools-editor/asset-store.md)
- **Rationale:** Fair, transparent revenue sharing incentivizes publisher participation.
- **Verification:** Unit test: process a refund within 14 days and verify publisher revenue is
  reversed.

## Asset Types and Licensing

### R-15.17.7 Asset Type Support

The marketplace **SHALL** support all engine-producible asset types (3D meshes, materials, VFX,
audio, logic graph templates, full project templates, Rust plugins) with per-platform compiled
binaries for plugins.

- **Derived from:**
  [F-15.17.7](../../features/tools-editor/asset-store.md)
- **Rationale:** Complete asset type coverage maximizes marketplace utility.
- **Verification:** Unit test: verify each supported asset type renders a correct preview in the
  store listing.

### R-15.17.8 License Management

The marketplace **SHALL** support per-asset usage rights (personal, commercial), license terms
displayed at import time, and no runtime DRM on imported assets.

- **Derived from:**
  [F-15.17.8](../../features/tools-editor/asset-store.md)
- **Rationale:** Clear licensing and DRM-free imports ensure assets work offline without
  restrictions.
- **Verification:** Unit test: import an asset and verify it functions offline without network
  connectivity.

---

## User Story Traceability

User stories for this domain are maintained in
[user-stories/tools-editor/asset-store.md](../../user-stories/tools-editor/asset-store.md).
Requirements in this document are derived from those
user stories.
