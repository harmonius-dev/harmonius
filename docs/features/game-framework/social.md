# 13.13 — Social and Guild Systems

## Guild

| ID         | Feature                             |
|------------|-------------------------------------|
| F-13.13.1a | Guild CRUD and Membership           |
| F-13.13.1b | Guild Rank and Permission System    |
| F-13.13.1c | Guild Leveling and Perks            |
| F-13.13.1d | Guild Roster UI                     |
| F-13.13.2  | Guild Storage and Bank              |
| F-13.13.3a | Territory Claim System              |
| F-13.13.3b | Guild War Declaration and PvP Rules |
| F-13.13.3c | Siege Mechanics                     |
| F-13.13.3d | Guild Rankings and Leaderboards     |

1. **F-13.13.1a** — Create, join, and disband player guilds. Guild creation includes name and emblem
   selection. Invitation is by player name or proximity. Players may join via invitation acceptance
   or application. Kick removes a member immediately. Guild dissolution requires founder
   confirmation with a cooldown period to prevent accidental disbandment.
   - **Deps:** F-8.2.1 (Property Replication), F-1.1.1 (ECS)
2. **F-13.13.1b** — Hierarchical rank system where each rank defines a configurable set of
   permissions: invite, kick, promote, demote, access guild bank, and start guild events. The
   founder holds the highest rank and can create, rename, reorder, and delete ranks. Rank
   assignments are stored per member and checked on every privileged action.
   - **Deps:** F-13.13.1a (Guild CRUD and Membership)
3. **F-13.13.1c** — Guild leveling with XP earned from member activities (quests, dungeons, PvP
   wins, crafting). Guild perks unlock at level thresholds: increased XP gain, larger roster
   capacity, guild mounts, and cosmetic rewards. XP contributions are tracked per member for
   recognition. Level thresholds and perk definitions are data-driven and configurable.
   - **Deps:** F-13.13.1a (Guild CRUD and Membership), F-13.7.6 (Currency)
4. **F-13.13.1d** — Guild roster UI displays all members with online/offline status, rank, level,
   class, and last-login date. The roster supports sorting by any column and filtering by rank or
   online status. Right-click context menu exposes rank-appropriate actions (promote, demote, kick,
   whisper). The roster renders responsively for guilds up to 1,000 members.
   - **Deps:** F-13.13.1a (Guild CRUD and Membership), F-13.13.1b (Guild Rank and Permission System)
5. **F-13.13.2** — Shared guild inventory and currency accessible by authorized guild members. The
   guild bank has multiple tabs (unlocked by guild level), each with configurable deposit/withdraw
   permissions per rank. Transaction logs record every deposit and withdrawal with timestamps and
   member names for audit. Guild gold storage is separate from the item bank. Daily withdrawal
   limits prevent single members from draining the bank. The guild bank UI mirrors the personal
   inventory layout with tab navigation and permission indicators.
   - **Deps:** F-13.13.1a (Guild CRUD and Membership), F-13.9.1 (Inventory Containers), F-13.7.6
     (Currency)
6. **F-13.13.3a** — Guilds claim territory by constructing guild halls or capturing control points
   on the world map. Territory ownership grants resource generation bonuses and exclusive vendor
   access. Only one guild may own a territory at a time. Territory is visible on the world map with
   guild emblem and boundary indicators.
   - **Deps:** F-13.13.1a (Guild CRUD and Membership), F-8.7.1 (World Sharding)
7. **F-13.13.3b** — Guilds declare war on other guilds through a formal declaration requiring
   officer-rank permission. Declared wars enable PvP between warring guild members in contested
   zones. Non-warring players are unaffected. Wars have a minimum duration and can end by mutual
   surrender or victory conditions. War status is visible in the guild UI and on nameplates.
   - **Deps:** F-13.13.1a (Guild CRUD and Membership), F-13.13.1b (Guild Rank and Permission System)
8. **F-13.13.3c** — Attackers may damage guild structures during scheduled war windows (time-limited
   periods). Siege weapons and abilities deal structure damage. Defenders may repair structures
   during and after sieges. Structure destruction revokes territory ownership. War windows are
   configured per server to ensure predictable engagement times.
   - **Deps:** F-13.13.3a (Territory Claim System), F-13.13.3b (Guild War Declaration and PvP Rules)
