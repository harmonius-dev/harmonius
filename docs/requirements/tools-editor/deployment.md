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

## User Stories

## US-15.14.1 Platform Build Packaging

### US-15.14.1.1

As a **developer (P-15)**, I want to package the game for macOS, Windows, Linux, iOS, Android, and
consoles so that I produce platform-native distributables from a single project.

### US-15.14.1.2

As a **developer (P-15)**, I want debug, development, and shipping build configurations so that I
control optimization level and symbol inclusion per build.

### US-15.14.1.3

As a **developer (P-15)**, I want packaging invocable from the editor UI or CLI so that builds can
be triggered manually or via CI.

### US-15.14.1.4

As a **DevOps (P-16)**, I want editor-only content stripped from shipping builds so that development
tools are not exposed to players.

### US-15.14.1.5

As a **DevOps (P-16)**, I want assets cooked for the target platform during packaging so that
textures and shaders are in platform-optimal format.

### US-15.14.1.6

As an **engine tester (P-27)**, I want to verify shipping builds exclude editor-only content so that
build content filtering is regression-tested.

---

## US-15.14.2 Deploy-to-Device Workflow

### US-15.14.2.1

As a **developer (P-15)**, I want one-click deployment to connected devices so that I can test on
real hardware without manual file transfer.

### US-15.14.2.2

As a **developer (P-15)**, I want incremental asset transfer for changed files only so that
iteration time is minimized.

### US-15.14.2.3

As a **developer (P-15)**, I want a device manager panel showing connected devices so that I can see
device status and storage at a glance.

### US-15.14.2.4

As a **developer (P-15)**, I want remote launch with configurable command-line arguments so that I
can start the game with specific options on the device.

### US-15.14.2.5

As a **developer (P-15)**, I want console output from remote devices streamed to the editor so that
I can debug device issues from my workstation.

### US-15.14.2.6

As an **engine tester (P-27)**, I want to verify incremental transfer sends only changed assets so
that transfer efficiency is regression-tested.

---

## US-15.14.3 Certification Compliance Checker

### US-15.14.3.1

As a **developer (P-15)**, I want automated platform certification checks so that compliance is
validated before store submission.

### US-15.14.3.2

As a **developer (P-15)**, I want pass/fail reports with remediation guidance so that I know exactly
what to fix for certification.

### US-15.14.3.3

As a **DevOps (P-16)**, I want platform checklists maintained as updatable data assets so that
certification rules update without engine updates.

### US-15.14.3.4

As a **designer (P-5)**, I want button glyph correctness validation so that UI glyphs match the
target platform's requirements.

### US-15.14.3.5

As an **engine tester (P-27)**, I want to verify known certification violations are reported with
pass/fail and remediation so that checker accuracy is regression-tested.

---

## US-15.14.4 Code Signing Pipeline

### US-15.14.4.1

As a **DevOps (P-16)**, I want automated code signing as part of the packaging pipeline so that
signed builds are produced without manual steps.

### US-15.14.4.2

As a **DevOps (P-16)**, I want iOS signing with Ad Hoc and App Store profiles so that both testing
and distribution builds are signed correctly.

### US-15.14.4.3

As a **DevOps (P-16)**, I want macOS notarization submission and ticket stapling so that macOS
builds pass Gatekeeper without warnings.

### US-15.14.4.4

As a **DevOps (P-16)**, I want Android APK/AAB signing with release keystores so that Play Store
submissions have valid signatures.

### US-15.14.4.5

As a **DevOps (P-16)**, I want Windows Authenticode signing via signtool so that Windows executables
are trusted by SmartScreen.

### US-15.14.4.6

As a **DevOps (P-16)**, I want signing credentials stored in platform keychain only so that
credentials are never committed to version control.

### US-15.14.4.7

As an **engine tester (P-27)**, I want to verify signed artifacts pass platform verification tools
so that signing correctness is regression-tested per platform.

---

## US-15.14.5 Platform-Specific Installers

### US-15.14.5.1

As a **DevOps (P-16)**, I want macOS .dmg with drag-to-install layout so that macOS users get the
standard installation experience.

### US-15.14.5.2

As a **DevOps (P-16)**, I want Windows .msi with silent install and file associations so that
Windows deployment is automated and complete.

### US-15.14.5.3

As a **DevOps (P-16)**, I want Linux AppImage and .deb with apt repository metadata so that Linux
users can install via their preferred method.

### US-15.14.5.4

As a **DevOps (P-16)**, I want SteamOS-verified builds with controller configuration so that Steam
Deck deployment meets Valve's requirements.

### US-15.14.5.5

As an **engine tester (P-27)**, I want to verify each installer type installs and uninstalls without
errors so that installer correctness is regression-tested per platform.

---

## US-15.14.6 Asset Bundle and DLC Packaging

### US-15.14.6.1

As a **developer (P-15)**, I want to package content subsets as signed asset bundles so that DLC and
seasonal content are distributed separately.

### US-15.14.6.2

As a **developer (P-15)**, I want DLC bundles gated by platform entitlements so that paid content
requires purchase verification.

### US-15.14.6.3

As a **developer (P-15)**, I want patch bundles containing only changed assets so that updates are
small incremental downloads.

### US-15.14.6.4

As a **developer (P-15)**, I want bundle integrity verified via BLAKE3 hashes on load so that
corrupted bundles are detected before use.

### US-15.14.6.5

As an **engine tester (P-27)**, I want to verify a DLC bundle loads only with valid entitlement so
that entitlement gating is regression-tested.

---

## US-15.14.7 Delta Patching System

### US-15.14.7.1

As a **developer (P-15)**, I want binary delta patches between game versions so that update
downloads are minimized.

### US-15.14.7.2

As a **developer (P-15)**, I want content-defined chunking for shift-resilient diffs so that patch
sizes are consistently small.

### US-15.14.7.3

As a **developer (P-15)**, I want the patcher to verify patched results via hash comparison so that
post-patch integrity is guaranteed.

### US-15.14.7.4

As a **DevOps (P-16)**, I want patch generation as a CI pipeline step so that patches are produced
automatically for each release.

### US-15.14.7.5

As an **engine tester (P-27)**, I want to verify patch size is under 25% of full update for typical
changes so that patch efficiency is regression-tested.

---

## US-15.14.8 Store Distribution Pipeline

### US-15.14.8.1

As a **DevOps (P-16)**, I want CLI-invocable submission to Steam via SteamCMD so that Steam builds
are deployed from the CI pipeline.

### US-15.14.8.2

As a **DevOps (P-16)**, I want submission to App Store via Transporter CLI so that iOS/macOS builds
are submitted automatically.

### US-15.14.8.3

As a **DevOps (P-16)**, I want submission to Windows Store via MSIX + Partner Center CLI so that
Windows Store builds are automated.

### US-15.14.8.4

As a **DevOps (P-16)**, I want pre-submission certification validation so that submissions fail fast
before reaching store review.

### US-15.14.8.5

As a **DevOps (P-16)**, I want status polling with team notification on review completion so that
the team knows when builds pass or fail store review.

### US-15.14.8.6

As an **engine tester (P-27)**, I want to verify submission to each store's staging/sandbox
environment so that store submission integration is regression-tested.
