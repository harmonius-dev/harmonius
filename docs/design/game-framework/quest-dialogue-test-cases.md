# Quest & Dialogue System Test Cases

Companion test cases for [quest-dialogue.md](quest-dialogue.md).

## Unit Tests

### TC-13.6.1.1 Quest DAG Validation Valid

| # | Requirement |
|---|-------------|
| 1 | R-13.6.1    |

1. **#1** — 5-node DAG with root, 3 branches, 1 terminal
   - **Expected:** `validate() == Ok(())`

### TC-13.6.1.2 Quest DAG Cycle Detected

| # | Requirement |
|---|-------------|
| 1 | R-13.6.1    |

1. **#1** — Graph with edge from node 3 back to node 1
   - **Expected:** `Err(CycleDetected { cycle: [1, 2, 3] })`

### TC-13.6.1.3 Quest DAG Orphan Node

| # | Requirement |
|---|-------------|
| 1 | R-13.6.1    |

1. **#1** — Node 4 with no incoming or outgoing edges
   - **Expected:** `Err(OrphanNode(4))`

### TC-13.6.1.4 Objective Kill Progress

| # | Requirement |
|---|-------------|
| 1 | R-13.6.1    |

1. **#1** — Kill event matching `target_tag`, quest active
   - **Expected:** `ObjectiveProgress.current += 1`

### TC-13.6.1.5 Objective Collect Progress

| # | Requirement |
|---|-------------|
| 1 | R-13.6.1    |

1. **#1** — Add matching item to inventory, collect objective active
   - **Expected:** `ObjectiveProgress.current += 1`

### TC-13.6.2.1 Prerequisite And

| # | Requirement |
|---|-------------|
| 1 | R-13.6.2    |
| 2 | R-13.6.2    |
| 3 | R-13.6.2    |

1. **#1** — `And(LevelAtLeast(10), QuestCompleted(Q1))`, level=10, Q1 done
   - **Expected:** `true`
2. **#2** — Same, level=9, Q1 done
   - **Expected:** `false`
3. **#3** — Same, level=10, Q1 not done
   - **Expected:** `false`

### TC-13.6.2.2 Prerequisite Or

| # | Requirement |
|---|-------------|
| 1 | R-13.6.2    |
| 2 | R-13.6.2    |
| 3 | R-13.6.2    |

1. **#1** — `Or(HasItem(I1, 1), RepAtLeast(F1, 100))`, has item
   - **Expected:** `true`
2. **#2** — Same, rep=100, no item
   - **Expected:** `true`
3. **#3** — Same, no item, rep=50
   - **Expected:** `false`

### TC-13.6.2.3 Prerequisite Not

| # | Requirement |
|---|-------------|
| 1 | R-13.6.2    |
| 2 | R-13.6.2    |

1. **#1** — `Not(QuestCompleted(Q2))`, Q2 not done
   - **Expected:** `true`
2. **#2** — Same, Q2 done
   - **Expected:** `false`

### TC-13.6.2.4 Prerequisite Time Window

| # | Requirement |
|---|-------------|
| 1 | R-13.6.2    |
| 2 | R-13.6.2    |

1. **#1** — `TimeWindow(Jan 1, Jan 31)`, current = Jan 15
   - **Expected:** `true`
2. **#2** — Same, current = Feb 15
   - **Expected:** `false`

### TC-13.6.2.5 Prerequisite Lazy Eval

| # | Requirement |
|---|-------------|
| 1 | R-13.6.2    |

1. **#1** — Track evaluation count over 60 frames without interaction
   - **Expected:** Evaluation count == 0

### TC-13.6.3.1 Journal Category Filter

| # | Requirement |
|---|-------------|
| 1 | R-13.6.3    |

1. **#1** — 5 quests: 2 Daily, 2 Side, 1 MainStory; filter `Daily`
   - **Expected:** Returns exactly 2 Daily quest IDs

### TC-13.6.3.2 Journal Search

| # | Requirement |
|---|-------------|
| 1 | R-13.6.3    |

1. **#1** — Quests named "Dragon Hunt", "Wolf Hunt", "Fetch Water"; search "Hunt"
   - **Expected:** Returns [Dragon Hunt, Wolf Hunt]

### TC-13.6.NF1.1 Journal 50 Active Quests

| # | Requirement |
|---|-------------|
| 1 | R-13.6.NF1  |

1. **#1** — Activate 50 quests simultaneously
   - **Expected:** All 50 track correctly, `active_count() == 50`

### TC-13.6.4.1 Dialogue Condition Branch

| # | Requirement |
|---|-------------|
| 1 | R-13.6.4    |

1. **#1** — 3-branch node: quest state, rep, class conditions
   - **Expected:** Correct branch visible per player state

### TC-13.6.4.2 Dialogue Effect Item Grant

| # | Requirement |
|---|-------------|
| 1 | R-13.6.4    |

1. **#1** — Traverse node with `GrantItem { item_id: I5, count: 3 }`
   - **Expected:** 3 of item I5 added to player inventory

### TC-13.6.4.3 Dialogue Effect Rep Change

| # | Requirement |
|---|-------------|
| 1 | R-13.6.4    |

1. **#1** — Traverse node with `ChangeReputation { faction: F2, delta: 200 }`
   - **Expected:** Faction F2 rep += 200

