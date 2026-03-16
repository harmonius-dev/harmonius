# User Stories — 7.8 Tactical Combat AI

## F-7.8.1 — Cover Evaluation and Scoring

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.8.1.1 | designer (P-5) | I want to configure cover scoring weights (protection angle, sight lines, flanking exposure, distance, objective proximity) per AI archetype | cautious AI prioritizes protection while aggressive AI prioritizes sight lines | F-7.8.1 | R-7.8.1 |
| US-7.8.1.2 | designer (P-5) | I want to see pre-computed cover positions with quality scores as debug overlays in the editor | I can verify cover placement in my level | F-7.8.1 | R-7.8.1 |
| US-7.8.1.3 | designer (P-5) | I want to configure when AI re-evaluates cover (target moves, damage from unexpected direction, cover destroyed) | cover decisions stay current | F-7.8.1 | R-7.8.1 |
| US-7.8.1.4 | player (P-23) | I want enemies to move to cover positions during combat, hiding behind walls and objects | combat AI feels tactically aware | F-7.8.1 | R-7.8.1 |
| US-7.8.1.5 | player (P-23) | I want enemies to abandon a cover position and find a better one when I flank them | AI adapts to my tactical positioning | F-7.8.1 | R-7.8.1 |
| US-7.8.1.6 | player (P-23) | I want enemies defending an objective to choose cover near that objective rather than running to distant safety | defenders hold their ground | F-7.8.1 | R-7.8.1 |
| US-7.8.1.7 | engine developer (P-26) | I want to implement a cover scoring system evaluating protection angle, sight lines, flanking exposure, target distance, and objective proximity | cover selection is context-aware | F-7.8.1 | R-7.8.1 |
| US-7.8.1.8 | engine developer (P-26) | I want to pre-compute cover positions from world geometry and cache them in the shared spatial index | runtime queries are fast | F-7.8.1 | R-7.8.1 |
| US-7.8.1.9 | engine developer (P-26) | I want to evaluate 4 cover candidates per query on mobile (vs. 16 on desktop) | spatial query cost fits the mobile budget | F-7.8.1 | R-7.8.1 |
| US-7.8.1.10 | engine tester (P-27) | I want to verify that cautious AI selects high-protection cover and aggressive AI selects cover with good sight lines | archetype weights work correctly | F-7.8.1 | R-7.8.1 |
| US-7.8.1.11 | engine tester (P-27) | I want to verify that AI re-evaluates cover when damaged from an unexpected direction and moves to a better position | re-evaluation triggers work | F-7.8.1 | R-7.8.1 |
| US-7.8.1.12 | engine tester (P-27) | I want to benchmark cover evaluation query time for the maximum candidate count | cover selection fits within the per-tick budget. --- | F-7.8.1 | R-7.8.1 |

## F-7.8.2 — Flanking and Pincer Behavior

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.8.2.1 | designer (P-5) | I want to configure the minimum squad size that triggers flanking behavior | solo enemies attack directly while groups coordinate | F-7.8.2 | R-7.8.2 |
| US-7.8.2.2 | designer (P-5) | I want to set how aggressively flanking paths avoid the target's line of sight | flanking stealth matches the tactical scenario | F-7.8.2 | R-7.8.2 |
| US-7.8.2.3 | designer (P-5) | I want to configure how long flankers wait at staging positions before attacking simultaneously | coordination timing is tunable | F-7.8.2 | R-7.8.2 |
| US-7.8.2.4 | player (P-23) | I want enemy squads to approach me from multiple angles, with some pinning me down while others flank | combat feels tactically challenging | F-7.8.2 | R-7.8.2 |
| US-7.8.2.5 | player (P-23) | I want flanking enemies to wait at staging positions until all are ready, then attack at once | coordinated assault feels planned | F-7.8.2 | R-7.8.2 |
| US-7.8.2.6 | player (P-23) | I want a lone enemy to attack directly without attempting to flank | flanking requires realistic squad coordination | F-7.8.2 | R-7.8.2 |
| US-7.8.2.7 | engine developer (P-26) | I want to implement squad leader designation of flanking assignments (frontal pressure, flank left, flank right) | coordinated movement emerges from role assignment | F-7.8.2 | R-7.8.2 |
| US-7.8.2.8 | engine developer (P-26) | I want to route flanking paths using geometry occlusion checks to avoid the target's line of sight | flankers approach undetected | F-7.8.2 | R-7.8.2 |
| US-7.8.2.9 | engine developer (P-26) | I want mobile to disable coordinated flanking and have agents attack independently | squad coordination cost is eliminated on mobile | F-7.8.2 | R-7.8.2 |
| US-7.8.2.10 | engine tester (P-27) | I want to verify that flankers approach from the target's flank or rear (not front) | flanking geometry is correct | F-7.8.2 | R-7.8.2 |
| US-7.8.2.11 | engine tester (P-27) | I want to verify that flankers wait at staging positions until all are ready before attacking | synchronization works | F-7.8.2 | R-7.8.2 |
| US-7.8.2.12 | engine tester (P-27) | I want to confirm that mobile builds never attempt coordinated flanking and agents attack independently | the platform gate works. --- | F-7.8.2 | R-7.8.2 |

