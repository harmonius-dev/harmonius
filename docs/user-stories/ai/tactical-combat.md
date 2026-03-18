# User Stories — 7.8 Tactical Combat AI

## F-7.8.1 — Cover Evaluation and Scoring

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-7.8.1.1  | designer (P-5)          | F-7.8.1  | R-7.8.1      |
| US-7.8.1.2  | designer (P-5)          | F-7.8.1  | R-7.8.1      |
| US-7.8.1.3  | designer (P-5)          | F-7.8.1  | R-7.8.1      |
| US-7.8.1.4  | player (P-23)           | F-7.8.1  | R-7.8.1      |
| US-7.8.1.5  | player (P-23)           | F-7.8.1  | R-7.8.1      |
| US-7.8.1.6  | player (P-23)           | F-7.8.1  | R-7.8.1      |
| US-7.8.1.7  | engine developer (P-26) | F-7.8.1  | R-7.8.1      |
| US-7.8.1.8  | engine developer (P-26) | F-7.8.1  | R-7.8.1      |
| US-7.8.1.9  | engine developer (P-26) | F-7.8.1  | R-7.8.1      |
| US-7.8.1.10 | engine tester (P-27)    | F-7.8.1  | R-7.8.1      |
| US-7.8.1.11 | engine tester (P-27)    | F-7.8.1  | R-7.8.1      |
| US-7.8.1.12 | engine tester (P-27)    | F-7.8.1  | R-7.8.1      |

1. **US-7.8.1.1** — I want to configure cover scoring weights (protection angle, sight lines,
   flanking exposure, distance, objective proximity) per AI archetype
   - **Acceptance:** cautious AI prioritizes protection while aggressive AI prioritizes sight lines
2. **US-7.8.1.2** — I want to see pre-computed cover positions with quality scores as debug overlays
   in the editor
   - **Acceptance:** I can verify cover placement in my level
3. **US-7.8.1.3** — I want to configure when AI re-evaluates cover (target moves, damage from
   unexpected direction, cover destroyed)
   - **Acceptance:** cover decisions stay current
4. **US-7.8.1.4** — I want enemies to move to cover positions during combat, hiding behind walls and
   objects
   - **Acceptance:** combat AI feels tactically aware
5. **US-7.8.1.5** — I want enemies to abandon a cover position and find a better one when I flank
   them
   - **Acceptance:** AI adapts to my tactical positioning
6. **US-7.8.1.6** — I want enemies defending an objective to choose cover near that objective rather
   than running to distant safety
   - **Acceptance:** defenders hold their ground
7. **US-7.8.1.7** — I want to implement a cover scoring system evaluating protection angle, sight
   lines, flanking exposure, target distance, and objective proximity
   - **Acceptance:** cover selection is context-aware
8. **US-7.8.1.8** — I want to pre-compute cover positions from world geometry and cache them in the
   shared spatial index
   - **Acceptance:** runtime queries are fast
9. **US-7.8.1.9** — I want to evaluate 4 cover candidates per query on mobile (vs. 16 on desktop)
   - **Acceptance:** spatial query cost fits the mobile budget
10. **US-7.8.1.10** — I want to verify that cautious AI selects high-protection cover and aggressive
    AI selects cover with good sight lines
    - **Acceptance:** archetype weights work correctly
11. **US-7.8.1.11** — I want to verify that AI re-evaluates cover when damaged from an unexpected
    direction and moves to a better position
    - **Acceptance:** re-evaluation triggers work
12. **US-7.8.1.12** — I want to benchmark cover evaluation query time for the maximum candidate
    count
    - **Acceptance:** cover selection fits within the per-tick budget. ---

