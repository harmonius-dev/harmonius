# Asset Store / Marketplace Test Cases

Companion test cases for [asset-store.md](asset-store.md).

## Unit Tests

### TC-15.17.1.1 Search Category Filter

| # | Requirement |
|---|-------------|
| 1 | R-15.17.1   |
| 2 | R-15.17.1   |

1. **#1** — `SearchQuery { category: Some(Mesh3d), .. }` against catalog with mixed types
   - **Expected:** Results contain only `Mesh3d` assets
2. **#2** — `SearchQuery { category: Some(VfxGraph), .. }` against catalog with no VFX assets
   - **Expected:** Results are empty, `total_count == 0`

### TC-15.17.1.2 Search Tag Filter

| # | Requirement |
|---|-------------|
| 1 | R-15.17.1   |
| 2 | R-15.17.1   |

1. **#1** — `SearchQuery { tags: ["medieval", "stone"], .. }`
   - **Expected:** Results contain only assets tagged with both "medieval" AND "stone"
2. **#2** — `SearchQuery { tags: ["nonexistent"], .. }`
   - **Expected:** Results are empty

### TC-15.17.1.3 Search Engine Version Filter

| # | Requirement |
|---|-------------|
| 1 | R-15.17.1   |
| 2 | R-15.17.1   |
| 3 | R-15.17.1   |

1. **#1** — `SearchQuery { engine_version: Some(SemVer(2,0,0)), .. }`
   - **Expected:** Excludes assets with `min_engine_version > 2.0.0`
2. **#2** — Asset with `min_engine_version: 1.5.0`, query version `2.0.0`
   - **Expected:** Asset included in results
3. **#3** — Asset with `min_engine_version: 3.0.0`, query version `2.0.0`
   - **Expected:** Asset excluded from results

### TC-15.17.1.4 Search Sort Orders

| # | Requirement |
|---|-------------|
| 1 | R-15.17.1   |
| 2 | R-15.17.1   |
| 3 | R-15.17.1   |

1. **#1** — `sort: Rating`, 3 assets rated 3.0, 5.0, 4.0
   - **Expected:** Results ordered [5.0, 4.0, 3.0]
2. **#2** — `sort: Newest`, 3 assets published at t1 < t2 < t3
   - **Expected:** Results ordered [t3, t2, t1]
3. **#3** — `sort: PriceLowToHigh`, prices 500, 100, 300
   - **Expected:** Results ordered [100, 300, 500]

### TC-15.17.2.1 Import Resolves Dependencies

| # | Requirement |
|---|-------------|
| 1 | R-15.17.2   |
| 2 | R-15.17.2   |

1. **#1** — Asset A depends on B, B depends on C
   - **Expected:** `resolve_dependencies(A)` returns `[C, B, A]` (topological)
2. **#2** — Asset with no dependencies
   - **Expected:** `resolve_dependencies(asset)` returns `[asset]`

### TC-15.17.2.2 Import Detects Version Conflict

| # | Requirement |
|---|-------------|
| 1 | R-15.17.2   |

1. **#1** — Asset requires plugin v2.0, installed v1.5
   - **Expected:** `ImportResult::VersionConflict { required: 2.0, installed: 1.5, .. }`

### TC-15.17.2.3 Import Detects Missing Plugin

| # | Requirement |
|---|-------------|
| 1 | R-15.17.2   |

1. **#1** — Asset requires plugin "physics_ext", not installed
   - **Expected:** `ImportResult::MissingPlugin { plugin_id: "physics_ext", .. }`

### TC-15.17.3.1 Review Rating Range

| # | Requirement |
|---|-------------|
| 1 | R-15.17.3   |
| 2 | R-15.17.3   |
| 3 | R-15.17.3   |

1. **#1** — `submit_review(asset, rating: 5, "Great")`
   - **Expected:** Returns `Ok(review_id)`
2. **#2** — `submit_review(asset, rating: 0, "Bad")`
   - **Expected:** Returns `Err(...)` (rating < 1)
3. **#3** — `submit_review(asset, rating: 6, "Amazing")`
   - **Expected:** Returns `Err(...)` (rating > 5)

