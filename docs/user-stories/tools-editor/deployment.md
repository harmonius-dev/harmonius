# User Stories: Build and Deployment

## F-15.14.1 Platform Build Packaging

| ID           | Persona                | Features | Requirements |
|--------------|------------------------|----------|--------------|
| US-15.14.1.1 | DevOps engineer (P-16) |          |              |
| US-15.14.1.2 | developer (P-15)       |          |              |
| US-15.14.1.3 | designer (P-5)         |          |              |
| US-15.14.1.4 | engine tester (P-27)   |          |              |

1. **US-15.14.1.1** — to package the game into platform-native formats (macOS .app, Windows .exe,
   Linux AppImage, iOS IPA, Android APK/AAB, console packages)
   - **Acceptance:** builds are distributable on every target platform
2. **US-15.14.1.2** — build configurations (debug, development, shipping) controlling optimization
   level, symbol stripping, and diagnostic inclusion
   - **Acceptance:** I can choose the appropriate build for my current task
3. **US-15.14.1.3** — to invoke packaging from the editor UI or CLI
   - **Acceptance:** I can create builds without learning platform-specific packaging tools
4. **US-15.14.1.4** — to verify that editor-only content is stripped from shipping builds
   - **Acceptance:** debug tools and test assets do not increase the shipped binary size

## F-15.14.2 Deploy-to-Device Workflow

| ID           | Persona                 | Features | Requirements |
|--------------|-------------------------|----------|--------------|
| US-15.14.2.1 | developer (P-15)        |          |              |
| US-15.14.2.2 | designer (P-5)          |          |              |
| US-15.14.2.3 | engine developer (P-26) |          |              |
| US-15.14.2.4 | engine tester (P-27)    |          |              |

1. **US-15.14.2.1** — one-click deployment to connected iOS, Android, and console development
   devices with incremental asset transfer
   - **Acceptance:** I can test changes on target hardware in seconds
2. **US-15.14.2.2** — a device manager panel listing connected devices with status, storage, and OS
   version
   - **Acceptance:** I can choose which device to deploy to
3. **US-15.14.2.3** — console output from the remote device to stream back to the editor's console
   panel
   - **Acceptance:** I can read logs and debug output without connecting a separate debugger
4. **US-15.14.2.4** — to verify that incremental transfer sends only changed assets
   - **Acceptance:** iteration time is proportional to the size of the change rather than the full
     project

## F-15.14.3 Certification Compliance Checker

| ID           | Persona                | Features | Requirements |
|--------------|------------------------|----------|--------------|
| US-15.14.3.1 | DevOps engineer (P-16) |          |              |
| US-15.14.3.2 | designer (P-5)         |          |              |
| US-15.14.3.3 | server admin (P-22)    |          |              |
| US-15.14.3.4 | engine tester (P-27)   |          |              |

1. **US-15.14.3.1** — automated validation of platform certification requirements (trophy icons,
   button glyphs, save data compliance, accessibility, content rating, network handling, performance
   baselines)
   - **Acceptance:** certification blockers are caught before submission
2. **US-15.14.3.2** — the checker to produce a pass/fail report per requirement with remediation
   guidance
   - **Acceptance:** I can fix issues without reading the full certification documentation
3. **US-15.14.3.3** — each platform's certification checklist maintained as a data asset updatable
   independently of the engine
   - **Acceptance:** new certification rules are incorporated without waiting for an engine release
4. **US-15.14.3.4** — to verify that the compliance checker catches all known certification failures
   for each platform
   - **Acceptance:** the automated check is a reliable gate before submission

## F-15.14.4 Code Signing Pipeline

| ID           | Persona                | Features | Requirements |
|--------------|------------------------|----------|--------------|
| US-15.14.4.1 | DevOps engineer (P-16) |          |              |
| US-15.14.4.2 | developer (P-15)       |          |              |
| US-15.14.4.3 | server admin (P-22)    |          |              |
| US-15.14.4.4 | engine tester (P-27)   |          |              |

1. **US-15.14.4.1** — automated code signing for all platforms (iOS provisioning, macOS
   notarization, Android keystore, Windows Authenticode) as part of the packaging pipeline
   - **Acceptance:** I do not manually sign each artifact
2. **US-15.14.4.2** — signing credentials stored in the platform keychain and never committed to
   version control
   - **Acceptance:** credentials are secure
3. **US-15.14.4.3** — CI/CD pipelines to access signing credentials through environment variables or
   secure vault integration
   - **Acceptance:** automated signing works on build servers
4. **US-15.14.4.4** — to verify that macOS notarization submission and ticket stapling work
   end-to-end
   - **Acceptance:** macOS users can open the game without Gatekeeper warnings

## F-15.14.5 Platform-Specific Installers and Distributions

| ID           | Persona                | Features | Requirements |
|--------------|------------------------|----------|--------------|
| US-15.14.5.1 | DevOps engineer (P-16) |          |              |
| US-15.14.5.2 | developer (P-15)       |          |              |
| US-15.14.5.3 | designer (P-5)         |          |              |
| US-15.14.5.4 | engine tester (P-27)   |          |              |

1. **US-15.14.5.1** — to generate .dmg (macOS), .msi (Windows), AppImage (Linux), .deb packages,
   Flatpak manifests, and Steam Deck-verified builds
   - **Acceptance:** each platform gets its native installer format
2. **US-15.14.5.2** — .msi installers with optional silent install mode, start menu shortcuts, and
   file associations
   - **Acceptance:** enterprise deployment is automated
3. **US-15.14.5.3** — macOS .dmg disk images with background art and Applications symlink
   - **Acceptance:** the install experience matches macOS conventions