## F-7.8.2 — Flanking and Pincer Behavior

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-7.8.2.1  | designer (P-5)          | F-7.8.2  | R-7.8.2      |
| US-7.8.2.2  | designer (P-5)          | F-7.8.2  | R-7.8.2      |
| US-7.8.2.3  | designer (P-5)          | F-7.8.2  | R-7.8.2      |
| US-7.8.2.4  | player (P-23)           | F-7.8.2  | R-7.8.2      |
| US-7.8.2.5  | player (P-23)           | F-7.8.2  | R-7.8.2      |
| US-7.8.2.6  | player (P-23)           | F-7.8.2  | R-7.8.2      |
| US-7.8.2.7  | engine developer (P-26) | F-7.8.2  | R-7.8.2      |
| US-7.8.2.8  | engine developer (P-26) | F-7.8.2  | R-7.8.2      |
| US-7.8.2.9  | engine developer (P-26) | F-7.8.2  | R-7.8.2      |
| US-7.8.2.10 | engine tester (P-27)    | F-7.8.2  | R-7.8.2      |
| US-7.8.2.11 | engine tester (P-27)    | F-7.8.2  | R-7.8.2      |
| US-7.8.2.12 | engine tester (P-27)    | F-7.8.2  | R-7.8.2      |

1. **US-7.8.2.1** — I want to configure the minimum squad size that triggers flanking behavior
   - **Acceptance:** solo enemies attack directly while groups coordinate
2. **US-7.8.2.2** — I want to set how aggressively flanking paths avoid the target's line of sight
   - **Acceptance:** flanking stealth matches the tactical scenario
3. **US-7.8.2.3** — I want to configure how long flankers wait at staging positions before attacking
   simultaneously
   - **Acceptance:** coordination timing is tunable
4. **US-7.8.2.4** — I want enemy squads to approach me from multiple angles, with some pinning me
   down while others flank
   - **Acceptance:** combat feels tactically challenging
5. **US-7.8.2.5** — I want flanking enemies to wait at staging positions until all are ready, then
   attack at once
   - **Acceptance:** coordinated assault feels planned
6. **US-7.8.2.6** — I want a lone enemy to attack directly without attempting to flank
   - **Acceptance:** flanking requires realistic squad coordination
7. **US-7.8.2.7** — I want to implement squad leader designation of flanking assignments (frontal
   pressure, flank left, flank right)
   - **Acceptance:** coordinated movement emerges from role assignment
8. **US-7.8.2.8** — I want to route flanking paths using geometry occlusion checks to avoid the
   target's line of sight
   - **Acceptance:** flankers approach undetected
9. **US-7.8.2.9** — I want mobile to disable coordinated flanking and have agents attack
   independently
   - **Acceptance:** squad coordination cost is eliminated on mobile
10. **US-7.8.2.10** — I want to verify that flankers approach from the target's flank or rear (not
    front)
    - **Acceptance:** flanking geometry is correct
11. **US-7.8.2.11** — I want to verify that flankers wait at staging positions until all are ready
    before attacking
    - **Acceptance:** synchronization works
12. **US-7.8.2.12** — I want to confirm that mobile builds never attempt coordinated flanking and
    agents attack independently
    - **Acceptance:** the platform gate works. ---

## F-7.8.3 — Squad Formation and Communication

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-7.8.3.1  | designer (P-5)          | F-7.8.3  | R-7.8.3      |
| US-7.8.3.2  | designer (P-5)          | F-7.8.3  | R-7.8.3      |
| US-7.8.3.3  | designer (P-5)          | F-7.8.3  | R-7.8.3      |
| US-7.8.3.4  | player (P-23)           | F-7.8.3  | R-7.8.3      |
| US-7.8.3.5  | player (P-23)           | F-7.8.3  | R-7.8.3      |
| US-7.8.3.6  | player (P-23)           | F-7.8.3  | R-7.8.3      |
| US-7.8.3.7  | engine developer (P-26) | F-7.8.3  | R-7.8.3      |
| US-7.8.3.8  | engine developer (P-26) | F-7.8.3  | R-7.8.3      |
| US-7.8.3.9  | engine developer (P-26) | F-7.8.3  | R-7.8.3      |
| US-7.8.3.10 | engine tester (P-27)    | F-7.8.3  | R-7.8.3      |
| US-7.8.3.11 | engine tester (P-27)    | F-7.8.3  | R-7.8.3      |
| US-7.8.3.12 | engine tester (P-27)    | F-7.8.3  | R-7.8.3      |