9. **F-13.13.3d** — Server-wide leaderboards track conquest points, war wins, and territory count
   per guild. Rankings update after each war conclusion and territory change. Seasonal resets
   archive historical rankings. Top-ranked guilds receive cosmetic rewards and titles. The
   leaderboard UI supports filtering by metric and time period.
   - **Deps:** F-13.13.3a (Territory Claim System), F-13.13.3b (Guild War Declaration and PvP Rules)

## Social Features

| ID          | Feature                         |
|-------------|---------------------------------|
| F-13.13.4   | Friends List and Social Graph   |
| F-13.13.5a  | Core Mail Send/Receive          |
| F-13.13.5b  | Mail Attachments                |
| F-13.13.5c  | System Mail                     |
| F-13.13.6a  | Core Chat Infrastructure        |
| F-13.13.6b  | Chat Content Features           |
| F-13.13.6c  | Custom Player-Created Channels  |
| F-13.13.7   | Emote and Social Action System  |
| F-13.13.8   | Player Inspection               |
| F-13.13.9   | Dungeon and Group Finder        |
| F-13.13.10a | Arena System                    |
| F-13.13.10b | Battleground System             |
| F-13.13.10c | PvP Rating and Seasonal Rewards |
| F-13.13.10d | PvP Stat Normalization          |

1. **F-13.13.4** — Persistent bi-directional friends list with online/offline status, current zone
   and activity display, and direct messaging. Friend requests are sent by player name and require
   acceptance. Block/ignore list hides all communication from blocked players. "Recently played
   with" list tracks recent party and group members. The friends list integrates with the platform
   friends system (Steam, PlayStation, Xbox) to import platform friends who also play the game.
   Social graph data is replicated through the networking system for cross-shard visibility.
   - **Deps:** F-8.2.1 (Property Replication), F-14.5.1 (Platform Services)
   - **Platform:** Platform friend import uses Steam Friends API, PSN Friends, Xbox Social APIs.
2. **F-13.13.5a** — Asynchronous player-to-player mail system. Players compose mail with recipient
   name, subject, and body text. Mail is stored server-side with configurable expiration (30 days
   default). Recipients receive notification on login and when new mail arrives during a session.
   Mail supports read/unread status, deletion, and return-to-sender for undeliverable mail.
   - **Deps:** F-8.7.7 (Cross-Shard Services)
3. **F-13.13.5b** — Mail may include item and currency attachments. Items are removed from the
   sender's inventory on send and held in escrow until collected. Currency attachments support gold
   and premium currency types. COD (cash-on-delivery) mail requires the recipient to pay before
   collecting attachments. Cross-character mail within the same account is supported for item
   transfers.
   - **Deps:** F-13.13.5a (Core Mail Send/Receive), F-13.9.1 (Inventory), F-13.7.6 (Currency)
4. **F-13.13.5c** — Automated server-generated mail for auction house results, guild invitations, GM
   notifications, and event rewards. System mail uses a reserved sender identity and cannot be
   replied to. System mail may include item and currency attachments using the same attachment
   system (F-13.13.5b). Delivery is guaranteed even if the recipient is offline.
   - **Deps:** F-13.13.5a (Core Mail Send/Receive), F-13.13.5b (Mail Attachments)
5. **F-13.13.6a** — Multiple simultaneous text chat channels: global (server-wide), zone (current
   area), trade, party, guild, whisper (private 1:1), and LFG (looking-for-group). Players join and
   leave channels dynamically. Chat messages are rate-limited per player (F-8.8.5) to prevent
   flooding. Chat history is scrollable and persists across zone transitions within a session.
   Channel colors and notification sounds are configurable per channel.
   - **Deps:** F-8.2.1 (Property Replication), F-8.8.5 (Rate Limiting)
   - **Platform:** Console platforms may require using platform-native text chat for compliance.
6. **F-13.13.6b** — Item linking embeds item data in chat messages with hover-tooltip display
   showing item stats, rarity, and icon. Profanity filtering replaces or blocks messages containing
   blacklisted terms. Spam filtering detects repeated messages and excessive caps. Filter lists are
   data-driven and updatable without a client patch. Filtered messages are logged for moderation
   review.
   - **Deps:** F-13.13.6a (Core Chat Infrastructure), F-13.9.1 (Inventory)
