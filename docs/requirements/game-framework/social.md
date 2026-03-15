# R-13.13 — Social and Guild System Requirements

## Guild

### R-13.13.1a Guild CRUD and Membership

The engine **SHALL** support guild creation with name and emblem, joining via invitation or
application, member removal (kick), and guild dissolution with founder confirmation and a cooldown
period.

- **Derived from:** [F-13.13.1a](../../features/game-framework/social.md)
- **Rationale:** Core guild lifecycle operations must exist independently so membership management
  is decoupled from ranking, leveling, and UI concerns.
- **Verification:** Create a guild and verify it appears in the guild list. Invite a player and
  verify they can accept and join. Kick a member and verify they are removed. Initiate dissolution
  and verify the cooldown period enforces a delay before the guild is destroyed.

### R-13.13.1b Guild Rank and Permission System

The engine **SHALL** provide a hierarchical rank system where each rank defines configurable
permissions (invite, kick, promote, demote, guild bank access, event start) and rank assignments are
enforced on every privileged action.

- **Derived from:** [F-13.13.1b](../../features/game-framework/social.md)
- **Rationale:** Per-rank permissions prevent unauthorized actions (e.g., junior members kicking
  others) and allow guild leaders to delegate responsibilities safely.
- **Verification:** Create custom ranks with different permission sets. Attempt to kick a member
  from a rank without kick permission and verify rejection. Promote a member and verify their
  permissions update immediately.

### R-13.13.1c Guild Leveling and Perks

The engine **SHALL** support guild leveling via XP earned from member activities, with data-driven
level thresholds and perk definitions that unlock at configured levels.

- **Derived from:** [F-13.13.1c](../../features/game-framework/social.md)
- **Rationale:** Guild leveling and perks reward collective engagement and provide long-term
  progression goals for the guild as a whole.
- **Verification:** Earn guild XP from a member quest completion and verify the guild XP total
  increases. Reach a level threshold and verify the configured perk unlocks. Verify perk definitions
  are loaded from data and can be modified without code changes.

### R-13.13.1d Guild Roster UI

The engine **SHALL** display a guild roster showing all members with online status, rank, level,
class, and last-login date, supporting sorting, filtering, and rank-appropriate context actions.

- **Derived from:** [F-13.13.1d](../../features/game-framework/social.md)
- **Rationale:** A responsive roster UI is essential for guild officers to manage membership and for
  members to coordinate with online guildmates.
- **Verification:** Open the roster and verify all member fields display correctly. Sort by rank and
  verify ordering. Filter by online status and verify only online members appear. Right-click a
  member and verify the context menu shows only permitted actions for the current user's rank.

### R-13.13.2 Guild Bank with Permissioned Access

The engine **SHALL** provide shared guild inventory and currency storage with multiple tabs unlocked
by guild level, per-rank deposit/withdraw permissions, transaction audit logs with timestamps and
member names, and daily withdrawal limits.

- **Derived from:** [F-13.13.2](../../features/game-framework/social.md)
- **Rationale:** Per-rank permissions and daily limits protect the guild bank from abuse, and audit
  logs provide accountability for dispute resolution.
- **Verification:** Deposit an item and gold into the guild bank; verify they appear in the bank UI.
  Attempt to withdraw from a tab the member's rank cannot access; verify rejection. Withdraw up to
  the daily limit; attempt one more withdrawal and verify rejection. Check the transaction log and
  verify it records depositor name, item, amount, and timestamp. Verify a new tab unlocks when the
  guild reaches the required level.

### R-13.13.3a Territory Claim System

The engine **SHALL** support guild territory claims via guild hall construction or control point
capture, with territory ownership granting resource bonuses and exclusive vendor access, enforcing
single-guild ownership per territory.

- **Derived from:** [F-13.13.3a](../../features/game-framework/social.md)
- **Rationale:** Territory ownership provides a tangible guild-level objective that drives
  competition and rewards coordinated guild activity.
- **Verification:** Construct a guild hall to claim territory; verify the territory is marked on the
  world map with the guild emblem. Verify resource generation bonuses apply to guild members in the
  territory. Attempt to claim an already-owned territory and verify rejection.

### R-13.13.3b Guild War Declaration and PvP Rules

The engine **SHALL** support formal guild war declarations requiring officer-rank permission,
enabling PvP between warring guild members in contested zones while leaving non-warring players
unaffected.

