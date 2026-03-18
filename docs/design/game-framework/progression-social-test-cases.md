# Progression and Social Systems Test Cases

Companion test cases for [progression-social.md](progression-social.md).

## Unit Tests

### TC-13.12.1b.1 XP Level Up

| # | Requirement |
|---|-------------|
| 1 | R-13.12.1b  |

1. **#1** — Level 9, XP 990/1000, award 20 XP
   - **Expected:** `level == 10`, `current_xp == 10`, stats grown per `StatGrowthTable`

### TC-13.12.2a.1 Talent Prerequisite Reject

| # | Requirement |
|---|-------------|
| 1 | R-13.12.2a  |

1. **#1** — Allocate tier-2 node without tier-1 parent allocated
   - **Expected:** `Err(PrerequisiteNotMet)`

### TC-13.12.2a.2 Talent Tier Gate

| # | Requirement |
|---|-------------|
| 1 | R-13.12.2a  |

1. **#1** — 3 points allocated, tier-2 gate requires 5
   - **Expected:** `Err(TierGateNotMet { required: 5, have: 3 })`

### TC-13.12.2b.1 Talent Allocation Success

| # | Requirement |
|---|-------------|
| 1 | R-13.12.2b  |

1. **#1** — Allocate valid node with 1 available point
   - **Expected:** `TalentState` updated, effect applied, `available_points -= 1`

### TC-13.12.2b.2 Talent Respec

| # | Requirement |
|---|-------------|
| 1 | R-13.12.2b  |

1. **#1** — Respec with 15 points allocated, cost 100 gold
   - **Expected:** All nodes cleared, 15 points refunded, effects removed, gold -= 100

### TC-13.12.NF1.1 Talent Validation Performance

| # | Requirement |
|---|-------------|
| 1 | R-13.12.NF1 |

1. **#1** — 100-node tree, sequential allocation of all nodes
   - **Expected:** p99 validation time < 1 ms

### TC-13.12.1d.1 Prestige Reset

| # | Requirement |
|---|-------------|
| 1 | R-13.12.1d  |

1. **#1** — Prestige at max level 60
   - **Expected:** `level == 1`, `prestige_level += 1`, permanent bonuses accumulated

### TC-13.12.1d.2 Prestige Cosmetics Preserved

| # | Requirement |
|---|-------------|
| 1 | R-13.12.1d  |

1. **#1** — Prestige with 3 cosmetic rewards unlocked
   - **Expected:** All 3 cosmetic rewards retained post-prestige

### TC-13.12.3a.1 Profession Recipe Unlock

| # | Requirement |
|---|-------------|
| 1 | R-13.12.3a  |

1. **#1** — Level profession from 14 to 15, threshold at 15
   - **Expected:** Recipe unlocked, accessible in crafting UI

### TC-13.12.3a.2 Profession Slot Limit

| # | Requirement |
|---|-------------|
| 1 | R-13.12.3a  |

1. **#1** — `max_slots == 2`, attempt to learn 3rd profession
   - **Expected:** `Err(SlotLimitReached)`

### TC-13.12.3b.1 Gathering Yield Scales

| # | Requirement |
|---|-------------|
| 1 | R-13.12.3b  |

1. **#1** — Gather at skill 10 vs skill 50
   - **Expected:** Skill-50 yield > skill-10 yield

### TC-13.12.3c.1 Crafting Level Gate

| # | Requirement |
|---|-------------|
| 1 | R-13.12.3c  |

1. **#1** — Attempt recipe requiring level 20 at level 15
   - **Expected:** `Err(LevelTooLow { required: 20, have: 15 })`

### TC-13.12.4.1 Station Type Gates

| # | Requirement |
|---|-------------|
| 1 | R-13.12.4   |

1. **#1** — Query forge recipes at AlchemyTable
   - **Expected:** Forge recipes absent from available list

### TC-13.12.5.1 Reputation Gain

| # | Requirement |
|---|-------------|
| 1 | R-13.12.5   |

1. **#1** — Complete quest awarding +500 rep to faction A
   - **Expected:** `FactionReputation.current_value += 500`

### TC-13.12.5.2 Reputation Asymmetric

