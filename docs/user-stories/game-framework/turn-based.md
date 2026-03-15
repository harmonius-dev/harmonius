# User Stories -- Turn-Based and Tactical Systems (13.21)

## Tactical Grid (F-13.21.1)

## US-13.21.1.1
**As a** player (P-23), **I want** the tactical grid to display colored highlights showing
reachable cells (blue), enemy range (red), and ability AoE (yellow), **so that** I can plan
moves visually.

## US-13.21.1.2
**As a** player (P-23), **I want** grid cells to show cover values, elevation tiers, and
terrain type, **so that** I can assess tactical advantages before moving.

## US-13.21.1.3
**As a** designer (P-5), **I want** to choose between square and hexagonal grids per level,
**so that** I can match the grid type to the game's tactical style.

## US-13.21.1.4
**As a** designer (P-5), **I want** to generate grids from world collision geometry or
hand-author them in the editor, **so that** both procedural and designed levels work.

## US-13.21.1.5
**As a** designer (P-5), **I want** multi-floor grids with stairs connecting elevation tiers,
**so that** buildings and vertical spaces are tactically usable.

## US-13.21.1.6
**As a** tester (P-27), **I want** to verify that blocked cells prevent unit movement, **so
that** traversability flags are enforced.

## Turn Manager and Initiative (F-13.21.2)

## US-13.21.2.1
**As a** player (P-23), **I want** a turn order bar showing upcoming actors with portraits,
**so that** I know who acts next.

## US-13.21.2.2
**As a** player (P-23), **I want** turn transitions to focus the camera on the active unit,
**so that** I always know which unit is acting.

## US-13.21.2.3
**As a** designer (P-5), **I want** to choose between round-robin, initiative-based,
team-based, and phase-based turn modes, **so that** the turn system fits my game design.

## US-13.21.2.4
**As a** designer (P-5), **I want** AI and player units to use the same turn interface, **so
that** AI turns execute identically to player turns.

## US-13.21.2.5
**As a** tester (P-27), **I want** to verify that initiative-based ordering sorts units by
speed stat each round, **so that** initiative calculation is correct.

## US-13.21.2.6
**As a** tester (P-27), **I want** to verify that AI turns execute behavior trees
automatically without player input, **so that** AI turn automation works.

## Action Point Movement and Abilities (F-13.21.3)

## US-13.21.3.1
**As a** player (P-23), **I want** each unit to have an action point budget spent on movement
and abilities, **so that** I must prioritize actions each turn.

## US-13.21.3.2
**As a** player (P-23), **I want** to split AP between movement and actions in any order, **so
that** I can move, attack, then move again if AP remains.

## US-13.21.3.3
**As a** player (P-23), **I want** movement range displayed as highlighted reachable cells
with path preview and AP cost on hover, **so that** I can plan moves before committing.

## US-13.21.3.4
**As a** player (P-23), **I want** ability targeting to show AoE shapes, range limits, and hit
probability, **so that** I can evaluate attacks before acting.

## US-13.21.3.5
**As a** designer (P-5), **I want** to configure AP costs per ability and movement cost per
terrain type, **so that** tactical economy is data-driven.

## US-13.21.3.6
**As a** tester (P-27), **I want** to verify that difficult terrain costs more AP to traverse,
**so that** terrain movement costs apply correctly.

## US-13.21.3.7
**As a** tester (P-27), **I want** to verify that AP resets at the start of each unit's turn,
**so that** the turn boundary is correctly handled.

## Grid Cover and Overwatch (F-13.21.4)

## US-13.21.4.1
**As a** player (P-23), **I want** cover objects to provide half or full defense bonuses
against attacks from the covered direction, **so that** positioning behind cover reduces damage.

## US-13.21.4.2
**As a** player (P-23), **I want** flanking to negate cover bonuses and grant accuracy or
damage bonuses, **so that** outmaneuvering enemies is rewarded.

## US-13.21.4.3
**As a** player (P-23), **I want** to spend AP to enter overwatch stance and automatically
fire at the first enemy that moves within line of sight, **so that** I can control space on
the enemy turn.

## US-13.21.4.4
**As a** designer (P-5), **I want** cover values (half, full) configurable per cover object
and per edge direction, **so that** I can design diverse cover layouts.

## US-13.21.4.5
**As a** designer (P-5), **I want** overwatch shots to use a reduced accuracy modifier, **so
that** overwatch is powerful but not guaranteed.

## US-13.21.4.6
**As a** tester (P-27), **I want** to verify that a flanking attack negates the target's cover
bonus, **so that** directional cover is correctly computed.

## US-13.21.4.7
**As a** tester (P-27), **I want** to verify that overwatch fires at the first enemy entering
line of sight during their turn, **so that** overwatch triggers correctly.

## Hit Probability and Combat Resolution (F-13.21.5)

## US-13.21.5.1
**As a** player (P-23), **I want** hit probability displayed as a percentage before confirming
attacks, **so that** I can make informed attack decisions.

## US-13.21.5.2
**As a** player (P-23), **I want** damage preview showing min-max range alongside hit chance,
**so that** I can assess expected damage output.

## US-13.21.5.3
**As a** player (P-23), **I want** miss, graze, hit, and critical results distinguished with
feedback animations and floating text, **so that** each outcome feels distinct.

## US-13.21.5.4
**As a** designer (P-5), **I want** hit probability computed from weapon accuracy, range,
cover, elevation, flanking, buffs, and unit stats, **so that** tactical factors create
meaningful variance.

## US-13.21.5.5
**As a** designer (P-5), **I want** critical hit chance and bonus damage configurable per
weapon, **so that** crits are balanced across weapon types.

## US-13.21.5.6
**As a** tester (P-27), **I want** to verify that full cover adds the configured defense bonus
to hit probability calculation, **so that** cover math is correct.

## US-13.21.5.7
**As a** tester (P-27), **I want** to verify that elevation advantage grants the configured
accuracy bonus, **so that** height advantage is computed correctly.
