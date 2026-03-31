# R-13.13 — Social and Guild System Requirements

## Guild Management

1. **R-13.13.1** — The engine **SHALL** support guild CRUD (create, join, disband) with name and
   emblem selection, hierarchical rank system with configurable permissions (invite, kick, promote,
   bank access), guild XP from member activities, and per-threshold perk unlocks.
   - **Rationale:** Configurable ranks and permissions enable flexible governance without hardcoded
     roles.
   - **Verification:** Create a guild and verify name and emblem. Assign a rank with invite-only
     permission and verify the member can invite but not kick. Contribute XP and verify guild level
     advances.

2. **R-13.13.2** — The engine **SHALL** provide shared guild storage with multiple tabs, per-rank
   deposit/withdraw permissions, transaction logs, daily withdrawal limits, and separate currency
   storage.
   - **Rationale:** Transaction logs and withdrawal limits prevent abuse while enabling shared
     resources.
   - **Verification:** Deposit an item and verify the transaction is logged. Attempt withdrawal from
     a rank without permission and verify rejection. Exceed daily limit and verify rejection.

## Territory and Warfare

3. **R-13.13.3** — The engine **SHALL** support territory claim via structures or control points
   with ownership bonuses, formal war declarations enabling PvP, scheduled siege windows with
   structure damage, and server-wide leaderboards.
   - **Rationale:** Structured warfare with time windows ensures predictable engagement periods.
   - **Verification:** Claim territory and verify ownership bonus. Declare war and verify PvP
     enabled between members. Attack a structure during siege window and verify damage. Verify
     leaderboard updates after territory change.

## Social Features

4. **R-13.13.4** — The engine **SHALL** provide a bi-directional friends list with online/offline
   status, current activity, direct messaging, block/ignore, and platform friend import (F-14.5.1).
   - **Rationale:** Cross-platform friend import reduces friction for players already connected on
     platform services.
   - **Verification:** Send a friend request and verify acceptance creates a bi-directional link.
     Block a player and verify all communication is hidden. Import platform friends and verify they
     appear in the list.

5. **R-13.13.5** — The engine **SHALL** support asynchronous player mail with subject, body,
   item/currency attachments, COD (cash-on-delivery), expiration, and automated system mail for
   server notifications with guaranteed delivery.
   - **Rationale:** Async mail enables communication and trading with offline players.
   - **Verification:** Send mail with an item attachment and verify item removed from sender.
     Recipient collects and verify item added. Send COD mail and verify recipient must pay before
     collecting. Send system mail to offline player and verify delivery on login.

6. **R-13.13.6** — The engine **SHALL** support multiple simultaneous chat channels (global, zone,
   party, guild, whisper, custom), rate limiting, item linking with hover tooltips, profanity/spam
   filtering, and player-created channels with moderation.
   - **Rationale:** Organized channels with filtering maintain healthy communication.
   - **Verification:** Send messages on multiple channels and verify isolation. Link an item and
     verify tooltip on hover. Trigger rate limit and verify message throttled. Create a custom
     channel with password and verify access control.

7. **R-13.13.7** — The engine **SHALL** support character emotes with synchronized animations,
   optional audio, paired emote interactions, and unlock-gating from achievements or purchases.
   - **Rationale:** Emotes enable social expression integrated with animation and unlock systems.
   - **Verification:** Trigger an emote and verify animation and audio play. Initiate a paired emote
     and verify synchronization when the target accepts. Attempt a locked emote and verify
     rejection.

8. **R-13.13.8** — The engine **SHALL** support inspecting another player's character sheet (gear,
   stats, talents, guild, title) with configurable privacy settings (public, friends-only, private)
   and proximity range limits.
   - **Rationale:** Inspection enables group composition evaluation while privacy protects player
     choice.
   - **Verification:** Inspect a public player within range and verify all fields displayed. Attempt
     inspecting a private player and verify rejection. Move out of range and verify inspection
     unavailable.

## Matchmaking and PvP

9. **R-13.13.9** — The engine **SHALL** support role-based group queuing with cross-shard pooling,
   estimated wait times, and deserter penalties for early leavers, integrated with the matchmaking
   system (F-8.5.2).
   - **Rationale:** Cross-shard pooling reduces wait times; deserter penalties discourage
     abandonment.
   - **Verification:** Queue as a healer and verify estimated wait time displayed. Match a group and
     verify role requirements met. Leave early and verify deserter penalty applied.

10. **R-13.13.10** — The engine **SHALL** support instanced PvP (arena 2v2/3v3/5v5, objective-based
    battlegrounds) with skill-based rating (Elo/Glicko), seasonal leaderboards with rewards, and
    optional stat normalization overriding gear advantages.
    - **Rationale:** Skill-based rating ensures fair matchmaking; normalization lets skill determine
      outcomes.
    - **Verification:** Complete 10 arena matches and verify rating changes. Enable stat
      normalization and verify gear stats are overridden. End a season and verify top-ranked players
      receive rewards.
