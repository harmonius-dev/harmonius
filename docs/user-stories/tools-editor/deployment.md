# User Stories: Build and Deployment

## F-15.14.1 Platform Build Packaging

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.14.1.1 | DevOps engineer (P-16) | to package the game into platform-native formats (macOS .app, Windows .exe, Linux AppImage, iOS IPA, Android APK/AAB, console packages) | builds are distributable on every target platform |  |  |
| US-15.14.1.2 | developer (P-15) | build configurations (debug, development, shipping) controlling optimization level, symbol stripping, and diagnostic inclusion | I can choose the appropriate build for my current task |  |  |
| US-15.14.1.3 | designer (P-5) | to invoke packaging from the editor UI or CLI | I can create builds without learning platform-specific packaging tools |  |  |
| US-15.14.1.4 | engine tester (P-27) | to verify that editor-only content is stripped from shipping builds | debug tools and test assets do not increase the shipped binary size |  |  |

## F-15.14.2 Deploy-to-Device Workflow

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.14.2.1 | developer (P-15) | one-click deployment to connected iOS, Android, and console development devices with incremental asset transfer | I can test changes on target hardware in seconds |  |  |
| US-15.14.2.2 | designer (P-5) | a device manager panel listing connected devices with status, storage, and OS version | I can choose which device to deploy to |  |  |
| US-15.14.2.3 | engine developer (P-26) | console output from the remote device to stream back to the editor's console panel | I can read logs and debug output without connecting a separate debugger |  |  |
| US-15.14.2.4 | engine tester (P-27) | to verify that incremental transfer sends only changed assets | iteration time is proportional to the size of the change rather than the full project |  |  |

## F-15.14.3 Certification Compliance Checker

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.14.3.1 | DevOps engineer (P-16) | automated validation of platform certification requirements (trophy icons, button glyphs, save data compliance, accessibility, content rating, network handling, performance baselines) | certification blockers are caught before submission |  |  |
| US-15.14.3.2 | designer (P-5) | the checker to produce a pass/fail report per requirement with remediation guidance | I can fix issues without reading the full certification documentation |  |  |
| US-15.14.3.3 | server admin (P-22) | each platform's certification checklist maintained as a data asset updatable independently of the engine | new certification rules are incorporated without waiting for an engine release |  |  |
| US-15.14.3.4 | engine tester (P-27) | to verify that the compliance checker catches all known certification failures for each platform | the automated check is a reliable gate before submission |  |  |

## F-15.14.4 Code Signing Pipeline

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.14.4.1 | DevOps engineer (P-16) | automated code signing for all platforms (iOS provisioning, macOS notarization, Android keystore, Windows Authenticode) as part of the packaging pipeline | I do not manually sign each artifact |  |  |
| US-15.14.4.2 | developer (P-15) | signing credentials stored in the platform keychain and never committed to version control | credentials are secure |  |  |
| US-15.14.4.3 | server admin (P-22) | CI/CD pipelines to access signing credentials through environment variables or secure vault integration | automated signing works on build servers |  |  |
| US-15.14.4.4 | engine tester (P-27) | to verify that macOS notarization submission and ticket stapling work end-to-end | macOS users can open the game without Gatekeeper warnings |  |  |

## F-15.14.5 Platform-Specific Installers and Distributions

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.14.5.1 | DevOps engineer (P-16) | to generate .dmg (macOS), .msi (Windows), AppImage (Linux), .deb packages, Flatpak manifests, and Steam Deck-verified builds | each platform gets its native installer format |  |  |
| US-15.14.5.2 | developer (P-15) | .msi installers with optional silent install mode, start menu shortcuts, and file associations | enterprise deployment is automated |  |  |
| US-15.14.5.3 | designer (P-5) | macOS .dmg disk images with background art and Applications symlink | the install experience matches macOS conventions |  |  |
| US-15.14.5.4 | engine tester (P-27) | to verify that Steam Deck-verified builds pass Valve's compatibility testing with appropriate controller configuration | the game receives the Steam Deck Verified badge |  |  |

