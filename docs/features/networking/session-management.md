# 8.5 -- Session Management

## Authentication and Login

| ID      | Feature                           |
|---------|-----------------------------------|
| F-8.5.1 | Login and Authentication Services |

1. **F-8.5.1** — Provide a login flow that authenticates players against an external identity
   provider (OAuth 2.0, platform accounts) and issues short-lived session tokens used by the
   transport handshake (F-8.1.1). The authentication service must handle thousands of concurrent
   logins during peak hours and game launches, support multi-factor authentication, and integrate
   with platform-specific identity systems (Steam, PlayStation Network, Xbox Live, Apple Game
   Center).
   - **Platform:** Each platform requires its own identity provider integration. Console platforms
     mandate use of their first-party authentication SDKs.

## Matchmaking

| ID      | Feature                                  |
|---------|------------------------------------------|
| F-8.5.2 | Skill-Based and Region-Based Matchmaking |
| F-8.5.3 | Lobby and Party System                   |

1. **F-8.5.2** — Match players into instanced content (dungeons, raids, battlegrounds, arenas) based
   on skill rating, latency to available servers, group composition requirements, and queue
   preferences. The matchmaker must balance match quality against queue time, support group queuing,
   and handle cross-region matching when regional populations are low. Must scale to tens of
   thousands of concurrent queue entries.
   - **Deps:** F-8.5.1, F-8.5.4
   - **Platform:** Console platforms may require integration with platform matchmaking APIs for
     certification compliance.
2. **F-8.5.3** — Allow players to form persistent parties and temporary lobbies for instanced
   content. Parties persist across zone transitions and server migrations, support role designation
   (tank, healer, DPS), ready checks, and leader-managed invitations. The lobby system coordinates
   with the matchmaker to place formed groups into appropriate instances and with the world server
   to handle open-world party gameplay.
   - **Deps:** F-8.5.1
   - **Platform:** Mobile party UI is simplified for smaller screens. Cross-platform parties require
     platform-specific invite flows (system share sheet on mobile).

## Server Infrastructure

| ID      | Feature                             |
|---------|-------------------------------------|
| F-8.5.4 | Dedicated Server Cluster Management |
| F-8.5.5 | Session Discovery and Reconnection  |

1. **F-8.5.4** — Manage a fleet of dedicated server processes that host game world zones and
   instanced content. The cluster manager provisions, monitors, and recycles server processes;
   routes players to the correct server based on their world location or matchmaking result; and
   scales capacity dynamically based on population. Must support rolling restarts for patches
   without disconnecting players by draining and migrating sessions.
   - **Platform:** Designed for Linux container orchestration (Kubernetes). Windows and macOS
     servers supported for development.
2. **F-8.5.5** — Allow clients to discover available sessions (character selection, world server,
   instance) via a directory service, and seamlessly reconnect to an active session after a
   transient disconnect. Reconnection must restore the player to their exact prior state --
   position, combat status, group membership -- within a configurable grace window (e.g., 120
   seconds). Critical for retaining players during ISP hiccups and Wi-Fi handoffs.
   - **Deps:** F-8.5.4, F-8.1.1
   - **Platform:** Mobile uses a longer reconnection grace window (180 s) to handle cellular
     dropouts, app backgrounding, and Wi-Fi/cellular handoffs.

## Cross-Platform

| ID      | Feature                                    |
|---------|--------------------------------------------|
| F-8.5.6 | Cross-Play Matchmaking and Account Linking |

1. **F-8.5.6** — Match players across different platforms (PC, PlayStation, Xbox, Switch, mobile)
   into shared game sessions. Platform-specific matchmaking APIs are unified behind the session
   system's abstraction layer. Players link multiple platform accounts to a single game account,
   enabling cross-platform progression, friends lists, and party formation. Matchmaking respects
   platform-specific requirements (console certification rules, input-type filtering) and provides
   opt-out for players who prefer same-platform matches. Leaderboards display platform badges
   alongside player names.
   - **Deps:** F-8.5.1, F-14.5.1 (Achievements and Platform Identity)
   - **Platform:** PlayStation uses PSN APIs, Xbox uses Xbox Live APIs, Switch uses NEX/NPLN. Each
     platform requires separate certification for cross-play features.

## Capacity Management

| ID      | Feature                             |
|---------|-------------------------------------|
| F-8.5.7 | Login Queue and Capacity Management |

