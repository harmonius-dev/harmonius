# R-14.5 — Platform Services Requirements

## Achievements and Progression

### R-14.5.1 Cross-Platform Achievement System

The engine **SHALL** unlock achievements through a unified abstraction over Steam, PlayStation
Network, and Xbox Live APIs, mapping internal achievement IDs to platform-specific identifiers and
deferring unlock attempts when the platform service is temporarily unavailable, retrying
automatically on reconnection.

- **Derived from:** [F-14.5.1](../../features/platform/platform-services.md)
- **Rationale:** A unified abstraction prevents gameplay code from branching on platform, and
  deferred unlocks ensure players never lose achievement progress during transient network or
  service outages.
- **Verification:** Integration test per platform: unlock an achievement while online and verify it
  appears in the platform's achievement list. Disconnect the network, unlock a second achievement,
  reconnect, and verify the deferred unlock completes automatically. Verify the internal-to-platform
  ID mapping resolves correctly for all configured achievements.

### R-14.5.2 Leaderboards

The engine **SHALL** submit and query ranked scores through platform leaderboard APIs, supporting
global, friends-only, and around-player queries, with score submissions batched and retried on
transient failures, and query results cached to respect platform rate limits.

- **Derived from:** [F-14.5.2](../../features/platform/platform-services.md)
- **Rationale:** Batching and retry ensure score submissions are not lost during network hiccups,
  while caching prevents rate-limit violations that would block leaderboard access for all players.
- **Verification:** Integration test: submit 10 scores, query global and friends-only leaderboards,
  and verify correct ranking order. Simulate a transient network failure during submission and verify
  scores are retried and eventually recorded. Verify cached results are served for repeated queries
  within the cache TTL.

## Social

### R-14.5.3 Rich Presence

The engine **SHALL** update the player's platform rich presence with contextual game information
(current zone, party size, activity type) visible to friends on the platform's social UI, throttled
to at most one update per 15 seconds to stay within platform API rate limits.

- **Derived from:** [F-14.5.3](../../features/platform/platform-services.md)
- **Rationale:** Rich presence drives organic discovery by showing friends what content a player is
  engaged with, while throttling prevents API rate-limit violations that would silently drop
  updates.
- **Verification:** Integration test: set rich presence with zone and activity fields and verify the
  platform API receives the correct values. Issue 10 rapid updates within 5 seconds and verify only
  one API call is made, with the most recent state. Verify presence clears on session disconnect.

### R-14.5.4 Platform Voice and Party Integration

The engine **SHALL** bridge the engine's voice chat with platform party and voice systems so that
players in a platform party hear each other in-game without manual channel setup, supporting mute,
per-player volume control, and voice activity indicators sourced from the platform voice stream.

- **Derived from:** [F-14.5.4](../../features/platform/platform-services.md)
- **Rationale:** Automatic platform party bridging eliminates friction for players who expect their
  platform party voice to work seamlessly in-game without manual configuration.
- **Verification:** Integration test: create a platform party of 3 players, launch the game, and
  verify all players hear each other without manual setup. Mute one player and verify their audio
  is silenced for other participants. Verify voice activity indicators reflect actual speech from
  the platform voice stream.

## Cloud and Entitlements

### R-14.5.5 Platform Cloud Storage

The engine **SHALL** save and load player settings, key bindings, UI layouts, and add-on
configurations to platform-managed cloud storage, using last-write-wins with timestamp comparison
for conflict resolution, and respecting platform-specific file size and quota limits.

- **Derived from:** [F-14.5.5](../../features/platform/platform-services.md)
- **Rationale:** Cloud-synced preferences ensure players retain their settings when switching
  machines, while respecting quota limits prevents certification failures on consoles.
- **Verification:** Integration test: save settings to cloud storage, read them back from a second
  device, and verify data integrity. Simulate a conflict by writing from two devices and verify
  last-write-wins resolves correctly via timestamp. Verify writes exceeding the platform quota are
  rejected gracefully with an error rather than silent truncation.

### R-14.5.6 Entitlements, DLC, and Subscription Verification

The engine **SHALL** query the platform for owned entitlements (base game, expansions, cosmetic DLC,
subscription tiers) at login and periodically during play, gating content access based on
entitlement state and detecting subscription lapses or new purchases without requiring a restart.

- **Derived from:** [F-14.5.6](../../features/platform/platform-services.md)
- **Rationale:** Periodic entitlement checks ensure content gating remains accurate during long play
  sessions where a subscription may lapse or a player may purchase DLC from an external storefront.
- **Verification:** Integration test: verify a player with base-game-only entitlement cannot access
  expansion content. Purchase DLC externally during a session and verify the periodic check detects
  the new entitlement within the configured polling interval. Verify unavailable content is hidden
  or marked as locked per platform certification requirements.

### R-14.5.8 User Preferences Storage

The engine **SHALL** persist player preferences to platform-appropriate local directories
(`%LOCALAPPDATA%` on Windows, `~/Library/Application Support/` on macOS, `$XDG_DATA_HOME`
on Linux) as human-readable TOML files. Preferences **SHALL** sync to platform cloud storage
(R-14.5.5) when available. Conflict between local and cloud **SHALL** present a choice dialog
with timestamps rather than silently overwriting. Write **SHALL** use atomic write-to-temp-
then-rename to prevent corruption. Writes **SHALL** complete within 1 second of modification.
A reset-to-defaults option **SHALL** restore all preferences to shipping defaults.

