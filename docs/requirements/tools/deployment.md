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

10. **R-15.14.10** — The engine **SHALL** compile codegen'd Rust source into a middleman .dylib for
    development hot-reload and a statically linked binary with LTO for shipping builds, using
    platform-specific linkers (MSVC link.exe, ld64, lld, platform SDK linker).
    - **Rationale:** Hot-reload accelerates iteration; LTO with static linking optimizes shipping
      builds.
    - **Verification:** Modify a codegen'd type, trigger hot-reload, and verify the change takes
      effect without restart. Build a shipping binary and verify LTO is applied.

11. **R-15.14.11** — The engine **SHALL** compile HLSL shaders offline via DXC and
    metal-shaderconverter CLI to DXIL, SPIR-V, and MSL IR with parallel variant compilation via the
    job system and per-platform variant caching keyed by BLAKE3(source + flags + tool version +
    platform).
    - **Rationale:** Offline shader compilation eliminates runtime stalls and enables per-platform
      optimization.
    - **Verification:** Compile a shader with 4 variants across 3 platforms and verify all 12
      outputs are cached and valid.

12. **R-15.14.12** — The engine **SHALL** package baked assets in rkyv-archived bundle files that
    are mmap-readable at runtime without deserialization, with BLAKE3 content integrity.
    - **Rationale:** Zero-copy mmap access eliminates deserialization cost at load time.
    - **Verification:** Bake an asset bundle, mmap it, and verify access without deserialization.
      Corrupt a byte and verify integrity check fails.

13. **R-15.14.13** — The engine **SHALL** ship baked assets as separate bundle files alongside the
    executable, reserving include_bytes! for inline data under 64 KB.
    - **Rationale:** Large assets in binaries inflate executable size and waste memory.
    - **Verification:** Build a shipping package and verify no asset over 64 KB is embedded in the
      binary.

14. **R-15.14.14** — The engine **SHALL** bake platform-specific asset variants (BC7/ASTC textures,
    DXIL/SPIR-V/MSL shaders, Opus/AAC/ADPCM audio, full/reduced meshlets) via parallel per-asset
    cooking using the job system.
    - **Rationale:** Per-platform variants ensure optimal format on each target.
    - **Verification:** Bake a texture for Windows (BC7) and iOS (ASTC) and verify both are valid
      for their platform.

15. **R-15.14.15** — The engine **SHALL** support SteamOS as a build target with Steam Deck verified
    packaging and controller-first UI validation.
    - **Rationale:** SteamOS is a distinct platform with controller-first constraints.
    - **Verification:** Build for SteamOS, run on Steam Deck, and verify controller navigation works
      without mouse input.
