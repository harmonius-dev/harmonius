# R-8.5 -- Session Management Requirements

## Authentication and Login

1. **R-8.5.1** — The engine **SHALL** authenticate players against external identity providers
   (OAuth 2.0, platform accounts) and issue short-lived session tokens for the transport handshake,
   supporting thousands of concurrent logins, multi-factor authentication, and platform-specific
   identity systems (Steam, PSN, Xbox Live, Apple Game Center).
   - **Rationale:** Player identity verification is the foundation of session security;
     platform-specific integration is required for console certification.
   - **Verification:** Integration test: authenticate 5,000 concurrent logins and verify all
     complete within 5 seconds. Verify tokens expire after configured TTL. Verify each platform
     identity provider returns a valid session token.

## Matchmaking

1. **R-8.5.2** — The engine **SHALL** match players into instanced content based on skill rating,
   latency to available servers, group composition requirements, and queue preferences, balancing
   match quality against queue time and scaling to tens of thousands of concurrent queue entries.
   - **Rationale:** Fair and responsive matchmaking is critical for player retention in competitive
     and cooperative content.
   - **Verification:** Load test: enqueue 20,000 players with varied skill ratings and regions.
     Verify all matched within 120 seconds. Verify matched groups have skill rating variance below
     the configured threshold. Verify group queuing places all members in the same instance.
2. **R-8.5.3** — The engine **SHALL** support persistent parties and temporary lobbies with role
   designation (tank, healer, DPS), ready checks, and leader-managed invitations, persisting across
   zone transitions and server migrations.
   - **Rationale:** Parties are the fundamental social unit for group content; they must survive
     server infrastructure events without disrupting the player experience.
   - **Verification:** Integration test: form a 5-player party, assign roles, trigger a zone
     transition. Verify all members remain with roles intact. Verify ready check completes when all
     respond. Verify invitation reaches the target player.

## Server Infrastructure

1. **R-8.5.4** — The engine **SHALL** manage a fleet of dedicated server processes that host game
   world zones and instanced content, provisioning, monitoring, and recycling server processes,
   routing players to the correct server, and supporting rolling restarts without disconnecting
   players by draining and migrating sessions.
   - **Rationale:** Live-service games must deploy patches without downtime; cluster management
     automates server lifecycle and player routing.
   - **Verification:** Integration test: provision 10 server processes and verify routing. Initiate
     a rolling restart and verify zero disconnections. Verify a crashed process is replaced within
     30 seconds.
2. **R-8.5.5** — The engine **SHALL** allow clients to discover available sessions via a directory
   service and seamlessly reconnect to an active session after a transient disconnect, restoring the
   player to their exact prior state within a configurable grace window (default 120 seconds).
   - **Rationale:** Transient disconnects must not force players to restart their session or lose
     progress.
   - **Verification:** Integration test: simulate a 10-second outage and verify reconnection
     restores exact state within 3 seconds of network restoration. Verify an outage exceeding the
     grace window results in clean logout.

## Cross-Platform

1. **R-8.5.6** — The engine **SHALL** match players across different platforms (PC, PlayStation,
   Xbox, Switch, mobile) into shared game sessions, unify platform-specific matchmaking APIs behind
   a common abstraction, and support linking multiple platform accounts to a single game account for
   cross-platform progression.
   - **Rationale:** Cross-play expands the matchmaking pool and lets players switch platforms
     without losing progress; platform API abstraction isolates certification requirements.
   - **Verification:** Integration test: match a PC, PlayStation, and Xbox player into the same
     instance. Verify cross-play opt-out restricts to same-platform. Verify account linking allows
     access from a second platform with the same progression.

## Capacity Management

1. **R-8.5.7** — The engine **SHALL** place incoming connections into a managed queue with position
   display, estimated wait time, and configurable priority tiers when a server reaches capacity,
   integrating with the load balancer to redirect players to less-populated shards.
   - **Rationale:** Unmanaged capacity overflows crash servers or degrade gameplay; queues provide a
     controlled experience during peak load.
   - **Verification:** Integration test: fill a server to capacity and connect 500 additional
     players. Verify all enter the queue with position and estimated wait. Verify VIP-tier
     advancement. Verify reconnection within timeout preserves position.

## Headless Server

