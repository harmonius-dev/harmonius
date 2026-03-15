# R-15.14 Build and Deployment

## R-15.14.1 Platform Build Packaging
The engine **SHALL** package the game project into platform-native distributable formats (macOS
.app, Windows .exe with installer, Linux AppImage, iOS IPA, Android APK/AAB, and console-specific
packages) by cooking assets for the target platform, stripping editor-only content, optimizing
binary size, and supporting debug/development/shipping build configurations, invocable from the
editor UI or CLI.
- **Derived from:** [F-15.14.1](../../features/tools-editor/deployment.md)
- **Rationale:** Automated platform-native packaging eliminates manual build steps and ensures
  consistent, reproducible distributable output across all target platforms.
- **Verification:** Package a project for each supported platform and confirm the output is a valid
  platform-native distributable; confirm editor-only content is absent from the shipping build;
  confirm debug, development, and shipping configurations produce different optimization levels and
  symbol inclusion; invoke packaging from CLI and confirm identical output to editor-initiated
  builds.

## R-15.14.2 Deploy-to-Device Workflow
The engine **SHALL** provide one-click deployment from the editor to connected development devices
(iOS via USB/WiFi, Android via ADB, console devkits via network), with incremental asset transfer,
a device manager panel showing connected devices with status and storage, remote launch with
configurable arguments, and console output streaming back to the editor.
- **Derived from:** [F-15.14.2](../../features/tools-editor/deployment.md)
- **Rationale:** Incremental deploy-to-device with remote console output minimizes iteration time
  during on-device testing, which is critical for platform-specific bug reproduction and performance
  profiling.
- **Verification:** Connect a development device; deploy a project and confirm the game launches on
  the device; modify an asset, redeploy, and confirm only the changed asset is transferred; confirm
  console output from the device appears in the editor's console panel; confirm the device manager
  displays correct device status and storage information.

## R-15.14.3 Certification Compliance Checker
The engine **SHALL** run automated validation against platform certification requirements before
submission, checking required UI elements, button glyph correctness, save data compliance,
accessibility requirements, content rating metadata, network connectivity handling, and performance
baselines, with each platform's checklist maintained as an independently updatable data asset and
results reported as pass/fail with remediation guidance.
- **Derived from:** [F-15.14.3](../../features/tools-editor/deployment.md)
- **Rationale:** Automated certification checks catch compliance failures early, avoiding costly
  submission rejections and resubmission delays from platform holders.
- **Verification:** Run the compliance checker against a project with known certification violations
  and confirm each violation is reported with pass/fail status and remediation guidance; fix the
  violations and re-run to confirm all checks pass; update a platform checklist data asset and
  confirm the checker uses the updated rules without an engine update.

## R-15.14.4 Code Signing Pipeline
The engine **SHALL** automatically sign all distributable artifacts with platform-appropriate
certificates as part of the packaging pipeline. iOS builds SHALL be signed with either Ad Hoc or App
Store distribution profiles. macOS .app bundles SHALL be signed with a Developer ID certificate and
submitted for Apple notarization. Android APK/AAB SHALL be signed with release keystores using APK
Signature Scheme v2+. Windows executables and installers SHALL be Authenticode-signed. Signing
credentials SHALL be stored in the platform keychain and SHALL NOT be committed to version control.
- **Derived from:** [F-15.14.4](../../features/tools-editor/deployment.md)
- **Rationale:** Unsigned binaries are rejected by modern OS gatekeepers and app stores. Automated
  signing prevents manual errors and enables CI/CD.
- **Verification:** Package a build for each platform; verify the signature is valid using platform
  verification tools (codesign --verify, apksigner verify, signtool verify).

## R-15.14.5 Platform-Specific Installers
The engine **SHALL** generate platform-native installer packages: macOS .dmg with drag-to-install
layout, Windows .msi with silent install support and file associations, Linux AppImage and .deb
packages with apt repository metadata, and SteamOS-verified builds. Each format SHALL be generated
as a post-packaging step invocable from CLI.
- **Derived from:** [F-15.14.5](../../features/tools-editor/deployment.md)
- **Rationale:** Platform-native installers provide the expected installation experience and are
  required for some distribution channels.
- **Verification:** Generate each installer type; verify installation and uninstallation completes
  without errors on the target platform.

## R-15.14.6 Asset Bundle and DLC Packaging
The engine **SHALL** package content subsets into signed, versioned asset bundles with content-hash
manifests. DLC bundles SHALL require entitlement verification (R-14.5.6) before mounting. Patch
bundles SHALL contain only changed assets relative to a specified base version. Bundle integrity
SHALL be verified via BLAKE3 hashes on load.
- **Derived from:** [F-15.14.6](../../features/tools-editor/deployment.md)
- **Rationale:** Separating content into bundles enables DLC monetization, incremental updates, and
  reduced initial download sizes.
- **Verification:** Create a DLC bundle; verify it loads only when the entitlement is present and is
  rejected when the entitlement is absent. Create a patch bundle; verify it contains only changed
  assets.

## R-15.14.7 Delta Patching
The engine **SHALL** generate binary delta patches between game versions using content-defined
chunking. Patch size SHALL be less than 25% of the full update size for typical content changes. The
runtime patcher SHALL apply patches and verify the result matches the full new build via BLAKE3 hash
comparison.
- **Derived from:** [F-15.14.7](../../features/tools-editor/deployment.md)
- **Rationale:** Delta patching reduces download sizes for updates, improving player experience and
  reducing bandwidth costs.
- **Verification:** Generate a delta patch between two builds with 10% content changes; verify patch
  size is under 25% of full update; apply patch and verify hash matches full build.

## Non-Functional Requirements

### R-15.14.NF1 Build and Packaging Performance

Incremental asset cooking **SHALL** process only changed assets, completing within 5 minutes for
projects with up to 100,000 assets when fewer than 1% of assets have changed. Full platform
packaging (cook + package + sign) **SHALL** complete within 2 hours for projects with up to 100,000
assets on a CI build server with 32 cores. Deploy-to-device incremental transfer **SHALL** complete
within 30 seconds when fewer than 100 assets have changed.

- **Derived from:** F-15.14.1 through F-15.14.8 (all deployment features).
- **Rationale:** Build and packaging time directly impacts iteration speed and CI pipeline
  throughput. Incremental cooking avoids reprocessing unchanged content.
- **Verification:** Modify 500 assets in a 100,000-asset project and run incremental cook; assert
  completion within 5 minutes. Run a full packaging pipeline on a 32-core CI server and assert
  completion within 2 hours. Change 50 assets and deploy to a connected device; assert transfer
  completes within 30 seconds.

## R-15.14.8 Store Distribution Pipeline
The engine **SHALL** provide CLI-invocable submission to Steam (via SteamCMD), App Store (via
Transporter/altool), Windows Store (via MSIX + Partner Center CLI), and Xbox (via Partner Center).
Each submission SHALL include pre-submission validation (R-15.14.3) and status polling with team
notification on review completion.
- **Derived from:** [F-15.14.8](../../features/tools-editor/deployment.md)
- **Rationale:** Automated store submission reduces manual error and integrates distribution into
  CI/CD pipelines.
- **Verification:** Submit a test build to each store's staging/sandbox environment; verify the
  submission is accepted and status polling reports the correct review state.