1. **F-8.5.7** — When a server or shard reaches player capacity, incoming connections enter a
   managed queue with position display, estimated wait time, and priority tiers (subscribers,
   founders, returning players). The queue system integrates with the load balancer (F-8.7.6) to
   redirect players to less-populated shards when available. Queue position updates are pushed to
   the client at regular intervals. VIP bypass and maintenance-mode queues are configurable per
   server. The system gracefully handles queue abandonment and reconnection (preserving position
   within a timeout window).
   - **Deps:** F-8.5.1, F-8.7.6 (Auto-Scaling)
   - **Platform:** Mobile queue UI supports background notifications (push notification when queue
     position is reached) via platform notification APIs.

## Headless Server

| ID      | Feature                        |
|---------|--------------------------------|
| F-8.5.8 | Headless Dedicated Game Server |

1. **F-8.5.8** — Run the game engine as a headless process without GPU, window, or audio output for
   dedicated server deployment. The headless server executes the full ECS simulation (physics, AI,
   networking, game framework) at a configurable tick rate without rendering, UI, or input systems.
   Server configuration is driven by command-line arguments and environment variables (port, tick
   rate, max players, map name, game mode). The server binary is a stripped build excluding
   rendering, audio, and editor code, producing a minimal executable suitable for containerized
   deployment (Docker). Health check endpoints (HTTP) report server status, player count, and
   performance metrics for load balancer integration. Graceful shutdown saves world state and
   migrates players before terminating. The headless server supports the full MMO infrastructure
   (F-8.7.x) including world sharding, zone transitions, and inter-server communication.
   - **Deps:** F-1.1.1 (ECS), F-8.7.1 (World Sharding), F-13.1.9 (Modular Systems)
   - **Platform:** Linux is the primary server platform. Docker images use Alpine or Debian slim
     base. Windows Server is supported for development.

## Advanced Matchmaking

| ID      | Feature                         |
|---------|---------------------------------|
| F-8.5.9 | Skill-Based Matchmaking Service |

1. **F-8.5.9** — A self-hosted matchmaking service that groups players by skill rating, region, and
   game mode preferences. The service maintains per-player skill ratings using Glicko-2 (rating,
   deviation, volatility) updated after each match. Matchmaking queues search for players within a
   configurable rating window that widens over time to reduce wait times. Region-based filtering
   prioritizes low-latency server assignments (player connects to nearest AWS region). Cross-play
   matchmaking respects platform preferences (opt-in/opt-out of cross-platform matches). The
   matchmaking service runs as a standalone microservice with a REST/gRPC API, deployable via the
   server deployment (F-15.18.1). Queue status, estimated wait time, and match quality metrics are
   exposed to the client for UI display.
   - **Deps:** F-8.5.1, F-8.5.6 (Cross-Play), F-8.5.8 (Headless Server)
   - **Platform:** Deployed as an AWS Lambda function or ECS Fargate container.

## Social and Progression Services

| ID       | Feature                            |
|----------|------------------------------------|
| F-8.5.10 | Self-Hosted Leaderboard Service    |
| F-8.5.11 | Self-Hosted Achievement Tracking   |
| F-8.5.12 | Self-Hosted Cloud Save Storage     |
| F-8.5.13 | Server-Side Telemetry and Analytics|

1. **F-8.5.10** — An engine-hosted leaderboard service supporting multiple boards per game, per-
   board reset schedules (daily, weekly, seasonal, all-time), global and friend-filtered queries,
   submit_score, query_range, query_around_player, and server-side score validation before
   submission. Leaderboards are persisted to TiKV using ordered range scans for efficient top-N and
   rank-around-player queries. Scores are validated by anti-cheat rules (plausibility bounds, replay
   cross-check) before being recorded.
   - **Deps:** F-8.7.10
   - **Platform:** Server-side only. Clients query the service via RPC from all platforms.
2. **F-8.5.11** — A server-side achievement service supporting progress-based (increment toward
   goal) and boolean (unlocked or not) achievements with persistent TiKV storage, automatic sync to
   platform SDK achievement systems (Steam, PSN, Xbox, Apple Game Center), and query API for the
   player's achievement list. Progress persists across sessions so players see incremental progress
   (e.g., 45/100 enemies) toward unlocking each achievement.
   - **Deps:** F-8.7.10, F-14.5.1
   - **Platform:** Server-side with platform SDK bridges. Mobile clients sync with Game Center or
     Google Play Games Services.
