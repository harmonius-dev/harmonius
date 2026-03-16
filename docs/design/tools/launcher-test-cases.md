# Engine Launcher Test Cases

Companion test cases for [launcher.md](launcher.md).

## Unit Tests

### TC-15.15.1.1 Version Catalog Parse

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Valid JSON with 3 `AvailableVersion` entries | `VersionCatalog { available: [v1.0, v1.1, v2.0] }` | R-15.15.1 |
| 2 | Malformed JSON | Returns parse error | R-15.15.1 |

### TC-15.15.1.2 Version Install Isolated

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Install v1.0 and v2.0 | Two directories `1.0.0/` and `2.0.0/`, no shared files | R-15.15.1 |

### TC-15.15.1.3 Version Uninstall Frees Space

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Install v1.0 (500 MB), `uninstall(1.0)` | Directory removed, `disk_usage` no longer lists v1.0 | R-15.15.1 |

### TC-15.15.1.4 Version Rollback

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Install v1.0 and v2.0, active is v2.0, `rollback(1.0)` | Active version pointer set to v1.0, no re-download | R-15.15.1 |

### TC-15.15.1.5 Version Integrity BLAKE3

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Download archive, verify BLAKE3 matches expected | Verification passes, install proceeds | R-15.15.1 |
| 2 | Download archive, corrupt 1 byte, verify BLAKE3 | `Err(IntegrityMismatch { expected, actual })` | R-15.15.1 |

### TC-15.15.4.1 Project File Roundtrip

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Write `ProjectFile` to TOML, read back | All fields match original | R-15.15.4 |

### TC-15.15.4.2 Project File Version Pin

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `ProjectFile { engine_version: SemVer(2,3,0), .. }`, write, read | `engine_version == SemVer(2,3,0)` | R-15.15.4 |

### TC-15.15.2.1 Migration Incremental

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `migrate(project, 1.2, 1.5)` with scripts for 1.2->1.3, 1.3->1.4, 1.4->1.5 | `MigrationReport { steps_completed: 3 }` | R-15.15.2 |

### TC-15.15.2.2 Migration Backup Created

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `migrate(project, 1.2, 1.3)` | `MigrationReport.backup_path` exists on disk | R-15.15.2 |

### TC-15.15.2.3 Migration Restore

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Migrate, then `restore_backup(backup_path, project_path)` | Project files match pre-migration state | R-15.15.2 |

### TC-15.15.2.4 Migration Report

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Migrate project with 2 deprecated APIs and 1 file change | Report contains 1 change, 2 deprecations | R-15.15.2 |

### TC-15.15.2.5 Migration Graph API

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Project with deprecated node "OldDelay", migration maps to "NewDelay" | Logic graph contains "NewDelay" with remapped parameters | R-15.15.2 |

### TC-15.15.3.1 Template Instantiate

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `instantiate("rpg_template", project_dir, "MyGame")` | Valid `.harmonius` file with correct modules for RPG template | R-15.15.3 |

### TC-15.15.3.2 Template Each Genre

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Instantiate `Rpg` template | Valid project created | R-15.15.3 |
| 2 | Instantiate `Fps` template | Valid project created | R-15.15.3 |
| 3 | Instantiate `Rts` template | Valid project created | R-15.15.3 |
| 4 | Instantiate `Platformer2d` template | Valid project created | R-15.15.3 |
| 5 | Instantiate `VrExperience` template | Valid project created | R-15.15.3 |
| 6 | Instantiate `Empty` template | Valid project created | R-15.15.3 |

### TC-15.15.5.1 Keychain Store Retrieve

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `store("harmonius", "github", secret_bytes)`, `retrieve("harmonius", "github")` | Returns `secret_bytes` | R-15.15.5 |

### TC-15.15.5.2 Keychain Delete

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Store credential, `delete("harmonius", "github")`, retrieve | Returns `Err(KeychainError::NotFound)` | R-15.15.5 |

### TC-15.15.5.3 Account Link OAuth

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `link(GitHub)` with mock OAuth server | Returns `LinkedAccount { provider: GitHub, is_active: true }` | R-15.15.5 |
| 2 | `link(GitLab)` with mock OAuth server | Returns `LinkedAccount { provider: GitLab, is_active: true }` | R-15.15.5 |

### TC-15.15.5.4 Account Switch

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Link accounts A and B, `switch(B)` | Account B is active, A is inactive | R-15.15.5 |

### TC-15.15.5.5 Preference Roundtrip

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `upload(prefs)`, `download()` | Downloaded prefs match uploaded prefs | R-15.15.5 |

### TC-15.15.6.1 Collab Wizard Validates

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `validate_connectivity("https://invalid.example.com")` | Returns `Err(ConnectivityFailed { url })` | R-15.15.6 |

### TC-15.15.6.2 Collab Team Config Saved

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Run wizard, write `CollabConfig` to `.harmonius` | `ProjectFile.team` contains collab server URL and members | R-15.15.6 |

## Integration Tests

### TC-15.15.1.I1 Install and Launch

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `install(2.3.0)`, launch editor binary | Editor process starts, returns exit code 0 | R-15.15.1 |

### TC-15.15.1.I2 Side by Side Versions

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Install v1.0 and v2.0, launch v1.0 then v2.0 | Both versions start independently, no shared state | R-15.15.1 |

### TC-15.15.1.I3 Auto Update macOS

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mock Sparkle appcast feed with newer version | Launcher downloads and applies update | R-15.15.1 |

### TC-15.15.1.I4 Auto Update Windows

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mock WinSparkle appcast feed with newer version | Launcher downloads and applies update | R-15.15.1 |

### TC-15.15.1.I5 Auto Update Linux

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mock AppImage update endpoint with newer version | AppImage delta applied, binary replaced | R-15.15.1 |

### TC-15.15.4.I1 File Association macOS

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `register()`, open `.harmonius` file | Launcher opens with project loaded | R-15.15.4 |

### TC-15.15.4.I2 File Association Windows

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `register()`, double-click `.harmonius` file | Launcher opens with project loaded | R-15.15.4 |

### TC-15.15.4.I3 File Association Linux

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `register()`, `xdg-open project.harmonius` | Launcher opens with project loaded | R-15.15.4 |

### TC-15.15.2.I1 Migration Full Pipeline

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Real project at v1.0, migrate to v1.3 | All assets re-cook, logic graphs migrated, project opens in v1.3 | R-15.15.2 |

### TC-15.15.5.I1 Preference Sync Two Machines

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Machine A uploads prefs, machine B downloads | Machine B preferences match machine A | R-15.15.5 |

### TC-15.15.5.I2 Keychain Per Platform

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Store and retrieve credential on current platform | Credential matches on macOS Keychain / Windows CredMgr / Linux libsecret | R-15.15.5 |

### TC-15.15.6.I1 Collab Wizard Full Flow

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Run wizard with mock Git, LFS, and collab server | `CollabConfig` written to `.harmonius`, all connections validated | R-15.15.6 |

### TC-15.15.6.I2 Collab Config Inherited

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Clone repo with `.harmonius` containing team config | Cloned project has correct `team` section | R-15.15.6 |

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
