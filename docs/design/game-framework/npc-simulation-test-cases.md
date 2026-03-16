# NPC Simulation Test Cases

Companion test cases for [npc-simulation.md](npc-simulation.md).

## Unit Tests

### TC-13.19.1.1 Affinity Modify Clamp

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `modify(target, friendship, +200)` from 50 | Clamped to 100 | R-13.19.1 |
| 2 | `modify(target, friendship, -200)` from 50 | Clamped to -100 | R-13.19.1 |

### TC-13.19.1.2 Gift Preference Loved

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Give loved item (table value: +25 friendship) | Affinity friendship increased by 25 | R-13.19.1 |

### TC-13.19.1.3 Gift Preference Hated

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Give hated item (table value: -15 friendship) | Affinity friendship decreased by 15 | R-13.19.1 |

### TC-13.19.1.4 Tier Advance

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Push friendship past Friend threshold (60) | Tier advances from Acquaintance to Friend | R-13.19.1 |

### TC-13.19.1.5 Tier Persist Save Load

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Save NPC with affinity=75, tier=Friend; reload | Affinity = 75, tier = Friend after load | R-13.19.1 |

### TC-13.19.2.1 Personality Emotion Baseline

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | High-courage NPC vs low-courage NPC; apply fear stimulus | High-courage NPC fear stays below low-courage NPC | R-13.19.2 |

### TC-13.19.2.2 Emotion Decay To Baseline

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Stimulate Happy to 0.8; tick decay for 100 frames | Happy returns to baseline (e.g., 0.3) | R-13.19.2 |

### TC-13.19.2.3 Emotion Dominant

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Happy=0.3, Angry=0.7, Afraid=0.5 | `dominant()` returns Angry | R-13.19.2 |

### TC-13.19.3a.1 Deed Memory Positive

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Witness rescue deed | Memory stored; affinity increased | R-13.19.3a |

### TC-13.19.3a.2 Deed Memory Negative

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Witness theft deed | Memory stored; affinity decreased | R-13.19.3a |

### TC-13.19.3a.3 Deed Memory Decay

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Wait for decay period (routine memory) | Memory emotional_weight reaches 0 | R-13.19.3a |

### TC-13.19.3a.4 Deed Memory Eviction

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fill to capacity (50); add 1 more | Lowest-weight entry evicted; count stays 50 | R-13.19.3a |

### TC-13.19.3b.1 Gossip Weight Degrades

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | A witnesses deed (weight=1.0); gossips to B | B receives weight = 1.0 * 0.7 = 0.7 | R-13.19.3b |

### TC-13.19.3b.2 Gossip Multi Hop

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | A->B->C gossip chain | C reliability = 0.3 (0.7 * 0.7 * degradation) | R-13.19.3b |

### TC-13.19.3b.3 Gossip Rate Hermit

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Hermit archetype NPC | Gossip propagation rate significantly lower | R-13.19.3b |

### TC-13.19.3c.1 Reputation Aggregate

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Improve affinity with 3 NPCs in group | Group reputation score crosses threshold | R-13.19.3c |

### TC-13.19.3c.2 Reputation Unlock

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Group reputation crosses unlock threshold | Group behavior activates (new dialogue, shops) | R-13.19.3c |

### TC-13.19.4a.1 Schedule Current Slot

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Query schedule at 10:00 (work slot: 8:00-17:00) | Returns work slot entry | R-13.19.4a |

### TC-13.19.4a.2 Schedule Day Filter

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Weekend schedule with alternate entry | Saturday returns alternate entry | R-13.19.4a |

### TC-13.19.4b.1 Schedule Navigation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Cross schedule boundary | Pathfind request issued to new location | R-13.19.4b |

### TC-13.19.4b.2 Schedule Skip Late

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | NPC arrives after slot end | Skips to current active slot | R-13.19.4b |

### TC-13.19.4c.1 Interaction Gate Sleeping

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Query sleeping NPC for interaction | `GateReason::Sleeping`; unavailable | R-13.19.4c |

### TC-13.19.4c.2 Interaction Gate Working

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Query NPC during work hours | Available for interaction | R-13.19.4c |

### TC-13.19.5.1 Bark Priority

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Two barks eligible: priority 5 and priority 8 | Priority 8 bark selected | R-13.19.5 |

### TC-13.19.5.2 Bark Cooldown

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Trigger bark; trigger again within cooldown | Second trigger suppressed | R-13.19.5 |

### TC-13.19.5.3 Bark Filter Combat

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Combat-only bark; NPC not in combat | Bark does not trigger | R-13.19.5 |

### TC-13.19.6.1 Threat Add Decay

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Add 100 threat; source moves out of range; decay | Threat reaches 0 | R-13.19.6 |

### TC-13.19.6.2 Threat Highest

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Three sources: 50, 80, 30 threat | `highest_threat()` returns 80-threat source | R-13.19.6 |

### TC-13.19.6.3 Threat Taunt

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Taunt ability used by entity A | Entity A tops the threat table | R-13.19.6 |

### TC-13.19.8.1 Memory Capacity Desktop

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Desktop NPC memory | Capacity = 50 entries | R-13.19.8 |

### TC-13.19.8.2 Memory Capacity Mobile

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mobile NPC memory | Capacity = 20 entries | R-13.19.8 |

