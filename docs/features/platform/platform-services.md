# 14.5 — Platform Services

## Achievements and Progression

### F-14.5.1 Cross-Platform Achievement System

Unlock achievements through Steam, PlayStation Network, and Xbox Live APIs using a unified
abstraction. The engine maps internal achievement IDs to platform-specific identifiers and
handles deferred unlocks when the platform service is temporarily unavailable (offline play,
network hiccup). MMO milestone achievements (first raid clear, max level) must unlock reliably
across all storefronts.

- **Requirements:** R-14.5.1
- **Dependencies:** None
- **Platform notes:** Steam uses `ISteamUserStats::SetAchievement` and `StoreStats`; PlayStation
  uses `sceNpTrophyUnlockTrophy`; Xbox uses `XblAchievementsUpdateAchievementAsync`. Each
  platform requires an initial sync to fetch current unlock state before the engine can detect
  already-earned achievements. Platinum/100% meta-achievements are platform-specific and handled
  by the platform SDK automatically.

### F-14.5.2 Leaderboards

Submit and query ranked scores through platform leaderboard APIs. Supports global, friends-only,
and around-player queries. Used for MMO features like DPS rankings, PvP ladders, speedrun
timers, and seasonal competitions. Score submission is batched and retried on transient failures.

- **Requirements:** R-14.5.2
- **Dependencies:** None
- **Platform notes:** Steam uses `ISteamUserStats::FindOrCreateLeaderboard` and
  `UploadLeaderboardScore`; PlayStation uses `sceNpScoreCreateRequest`; Xbox uses
  `XblLeaderboardGetLeaderboardAsync`. Console platforms impose rate limits on leaderboard
  queries — the engine must cache results and throttle requests.

## Social

### F-14.5.3 Rich Presence

Update the player's platform status with contextual game information (current zone, party size,
activity type) visible to friends on their platform's social UI. Rich presence drives organic
discovery and re-engagement for the MMO by showing friends what content others are playing.

- **Requirements:** R-14.5.3
- **Dependencies:** None
- **Platform notes:** Steam uses `ISteamFriends::SetRichPresence` with localization tokens;
  PlayStation uses `sceNpReachabilityCreateRequest`; Xbox uses
  `XblPresenceSetPresenceAsync` with rich presence strings. Update frequency should be
  throttled to avoid API rate limits (typically once per 15-30 seconds).

### F-14.5.4 Platform Voice and Party Integration

Bridge the engine's voice chat with platform party and voice systems so that players in a
platform party hear each other in-game without manual channel setup. Supports muting, volume
control, and voice activity indicators sourced from the platform's voice stream.

- **Requirements:** R-14.5.4
- **Dependencies:** None
- **Platform notes:** Steam uses Steam Voice API (`ISteamNetworkingMessages`); PlayStation uses
  platform party voice via `sceVoiceQoS`; Xbox uses Game Chat 2 (`chat_manager::start_processing`).
  PC cross-play scenarios may require a third-party voice solution (e.g., Vivox) as a fallback
  when platform voice is unavailable.

## Cloud and Entitlements

### F-14.5.5 Platform Cloud Storage

Save and load player settings, key bindings, UI layouts, and add-on configurations to
platform-managed cloud storage. Conflict resolution uses last-write-wins with a timestamp
comparison. Cloud saves ensure MMO players retain their preferences when switching machines.

- **Requirements:** R-14.5.5
- **Dependencies:** None
- **Platform notes:** Steam uses `ISteamRemoteStorage` or Steam Auto-Cloud with manifest
  configuration; PlayStation uses `sceNpSaveData`; Xbox uses Connected Storage
  (`XGameSaveInitializeProviderAsync`). Maximum file sizes and total quotas differ by platform
  and must be respected to pass certification.

### F-14.5.6 Entitlements, DLC, and Subscription Verification

Query the platform for owned entitlements (base game, expansions, cosmetic DLC, subscription
tiers) and gate content access accordingly. Entitlement checks run at login and periodically
during play to detect subscription lapses or new purchases without requiring a restart.

- **Requirements:** R-14.5.6
- **Dependencies:** None
- **Platform notes:** Steam uses `ISteamApps::BIsSubscribedApp` and `BIsDlcInstalled`;
  PlayStation uses `sceNpEntitlementAccess`; Xbox uses
  `XStoreQueryEntitledProductsAsync`. Console certification requires that unavailable content
  is hidden or clearly marked, and that purchasing flows redirect to the platform storefront.

### F-14.5.7 Console Certification Compliance

Enforce platform-specific certification requirements: proper suspend/resume handling, mandatory
system UI overlays, controller disconnection prompts, content rating gates, accessibility
mandates, and safe-area rendering. Non-compliance blocks release on console storefronts.

- **Requirements:** R-14.5.7
- **Dependencies:** F-14.1.1, F-14.5.1, F-14.5.5, F-14.5.6
- **Platform notes:** PlayStation requires `sceSystemServiceLoadExec` handling for suspend and
  mandatory trophy support; Xbox requires proper `CoreApplication` lifecycle events and Xbox
  Accessibility Guidelines (XAG) compliance. Both platforms require responding to memory
  pressure notifications and releasing non-essential resources within platform-mandated
  timeframes.