## F-7.8.3 — Squad Formation and Communication

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.8.3.1 | designer (P-5) | I want to configure which formation shape a squad uses per context (wedge for open terrain, column for corridors, line for sweeping) | formation adapts to the environment | F-7.8.3 | R-7.8.3 |
| US-7.8.3.2 | designer (P-5) | I want to configure squad communication barks ("contact front-left", "flanking right", "retreat") | squad chatter is audible and informative to the player | F-7.8.3 | R-7.8.3 |
| US-7.8.3.3 | designer (P-5) | I want to place rally points in the level where squads regroup after combat | post-combat behavior is designer-controlled | F-7.8.3 | R-7.8.3 |
| US-7.8.3.4 | player (P-23) | I want enemy squads to move in formation (wedge, column, line) through the level | military AI looks organized and professional | F-7.8.3 | R-7.8.3 |
| US-7.8.3.5 | player (P-23) | I want hear squad members call out target positions ("contact front-left") and flanking moves | communication adds atmosphere and information | F-7.8.3 | R-7.8.3 |
| US-7.8.3.6 | player (P-23) | I want scattered squad members to regroup at a rally point after combat, with stragglers rejoining the formation | post-combat recovery looks realistic | F-7.8.3 | R-7.8.3 |
| US-7.8.3.7 | engine developer (P-26) | I want to implement formation selection based on environmental context (open terrain, corridor, sweep area) | squads automatically adapt their shape | F-7.8.3 | R-7.8.3 |
| US-7.8.3.8 | engine developer (P-26) | I want to implement a squad communication system that sends functional messages (target position, flanking call, retreat order) and plays bark audio | communication is both functional and presentational | F-7.8.3 | R-7.8.3 |
| US-7.8.3.9 | engine developer (P-26) | I want mobile to limit squad size to 4 (vs. 8 on desktop) and use column formation only | formation cost is reduced | F-7.8.3 | R-7.8.3 |
| US-7.8.3.10 | engine tester (P-27) | I want to verify that the squad selects the correct formation shape for each environmental context | context-adaptive selection works | F-7.8.3 | R-7.8.3 |
| US-7.8.3.11 | engine tester (P-27) | I want to verify that all surviving squad members reach the rally point after combat and reform their formation | regrouping works | F-7.8.3 | R-7.8.3 |
| US-7.8.3.12 | engine tester (P-27) | I want to verify that squad communication messages (target call, retreat order) actually affect receiving agents' behavior tree decisions | communication is functional. --- | F-7.8.3 | R-7.8.3 |

## F-7.8.4 — Suppressive Fire and Pinning

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.8.4.1 | designer (P-5) | I want to configure suppression zone radius, duration, and accuracy penalty per weapon type | suppression intensity matches the weapon | F-7.8.4 | R-7.8.4 |
| US-7.8.4.2 | designer (P-5) | I want to configure the "suppressed" debuff effects (accuracy penalty, camera shake, reduced return fire) | being suppressed feels impactful | F-7.8.4 | R-7.8.4 |
| US-7.8.4.3 | designer (P-5) | I want to configure when AI uses suppressive fire (while flankers move, when target enters cover, on retreat) | suppression is used tactically | F-7.8.4 | R-7.8.4 |
| US-7.8.4.4 | player (P-23) | I want feel pinned down when enemies lay suppressive fire on my position, with camera effects and accuracy penalties | suppression creates tactical pressure | F-7.8.4 | R-7.8.4 |
| US-7.8.4.5 | player (P-23) | I want one enemy to suppress my cover while another flanks | combined tactics force me to reposition | F-7.8.4 | R-7.8.4 |
| US-7.8.4.6 | player (P-23) | I want suppressive fire to stop when the suppressor runs low on ammo | suppression has a resource cost and a natural end | F-7.8.4 | R-7.8.4 |
| US-7.8.4.7 | engine developer (P-26) | I want to implement suppressive fire targeting a zone rather than an entity, maintaining fire at a point | area denial is distinct from aimed fire | F-7.8.4 | R-7.8.4 |
| US-7.8.4.8 | engine developer (P-26) | I want to apply a "suppressed" debuff to all entities in the suppression zone | suppression effects are gameplay-impactful | F-7.8.4 | R-7.8.4 |
| US-7.8.4.9 | engine developer (P-26) | I want mobile to disable coordinated suppression (which requires flanking), while keeping the suppressed debuff and camera effects | suppression feel is preserved without coordination cost | F-7.8.4 | R-7.8.4 |
| US-7.8.4.10 | engine tester (P-27) | I want to verify that entities within the suppression zone receive the suppressed debuff with correct accuracy penalty | zone effects work | F-7.8.4 | R-7.8.4 |
| US-7.8.4.11 | engine tester (P-27) | I want to verify that suppressive fire ceases when the AI exhausts its ammo | resource management is enforced | F-7.8.4 | R-7.8.4 |
| US-7.8.4.12 | engine tester (P-27) | I want to verify that entities outside the suppression zone do not receive the suppressed debuff | zone boundaries are accurate. --- | F-7.8.4 | R-7.8.4 |