1. **R-8.5.8** — The engine **SHALL** provide a headless server mode that executes the full ECS
   simulation without GPU, window, audio, or input systems. The server **SHALL** accept
   configuration via command-line arguments and environment variables, produce a minimal binary
   suitable for Docker deployment, expose HTTP health check endpoints, and support graceful shutdown
   with world state persistence and player migration.
   - **Rationale:** Dedicated servers are the backbone of multiplayer infrastructure; headless mode
     eliminates GPU costs and enables containerized deployment at scale.
   - **Verification:** Integration test: launch in Docker without GPU. Connect 64 players and verify
     ECS simulation at configured tick rate. Query health endpoint and verify correct status.
     Initiate graceful shutdown and verify state saved and players migrated.

## Advanced Matchmaking

1. **R-8.5.9** — The engine **SHALL** provide a self-hosted matchmaking microservice that groups
   players by Glicko-2 skill rating (rating, deviation, volatility), geographic region, and game
   mode preferences, widening the rating window over time, supporting cross-play opt-in/opt-out, and
   exposing a REST/gRPC API.
   - **Rationale:** Glicko-2 provides statistically robust skill estimation; a self-hosted
     microservice avoids vendor lock-in and integrates with existing infrastructure.
   - **Verification:** Integration test: enqueue 1,000 players with varied ratings. Verify all
     matched within latency budget. Verify cross-play opt-out. Verify REST API returns correct queue
     status. Verify Glicko-2 recalculation after match.

## Non-Functional

1. **R-8.5.10** — Session reconnection **SHALL** restore the player to their exact prior state
   within 3 seconds of network restoration, provided the disconnect duration is within the
   configured grace window (default 120 seconds).
   - **Rationale:** Slow reconnection frustrates players returning after transient network issues.
   - **Verification:** Simulate a 10-second disconnect. Measure time to full state restoration and
     verify under 3 seconds. Verify position, buffs, group, and combat state restored.
2. **R-8.5.11** — The headless server **SHALL** consume under 512 MB of RAM for a 64-player session
   running physics, AI, networking, and game framework systems at 30 ticks per second.
   - **Rationale:** Low memory footprint allows dense packing of server instances, reducing
     per-player hosting cost.
   - **Verification:** Launch with 64 players running a standard game mode. Monitor RSS memory over
     10 minutes and verify peak RSS under 512 MB.
3. **R-8.5.12** — The matchmaking service **SHALL** find a match within 60 seconds for 95% of
   players during peak hours. The rating window **SHALL** widen at configurable intervals (default
   every 10 seconds).
   - **Rationale:** Long queue times drive player abandonment; a 60-second target balances match
     quality against wait time.
   - **Verification:** Load test: simulate 10,000 concurrent queue entries. Verify 95th percentile
     time-to-match under 60 seconds. Verify rating window widens at the configured interval.
4. **R-8.5.13** — The server cluster **SHALL** maintain 99.9% availability (< 8.77 hours downtime
   per year), with automatic failover replacing crashed processes within 30 seconds and rolling
   restarts completing without disconnecting any player.
   - **Rationale:** MMO players expect near-continuous availability; extended downtime causes player
     attrition and revenue loss.
   - **Verification:** Chaos test: kill a process during peak load. Verify replacement provisioned
     within 30 seconds. Verify affected players migrated. Initiate rolling restart and verify zero
     disconnections.

## Leaderboard Service

5. **R-8.5.14** — The engine **SHALL** provide a self-hosted leaderboard service supporting multiple
   boards per game, per-board reset schedules, global and friend-filtered queries, and server-side
   score validation before submission.
   - **Rationale:** Self-hosted leaderboards avoid platform rate limits and provide cross-platform
     unified rankings with server-side anti-cheat validation.
   - **Verification:** Integration test: submit scores to daily and all-time boards. Query global
     and friend-filtered results. Verify reset clears the daily board. Submit an invalid score and
     verify rejection.

## Achievement Service

6. **R-8.5.15** — The engine **SHALL** provide a server-side achievement service supporting
   progress-based and boolean achievements, persisted to TiKV, with automatic sync to platform SDK
   achievement systems.
   - **Rationale:** Server-side tracking prevents client-side achievement manipulation and provides
     a single source of truth that syncs to each platform's trophy/achievement system.
   - **Verification:** Integration test: increment a progress-based achievement across sessions.
     Verify persistence in TiKV. Verify platform SDK sync on unlock. Verify boolean achievement
     unlocks correctly.

