# User Stories: Build and Deployment

## F-15.14.1 Platform Build Packaging

## US-15.14.1.1 DevOps Packages for All Platforms
**As a** DevOps engineer (P-16), **I want** to package the game into platform-native formats
(macOS .app, Windows .exe, Linux AppImage, iOS IPA, Android APK/AAB, console packages), **so
that** builds are distributable on every target platform.

## US-15.14.1.2 Developer Selects Build Configuration
**As a** developer (P-15), **I want** build configurations (debug, development, shipping)
controlling optimization level, symbol stripping, and diagnostic inclusion, **so that** I can
choose the appropriate build for my current task.

## US-15.14.1.3 Designer Packages from Editor UI
**As a** designer (P-5), **I want** to invoke packaging from the editor UI or CLI, **so that**
I can create builds without learning platform-specific packaging tools.

## US-15.14.1.4 Engine Tester Validates Asset Stripping
**As an** engine tester (P-27), **I want** to verify that editor-only content is stripped from
shipping builds, **so that** debug tools and test assets do not increase the shipped binary
size.

## F-15.14.2 Deploy-to-Device Workflow

## US-15.14.2.1 Developer Deploys with One Click
**As a** developer (P-15), **I want** one-click deployment to connected iOS, Android, and
console development devices with incremental asset transfer, **so that** I can test changes
on target hardware in seconds.

## US-15.14.2.2 Designer Sees Device Manager
**As a** designer (P-5), **I want** a device manager panel listing connected devices with
status, storage, and OS version, **so that** I can choose which device to deploy to.

## US-15.14.2.3 Engine Dev Streams Console Output
**As an** engine developer (P-26), **I want** console output from the remote device to stream
back to the editor's console panel, **so that** I can read logs and debug output without
connecting a separate debugger.

## US-15.14.2.4 Engine Tester Validates Incremental Transfer
**As an** engine tester (P-27), **I want** to verify that incremental transfer sends only
changed assets, **so that** iteration time is proportional to the size of the change rather
than the full project.

## F-15.14.3 Certification Compliance Checker

## US-15.14.3.1 DevOps Runs Pre-Submission Checks
**As a** DevOps engineer (P-16), **I want** automated validation of platform certification
requirements (trophy icons, button glyphs, save data compliance, accessibility, content
rating, network handling, performance baselines), **so that** certification blockers are
caught before submission.

## US-15.14.3.2 Designer Reviews Compliance Report
**As a** designer (P-5), **I want** the checker to produce a pass/fail report per requirement
with remediation guidance, **so that** I can fix issues without reading the full certification
documentation.

## US-15.14.3.3 Server Admin Updates Checklists
**As a** server admin (P-22), **I want** each platform's certification checklist maintained as
a data asset updatable independently of the engine, **so that** new certification rules are
incorporated without waiting for an engine release.

## US-15.14.3.4 Engine Tester Validates Checker Accuracy
**As an** engine tester (P-27), **I want** to verify that the compliance checker catches all
known certification failures for each platform, **so that** the automated check is a reliable
gate before submission.

## F-15.14.4 Code Signing Pipeline

## US-15.14.4.1 DevOps Automates Signing
**As a** DevOps engineer (P-16), **I want** automated code signing for all platforms (iOS
provisioning, macOS notarization, Android keystore, Windows Authenticode) as part of the
packaging pipeline, **so that** I do not manually sign each artifact.

## US-15.14.4.2 Developer Stores Credentials Securely
**As a** developer (P-15), **I want** signing credentials stored in the platform keychain and
never committed to version control, **so that** credentials are secure.

## US-15.14.4.3 Server Admin Accesses via Vault
**As a** server admin (P-22), **I want** CI/CD pipelines to access signing credentials through
environment variables or secure vault integration, **so that** automated signing works on build
servers.

## US-15.14.4.4 Engine Tester Validates Notarization
**As an** engine tester (P-27), **I want** to verify that macOS notarization submission and
ticket stapling work end-to-end, **so that** macOS users can open the game without Gatekeeper
warnings.

