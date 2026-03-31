# User Stories -- 14.7 Platform SDK Integration

## Steam (Steamworks)

| ID        | Persona                     |
|-----------|-----------------------------|
| US-14.7.1 | designer (P-5)              |
| US-14.7.2 | game developer (P-15)       |
| US-14.7.3 | modder (P-24)               |
| US-14.7.4 | player (P-23)               |
| US-14.7.5 | player (P-23)               |
| US-14.7.6 | player (P-23)               |
| US-14.7.7 | DevOps engineer (P-16)      |
| US-14.7.8 | server administrator (P-22) |

1. **US-14.7.1** — **As a** designer (P-5), **I want** to map internal achievement IDs to Steam
   identifiers in the editor, **so that** achievements unlock on Steam without code changes or
   portal edits.
2. **US-14.7.2** — **As a** game developer (P-15), **I want** to integrate Steam Workshop upload,
   download, and subscription through the mod pipeline, **so that** modders can publish and players
   can install mods through Steam.
3. **US-14.7.3** — **As a** modder (P-24), **I want** to package and upload my mod to Steam Workshop
   with a title, description, tags, and preview images, **so that** other players can discover and
   download my content.
4. **US-14.7.4** — **As a** player (P-23), **I want** subscribed Steam Workshop mods to
   auto-download and activate on launch, **so that** I can install mods through the Steam client
   without manual file management.
5. **US-14.7.5** — **As a** player (P-23), **I want** achievements to unlock on my Steam profile
   when I reach milestones, **so that** my accomplishments are visible to friends.
6. **US-14.7.6** — **As a** player (P-23), **I want** to create and join lobbies through Steam
   matchmaking with filters for game mode and region, **so that** I can find games without leaving
   the Steam ecosystem.
7. **US-14.7.7** — **As a** DevOps engineer (P-16), **I want** Steam-specific build packaging
   automated through the build pipeline, **so that** Steam builds are consistent without manual
   portal intervention.
8. **US-14.7.8** — **As a** server administrator (P-22), **I want** to enable and monitor VAC
   enforcement on game servers, **so that** VAC-banned players are rejected automatically.

## Apple (iOS/macOS)

| ID         | Persona               |
|------------|-----------------------|
| US-14.7.9  | game developer (P-15) |
| US-14.7.10 | player (P-23)         |
| US-14.7.11 | player (P-23)         |
| US-14.7.12 | QA tester (P-19)      |

1. **US-14.7.9** — **As a** game developer (P-15), **I want** to configure StoreKit 2 product
   definitions in the engine's IAP configuration, **so that** purchases work on iOS and macOS
   through the unified purchase abstraction.
2. **US-14.7.10** — **As a** player (P-23), **I want** to purchase premium currency through the App
   Store with a single tap and have it credited immediately after validation, **so that** I can buy
   items without leaving the game.
3. **US-14.7.11** — **As a** player (P-23), **I want** achievements to unlock in Game Center when I
   complete objectives, **so that** my accomplishments appear on my Apple gaming profile.
4. **US-14.7.12** — **As a** QA tester (P-19), **I want** to verify that the ATT prompt appears
   before any IDFA access and that denying tracking disables all IDFA-dependent analytics,
   **so that** the app passes App Review.

## Google Play

| ID         | Persona                     |
|------------|-----------------------------|
| US-14.7.13 | game developer (P-15)       |
| US-14.7.14 | player (P-23)               |
| US-14.7.15 | player (P-23)               |
| US-14.7.16 | server administrator (P-22) |

1. **US-14.7.13** — **As a** game developer (P-15), **I want** to configure Play Billing Library 7
   products in the engine's IAP configuration, **so that** purchases work on Android through the
   unified purchase abstraction.
2. **US-14.7.14** — **As a** player (P-23), **I want** to purchase items and subscriptions through
   the Google Play Store with my linked payment method, **so that** I can buy content on my Android
   device.
3. **US-14.7.15** — **As a** player (P-23), **I want** achievements to unlock in Google Play Games
   Services when I reach milestones, **so that** my progress is visible in the GPGS overlay.
4. **US-14.7.16** — **As a** server administrator (P-22), **I want** to receive and validate Play
   Integrity tokens from Android clients, **so that** modified APKs are detected per our anti-cheat
   policy.

## Microsoft (Xbox/Windows)