7. **F-13.13.6c** — Players create named chat channels with optional password protection. Channel
   creators become the channel owner with moderator privileges: kick, mute, and transfer ownership.
   Additional moderator roles can be assigned to other members. Custom channels persist for the
   session duration and are destroyed when the last member leaves.
   - **Deps:** F-13.13.6a (Core Chat Infrastructure)
8. **F-13.13.7** — Character emotes (dance, wave, sit, laugh, cry, cheer, bow, sleep) with
   synchronized animations and optional audio. Emotes are triggered via chat commands (/dance) or an
   emote wheel UI. Emotes play looping or one-shot animations through the animation state machine
   (F-9.4.1). Paired emotes synchronize between two characters (handshake, high-five) when the
   target accepts. Custom emotes are unlockable rewards from achievements, events, or purchases.
   Emotes respect location restrictions (no dancing in combat). The emote system feeds into social
   features — nearby players see emote chat messages.
   - **Deps:** F-9.4.1 (Animation State Machine), F-13.13.6a (Core Chat Infrastructure)
9. **F-13.13.8** — Inspect another player's character sheet: equipped gear, stats, talent build,
   achievements, guild, and title. Inspection opens a read-only version of the target's character
   panel. Privacy settings allow players to restrict inspection (public, friends-only, guild-only,
   private). Inspection range is limited to visual proximity. The inspection UI displays item
   tooltips on hover and links to armory-style external views if supported. Used for evaluating
   group composition, admiring gear, and social comparison.
   - **Deps:** F-8.2.1 (Property Replication), F-13.9.1 (Inventory)
10. **F-13.13.9** — Queue for instanced content by role (tank, healer, DPS, support). The matchmaker
    (F-8.5.2) composes groups meeting role requirements, estimates wait times per role, and
    teleports the group to the instance entrance on match. Cross-shard queuing pools players from
    multiple servers for faster matches. Deserter penalties temporarily block re-queuing for players
    who leave instances early. In-demand roles receive bonus rewards (extra currency, loot chance).
    The group finder UI displays available content filtered by level, difficulty, and role
    availability.
    - **Deps:** F-8.5.2 (Matchmaking), F-8.7.1 (World Sharding), F-13.2.4 (Instanced Zones)
11. **F-13.13.10a** — Instanced small-team deathmatch PvP: 2v2, 3v3, and 5v5 brackets. Each arena
    match has a time limit and win condition (eliminate all opponents or highest health at timeout).
    Players queue from anywhere and are teleported to the arena on match. Arena rating is tracked
    per bracket and determines matchmaking and seasonal ranking.
    - **Deps:** F-8.5.2 (Matchmaking), F-8.7.1 (World Sharding), F-13.10.1 (Abilities)
12. **F-13.13.10b** — Larger objective-based PvP instances: capture the flag, point control, and
    payload modes. Each mode defines unique score tracking, win conditions, and map layouts. Teams
    are composed by the matchmaker with balanced sizes (10v10, 15v15). Individual performance
    metrics (kills, objective contributions, healing) are tracked for scoreboard display and reward
    calculation.
    - **Deps:** F-8.5.2 (Matchmaking), F-8.7.1 (World Sharding), F-13.10.1 (Abilities)
13. **F-13.13.10c** — Skill-based rating system using Elo/Glicko for arenas and battlegrounds.
    Seasonal leaderboards reset periodically (configurable cadence). Top-ranked players at season
    end receive exclusive rewards: titles, cosmetics, and mounts. Rating history is preserved for
    career statistics. Placement matches determine initial seasonal rating.
    - **Deps:** F-13.13.10a (Arena System), F-13.13.10b (Battleground System)
14. **F-13.13.10d** — Optional PvP-specific stat templates that normalize gear advantages in rated
    matches. When enabled, player stats are overridden with bracket-appropriate templates so skill
    determines outcomes rather than gear progression. Normalization is toggled per bracket or mode
    by server configuration. Cosmetic appearance is unaffected.
    - **Deps:** F-13.13.10a (Arena System), F-13.13.10b (Battleground System)