### TC-13.6.4.4 Dialogue Localization

| # | Requirement |
|---|-------------|
| 1 | R-13.6.4    |

1. **#1** — Load English and French locales, query node text
   - **Expected:** Distinct text per locale

### TC-13.6.NF2.1 Dialogue Eval Latency

| # | Requirement |
|---|-------------|
| 1 | R-13.6.NF2  |

1. **#1** — 100-node tree, 20 conditional branches
   - **Expected:** Traversal completes in < 5 ms

### TC-13.6.6.1 Reward Level Scaling

| # | Requirement |
|---|-------------|
| 1 | R-13.6.6    |

1. **#1** — XP reward 1000, level curve sample at 20 = 1.5
   - **Expected:** Granted XP = 1500

### TC-13.6.6.2 Reward Choice Of N

| # | Requirement |
|---|-------------|
| 1 | R-13.6.6    |

1. **#1** — 3 item choices, player selects index 1
   - **Expected:** Only item at index 1 granted

### TC-13.6.6.3 Reward Transactional

| # | Requirement |
|---|-------------|
| 1 | R-13.6.6    |

1. **#1** — Inventory full during reward grant
   - **Expected:** `Err(InventoryFull)`, no partial rewards applied

### TC-13.6.6.4 Reward Group Loot

| # | Requirement |
|---|-------------|
| 1 | R-13.6.6    |

1. **#1** — 5-player group completes quest
   - **Expected:** Each player receives rewards per loot mode

### TC-13.6.5b.1 Conversation State Suppress

| # | Requirement |
|---|-------------|
| 1 | R-13.6.5b   |

1. **#1** — Start conversation with `HudSuppressionLevel::Full`
   - **Expected:** HUD hidden, audio ducked, input suppressed

### TC-13.6.5b.2 Conversation State Restore

| # | Requirement |
|---|-------------|
| 1 | R-13.6.5b   |

1. **#1** — End conversation
   - **Expected:** HUD visible, audio restored, input enabled

### TC-13.6.5c.1 Conversation Interrupt

| # | Requirement |
|---|-------------|
| 1 | R-13.6.5c   |

1. **#1** — Combat event during dialogue
   - **Expected:** Gameplay restored, `ConversationInterrupted` component saved with `last_node`

### TC-13.6.5c.2 Conversation Resume

| # | Requirement |
|---|-------------|
| 1 | R-13.6.5c   |

1. **#1** — Re-engage NPC after interruption
   - **Expected:** Conversation resumes from `last_node`

## Integration Tests

### TC-13.6.1.I1 Quest Full Lifecycle

| # | Requirement |
|---|-------------|
| 1 | R-13.6.1    |

1. **#1** — Accept quest, complete all objectives, turn in
   - **Expected:** Rewards granted, journal shows Completed

### TC-13.6.3.I1 Quest Event Propagation

| # | Requirement |
|---|-------------|
| 1 | R-13.6.3    |

1. **#1** — Advance objective
   - **Expected:** UI, map markers, minimap all update same frame

### TC-13.6.1.I2 Server Rejects Tampered

| # | Requirement |
|---|-------------|
| 1 | R-13.6.1    |

1. **#1** — Forged quest completion from modified client
   - **Expected:** Server rejects; quest state unchanged

### TC-13.6.7b.I1 Phasing Two Players

| # | Requirement |
|---|-------------|
| 1 | R-13.6.7b   |

1. **#1** — Player A at phase 1, player B at phase 2, same zone
   - **Expected:** Each sees correct sub-level

### TC-13.6.7b.I2 Phase Transition Swap

| # | Requirement |
|---|-------------|
| 1 | R-13.6.7b   |

1. **#1** — Advance through phase boundary
   - **Expected:** Old sub-level unloads, new loads with correct entities

### TC-13.6.7a.I1 World Event Broadcast

| # | Requirement |
|---|-------------|
| 1 | R-13.6.7a   |

1. **#1** — Trigger world event
   - **Expected:** All clients receive zone state change within 1 second

### TC-13.6.5c.I1 Disconnect During Dialogue

| # | Requirement |
|---|-------------|
| 1 | R-13.6.5c   |

1. **#1** — Disconnect mid-conversation, reconnect
   - **Expected:** Conversation state preserved for resumption

### TC-13.6.5a.I1 Camera Multi Speaker

| # | Requirement |
|---|-------------|
| 1 | R-13.6.5a   |

1. **#1** — Multi-NPC conversation with 3 speakers
   - **Expected:** Camera switches to face active speaker

## Benchmarks

### TC-13.6.NF1.B1 Quest Tracking Per Frame

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 50 active quests per frame | Tracking time | < 0.5 ms | R-13.6.NF1 |

### TC-13.6.NF2.B1 Dialogue Branch Evaluation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 100-node tree, 20 branches | Evaluation time | < 5 ms | R-13.6.NF2 |

### TC-13.6.2.B1 Prerequisite Evaluation Deep

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 10-level nested prerequisite expression | Evaluation time | < 0.1 ms | R-13.6.2 |

### TC-13.6.6.B1 Reward Distribution Group

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 5-player group reward distribution | Total time | < 1 ms | R-13.6.6 |

### TC-13.6.1.B1 Quest Graph Validation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 100-node quest graph | Validation time | < 10 ms | R-13.6.1 |
