# User Stories -- 14.7 Platform SDK Integration

## Steam (Steamworks)

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|-------------------|----------|-------------|
| US-14.7.1 | designer (P-5) | to map internal achievement IDs to Steam-specific identifiers in the editor's achievement configuration panel, so that achievements unlock on Steam without requiring code changes or Steamworks portal edits |  |  |  |
| US-14.7.2 | game developer (P-15) | to integrate Steam Workshop upload, download, and subscription through the engine's mod pipeline, so that modders can publish and players can install mods through a familiar storefront-native experience |  |  |  |
| US-14.7.3 | modder (P-24) | to package and upload my mod to Steam Workshop with a title, description, tags, and preview images, so that other players can discover, subscribe, and download my content directly through Steam |  |  |  |
| US-14.7.4 | player (P-23) | subscribed Steam Workshop mods to auto-download and activate when I launch the game, so that I can browse and install mods through the Steam client without manual file management |  |  |  |
| US-14.7.5 | player (P-23) | achievements to unlock on my Steam profile when I reach milestones in-game, so that my accomplishments are visible to my Steam friends and tracked on my profile |  |  |  |
| US-14.7.6 | player (P-23) | to create and join multiplayer lobbies through Steam matchmaking with filters for game mode and region, so that I can find games with friends or suitable strangers without leaving the Steam ecosystem |  |  |  |
| US-14.7.7 | DevOps engineer (P-16) | to configure Steam-specific build packaging (depots, branches, launch options) through the engine's build pipeline, so that Steam builds are automated and consistent without manual Steamworks portal intervention |  |  |  |
| US-14.7.8 | server administrator (P-22) | to enable and monitor VAC enforcement on dedicated game servers, so that VAC-banned players are automatically rejected and server integrity is maintained |  |  |  |

## Apple (iOS/macOS)

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|-------------------|----------|-------------|
| US-14.7.9 | game developer (P-15) | to configure StoreKit 2 product definitions (consumables, non-consumables, subscriptions) in the engine's IAP configuration, so that purchases work on iOS and macOS through the unified purchase abstraction without platform-specific code |  |  |  |
| US-14.7.10 | player (P-23) | to purchase premium currency through the App Store with a single tap and have it credited to my account immediately after validation, so that I can buy cosmetic items without leaving the game |  |  |  |
| US-14.7.11 | player (P-23) | achievements to unlock in Game Center when I complete objectives, so that my accomplishments appear on my Apple gaming profile and are visible to friends |  |  |  |
| US-14.7.12 | QA tester (P-19) | to verify that the ATT prompt appears before any IDFA access and that denying tracking disables all behavioral advertising, so that the app passes App Review without rejection for tracking violations |  |  |  |

## Google Play

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|-------------------|----------|-------------|
| US-14.7.13 | game developer (P-15) | to configure Google Play Billing Library 7 product definitions in the engine's IAP configuration, so that consumable and subscription purchases work on Android through the unified purchase abstraction |  |  |  |
| US-14.7.14 | player (P-23) | to purchase in-game items and subscriptions through the Google Play Store with my linked payment method, so that I can buy content seamlessly on my Android device |  |  |  |
| US-14.7.15 | player (P-23) | achievements to unlock in Google Play Games Services when I reach milestones, so that my progress is tracked and visible in the GPGS overlay and player profile |  |  |  |
| US-14.7.16 | server administrator (P-22) | to receive and validate Play Integrity tokens from Android clients on the game server, so that rooted devices and modified APKs are detected and handled according to our anti-cheat policy |  |  |  |

## Microsoft (Xbox/Windows)

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|-------------------|----------|-------------|
| US-14.7.17 | designer (P-5) | to define Xbox achievement unlock rules based on stat events in Partner Center, with the engine automatically writing the correct events, so that achievements unlock through Xbox Live's server-side evaluation without client-side logic |  |  |  |
| US-14.7.18 | player (P-23) | to see my ranking on Xbox Live leaderboards among friends and globally, so that I can compare my performance with other players on the Xbox ecosystem |  |  |  |
| US-14.7.19 | player (P-23) | to access the game through my Xbox Game Pass subscription with Game Pass-specific benefits, so that I can play without a separate purchase and enjoy member perks |  |  |  |
| US-14.7.20 | DevOps engineer (P-16) | the build pipeline to produce Xbox-ready packages that meet XR certification requirements, so that builds pass certification without manual intervention or last-minute fixes |  |  |  |
| US-14.7.21 | player (P-23) | voice chat to work automatically with my Xbox party through Game Chat 2 with accessibility features (text-to-speech, speech-to-text), so that I can communicate with teammates regardless of hearing ability |  |  |  |

