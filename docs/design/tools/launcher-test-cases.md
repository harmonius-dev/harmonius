# Engine Launcher Test Cases

Companion test cases for [launcher.md](launcher.md).

## Unit Tests

### TC-15.15.1.1 Version Catalog Parse

| # | Requirement |
|---|-------------|
| 1 | R-15.15.1   |
| 2 | R-15.15.1   |

1. **#1** — Valid JSON with 3 `AvailableVersion` entries
   - **Expected:** `VersionCatalog { available: [v1.0, v1.1, v2.0] }`
2. **#2** — Malformed JSON
   - **Expected:** Returns parse error

### TC-15.15.1.2 Version Install Isolated

| # | Requirement |
|---|-------------|
| 1 | R-15.15.1   |

1. **#1** — Install v1.0 and v2.0
   - **Expected:** Two directories `1.0.0/` and `2.0.0/`, no shared files

### TC-15.15.1.3 Version Uninstall Frees Space

| # | Requirement |
|---|-------------|
| 1 | R-15.15.1   |

1. **#1** — Install v1.0 (500 MB), `uninstall(1.0)`
   - **Expected:** Directory removed, `disk_usage` no longer lists v1.0

### TC-15.15.1.4 Version Rollback

| # | Requirement |
|---|-------------|
| 1 | R-15.15.1   |

1. **#1** — Install v1.0 and v2.0, active is v2.0, `rollback(1.0)`
   - **Expected:** Active version pointer set to v1.0, no re-download

### TC-15.15.1.5 Version Integrity BLAKE3

| # | Requirement |
|---|-------------|
| 1 | R-15.15.1   |
| 2 | R-15.15.1   |

1. **#1** — Download archive, verify BLAKE3 matches expected
   - **Expected:** Verification passes, install proceeds
2. **#2** — Download archive, corrupt 1 byte, verify BLAKE3
   - **Expected:** `Err(IntegrityMismatch { expected, actual })`

### TC-15.15.4.1 Project File Roundtrip

| # | Requirement |
|---|-------------|
| 1 | R-15.15.4   |

1. **#1** — Write `ProjectFile` to TOML, read back
   - **Expected:** All fields match original

### TC-15.15.4.2 Project File Version Pin

| # | Requirement |
|---|-------------|
| 1 | R-15.15.4   |

1. **#1** — `ProjectFile { engine_version: SemVer(2,3,0), .. }`, write, read
   - **Expected:** `engine_version == SemVer(2,3,0)`

### TC-15.15.2.1 Migration Incremental

| # | Requirement |
|---|-------------|
| 1 | R-15.15.2   |

1. **#1** — `migrate(project, 1.2, 1.5)` with scripts for 1.2->1.3, 1.3->1.4, 1.4->1.5
   - **Expected:** `MigrationReport { steps_completed: 3 }`

### TC-15.15.2.2 Migration Backup Created

| # | Requirement |
|---|-------------|
| 1 | R-15.15.2   |

1. **#1** — `migrate(project, 1.2, 1.3)`
   - **Expected:** `MigrationReport.backup_path` exists on disk

### TC-15.15.2.3 Migration Restore

| # | Requirement |
|---|-------------|
| 1 | R-15.15.2   |

1. **#1** — Migrate, then `restore_backup(backup_path, project_path)`
   - **Expected:** Project files match pre-migration state

### TC-15.15.2.4 Migration Report

| # | Requirement |
|---|-------------|
| 1 | R-15.15.2   |

1. **#1** — Migrate project with 2 deprecated APIs and 1 file change
   - **Expected:** Report contains 1 change, 2 deprecations

### TC-15.15.2.5 Migration Graph API

| # | Requirement |
|---|-------------|
| 1 | R-15.15.2   |

1. **#1** — Project with deprecated node "OldDelay", migration maps to "NewDelay"
   - **Expected:** Logic graph contains "NewDelay" with remapped parameters

### TC-15.15.3.1 Template Instantiate

| # | Requirement |
|---|-------------|
| 1 | R-15.15.3   |

1. **#1** — `instantiate("rpg_template", project_dir, "MyGame")`
   - **Expected:** Valid `.harmonius` file with correct modules for RPG template

### TC-15.15.3.2 Template Each Genre

| # | Requirement |
|---|-------------|
| 1 | R-15.15.3   |
| 2 | R-15.15.3   |
| 3 | R-15.15.3   |
| 4 | R-15.15.3   |
| 5 | R-15.15.3   |
| 6 | R-15.15.3   |

1. **#1** — Instantiate `Rpg` template
   - **Expected:** Valid project created
2. **#2** — Instantiate `Fps` template
   - **Expected:** Valid project created
3. **#3** — Instantiate `Rts` template
   - **Expected:** Valid project created
4. **#4** — Instantiate `Platformer2d` template
   - **Expected:** Valid project created
5. **#5** — Instantiate `VrExperience` template
   - **Expected:** Valid project created
6. **#6** — Instantiate `Empty` template
   - **Expected:** Valid project created

### TC-15.15.5.1 Keychain Store Retrieve

| # | Requirement |
|---|-------------|
| 1 | R-15.15.5   |

1. **#1** — `store("harmonius", "github", secret_bytes)`, `retrieve("harmonius", "github")`
   - **Expected:** Returns `secret_bytes`

### TC-15.15.5.2 Keychain Delete

| # | Requirement |
|---|-------------|
| 1 | R-15.15.5   |

1. **#1** — Store credential, `delete("harmonius", "github")`, retrieve
   - **Expected:** Returns `Err(KeychainError::NotFound)`

### TC-15.15.5.3 Account Link OAuth

| # | Requirement |
|---|-------------|
| 1 | R-15.15.5   |
| 2 | R-15.15.5   |

