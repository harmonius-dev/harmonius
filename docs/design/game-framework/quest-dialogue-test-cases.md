# Quest & Dialogue System Test Cases

Companion test cases for [quest-dialogue.md](quest-dialogue.md).

## Unit Tests

### TC-13.6.1.1 Quest DAG Validation Valid

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 5-node DAG with root, 3 branches, 1 terminal | `validate() == Ok(())` | R-13.6.1 |

### TC-13.6.1.2 Quest DAG Cycle Detected

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Graph with edge from node 3 back to node 1 | `Err(CycleDetected { cycle: [1, 2, 3] })` | R-13.6.1 |

### TC-13.6.1.3 Quest DAG Orphan Node

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Node 4 with no incoming or outgoing edges | `Err(OrphanNode(4))` | R-13.6.1 |

### TC-13.6.1.4 Objective Kill Progress

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Kill event matching `target_tag`, quest active | `ObjectiveProgress.current += 1` | R-13.6.1 |

### TC-13.6.1.5 Objective Collect Progress

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Add matching item to inventory, collect objective active | `ObjectiveProgress.current += 1` | R-13.6.1 |

### TC-13.6.2.1 Prerequisite And

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `And(LevelAtLeast(10), QuestCompleted(Q1))`, level=10, Q1 done | `true` | R-13.6.2 |
| 2 | Same, level=9, Q1 done | `false` | R-13.6.2 |
| 3 | Same, level=10, Q1 not done | `false` | R-13.6.2 |

### TC-13.6.2.2 Prerequisite Or

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `Or(HasItem(I1, 1), RepAtLeast(F1, 100))`, has item | `true` | R-13.6.2 |
| 2 | Same, rep=100, no item | `true` | R-13.6.2 |
| 3 | Same, no item, rep=50 | `false` | R-13.6.2 |

### TC-13.6.2.3 Prerequisite Not

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `Not(QuestCompleted(Q2))`, Q2 not done | `true` | R-13.6.2 |
| 2 | Same, Q2 done | `false` | R-13.6.2 |

### TC-13.6.2.4 Prerequisite Time Window

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `TimeWindow(Jan 1, Jan 31)`, current = Jan 15 | `true` | R-13.6.2 |
| 2 | Same, current = Feb 15 | `false` | R-13.6.2 |

### TC-13.6.2.5 Prerequisite Lazy Eval

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Track evaluation count over 60 frames without interaction | Evaluation count == 0 | R-13.6.2 |

### TC-13.6.3.1 Journal Category Filter

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 5 quests: 2 Daily, 2 Side, 1 MainStory; filter `Daily` | Returns exactly 2 Daily quest IDs | R-13.6.3 |

### TC-13.6.3.2 Journal Search

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Quests named "Dragon Hunt", "Wolf Hunt", "Fetch Water"; search "Hunt" | Returns [Dragon Hunt, Wolf Hunt] | R-13.6.3 |

### TC-13.6.NF1.1 Journal 50 Active Quests

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Activate 50 quests simultaneously | All 50 track correctly, `active_count() == 50` | R-13.6.NF1 |

### TC-13.6.4.1 Dialogue Condition Branch

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 3-branch node: quest state, rep, class conditions | Correct branch visible per player state | R-13.6.4 |

### TC-13.6.4.2 Dialogue Effect Item Grant

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Traverse node with `GrantItem { item_id: I5, count: 3 }` | 3 of item I5 added to player inventory | R-13.6.4 |

### TC-13.6.4.3 Dialogue Effect Rep Change

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Traverse node with `ChangeReputation { faction: F2, delta: 200 }` | Faction F2 rep += 200 | R-13.6.4 |

### TC-13.6.4.4 Dialogue Localization

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Load English and French locales, query node text | Distinct text per locale | R-13.6.4 |

### TC-13.6.NF2.1 Dialogue Eval Latency

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 100-node tree, 20 conditional branches | Traversal completes in < 5 ms | R-13.6.NF2 |

### TC-13.6.6.1 Reward Level Scaling

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | XP reward 1000, level curve sample at 20 = 1.5 | Granted XP = 1500 | R-13.6.6 |

### TC-13.6.6.2 Reward Choice Of N

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 3 item choices, player selects index 1 | Only item at index 1 granted | R-13.6.6 |

### TC-13.6.6.3 Reward Transactional

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Inventory full during reward grant | `Err(InventoryFull)`, no partial rewards applied | R-13.6.6 |

### TC-13.6.6.4 Reward Group Loot

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 5-player group completes quest | Each player receives rewards per loot mode | R-13.6.6 |

### TC-13.6.5b.1 Conversation State Suppress

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Start conversation with `HudSuppressionLevel::Full` | HUD hidden, audio ducked, input suppressed | R-13.6.5b |

### TC-13.6.5b.2 Conversation State Restore

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | End conversation | HUD visible, audio restored, input enabled | R-13.6.5b |

### TC-13.6.5c.1 Conversation Interrupt

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Combat event during dialogue | Gameplay restored, `ConversationInterrupted` component saved with `last_node` | R-13.6.5c |

### TC-13.6.5c.2 Conversation Resume

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Re-engage NPC after interruption | Conversation resumes from `last_node` | R-13.6.5c |

## Integration Tests

### TC-13.6.1.I1 Quest Full Lifecycle

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Accept quest, complete all objectives, turn in | Rewards granted, journal shows Completed | R-13.6.1 |

### TC-13.6.3.I1 Quest Event Propagation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Advance objective | UI, map markers, minimap all update same frame | R-13.6.3 |

### TC-13.6.1.I2 Server Rejects Tampered

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Forged quest completion from modified client | Server rejects; quest state unchanged | R-13.6.1 |

### TC-13.6.7b.I1 Phasing Two Players

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Player A at phase 1, player B at phase 2, same zone | Each sees correct sub-level | R-13.6.7b |

### TC-13.6.7b.I2 Phase Transition Swap

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Advance through phase boundary | Old sub-level unloads, new loads with correct entities | R-13.6.7b |

### TC-13.6.7a.I1 World Event Broadcast

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Trigger world event | All clients receive zone state change within 1 second | R-13.6.7a |

### TC-13.6.5c.I1 Disconnect During Dialogue

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Disconnect mid-conversation, reconnect | Conversation state preserved for resumption | R-13.6.5c |

### TC-13.6.5a.I1 Camera Multi Speaker

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Multi-NPC conversation with 3 speakers | Camera switches to face active speaker | R-13.6.5a |

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