## F-7.8.5 — Search and Investigation Patterns

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.8.5.1 | designer (P-5) | I want to configure search pattern types (spiral from LKP, check known hiding spots, room-by-room sweep) | search behavior matches the level design | F-7.8.5 | R-7.8.5 |
| US-7.8.5.2 | designer (P-5) | I want to set a search timeout after which AI returns to patrol, with intensity decreasing over time | searches feel natural and finite | F-7.8.5 | R-7.8.5 |
| US-7.8.5.3 | designer (P-5) | I want search patterns to be authored as behavior tree subtrees with configurable thoroughness | I can create and reuse search behaviors | F-7.8.5 | R-7.8.5 |
| US-7.8.5.4 | player (P-23) | I want enemies to systematically search the area after I break contact, checking cover points, rooms, and corners | escape feels tense | F-7.8.5 | R-7.8.5 |
| US-7.8.5.5 | player (P-23) | I want squad members to divide the search area among themselves, each checking different rooms | coordinated search feels efficient | F-7.8.5 | R-7.8.5 |
| US-7.8.5.6 | player (P-23) | I want enemies to give up searching after a configurable timeout and return to patrol | hiding long enough is a valid escape strategy | F-7.8.5 | R-7.8.5 |
| US-7.8.5.7 | engine developer (P-26) | I want to implement search patterns (spiral, hiding spot check, room sweep) as configurable behaviors driven by spatial queries | search is systematic | F-7.8.5 | R-7.8.5 |
| US-7.8.5.8 | engine developer (P-26) | I want to coordinate squad members to divide the search area and avoid redundant searches | team search is efficient | F-7.8.5 | R-7.8.5 |
| US-7.8.5.9 | engine developer (P-26) | I want mobile to use simplified search (move to LKP, scan once, return to patrol), disabling coordinated squad search | search cost fits the mobile budget | F-7.8.5 | R-7.8.5 |
| US-7.8.5.10 | engine tester (P-27) | I want to verify that a thorough search pattern checks all known hiding spots in the search area | no cover points are missed | F-7.8.5 | R-7.8.5 |
| US-7.8.5.11 | engine tester (P-27) | I want to verify that the search ends after the configured timeout and the agent transitions to patrol state | timeout works | F-7.8.5 | R-7.8.5 |
| US-7.8.5.12 | engine tester (P-27) | I want to verify that re-acquiring the target during search immediately transitions the agent to combat state | search-to-combat transition is instant. --- | F-7.8.5 | R-7.8.5 |

## F-7.8.6 — Retreat and Fallback Behavior

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.8.6.1 | designer (P-5) | I want to configure retreat triggers (health below threshold, cover destroyed, outnumbered, squad leader order) | retreat behavior matches the combat scenario | F-7.8.6 | R-7.8.6 |
| US-7.8.6.2 | designer (P-5) | I want to set a casualty threshold that triggers squad-wide retreat | entire squads fall back when casualties are too high | F-7.8.6 | R-7.8.6 |
| US-7.8.6.3 | designer (P-5) | I want to configure which abilities retreating AI uses to cover withdrawal (smoke grenades, suppressive fire) | retreat is tactically varied | F-7.8.6 | R-7.8.6 |
| US-7.8.6.4 | player (P-23) | I want enemies to retreat to secondary cover positions when their health drops low | AI shows self-preservation | F-7.8.6 | R-7.8.6 |
| US-7.8.6.5 | player (P-23) | I want an entire squad to fall back when casualties exceed a threshold | morale-based retreat makes combat feel dynamic | F-7.8.6 | R-7.8.6 |
| US-7.8.6.6 | player (P-23) | I want retreating enemies to throw smoke grenades to cover their withdrawal | retreat behavior feels tactical and deliberate | F-7.8.6 | R-7.8.6 |
| US-7.8.6.7 | engine developer (P-26) | I want to select retreat destinations using the cover evaluation system, finding positions further from threats | retreat targets are tactically sound | F-7.8.6 | R-7.8.6 |
| US-7.8.6.8 | engine developer (P-26) | I want retreating agents to use smoke grenades or suppressive fire to cover withdrawal when available in their ability set | retreat is not a simple flee | F-7.8.6 | R-7.8.6 |
| US-7.8.6.9 | engine developer (P-26) | I want mobile to use simplified retreat (flee directly away from threat) without cover re-evaluation or squad coordination | retreat cost is minimal | F-7.8.6 | R-7.8.6 |
| US-7.8.6.10 | engine tester (P-27) | I want to verify that individual retreat triggers at the configured health threshold and squad retreat triggers at the configured casualty count | thresholds are correct | F-7.8.6 | R-7.8.6 |
| US-7.8.6.11 | engine tester (P-27) | I want to verify that retreat destinations are further from the threat source than the current position | retreat moves away from danger | F-7.8.6 | R-7.8.6 |
| US-7.8.6.12 | engine tester (P-27) | I want to verify that AI at a fallback position reassesses and re-engages if conditions improve, or continues retreating if outmatched | post-retreat behavior adapts correctly | F-7.8.6 | R-7.8.6 |