- **Derived from:** [F-13.13.3b](../../features/game-framework/social.md)
- **Rationale:** Formal war declarations scope PvP to consenting guilds, preventing unwanted
  player-killing while enabling structured guild-vs-guild conflict.
- **Verification:** Declare war on another guild; verify PvP is enabled between warring members in
  contested zones. Verify non-warring players cannot be attacked. Attempt to declare war from a rank
  without permission and verify rejection. End a war by mutual surrender and verify PvP is disabled.

### R-13.13.3c Siege Mechanics

The engine **SHALL** allow attackers to damage guild structures during scheduled war windows, with
defenders able to repair structures, and structure destruction revoking territory ownership.

- **Derived from:** [F-13.13.3c](../../features/game-framework/social.md)
- **Rationale:** Time-windowed sieges create predictable high-engagement events that guilds can plan
  around, preventing 24/7 harassment of offline guilds.
- **Verification:** Attack a guild structure during a siege window; verify damage applies. Attempt
  to attack outside the war window; verify it is blocked. Repair a damaged structure and verify its
  health increases. Destroy a structure and verify territory ownership reverts.

### R-13.13.3d Guild Rankings and Leaderboards

The engine **SHALL** maintain server-wide guild leaderboards tracking conquest points, war wins, and
territory count, with seasonal resets and rewards for top-ranked guilds.

- **Derived from:** [F-13.13.3d](../../features/game-framework/social.md)
- **Rationale:** Leaderboards drive inter-guild competition and provide visible recognition for
  dominant guilds, sustaining long-term engagement.
- **Verification:** Win a guild war and verify conquest points update on the leaderboard. Verify
  territory count reflects current ownership. Trigger a seasonal reset and verify historical
  rankings are archived. Verify top-ranked guilds receive configured rewards.

## Social Features

### R-13.13.4 Friends List with Platform Integration

The engine **SHALL** maintain a persistent bi-directional friends list with online/offline status,
current activity display, direct messaging, block/ignore functionality, recently-played-with
tracking, and platform friend import from Steam, PlayStation, and Xbox APIs.

- **Derived from:** [F-13.13.4](../../features/game-framework/social.md)
- **Rationale:** Cross-platform friend import reduces onboarding friction by automatically surfacing
  existing social connections, and block/ignore protects players from unwanted communication.
- **Verification:** Send a friend request by name; accept on the recipient; verify both players'
  friend lists update. Verify online/offline status and current zone display update in real-time.
  Send a direct message and verify delivery. Block a player and verify all communication from them
  is hidden. Verify the recently-played-with list populates after a party session. Import platform
  friends and verify matching players appear in the friends list.

### R-13.13.5a Core Mail Send/Receive

The engine **SHALL** deliver asynchronous player-to-player mail with subject and body text,
server-side storage with configurable expiration, and notifications on login and arrival.

- **Derived from:** [F-13.13.5a](../../features/game-framework/social.md)
- **Rationale:** Asynchronous text mail enables offline communication between players without
  requiring both to be online simultaneously.
- **Verification:** Send a text-only mail to another player; verify the recipient receives it.
  Verify mail expires after the configured period. Log in and verify the new-mail notification
  displays. Delete a mail and verify it is removed from the inbox.

### R-13.13.5b Mail Attachments

The engine **SHALL** support item and currency attachments on mail, with sender inventory escrow,
COD (cash-on-delivery) requiring payment before collection, and cross-character mail within the same
account.

- **Derived from:** [F-13.13.5b](../../features/game-framework/social.md)
- **Rationale:** Item and currency attachments enable offline trading, and COD supports secure
  peer-to-peer transactions outside the auction house.
- **Verification:** Send mail with an item attachment; verify the item is removed from the sender's
  inventory and the recipient can collect it. Send COD mail; verify the recipient must pay before
  collecting attachments. Send cross-character mail within the same account; verify delivery.

### R-13.13.5c System Mail

The engine **SHALL** generate automated system mail for auction results, guild invitations, GM
notifications, and event rewards, using a reserved sender identity with guaranteed offline delivery.

- **Derived from:** [F-13.13.5c](../../features/game-framework/social.md)
- **Rationale:** System mail provides a reliable delivery channel for automated game events that
  must reach the player regardless of online status.
- **Verification:** Trigger an auction completion and verify a system mail arrives with the correct
  item attachment. Send a GM notification and verify it arrives with the reserved sender identity.
  Verify system mail cannot be replied to.

### R-13.13.6a Core Chat Infrastructure