## F-15.14.6 Asset Bundle and DLC Packaging

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.14.6.1 | developer (P-15) | to package subsets of game content into signed, compressed asset bundles with metadata, dependencies, and content hashes | DLC and content updates are distributed separately from the base game |  |  |
| US-15.14.6.2 | designer (P-5) | patch bundles containing only changed assets since a base version | incremental updates are small and fast for players to download |  |  |
| US-15.14.6.3 | DevOps engineer (P-16) | bundle integrity verified via BLAKE3 content hashes on load | corrupted downloads are detected before they cause runtime errors |  |  |
| US-15.14.6.4 | engine tester (P-27) | to verify that DLC packs check platform entitlements before mounting | only purchased DLC is accessible to each player |  |  |

## F-15.14.7 Delta Patching System

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.14.7.1 | DevOps engineer (P-16) | binary delta patches generated between game versions using content-defined chunking | update download sizes are typically 5-20% of the full update |  |  |
| US-15.14.7.2 | developer (P-15) | patch verification to ensure the patched result matches the full new build via content hash comparison | delta patching is provably correct |  |  |
| US-15.14.7.3 | server admin (P-22) | the delta patching system to work alongside platform- native patching (Steam, Xbox, PlayStation) | self-distributed builds also benefit from small updates |  |  |
| US-15.14.7.4 | engine tester (P-27) | to verify that content-defined chunking produces shift-resilient diffs | minor asset changes do not inflate patch sizes |  |  |

## F-15.14.8 Store Distribution Pipeline

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.14.8.1 | DevOps engineer (P-16) | automated submission of builds to Steam (SteamCMD), App Store (Transporter), Windows Store (MSIX), and Xbox (Partner Center) | store releases are part of the CI/CD pipeline |  |  |
| US-15.14.8.2 | developer (P-15) | to set live branches on Steam (default, beta, staging) with metadata management | I can control which builds players access |  |  |
| US-15.14.8.3 | server admin (P-22) | submission status polling that notifies the team when builds pass or fail store review | the team is informed without manually checking each storefront |  |  |
| US-15.14.8.4 | engine tester (P-27) | configurable pre-submission validation gates that run certification checks before uploading | builds failing certification are never submitted to stores |  |  |

## F-15.14.9 Host-Target Build Matrix Documentation

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.14.9.1 | developer (P-15) | to know exactly which platforms I can build locally on my Linux machine vs. which require the cloud build service | I do not waste time attempting unsupported local cross-compilation | F-15.14.9, F-15.24.6 | R-15.14.9 |
| US-15.14.9.2 | DevOps engineer (P-16) | to know which cloud worker types can build which target platforms so I can configure CI/CD pipelines correctly | every CI job runs on a worker capable of producing artifacts for its target platform | F-15.14.9, F-15.24.2 | R-15.14.9 |
| US-15.14.9.3 | engine developer (P-26) | to know what toolchains I must install locally to work on each GPU backend (Metal, Vulkan, D3D12) | I set up my development environment correctly without trial and error | F-15.14.9 | R-15.14.9 |
| US-15.14.9.4 | developer (P-15) | to know that Metal shader compilation requires macOS or the cloud service | I understand why my Windows machine cannot compile Metal shaders locally and use the cloud instead | F-15.14.9, F-15.24.3 | R-15.14.9 |
| US-15.14.9.5 | DevOps engineer (P-16) | to know that console builds require dedicated licensed workers | I provision the correct infrastructure for console CI/CD pipelines | F-15.14.9, F-14.8.1 | R-15.14.9 |
| US-15.14.9.6 | engine developer (P-26) | to see a single reference table of all shader compilation host restrictions | I know which bytecode formats I can produce on my development machine | F-15.14.9, F-12.2.9 | R-15.14.9 |