### TC-15.17.3.2 Review Moderation Pending

| # | Requirement |
|---|-------------|
| 1 | R-15.17.3   |

1. **#1** — Submit new review
   - **Expected:** `review.moderation == ModerationStatus::Pending`

### TC-15.17.4.1 Publisher Registration

| # | Requirement |
|---|-------------|
| 1 | R-15.17.4   |

1. **#1** — `register(PublisherRegistration { display_name: "Studio", .. })`
   - **Expected:** Returns `Ok(publisher_id)`, publisher status `Unverified`

### TC-15.17.4.2 Regional Pricing

| # | Requirement |
|---|-------------|
| 1 | R-15.17.4   |

1. **#1** — Base price 999 cents USD, regional override BR: 499 BRL
   - **Expected:** Brazil query returns 499 BRL, US query returns 999 USD

### TC-15.17.4.3 Discount Code Apply

| # | Requirement |
|---|-------------|
| 1 | R-15.17.4   |

1. **#1** — Asset price 1000 cents, discount code 20% off
   - **Expected:** Checkout price = 800 cents

### TC-15.17.4.4 Discount Code Expired

| # | Requirement |
|---|-------------|
| 1 | R-15.17.4   |

1. **#1** — Discount code with `expires_at` in the past
   - **Expected:** Code rejected, full price charged

### TC-15.17.5.1 Compat Badge Awarded

| # | Requirement |
|---|-------------|
| 1 | R-15.17.5   |

1. **#1** — Asset passes compat test for engine v2.3.0
   - **Expected:** `compat_badges` contains
     `CompatBadge { engine_version: 2.3.0, test_passed: true }`

### TC-15.17.5.2 Compat Badge Revoked

| # | Requirement |
|---|-------------|
| 1 | R-15.17.5   |

1. **#1** — Asset fails compat test for engine v2.4.0
   - **Expected:** `compat_badges` for v2.4.0 has `test_passed: false`

### TC-15.17.6.1 Commission Calculation

| # | Requirement |
|---|-------------|
| 1 | R-15.17.6   |

1. **#1** — Sale price 1000 cents, commission rate 0.12
   - **Expected:** Commission = 120 cents, net = 880 cents

### TC-15.17.6.2 Refund Within Window

| # | Requirement |
|---|-------------|
| 1 | R-15.17.6   |
| 2 | R-15.17.6   |

1. **#1** — Purchase 10 days ago, request refund
   - **Expected:** `RefundResult::Refunded { refund_id }`
2. **#2** — Publisher revenue after refund
   - **Expected:** Revenue reversed by sale amount minus commission

### TC-15.17.6.3 Refund Outside Window

| # | Requirement |
|---|-------------|
| 1 | R-15.17.6   |

1. **#1** — Purchase 15 days ago, request refund
   - **Expected:** `RefundResult::WindowExpired`

### TC-15.17.7.1 All Asset Types Accepted

| # | Requirement |
|---|-------------|
| 1 | R-15.17.7   |
| 2 | R-15.17.7   |
| 3 | R-15.17.7   |

1. **#1** — Upload asset with category `Mesh3d`
   - **Expected:** Upload succeeds
2. **#2** — Upload asset with category `RustPlugin`
   - **Expected:** Upload succeeds
3. **#3** — Upload asset for each of 15 `AssetCategory` variants
   - **Expected:** All 15 uploads succeed

### TC-15.17.8.1 License Validation Commercial

| # | Requirement |
|---|-------------|
| 1 | R-15.17.8   |
| 2 | R-15.17.8   |

1. **#1** — `validate_license(Personal, project_commercial: true)`
   - **Expected:** `LicenseValidation::CommercialRestriction`
2. **#2** — `validate_license(Commercial, project_commercial: true)`
   - **Expected:** `LicenseValidation::Valid`

### TC-15.17.8.2 License No Runtime DRM

| # | Requirement |
|---|-------------|
| 1 | R-15.17.8   |

1. **#1** — Import asset, disconnect network, use asset
   - **Expected:** Asset functions without network

## Integration Tests

### TC-15.17.2.I1 End to End Purchase Import