## Cloud Save Service

7. **R-8.5.16** — The engine **SHALL** provide a self-hosted cloud save service with per-player
   storage slots, configurable quota, last-write-wins or merge-callback conflict resolution, and
   rkyv serialization.
   - **Rationale:** Self-hosted cloud saves work across all platforms without relying on
     platform-specific cloud storage, enabling cross-platform save continuity.
   - **Verification:** Integration test: save and load player data across two devices. Verify
     conflict resolution with concurrent writes. Verify quota enforcement rejects oversized saves.

## Telemetry Service

8. **R-8.5.17** — The engine **SHALL** provide a telemetry service for fire-and-forget event
   submission with buffered batching, player session tracking, and GDPR opt-out with data deletion
   API.
   - **Rationale:** Telemetry enables data-driven balancing and retention analysis; GDPR compliance
     is legally required for EU players.
   - **Verification:** Integration test: submit 10,000 events and verify batched delivery. Verify
     session tracking records login and playtime. Opt out and verify data deletion API removes all
     player telemetry.

## Matchmaking Extensions

9. **R-8.5.18** — The engine matchmaker **SHALL** support backfill requests to replace players who
   leave mid-match, searching the queue for skill-compatible replacements within a configurable wait
   window.
   - **Rationale:** Backfill prevents permanent team disadvantage when a player leaves, improving
     match quality for remaining participants.
   - **Verification:** Integration test: remove a player mid-match. Verify backfill request is
     issued. Verify a replacement is matched within the configured wait window. Verify the
     replacement's skill rating is within the allowed range.
10. **R-8.5.19** — The engine **SHALL** detect match abandonment, apply escalating penalties
    (cooldown, rating penalty, temporary ban) tracked per rolling window, with a reconnect grace
    period and team vote exemption.
    - **Rationale:** Escalating penalties deter repeat abandonment while the grace period and team
      vote exemption prevent punishing legitimate disconnects and consensual departures.
    - **Verification:** Integration test: abandon 3 matches in a rolling window. Verify escalating
      penalties applied. Reconnect within the grace period and verify no penalty. Trigger a team
      vote exemption and verify no penalty.
11. **R-8.5.20** — The engine **SHALL** track per-player skill ratings with Glicko-2, supporting
    rating decay on inactivity, placement matches with wide deviation, per-game-mode ratings, soft
    season resets, and display rank tier mapping.
    - **Rationale:** Extended Glicko-2 with decay, placement, and seasons provides accurate skill
      estimation across all game modes and time periods.
    - **Verification:** Integration test: complete 10 placement matches and verify wide deviation
      narrows. Verify per-mode ratings are independent. Verify season reset softens deviation.
      Verify display rank maps correctly from rating.
12. **R-8.5.21** — The engine **SHALL** provide a PlatformAdapter trait with cfg-gated static
    dispatch bridging engine session services to each platform SDK for authentication, achievements,
    friends, voice, and store integration.
    - **Rationale:** A unified adapter isolates platform-specific API differences behind a single
      trait, preventing platform branching in game code.
    - **Verification:** Integration test per platform: authenticate via the adapter. Verify
      achievement unlock, friends list query, and store entitlement check route to the correct
      platform SDK.
13. **R-8.5.22** — The engine **SHALL** provide a StoreService abstraction for querying products,
    initiating purchases, and verifying entitlements across platform stores, with overlay detection
    that pauses game input while the platform overlay is active.
    - **Rationale:** A unified store abstraction enables in-game purchasing without platform
      branching; overlay detection prevents accidental input during purchase flows.
    - **Verification:** Integration test: query products, initiate a purchase, and verify
      entitlement after completion. Activate the platform overlay and verify game input is paused.
14. **R-8.5.23** — The engine matchmaker **SHALL** support multiple queue types (casual,
    competitive, raid with role-based filling, custom) and a leaver queue penalty pool for players
    with high abandon rates.
    - **Rationale:** Different game modes require different matching rules; a leaver queue isolates
      chronic abandoners from the general population.
    - **Verification:** Integration test: enqueue players into casual, competitive, and raid queues.
      Verify role-based filling for raid. Verify a player with high abandon rate is routed to the
      leaver queue.
