# Progression and Social Systems Test Cases

Companion test cases for [progression-social.md](progression-social.md).

## Unit Tests

### TC-13.12.1b.1 XP Level Up

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Level 9, XP 990/1000, award 20 XP | `level == 10`, `current_xp == 10`, stats grown per `StatGrowthTable` | R-13.12.1b |

### TC-13.12.2a.1 Talent Prerequisite Reject

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Allocate tier-2 node without tier-1 parent allocated | `Err(PrerequisiteNotMet)` | R-13.12.2a |

### TC-13.12.2a.2 Talent Tier Gate

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 3 points allocated, tier-2 gate requires 5 | `Err(TierGateNotMet { required: 5, have: 3 })` | R-13.12.2a |

### TC-13.12.2b.1 Talent Allocation Success

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Allocate valid node with 1 available point | `TalentState` updated, effect applied, `available_points -= 1` | R-13.12.2b |

### TC-13.12.2b.2 Talent Respec

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Respec with 15 points allocated, cost 100 gold | All nodes cleared, 15 points refunded, effects removed, gold -= 100 | R-13.12.2b |

### TC-13.12.NF1.1 Talent Validation Performance

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 100-node tree, sequential allocation of all nodes | p99 validation time < 1 ms | R-13.12.NF1 |

### TC-13.12.1d.1 Prestige Reset

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Prestige at max level 60 | `level == 1`, `prestige_level += 1`, permanent bonuses accumulated | R-13.12.1d |

### TC-13.12.1d.2 Prestige Cosmetics Preserved

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Prestige with 3 cosmetic rewards unlocked | All 3 cosmetic rewards retained post-prestige | R-13.12.1d |

### TC-13.12.3a.1 Profession Recipe Unlock

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Level profession from 14 to 15, threshold at 15 | Recipe unlocked, accessible in crafting UI | R-13.12.3a |

### TC-13.12.3a.2 Profession Slot Limit

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `max_slots == 2`, attempt to learn 3rd profession | `Err(SlotLimitReached)` | R-13.12.3a |

### TC-13.12.3b.1 Gathering Yield Scales

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Gather at skill 10 vs skill 50 | Skill-50 yield > skill-10 yield | R-13.12.3b |

### TC-13.12.3c.1 Crafting Level Gate

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Attempt recipe requiring level 20 at level 15 | `Err(LevelTooLow { required: 20, have: 15 })` | R-13.12.3c |

### TC-13.12.4.1 Station Type Gates

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Query forge recipes at AlchemyTable | Forge recipes absent from available list | R-13.12.4 |

### TC-13.12.5.1 Reputation Gain

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Complete quest awarding +500 rep to faction A | `FactionReputation.current_value += 500` | R-13.12.5 |

### TC-13.12.5.2 Reputation Asymmetric

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Gain 1000 rep with faction A, rival B loss_ratio 0.5 | Faction B rep decreases by 500 | R-13.12.5 |

### TC-13.12.5.3 Reputation Tier Gating

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Reach Honored standing with faction A | Honored-tier vendor items accessible | R-13.12.5 |

### TC-13.12.6a.1 Achievement Incremental

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Kill 50 of 100 enemies for achievement | `current == 50`, `completed == false` | R-13.12.6a |
| 2 | Kill 50 more (total 100) | `completed == true`, timestamp recorded | R-13.12.6a |

### TC-13.12.NF2.1 Achievement Observer Budget

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1000 achievement definitions, 100 events/frame | Observer evaluation < 0.1 ms/frame | R-13.12.NF2 |

### TC-13.12.6c.1 Achievement Platform Sync

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Complete achievement with Steam mapping | `ISteamUserStats::SetAchievement` called | R-13.12.6c |

### TC-13.12.7.1 Enhancement Success

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Enhance +0 item at 100% success rate | `EnhancementLevel.level == 1`, stat bonus applied | R-13.12.7 |

### TC-13.12.7.2 Enhancement Failure

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Enhance at 10% rate, roll fails | Consequence applied per `FailureConsequence` | R-13.12.7 |

### TC-13.12.7.3 Protection Item

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Enhancement failure with protection item equipped | Item not destroyed; protection consumed | R-13.12.7 |

