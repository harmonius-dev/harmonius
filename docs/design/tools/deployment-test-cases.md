# Deployment and Mod Support Test Cases

Companion test cases for [deployment.md](deployment.md).

## Unit Tests

### TC-15.14.1.1 Cook Strips Editor Content

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Cook with `BuildProfile::Shipping`, include editor-only asset | Cooked output does not contain editor-only asset | R-15.14.1 |
| 2 | Cook with `BuildProfile::Development`, include editor-only asset | Cooked output contains editor-only asset | R-15.14.1 |

### TC-15.14.1.2 Cook Platform Texture Format

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Cook texture for `TargetPlatform::Windows` | Output texture compressed as BC7 | R-15.14.1 |
| 2 | Cook texture for `TargetPlatform::Ios` | Output texture compressed as ASTC | R-15.14.1 |

### TC-15.14.6.1 Bundle BLAKE3 Manifest

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Build bundle with 100 assets | Manifest contains 100 entries, each with correct BLAKE3 hash | R-15.14.6 |

### TC-15.14.6.2 Bundle Integrity Verify

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Build bundle, verify integrity | Verification passes | R-15.14.6 |
| 2 | Build bundle, flip 1 bit in asset data, verify integrity | Verification fails with hash mismatch | R-15.14.6 |

### TC-15.14.7.1 CDC Shift Resilience

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Chunk 10 MB file, insert 1 byte at offset 5 MB, re-chunk | Only 1-2 chunks differ, remaining chunks identical | R-15.14.7 |

### TC-15.14.7.2 Delta Patch Size

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Old version 100 MB, new version with 5% content change | Patch size < 25 MB (< 25% of full size) | R-15.14.7 |

### TC-15.14.7.3 Patch Apply Verify

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Apply patch to installed bundles | `PatchResult { integrity_verified: true }`, post-patch BLAKE3 matches expected | R-15.14.7 |

### TC-15.14.3.1 Cert Rule Pass

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Compliant build against all certification rules | `CertReport { passed: true, results: [all passed] }` | R-15.14.3 |

### TC-15.14.3.2 Cert Rule Fail Remediation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Build missing required icon, run cert check | `CertReport { passed: false }` with `remediation` text for icon rule | R-15.14.3 |

### TC-15.16.2.1 Mod Policy Constraint

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mod with 5000 entities, policy `entity_budget: 1000` | `Err(ConstraintViolation::ExceedsEntityBudget { count: 5000, max: 1000 })` | R-15.16.2 |

### TC-15.16.4.1 Mod Sandbox Blocks FS

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mod logic graph containing "FileRead" node, policy blocks FS | `Err(SandboxError::RestrictedNode { node_name: "FileRead", .. })` | R-15.16.4 |

### TC-15.16.4.2 Mod Sandbox Blocks Network

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mod logic graph containing "HttpRequest" node, policy blocks network | `Err(SandboxError::RestrictedNode { node_name: "HttpRequest", .. })` | R-15.16.4 |

### TC-15.16.3.1 Mod Integrity BLAKE3

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Package mod, verify integrity | Verification passes | R-15.16.3 |
| 2 | Package mod, tamper with 1 byte, verify integrity | `Err(ModLoadError::IntegrityFailed)` | R-15.16.3 |

### TC-15.16.3.2 Mod Unsigned Warning

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Load mod with `signature: None` | `Err(ModLoadError::UnsignedModRejected)` | R-15.16.3 |

### TC-15.16.4.3 Mod Dependency Order

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mod A depends on B, B depends on C | Load order is `[C, B, A]` | R-15.16.4 |

### TC-15.16.4.4 Mod Conflict Detection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mod X modifies asset 42, mod Y modifies asset 42 | `ModLoadReport.conflicts` contains `ModConflict { asset_id: 42, mod_a: X, mod_b: Y }` | R-15.16.4 |

### TC-15.16.4.5 Mod Source Tagging

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Load mod, spawn entity from mod content | Entity has `ModSource { mod_id }` component | R-15.16.4 |

### TC-15.16.6.1 Moderation Scan Restricted

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mod contains blocked node category | `ScanReport { restricted_nodes_found: ["NetworkAccess"] }` | R-15.16.6 |