| # | Requirement |
|---|-------------|
| 1 | R-13.12.5   |

1. **#1** — Gain 1000 rep with faction A, rival B loss_ratio 0.5
   - **Expected:** Faction B rep decreases by 500

### TC-13.12.5.3 Reputation Tier Gating

| # | Requirement |
|---|-------------|
| 1 | R-13.12.5   |

1. **#1** — Reach Honored standing with faction A
   - **Expected:** Honored-tier vendor items accessible

### TC-13.12.6a.1 Achievement Incremental

| # | Requirement |
|---|-------------|
| 1 | R-13.12.6a  |
| 2 | R-13.12.6a  |

1. **#1** — Kill 50 of 100 enemies for achievement
   - **Expected:** `current == 50`, `completed == false`
2. **#2** — Kill 50 more (total 100)
   - **Expected:** `completed == true`, timestamp recorded

### TC-13.12.NF2.1 Achievement Observer Budget

| # | Requirement |
|---|-------------|
| 1 | R-13.12.NF2 |

1. **#1** — 1000 achievement definitions, 100 events/frame
   - **Expected:** Observer evaluation < 0.1 ms/frame

### TC-13.12.6c.1 Achievement Platform Sync

| # | Requirement |
|---|-------------|
| 1 | R-13.12.6c  |

1. **#1** — Complete achievement with Steam mapping
   - **Expected:** `ISteamUserStats::SetAchievement` called

### TC-13.12.7.1 Enhancement Success

| # | Requirement |
|---|-------------|
| 1 | R-13.12.7   |

1. **#1** — Enhance +0 item at 100% success rate
   - **Expected:** `EnhancementLevel.level == 1`, stat bonus applied

### TC-13.12.7.2 Enhancement Failure

| # | Requirement |
|---|-------------|
| 1 | R-13.12.7   |

1. **#1** — Enhance at 10% rate, roll fails
   - **Expected:** Consequence applied per `FailureConsequence`

### TC-13.12.7.3 Protection Item

| # | Requirement |
|---|-------------|
| 1 | R-13.12.7   |

1. **#1** — Enhancement failure with protection item equipped
   - **Expected:** Item not destroyed; protection consumed

### TC-13.12.8a.1 Rarity Stat Ranges

| # | Requirement |
|---|-------------|
| 1 | R-13.12.8a  |

1. **#1** — Generate 1000 items per rarity tier
   - **Expected:** All stat values within configured min/max per tier

### TC-13.12.8b.1 Affix Count By Rarity

| # | Requirement |
|---|-------------|
| 1 | R-13.12.8b  |

1. **#1** — Generate items at each rarity
   - **Expected:** Affix counts match per-rarity config (e.g., Rare=2, Epic=3)

### TC-13.12.8c.1 Reroll Preserves Base

| # | Requirement |
|---|-------------|
| 1 | R-13.12.8c  |

1. **#1** — Re-roll Epic sword affixes
   - **Expected:** Base item type and rarity tier preserved; affixes changed

### TC-13.12.9.1 Set Bonus Thresholds

| # | Requirement |
|---|-------------|
| 1 | R-13.12.9   |

1. **#1** — Equip 2/4/6 set pieces
   - **Expected:** Bonuses activate at each threshold

### TC-13.12.9.2 Set Bonus Deactivate

| # | Requirement |
|---|-------------|
| 1 | R-13.12.9   |

1. **#1** — Unequip below 4-piece threshold
   - **Expected:** 4-piece bonus removed, 2-piece remains

### TC-13.12.10.1 Durability Drain

| # | Requirement |
|---|-------------|
| 1 | R-13.12.10  |

1. **#1** — Attack 10 times with weapon
   - **Expected:** `Durability.current` decreased per attack

### TC-13.12.10.2 Durability Zero Nonfunctional

| # | Requirement |
|---|-------------|
| 1 | R-13.12.10  |

1. **#1** — Reduce durability to 0%
   - **Expected:** Weapon deals 0 damage

### TC-13.12.10.3 Repair Restores

| # | Requirement |
|---|-------------|
| 1 | R-13.12.10  |

1. **#1** — Repair from 0% to 100%
   - **Expected:** `current == max`, full stat restoration

### TC-13.13.1a.1 Guild Create