| ID         | Persona                |
|------------|------------------------|
| US-14.7.17 | designer (P-5)         |
| US-14.7.18 | player (P-23)          |
| US-14.7.19 | player (P-23)          |
| US-14.7.20 | player (P-23)          |
| US-14.7.21 | DevOps engineer (P-16) |

1. **US-14.7.17** — **As a** designer (P-5), **I want** to define Xbox achievement unlock rules
   based on stat events, **so that** achievements unlock through Xbox Live server-side evaluation.
2. **US-14.7.18** — **As a** player (P-23), **I want** to see my ranking on Xbox Live leaderboards
   among friends and globally, **so that** I can compare performance with other Xbox players.
3. **US-14.7.19** — **As a** player (P-23), **I want** to access the game through my Xbox Game Pass
   subscription with member benefits, **so that** I can play without a separate purchase.
4. **US-14.7.20** — **As a** player (P-23), **I want** voice chat to work with my Xbox party through
   Game Chat 2 with text-to-speech and speech-to-text, **so that** I can communicate regardless of
   hearing ability.
5. **US-14.7.21** — **As a** DevOps engineer (P-16), **I want** the build pipeline to produce
   Xbox-ready packages meeting XR certification, **so that** builds pass certification without
   manual fixes.

## PlayStation

| ID         | Persona          |
|------------|------------------|
| US-14.7.22 | player (P-23)    |
| US-14.7.23 | player (P-23)    |
| US-14.7.24 | player (P-23)    |
| US-14.7.25 | designer (P-5)   |
| US-14.7.26 | QA tester (P-19) |

1. **US-14.7.22** — **As a** player (P-23), **I want** trophies to unlock on my PSN profile when I
   achieve milestones including automatic platinum, **so that** my accomplishments are recognized on
   PlayStation.
2. **US-14.7.23** — **As a** player (P-23), **I want** to buy DLC and virtual currency through the
   PlayStation Store overlay, **so that** I can purchase content without leaving the game.
3. **US-14.7.24** — **As a** player (P-23), **I want** to launch directly into a quest or
   multiplayer session from a PlayStation Activity card, **so that** I resume playing without
   navigating menus.
4. **US-14.7.25** — **As a** designer (P-5), **I want** to define trophy configurations that the
   engine maps to internal achievement IDs, **so that** trophy packs meet TRC R4060 without manual
   edits.
5. **US-14.7.26** — **As a** QA tester (P-19), **I want** automated certification pre-checks for TRC
   compliance, **so that** certification-blocking issues are caught before Sony submission.

## Nintendo Switch

| ID         | Persona          |
|------------|------------------|
| US-14.7.27 | player (P-23)    |
| US-14.7.28 | player (P-23)    |
| US-14.7.29 | QA tester (P-19) |

1. **US-14.7.27** — **As a** player (P-23), **I want** the game to verify my NSO membership and
   allow online play when active, **so that** I get a clear prompt if my membership has expired.
2. **US-14.7.28** — **As a** player (P-23), **I want** to buy in-game items through the eShop
   overlay, **so that** I can purchase on Switch using my Nintendo account funds.
3. **US-14.7.29** — **As a** QA tester (P-19), **I want** automated pre-checks for Lotcheck
   requirements, **so that** certification issues are caught before Nintendo submission.

## Epic Online Services (EOS)

| ID         | Persona                     |
|------------|-----------------------------|
| US-14.7.30 | game developer (P-15)       |
| US-14.7.31 | player (P-23)               |
| US-14.7.32 | server administrator (P-22) |

1. **US-14.7.30** — **As a** game developer (P-15), **I want** to configure EOS achievement
   definitions and have the engine sync them through the platform abstraction layer, **so that**
   achievements work cross-platform on EOS storefronts.
2. **US-14.7.31** — **As a** player (P-23), **I want** voice chat to work in cross-platform lobbies
   through EOS Voice with mute and volume controls, **so that** I can communicate with players on
   other platforms.
3. **US-14.7.32** — **As a** server administrator (P-22), **I want** to monitor Easy Anti-Cheat
   status and receive notifications on kicks or bans, **so that** I have visibility into anti-cheat
   enforcement.

## Cross-Platform

| ID         | Persona                     |
|------------|-----------------------------|
| US-14.7.33 | game developer (P-15)       |
| US-14.7.34 | player (P-23)               |
| US-14.7.35 | player (P-23)               |
| US-14.7.36 | player (P-23)               |
| US-14.7.37 | DevOps engineer (P-16)      |
| US-14.7.38 | server administrator (P-22) |
| US-14.7.39 | QA tester (P-19)            |

