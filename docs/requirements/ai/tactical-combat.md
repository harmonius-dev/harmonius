# R-7.8 -- Tactical Combat AI Requirements

## Cover and Positioning

1. **R-7.8.1** -- The engine **SHALL** evaluate cover positions by protection angle, sight lines,
   flanking exposure, target distance, and objective proximity, with configurable scoring weights
   per AI archetype and re-evaluation on target movement, unexpected damage, or cover destruction.
   - **Rationale:** Context-aware cover selection produces tactically intelligent combat behavior
     that adapts to changing battlefield conditions.
   - **Verification:** Verify cautious AI selects high-protection cover. Verify aggressive AI
     selects cover with good sight lines. Damage from an unexpected direction and verify the AI
     re-evaluates and moves to better cover.

2. **R-7.8.2** -- The engine **SHALL** coordinate flanking approaches where agents move to the
   target's flank or rear while others maintain frontal pressure, with flanking paths avoiding the
   target's line of sight and synchronized staging.
   - **Rationale:** Coordinated flanking produces challenging tactical combat that rewards player
     awareness and positioning.
   - **Verification:** Verify flankers approach from the target's flank or rear. Verify flankers
     wait at staging positions until all are ready. Verify mobile builds disable coordinated
     flanking and have agents attack independently.

## Squad Coordination

1. **R-7.8.3** -- The engine **SHALL** maintain squad formations during movement with
   context-adaptive shape selection, inter-member communication of target positions and orders, and
   rally-point-based regrouping after combat.
   - **Rationale:** Squad coordination produces professional-looking military AI with functional
     communication that aids both AI decision-making and player experience.
   - **Verification:** Verify the squad selects the correct formation for each environment. Verify
     communication messages affect receiving agents' behavior. Verify surviving members regroup at
     the rally point.

2. **R-7.8.4** -- The engine **SHALL** support zone-targeted suppressive fire that applies a
   suppressed debuff to entities in the fire zone, with configurable duration, accuracy penalty, and
   ammo consumption per weapon type.
   - **Rationale:** Suppressive fire creates tactical pressure through area denial rather than
     direct damage, enabling fire-and-maneuver tactics.
   - **Verification:** Verify entities in the suppression zone receive the debuff. Verify entities
     outside the zone do not. Verify suppression ceases when ammo is exhausted.

## Search and Retreat

1. **R-7.8.5** -- The engine **SHALL** provide configurable search patterns that expand from the
   last known position, check hiding spots, and coordinate among squad members, with a timeout
   transition back to patrol state.
   - **Rationale:** Systematic search creates tension after the player breaks contact and rewards
     stealth play.
   - **Verification:** Verify search checks all known hiding spots. Verify the search ends after
     timeout and the agent returns to patrol. Verify re-acquiring the target during search
     immediately transitions to combat.

2. **R-7.8.6** -- The engine **SHALL** trigger retreat when health, cover, or numerical conditions
   deteriorate, selecting fallback positions via cover evaluation, with morale-based squad-wide
   retreat and tactical withdrawal cover (smoke, suppressive fire).
   - **Rationale:** Self-preservation behavior makes combat AI feel alive and creates dynamic battle
     flow as enemies fall back and regroup.
   - **Verification:** Verify retreat triggers at the configured health threshold. Verify retreat
     destinations are further from threats. Verify squad retreat triggers at the configured casualty
     count.
