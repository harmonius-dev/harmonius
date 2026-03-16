# User Stories — 7.8 Tactical Combat AI

## F-7.8.1 — Cover Evaluation and Scoring

## US-7.8.1.1 Configure Cover Scoring Weights Per Archetype

**As a** designer (P-5), **I want to** configure cover scoring weights (protection angle, sight
lines, flanking exposure, distance, objective proximity) per AI archetype, **so that** cautious AI
prioritizes protection while aggressive AI prioritizes sight lines.

## US-7.8.1.2 Preview Cover Positions in Editor

**As a** designer (P-5), **I want to** see pre-computed cover positions with quality scores as debug
overlays in the editor, **so that** I can verify cover placement in my level.

## US-7.8.1.3 Set Cover Re-Evaluation Triggers

**As a** designer (P-5), **I want to** configure when AI re-evaluates cover (target moves, damage
from unexpected direction, cover destroyed), **so that** cover decisions stay current.

## US-7.8.1.4 See Enemies Take Cover During Combat

**As a** player (P-23), **I want** enemies to move to cover positions during combat, hiding behind
walls and objects, **so that** combat AI feels tactically aware.

## US-7.8.1.5 Watch AI Switch Cover When Flanked

**As a** player (P-23), **I want** enemies to abandon a cover position and find a better one when I
flank them, **so that** AI adapts to my tactical positioning.

## US-7.8.1.6 See AI Choose Cover Near Objectives

**As a** player (P-23), **I want** enemies defending an objective to choose cover near that
objective rather than running to distant safety, **so that** defenders hold their ground.

## US-7.8.1.7 Implement Multi-Factor Cover Scoring System

**As an** engine developer (P-26), **I want to** implement a cover scoring system evaluating
protection angle, sight lines, flanking exposure, target distance, and objective proximity,
**so that** cover selection is context-aware.

## US-7.8.1.8 Pre-Compute Cover Positions from World Geometry

**As an** engine developer (P-26), **I want to** pre-compute cover positions from world geometry and
cache them in the shared spatial index, **so that** runtime queries are fast.

## US-7.8.1.9 Evaluate Fewer Cover Candidates on Mobile

**As an** engine developer (P-26), **I want to** evaluate 4 cover candidates per query on mobile
(vs. 16 on desktop), **so that** spatial query cost fits the mobile budget.

## US-7.8.1.10 Verify Cover Selection Matches Archetype Weights

**As an** engine tester (P-27), **I want to** verify that cautious AI selects high-protection cover
and aggressive AI selects cover with good sight lines, **so that** archetype weights work correctly.

## US-7.8.1.11 Test Cover Re-Evaluation on Flank

**As an** engine tester (P-27), **I want to** verify that AI re-evaluates cover when damaged from an
unexpected direction and moves to a better position, **so that** re-evaluation triggers work.

## US-7.8.1.12 Benchmark Cover Query Performance

**As an** engine tester (P-27), **I want to** benchmark cover evaluation query time for the maximum
candidate count, **so that** cover selection fits within the per-tick budget.

---

## F-7.8.2 — Flanking and Pincer Behavior

## US-7.8.2.1 Configure Flanking Group Size

**As a** designer (P-5), **I want to** configure the minimum squad size that triggers flanking
behavior, **so that** solo enemies attack directly while groups coordinate.

## US-7.8.2.2 Set Flanking Path Occlusion Requirements

**As a** designer (P-5), **I want to** set how aggressively flanking paths avoid the target's line
of sight, **so that** flanking stealth matches the tactical scenario.

## US-7.8.2.3 Configure Synchronization Wait Time

**As a** designer (P-5), **I want to** configure how long flankers wait at staging positions before
attacking simultaneously, **so that** coordination timing is tunable.

## US-7.8.2.4 See Enemies Approach from Multiple Angles

**As a** player (P-23), **I want** enemy squads to approach me from multiple angles, with some
pinning me down while others flank, **so that** combat feels tactically challenging.

## US-7.8.2.5 Watch Flankers Wait Then Attack Simultaneously

**As a** player (P-23), **I want** flanking enemies to wait at staging positions until all are
ready, then attack at once, **so that** coordinated assault feels planned.

## US-7.8.2.6 See Solo Enemies Not Attempt Flanking

