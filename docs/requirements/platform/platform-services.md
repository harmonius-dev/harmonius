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
