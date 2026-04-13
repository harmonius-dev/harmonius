# User Stories -- 15.14 Build and Deployment

## Stories

| ID           | Persona                    |
|--------------|----------------------------|
| US-15.14.1.1 | build engineer (P-16)      |
| US-15.14.1.2 | engine developer (P-26)    |
| US-15.14.2.1 | game designer (P-5)        |
| US-15.14.2.2 | build engineer (P-16)      |
| US-15.14.3.1 | build engineer (P-16)      |
| US-15.14.3.2 | engine developer (P-26)    |
| US-15.14.4.1 | build engineer (P-16)      |
| US-15.14.4.2 | engine developer (P-26)    |
| US-15.14.5.1 | build engineer (P-16)      |
| US-15.14.5.2 | game designer (P-5)        |
| US-15.14.6.1 | build engineer (P-16)      |
| US-15.14.6.2 | game designer (P-5)        |
| US-15.14.7.1 | build engineer (P-16)      |
| US-15.14.7.2 | engine developer (P-26)    |
| US-15.14.8.1 | build engineer (P-16)      |
| US-15.14.8.2 | game designer (P-5)        |
| US-15.14.9.1 | build engineer (P-16)      |
| US-15.14.9.2 | engine developer (P-26)    |
| US-15.14.10.1 | build engineer (P-16)     |
| US-15.14.11.1 | engine developer (P-26)   |

1. **US-15.14.1.1** — **As a** build engineer (P-16), **I want** packaging into platform-native
   formats (macOS .app, Windows .exe, Linux AppImage, iOS IPA, Android APK/AAB), **so that** builds
   are distributable on every target.

2. **US-15.14.1.2** — **As a** engine developer (P-26), **I want** build configurations (debug,
   development, shipping) to control optimization, symbols, and diagnostics, **so that** each stage
   gets the appropriate build.

3. **US-15.14.2.1** — **As a** game designer (P-5), **I want** one-click deploy to connected devices
   with incremental asset transfer, **so that** I can test on target hardware instantly.

4. **US-15.14.2.2** — **As a** build engineer (P-16), **I want** a device manager panel listing
   connected devices with status and remote console, **so that** I manage test devices from the
   editor.

5. **US-15.14.3.1** — **As a** build engineer (P-16), **I want** automated certification compliance
   checking per platform, **so that** submission failures are caught before submission.

6. **US-15.14.3.2** — **As a** engine developer (P-26), **I want** certification rules maintained as
   updatable data assets, **so that** platform changes are applied without engine updates.

7. **US-15.14.4.1** — **As a** build engineer (P-16), **I want** automated code signing for all
   platform artifacts, **so that** distribution builds are signed without manual steps.

8. **US-15.14.4.2** — **As a** engine developer (P-26), **I want** signing credentials stored in the
   platform keychain and never committed to version control, **so that** keys are secure.

9. **US-15.14.5.1** — **As a** build engineer (P-16), **I want** platform-native installer
   generation (.dmg, .msi, .deb, AppImage, Flatpak), **so that** end users install the game through
   familiar mechanisms.

10. **US-15.14.5.2** — **As a** game designer (P-5), **I want** Steam Deck verified builds with
    controller configuration, **so that** the game works out-of-the-box on SteamOS.

11. **US-15.14.6.1** — **As a** build engineer (P-16), **I want** asset bundle and DLC packaging
    with entitlement gating and content hash verification, **so that** downloadable content is
    secure and modular.

12. **US-15.14.6.2** — **As a** game designer (P-5), **I want** patch bundles containing only
    changed assets, **so that** updates are small and fast to download.

13. **US-15.14.7.1** — **As a** build engineer (P-16), **I want** delta patching between game
    versions at the chunk level, **so that** update sizes are 5-20% of full downloads.

14. **US-15.14.7.2** — **As a** engine developer (P-26), **I want** patch verification via content
    hash comparison, **so that** patched results exactly match full builds.

15. **US-15.14.8.1** — **As a** build engineer (P-16), **I want** automated store submission to
    Steam, App Store, and Windows Store with pre-submission validation, **so that** submission is
    part of CI.

16. **US-15.14.8.2** — **As a** game designer (P-5), **I want** submission status polling with team
    notification, **so that** I know when builds pass or fail store review.

17. **US-15.14.9.1** — **As a** build engineer (P-16), **I want** documented host-to-target build
    matrices for local and cloud builds, **so that** I know which platforms build where.

18. **US-15.14.9.2** — **As a** engine developer (P-26), **I want** the cloud build service to
    handle all targets from any host OS, **so that** no developer needs every toolchain locally.

19. **US-15.14.10.1** — **As a** build engineer (P-16), **I want** to trigger platform builds via a
    CLI command, **so that** I can integrate engine builds into my CI/CD pipeline.

20. **US-15.14.11.1** — **As a** engine developer (P-26), **I want** all compiled shaders prefetched
    from the shared cache on first launch after clone, **so that** I avoid a multi-hour initial
    shader compilation.

## Parent Stories

The 3-segment parent stories below are umbrella rollups for the refined 4-segment sub-stories listed
above. Each parent inherits the persona of its first sub-story and describes the umbrella capability
that the sub-stories refine.

| ID | Persona |
|----|---------|
| US-15.14.1 | build engineer (P-16) |
| US-15.14.10 | build engineer (P-16) |
| US-15.14.11 | engine developer (P-26) |
| US-15.14.2 | game designer (P-5) |
| US-15.14.3 | build engineer (P-16) |
| US-15.14.4 | build engineer (P-16) |
| US-15.14.5 | build engineer (P-16) |
| US-15.14.6 | build engineer (P-16) |
| US-15.14.7 | build engineer (P-16) |
| US-15.14.8 | build engineer (P-16) |
| US-15.14.9 | build engineer (P-16) |

1. **US-15.14.1** -- **As a** build engineer (P-16), **I want** the capabilities defined in
   sub-stories US-15.14.1.1 through US-15.14.1.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

2. **US-15.14.10** -- **As a** build engineer (P-16), **I want** the capabilities defined in
   sub-stories US-15.14.10.1 through US-15.14.10.1 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

3. **US-15.14.11** -- **As a** engine developer (P-26), **I want** the capabilities defined in
   sub-stories US-15.14.11.1 through US-15.14.11.1 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

4. **US-15.14.2** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-15.14.2.1 through US-15.14.2.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

5. **US-15.14.3** -- **As a** build engineer (P-16), **I want** the capabilities defined in
   sub-stories US-15.14.3.1 through US-15.14.3.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

6. **US-15.14.4** -- **As a** build engineer (P-16), **I want** the capabilities defined in
   sub-stories US-15.14.4.1 through US-15.14.4.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

7. **US-15.14.5** -- **As a** build engineer (P-16), **I want** the capabilities defined in
   sub-stories US-15.14.5.1 through US-15.14.5.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

8. **US-15.14.6** -- **As a** build engineer (P-16), **I want** the capabilities defined in
   sub-stories US-15.14.6.1 through US-15.14.6.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

9. **US-15.14.7** -- **As a** build engineer (P-16), **I want** the capabilities defined in
   sub-stories US-15.14.7.1 through US-15.14.7.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

10. **US-15.14.8** -- **As a** build engineer (P-16), **I want** the capabilities defined in
    sub-stories
US-15.14.8.1 through US-15.14.8.2 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

11. **US-15.14.9** -- **As a** build engineer (P-16), **I want** the capabilities defined in
    sub-stories
US-15.14.9.1 through US-15.14.9.2 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.
