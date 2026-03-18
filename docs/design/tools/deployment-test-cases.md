# Deployment and Mod Support Test Cases

Companion test cases for [deployment.md](deployment.md).

## Unit Tests

### TC-15.14.1.1 Cook Strips Editor Content

| # | Requirement |
|---|-------------|
| 1 | R-15.14.1   |
| 2 | R-15.14.1   |

1. **#1** — Cook with `BuildProfile::Shipping`, include editor-only asset
   - **Expected:** Cooked output does not contain editor-only asset
2. **#2** — Cook with `BuildProfile::Development`, include editor-only asset
   - **Expected:** Cooked output contains editor-only asset

### TC-15.14.1.2 Cook Platform Texture Format

| # | Requirement |
|---|-------------|
| 1 | R-15.14.1   |
| 2 | R-15.14.1   |

1. **#1** — Cook texture for `TargetPlatform::Windows`
   - **Expected:** Output texture compressed as BC7
2. **#2** — Cook texture for `TargetPlatform::Ios`
   - **Expected:** Output texture compressed as ASTC

### TC-15.14.6.1 Bundle BLAKE3 Manifest

| # | Requirement |
|---|-------------|
| 1 | R-15.14.6   |

1. **#1** — Build bundle with 100 assets
   - **Expected:** Manifest contains 100 entries, each with correct BLAKE3 hash

### TC-15.14.6.2 Bundle Integrity Verify

| # | Requirement |
|---|-------------|
| 1 | R-15.14.6   |
| 2 | R-15.14.6   |

1. **#1** — Build bundle, verify integrity
   - **Expected:** Verification passes
2. **#2** — Build bundle, flip 1 bit in asset data, verify integrity
   - **Expected:** Verification fails with hash mismatch

### TC-15.14.7.1 CDC Shift Resilience

| # | Requirement |
|---|-------------|
| 1 | R-15.14.7   |

1. **#1** — Chunk 10 MB file, insert 1 byte at offset 5 MB, re-chunk
   - **Expected:** Only 1-2 chunks differ, remaining chunks identical

### TC-15.14.7.2 Delta Patch Size

| # | Requirement |
|---|-------------|
| 1 | R-15.14.7   |

1. **#1** — Old version 100 MB, new version with 5% content change
   - **Expected:** Patch size < 25 MB (< 25% of full size)

### TC-15.14.7.3 Patch Apply Verify

| # | Requirement |
|---|-------------|
| 1 | R-15.14.7   |

1. **#1** — Apply patch to installed bundles
   - **Expected:** `PatchResult { integrity_verified: true }`, post-patch BLAKE3 matches expected

### TC-15.14.3.1 Cert Rule Pass

| # | Requirement |
|---|-------------|
| 1 | R-15.14.3   |

1. **#1** — Compliant build against all certification rules
   - **Expected:** `CertReport { passed: true, results: [all passed] }`

### TC-15.14.3.2 Cert Rule Fail Remediation

| # | Requirement |
|---|-------------|
| 1 | R-15.14.3   |

1. **#1** — Build missing required icon, run cert check
   - **Expected:** `CertReport { passed: false }` with `remediation` text for icon rule

### TC-15.16.2.1 Mod Policy Constraint

| # | Requirement |
|---|-------------|
| 1 | R-15.16.2   |

1. **#1** — Mod with 5000 entities, policy `entity_budget: 1000`
   - **Expected:** `Err(ConstraintViolation::ExceedsEntityBudget { count: 5000, max: 1000 })`

### TC-15.16.4.1 Mod Sandbox Blocks FS

| # | Requirement |
|---|-------------|
| 1 | R-15.16.4   |

1. **#1** — Mod logic graph containing "FileRead" node, policy blocks FS
   - **Expected:** `Err(SandboxError::RestrictedNode { node_name: "FileRead", .. })`

### TC-15.16.4.2 Mod Sandbox Blocks Network

| # | Requirement |
|---|-------------|
| 1 | R-15.16.4   |

1. **#1** — Mod logic graph containing "HttpRequest" node, policy blocks network
   - **Expected:** `Err(SandboxError::RestrictedNode { node_name: "HttpRequest", .. })`

### TC-15.16.3.1 Mod Integrity BLAKE3

| # | Requirement |
|---|-------------|
| 1 | R-15.16.3   |
| 2 | R-15.16.3   |

1. **#1** — Package mod, verify integrity
   - **Expected:** Verification passes
2. **#2** — Package mod, tamper with 1 byte, verify integrity
   - **Expected:** `Err(ModLoadError::IntegrityFailed)`

### TC-15.16.3.2 Mod Unsigned Warning

| # | Requirement |
|---|-------------|
| 1 | R-15.16.3   |

1. **#1** — Load mod with `signature: None`
   - **Expected:** `Err(ModLoadError::UnsignedModRejected)`

### TC-15.16.4.3 Mod Dependency Order

| # | Requirement |
|---|-------------|
| 1 | R-15.16.4   |

1. **#1** — Mod A depends on B, B depends on C
   - **Expected:** Load order is `[C, B, A]`

### TC-15.16.4.4 Mod Conflict Detection

| # | Requirement |
|---|-------------|
| 1 | R-15.16.4   |

