# R-15.14 -- Build and Deployment Requirements

## Requirements

1. **R-15.14.1** — The engine **SHALL** package projects into platform-native distributable formats
   for macOS, Windows, Linux, iOS, Android, and consoles with configurable build configurations.
   - **Rationale:** Platform-native packaging is required for distribution on every target platform.
   - **Verification:** Package a project for each platform and verify each runs correctly.

2. **R-15.14.2** — The engine **SHALL** support one-click deploy-to-device with incremental asset
   transfer, device manager panel, and remote console output.
   - **Rationale:** Fast deploy-to-device accelerates testing on target hardware.
   - **Verification:** Deploy to an iOS device via USB, modify an asset, redeploy, and verify only
     the changed asset transfers.

3. **R-15.14.3** — The engine **SHALL** validate certification compliance per platform using
   updatable rule data assets, producing a pass/fail report with remediation guidance.
   - **Rationale:** Pre-submission validation prevents costly certification failures.
   - **Verification:** Introduce a certification violation and verify the checker reports it with a
     fix suggestion.

4. **R-15.14.4** — The engine **SHALL** automate code signing for all platforms with credentials
   stored in the platform keychain, never in version control.
   - **Rationale:** Automated signing prevents manual errors in distribution builds.
   - **Verification:** Sign a macOS build and verify notarization completes successfully.

5. **R-15.14.5** — The engine **SHALL** generate platform-native installers (.dmg, .msi, .deb,
   AppImage, Flatpak) with Steam Deck verified build support.
   - **Rationale:** Familiar installation mechanisms improve end-user experience.
   - **Verification:** Generate a .dmg on macOS, install, and verify the application launches.

6. **R-15.14.6** — The engine **SHALL** support asset bundle and DLC packaging with entitlement
   gating, patch bundles, and BLAKE3 content hash verification.
   - **Rationale:** Modular content packaging enables DLC and incremental updates.
   - **Verification:** Package a DLC bundle, load it at runtime with valid entitlements, and verify
     content hash integrity.

7. **R-15.14.7** — The engine **SHALL** generate delta patches between game versions using
   content-defined chunking with hash verification, producing patches at 5-20% of full size.
   - **Rationale:** Small patches reduce download time and bandwidth costs for updates.
   - **Verification:** Generate a patch, apply it, and verify the result matches the full new build
     by hash.

8. **R-15.14.8** — The engine **SHALL** automate store submission to Steam, App Store, and Windows
   Store with pre-submission validation and status polling with team notification.
   - **Rationale:** Automated submission integrates distribution into CI/CD pipelines.
   - **Verification:** Submit a build to a store staging environment and verify status polling
     reports success.

9. **R-15.14.9** — The engine **SHALL** document host-to-target build matrices for local and cloud
   builds, with the cloud service covering all targets from any host OS.
   - **Rationale:** Clear documentation prevents build configuration errors.
   - **Verification:** Verify the matrix documentation matches actual build capabilities by
     triggering each combination.