| # | Requirement |
|---|-------------|
| 1 | R-13.13.1a  |

1. **#1** — Create guild "TestGuild"
   - **Expected:** Entity with `Guild { name: "TestGuild" }` component

### TC-13.13.1a.2 Guild Invite Accept

| # | Requirement |
|---|-------------|
| 1 | R-13.13.1a  |

1. **#1** — Invite player B, B accepts
   - **Expected:** `GuildMember` added to player B entity

### TC-13.13.1a.3 Guild Kick

| # | Requirement |
|---|-------------|
| 1 | R-13.13.1a  |

1. **#1** — Kick member B (with permission)
   - **Expected:** `GuildMember` removed from player B

### TC-13.13.1a.4 Guild Dissolve Cooldown

| # | Requirement |
|---|-------------|
| 1 | R-13.13.1a  |

1. **#1** — Dissolve guild, attempt create within cooldown
   - **Expected:** `Err(CooldownActive)`

### TC-13.13.1b.1 Guild Permission Check

| # | Requirement |
|---|-------------|
| 1 | R-13.13.1b  |

1. **#1** — Rank without `can_kick`, attempt kick
   - **Expected:** `Err(InsufficientPermission)`

### TC-13.13.1b.2 Guild Rank Assignment

| # | Requirement |
|---|-------------|
| 1 | R-13.13.1b  |

1. **#1** — Assign Officer rank to member
   - **Expected:** `GuildMember.rank` updated, permissions match Officer

### TC-13.13.1c.1 Guild XP Accumulation

| # | Requirement |
|---|-------------|
| 1 | R-13.13.1c  |

1. **#1** — Complete quest awarding 100 guild XP
   - **Expected:** `GuildLevel.current_xp += 100`

### TC-13.13.1c.2 Guild Perk Unlock

| # | Requirement |
|---|-------------|
| 1 | R-13.13.1c  |

1. **#1** — Reach guild level threshold for perk
   - **Expected:** Perk added to `active_perks`

### TC-13.13.NF1.1 Guild Roster 1000

| # | Requirement |
|---|-------------|
| 1 | R-13.13.NF1 |

1. **#1** — 1000-member guild, render roster
   - **Expected:** Renders within 1 frame (< 16.67 ms)

### TC-13.13.2.1 Guild Bank Permission

| # | Requirement |
|---|-------------|
| 1 | R-13.13.2   |

1. **#1** — Withdraw from tab requiring Officer rank as Member
   - **Expected:** `Err(InsufficientRank)`

### TC-13.13.2.2 Guild Bank Daily Limit

| # | Requirement |
|---|-------------|
| 1 | R-13.13.2   |

1. **#1** — Exhaust daily withdrawal limit, attempt one more
   - **Expected:** `Err(DailyLimitReached)`

### TC-13.13.2.3 Guild Bank Audit Log

| # | Requirement |
|---|-------------|
| 1 | R-13.13.2   |

1. **#1** — Perform 100 deposit/withdraw transactions
   - **Expected:** `transaction_log.len() == 100`

### TC-13.13.3a.1 Territory Claim

| # | Requirement |
|---|-------------|
| 1 | R-13.13.3a  |

1. **#1** — Claim unclaimed territory
   - **Expected:** `TerritoryOwnership.owning_guild == guild_entity`

### TC-13.13.3a.2 Territory Exclusive

| # | Requirement |
|---|-------------|
| 1 | R-13.13.3a  |

1. **#1** — Two guilds claim same territory
   - **Expected:** Only first succeeds; second gets `Err(AlreadyClaimed)`

### TC-13.13.3b.1 War Declaration

| # | Requirement |
|---|-------------|
| 1 | R-13.13.3b  |

1. **#1** — Declare war between guilds A and B
   - **Expected:** PvP enabled between members of A and B

### TC-13.13.3b.2 War Non-Warring Safe

| # | Requirement |
|---|-------------|
| 1 | R-13.13.3b  |

1. **#1** — Non-warring guild member attacks warring guild
   - **Expected:** PvP blocked, damage = 0

### TC-13.13.3c.1 Siege Window

| # | Requirement |
|---|-------------|
| 1 | R-13.13.3c  |

