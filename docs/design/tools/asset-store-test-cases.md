# Asset Store / Marketplace Test Cases

Companion test cases for [asset-store.md](asset-store.md).

## Unit Tests

### TC-15.17.1.1 Search Category Filter

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `SearchQuery { category: Some(Mesh3d), .. }` against catalog with mixed types | Results contain only `Mesh3d` assets | R-15.17.1 |
| 2 | `SearchQuery { category: Some(VfxGraph), .. }` against catalog with no VFX assets | Results are empty, `total_count == 0` | R-15.17.1 |

### TC-15.17.1.2 Search Tag Filter

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `SearchQuery { tags: ["medieval", "stone"], .. }` | Results contain only assets tagged with both "medieval" AND "stone" | R-15.17.1 |
| 2 | `SearchQuery { tags: ["nonexistent"], .. }` | Results are empty | R-15.17.1 |

### TC-15.17.1.3 Search Engine Version Filter

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `SearchQuery { engine_version: Some(SemVer(2,0,0)), .. }` | Excludes assets with `min_engine_version > 2.0.0` | R-15.17.1 |
| 2 | Asset with `min_engine_version: 1.5.0`, query version `2.0.0` | Asset included in results | R-15.17.1 |
| 3 | Asset with `min_engine_version: 3.0.0`, query version `2.0.0` | Asset excluded from results | R-15.17.1 |

### TC-15.17.1.4 Search Sort Orders

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `sort: Rating`, 3 assets rated 3.0, 5.0, 4.0 | Results ordered [5.0, 4.0, 3.0] | R-15.17.1 |
| 2 | `sort: Newest`, 3 assets published at t1 < t2 < t3 | Results ordered [t3, t2, t1] | R-15.17.1 |
| 3 | `sort: PriceLowToHigh`, prices 500, 100, 300 | Results ordered [100, 300, 500] | R-15.17.1 |

### TC-15.17.2.1 Import Resolves Dependencies

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Asset A depends on B, B depends on C | `resolve_dependencies(A)` returns `[C, B, A]` (topological) | R-15.17.2 |
| 2 | Asset with no dependencies | `resolve_dependencies(asset)` returns `[asset]` | R-15.17.2 |

### TC-15.17.2.2 Import Detects Version Conflict

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Asset requires plugin v2.0, installed v1.5 | `ImportResult::VersionConflict { required: 2.0, installed: 1.5, .. }` | R-15.17.2 |

### TC-15.17.2.3 Import Detects Missing Plugin

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Asset requires plugin "physics_ext", not installed | `ImportResult::MissingPlugin { plugin_id: "physics_ext", .. }` | R-15.17.2 |

### TC-15.17.3.1 Review Rating Range

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `submit_review(asset, rating: 5, "Great")` | Returns `Ok(review_id)` | R-15.17.3 |
| 2 | `submit_review(asset, rating: 0, "Bad")` | Returns `Err(...)` (rating < 1) | R-15.17.3 |
| 3 | `submit_review(asset, rating: 6, "Amazing")` | Returns `Err(...)` (rating > 5) | R-15.17.3 |

### TC-15.17.3.2 Review Moderation Pending

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Submit new review | `review.moderation == ModerationStatus::Pending` | R-15.17.3 |

### TC-15.17.4.1 Publisher Registration

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `register(PublisherRegistration { display_name: "Studio", .. })` | Returns `Ok(publisher_id)`, publisher status `Unverified` | R-15.17.4 |

### TC-15.17.4.2 Regional Pricing

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Base price 999 cents USD, regional override BR: 499 BRL | Brazil query returns 499 BRL, US query returns 999 USD | R-15.17.4 |

### TC-15.17.4.3 Discount Code Apply

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Asset price 1000 cents, discount code 20% off | Checkout price = 800 cents | R-15.17.4 |

### TC-15.17.4.4 Discount Code Expired

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Discount code with `expires_at` in the past | Code rejected, full price charged | R-15.17.4 |