### TC-13.19.8.3 Memory Reliability Direct

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Direct witness of deed | reliability = 1.0 | R-13.19.8 |

### TC-13.19.8.4 Memory Reliability Heard

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Heard from trusted NPC (1 hop) | reliability = 0.7 | R-13.19.8 |

### TC-13.19.8.5 Memory Trauma Persists

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Traumatic memory; advance time past decay window | Memory persists; weight unchanged | R-13.19.8 |

### TC-13.19.8.6 Memory Routine Fades

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Routine memory; advance game-hours past decay window | Memory decays to zero | R-13.19.8 |

### TC-13.19.10.1 Search No Omniscience

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Player hidden; searcher NPC active | Searcher never moves directly to player position | R-13.19.10 |

### TC-13.19.10.2 Search Ask NPC

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Queried NPC has sighting memory | Searcher navigates to reported location | R-13.19.10 |

### TC-13.19.10.3 Search Faction Delay

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Faction broadcast alert issued | Responses arrive with delay (not instant) | R-13.19.10 |

### TC-13.19.11.1 Story Dialogue Change

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Complete quest triggering NPC reaction | NPC dialogue overridden per QuestReaction | R-13.19.11 |

### TC-13.19.11.2 Story Price Modifier

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Trigger story event with price_modifier=0.8 | Merchant prices reduced by 20% | R-13.19.11 |

### TC-13.19.12.1 Eavesdrop Stealth Range

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Stealthed player near NPC conversation | Extended eavesdrop range; can hear conversation | R-13.19.12 |

### TC-13.19.12.2 Eavesdrop No Stealth

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Non-stealthed player | Shorter eavesdrop range | R-13.19.12 |

### TC-13.19.7.1 Conversation Interrupt

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Player approaches two conversing NPCs | Conversation interrupted; NPCs acknowledge player | R-13.19.7 |

## Integration Tests

### TC-NFR-13.19.1.I1 200 NPC Frame Budget

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Spawn 200 scheduled NPCs | Total NPC simulation time < 4 ms/frame | NFR-13.19.1 |

### TC-NFR-13.19.1.I2 Gossip Amortization

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Trigger 50 gossip events | Amortized at 10/frame across 5 frames | NFR-13.19.1 |

### TC-NFR-13.19.2.I1 Memory Global Budget

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Generate 1000 deeds across 200 NPCs | Total memory <= 4 MB | NFR-13.19.2 |

### TC-NFR-13.19.2.I2 Memory Entry Size

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Measure `MemoryEntry` struct size | <= 256 bytes | NFR-13.19.2 |

### TC-13.19.8.I1 LOD Promotion Demotion

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Move player away from NPC | LOD demotes (Full -> Reduced -> Minimal) | R-13.19.8 |
| 2 | Move player back toward NPC | LOD promotes; memory/conversation resumes | R-13.19.8 |

### TC-13.19.3b.I1 Full Social Chain

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Player performs deed; A witnesses; A->B->C gossip | Reputation propagates end-to-end through social graph | R-13.19.3b |

### TC-13.19.4b.I1 Schedule Full Day

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Run 24 game-hours | NPC visits all scheduled locations in order | R-13.19.4b |

### TC-13.19.7.I1 Conversation To Memory

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Two NPCs converse about a topic | Both gain memory entries with source attribution | R-13.19.7 |

### TC-13.19.9.I1 Env Interaction Door

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | NPC with key pathfinds through locked door | Door opened; NPC passes through | R-13.19.9 |
| 2 | NPC without key encounters locked door | NPC routes around the door | R-13.19.9 |

### TC-13.19.7.I2 NPC Conversation Mobile Limit

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mobile config; start 5 NPC conversations | Only 4 active; 5th queued | R-13.19.7 |

### TC-13.19.10.I1 Search Expand Radius

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | All nearby NPC queries fail | Search radius expands | R-13.19.10 |

## Benchmarks

### TC-NFR-13.19.1.B1 200 NPC Full Simulation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 200 NPCs with full simulation | Frame time | < 4 ms | NFR-13.19.1 |

### TC-NFR-13.19.1.B2 Gossip Drain

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Process 10 gossip events | Wall-clock time | < 200 us/frame | NFR-13.19.1 |

### TC-NFR-13.19.1.B3 Memory Decay

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Decay pass over 200 NPCs | Wall-clock time | < 500 us/frame | NFR-13.19.1 |

### TC-NFR-13.19.1.B4 Bark Evaluation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Evaluate barks for 50 NPCs | Wall-clock time | < 300 us/frame | NFR-13.19.1 |

### TC-13.19.4b.B1 Schedule Transition

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Single NPC schedule transition | Wall-clock time | < 50 us | R-13.19.4b |

### TC-13.19.6.B1 Threat Table Update

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Update threat for 1 enemy | Wall-clock time | < 10 us | R-13.19.6 |

### TC-13.19.3c.B1 Reputation Aggregation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Aggregate reputation for 1 group | Wall-clock time | < 100 us | R-13.19.3c |

### TC-NFR-13.19.2.B1 Memory Entry Allocation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Allocate single MemoryEntry | Wall-clock time | < 1 us | NFR-13.19.2 |