4. **US-15.14.5.4** — to verify that Steam Deck-verified builds pass Valve's compatibility testing
   with appropriate controller configuration
   - **Acceptance:** the game receives the Steam Deck Verified badge

## F-15.14.6 Asset Bundle and DLC Packaging

| ID           | Persona                | Features | Requirements |
|--------------|------------------------|----------|--------------|
| US-15.14.6.1 | developer (P-15)       |          |              |
| US-15.14.6.2 | designer (P-5)         |          |              |
| US-15.14.6.3 | DevOps engineer (P-16) |          |              |
| US-15.14.6.4 | engine tester (P-27)   |          |              |

1. **US-15.14.6.1** — to package subsets of game content into signed, compressed asset bundles with
   metadata, dependencies, and content hashes
   - **Acceptance:** DLC and content updates are distributed separately from the base game
2. **US-15.14.6.2** — patch bundles containing only changed assets since a base version
   - **Acceptance:** incremental updates are small and fast for players to download
3. **US-15.14.6.3** — bundle integrity verified via BLAKE3 content hashes on load
   - **Acceptance:** corrupted downloads are detected before they cause runtime errors
4. **US-15.14.6.4** — to verify that DLC packs check platform entitlements before mounting
   - **Acceptance:** only purchased DLC is accessible to each player

## F-15.14.7 Delta Patching System

| ID           | Persona                | Features | Requirements |
|--------------|------------------------|----------|--------------|
| US-15.14.7.1 | DevOps engineer (P-16) |          |              |
| US-15.14.7.2 | developer (P-15)       |          |              |
| US-15.14.7.3 | server admin (P-22)    |          |              |
| US-15.14.7.4 | engine tester (P-27)   |          |              |

1. **US-15.14.7.1** — binary delta patches generated between game versions using content-defined
   chunking
   - **Acceptance:** update download sizes are typically 5-20% of the full update
2. **US-15.14.7.2** — patch verification to ensure the patched result matches the full new build via
   content hash comparison
   - **Acceptance:** delta patching is provably correct
3. **US-15.14.7.3** — the delta patching system to work alongside platform- native patching (Steam,
   Xbox, PlayStation)
   - **Acceptance:** self-distributed builds also benefit from small updates
4. **US-15.14.7.4** — to verify that content-defined chunking produces shift-resilient diffs
   - **Acceptance:** minor asset changes do not inflate patch sizes

## F-15.14.8 Store Distribution Pipeline

| ID           | Persona                | Features | Requirements |
|--------------|------------------------|----------|--------------|
| US-15.14.8.1 | DevOps engineer (P-16) |          |              |
| US-15.14.8.2 | developer (P-15)       |          |              |
| US-15.14.8.3 | server admin (P-22)    |          |              |
| US-15.14.8.4 | engine tester (P-27)   |          |              |

1. **US-15.14.8.1** — automated submission of builds to Steam (SteamCMD), App Store (Transporter),
   Windows Store (MSIX), and Xbox (Partner Center)
   - **Acceptance:** store releases are part of the CI/CD pipeline
2. **US-15.14.8.2** — to set live branches on Steam (default, beta, staging) with metadata
   management
   - **Acceptance:** I can control which builds players access
3. **US-15.14.8.3** — submission status polling that notifies the team when builds pass or fail
   store review
   - **Acceptance:** the team is informed without manually checking each storefront
4. **US-15.14.8.4** — configurable pre-submission validation gates that run certification checks
   before uploading
   - **Acceptance:** builds failing certification are never submitted to stores

## F-15.14.9 Host-Target Build Matrix Documentation

| ID           | Persona                 | Features             | Requirements |
|--------------|-------------------------|----------------------|--------------|
| US-15.14.9.1 | developer (P-15)        | F-15.14.9, F-15.24.6 | R-15.14.9    |
| US-15.14.9.2 | DevOps engineer (P-16)  | F-15.14.9, F-15.24.2 | R-15.14.9    |
| US-15.14.9.3 | engine developer (P-26) | F-15.14.9            | R-15.14.9    |
| US-15.14.9.4 | developer (P-15)        | F-15.14.9, F-15.24.3 | R-15.14.9    |
| US-15.14.9.5 | DevOps engineer (P-16)  | F-15.14.9, F-14.8.1  | R-15.14.9    |
| US-15.14.9.6 | engine developer (P-26) | F-15.14.9, F-12.2.9  | R-15.14.9    |

1. **US-15.14.9.1** — to know exactly which platforms I can build locally on my Linux machine vs.
   which require the cloud build service
   - **Acceptance:** I do not waste time attempting unsupported local cross-compilation
2. **US-15.14.9.2** — to know which cloud worker types can build which target platforms so I can
   configure CI/CD pipelines correctly
   - **Acceptance:** every CI job runs on a worker capable of producing artifacts for its target
     platform
3. **US-15.14.9.3** — to know what toolchains I must install locally to work on each GPU backend
   (Metal, Vulkan, D3D12)
   - **Acceptance:** I set up my development environment correctly without trial and error
4. **US-15.14.9.4** — to know that Metal shader compilation requires macOS or the cloud service
   - **Acceptance:** I understand why my Windows machine cannot compile Metal shaders locally and
     use the cloud instead
5. **US-15.14.9.5** — to know that console builds require dedicated licensed workers
   - **Acceptance:** I provision the correct infrastructure for console CI/CD pipelines
6. **US-15.14.9.6** — to see a single reference table of all shader compilation host restrictions
   - **Acceptance:** I know which bytecode formats I can produce on my development machine
