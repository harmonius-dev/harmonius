# R-15.14 -- Build and Deployment Requirements

## Packaging

### R-15.14.1 Platform Build Packaging

The editor **SHALL** package the game for macOS, Windows, Linux, iOS, Android, and consoles with
debug, development, and shipping build configurations, stripping editor-only content from shipping
builds, and cooking assets for the target platform. Packaging **SHALL** be invocable from the editor
UI or CLI.

- **Derived from:**
  [F-15.14.1](../../features/tools-editor/deployment.md)
- **Rationale:** Cross-platform distribution requires platform-native packaging with build
  configuration control.
- **Verification:** Integration test: package a shipping build and verify editor-only content is
  excluded.

### R-15.14.2 Deploy-to-Device Workflow

The editor **SHALL** support one-click deployment to connected devices with incremental asset
transfer for changed files only, a device manager panel, remote launch with configurable arguments,
and console output streaming from remote devices.

- **Derived from:**
  [F-15.14.2](../../features/tools-editor/deployment.md)
- **Rationale:** Fast device iteration requires incremental transfer and integrated device
  management.
- **Verification:** Unit test: deploy with one changed asset and verify only that asset is
  transferred.

### R-15.14.3 Certification Compliance Checker

The editor **SHALL** provide automated platform certification checks with pass/fail reports and
remediation guidance, using updatable platform checklists maintained as data assets.

- **Derived from:**
  [F-15.14.3](../../features/tools-editor/deployment.md)
- **Rationale:** Automated certification prevents submission failures and reduces manual review.
- **Verification:** Unit test: introduce a known certification violation and verify it is reported
  with remediation guidance.

### R-15.14.4 Code Signing Pipeline

The editor **SHALL** automate code signing for iOS (Ad Hoc and App Store profiles), macOS
(notarization with ticket stapling), Android (APK/AAB with release keystores), and Windows
(Authenticode via signtool), with signing credentials stored in platform keychain only.

- **Derived from:**
  [F-15.14.4](../../features/tools-editor/deployment.md)
- **Rationale:** Automated signing eliminates manual steps and ensures credentials never leave
  secure storage.
- **Verification:** Integration test: sign artifacts for each platform and verify they pass platform
  verification tools.

### R-15.14.5 Platform-Specific Installers

The editor **SHALL** generate macOS .dmg with drag-to-install, Windows .msi with silent install and
file associations, Linux AppImage and .deb with apt metadata, and SteamOS-verified builds with
controller configuration.

- **Derived from:**
  [F-15.14.5](../../features/tools-editor/deployment.md)
- **Rationale:** Platform-native installers provide the expected user experience per platform.
- **Verification:** Integration test: verify each installer type installs and uninstalls without
  errors.

## Asset Bundles and DLC

### R-15.14.6 Asset Bundle and DLC Packaging

The editor **SHALL** package content subsets as signed asset bundles with platform entitlement
gating, patch bundles containing only changed assets, and BLAKE3 hash integrity verification on
load.

- **Derived from:**
  [F-15.14.6](../../features/tools-editor/deployment.md)
- **Rationale:** DLC and seasonal content require separate distribution with integrity and
  entitlement control.
- **Verification:** Unit test: verify a DLC bundle loads only with valid entitlement and fails with
  invalid.

### R-15.14.7 Delta Patching System

The editor **SHALL** generate binary delta patches between game versions using content-defined
chunking for shift-resilient diffs, with post-patch hash verification, as a CI pipeline step.

- **Derived from:**
  [F-15.14.7](../../features/tools-editor/deployment.md)
- **Rationale:** Minimal patch sizes reduce download time and bandwidth costs.
- **Verification:** Benchmark: verify patch size is under 25% of full update for typical changes.

## Store Distribution

### R-15.14.8 Store Distribution Pipeline

The editor **SHALL** support CLI-invocable submission to Steam (SteamCMD), App Store (Transporter),
and Windows Store (MSIX + Partner Center), with pre-submission certification validation and status
polling with team notification.

- **Derived from:**
  [F-15.14.8](../../features/tools-editor/deployment.md)
- **Rationale:** Automated store submission reduces manual steps and catch failures early.
- **Verification:** Integration test: verify submission to each store's staging/sandbox environment.

---

## User Story Traceability

User stories for this domain are maintained in
[user-stories/tools-editor/deployment.md](../../user-stories/tools-editor/deployment.md).
Requirements in this document are derived from those
user stories.