1. **US-7.8.3.1** — I want to configure which formation shape a squad uses per context (wedge for
   open terrain, column for corridors, line for sweeping)
   - **Acceptance:** formation adapts to the environment
2. **US-7.8.3.2** — I want to configure squad communication barks ("contact front-left", "flanking
   right", "retreat")
   - **Acceptance:** squad chatter is audible and informative to the player
3. **US-7.8.3.3** — I want to place rally points in the level where squads regroup after combat
   - **Acceptance:** post-combat behavior is designer-controlled
4. **US-7.8.3.4** — I want enemy squads to move in formation (wedge, column, line) through the level
   - **Acceptance:** military AI looks organized and professional
5. **US-7.8.3.5** — I want hear squad members call out target positions ("contact front-left") and
   flanking moves
   - **Acceptance:** communication adds atmosphere and information
6. **US-7.8.3.6** — I want scattered squad members to regroup at a rally point after combat, with
   stragglers rejoining the formation
   - **Acceptance:** post-combat recovery looks realistic
7. **US-7.8.3.7** — I want to implement formation selection based on environmental context (open
   terrain, corridor, sweep area)
   - **Acceptance:** squads automatically adapt their shape
8. **US-7.8.3.8** — I want to implement a squad communication system that sends functional messages
   (target position, flanking call, retreat order) and plays bark audio
   - **Acceptance:** communication is both functional and presentational
9. **US-7.8.3.9** — I want mobile to limit squad size to 4 (vs. 8 on desktop) and use column
   formation only
   - **Acceptance:** formation cost is reduced
10. **US-7.8.3.10** — I want to verify that the squad selects the correct formation shape for each
    environmental context
    - **Acceptance:** context-adaptive selection works
11. **US-7.8.3.11** — I want to verify that all surviving squad members reach the rally point after
    combat and reform their formation
    - **Acceptance:** regrouping works
12. **US-7.8.3.12** — I want to verify that squad communication messages (target call, retreat
    order) actually affect receiving agents' behavior tree decisions
    - **Acceptance:** communication is functional. ---

## F-7.8.4 — Suppressive Fire and Pinning

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-7.8.4.1  | designer (P-5)          | F-7.8.4  | R-7.8.4      |
| US-7.8.4.2  | designer (P-5)          | F-7.8.4  | R-7.8.4      |
| US-7.8.4.3  | designer (P-5)          | F-7.8.4  | R-7.8.4      |
| US-7.8.4.4  | player (P-23)           | F-7.8.4  | R-7.8.4      |
| US-7.8.4.5  | player (P-23)           | F-7.8.4  | R-7.8.4      |
| US-7.8.4.6  | player (P-23)           | F-7.8.4  | R-7.8.4      |
| US-7.8.4.7  | engine developer (P-26) | F-7.8.4  | R-7.8.4      |
| US-7.8.4.8  | engine developer (P-26) | F-7.8.4  | R-7.8.4      |
| US-7.8.4.9  | engine developer (P-26) | F-7.8.4  | R-7.8.4      |
| US-7.8.4.10 | engine tester (P-27)    | F-7.8.4  | R-7.8.4      |
| US-7.8.4.11 | engine tester (P-27)    | F-7.8.4  | R-7.8.4      |
| US-7.8.4.12 | engine tester (P-27)    | F-7.8.4  | R-7.8.4      |

1. **US-7.8.4.1** — I want to configure suppression zone radius, duration, and accuracy penalty per
   weapon type
   - **Acceptance:** suppression intensity matches the weapon
2. **US-7.8.4.2** — I want to configure the "suppressed" debuff effects (accuracy penalty, camera
   shake, reduced return fire)
   - **Acceptance:** being suppressed feels impactful
3. **US-7.8.4.3** — I want to configure when AI uses suppressive fire (while flankers move, when
   target enters cover, on retreat)
   - **Acceptance:** suppression is used tactically
4. **US-7.8.4.4** — I want feel pinned down when enemies lay suppressive fire on my position, with
   camera effects and accuracy penalties
   - **Acceptance:** suppression creates tactical pressure
5. **US-7.8.4.5** — I want one enemy to suppress my cover while another flanks
   - **Acceptance:** combined tactics force me to reposition
6. **US-7.8.4.6** — I want suppressive fire to stop when the suppressor runs low on ammo
   - **Acceptance:** suppression has a resource cost and a natural end
7. **US-7.8.4.7** — I want to implement suppressive fire targeting a zone rather than an entity,
   maintaining fire at a point
   - **Acceptance:** area denial is distinct from aimed fire
8. **US-7.8.4.8** — I want to apply a "suppressed" debuff to all entities in the suppression zone
   - **Acceptance:** suppression effects are gameplay-impactful
9. **US-7.8.4.9** — I want mobile to disable coordinated suppression (which requires flanking),
   while keeping the suppressed debuff and camera effects
   - **Acceptance:** suppression feel is preserved without coordination cost
10. **US-7.8.4.10** — I want to verify that entities within the suppression zone receive the
    suppressed debuff with correct accuracy penalty
    - **Acceptance:** zone effects work
11. **US-7.8.4.11** — I want to verify that suppressive fire ceases when the AI exhausts its ammo
    - **Acceptance:** resource management is enforced
12. **US-7.8.4.12** — I want to verify that entities outside the suppression zone do not receive the
    suppressed debuff
    - **Acceptance:** zone boundaries are accurate. ---

## F-7.8.5 — Search and Investigation Patterns

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-7.8.5.1  | designer (P-5)          | F-7.8.5  | R-7.8.5      |
| US-7.8.5.2  | designer (P-5)          | F-7.8.5  | R-7.8.5      |
| US-7.8.5.3  | designer (P-5)          | F-7.8.5  | R-7.8.5      |
| US-7.8.5.4  | player (P-23)           | F-7.8.5  | R-7.8.5      |
| US-7.8.5.5  | player (P-23)           | F-7.8.5  | R-7.8.5      |
| US-7.8.5.6  | player (P-23)           | F-7.8.5  | R-7.8.5      |
| US-7.8.5.7  | engine developer (P-26) | F-7.8.5  | R-7.8.5      |
| US-7.8.5.8  | engine developer (P-26) | F-7.8.5  | R-7.8.5      |
| US-7.8.5.9  | engine developer (P-26) | F-7.8.5  | R-7.8.5      |
| US-7.8.5.10 | engine tester (P-27)    | F-7.8.5  | R-7.8.5      |
| US-7.8.5.11 | engine tester (P-27)    | F-7.8.5  | R-7.8.5      |
| US-7.8.5.12 | engine tester (P-27)    | F-7.8.5  | R-7.8.5      |

1. **US-7.8.5.1** — I want to configure search pattern types (spiral from LKP, check known hiding
   spots, room-by-room sweep)
   - **Acceptance:** search behavior matches the level design
2. **US-7.8.5.2** — I want to set a search timeout after which AI returns to patrol, with intensity
   decreasing over time
   - **Acceptance:** searches feel natural and finite
3. **US-7.8.5.3** — I want search patterns to be authored as behavior tree subtrees with
   configurable thoroughness
   - **Acceptance:** I can create and reuse search behaviors
4. **US-7.8.5.4** — I want enemies to systematically search the area after I break contact, checking
   cover points, rooms, and corners
   - **Acceptance:** escape feels tense
5. **US-7.8.5.5** — I want squad members to divide the search area among themselves, each checking
   different rooms
   - **Acceptance:** coordinated search feels efficient
6. **US-7.8.5.6** — I want enemies to give up searching after a configurable timeout and return to
   patrol
   - **Acceptance:** hiding long enough is a valid escape strategy
7. **US-7.8.5.7** — I want to implement search patterns (spiral, hiding spot check, room sweep) as
   configurable behaviors driven by spatial queries
   - **Acceptance:** search is systematic
8. **US-7.8.5.8** — I want to coordinate squad members to divide the search area and avoid redundant
   searches
   - **Acceptance:** team search is efficient
9. **US-7.8.5.9** — I want mobile to use simplified search (move to LKP, scan once, return to
   patrol), disabling coordinated squad search
   - **Acceptance:** search cost fits the mobile budget
10. **US-7.8.5.10** — I want to verify that a thorough search pattern checks all known hiding spots
    in the search area
    - **Acceptance:** no cover points are missed
11. **US-7.8.5.11** — I want to verify that the search ends after the configured timeout and the
    agent transitions to patrol state
    - **Acceptance:** timeout works
12. **US-7.8.5.12** — I want to verify that re-acquiring the target during search immediately
    transitions the agent to combat state
    - **Acceptance:** search-to-combat transition is instant. ---

## F-7.8.6 — Retreat and Fallback Behavior

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-7.8.6.1  | designer (P-5)          | F-7.8.6  | R-7.8.6      |
| US-7.8.6.2  | designer (P-5)          | F-7.8.6  | R-7.8.6      |
| US-7.8.6.3  | designer (P-5)          | F-7.8.6  | R-7.8.6      |
| US-7.8.6.4  | player (P-23)           | F-7.8.6  | R-7.8.6      |
| US-7.8.6.5  | player (P-23)           | F-7.8.6  | R-7.8.6      |
| US-7.8.6.6  | player (P-23)           | F-7.8.6  | R-7.8.6      |
| US-7.8.6.7  | engine developer (P-26) | F-7.8.6  | R-7.8.6      |
| US-7.8.6.8  | engine developer (P-26) | F-7.8.6  | R-7.8.6      |
| US-7.8.6.9  | engine developer (P-26) | F-7.8.6  | R-7.8.6      |
| US-7.8.6.10 | engine tester (P-27)    | F-7.8.6  | R-7.8.6      |
| US-7.8.6.11 | engine tester (P-27)    | F-7.8.6  | R-7.8.6      |
| US-7.8.6.12 | engine tester (P-27)    | F-7.8.6  | R-7.8.6      |

1. **US-7.8.6.1** — I want to configure retreat triggers (health below threshold, cover destroyed,
   outnumbered, squad leader order)
   - **Acceptance:** retreat behavior matches the combat scenario
2. **US-7.8.6.2** — I want to set a casualty threshold that triggers squad-wide retreat
   - **Acceptance:** entire squads fall back when casualties are too high
3. **US-7.8.6.3** — I want to configure which abilities retreating AI uses to cover withdrawal
   (smoke grenades, suppressive fire)
   - **Acceptance:** retreat is tactically varied
4. **US-7.8.6.4** — I want enemies to retreat to secondary cover positions when their health drops
   low
   - **Acceptance:** AI shows self-preservation
5. **US-7.8.6.5** — I want an entire squad to fall back when casualties exceed a threshold
   - **Acceptance:** morale-based retreat makes combat feel dynamic
6. **US-7.8.6.6** — I want retreating enemies to throw smoke grenades to cover their withdrawal
   - **Acceptance:** retreat behavior feels tactical and deliberate
7. **US-7.8.6.7** — I want to select retreat destinations using the cover evaluation system, finding
   positions further from threats
   - **Acceptance:** retreat targets are tactically sound
8. **US-7.8.6.8** — I want retreating agents to use smoke grenades or suppressive fire to cover
   withdrawal when available in their ability set
   - **Acceptance:** retreat is not a simple flee
9. **US-7.8.6.9** — I want mobile to use simplified retreat (flee directly away from threat) without
   cover re-evaluation or squad coordination
   - **Acceptance:** retreat cost is minimal
10. **US-7.8.6.10** — I want to verify that individual retreat triggers at the configured health
    threshold and squad retreat triggers at the configured casualty count
    - **Acceptance:** thresholds are correct
11. **US-7.8.6.11** — I want to verify that retreat destinations are further from the threat source
    than the current position
    - **Acceptance:** retreat moves away from danger
12. **US-7.8.6.12** — I want to verify that AI at a fallback position reassesses and re-engages if
    conditions improve, or continues retreating if outmatched
    - **Acceptance:** post-retreat behavior adapts correctly
