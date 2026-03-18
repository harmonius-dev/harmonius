# User Stories — 13.6 Quest & Dialogue

## F-13.6.1 Quest Graph System

| ID          | Persona              | Features | Requirements |
|-------------|----------------------|----------|--------------|
| US-13.6.1.1 | story director (P-4) | F-13.6.1 | R-13.6.1     |
| US-13.6.1.2 | writer (P-17)        | F-13.6.1 | R-13.6.1     |
| US-13.6.1.3 | story director (P-4) | F-13.6.1 | R-13.6.1     |
| US-13.6.1.4 | player (P-23)        | F-13.6.1 | R-13.6.1     |
| US-13.6.1.5 | engine tester (P-27) | F-13.6.1 | R-13.6.1     |
| US-13.6.1.6 | engine tester (P-27) | F-13.6.1 | R-13.6.1     |

1. **US-13.6.1.1** — I want to create quests as directed acyclic graphs of objectives,
   prerequisites, and branching paths in the visual editor so that quest flow is designed visually
   without code
2. **US-13.6.1.2** — I want to define objective types (kill, collect, escort, interact, explore,
   defend, craft) per quest node with completion criteria so that quests use the full range of
   objective mechanics
3. **US-13.6.1.3** — I want to categorize quests as main story, side quest, daily/weekly repeatable,
   world quest, or seasonal event chain so that the quest journal can sort and filter by type
4. **US-13.6.1.4** — I want to see quest objectives update in real time with progress counters,
   waypoint markers, and zone indicators so that I always know what to do next
5. **US-13.6.1.5** — I want to attempt to complete a quest objective from a modified client and
   verify the server rejects it so that quest progression cannot be exploited
6. **US-13.6.1.6** — I want to create a quest graph with a cycle and verify the editor rejects it
   with a clear error so that invalid quest structures are caught at author time

## F-13.6.2 Quest Prerequisites and Gating

| ID          | Persona              | Features | Requirements |
|-------------|----------------------|----------|--------------|
| US-13.6.2.1 | story director (P-4) | F-13.6.2 | R-13.6.2     |
| US-13.6.2.2 | writer (P-17)        | F-13.6.2 | R-13.6.2     |
| US-13.6.2.3 | player (P-23)        | F-13.6.2 | R-13.6.2     |
| US-13.6.2.4 | engine tester (P-27) | F-13.6.2 | R-13.6.2     |

1. **US-13.6.2.1** — I want to define prerequisites using boolean expressions (AND/OR/NOT) over
   completed quests, character level, faction reputation, item possession, and achievement unlocks
   so that quest availability reflects meaningful progression
2. **US-13.6.2.2** — I want to gate quest availability by real-world calendar date and time-of-day
   so that seasonal events appear only during their intended period
3. **US-13.6.2.3** — I want to new quests to become available when I meet their prerequisites so
   that progression naturally unlocks new content
4. **US-13.6.2.4** — I want to verify that prerequisites are evaluated only on player interaction
   with quest givers or trigger volumes so that prerequisite checks do not consume resources
   continuously

## F-13.6.3 Quest Tracking and Journal

| ID          | Persona              | Features | Requirements |
|-------------|----------------------|----------|--------------|
| US-13.6.3.1 | writer (P-17)        | F-13.6.3 | R-13.6.3     |
| US-13.6.3.2 | player (P-23)        | F-13.6.3 | R-13.6.3     |
| US-13.6.3.3 | player (P-23)        | F-13.6.3 | R-13.6.3     |
| US-13.6.3.4 | engine tester (P-27) | F-13.6.3 | R-13.6.3     |

1. **US-13.6.3.1** — I want to assign quests to journal categories (main story, side, daily, weekly,
   event) with sorting and filtering support so that the journal is organized for the player
2. **US-13.6.3.2** — I want to browse active, completed, and abandoned quests with category
   filtering and text search so that I can find any quest quickly
3. **US-13.6.3.3** — I want to see objective waypoints and zone indicators on the map and HUD for my
   active quests so that I know where to go without consulting external guides