1. **#1** — Attack territory outside scheduled war window
   - **Expected:** `Err(OutsideWindow)`

### TC-13.13.3d.1 Leaderboard Update

| # | Requirement |
|---|-------------|
| 1 | R-13.13.3d  |

1. **#1** — Win guild war
   - **Expected:** Guild leaderboard points increase

### TC-13.13.4.1 Friend Add Remove

| # | Requirement |
|---|-------------|
| 1 | R-13.13.4   |

1. **#1** — Add friend B, then block B
   - **Expected:** Both lists update; B hidden from A's view

### TC-13.13.NF2.1 Friend Status Latency

| # | Requirement |
|---|-------------|
| 1 | R-13.13.NF2 |

1. **#1** — Toggle online status
   - **Expected:** Friends list update received within 2 seconds

### TC-13.13.5a.1 Mail Send Receive

| # | Requirement |
|---|-------------|
| 1 | R-13.13.5a  |

1. **#1** — Send text mail from A to B
   - **Expected:** B receives mail with correct text

### TC-13.13.5b.1 Mail Attachment Escrow

| # | Requirement |
|---|-------------|
| 1 | R-13.13.5b  |

1. **#1** — Attach item to mail
   - **Expected:** Item removed from sender inventory

### TC-13.13.5b.2 Mail COD

| # | Requirement |
|---|-------------|
| 1 | R-13.13.5b  |

1. **#1** — Send COD mail, recipient claims
   - **Expected:** Payment deducted before item delivered

### TC-13.13.5c.1 System Mail

| # | Requirement |
|---|-------------|
| 1 | R-13.13.5c  |

1. **#1** — Auction completion event
   - **Expected:** System mail arrives with proceeds

### TC-13.13.6a.1 Chat Rate Limit

| # | Requirement |
|---|-------------|
| 1 | R-13.13.6a  |

1. **#1** — Send 20 messages in 1 second (limit 10/s)
   - **Expected:** Last 10 messages blocked

### TC-13.13.6a.2 Chat Zone Transition

| # | Requirement |
|---|-------------|
| 1 | R-13.13.6a  |

1. **#1** — Send messages, change zone, check history
   - **Expected:** Previous messages still in history

### TC-13.13.6b.1 Item Link Tooltip

| # | Requirement |
|---|-------------|
| 1 | R-13.13.6b  |

1. **#1** — Link Epic Sword item in chat
   - **Expected:** Tooltip shows correct stats and rarity

### TC-13.13.6b.2 Profanity Filter

| # | Requirement |
|---|-------------|
| 1 | R-13.13.6b  |

1. **#1** — Send message with blacklisted term
   - **Expected:** Term replaced with asterisks

### TC-13.13.6c.1 Custom Channel Password

| # | Requirement |
|---|-------------|
| 1 | R-13.13.6c  |

1. **#1** — Create password channel, join without password
   - **Expected:** `Err(PasswordRequired)`

### TC-13.13.7.1 Emote Animation

| # | Requirement |
|---|-------------|
| 1 | R-13.13.7   |

1. **#1** — Trigger `/dance` emote
   - **Expected:** Looping dance animation plays on character

### TC-13.13.7.2 Paired Emote Sync

| # | Requirement |
|---|-------------|
| 1 | R-13.13.7   |

1. **#1** — Initiate handshake emote with target
   - **Expected:** Both characters play synchronized animation

### TC-13.13.8.1 Inspection Privacy

| # | Requirement |
|---|-------------|
| 1 | R-13.13.8   |

1. **#1** — Set friends-only inspection, non-friend inspects
   - **Expected:** `Err(PrivacyRestricted)`

### TC-13.13.9.1 Group Finder Role

| # | Requirement |
|---|-------------|
| 1 | R-13.13.9   |

1. **#1** — Queue as Tank for dungeon
   - **Expected:** Group composed with Tank + Healer + DPS

### TC-13.13.9.2 Deserter Penalty

| # | Requirement |
|---|-------------|
| 1 | R-13.13.9   |

1. **#1** — Leave dungeon instance early
   - **Expected:** Re-queue blocked for penalty duration

### TC-13.13.10a.1 Arena Rating Update