1. **US-14.7.33** — **As a** game developer (P-15), **I want** a unified Rust trait API for
   achievements, leaderboards, IAP, matchmaking, cloud save, and voice that dispatches to the
   correct backend, **so that** I write game logic once without per-platform branches.
2. **US-14.7.34** — **As a** player (P-23), **I want** my achievements, save data, and purchases to
   sync across platforms where I have linked accounts, **so that** switching platforms does not lose
   my progress.
3. **US-14.7.35** — **As a** player (P-23), **I want** the game to queue achievements, scores, and
   saves when the platform is offline and sync automatically on reconnect, **so that** I never lose
   progress due to a service outage.
4. **US-14.7.36** — **As a** player (P-23), **I want** to purchase premium currency through my
   platform's native store, **so that** I use my preferred payment method on every device.
5. **US-14.7.37** — **As a** DevOps engineer (P-16), **I want** the engine to detect the active
   storefront at runtime and load the correct SDK backend, **so that** I ship a single PC binary
   instead of maintaining separate builds per storefront.
6. **US-14.7.38** — **As a** server administrator (P-22), **I want** server-side receipt validation
   handling all platform receipt formats, **so that** purchase fraud is prevented uniformly.
7. **US-14.7.39** — **As a** QA tester (P-19), **I want** automated test suites exercising each
   platform SDK integration on every target platform, **so that** regressions are caught before
   certification submission.

## Server-Side Proprietary SDK Isolation

| ID         | Persona                     |
|------------|-----------------------------|
| US-14.8.1  | game developer (P-15)       |
| US-14.8.2  | game developer (P-15)       |
| US-14.8.3  | game developer (P-15)       |
| US-14.8.4  | DevOps engineer (P-16)      |
| US-14.8.5  | DevOps engineer (P-16)      |
| US-14.8.6  | server administrator (P-22) |
| US-14.8.7  | executive (P-1)             |
| US-14.8.8  | engine developer (P-26)     |
| US-14.8.9  | engine developer (P-26)     |
| US-14.8.10 | QA tester (P-19)            |
| US-14.8.11 | QA tester (P-19)            |

1. **US-14.8.1** — **As a** game developer (P-15), **I want** to trigger a PS5 build from the editor
   without owning a PlayStation SDK license, **so that** I can develop and test console builds using
   only the shared build server.
2. **US-14.8.2** — **As a** game developer (P-15), **I want** to trigger an Xbox build from the
   editor without owning an Xbox GDK license, **so that** I can iterate on console features without
   per-developer licensing.
3. **US-14.8.3** — **As a** game developer (P-15), **I want** to trigger a Nintendo Switch build
   from the editor without a local Nintendo SDK, **so that** I can test Switch builds through the
   shared server.
4. **US-14.8.4** — **As a** DevOps engineer (P-16), **I want** to set up a shared console build
   server with a single set of SDK licenses, **so that** all teams build for PS5, Xbox, and Switch
   without per-developer licenses.
5. **US-14.8.5** — **As a** DevOps engineer (P-16), **I want** per-project isolation on the shared
   build server, **so that** one team's source code and artifacts are not accessible to another
   team.
6. **US-14.8.6** — **As a** server administrator (P-22), **I want** a priority-based build queue for
   multiple teams, **so that** urgent certification builds take precedence over routine builds.
7. **US-14.8.7** — **As an** executive (P-1), **I want** the entire studio to share one console SDK
   license per platform on the build server, **so that** console development costs are minimized.
8. **US-14.8.8** — **As an** engine developer (P-26), **I want** to implement abstract platform
   traits without needing console SDK access on my machine, **so that** I can contribute as an
   open-source developer.
9. **US-14.8.9** — **As an** engine developer (P-26), **I want** the engine to compile with zero
   proprietary references from the open-source repository, **so that** anyone can build and
   contribute without NDA dependencies.
10. **US-14.8.10** — **As a** QA tester (P-19), **I want** to download a console build and deploy it
    to a connected dev kit from the editor, **so that** I can test on console hardware without
    proprietary SDK tools on my workstation.
11. **US-14.8.11** — **As a** QA tester (P-19), **I want** console output to stream from the dev kit
    to my editor in real time via the build server, **so that** I can debug console issues without
    console SDK debugging tools.
