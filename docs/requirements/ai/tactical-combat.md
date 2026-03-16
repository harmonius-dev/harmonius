# R-7.8 -- Tactical Combat AI Requirements

## R-7.8.1 Cover Evaluation and Scoring

The engine **SHALL** evaluate cover positions by protection angle, sight lines to targets, flanking
exposure, distance, and proximity to objectives, with scoring weights configurable per AI archetype,
and re-evaluate cover when the target moves significantly, the AI takes unexpected damage, or the
current cover is destroyed.

- **Derived from:**
  [F-7.8.1](../../features/ai/tactical-combat.md)
- **Rationale:** Tactical combat requires AI to select cover positions that balance offense and
  defense; per-archetype weights let cautious AI prioritize protection while aggressive AI
  prioritizes sight lines.
- **Verification:** Place full-cover and partial-cover positions equidistant from a target. Verify
  the AI selects full cover with default weights. Configure aggressive weights favoring sight lines
  and verify the AI selects the position with better target visibility. Destroy the AI's cover and
  verify re-evaluation triggers within 1 tick.

### R-7.8.2 Flanking and Pincer Behavior

The engine **SHALL** coordinate multiple AI agents engaging the same target to approach from
different angles separated by at least 60 degrees, with flanking paths avoiding the target's line of
sight and flankers synchronizing at staging positions before simultaneous attack.

- **Derived from:**
  [F-7.8.2](../../features/ai/tactical-combat.md)
- **Rationale:** Flanking creates challenging tactical encounters that require the player to
  maintain positional awareness; synchronized attacks prevent piecemeal engagement.
- **Verification:** Assign 3 AI agents to flank a single target. Verify their approach angles differ
  by at least 60 degrees. Verify flanking paths do not cross the target's line of sight. Verify
  flankers wait at staging positions until all are ready before attacking simultaneously.

### R-7.8.3 Squad Formation and Communication

The engine **SHALL** maintain squad formations (line, wedge, column, diamond) with configurable
spacing, select formations based on terrain context (wedge in open, column in corridors), and
support squad communication events (contact calls, flanking calls, retreat orders) that affect AI
decision-making and trigger bark audio.

- **Derived from:**
  [F-7.8.3](../../features/ai/tactical-combat.md)
- **Rationale:** Squad formations and communication make combat feel organized and tactically
  sophisticated, while terrain-adaptive selection prevents formations from breaking in constrained
  geometry.
- **Verification:** Move a squad in wedge formation through open terrain and verify spacing matches
  configuration. Move the squad into a narrow corridor and verify it switches to column formation
  automatically. Trigger a contact call and verify affected squad members update their behavior
  state.

### R-7.8.4 Suppressive Fire and Pinning

The engine **SHALL** enable AI to suppress a zone (rather than an entity) with sustained fire,
applying a "suppressed" debuff to entities in the fire zone that includes an accuracy penalty and
camera effects, with suppression duration and ammo consumption configurable per weapon type and the
debuff removed within 2 seconds after fire ceases.

- **Derived from:**
  [F-7.8.4](../../features/ai/tactical-combat.md)
- **Rationale:** Suppressive fire creates tactical depth by pinning targets behind cover while other
  agents maneuver; zone-based targeting is more realistic than entity-locked suppression.
- **Verification:** Have an AI suppress a zone containing the player. Verify the suppressed debuff
  is applied with accuracy penalty. Verify the debuff is removed within 2 seconds after fire ceases.
  Verify ammo consumption matches the configured rate for the weapon type.

### R-7.8.5 Search and Investigation Patterns

The engine **SHALL** provide systematic search patterns from the last-known target position, with
squad members dividing the search area, search intensity decreasing over a configurable timeout, and
search visiting at least 80% of nearby cover points before returning to patrol state.

- **Derived from:**
  [F-7.8.5](../../features/ai/tactical-combat.md)
- **Rationale:** Methodical search creates tension during stealth gameplay and gives players
  meaningful evasion challenges; coordinated search prevents redundant area coverage.
- **Verification:** Trigger a search from a last-known position. Verify the searching AI visits at
  least 80% of cover points within 20 m. Assign 3 squad members to search and verify they divide the
  area without overlap. Verify search intensity decreases and the AI returns to patrol after the
  configured timeout.

### R-7.8.6 Retreat and Fallback Behavior

The engine **SHALL** trigger AI retreat when health drops below a configurable threshold, current
cover is destroyed, or the AI is outnumbered beyond a configurable ratio, with retreating agents
selecting fallback cover positions farther from threats and morale-based squad retreat when
casualties exceed a threshold. Retreat **SHALL** trigger within 2 ticks of the health threshold
being crossed.

- **Derived from:**
  [F-7.8.6](../../features/ai/tactical-combat.md)
- **Rationale:** Self-preservation behavior creates dynamic combat flow where AI adapts to losing
  situations; morale- based squad retreat produces realistic group behavior.
- **Verification:** Reduce an AI's health below the retreat threshold and verify retreat triggers
  within 2 ticks. Verify the AI selects a cover position farther from the threat. Kill 3 of 5 squad
  members and verify the remaining 2 execute morale-based squad retreat. Verify retreated AI
  re-engages when conditions improve.

---

## User Story Traceability

User stories for this domain are maintained in
[user-stories/ai/tactical-combat.md](../../user-stories/ai/tactical-combat.md).
Requirements in this document are derived from those
user stories.
