# User Stories — Social and Guild Systems (13.13)

## Guild Management

| ID            | Persona              |
|---------------|----------------------|
| US-13.13.1a.1 | game designer (P-5)  |
| US-13.13.1b.1 | game designer (P-5)  |
| US-13.13.1c.1 | player (P-23)        |
| US-13.13.1d.1 | player (P-23)        |
| US-13.13.2.1  | game designer (P-5)  |
| US-13.13.2.2  | player (P-23)        |

1. **US-13.13.1a.1** — **As a** game designer (P-5), **I want** guild CRUD with name, emblem,
   invitation, and dissolution, **so that** player organizations are data-driven.
2. **US-13.13.1b.1** — **As a** game designer (P-5), **I want** hierarchical ranks with configurable
   permissions (invite, kick, bank access), **so that** guild governance is flexible.
3. **US-13.13.1c.1** — [game-specific] **As a** player (P-23), **I want** guild XP from member
   activities unlocking perks at level thresholds, **so that** guilds grow together.
4. **US-13.13.1d.1** — [game-specific] **As a** player (P-23), **I want** a guild roster with online
   status, rank, level, and last-login, **so that** I see who is active.
5. **US-13.13.2.1** — **As a** game designer (P-5), **I want** shared guild storage with per-rank
   permissions and transaction logs, **so that** guild inventory is auditable.
6. **US-13.13.2.2** — [game-specific] **As a** player (P-23), **I want** daily withdrawal limits on
   guild bank, **so that** no member can drain shared resources.

## Territory and Warfare

| ID            | Persona              |
|---------------|----------------------|
| US-13.13.3a.1 | game designer (P-5)  |
| US-13.13.3b.1 | game designer (P-5)  |
| US-13.13.3c.1 | game developer (P-15)|
| US-13.13.3d.1 | player (P-23)        |

1. **US-13.13.3a.1** — [game-specific] **As a** game designer (P-5), **I want** territory claim via
   guild halls or control points with resource bonuses, **so that** territory ownership matters.
2. **US-13.13.3b.1** — [game-specific] **As a** game designer (P-5), **I want** formal war
   declarations enabling PvP between warring guilds, **so that** conflict is opt-in.
3. **US-13.13.3c.1** — **As a** game developer (P-15), **I want** siege mechanics with scheduled war
   windows and structure damage, **so that** territory conflict has clear rules.
4. **US-13.13.3d.1** — [game-specific] **As a** player (P-23), **I want** server-wide leaderboards
   tracking conquest points and territory count, **so that** guild competition is visible.

## Social Features

| ID             | Persona              |
|----------------|----------------------|
| US-13.13.4.1   | player (P-23)        |
| US-13.13.5a.1  | player (P-23)        |
| US-13.13.5b.1  | player (P-23)        |
| US-13.13.5c.1  | server administrator (P-22)|
| US-13.13.6a.1  | game designer (P-5)  |
| US-13.13.6b.1  | game developer (P-15)|
| US-13.13.6c.1  | player (P-23)        |
| US-13.13.7.1   | player (P-23)        |
| US-13.13.8.1   | player (P-23)        |

1. **US-13.13.4.1** — **As a** player (P-23), **I want** a friends list with online/offline status,
   current activity, and direct messaging, **so that** I stay connected with friends.
2. **US-13.13.5a.1** — **As a** player (P-23), **I want** asynchronous player-to-player mail with
   read/unread status, **so that** I communicate with offline players.
3. **US-13.13.5b.1** — [game-specific] **As a** player (P-23), **I want** mail attachments for items
   and currency with COD support, **so that** I trade asynchronously.
4. **US-13.13.5c.1** — **As a** server administrator (P-22), **I want** system mail for automated
   notifications with guaranteed delivery, **so that** server events reach all players.
5. **US-13.13.6a.1** — **As a** game designer (P-5), **I want** multiple chat channels (global,
   zone, party, guild, whisper) with rate limiting, **so that** communication is organized.
6. **US-13.13.6b.1** — **As a** game developer (P-15), **I want** item linking in chat with hover
   tooltips and profanity/spam filtering, **so that** chat content is rich and safe.
7. **US-13.13.6c.1** — [game-specific] **As a** player (P-23), **I want** to create custom chat
   channels with password protection and moderation, **so that** I organize private conversations.
8. **US-13.13.7.1** — [game-specific] **As a** player (P-23), **I want** character emotes (dance,
   wave, sit) with animations and paired interactions, **so that** I express myself socially.
9. **US-13.13.8.1** — [game-specific] **As a** player (P-23), **I want** to inspect another player's
   gear, stats, and talents with privacy settings, **so that** I evaluate group composition.

## Matchmaking and PvP

| ID             | Persona              |
|----------------|----------------------|
| US-13.13.9.1   | game designer (P-5)  |
| US-13.13.9.2   | player (P-23)        |
| US-13.13.10a.1 | game designer (P-5)  |
| US-13.13.10b.1 | game designer (P-5)  |
| US-13.13.10c.1 | player (P-23)        |
| US-13.13.10d.1 | game designer (P-5)  |

1. **US-13.13.9.1** — **As a** game designer (P-5), **I want** role-based group queuing (tank,
   healer, DPS) with cross-shard pooling, **so that** matchmaking is configurable.
2. **US-13.13.9.2** — [game-specific] **As a** player (P-23), **I want** estimated wait times per
   role and bonus rewards for in-demand roles, **so that** I choose roles wisely.
3. **US-13.13.10a.1** — [game-specific] **As a** game designer (P-5), **I want** instanced arena PvP
   (2v2, 3v3, 5v5) with rating per bracket, **so that** small-team competitive play exists.
4. **US-13.13.10b.1** — [game-specific] **As a** game designer (P-5), **I want** objective-based
   battlegrounds (CTF, point control, payload) with team composition, **so that** large-scale PvP is
   varied.
5. **US-13.13.10c.1** — [game-specific] **As a** player (P-23), **I want** seasonal ratings with
   exclusive rewards for top ranks, **so that** PvP competition has ongoing incentives.
6. **US-13.13.10d.1** — **As a** game designer (P-5), **I want** optional PvP stat normalization
   that overrides gear advantages in rated matches, **so that** skill determines outcomes.
