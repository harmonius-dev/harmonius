# R-15.17 -- Asset Marketplace Requirements

## Requirements

1. **R-15.17.1** — The engine **SHALL** provide an in-editor marketplace browser with search,
   filters, 3D previews, compatibility badges, and engine version filtering.
   - **Rationale:** In-editor browsing reduces context switching when acquiring assets.
   - **Verification:** Search for a 3D model, preview it, and verify engine version compatibility is
     displayed.

2. **R-15.17.2** — The engine **SHALL** support one-click asset import with compatibility
   validation, dependency resolution, and version conflict warnings.
   - **Rationale:** Automated import ensures assets integrate without manual configuration.
   - **Verification:** Import an asset with a dependency, verify the dependency auto-downloads.

3. **R-15.17.3** — The engine **SHALL** support ratings, reviews, curated collections, and review
   moderation for marketplace assets.
   - **Rationale:** Quality signals help users choose assets and maintain marketplace trust.
   - **Verification:** Rate an asset, write a review, and verify both appear on the listing.

4. **R-15.17.4** — The engine **SHALL** provide a publisher dashboard with revenue analytics,
   regional pricing, sales, bundles, and compatibility test results.
   - **Rationale:** Publishers need business tools to manage their asset portfolio.
   - **Verification:** Publish an asset, configure regional pricing, and verify the dashboard shows
     correct analytics.

5. **R-15.17.5** — The engine **SHALL** automatically test published assets against new engine
   versions on CI infrastructure, displaying compatibility badges.
   - **Rationale:** Automated testing keeps the marketplace ecosystem healthy across engine updates.
   - **Verification:** Release a new engine version and verify compatibility tests run for all
     published assets.

6. **R-15.17.6** — The engine **SHALL** support configurable revenue sharing with monthly payouts,
   per-asset reports, and zero-cost free asset publishing.
   - **Rationale:** Fair economics sustain the publisher ecosystem.
   - **Verification:** Simulate a purchase and verify the revenue split matches the configured
     commission.

7. **R-15.17.7** — The engine **SHALL** support all engine asset types in the marketplace including
   plugins with per-platform compiled binaries.
   - **Rationale:** Complete asset type coverage maximizes marketplace utility.
   - **Verification:** Publish a Rust plugin and verify per-platform binaries are available.

8. **R-15.17.8** — The engine **SHALL** attach license metadata at import time with no runtime DRM.
   - **Rationale:** License tracking at import is sufficient; runtime DRM degrades player
     experience.
   - **Verification:** Import an asset, disconnect from network, and verify the asset still loads.

9. **R-15.17.9** — The engine **SHALL** provide an open-source asset browser backed by a community
   Git repository with license filtering and PR-based publishing.
   - **Rationale:** Open-source assets lower the barrier to entry for new developers.
   - **Verification:** Submit an asset via PR, verify CI validates it, and confirm it appears after
     merge.

10. **R-15.17.10** — The engine **SHALL** provide a store plugin API for third-party marketplace
    integration and per-project license compliance reports.
    - **Rationale:** Multi-store support and license tracking serve diverse team needs.
    - **Verification:** Register a custom store plugin and verify it appears as a tab in the asset
      browser.

11. **R-15.17.11** — The engine **SHALL** communicate with marketplace servers via platform-native
    QUIC with rkyv-serialized payloads, not HTTP/HTTPS.
    - **Rationale:** QUIC with binary payloads reduces latency and overhead compared to REST/JSON
      over HTTP.
    - **Verification:** Capture marketplace traffic and verify all payloads use QUIC with rkyv
      encoding and no HTTP fallback.

12. **R-15.17.12** — The engine **SHALL** support self-hosted private marketplace instances using
    the same OSS codebase with OAuth or API token authentication.
    - **Rationale:** Studios need private asset sharing without exposing proprietary content on the
      public marketplace.
    - **Verification:** Deploy a private instance, authenticate with OAuth, publish an asset, and
      verify it is not visible on the public marketplace.

13. **R-15.17.13** — The engine **SHALL** check all installed marketplace packages for available
    updates and provide one-click bulk update.
    - **Rationale:** Outdated packages cause compatibility issues; bulk update reduces maintenance
      burden.
    - **Verification:** Install a package, publish a newer version, and verify the editor shows an
      update notification with a working bulk-update action.

14. **R-15.17.14** — The engine **SHALL** generate a per-project license compliance report for all
    installed marketplace packages and warn on incompatible license combinations.
    - **Rationale:** License compliance is a legal obligation that must be tracked automatically.
    - **Verification:** Install a GPL-licensed package in a proprietary project and verify the
      compliance report flags the incompatibility.