The engine **SHALL** support simultaneous text chat channels (global, zone, trade, party, guild,
whisper, LFG) with dynamic join/leave, per-player rate limiting, scrollable history persisting
across zone transitions, and per-channel color and notification configuration.

- **Derived from:** [F-13.13.6a](../../features/game-framework/social.md)
- **Rationale:** Multiple simultaneous channels separate distinct communication contexts (trade from
  combat from social), and rate limiting prevents chat flooding.
- **Verification:** Send a message in global chat; verify it is visible server-wide. Send a zone
  message; change zones and verify it is no longer visible. Send messages exceeding the rate limit
  and verify throttling. Scroll chat history after a zone transition and verify prior messages are
  retained.

### R-13.13.6b Chat Content Features

The engine **SHALL** support item linking with hover tooltips in chat messages, profanity filtering
with configurable blacklists, and spam detection for repeated messages.

- **Derived from:** [F-13.13.6b](../../features/game-framework/social.md)
- **Rationale:** Item linking enables trade communication, and profanity/spam filtering maintains a
  healthy chat environment without requiring constant moderator presence.
- **Verification:** Link an item in trade chat; hover over the link and verify the tooltip displays
  item stats and rarity. Send profanity and verify filtering replaces the content. Send repeated
  messages rapidly and verify spam detection triggers.

### R-13.13.6c Custom Player-Created Channels

The engine **SHALL** support player-created named chat channels with optional password protection,
owner and moderator roles with kick and mute privileges, and session-scoped lifetime.

- **Derived from:** [F-13.13.6c](../../features/game-framework/social.md)
- **Rationale:** Custom channels allow players to organize private communication for raids, events,
  or social groups beyond the built-in channel types.
- **Verification:** Create a custom channel with a password; verify unauthorized players cannot
  join. Assign a moderator role and verify they can kick and mute members. Verify the channel is
  destroyed when the last member leaves.

### R-13.13.7 Emote and Social Action System

The engine **SHALL** trigger character emotes via chat commands or emote wheel UI, playing looping
or one-shot animations through the animation state machine, supporting paired emotes with target
acceptance, unlockable custom emotes, and location restrictions (no emotes in combat).

- **Derived from:** [F-13.13.7](../../features/game-framework/social.md)
- **Rationale:** Emotes enable non-verbal social expression that enhances multiplayer atmosphere,
  and paired emotes create cooperative social moments between players.
- **Verification:** Type /dance and verify the dance animation loops. Type /wave and verify a
  one-shot wave animation plays. Initiate a paired emote (handshake); verify the target receives an
  acceptance prompt and both characters synchronize on accept. Attempt an emote during combat and
  verify it is blocked. Unlock a custom emote via achievement and verify it appears in the emote
  wheel.

### R-13.13.8 Player Character Inspection

The engine **SHALL** allow players to inspect another character's equipped gear, stats, talents,
achievements, guild, and title through a read-only character panel, with configurable privacy
settings (public, friends-only, guild-only, private) and range-limited access.

- **Derived from:** [F-13.13.8](../../features/game-framework/social.md)
- **Rationale:** Inspection enables social comparison and group composition evaluation without
  requiring external tools, and privacy settings let players control their exposure.
- **Verification:** Inspect a nearby player and verify their equipment, stats, and talents display
  correctly. Set privacy to friends-only; verify a non-friend cannot inspect. Set privacy to
  private; verify no one can inspect. Move out of visual range and attempt inspection; verify it is
  blocked. Hover over an inspected item and verify the tooltip displays.

### R-13.13.9 Dungeon and Group Finder with Role Queuing

The engine **SHALL** queue players for instanced content by role (tank, healer, DPS, support),
compose groups meeting role requirements via the matchmaker, estimate wait times, teleport matched
groups to instance entrances, pool players cross-shard, apply deserter penalties for early leavers,
and grant bonus rewards to in-demand roles.

- **Derived from:** [F-13.13.9](../../features/game-framework/social.md)
- **Rationale:** Automated group composition by role reduces manual group-forming friction,
  cross-shard pooling shortens queue times, and deserter penalties discourage abandonment.
- **Verification:** Queue as a tank and verify a group is composed with the required roles. Verify
  the group is teleported to the instance entrance on match. Queue cross-shard and verify players
  from different shards are matched. Leave an instance early and verify the deserter debuff prevents
  re-queuing for the penalty duration. Queue as an in-demand role and verify bonus rewards are
  granted on completion. Verify wait time estimates display in the group finder UI.

