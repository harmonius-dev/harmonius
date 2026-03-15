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

## User Data Storage

### F-14.5.8 User Preferences Storage

Persist player preferences (graphics quality, resolution, keybindings, audio volumes, language,
accessibility settings, UI layout, controller deadzone calibration) across sessions using a
tiered storage strategy. **Local storage**: preferences are saved to the platform-appropriate
user data directory — `%LOCALAPPDATA%\Harmonius\{GameName}\` on Windows,
`~/Library/Application Support/{GameName}/` on macOS, `~/.local/share/{GameName}/` on Linux
(XDG_DATA_HOME). **Cloud sync**: when a platform cloud service is available (F-14.5.5),
preferences are mirrored to cloud storage for cross-device roaming. **Conflict resolution**:
if local and cloud preferences diverge (played on two machines offline), the system presents a
choice dialog showing timestamps and diff summary rather than silently overwriting. Preferences
are stored as a human-readable TOML file (inspectable and manually editable as a recovery
option). The preferences API provides typed get/set with default values — missing keys return
defaults without error. Preferences changes are written to disk within 1 second of modification
with atomic write (write-to-temp, rename) to prevent corruption from crashes. A reset-to-
defaults option restores all preferences to the shipping defaults.

- **Requirements:** R-14.5.8
- **Dependencies:** F-14.5.5 (Platform Cloud Storage), F-14.6.1 (Async File Operations)
- **Platform notes:** Windows: `%LOCALAPPDATA%`. macOS: `~/Library/Application Support/`.
  Linux: `$XDG_DATA_HOME` (defaults to `~/.local/share/`). Console: title storage APIs.
  iOS: `NSUserDefaults` for lightweight prefs, app container for full file. Android:
  `SharedPreferences` for lightweight, app internal storage for full file.

### F-14.5.9 Local File Cache for Players

A managed local cache directory for downloaded content that the game runtime controls:
downloaded asset bundles (F-15.14.6), on-demand DLC content (F-13.23.7), mod packages
(F-15.16.3), downloaded textures/meshes from streaming (F-12.5.4), and universe generation
output (F-3.6.62). The cache is stored in `%LOCALAPPDATA%\Harmonius\{GameName}\Cache\` on
Windows, `~/Library/Caches/{GameName}/` on macOS, `~/.cache/{GameName}/` on Linux
(XDG_CACHE_HOME). The cache manager tracks: total cache size, per-category size (bundles,
mods, streaming, generation), last-access timestamps, and a configurable maximum size
(default: 10 GB). When the cache exceeds its budget, the manager evicts least-recently-used
entries starting with the lowest-priority category. Players can view cache usage and clear
specific categories from the game settings UI. The cache directory follows platform conventions
for data that can be safely deleted without losing player progress — OS-level storage
management (iOS offloading, Android auto-clear) can reclaim cache space transparently.

- **Requirements:** R-14.5.9
- **Dependencies:** F-14.6.1 (Async File Operations), F-15.14.6 (Asset Bundles)
- **Platform notes:** Windows: `%LOCALAPPDATA%\...\Cache\`. macOS: `~/Library/Caches/`.
  Linux: `$XDG_CACHE_HOME`. iOS: `NSCachesDirectory` (system may purge when storage is low).
  Android: `getCacheDir()` (system may purge). Console: scratch storage with title-managed
  lifecycle.

### F-14.5.10 Developer Local Cache

A managed local cache for game developers (separate from player cache) storing: compiled asset
cache (cooked textures, meshlet data, LODs), shader bytecode cache (compiled DXIL/SPIR-V/MSL
keyed by HLSL source hash), logic graph bytecode cache, editor thumbnail cache, and hot-reload
intermediate files. Stored in the project directory under `.harmonius/cache/` (git-ignored).
The developer cache integrates with the shared build cache (F-15.11.1) — local cache is
checked first, shared cache second, full rebuild last. Cache invalidation is hash-based: if
the source asset hash changes, the cached output is evicted. A CLI command
`harmonius cache clear [--category=shaders|assets|all]` manages the cache. Total cache size
is displayed in the editor status bar with a one-click purge button. The developer cache
reduces incremental build times from minutes to seconds for unchanged assets.

- **Requirements:** R-14.5.10
- **Dependencies:** F-15.11.1 (Shared Build Cache), F-12.3.3 (Hash-Based Caching)
- **Platform notes:** Desktop only (editor feature). Cache stored in project `.harmonius/cache/`
  directory, excluded from version control via `.gitignore`.

### F-14.5.11 Pipeline State Object (PSO) Cache

Cache compiled GPU pipeline state objects (PSOs) to disk to eliminate shader compilation
stutter on subsequent runs. PSOs are keyed by a hash of: shader bytecode, render state
(blend, depth, stencil, rasterizer), vertex layout, and render target formats. On first
encounter, the GPU compiles the PSO and the engine serializes it to the PSO cache directory.
On subsequent launches, cached PSOs are loaded during a warmup phase (loading screen or
background thread) to pre-populate the GPU driver cache. The PSO cache is per-GPU-model and
per-driver-version — driver updates invalidate stale entries. For shipped games, a pre-built
PSO cache is generated during the cook process (F-15.14.1) from a reference playthrough that
exercises all material/state combinations, included in the distribution package. The PSO cache
reduces first-frame stutter from 100+ms to <1ms for previously-seen pipeline states. Cache
is stored in the player local cache directory (F-14.5.9) under a `pso/` subdirectory.

- **Requirements:** R-14.5.11
- **Dependencies:** F-2.1.1 (GPU Backend Trait), F-14.5.9 (Local File Cache)
- **Platform notes:** D3D12: uses `ID3D12Device::CreatePipelineState` with cached blob via
  `GetCachedBlob`. Vulkan: uses `VkPipelineCache` with `vkGetPipelineCacheData`. Metal:
  uses `MTLBinaryArchive` for pre-compiled pipelines. Mobile GPUs often have driver-managed
  PSO caches; the engine's cache supplements but does not replace them.

### F-14.5.12 Temporary File Management

A managed temporary directory for short-lived files needed during engine and game operation:
screenshot captures before save, video encoding intermediates, crash dump staging, shader
compilation intermediates, asset cooking temp files, network packet replay buffers, and
game-specific scratch data. The temp directory is located at the platform-appropriate path:
`%TEMP%\Harmonius\{GameName}\` on Windows, `/tmp/harmonius-{GameName}/` on macOS/Linux,
or the platform's designated temp storage on consoles and mobile. The temp manager: creates
the directory on engine startup, provides an API for allocating named temp files with
automatic cleanup on engine shutdown, supports configurable max temp size (default: 1 GB)
with oldest-first eviction, cleans up orphaned temp files from previous crashed sessions on
startup (files older than 24 hours), and never stores player-important data (all temp data
is recreatable). Temp files are excluded from backup systems. A `TempFileHandle` RAII type
ensures files are deleted when the handle is dropped, preventing leaks.

- **Requirements:** R-14.5.12
- **Dependencies:** F-14.6.1 (Async File Operations)
- **Platform notes:** Windows: `%TEMP%`. macOS/Linux: `/tmp/` or `$TMPDIR`. iOS: `NSTemporaryDirectory()`. Android: `getCacheDir()` (shared with player cache). Console: title scratch storage.

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