| # | Requirement |
|---|-------------|
| 1 | R-13.13.10a |

1. **#1** — Win arena 2v2 match
   - **Expected:** `RatingEntry.rating` increases, `wins += 1`

### TC-13.13.10d.1 PvP Normalization

| # | Requirement |
|---|-------------|
| 1 | R-13.13.10d |

1. **#1** — Enter arena with normalization enabled
   - **Expected:** Character stats match `PvpStatTemplate` values

### TC-13.13.10c.1 Seasonal Reset

| # | Requirement |
|---|-------------|
| 1 | R-13.13.10c |

1. **#1** — Trigger seasonal reset
   - **Expected:** Ratings reset to default, seasonal rewards distributed

### TC-13.13.NF3.1 Chat Throughput

| # | Requirement |
|---|-------------|
| 1 | R-13.13.NF3 |

1. **#1** — 100 msg/s for 60 seconds
   - **Expected:** Zero drops, processing < 1 ms/batch

## Integration Tests

### TC-13.12.1b.I1 Full Level Journey

| # | Requirement |
|---|-------------|
| 1 | R-13.12.1b  |

1. **#1** — Level 1 to max via XP awards
   - **Expected:** All ability unlocks and stat growth correct at every level

### TC-13.12.2c.I1 Talent Editor Roundtrip

| # | Requirement |
|---|-------------|
| 1 | R-13.12.2c  |

1. **#1** — Author 50-node tree in editor, load at runtime
   - **Expected:** Runtime behavior matches editor preview

### TC-13.12.3.I1 Profession Full Loop

| # | Requirement |
|---|-------------|
| 1 | R-13.12.3a  |

1. **#1** — Learn profession, gather, craft, level to max
   - **Expected:** Full profession loop completes without errors

### TC-13.13.1.I1 Guild Full Lifecycle

| # | Requirement |
|---|-------------|
| 1 | R-13.13.1a  |

1. **#1** — Create guild, invite, level, war, siege, dissolve
   - **Expected:** All state transitions correct; no leaks

### TC-13.13.4.I1 Cross Shard Friends

| # | Requirement |
|---|-------------|
| 1 | R-13.13.4   |

1. **#1** — Add friend on different shard
   - **Expected:** Friend status visible across shards

### TC-13.13.9.I1 Cross Shard Group Finder

| # | Requirement |
|---|-------------|
| 1 | R-13.13.9   |

1. **#1** — Queue from different shards
   - **Expected:** Players matched into group correctly

### TC-13.12.1d.I1 Prestige Full Cycle

| # | Requirement |
|---|-------------|
| 1 | R-13.12.1d  |

1. **#1** — Level to max, prestige 3 times
   - **Expected:** Bonuses accumulate across all 3 prestiges

### TC-13.12.9.I1 Set Bonus With Enhancement

| # | Requirement |
|---|-------------|
| 1 | R-13.12.9   |

1. **#1** — Equip enhanced set items (4-piece)
   - **Expected:** Both set bonus and enhancement bonuses active

## Benchmarks

### TC-13.12.NF1.B1 Talent Validation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 100-node tree, full allocation | p99 latency | < 1 ms | R-13.12.NF1 |

### TC-13.12.NF2.B1 Achievement Observer

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1000 definitions, 100 events/frame | Per-frame eval time | < 0.1 ms | R-13.12.NF2 |

### TC-13.13.NF1.B1 Guild Roster Render

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1000-member guild roster | Render time | < 1 frame | R-13.13.NF1 |

### TC-13.13.NF1.B2 Guild Permission Check

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Permission check per operation | Latency | < 0.1 ms | R-13.13.NF1 |

### TC-13.13.NF2.B1 Friend Status Propagation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Toggle online status | Propagation latency | < 2 s | R-13.13.NF2 |

### TC-13.13.NF3.B1 Chat Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Sustained message traffic | Throughput | >= 100 msg/s | R-13.13.NF3 |

### TC-13.12.7.B1 Enhancement Roll

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1000 enhancement attempts | Total time | < 1 ms | R-13.12.7 |

### TC-13.12.8a.B1 Rarity Stat Generation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 10,000 item stat rolls | Total time | < 10 ms | R-13.12.8a |
