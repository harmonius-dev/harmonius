# NPC Simulation Test Cases

Companion test cases for [npc-simulation.md](npc-simulation.md).

## Unit Tests

### TC-13.19.1.1 Affinity Modify Clamp

| # | Requirement |
|---|-------------|
| 1 | R-13.19.1   |
| 2 | R-13.19.1   |

1. **#1** — `modify(target, friendship, +200)` from 50
   - **Expected:** Clamped to 100
2. **#2** — `modify(target, friendship, -200)` from 50
   - **Expected:** Clamped to -100

### TC-13.19.1.2 Gift Preference Loved

| # | Requirement |
|---|-------------|
| 1 | R-13.19.1   |

1. **#1** — Give loved item (table value: +25 friendship)
   - **Expected:** Affinity friendship increased by 25

### TC-13.19.1.3 Gift Preference Hated

| # | Requirement |
|---|-------------|
| 1 | R-13.19.1   |

1. **#1** — Give hated item (table value: -15 friendship)
   - **Expected:** Affinity friendship decreased by 15

### TC-13.19.1.4 Tier Advance

| # | Requirement |
|---|-------------|
| 1 | R-13.19.1   |

1. **#1** — Push friendship past Friend threshold (60)
   - **Expected:** Tier advances from Acquaintance to Friend

### TC-13.19.1.5 Tier Persist Save Load

| # | Requirement |
|---|-------------|
| 1 | R-13.19.1   |

1. **#1** — Save NPC with affinity=75, tier=Friend; reload
   - **Expected:** Affinity = 75, tier = Friend after load

### TC-13.19.2.1 Personality Emotion Baseline

| # | Requirement |
|---|-------------|
| 1 | R-13.19.2   |

1. **#1** — High-courage NPC vs low-courage NPC; apply fear stimulus
   - **Expected:** High-courage NPC fear stays below low-courage NPC

### TC-13.19.2.2 Emotion Decay To Baseline

| # | Requirement |
|---|-------------|
| 1 | R-13.19.2   |

1. **#1** — Stimulate Happy to 0.8; tick decay for 100 frames
   - **Expected:** Happy returns to baseline (e.g., 0.3)

### TC-13.19.2.3 Emotion Dominant

| # | Requirement |
|---|-------------|
| 1 | R-13.19.2   |

1. **#1** — Happy=0.3, Angry=0.7, Afraid=0.5
   - **Expected:** `dominant()` returns Angry

### TC-13.19.3a.1 Deed Memory Positive

| # | Requirement |
|---|-------------|
| 1 | R-13.19.3a  |

1. **#1** — Witness rescue deed
   - **Expected:** Memory stored; affinity increased

### TC-13.19.3a.2 Deed Memory Negative

| # | Requirement |
|---|-------------|
| 1 | R-13.19.3a  |

1. **#1** — Witness theft deed
   - **Expected:** Memory stored; affinity decreased

### TC-13.19.3a.3 Deed Memory Decay

| # | Requirement |
|---|-------------|
| 1 | R-13.19.3a  |

1. **#1** — Wait for decay period (routine memory)
   - **Expected:** Memory emotional_weight reaches 0

### TC-13.19.3a.4 Deed Memory Eviction

| # | Requirement |
|---|-------------|
| 1 | R-13.19.3a  |

1. **#1** — Fill to capacity (50); add 1 more
   - **Expected:** Lowest-weight entry evicted; count stays 50

### TC-13.19.3b.1 Gossip Weight Degrades

| # | Requirement |
|---|-------------|
| 1 | R-13.19.3b  |

1. **#1** — A witnesses deed (weight=1.0); gossips to B
   - **Expected:** B receives weight = 1.0 * 0.7 = 0.7

### TC-13.19.3b.2 Gossip Multi Hop

| # | Requirement |
|---|-------------|
| 1 | R-13.19.3b  |

1. **#1** — A->B->C gossip chain
   - **Expected:** C reliability = 0.3 (0.7 * 0.7 * degradation)

### TC-13.19.3b.3 Gossip Rate Hermit

| # | Requirement |
|---|-------------|
| 1 | R-13.19.3b  |

1. **#1** — Hermit archetype NPC
   - **Expected:** Gossip propagation rate significantly lower

### TC-13.19.3c.1 Reputation Aggregate

| # | Requirement |
|---|-------------|
| 1 | R-13.19.3c  |

1. **#1** — Improve affinity with 3 NPCs in group
   - **Expected:** Group reputation score crosses threshold

### TC-13.19.3c.2 Reputation Unlock

| # | Requirement |
|---|-------------|
| 1 | R-13.19.3c  |

1. **#1** — Group reputation crosses unlock threshold
   - **Expected:** Group behavior activates (new dialogue, shops)