## F-15.14.5 Platform-Specific Installers and Distributions

## US-15.14.5.1 DevOps Generates Platform Installers
**As a** DevOps engineer (P-16), **I want** to generate .dmg (macOS), .msi (Windows), AppImage
(Linux), .deb packages, Flatpak manifests, and Steam Deck-verified builds, **so that** each
platform gets its native installer format.

## US-15.14.5.2 Developer Creates Silent Installer
**As a** developer (P-15), **I want** .msi installers with optional silent install mode, start
menu shortcuts, and file associations, **so that** enterprise deployment is automated.

## US-15.14.5.3 Designer Builds Drag-to-Install DMG
**As a** designer (P-5), **I want** macOS .dmg disk images with background art and
Applications symlink, **so that** the install experience matches macOS conventions.

## US-15.14.5.4 Engine Tester Validates SteamOS Build
**As an** engine tester (P-27), **I want** to verify that Steam Deck-verified builds pass
Valve's compatibility testing with appropriate controller configuration, **so that** the game
receives the Steam Deck Verified badge.

## F-15.14.6 Asset Bundle and DLC Packaging

## US-15.14.6.1 Developer Packages DLC Bundles
**As a** developer (P-15), **I want** to package subsets of game content into signed, compressed
asset bundles with metadata, dependencies, and content hashes, **so that** DLC and content
updates are distributed separately from the base game.

## US-15.14.6.2 Designer Creates Patch Bundles
**As a** designer (P-5), **I want** patch bundles containing only changed assets since a base
version, **so that** incremental updates are small and fast for players to download.

## US-15.14.6.3 DevOps Verifies Bundle Integrity
**As a** DevOps engineer (P-16), **I want** bundle integrity verified via BLAKE3 content hashes
on load, **so that** corrupted downloads are detected before they cause runtime errors.

## US-15.14.6.4 Engine Tester Validates Entitlement Gating
**As an** engine tester (P-27), **I want** to verify that DLC packs check platform entitlements
before mounting, **so that** only purchased DLC is accessible to each player.

## F-15.14.7 Delta Patching System

## US-15.14.7.1 DevOps Generates Delta Patches
**As a** DevOps engineer (P-16), **I want** binary delta patches generated between game
versions using content-defined chunking, **so that** update download sizes are typically
5-20% of the full update.

## US-15.14.7.2 Developer Verifies Patch Results
**As a** developer (P-15), **I want** patch verification to ensure the patched result matches
the full new build via content hash comparison, **so that** delta patching is provably correct.

## US-15.14.7.3 Server Admin Integrates with Distribution
**As a** server admin (P-22), **I want** the delta patching system to work alongside platform-
native patching (Steam, Xbox, PlayStation), **so that** self-distributed builds also benefit
from small updates.

## US-15.14.7.4 Engine Tester Validates Shift Resilience
**As an** engine tester (P-27), **I want** to verify that content-defined chunking produces
shift-resilient diffs, **so that** minor asset changes do not inflate patch sizes.

## F-15.14.8 Store Distribution Pipeline

## US-15.14.8.1 DevOps Submits to Storefronts
**As a** DevOps engineer (P-16), **I want** automated submission of builds to Steam (SteamCMD),
App Store (Transporter), Windows Store (MSIX), and Xbox (Partner Center), **so that** store
releases are part of the CI/CD pipeline.

## US-15.14.8.2 Developer Manages Build Branches
**As a** developer (P-15), **I want** to set live branches on Steam (default, beta, staging)
with metadata management, **so that** I can control which builds players access.

## US-15.14.8.3 Server Admin Polls Submission Status
**As a** server admin (P-22), **I want** submission status polling that notifies the team when
builds pass or fail store review, **so that** the team is informed without manually checking
each storefront.

## US-15.14.8.4 Engine Tester Validates Pre-Submission Gates
**As an** engine tester (P-27), **I want** configurable pre-submission validation gates that
run certification checks before uploading, **so that** builds failing certification are never
submitted to stores.
