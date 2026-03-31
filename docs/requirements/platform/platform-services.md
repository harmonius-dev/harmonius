# R-14.5 — Platform Services Requirements

## Achievements and Progression

1. **R-14.5.1** — The engine **SHALL** unlock achievements through a unified abstraction over Steam,
   PlayStation Network, and Xbox Live APIs, mapping internal achievement IDs to platform-specific
   identifiers and deferring unlock attempts when the service is temporarily unavailable, retrying
   on reconnection.
   - **Rationale:** A unified abstraction prevents gameplay branching on platform. Deferred unlocks
     ensure no achievement is lost during transient outages.
   - **Verification:** Integration test per platform: unlock while online, verify on platform list.
     Disconnect, unlock, reconnect, verify deferred unlock completes. Verify ID mapping resolves.

2. **R-14.5.2** — The engine **SHALL** submit and query ranked scores through platform leaderboard
   APIs, supporting global, friends-only, and around-player queries, with submissions batched and
   retried on transient failures and query results cached to respect platform rate limits.
   - **Rationale:** Batching and retry prevent score loss during network issues. Caching prevents
     rate-limit violations.
   - **Verification:** Integration test: submit 10 scores, query global and friends-only, verify
     ranking. Simulate transient failure, verify retry. Query twice within cache TTL, verify second
     uses cache.

## Social

3. **R-14.5.3** — The engine **SHALL** update rich presence with contextual game information (zone,
   party size, activity), throttled to at most one update per 15 seconds to stay within API rate
   limits.
   - **Rationale:** Rich presence drives organic discovery. Throttling prevents rate-limit
     violations.
   - **Verification:** Integration test: set rich presence, verify platform API receives correct
     values. Issue 10 rapid updates within 5 seconds, verify only one API call. Verify presence
     clears on disconnect.

4. **R-14.5.4** — The engine **SHALL** bridge voice chat with platform party systems so players hear
   each other in-game without manual channel setup, supporting mute, per-player volume, and voice
   activity indicators.
   - **Rationale:** Automatic bridging eliminates friction for players expecting platform party
     voice to work seamlessly.
   - **Verification:** Integration test: create a party of 3 players, launch game, verify all hear
     each other. Mute one player, verify silenced. Verify voice activity indicators reflect speech.

## Cloud and Entitlements

5. **R-14.5.5** — The engine **SHALL** save and load player settings, key bindings, and UI layouts
   to platform cloud storage, using last-write-wins with timestamp comparison for conflict
   resolution, and respecting platform quota limits.
   - **Rationale:** Cloud sync ensures preferences roam across devices. Respecting quotas prevents
     certification failures.
   - **Verification:** Integration test: save settings, read from second device, verify integrity.
     Simulate conflict, verify last-write-wins. Exceed quota, verify graceful rejection.

6. **R-14.5.6** — The engine **SHALL** query the platform for owned entitlements at login and
   periodically during play, gating content access and detecting subscription lapses or new
   purchases without requiring a restart.
   - **Rationale:** Periodic checks ensure content gating remains accurate during long sessions.
   - **Verification:** Integration test: verify base-game-only entitlement blocks expansion content.
     Purchase DLC mid-session, verify periodic check detects new entitlement. Verify locked content
     is hidden or marked per certification.

## User Preferences

7. **R-14.5.8** — The engine **SHALL** persist preferences to platform-appropriate local directories
   as human-readable TOML files. Preferences **SHALL** sync to cloud (R-14.5.5) when available.
   Conflict between local and cloud **SHALL** present a choice dialog. Writes **SHALL** use atomic
   write-to-temp- then-rename. Writes **SHALL** complete within 1 second. A reset-to-defaults
   **SHALL** be available.
   - **Rationale:** Preferences must survive crashes (atomic write), roam across devices (cloud
     sync), and be recoverable (human-readable TOML, reset).
   - **Verification:** Set preference, kill mid-write; verify no corruption. Modify on two machines
     offline; reconnect, verify conflict dialog. Verify reset restores all defaults.

## Local Caching

8. **R-14.5.9** — The engine **SHALL** manage a local cache directory for downloaded content (asset
   bundles, DLC, mods, streaming data) in platform-appropriate cache locations. Cache size **SHALL**
   be capped at a configurable maximum (default 10 GB) with LRU eviction by category priority.
   Players **SHALL** view usage and clear categories from settings.
   - **Rationale:** Unmanaged caches grow unbounded. Managed LRU eviction prevents disk-full errors.
   - **Verification:** Fill to 10 GB; download new asset; verify LRU eviction. Clear a category;
     verify only it is removed. On iOS, trigger storage pressure; verify cache is purged.

9. **R-14.5.10** — The engine **SHALL** maintain a developer build cache in `.harmonius/cache/`
   (git-ignored) with a three-tier lookup: local cache, shared network cache, full rebuild. Cache
   keys **SHALL** be BLAKE3 content hashes. A CLI command **SHALL** allow clearing by category.
   - **Rationale:** Local caching reduces incremental build times. Three-tier lookup shares results
     across team members.
   - **Verification:** Build an asset, verify cached output. Modify source, verify stale output
     evicted. Clear local, verify shared cache provides asset.

10. **R-14.5.11** — The engine **SHALL** serialize compiled GPU pipeline state objects to disk keyed
    by hash of shader bytecode, render state, vertex layout, and render target formats. Cache
    **SHALL** be invalidated on GPU model or driver change. Shipped games **SHALL** include a
    pre-built PSO cache. Loading a cached PSO **SHALL** take under 1 ms.
    - **Rationale:** Shader compilation stutter is the most visible performance issue. PSO caching
      eliminates it for previously-seen states.
    - **Verification:** Play a scene, quit, relaunch; verify zero stutter on second run. Update
      driver; verify stale entries invalidated. Measure PSO load time; verify under 1 ms.

## Temporary Files

11. **R-14.5.12** — The engine **SHALL** provide a managed temp directory with automatic cleanup of
    orphaned files (older than 24 hours) on startup. A `TempFileHandle` RAII type **SHALL** delete
    on drop. Temp size **SHALL** be capped (default 1 GB) with oldest-first eviction. Temp data
    **SHALL** be recreatable from non-temp sources.
    - **Rationale:** Unmanaged temp files accumulate. RAII handles prevent leaks; orphan cleanup
      handles crash scenarios.
    - **Verification:** Allocate temp file, drop handle, verify deleted. Kill process with open
      temps, relaunch, verify cleanup. Fill to 1 GB, allocate another, verify oldest evicted.

## Console Certification

12. **R-14.5.7** — The engine **SHALL** enforce platform certification requirements including
    suspend/resume, system UI overlays, controller disconnection prompts, content rating gates,
    accessibility mandates, and safe-area rendering on all supported console platforms.
    - **Rationale:** Non-compliance blocks console store release. Engine-level enforcement
      eliminates per-title certification work.
    - **Verification:** Integration test per console: trigger suspend, verify state save and
      resource release within platform deadline. Trigger controller disconnect, verify prompt. Run
      platform's automated certification suite.