### TC-13.12.8a.1 Rarity Stat Ranges

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Generate 1000 items per rarity tier | All stat values within configured min/max per tier | R-13.12.8a |

### TC-13.12.8b.1 Affix Count By Rarity

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Generate items at each rarity | Affix counts match per-rarity config (e.g., Rare=2, Epic=3) | R-13.12.8b |

### TC-13.12.8c.1 Reroll Preserves Base

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Re-roll Epic sword affixes | Base item type and rarity tier preserved; affixes changed | R-13.12.8c |

### TC-13.12.9.1 Set Bonus Thresholds

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Equip 2/4/6 set pieces | Bonuses activate at each threshold | R-13.12.9 |

### TC-13.12.9.2 Set Bonus Deactivate

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Unequip below 4-piece threshold | 4-piece bonus removed, 2-piece remains | R-13.12.9 |

### TC-13.12.10.1 Durability Drain

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Attack 10 times with weapon | `Durability.current` decreased per attack | R-13.12.10 |

### TC-13.12.10.2 Durability Zero Nonfunctional

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Reduce durability to 0% | Weapon deals 0 damage | R-13.12.10 |

### TC-13.12.10.3 Repair Restores

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Repair from 0% to 100% | `current == max`, full stat restoration | R-13.12.10 |

### TC-13.13.1a.1 Guild Create

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Create guild "TestGuild" | Entity with `Guild { name: "TestGuild" }` component | R-13.13.1a |

### TC-13.13.1a.2 Guild Invite Accept

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Invite player B, B accepts | `GuildMember` added to player B entity | R-13.13.1a |

### TC-13.13.1a.3 Guild Kick

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Kick member B (with permission) | `GuildMember` removed from player B | R-13.13.1a |

### TC-13.13.1a.4 Guild Dissolve Cooldown

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Dissolve guild, attempt create within cooldown | `Err(CooldownActive)` | R-13.13.1a |

### TC-13.13.1b.1 Guild Permission Check

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Rank without `can_kick`, attempt kick | `Err(InsufficientPermission)` | R-13.13.1b |

### TC-13.13.1b.2 Guild Rank Assignment

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Assign Officer rank to member | `GuildMember.rank` updated, permissions match Officer | R-13.13.1b |

### TC-13.13.1c.1 Guild XP Accumulation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Complete quest awarding 100 guild XP | `GuildLevel.current_xp += 100` | R-13.13.1c |

### TC-13.13.1c.2 Guild Perk Unlock

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Reach guild level threshold for perk | Perk added to `active_perks` | R-13.13.1c |

### TC-13.13.NF1.1 Guild Roster 1000

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1000-member guild, render roster | Renders within 1 frame (< 16.67 ms) | R-13.13.NF1 |

### TC-13.13.2.1 Guild Bank Permission

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Withdraw from tab requiring Officer rank as Member | `Err(InsufficientRank)` | R-13.13.2 |

### TC-13.13.2.2 Guild Bank Daily Limit

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Exhaust daily withdrawal limit, attempt one more | `Err(DailyLimitReached)` | R-13.13.2 |

### TC-13.13.2.3 Guild Bank Audit Log

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Perform 100 deposit/withdraw transactions | `transaction_log.len() == 100` | R-13.13.2 |

### TC-13.13.3a.1 Territory Claim

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Claim unclaimed territory | `TerritoryOwnership.owning_guild == guild_entity` | R-13.13.3a |

### TC-13.13.3a.2 Territory Exclusive

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Two guilds claim same territory | Only first succeeds; second gets `Err(AlreadyClaimed)` | R-13.13.3a |

### TC-13.13.3b.1 War Declaration

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Declare war between guilds A and B | PvP enabled between members of A and B | R-13.13.3b |

### TC-13.13.3b.2 War Non-Warring Safe

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Non-warring guild member attacks warring guild | PvP blocked, damage = 0 | R-13.13.3b |

### TC-13.13.3c.1 Siege Window

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Attack territory outside scheduled war window | `Err(OutsideWindow)` | R-13.13.3c |

### TC-13.13.3d.1 Leaderboard Update

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Win guild war | Guild leaderboard points increase | R-13.13.3d |