4. **US-13.6.3.4** — I want to advance a quest objective and verify that the UI, map markers, and
   minimap all update within the same frame so that quest state changes propagate consistently

## F-13.6.4 Dialogue Tree System

| ID          | Persona              | Features | Requirements |
|-------------|----------------------|----------|--------------|
| US-13.6.4.1 | story director (P-4) | F-13.6.4 | R-13.6.4     |
| US-13.6.4.2 | writer (P-17)        | F-13.6.4 | R-13.6.4     |
| US-13.6.4.3 | player (P-23)        | F-13.6.4 | R-13.6.4     |
| US-13.6.4.4 | engine tester (P-27) | F-13.6.4 | R-13.6.4     |

1. **US-13.6.4.1** — I want to create branching dialogue trees with NPC lines, player response
   options, and conditional branches in the visual editor so that narrative conversations are
   designed without code
2. **US-13.6.4.2** — I want to write dialogue text with localization keys and audio references per
   node so that dialogue content supports multiple languages and voice-over
3. **US-13.6.4.3** — I want to choose dialogue responses that affect quest state, reputation, or
   available rewards so that conversations feel consequential
4. **US-13.6.4.4** — I want to set up dialogue branches conditioned on quest state, faction
   reputation, inventory contents, and class, then verify each branch activates only when its
   conditions are met so that branching logic is correct

## F-13.6.5a Conversation Camera and Framing

| ID        | Persona              | Features | Requirements |
|-----------|----------------------|----------|--------------|
| US-13.6.5 | story director (P-4) | F-13.6.5 | R-13.6.5     |
| US-13.6.5 | writer (P-17)        | F-13.6.5 | R-13.6.5     |
| US-13.6.5 | player (P-23)        | F-13.6.5 | R-13.6.5     |
| US-13.6.5 | engine tester (P-27) | F-13.6.5 | R-13.6.5     |

1. **US-13.6.5** — I want to configure over-the-shoulder shots, close-ups, and automatic camera
   switching between speakers in multi-NPC conversations so that dialogue scenes feel cinematic
2. **US-13.6.5** — I want to author conversations where several NPCs speak in sequence with
   automatic camera focus changes so that group conversations flow naturally
3. **US-13.6.5** — I want to the camera to frame NPC conversations with appropriate shots and
   character facing so that dialogue feels immersive
4. **US-13.6.5** — I want to start a multi-NPC conversation and verify the camera switches to face
   each speaker when they deliver their line so that camera behavior matches the dialogue sequence

## F-13.6.5b Conversation Gameplay State

| ID        | Persona              | Features | Requirements |
|-----------|----------------------|----------|--------------|
| US-13.6.5 | story director (P-4) | F-13.6.5 | R-13.6.5     |
| US-13.6.5 | writer (P-17)        | F-13.6.5 | R-13.6.5     |
| US-13.6.5 | player (P-23)        | F-13.6.5 | R-13.6.5     |
| US-13.6.5 | engine tester (P-27) | F-13.6.5 | R-13.6.5     |

1. **US-13.6.5** — I want to configure which HUD elements are suppressed during each conversation
   (full suppression vs retain minimap and health) so that conversation immersion is tunable per
   dialogue asset
2. **US-13.6.5** — I want to conversations to automatically duck ambient audio so that dialogue is
   clearly audible without manual volume adjustment
3. **US-13.6.5** — I want to gameplay inputs to be suppressed during conversations so that I do not
   accidentally attack or move while reading dialogue
4. **US-13.6.5** — I want to start and end a conversation and verify that HUD elements, audio
   levels, and input bindings return to their pre-conversation state so that no state leaks after
   dialogue

## F-13.6.5c Conversation Interruption and Resumption

| ID        | Persona              | Features | Requirements |
|-----------|----------------------|----------|--------------|
| US-13.6.5 | story director (P-4) | F-13.6.5 | R-13.6.5     |
| US-13.6.5 | player (P-23)        | F-13.6.5 | R-13.6.5     |
| US-13.6.5 | engine tester (P-27) | F-13.6.5 | R-13.6.5     |
| US-13.6.5 | engine tester (P-27) | F-13.6.5 | R-13.6.5     |