### TC-13.19.4a.1 Schedule Current Slot

| # | Requirement |
|---|-------------|
| 1 | R-13.19.4a  |

1. **#1** — Query schedule at 10:00 (work slot: 8:00-17:00)
   - **Expected:** Returns work slot entry

### TC-13.19.4a.2 Schedule Day Filter

| # | Requirement |
|---|-------------|
| 1 | R-13.19.4a  |

1. **#1** — Weekend schedule with alternate entry
   - **Expected:** Saturday returns alternate entry

### TC-13.19.4b.1 Schedule Navigation

| # | Requirement |
|---|-------------|
| 1 | R-13.19.4b  |

1. **#1** — Cross schedule boundary
   - **Expected:** Pathfind request issued to new location

### TC-13.19.4b.2 Schedule Skip Late

| # | Requirement |
|---|-------------|
| 1 | R-13.19.4b  |

1. **#1** — NPC arrives after slot end
   - **Expected:** Skips to current active slot

### TC-13.19.4c.1 Interaction Gate Sleeping

| # | Requirement |
|---|-------------|
| 1 | R-13.19.4c  |

1. **#1** — Query sleeping NPC for interaction
   - **Expected:** `GateReason::Sleeping`; unavailable

### TC-13.19.4c.2 Interaction Gate Working

| # | Requirement |
|---|-------------|
| 1 | R-13.19.4c  |

1. **#1** — Query NPC during work hours
   - **Expected:** Available for interaction

### TC-13.19.5.1 Bark Priority

| # | Requirement |
|---|-------------|
| 1 | R-13.19.5   |

1. **#1** — Two barks eligible: priority 5 and priority 8
   - **Expected:** Priority 8 bark selected

### TC-13.19.5.2 Bark Cooldown

| # | Requirement |
|---|-------------|
| 1 | R-13.19.5   |

1. **#1** — Trigger bark; trigger again within cooldown
   - **Expected:** Second trigger suppressed

### TC-13.19.5.3 Bark Filter Combat

| # | Requirement |
|---|-------------|
| 1 | R-13.19.5   |

1. **#1** — Combat-only bark; NPC not in combat
   - **Expected:** Bark does not trigger

### TC-13.19.6.1 Threat Add Decay

| # | Requirement |
|---|-------------|
| 1 | R-13.19.6   |

1. **#1** — Add 100 threat; source moves out of range; decay
   - **Expected:** Threat reaches 0

### TC-13.19.6.2 Threat Highest

| # | Requirement |
|---|-------------|
| 1 | R-13.19.6   |

1. **#1** — Three sources: 50, 80, 30 threat
   - **Expected:** `highest_threat()` returns 80-threat source

### TC-13.19.6.3 Threat Taunt

| # | Requirement |
|---|-------------|
| 1 | R-13.19.6   |

1. **#1** — Taunt ability used by entity A
   - **Expected:** Entity A tops the threat table

### TC-13.19.8.1 Memory Capacity Desktop

| # | Requirement |
|---|-------------|
| 1 | R-13.19.8   |

1. **#1** — Desktop NPC memory
   - **Expected:** Capacity = 50 entries

### TC-13.19.8.2 Memory Capacity Mobile

| # | Requirement |
|---|-------------|
| 1 | R-13.19.8   |

1. **#1** — Mobile NPC memory
   - **Expected:** Capacity = 20 entries

### TC-13.19.8.3 Memory Reliability Direct

| # | Requirement |
|---|-------------|
| 1 | R-13.19.8   |

1. **#1** — Direct witness of deed
   - **Expected:** reliability = 1.0

### TC-13.19.8.4 Memory Reliability Heard

| # | Requirement |
|---|-------------|
| 1 | R-13.19.8   |

1. **#1** — Heard from trusted NPC (1 hop)
   - **Expected:** reliability = 0.7

### TC-13.19.8.5 Memory Trauma Persists

| # | Requirement |
|---|-------------|
| 1 | R-13.19.8   |

1. **#1** — Traumatic memory; advance time past decay window
   - **Expected:** Memory persists; weight unchanged

### TC-13.19.8.6 Memory Routine Fades

| # | Requirement |
|---|-------------|
| 1 | R-13.19.8   |

1. **#1** — Routine memory; advance game-hours past decay window
   - **Expected:** Memory decays to zero

### TC-13.19.10.1 Search No Omniscience

| # | Requirement |
|---|-------------|
| 1 | R-13.19.10  |

1. **#1** — Player hidden; searcher NPC active
   - **Expected:** Searcher never moves directly to player position

### TC-13.19.10.2 Search Ask NPC

| # | Requirement |
|---|-------------|
| 1 | R-13.19.10  |