### R-13.13.10a Arena System

The engine **SHALL** provide instanced small-team PvP arenas (2v2, 3v3, 5v5 deathmatch) with
per-bracket rating, time limits, queue-from-anywhere, and teleport on match.

- **Derived from:** [F-13.13.10a](../../features/game-framework/social.md)
- **Rationale:** Small-team arenas provide a focused, skill-intensive competitive mode that is quick
  to queue and play, forming the core of ranked PvP.
- **Verification:** Queue for a 3v3 arena; verify players are matched and teleported to the arena.
  Win a match and verify the bracket rating increases. Verify a match ends at the time limit with
  the highest-health team winning.

### R-13.13.10b Battleground System

The engine **SHALL** provide larger objective-based PvP instances with capture-the-flag, point
control, and payload modes, balanced team composition, and individual performance tracking.

- **Derived from:** [F-13.13.10b](../../features/game-framework/social.md)
- **Rationale:** Objective-based battlegrounds provide varied PvP gameplay beyond deathmatch,
  rewarding teamwork and strategic play.
- **Verification:** Queue for a battleground; verify teams are composed with balanced sizes. Verify
  objective scoring (flag captures, point control) tracks correctly. Verify individual performance
  metrics display on the scoreboard.

### R-13.13.10c PvP Rating and Seasonal Rewards

The engine **SHALL** maintain skill-based ratings (Elo/Glicko) for arenas and battlegrounds, with
seasonal leaderboard resets, placement matches, and exclusive end-of-season rewards for top-ranked
players.

- **Derived from:** [F-13.13.10c](../../features/game-framework/social.md)
- **Rationale:** Seasonal resets and exclusive rewards sustain long-term competitive engagement by
  giving players recurring goals and visible prestige.
- **Verification:** Complete placement matches and verify an initial rating is assigned. Verify
  seasonal leaderboard updates with each match result. Trigger a season reset and verify rewards are
  distributed to top-ranked players and ratings reset.

### R-13.13.10d PvP Stat Normalization

The engine **SHALL** optionally normalize player stats in rated PvP matches using
bracket-appropriate templates, toggled per bracket or mode by server configuration, without
affecting cosmetic appearance.

- **Derived from:** [F-13.13.10d](../../features/game-framework/social.md)
- **Rationale:** Stat normalization ensures fair matches where skill determines outcomes rather than
  gear progression, lowering the barrier to competitive PvP entry.
- **Verification:** Enable stat normalization for a bracket; verify player stats are overridden with
  the template values. Verify cosmetic appearance is unaffected. Disable normalization and verify
  original stats are restored.

## Non-Functional Requirements

### R-13.13.NF1 Maximum Guild Roster Size

The engine **SHALL** support guilds with at least 1,000 members without degrading roster UI display,
permission checks, or guild XP calculation performance.

- **Derived from:** F-13.13.1a, F-13.13.1b, F-13.13.1d
- **Rationale:** Large-scale guilds in MMOs commonly exceed 500 members; the system must remain
  responsive at scale for roster browsing and administrative operations.
- **Verification:** Create a guild with 1,000 members. Open the roster UI and verify it renders
  within 1 frame. Perform a permission check (can member X kick member Y) and verify it completes in
  under 0.1 ms. Trigger guild XP recalculation and verify it completes within 10 ms.

### R-13.13.NF2 Maximum Friends List Size

The engine **SHALL** support friends lists of at least 500 entries per player with real-time online
status updates delivered within 2 seconds of a friend's status change.

- **Derived from:** F-13.13.4
- **Rationale:** Players with large social networks expect prompt status updates; delayed
  online/offline notifications reduce the utility of the friends list for coordinating play.
- **Verification:** Add 500 friends to a player's list. Toggle one friend's online status and
  measure time until the friends list UI reflects the change. Verify the update arrives within 2
  seconds.

### R-13.13.NF3 Chat Message Throughput

The engine **SHALL** support at least 100 incoming chat messages per second across all channels
without dropping messages, exceeding 1 ms of main-thread processing per message batch, or degrading
chat UI scroll performance.

- **Derived from:** F-13.13.6a
- **Rationale:** High-traffic global and trade channels in populated MMO servers generate sustained
  message rates that must not impact game performance or message delivery reliability.
- **Verification:** Simulate 100 incoming chat messages per second for 60 seconds. Verify no
  messages are dropped, main-thread processing per batch stays under 1 ms, and the chat UI scrolls
  smoothly at 60 fps.