## PlayStation

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|-------------------|----------|-------------|
| US-14.7.22 | player (P-23) | trophies to unlock on my PSN profile when I achieve milestones, including automatic platinum trophy when all base-game trophies are earned, so that my accomplishments are recognized on PlayStation |  |  |  |
| US-14.7.23 | player (P-23) | to buy DLC and virtual currency through the PlayStation Store overlay with wallet funding support, so that I can purchase content without leaving the game |  |  |  |
| US-14.7.24 | designer (P-5) | to define trophy configurations (bronze, silver, gold, platinum mapping) that the engine maps to internal achievement IDs, so that trophy packs meet TRC R4060 without manual trophy tool edits |  |  |  |
| US-14.7.25 | QA tester (P-19) | automated certification pre-checks that verify TRC compliance (trophies, save data metadata, activities, age rating), so that certification-blocking issues are caught before Sony submission |  |  |  |
| US-14.7.26 | player (P-23) | to launch directly into a specific quest or multiplayer session from a PlayStation Activity card on the PS5 home screen, so that I can resume playing without navigating through menus |  |  |  |

## Nintendo Switch

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|-------------------|----------|-------------|
| US-14.7.27 | player (P-23) | the game to verify my Nintendo Switch Online membership and allow online play when active, so that I can participate in multiplayer with a clear prompt if my membership has expired |  |  |  |
| US-14.7.28 | player (P-23) | to buy in-game items through the Nintendo eShop overlay, so that I can purchase content on my Switch using my Nintendo account funds |  |  |  |
| US-14.7.29 | QA tester (P-19) | automated pre-checks that verify Lotcheck requirements (NSO gating, purchase flow UX, save data compliance), so that certification issues are caught before Nintendo submission |  |  |  |

## Epic Online Services (EOS)

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|-------------------|----------|-------------|
| US-14.7.30 | game developer (P-15) | to configure EOS achievement definitions in the Epic Developer Portal and have the engine sync them through the platform abstraction layer, so that achievements work cross-platform on all EOS-supported storefronts |  |  |  |
| US-14.7.31 | player (P-23) | voice chat to work seamlessly in cross-platform lobbies through EOS Voice with mute and volume controls, so that I can communicate with players on other platforms without extra setup |  |  |  |
| US-14.7.32 | server administrator (P-22) | to monitor Easy Anti-Cheat status on game servers and receive notifications when EAC kicks or bans a player, so that I have visibility into anti-cheat enforcement across the player base |  |  |  |
| US-14.7.33 | modder (P-24) | to browse, upload, and download mods through the EOS ecosystem when the game is distributed on the Epic Games Store, so that I have the same modding experience regardless of storefront |  |  |  |

## Cross-Platform

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|-------------------|----------|-------------|
| US-14.7.34 | game developer (P-15) | a unified Rust trait API for achievements, leaderboards, IAP, matchmaking, cloud save, and voice that dispatches to the correct platform SDK backend automatically, so that I write game logic once without per-platform branches |  |  |  |
| US-14.7.35 | player (P-23) | my achievements, save data, and purchases to sync across all platforms where I have linked accounts, so that switching from Steam to Xbox does not lose my progress |  |  |  |
| US-14.7.36 | DevOps engineer (P-16) | the engine to detect the active storefront (Steam, Epic, Microsoft Store) at runtime and load the correct SDK backend, so that I ship a single PC binary instead of maintaining separate builds per storefront |  |  |  |
| US-14.7.37 | player (P-23) | the game to queue my achievements, scores, and save data when the platform service is offline and sync them automatically when it reconnects, so that I never lose progress due to a service outage |  |  |  |
| US-14.7.38 | server administrator (P-22) | a dashboard showing the health and status of all platform SDK integrations (Steam, PSN, Xbox Live, Apple, Google Play, Nintendo, EOS), so that I can detect and respond to platform service outages affecting players |  |  |  |
| US-14.7.39 | server administrator (P-22) | the server-side receipt validation pipeline to handle all platform receipt formats (Apple JWS, Google token, Steam MicroTxn, Microsoft collections, PSN, eShop, EOS), so that purchase fraud is prevented uniformly regardless of storefront |  |  |  |
| US-14.7.40 | QA tester (P-19) | automated test suites that exercise each platform SDK integration (achievement unlock, leaderboard submit, purchase flow, cloud save) on every target platform, so that platform regressions are caught before certification submission |  |  |  |
| US-14.7.41 | player (P-23) | to purchase premium currency and cosmetic items through my platform's native store (Steam, App Store, Play Store, Microsoft Store, PlayStation Store, eShop), so that I use my preferred payment method on every device |  |  |  |
| US-14.7.42 | player (P-23) | matchmaking to work through my platform's native system (Steam lobbies, Xbox SmartMatch, PSN matchmaking, Game Center, GPGS, EOS lobbies, NSO), so that I find games using the matchmaking experience native to my platform |  |  |  |