3. **F-8.5.12** — A self-hosted cloud save service providing per-player storage slots, configurable
   quota per game, last-write-wins or merge-callback conflict resolution, rkyv zero-copy
   serialization of save payloads, and save/load/list/delete APIs. Saves are stored on Garage (S3-
   compatible blob storage) with metadata in TiKV for quota and slot tracking. Clients can save on
   one device and load on another, preserving progress across platforms.
   - **Deps:** F-8.7.10, F-8.7.11
   - **Platform:** Server-side storage. Client platforms access via the session RPC channel.
4. **F-8.5.13** — A telemetry service accepting fire-and-forget event submission from clients and
   servers with buffered batching, player session tracking (login/logout, playtime, progression
   events), aggregation pipelines for retention and balance analysis, and a GDPR opt-out flag with
   data deletion API. Telemetry events flow through Vector into Loki (logs) and a TiKV aggregation
   tier for time-series queries.
   - **Deps:** F-8.7.10, F-8.7.12
   - **Platform:** Server-side pipeline. All client platforms submit telemetry via the session
     channel unless the player opts out.

## Matchmaking Extensions

| ID       | Feature                                      |
|----------|----------------------------------------------|
| F-8.5.14 | Mid-Match Backfill Matchmaking               |
| F-8.5.15 | Abandoned Match Detection and Penalty       |
| F-8.5.16 | Extended Skill Rating System                 |
| F-8.5.17 | Specialized Matchmaking Queue Types          |

1. **F-8.5.14** — Backfill matchmaking replaces players who leave mid-match. A BackfillRequest
   specifies the vacated slot's skill range, role requirement, and maximum wait window. The
   matchmaker searches the queue for skill-compatible replacements and prompts them with a
   join-in-progress UI. Backfilled players receive a reduced rating change on match completion to
   account for partial participation.
   - **Deps:** F-8.5.9
   - **Platform:** Server-side. Clients on all platforms support backfill prompts.
2. **F-8.5.15** — Detect match abandonment (quitting before the match ends or failing to reconnect
   within a grace period) and apply escalating penalties: cooldown, rating penalty, temporary ban.
   Penalties are tracked per rolling window (e.g., last 20 matches) so occasional disconnects do not
   trigger harsh penalties while habitual leavers are filtered out. A team vote exemption lets the
   remaining team forgive the leaver (surrender after disconnect). Reconnection within the grace
   window cancels the penalty.
   - **Deps:** F-8.5.9, F-8.7.10
   - **Platform:** Server-side. Clients display penalty status in the UI.
3. **F-8.5.16** — An extended Glicko-2 skill rating system with per-player PlayerRating tracking
   rating, deviation, volatility, plus rating decay on inactivity, placement matches with wide
   initial deviation, per-game-mode ratings (one rating per queue), soft season resets that preserve
   relative ordering, anti-smurf heuristics, and display rank tier mapping (Bronze, Silver, Gold,
   Platinum, Diamond, Master, Grandmaster) surfaced to the player.
   - **Deps:** F-8.5.9, F-8.7.10
   - **Platform:** Server-side rating storage. Display rank badges shown on all client platforms.
4. **F-8.5.17** — Multiple specialized matchmaking queues: Casual (loose skill, fast queue),
   Competitive (strict skill, veto/accept flow, placement matches, season), Raid (role-based filling
   with tank/healer/DPS requirements, group composition checks), Custom (private lobby with manual
   invite), and Leaver (penalty pool for players with high abandon rates). Each queue has its own
   matching rules, timeout, and reward structure.
   - **Deps:** F-8.5.9, F-8.5.15
   - **Platform:** Server-side queue logic. Client UI presents queue choices per game mode.

## Platform Integration

| ID       | Feature                                        |
|----------|------------------------------------------------|
| F-8.5.18 | Platform Store and Overlay Integration        |

1. **F-8.5.18** — A StoreService abstraction for querying products, initiating purchases, restoring
   previous purchases, and verifying entitlements across platform stores (Steam, PSN, Xbox Live,
   Nintendo eShop, Epic Games Store, App Store, Google Play). The service detects when the platform
   overlay (e.g., Steam overlay, PSN purchase dialog) is active and pauses game input so the overlay
   receives controller and keyboard events without interference. Overlay dismissal resumes game
   input.
   - **Deps:** F-14.5.6
   - **Platform:** Delegates to per-platform SDKs via F-14.5.x platform services.