### TC-15.16.6.2 Moderation Audit Log

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Approve mod, then reject another mod | Two `ModerationLogEntry` records with correct actions | R-15.16.6 |

## Integration Tests

### TC-15.14.1.I1 Full Pipeline macOS

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Cook, sign (`codesign`), create `.dmg` on macOS | Valid `.dmg` with signed binary | R-15.14.1 |

### TC-15.14.1.I2 Full Pipeline Windows

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Cook, sign (`signtool`), create `.msi` on Windows | Valid `.msi` with Authenticode signature | R-15.14.1 |

### TC-15.14.1.I3 Full Pipeline Linux

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Cook, create AppImage on Linux | Valid AppImage that executes | R-15.14.1 |

### TC-15.14.2.I1 Deploy to Device

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Deploy to connected device, second deploy with 1 changed asset | Second deploy transfers only the changed asset | R-15.14.2 |

### TC-15.14.2.I2 Device Console Stream

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Deploy and launch on device, log "hello" from game | Editor console receives "hello" via `stream_console` | R-15.14.2 |

### TC-15.14.4.I1 Signing macOS Notarize

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Sign with `codesign`, notarize with `notarytool`, staple ticket | `spctl --assess` passes on result | R-15.14.4 |

### TC-15.14.4.I2 Signing Windows Authenticode

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Sign with `signtool` using test certificate | `signtool verify` passes | R-15.14.4 |

### TC-15.14.6.I1 DLC Entitlement Gate

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Load DLC bundle with valid entitlement | Bundle loads successfully | R-15.14.6 |
| 2 | Load DLC bundle without entitlement | Bundle rejected with entitlement error | R-15.14.6 |

### TC-15.14.8.I1 Store Submit Steam Staging

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Submit to Steam staging depot via SteamCMD | `SubmissionReceipt` with valid `submission_id` | R-15.14.8 |

### TC-15.14.8.I2 Store Status Poll

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Submit, then `poll_status(receipt)` | Returns one of `Pending`, `InReview`, `Approved`, or `Rejected` | R-15.14.8 |

### TC-15.16.1.I1 Mod SDK to Runtime

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Create mod in SDK, package, load in game runtime | Mod content visible in game, entities spawned | R-15.16.1 |

### TC-15.16.5.I1 Mod Workshop Upload

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Upload mod to Steam Workshop via `WorkshopClient` | Workshop item ID returned, listing visible | R-15.16.5 |

### TC-15.16.5.I2 Mod Repository Upload

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Upload mod to self-hosted repo, download, verify | Downloaded mod matches uploaded content hash | R-15.16.5 |

### TC-15.16.5.I3 Mod Auto Update

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Install mod v1.0, publish v1.1, launch game | `check_updates` returns `(mod_id, 1.1)`, update applied | R-15.16.5 |

### TC-15.16.6.I1 Mod Revoke Force Uninstall

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Install mod, revoke via `ModerationService`, launch game | Mod removed from local install | R-15.16.6 |

## Benchmarks

### TC-15.14.1.B1 Asset Cooking Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Cook 10 GB of assets for Windows | Throughput | >= 500 MB/s cooked output | R-15.14.1 |

### TC-15.14.6.B1 Bundle Building Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Build bundle from 5 GB cooked assets | I/O throughput | >= 1 GB/s | R-15.14.6 |

### TC-15.14.7.B1 CDC Chunking Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Chunk 10 GB of bundle data using Rabin fingerprinting | Throughput | >= 800 MB/s | R-15.14.7 |

### TC-15.14.7.B2 Delta Patch Generation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Generate delta between two 5 GB bundle sets | Input scan rate | >= 500 MB/s | R-15.14.7 |

### TC-15.14.7.B3 Patch Application Speed

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Apply 500 MB patch to installed bundles | Write throughput | >= 200 MB/s | R-15.14.7 |

### TC-15.16.4.B1 Mod Load Time

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Load 10 MB mod with 5 assets and 2 logic graphs | Duration | < 500 ms | R-15.16.4 |

### TC-15.16.3.B1 Mod Integrity Check

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | BLAKE3 integrity verification for 10 MB mod | Duration | < 50 ms | R-15.16.3 |

### TC-15.16.4.B2 Budget Monitoring Overhead

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | `check_budgets` for 10 loaded mods per frame | Frame overhead | < 0.1 ms | R-15.16.4 |