1. **#1** — Queried NPC has sighting memory
   - **Expected:** Searcher navigates to reported location

### TC-13.19.10.3 Search Faction Delay

| # | Requirement |
|---|-------------|
| 1 | R-13.19.10  |

1. **#1** — Faction broadcast alert issued
   - **Expected:** Responses arrive with delay (not instant)

### TC-13.19.11.1 Story Dialogue Change

| # | Requirement |
|---|-------------|
| 1 | R-13.19.11  |

1. **#1** — Complete quest triggering NPC reaction
   - **Expected:** NPC dialogue overridden per QuestReaction

### TC-13.19.11.2 Story Price Modifier

| # | Requirement |
|---|-------------|
| 1 | R-13.19.11  |

1. **#1** — Trigger story event with price_modifier=0.8
   - **Expected:** Merchant prices reduced by 20%

### TC-13.19.12.1 Eavesdrop Stealth Range

| # | Requirement |
|---|-------------|
| 1 | R-13.19.12  |

1. **#1** — Stealthed player near NPC conversation
   - **Expected:** Extended eavesdrop range; can hear conversation

### TC-13.19.12.2 Eavesdrop No Stealth

| # | Requirement |
|---|-------------|
| 1 | R-13.19.12  |

1. **#1** — Non-stealthed player
   - **Expected:** Shorter eavesdrop range

### TC-13.19.7.1 Conversation Interrupt

| # | Requirement |
|---|-------------|
| 1 | R-13.19.7   |

1. **#1** — Player approaches two conversing NPCs
   - **Expected:** Conversation interrupted; NPCs acknowledge player

## Integration Tests

### TC-NFR-13.19.1.I1 200 NPC Frame Budget

| # | Requirement |
|---|-------------|
| 1 | NFR-13.19.1 |

1. **#1** — Spawn 200 scheduled NPCs
   - **Expected:** Total NPC simulation time < 4 ms/frame

### TC-NFR-13.19.1.I2 Gossip Amortization

| # | Requirement |
|---|-------------|
| 1 | NFR-13.19.1 |

1. **#1** — Trigger 50 gossip events
   - **Expected:** Amortized at 10/frame across 5 frames

### TC-NFR-13.19.2.I1 Memory Global Budget

| # | Requirement |
|---|-------------|
| 1 | NFR-13.19.2 |

1. **#1** — Generate 1000 deeds across 200 NPCs
   - **Expected:** Total memory <= 4 MB

### TC-NFR-13.19.2.I2 Memory Entry Size

| # | Requirement |
|---|-------------|
| 1 | NFR-13.19.2 |

1. **#1** — Measure `MemoryEntry` struct size
   - **Expected:** <= 256 bytes

### TC-13.19.8.I1 LOD Promotion Demotion

| # | Requirement |
|---|-------------|
| 1 | R-13.19.8   |
| 2 | R-13.19.8   |

1. **#1** — Move player away from NPC
   - **Expected:** LOD demotes (Full -> Reduced -> Minimal)
2. **#2** — Move player back toward NPC
   - **Expected:** LOD promotes; memory/conversation resumes

### TC-13.19.3b.I1 Full Social Chain

| # | Requirement |
|---|-------------|
| 1 | R-13.19.3b  |

1. **#1** — Player performs deed; A witnesses; A->B->C gossip
   - **Expected:** Reputation propagates end-to-end through social graph

### TC-13.19.4b.I1 Schedule Full Day

| # | Requirement |
|---|-------------|
| 1 | R-13.19.4b  |

1. **#1** — Run 24 game-hours
   - **Expected:** NPC visits all scheduled locations in order

### TC-13.19.7.I1 Conversation To Memory

| # | Requirement |
|---|-------------|
| 1 | R-13.19.7   |

1. **#1** — Two NPCs converse about a topic
   - **Expected:** Both gain memory entries with source attribution

### TC-13.19.9.I1 Env Interaction Door

| # | Requirement |
|---|-------------|
| 1 | R-13.19.9   |
| 2 | R-13.19.9   |

1. **#1** — NPC with key pathfinds through locked door
   - **Expected:** Door opened; NPC passes through
2. **#2** — NPC without key encounters locked door
   - **Expected:** NPC routes around the door

### TC-13.19.7.I2 NPC Conversation Mobile Limit

| # | Requirement |
|---|-------------|
| 1 | R-13.19.7   |

1. **#1** — Mobile config; start 5 NPC conversations
   - **Expected:** Only 4 active; 5th queued

### TC-13.19.10.I1 Search Expand Radius

| # | Requirement |
|---|-------------|
| 1 | R-13.19.10  |

1. **#1** — All nearby NPC queries fail
   - **Expected:** Search radius expands

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