**As a** player (P-23), **I want** a lone enemy to attack directly without attempting to flank,
**so that** flanking requires realistic squad coordination.

## US-7.8.2.7 Implement Squad Leader Flanking Assignment

**As an** engine developer (P-26), **I want to** implement squad leader designation of flanking
assignments (frontal pressure, flank left, flank right), **so that** coordinated movement emerges
from role assignment.

## US-7.8.2.8 Route Flanking Paths to Avoid Target LOS

**As an** engine developer (P-26), **I want to** route flanking paths using geometry occlusion
checks to avoid the target's line of sight, **so that** flankers approach undetected.

## US-7.8.2.9 Disable Coordinated Flanking on Mobile

**As an** engine developer (P-26), **I want** mobile to disable coordinated flanking and have agents
attack independently, **so that** squad coordination cost is eliminated on mobile.

## US-7.8.2.10 Verify Flankers Approach from Correct Angles

**As an** engine tester (P-27), **I want to** verify that flankers approach from the target's flank
or rear (not front), **so that** flanking geometry is correct.

## US-7.8.2.11 Test Synchronization at Staging Positions

**As an** engine tester (P-27), **I want to** verify that flankers wait at staging positions until
all are ready before attacking, **so that** synchronization works.

## US-7.8.2.12 Validate Mobile Disables Flanking

**As an** engine tester (P-27), **I want to** confirm that mobile builds never attempt coordinated
flanking and agents attack independently, **so that** the platform gate works.

---

## F-7.8.3 — Squad Formation and Communication

## US-7.8.3.1 Configure Formation Shapes Per Context

**As a** designer (P-5), **I want to** configure which formation shape a squad uses per context
(wedge for open terrain, column for corridors, line for sweeping), **so that** formation adapts to
the environment.

## US-7.8.3.2 Set Squad Communication Barks

**As a** designer (P-5), **I want to** configure squad communication barks ("contact front-left",
"flanking right", "retreat"), **so that** squad chatter is audible and informative to the player.

## US-7.8.3.3 Configure Rally Points for Regrouping

**As a** designer (P-5), **I want to** place rally points in the level where squads regroup after
combat, **so that** post-combat behavior is designer-controlled.

## US-7.8.3.4 See Squads Move in Formation Through the Level

**As a** player (P-23), **I want** enemy squads to move in formation (wedge, column, line) through
the level, **so that** military AI looks organized and professional.

## US-7.8.3.5 Hear Squad Members Call Out Targets