| # | Requirement |
|---|-------------|
| 1 | R-15.17.2   |

1. **#1** — Purchase paid asset, download, import
   - **Expected:** Asset visible in asset browser, files on disk

### TC-15.17.2.I2 Dependency Chain Import

| # | Requirement |
|---|-------------|
| 1 | R-15.17.2   |

1. **#1** — Import asset with 3-level dependency chain (A->B->C)
   - **Expected:** All three assets imported, C first, then B, then A

### TC-15.17.2.I3 Free Asset No Payment

| # | Requirement |
|---|-------------|
| 1 | R-15.17.2   |

1. **#1** — Import free asset (price = 0)
   - **Expected:** No Stripe payment flow, asset downloaded and imported

### TC-15.17.1.I1 Search Semantic Query

| # | Requirement |
|---|-------------|
| 1 | R-15.17.1   |

1. **#1** — Search "dark fantasy castle" against catalog with relevant assets
   - **Expected:** Top results include castle/medieval assets

### TC-15.17.4.I1 Publisher Upload to Listing

| # | Requirement |
|---|-------------|
| 1 | R-15.17.4   |

1. **#1** — Upload package, pass compat test, approve review
   - **Expected:** Asset visible on marketplace with `Published` status

### TC-15.17.5.I1 Compat Test Pipeline

| # | Requirement |
|---|-------------|
| 1 | R-15.17.5   |

1. **#1** — Upload asset, trigger compat test via SQS/CodeBuild
   - **Expected:** Badge assigned, publisher notified

### TC-15.17.6.I1 Payout Calculation

| # | Requirement |
|---|-------------|
| 1 | R-15.17.6   |

1. **#1** — 5 purchases at 1000 cents + 1 refund
   - **Expected:** Net payout = (5 * 880) - 880 = 3520 cents

### TC-15.17.6.I2 Stripe Payment Flow

| # | Requirement |
|---|-------------|
| 1 | R-15.17.6   |

1. **#1** — Purchase with Stripe test card `4242...`
   - **Expected:** `PurchaseResponse` with valid `download_url`

### TC-15.17.3.I1 Review Moderation Flow

| # | Requirement |
|---|-------------|
| 1 | R-15.17.3   |
| 2 | R-15.17.3   |

1. **#1** — Submit review, moderate to Approved
   - **Expected:** Review visible in `list_reviews`
2. **#2** — Submit review, moderate to Rejected
   - **Expected:** Review not visible in `list_reviews`

### TC-15.17.3.I2 Collection Browse

| # | Requirement |
|---|-------------|
| 1 | R-15.17.3   |

1. **#1** — Create collection with 5 assets, browse from editor
   - **Expected:** Collection returned with all 5 asset IDs

### TC-15.17.7.I1 Rust Plugin Per Platform

| # | Requirement |
|---|-------------|
| 1 | R-15.17.7   |

1. **#1** — Upload Rust plugin, request download for macOS
   - **Expected:** Receives macOS-compiled binary

### TC-15.17.8.I1 Offline After Import

| # | Requirement |
|---|-------------|
| 1 | R-15.17.8   |

1. **#1** — Import asset, disconnect network, open project
   - **Expected:** Asset loads and renders correctly

## Benchmarks

### TC-15.17.1.B1 Catalog Search Latency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Text search against 10,000 asset catalog | p95 latency | < 200 ms | R-15.17.1 |

### TC-15.17.2.B1 Asset Download

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Download 100 MB asset on 100 Mbps connection | Duration | < 30 s | R-15.17.2 |

### TC-15.17.2.B2 Import Pipeline Single

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Import small asset (< 10 MB) with no dependencies | Duration | < 10 s | R-15.17.2 |

### TC-15.17.2.B3 Dependency Resolution

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | `resolve_dependencies` for asset with 10 transitive deps | Duration | < 2 s | R-15.17.2 |

### TC-15.17.5.B1 Compat Test Per Asset

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Full compat test (import, load, run tests) for one asset | Duration | < 5 min | R-15.17.5 |

### TC-15.17.4.B1 Publisher Upload

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Upload 1 GB package on 100 Mbps connection | Duration | < 60 s | R-15.17.4 |