1. **US-13.6.5** — I want to conversations interrupted by combat to immediately restore gameplay
   state (audio, HUD, input) so that players can defend themselves without delay
2. **US-13.6.5** — I want to resume an interrupted conversation from the last visited dialogue node
   so that I do not have to re-read dialogue I have already seen
3. **US-13.6.5** — I want to interrupt a conversation and verify it is marked as incomplete with the
   current node index saved so that resumption returns to the correct dialogue position
4. **US-13.6.5** — I want to disconnect during a conversation and reconnect, then verify the
   conversation state is preserved for resumption so that network issues do not lose dialogue
   progress

## F-13.6.6 Quest Rewards and Economy Hooks

| ID          | Persona              | Features | Requirements |
|-------------|----------------------|----------|--------------|
| US-13.6.6.1 | story director (P-4) | F-13.6.6 | R-13.6.6     |
| US-13.6.6.2 | writer (P-17)        | F-13.6.6 | R-13.6.6     |
| US-13.6.6.3 | player (P-23)        | F-13.6.6 | R-13.6.6     |
| US-13.6.6.4 | engine tester (P-27) | F-13.6.6 | R-13.6.6     |

1. **US-13.6.6.1** — I want to define per-quest reward tables with XP, currency, items (fixed or
   choice-of-N), reputation, and achievement credit so that quest rewards are authored as data
2. **US-13.6.6.2** — I want to rewards to scale with character level and support seasonal
   multipliers so that quest rewards remain relevant as players progress
3. **US-13.6.6.3** — I want to select my preferred reward from choice-of-N options when completing a
   quest so that I receive gear suited to my character
4. **US-13.6.6.4** — I want to interrupt a reward grant mid-distribution and verify either all
   rewards were granted or none were so that partial grants and duplication exploits are impossible

## F-13.6.7a Server-Driven World Events

| ID        | Persona              | Features | Requirements |
|-----------|----------------------|----------|--------------|
| US-13.6.7 | story director (P-4) | F-13.6.7 | R-13.6.7     |
| US-13.6.7 | player (P-23)        | F-13.6.7 | R-13.6.7     |
| US-13.6.7 | engine tester (P-27) | F-13.6.7 | R-13.6.7     |

1. **US-13.6.7** — I want to define server-driven world events (invasions, world bosses, festivals)
   triggered by time, player count, or quest completion rates so that events activate based on world
   state
2. **US-13.6.7** — I want to see invasion spawns, world boss activations, and seasonal festivals
   that affect all players in a zone simultaneously so that the world feels dynamic and shared
3. **US-13.6.7** — I want to trigger a world event and verify all connected clients in the zone
   receive the state change within 1 second so that events are synchronized across players

## F-13.6.7b Quest Phasing System

| ID        | Persona              | Features | Requirements |
|-----------|----------------------|----------|--------------|
| US-13.6.7 | story director (P-4) | F-13.6.7 | R-13.6.7     |
| US-13.6.7 | player (P-23)        | F-13.6.7 | R-13.6.7     |
| US-13.6.7 | engine tester (P-27) | F-13.6.7 | R-13.6.7     |
| US-13.6.7 | engine tester (P-27) | F-13.6.7 | R-13.6.7     |

1. **US-13.6.7** — I want to author phase mappings as data assets that define which sub-level
   version each quest stage sees so that phased content is configured visually
2. **US-13.6.7** — I want to see different versions of a zone based on my quest progress (e.g., a
   town rebuilt after a siege) so that my choices visibly change the world
3. **US-13.6.7** — I want to have two players at different quest stages in the same zone and verify
   each sees the correct phase without affecting the other so that phasing is per-player
4. **US-13.6.7** — I want to advance through a quest phase boundary and verify the old sub-level
   unloads and the new sub-level loads with correct entity placement so that phase swaps are
   seamless