1. **#1** — `link(GitHub)` with mock OAuth server
   - **Expected:** Returns `LinkedAccount { provider: GitHub, is_active: true }`
2. **#2** — `link(GitLab)` with mock OAuth server
   - **Expected:** Returns `LinkedAccount { provider: GitLab, is_active: true }`

### TC-15.15.5.4 Account Switch

| # | Requirement |
|---|-------------|
| 1 | R-15.15.5   |

1. **#1** — Link accounts A and B, `switch(B)`
   - **Expected:** Account B is active, A is inactive

### TC-15.15.5.5 Preference Roundtrip

| # | Requirement |
|---|-------------|
| 1 | R-15.15.5   |

1. **#1** — `upload(prefs)`, `download()`
   - **Expected:** Downloaded prefs match uploaded prefs

### TC-15.15.6.1 Collab Wizard Validates

| # | Requirement |
|---|-------------|
| 1 | R-15.15.6   |

1. **#1** — `validate_connectivity("https://invalid.example.com")`
   - **Expected:** Returns `Err(ConnectivityFailed { url })`

### TC-15.15.6.2 Collab Team Config Saved

| # | Requirement |
|---|-------------|
| 1 | R-15.15.6   |

1. **#1** — Run wizard, write `CollabConfig` to `.harmonius`
   - **Expected:** `ProjectFile.team` contains collab server URL and members

## Integration Tests

### TC-15.15.1.I1 Install and Launch

| # | Requirement |
|---|-------------|
| 1 | R-15.15.1   |

1. **#1** — `install(2.3.0)`, launch editor binary
   - **Expected:** Editor process starts, returns exit code 0

### TC-15.15.1.I2 Side by Side Versions

| # | Requirement |
|---|-------------|
| 1 | R-15.15.1   |

1. **#1** — Install v1.0 and v2.0, launch v1.0 then v2.0
   - **Expected:** Both versions start independently, no shared state

### TC-15.15.1.I3 Auto Update macOS

| # | Requirement |
|---|-------------|
| 1 | R-15.15.1   |

1. **#1** — Mock Sparkle appcast feed with newer version
   - **Expected:** Launcher downloads and applies update

### TC-15.15.1.I4 Auto Update Windows

| # | Requirement |
|---|-------------|
| 1 | R-15.15.1   |

1. **#1** — Mock WinSparkle appcast feed with newer version
   - **Expected:** Launcher downloads and applies update

### TC-15.15.1.I5 Auto Update Linux

| # | Requirement |
|---|-------------|
| 1 | R-15.15.1   |

1. **#1** — Mock AppImage update endpoint with newer version
   - **Expected:** AppImage delta applied, binary replaced

### TC-15.15.4.I1 File Association macOS

| # | Requirement |
|---|-------------|
| 1 | R-15.15.4   |

1. **#1** — `register()`, open `.harmonius` file
   - **Expected:** Launcher opens with project loaded

### TC-15.15.4.I2 File Association Windows

| # | Requirement |
|---|-------------|
| 1 | R-15.15.4   |

1. **#1** — `register()`, double-click `.harmonius` file
   - **Expected:** Launcher opens with project loaded

### TC-15.15.4.I3 File Association Linux

| # | Requirement |
|---|-------------|
| 1 | R-15.15.4   |

1. **#1** — `register()`, `xdg-open project.harmonius`
   - **Expected:** Launcher opens with project loaded

### TC-15.15.2.I1 Migration Full Pipeline

| # | Requirement |
|---|-------------|
| 1 | R-15.15.2   |

1. **#1** — Real project at v1.0, migrate to v1.3
   - **Expected:** All assets re-cook, logic graphs migrated, project opens in v1.3

### TC-15.15.5.I1 Preference Sync Two Machines

| # | Requirement |
|---|-------------|
| 1 | R-15.15.5   |

1. **#1** — Machine A uploads prefs, machine B downloads
   - **Expected:** Machine B preferences match machine A

### TC-15.15.5.I2 Keychain Per Platform

| # | Requirement |
|---|-------------|
| 1 | R-15.15.5   |

1. **#1** — Store and retrieve credential on current platform
   - **Expected:** Credential matches on macOS Keychain / Windows CredMgr / Linux libsecret

### TC-15.15.6.I1 Collab Wizard Full Flow

| # | Requirement |
|---|-------------|
| 1 | R-15.15.6   |

1. **#1** — Run wizard with mock Git, LFS, and collab server
   - **Expected:** `CollabConfig` written to `.harmonius`, all connections validated

### TC-15.15.6.I2 Collab Config Inherited

| # | Requirement |
|---|-------------|
| 1 | R-15.15.6   |

1. **#1** — Clone repo with `.harmonius` containing team config
   - **Expected:** Cloned project has correct `team` section

## Benchmarks

### TC-15.15.1.B1 Version Download

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Download 1 GB archive from S3/CloudFront | Throughput | >= 100 MB/s | R-15.15.1 |

### TC-15.15.1.B2 Version Install

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Extract + BLAKE3 verify 2 GB archive | Duration | < 30 s | R-15.15.1 |

### TC-15.15.3.B1 Project Open No Migration

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Open project with matching engine version | Duration | < 500 ms | R-15.15.3 |

### TC-15.15.2.B1 Migration Single Step

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Single version migration on 10 GB project | Duration | < 60 s | R-15.15.2 |

### TC-15.15.5.B1 Preference Sync Upload

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Upload `UserPreferences` (~10 KB) to cloud | Duration | < 1 s | R-15.15.5 |

### TC-15.15.3.B2 Template Instantiation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Instantiate cached RPG template into new project | Duration | < 5 s | R-15.15.3 |

### TC-15.15.NF.B1 Launcher Startup

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Cold start launcher to home screen display | Duration | < 2 s | R-15.15.1 |
