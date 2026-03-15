# 7.8 — Tactical Combat AI

### F-7.8.1 Cover Evaluation and Scoring

AI agents evaluate cover positions by quality: protection angle (how much of the threat arc is blocked),
sight lines to targets, flanking exposure (how vulnerable the position is from the sides/rear), distance to
current target, and proximity to objectives. Cover positions are pre-computed from world geometry and cached in
the shared spatial index (F-1.9.7). AI selects the highest-scoring cover position given the current tactical
situation. Cover re-evaluation triggers when: the target moves significantly, the AI takes damage from an
unexpected direction, or the current cover is destroyed. Cover scoring weights are configurable per AI archetype
(cautious AI prioritizes protection, aggressive AI prioritizes sight lines).

- **Requirements:** R-7.8.1
- **Dependencies:** F-7.3.1 (Behavior Trees), F-1.9.4 (Spatial Query), F-13.18.5 (Cover System)
- **Platform notes:** Mobile evaluates fewer cover candidates (4 vs. 16 on desktop) per
  query and re-evaluates less frequently to bound spatial query cost.

### F-7.8.2 Flanking and Pincer Behavior

AI agents coordinate to approach targets from multiple angles. When multiple AI agents engage the same target,
the squad leader (highest-priority agent) designates flanking assignments: one or more agents move to positions
on the target's flank or rear while others maintain frontal pressure. Flanking paths avoid the target's
line-of-sight using the spatial index for geometry occlusion checks. Agents synchronize their flanking
approach — flankers wait at staging positions until all flankers are in position, then attack simultaneously.
Flanking bonuses (increased damage, bypassed cover) incentivize the behavior in the combat system.

- **Requirements:** R-7.8.2
- **Dependencies:** F-7.8.1, F-7.2.1 (Steering Behaviors), F-7.1.1 (NavMesh)
- **Platform notes:** Mobile disables coordinated flanking; AI agents attack independently.
  Switch supports simplified flanking with 2-agent coordination max.

### F-7.8.3 Squad Formation and Communication

AI squads maintain formations during movement (line, wedge, column, diamond, staggered) with configurable
spacing. The squad leader determines the formation based on context: wedge for open terrain assault, column for
narrow corridors, line for sweeping an area. Squad members communicate target positions ("contact front-left"),
flanking calls ("flanking right"), retreat orders, and regroup commands. Communication is both functional
(affects AI decision-making) and presentational (plays bark audio and floating text for player awareness).
Squads reform after combat with stragglers rejoining at rally points.

- **Requirements:** R-7.8.3
- **Dependencies:** F-7.2.5 (Formation Movement), F-7.3.1 (Behavior Trees), F-13.19.5 (Bark System)
- **Platform notes:** Mobile limits squad size to 4 (vs. 8 on desktop). Mobile uses column
  formation only; context-adaptive formation selection disabled.

### F-7.8.4 Suppressive Fire and Pinning

AI agents fire at a position to pin down targets without necessarily aiming to hit. Suppressive fire targets a
zone rather than an entity, maintaining fire at a point to prevent the target from leaving cover. The
suppression system applies a "suppressed" debuff to entities in the fire zone that: increases accuracy penalty,
triggers suppression camera effects (blur, shake), and inhibits the suppressed entity's ability to return fire
effectively. AI uses suppression tactically — suppressing one target while another agent flanks. Suppression
duration, accuracy, and ammo consumption are configurable per weapon type.

- **Requirements:** R-7.8.4
- **Dependencies:** F-7.8.1, F-13.16.1 (Weapon System), F-13.10.3 (Gameplay Effects)
- **Platform notes:** Mobile disables coordinated suppression (requires flanking F-7.8.2).
  Suppression debuff and camera effects apply on all platforms.

### F-7.8.5 Search and Investigation Patterns

When AI loses visual contact with a target, it systematically searches the area. Search patterns: expand from
last-known-position in a spiral, check known hiding spots (cover points, rooms, corners) in priority order,
search room-by-room (entering, scanning, moving to next), and coordinate with squad members to divide the
search area. Search intensity decreases over time — after a configurable timeout, AI returns to patrol state.
If the target is re-acquired during search, AI immediately transitions to combat. Search patterns are authored
as behavior tree subtrees with configurable parameters for thoroughness and duration.

- **Requirements:** R-7.8.5
- **Dependencies:** F-7.3.1 (Behavior Trees), F-13.18.2 (AI Alert States), F-7.6.1 (Perception)
- **Platform notes:** Mobile uses simplified search (move to last-known-position, scan once,
  return to patrol). Coordinated squad search division disabled on mobile.

### F-7.8.6 Retreat and Fallback Behavior

AI retreats to secondary positions when: health drops below a threshold, current cover is destroyed,
outnumbered beyond a configurable ratio, or ordered by squad leader. Retreat destination selection uses the
cover evaluation system (F-7.8.1) to find positions further from threats. Retreating agents use smoke grenades
or suppressive fire to cover their withdrawal (if available in their ability set). After reaching the fallback
position, AI reassesses — engaging again if conditions improve, or continuing to retreat if outmatched.
Morale-based retreat triggers cause entire squads to fall back when casualties exceed a threshold.

- **Requirements:** R-7.8.6
- **Dependencies:** F-7.8.1, F-7.3.1 (Behavior Trees)
- **Platform notes:** Mobile uses simplified retreat (flee directly away from threat) without
  cover re-evaluation. Coordinated squad withdrawal disabled on mobile.