- **Derived from:** [F-14.5.8](../../features/platform/platform-services.md)
- **Rationale:** Player preferences must survive crashes (atomic write), roam across devices
  (cloud sync), and be recoverable from corruption (human-readable TOML, reset option).
- **Verification:** Set a preference, kill the process mid-write; verify the file is not
  corrupted. Modify preferences on two machines offline; reconnect and verify conflict dialog
  appears. Verify reset-to-defaults restores all values.

### R-14.5.9 Local File Cache for Players

The engine **SHALL** manage a local cache directory for downloaded content (asset bundles,
DLC, mods, streaming data, generation output) in platform-appropriate cache locations
(`%LOCALAPPDATA%\...\Cache\` on Windows, `~/Library/Caches/` on macOS, `$XDG_CACHE_HOME`
on Linux). Cache size **SHALL** be capped at a configurable maximum (default 10 GB) with
LRU eviction by category priority. Players **SHALL** be able to view cache usage and clear
categories from the settings UI. The OS **SHALL** be able to reclaim cache space on mobile
without losing player progress.

- **Derived from:** [F-14.5.9](../../features/platform/platform-services.md)
- **Rationale:** Unmanaged caches grow unbounded; managed LRU eviction prevents disk-full
  errors. Platform-appropriate directories ensure OS storage management works correctly.
- **Verification:** Fill cache to 10 GB; download a new asset; verify LRU eviction frees
  space and the download succeeds. Clear a specific category; verify only that category is
  removed. On iOS, trigger system storage pressure; verify cache is purged.

### R-14.5.10 Developer Local Cache

The engine **SHALL** maintain a developer build artifact cache in `.harmonius/cache/` within
the project directory (git-ignored) storing compiled assets, shader bytecode, logic graph
bytecode, and thumbnails. Cache lookup **SHALL** follow a three-tier chain: local cache
first, shared network cache (R-15.11.1) second, full rebuild last. Cache keys **SHALL** be
content hashes (BLAKE3) so changed source assets automatically invalidate stale outputs.
A CLI command **SHALL** allow clearing the cache by category. Cache size **SHALL** be
displayed in the editor status bar.

- **Derived from:** [F-14.5.10](../../features/platform/platform-services.md)
- **Rationale:** Local caching reduces incremental build times from minutes to seconds.
  Three-tier lookup ensures cache hits from team members via the shared network cache.
- **Verification:** Build an asset; verify cached output exists. Modify the source; verify
  the stale cached output is evicted. Clear the local cache; verify the shared network cache
  provides the asset without a full rebuild.

### R-14.5.11 Pipeline State Object (PSO) Cache

The engine **SHALL** serialize compiled GPU pipeline state objects to disk keyed by hash of
shader bytecode, render state, vertex layout, and render target formats. Cached PSOs
**SHALL** be loaded during a warmup phase to eliminate shader compilation stutter. Cache
entries **SHALL** be invalidated when GPU model or driver version changes. Shipped games
**SHALL** include a pre-built PSO cache generated from a reference playthrough during the
cook process. Loading a cached PSO **SHALL** take less than 1ms (vs 100+ms for first-time
compilation).

- **Derived from:** [F-14.5.11](../../features/platform/platform-services.md)
- **Rationale:** Shader compilation stutter is the most visible performance issue in modern
  games. PSO caching eliminates it for all previously-encountered pipeline states.
- **Verification:** Play a scene; quit and relaunch; verify zero compilation stutter on the
  second run. Update the GPU driver; verify stale cache entries are invalidated and
  recompiled. Measure PSO load time from cache; verify under 1ms per state.

### R-14.5.12 Temporary File Management

The engine **SHALL** provide a managed temporary directory at the platform-appropriate path
with automatic cleanup of orphaned files from crashed sessions (files older than 24 hours)
on startup. A `TempFileHandle` RAII type **SHALL** guarantee deletion when dropped. Temp
directory size **SHALL** be capped at a configurable maximum (default 1 GB) with oldest-first
eviction. Temp files **SHALL NOT** contain player-important data — all temp data **SHALL**
be recreatable from non-temp sources.

- **Derived from:** [F-14.5.12](../../features/platform/platform-services.md)
- **Rationale:** Unmanaged temp files accumulate across sessions and crashes, wasting disk
  space. RAII handles prevent leaks; orphan cleanup handles crash scenarios.
- **Verification:** Allocate a temp file via the API; drop the handle; verify the file is
  deleted. Kill the process with temp files open; relaunch; verify orphaned files are cleaned
  up. Fill temp to 1 GB; allocate another; verify oldest file is evicted.

### R-14.5.7 Console Certification Compliance

The engine **SHALL** enforce platform-specific certification requirements including proper
suspend/resume handling, mandatory system UI overlays, controller disconnection prompts, content
rating gates, accessibility mandates, and safe-area rendering, on all supported console platforms.

- **Derived from:** [F-14.5.7](../../features/platform/platform-services.md)
- **Rationale:** Non-compliance with any mandatory certification requirement blocks release on
  console storefronts, making compliance a hard prerequisite for shipping.
- **Verification:** Integration test per console: trigger a suspend event and verify the engine
  saves state and releases non-essential resources within the platform-mandated timeframe. Trigger a
  controller disconnection and verify the engine displays a prompt. Verify system UI overlay
  rendering is not obstructed. Run the platform's automated certification test suite and verify all
  mandatory checks pass.
