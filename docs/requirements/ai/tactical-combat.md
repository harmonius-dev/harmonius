# R-7.8 -- Tactical Combat AI Requirements

## R-7.8.1 Cover Evaluation and Scoring

The engine **SHALL** evaluate cover positions by protection angle, sight lines to targets,
flanking exposure, distance to target, and proximity to objectives, selecting the highest-
scoring position given the current tactical situation, with re-evaluation triggered by
significant target movement, unexpected damage, or cover destruction, and scoring weights
configurable per AI archetype.

- **Derived from:** [F-7.8.1](../../features/ai/tactical-combat.md)
- **Rationale:** Believable combat AI requires agents to select tactically sound positions
  based on multiple criteria rather than simply hiding behind the nearest wall.
- **Verification:** Place 5 cover positions with varying protection angles (30-180 degrees)
  and distances to a target. Verify the system ranks full-cover positions higher than partial-
  cover positions. Move the target 10 m and verify re-evaluation triggers and the agent
  selects a new optimal position. Destroy the agent's current cover and verify it transitions
  to the next-best position within 2 ticks.

## R-7.8.2 Flanking and Pincer Behavior

The engine **SHALL** coordinate multiple AI agents engaging the same target to approach from
different angles, with a squad leader designating flanking assignments, flanking paths avoiding
the target's line-of-sight, and flankers synchronizing at staging positions before simultaneous
attack.

- **Derived from:** [F-7.8.2](../../features/ai/tactical-combat.md)
- **Rationale:** Coordinated multi-angle attacks create challenging and believable tactical
  encounters that test player positioning and awareness.
- **Verification:** Assign a 4-agent squad against a single target. Verify at least 2 agents
  are assigned flanking positions with approach angles differing by at least 60 degrees from
  the frontal agents. Verify flanking paths do not cross the target's vision cone. Verify
  flankers wait at staging positions until all flankers arrive (within a 3-second timeout)
  before attacking simultaneously.

## R-7.8.3 Squad Formation and Communication

The engine **SHALL** maintain squad formations (line, wedge, column, diamond, staggered) with
configurable spacing during movement, with the squad leader selecting formation based on terrain
context, and squad members communicating target positions, flanking calls, retreat orders, and
regroup commands that are both functional (affect AI decisions) and presentational (trigger bark
audio and floating text). Formation movement geometry delegates to the steering-domain formation
system (R-7.2.5). This requirement adds tactical context selection and squad communication on top
of the base formation movement.

- **Derived from:** [F-7.8.3](../../features/ai/tactical-combat.md)
- **Rationale:** Formation-based movement and inter-agent communication produce organized
  military behavior that is both tactically effective and readable by the player.
- **Verification:** Move a 4-agent squad through open terrain and verify wedge formation with
  spacing within 0.5 m of the configured value. Enter a narrow corridor and verify automatic
  switch to column formation. Issue a "contact front-left" communication and verify all squad
  members orient toward the indicated direction. Verify the communication triggers a bark
  audio event. Kill 2 squad members and verify the remaining agents regroup at a rally point.

## R-7.8.4 Suppressive Fire and Pinning

The engine **SHALL** enable AI agents to fire at a zone to suppress targets, applying a
"suppressed" debuff to entities in the fire zone that increases accuracy penalty, triggers
suppression camera effects, and inhibits return fire, with suppression duration and ammo
consumption configurable per weapon type.

- **Derived from:** [F-7.8.4](../../features/ai/tactical-combat.md)
- **Rationale:** Suppressive fire adds tactical depth by allowing AI to pin down targets while
  teammates maneuver, creating dynamic combat encounters.
- **Verification:** Designate a suppression zone and fire for 5 seconds. Verify entities
  within the zone receive the "suppressed" debuff. Verify the debuff increases accuracy
  penalty by the configured amount. Verify the debuff is removed within 2 seconds after
  suppressive fire ceases. Configure two weapon types with different ammo-per-second rates
  and verify each consumes ammo at its configured rate during suppression.

## R-7.8.5 Search and Investigation Patterns

The engine **SHALL** execute systematic search patterns when AI loses visual contact with a
target, expanding from the last-known position via spiral, priority-ordered cover checks, and
room-by-room scanning, with squad members dividing the search area, and search intensity
decreasing over a configurable timeout before returning to patrol state.

- **Derived from:** [F-7.8.5](../../features/ai/tactical-combat.md)
- **Rationale:** Believable AI must actively search for lost targets rather than instantly
  forgetting or omnisciently re-acquiring them.
- **Verification:** Break line-of-sight with an agent. Verify the agent begins searching from
  the target's last-known position. Verify the search pattern visits at least 80% of nearby
  cover points within the configured search duration. Assign a 3-agent squad to search and
  verify they divide the area (no two agents search the same sub-region simultaneously).
  Wait for the timeout and verify all agents return to patrol state.

## R-7.8.6 Retreat and Fallback Behavior

The engine **SHALL** trigger retreat to secondary positions when health drops below a threshold,
current cover is destroyed, the agent is outnumbered beyond a configurable ratio, or the squad
leader orders retreat, with retreating agents selecting fallback positions further from threats
and using available abilities (smoke, suppressive fire) to cover their withdrawal.

- **Derived from:** [F-7.8.6](../../features/ai/tactical-combat.md)
- **Rationale:** AI that fights to the death regardless of circumstances is neither believable
  nor tactically interesting; retreat adds dynamic flow to combat encounters.
- **Verification:** Reduce an agent's health below the retreat threshold (e.g., 20%). Verify
  the agent begins retreating within 2 ticks. Verify the selected fallback position is farther
  from the threat than the current position. If the agent has a smoke grenade ability, verify
  it is used during withdrawal. Set a squad casualty threshold of 50% and kill half the squad;
  verify all surviving members retreat. After reaching fallback, verify the agent re-engages
  if conditions improve (e.g., health restored above threshold).