**As a** player (P-23), **I want** to hear squad members call out target positions ("contact
front-left") and flanking moves, **so that** communication adds atmosphere and information.

## US-7.8.3.6 Watch Squad Regroup at Rally Point After Combat

**As a** player (P-23), **I want** scattered squad members to regroup at a rally point after combat,
with stragglers rejoining the formation, **so that** post-combat recovery looks realistic.

## US-7.8.3.7 Implement Context-Adaptive Formation Selection

**As an** engine developer (P-26), **I want to** implement formation selection based on
environmental context (open terrain, corridor, sweep area), **so that** squads automatically adapt
their shape.

## US-7.8.3.8 Implement Squad Communication System

**As an** engine developer (P-26), **I want to** implement a squad communication system that sends
functional messages (target position, flanking call, retreat order) and plays bark audio,
**so that** communication is both functional and presentational.

## US-7.8.3.9 Limit Squad Size on Mobile

**As an** engine developer (P-26), **I want** mobile to limit squad size to 4 (vs. 8 on desktop) and
use column formation only, **so that** formation cost is reduced.

## US-7.8.3.10 Verify Formation Shape Matches Context

**As an** engine tester (P-27), **I want to** verify that the squad selects the correct formation
shape for each environmental context, **so that** context-adaptive selection works.

## US-7.8.3.11 Test Squad Regrouping at Rally Points

**As an** engine tester (P-27), **I want to** verify that all surviving squad members reach the
rally point after combat and reform their formation, **so that** regrouping works.

## US-7.8.3.12 Validate Communication Messages Affect AI Decisions

**As an** engine tester (P-27), **I want to** verify that squad communication messages (target call,
retreat order) actually affect receiving agents' behavior tree decisions, **so that** communication
is functional.

---

## F-7.8.4 — Suppressive Fire and Pinning

## US-7.8.4.1 Configure Suppression Zone Parameters

**As a** designer (P-5), **I want to** configure suppression zone radius, duration, and accuracy
penalty per weapon type, **so that** suppression intensity matches the weapon.

## US-7.8.4.2 Set Suppression Debuff Effects

**As a** designer (P-5), **I want to** configure the "suppressed" debuff effects (accuracy penalty,
camera shake, reduced return fire), **so that** being suppressed feels impactful.

## US-7.8.4.3 Configure AI Suppression Tactics

**As a** designer (P-5), **I want to** configure when AI uses suppressive fire (while flankers move,
when target enters cover, on retreat), **so that** suppression is used tactically.

## US-7.8.4.4 Feel Pinned Down by Suppressive Fire

**As a** player (P-23), **I want** to feel pinned down when enemies lay suppressive fire on my
position, with camera effects and accuracy penalties, **so that** suppression creates tactical
pressure.

## US-7.8.4.5 See AI Suppress While Flanker Moves

**As a** player (P-23), **I want** one enemy to suppress my cover while another flanks, **so that**
combined tactics force me to reposition.

## US-7.8.4.6 Watch Suppression End When Ammo Runs Low

**As a** player (P-23), **I want** suppressive fire to stop when the suppressor runs low on ammo,
**so that** suppression has a resource cost and a natural end.

## US-7.8.4.7 Implement Zone-Based Suppression System

**As an** engine developer (P-26), **I want to** implement suppressive fire targeting a zone rather
than an entity, maintaining fire at a point, **so that** area denial is distinct from aimed fire.

## US-7.8.4.8 Apply Suppressed Debuff to Entities in Zone

**As an** engine developer (P-26), **I want to** apply a "suppressed" debuff to all entities in the
suppression zone, **so that** suppression effects are gameplay-impactful.

## US-7.8.4.9 Disable Coordinated Suppression on Mobile

**As an** engine developer (P-26), **I want** mobile to disable coordinated suppression (which
requires flanking), while keeping the suppressed debuff and camera effects, **so that** suppression
feel is preserved without coordination cost.

## US-7.8.4.10 Verify Suppression Zone Applies Debuff

**As an** engine tester (P-27), **I want to** verify that entities within the suppression zone
receive the suppressed debuff with correct accuracy penalty, **so that** zone effects work.

## US-7.8.4.11 Test Suppression Ends on Ammo Depletion

**As an** engine tester (P-27), **I want to** verify that suppressive fire ceases when the AI
exhausts its ammo, **so that** resource management is enforced.

## US-7.8.4.12 Validate Entities Outside Zone Are Unaffected

**As an** engine tester (P-27), **I want to** verify that entities outside the suppression zone do
not receive the suppressed debuff, **so that** zone boundaries are accurate.

---

## F-7.8.5 — Search and Investigation Patterns

## US-7.8.5.1 Configure Search Pattern Types

**As a** designer (P-5), **I want to** configure search pattern types (spiral from LKP, check known
hiding spots, room-by-room sweep), **so that** search behavior matches the level design.

## US-7.8.5.2 Set Search Timeout and Intensity Decay

**As a** designer (P-5), **I want to** set a search timeout after which AI returns to patrol, with
intensity decreasing over time, **so that** searches feel natural and finite.

## US-7.8.5.3 Author Search Patterns as BT Subtrees

**As a** designer (P-5), **I want** search patterns to be authored as behavior tree subtrees with
configurable thoroughness, **so that** I can create and reuse search behaviors.

## US-7.8.5.4 Watch Enemies Systematically Search After I Escape

**As a** player (P-23), **I want** enemies to systematically search the area after I break contact,
checking cover points, rooms, and corners, **so that** escape feels tense.

## US-7.8.5.5 See Squad Divide Search Area

**As a** player (P-23), **I want** squad members to divide the search area among themselves, each
checking different rooms, **so that** coordinated search feels efficient.

## US-7.8.5.6 Watch AI Give Up and Return to Patrol

**As a** player (P-23), **I want** enemies to give up searching after a configurable timeout and
return to patrol, **so that** hiding long enough is a valid escape strategy.

## US-7.8.5.7 Implement Search Pattern System

**As an** engine developer (P-26), **I want to** implement search patterns (spiral, hiding spot
check, room sweep) as configurable behaviors driven by spatial queries, **so that** search is
systematic.

## US-7.8.5.8 Coordinate Squad Search Division

**As an** engine developer (P-26), **I want to** coordinate squad members to divide the search area
and avoid redundant searches, **so that** team search is efficient.

## US-7.8.5.9 Simplify Search on Mobile

**As an** engine developer (P-26), **I want** mobile to use simplified search (move to LKP, scan
once, return to patrol), disabling coordinated squad search, **so that** search cost fits the mobile
budget.

## US-7.8.5.10 Verify Search Covers All Hiding Spots

**As an** engine tester (P-27), **I want to** verify that a thorough search pattern checks all known
hiding spots in the search area, **so that** no cover points are missed.

## US-7.8.5.11 Test Search Timeout Returns to Patrol

**As an** engine tester (P-27), **I want to** verify that the search ends after the configured
timeout and the agent transitions to patrol state, **so that** timeout works.

## US-7.8.5.12 Validate Re-Acquisition Triggers Combat

**As an** engine tester (P-27), **I want to** verify that re-acquiring the target during search
immediately transitions the agent to combat state, **so that** search-to-combat transition is
instant.

---

## F-7.8.6 — Retreat and Fallback Behavior

## US-7.8.6.1 Configure Retreat Trigger Conditions

**As a** designer (P-5), **I want to** configure retreat triggers (health below threshold, cover
destroyed, outnumbered, squad leader order), **so that** retreat behavior matches the combat
scenario.

## US-7.8.6.2 Set Morale-Based Squad Retreat Threshold

**As a** designer (P-5), **I want to** set a casualty threshold that triggers squad-wide retreat,
**so that** entire squads fall back when casualties are too high.

## US-7.8.6.3 Configure Retreat Cover-Up Abilities

**As a** designer (P-5), **I want to** configure which abilities retreating AI uses to cover
withdrawal (smoke grenades, suppressive fire), **so that** retreat is tactically varied.

## US-7.8.6.4 See Enemies Retreat When Badly Hurt

**As a** player (P-23), **I want** enemies to retreat to secondary cover positions when their health
drops low, **so that** AI shows self-preservation.

## US-7.8.6.5 Watch Squad Fall Back After Heavy Casualties

**As a** player (P-23), **I want** an entire squad to fall back when casualties exceed a threshold,
**so that** morale-based retreat makes combat feel dynamic.

## US-7.8.6.6 See Retreating AI Pop Smoke to Cover Withdrawal

**As a** player (P-23), **I want** retreating enemies to throw smoke grenades to cover their
withdrawal, **so that** retreat behavior feels tactical and deliberate.

## US-7.8.6.7 Implement Retreat Destination Selection

**As an** engine developer (P-26), **I want to** select retreat destinations using the cover
evaluation system, finding positions further from threats, **so that** retreat targets are
tactically sound.

## US-7.8.6.8 Implement Retreat with Cover-Up Actions

**As an** engine developer (P-26), **I want** retreating agents to use smoke grenades or suppressive
fire to cover withdrawal when available in their ability set, **so that** retreat is not a simple
flee.

## US-7.8.6.9 Simplify Retreat on Mobile

**As an** engine developer (P-26), **I want** mobile to use simplified retreat (flee directly away
from threat) without cover re-evaluation or squad coordination, **so that** retreat cost is minimal.

## US-7.8.6.10 Verify Retreat Triggers at Correct Thresholds

**As an** engine tester (P-27), **I want to** verify that individual retreat triggers at the
configured health threshold and squad retreat triggers at the configured casualty count, **so that**
thresholds are correct.

## US-7.8.6.11 Test Retreat Destination Is Further from Threat

**As an** engine tester (P-27), **I want to** verify that retreat destinations are further from the
threat source than the current position, **so that** retreat moves away from danger.

## US-7.8.6.12 Validate Post-Retreat Re-Engagement

**As an** engine tester (P-27), **I want to** verify that AI at a fallback position reassesses and
re-engages if conditions improve, or continues retreating if outmatched, **so that** post-retreat
behavior adapts correctly.