### TC-13.13.4.1 Friend Add Remove

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Add friend B, then block B | Both lists update; B hidden from A's view | R-13.13.4 |

### TC-13.13.NF2.1 Friend Status Latency

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Toggle online status | Friends list update received within 2 seconds | R-13.13.NF2 |

### TC-13.13.5a.1 Mail Send Receive

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Send text mail from A to B | B receives mail with correct text | R-13.13.5a |

### TC-13.13.5b.1 Mail Attachment Escrow

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Attach item to mail | Item removed from sender inventory | R-13.13.5b |

### TC-13.13.5b.2 Mail COD

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Send COD mail, recipient claims | Payment deducted before item delivered | R-13.13.5b |

### TC-13.13.5c.1 System Mail

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Auction completion event | System mail arrives with proceeds | R-13.13.5c |

### TC-13.13.6a.1 Chat Rate Limit

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Send 20 messages in 1 second (limit 10/s) | Last 10 messages blocked | R-13.13.6a |

### TC-13.13.6a.2 Chat Zone Transition

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Send messages, change zone, check history | Previous messages still in history | R-13.13.6a |

### TC-13.13.6b.1 Item Link Tooltip

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Link Epic Sword item in chat | Tooltip shows correct stats and rarity | R-13.13.6b |

### TC-13.13.6b.2 Profanity Filter

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Send message with blacklisted term | Term replaced with asterisks | R-13.13.6b |

### TC-13.13.6c.1 Custom Channel Password

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Create password channel, join without password | `Err(PasswordRequired)` | R-13.13.6c |

### TC-13.13.7.1 Emote Animation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Trigger `/dance` emote | Looping dance animation plays on character | R-13.13.7 |

### TC-13.13.7.2 Paired Emote Sync

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Initiate handshake emote with target | Both characters play synchronized animation | R-13.13.7 |

### TC-13.13.8.1 Inspection Privacy

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Set friends-only inspection, non-friend inspects | `Err(PrivacyRestricted)` | R-13.13.8 |

### TC-13.13.9.1 Group Finder Role

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Queue as Tank for dungeon | Group composed with Tank + Healer + DPS | R-13.13.9 |

### TC-13.13.9.2 Deserter Penalty

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Leave dungeon instance early | Re-queue blocked for penalty duration | R-13.13.9 |

### TC-13.13.10a.1 Arena Rating Update

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Win arena 2v2 match | `RatingEntry.rating` increases, `wins += 1` | R-13.13.10a |

### TC-13.13.10d.1 PvP Normalization

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Enter arena with normalization enabled | Character stats match `PvpStatTemplate` values | R-13.13.10d |

### TC-13.13.10c.1 Seasonal Reset

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Trigger seasonal reset | Ratings reset to default, seasonal rewards distributed | R-13.13.10c |

### TC-13.13.NF3.1 Chat Throughput

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 100 msg/s for 60 seconds | Zero drops, processing < 1 ms/batch | R-13.13.NF3 |

## Integration Tests

### TC-13.12.1b.I1 Full Level Journey

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Level 1 to max via XP awards | All ability unlocks and stat growth correct at every level | R-13.12.1b |

### TC-13.12.2c.I1 Talent Editor Roundtrip

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Author 50-node tree in editor, load at runtime | Runtime behavior matches editor preview | R-13.12.2c |

### TC-13.12.3.I1 Profession Full Loop

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Learn profession, gather, craft, level to max | Full profession loop completes without errors | R-13.12.3a |

### TC-13.13.1.I1 Guild Full Lifecycle

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Create guild, invite, level, war, siege, dissolve | All state transitions correct; no leaks | R-13.13.1a |

### TC-13.13.4.I1 Cross Shard Friends

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Add friend on different shard | Friend status visible across shards | R-13.13.4 |

### TC-13.13.9.I1 Cross Shard Group Finder

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Queue from different shards | Players matched into group correctly | R-13.13.9 |

### TC-13.12.1d.I1 Prestige Full Cycle

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Level to max, prestige 3 times | Bonuses accumulate across all 3 prestiges | R-13.12.1d |

### TC-13.12.9.I1 Set Bonus With Enhancement

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Equip enhanced set items (4-piece) | Both set bonus and enhancement bonuses active | R-13.12.9 |

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