### TC-15.17.5.1 Compat Badge Awarded

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Asset passes compat test for engine v2.3.0 | `compat_badges` contains `CompatBadge { engine_version: 2.3.0, test_passed: true }` | R-15.17.5 |

### TC-15.17.5.2 Compat Badge Revoked

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Asset fails compat test for engine v2.4.0 | `compat_badges` for v2.4.0 has `test_passed: false` | R-15.17.5 |

### TC-15.17.6.1 Commission Calculation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Sale price 1000 cents, commission rate 0.12 | Commission = 120 cents, net = 880 cents | R-15.17.6 |

### TC-15.17.6.2 Refund Within Window

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Purchase 10 days ago, request refund | `RefundResult::Refunded { refund_id }` | R-15.17.6 |
| 2 | Publisher revenue after refund | Revenue reversed by sale amount minus commission | R-15.17.6 |

### TC-15.17.6.3 Refund Outside Window

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Purchase 15 days ago, request refund | `RefundResult::WindowExpired` | R-15.17.6 |

### TC-15.17.7.1 All Asset Types Accepted

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Upload asset with category `Mesh3d` | Upload succeeds | R-15.17.7 |
| 2 | Upload asset with category `RustPlugin` | Upload succeeds | R-15.17.7 |
| 3 | Upload asset for each of 15 `AssetCategory` variants | All 15 uploads succeed | R-15.17.7 |

### TC-15.17.8.1 License Validation Commercial

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `validate_license(Personal, project_commercial: true)` | `LicenseValidation::CommercialRestriction` | R-15.17.8 |
| 2 | `validate_license(Commercial, project_commercial: true)` | `LicenseValidation::Valid` | R-15.17.8 |

### TC-15.17.8.2 License No Runtime DRM

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Import asset, disconnect network, use asset | Asset functions without network | R-15.17.8 |

## Integration Tests

### TC-15.17.2.I1 End to End Purchase Import

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Purchase paid asset, download, import | Asset visible in asset browser, files on disk | R-15.17.2 |

### TC-15.17.2.I2 Dependency Chain Import

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Import asset with 3-level dependency chain (A->B->C) | All three assets imported, C first, then B, then A | R-15.17.2 |

### TC-15.17.2.I3 Free Asset No Payment

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Import free asset (price = 0) | No Stripe payment flow, asset downloaded and imported | R-15.17.2 |

### TC-15.17.1.I1 Search Semantic Query

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Search "dark fantasy castle" against catalog with relevant assets | Top results include castle/medieval assets | R-15.17.1 |

### TC-15.17.4.I1 Publisher Upload to Listing

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Upload package, pass compat test, approve review | Asset visible on marketplace with `Published` status | R-15.17.4 |

### TC-15.17.5.I1 Compat Test Pipeline

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Upload asset, trigger compat test via SQS/CodeBuild | Badge assigned, publisher notified | R-15.17.5 |

### TC-15.17.6.I1 Payout Calculation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 5 purchases at 1000 cents + 1 refund | Net payout = (5 * 880) - 880 = 3520 cents | R-15.17.6 |

### TC-15.17.6.I2 Stripe Payment Flow

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Purchase with Stripe test card `4242...` | `PurchaseResponse` with valid `download_url` | R-15.17.6 |

### TC-15.17.3.I1 Review Moderation Flow

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Submit review, moderate to Approved | Review visible in `list_reviews` | R-15.17.3 |
| 2 | Submit review, moderate to Rejected | Review not visible in `list_reviews` | R-15.17.3 |

### TC-15.17.3.I2 Collection Browse

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Create collection with 5 assets, browse from editor | Collection returned with all 5 asset IDs | R-15.17.3 |

### TC-15.17.7.I1 Rust Plugin Per Platform

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Upload Rust plugin, request download for macOS | Receives macOS-compiled binary | R-15.17.7 |

### TC-15.17.8.I1 Offline After Import

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Import asset, disconnect network, open project | Asset loads and renders correctly | R-15.17.8 |

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