1. **#1** — Mod X modifies asset 42, mod Y modifies asset 42
   - **Expected:** `ModLoadReport.conflicts` contains
     `ModConflict { asset_id: 42, mod_a: X, mod_b: Y }`

### TC-15.16.4.5 Mod Source Tagging

| # | Requirement |
|---|-------------|
| 1 | R-15.16.4   |

1. **#1** — Load mod, spawn entity from mod content
   - **Expected:** Entity has `ModSource { mod_id }` component

### TC-15.16.6.1 Moderation Scan Restricted

| # | Requirement |
|---|-------------|
| 1 | R-15.16.6   |

1. **#1** — Mod contains blocked node category
   - **Expected:** `ScanReport { restricted_nodes_found: ["NetworkAccess"] }`

### TC-15.16.6.2 Moderation Audit Log

| # | Requirement |
|---|-------------|
| 1 | R-15.16.6   |

1. **#1** — Approve mod, then reject another mod
   - **Expected:** Two `ModerationLogEntry` records with correct actions

## Integration Tests

### TC-15.14.1.I1 Full Pipeline macOS

| # | Requirement |
|---|-------------|
| 1 | R-15.14.1   |

1. **#1** — Cook, sign (`codesign`), create `.dmg` on macOS
   - **Expected:** Valid `.dmg` with signed binary

### TC-15.14.1.I2 Full Pipeline Windows

| # | Requirement |
|---|-------------|
| 1 | R-15.14.1   |

1. **#1** — Cook, sign (`signtool`), create `.msi` on Windows
   - **Expected:** Valid `.msi` with Authenticode signature

### TC-15.14.1.I3 Full Pipeline Linux

| # | Requirement |
|---|-------------|
| 1 | R-15.14.1   |

1. **#1** — Cook, create AppImage on Linux
   - **Expected:** Valid AppImage that executes

### TC-15.14.2.I1 Deploy to Device

| # | Requirement |
|---|-------------|
| 1 | R-15.14.2   |

1. **#1** — Deploy to connected device, second deploy with 1 changed asset
   - **Expected:** Second deploy transfers only the changed asset

### TC-15.14.2.I2 Device Console Stream

| # | Requirement |
|---|-------------|
| 1 | R-15.14.2   |

1. **#1** — Deploy and launch on device, log "hello" from game
   - **Expected:** Editor console receives "hello" via `stream_console`

### TC-15.14.4.I1 Signing macOS Notarize

| # | Requirement |
|---|-------------|
| 1 | R-15.14.4   |

1. **#1** — Sign with `codesign`, notarize with `notarytool`, staple ticket
   - **Expected:** `spctl --assess` passes on result

### TC-15.14.4.I2 Signing Windows Authenticode

| # | Requirement |
|---|-------------|
| 1 | R-15.14.4   |

1. **#1** — Sign with `signtool` using test certificate
   - **Expected:** `signtool verify` passes

### TC-15.14.6.I1 DLC Entitlement Gate

| # | Requirement |
|---|-------------|
| 1 | R-15.14.6   |
| 2 | R-15.14.6   |

1. **#1** — Load DLC bundle with valid entitlement
   - **Expected:** Bundle loads successfully
2. **#2** — Load DLC bundle without entitlement
   - **Expected:** Bundle rejected with entitlement error

### TC-15.14.8.I1 Store Submit Steam Staging

| # | Requirement |
|---|-------------|
| 1 | R-15.14.8   |

1. **#1** — Submit to Steam staging depot via SteamCMD
   - **Expected:** `SubmissionReceipt` with valid `submission_id`

### TC-15.14.8.I2 Store Status Poll

| # | Requirement |
|---|-------------|
| 1 | R-15.14.8   |

1. **#1** — Submit, then `poll_status(receipt)`
   - **Expected:** Returns one of `Pending`, `InReview`, `Approved`, or `Rejected`

### TC-15.16.1.I1 Mod SDK to Runtime

| # | Requirement |
|---|-------------|
| 1 | R-15.16.1   |

1. **#1** — Create mod in SDK, package, load in game runtime
   - **Expected:** Mod content visible in game, entities spawned

### TC-15.16.5.I1 Mod Workshop Upload

| # | Requirement |
|---|-------------|
| 1 | R-15.16.5   |

1. **#1** — Upload mod to Steam Workshop via `WorkshopClient`
   - **Expected:** Workshop item ID returned, listing visible

### TC-15.16.5.I2 Mod Repository Upload

| # | Requirement |
|---|-------------|
| 1 | R-15.16.5   |

1. **#1** — Upload mod to self-hosted repo, download, verify
   - **Expected:** Downloaded mod matches uploaded content hash

### TC-15.16.5.I3 Mod Auto Update

| # | Requirement |
|---|-------------|
| 1 | R-15.16.5   |

1. **#1** — Install mod v1.0, publish v1.1, launch game
   - **Expected:** `check_updates` returns `(mod_id, 1.1)`, update applied

### TC-15.16.6.I1 Mod Revoke Force Uninstall

| # | Requirement |
|---|-------------|
| 1 | R-15.16.6   |

1. **#1** — Install mod, revoke via `ModerationService`, launch game
   - **Expected:** Mod removed from local install

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
