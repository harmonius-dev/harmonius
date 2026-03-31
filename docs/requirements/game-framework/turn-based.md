# R-13.21 -- Turn-Based and Tactical Systems Requirements

## Tactical Grid

1. **R-13.21.1** -- The engine **SHALL** provide square or hexagonal grid overlays with per-cell
   traversability, elevation, cover values, terrain type, and occupancy, generated from collision
   geometry or hand-authored, with multi-floor support via stair connections.
   - **Rationale:** A tactical grid is the engine primitive for any turn-based strategy or tactics
     game.
   - **Verification:** Generate a grid from collision geometry and verify blocked cells prevent
     movement. Verify multi-floor grids connect via stairs.

## Turn Queue

1. **R-13.21.2** -- The engine **SHALL** provide a turn manager supporting round-robin,
   initiative-based, team-based, and phase-based turn modes, with a unified interface for AI and
   player units.
   - **Rationale:** A turn queue is the engine primitive for any turn- based gameplay system.
   - **Verification:** Configure initiative mode and verify units sort by speed stat each round.
     Verify AI turns execute behavior trees without player input.

## Action Point Pool

1. **R-13.21.3** -- The engine **SHALL** provide a per-unit action point budget spent on movement
   and abilities each turn, with terrain-based movement costs, ability AP costs, and range/path
   preview display.
   - **Rationale:** An action point pool is the engine primitive for any resource-limited turn-based
     action system.
   - **Verification:** Move through difficult terrain and verify higher AP cost. Verify AP resets at
     the start of each unit's turn.

## Grid Cover and Overwatch

1. **R-13.21.4** -- The engine **SHALL** provide directional cover with half and full values per
   edge, flanking negation of cover bonuses, and an overwatch stance that fires at the first enemy
   moving within line of sight on the enemy turn.
   - **Rationale:** Directional cover and overwatch are engine primitives for tactical positioning
     and area denial.
   - **Verification:** Flank an enemy in cover and verify their defense bonus is negated. Enter
     overwatch and verify the unit fires at the first moving enemy.

## Hit Probability Resolution

1. **R-13.21.5** -- The engine **SHALL** compute and display hit probability from weapon accuracy,
   range, cover, elevation, flanking, buffs, and unit stats, with weighted random combat resolution
   distinguishing miss, graze, hit, and critical outcomes.
   - **Rationale:** Transparent hit probability enables informed tactical decisions with meaningful
     variance.
   - **Verification:** Verify full cover adds the configured defense bonus. Verify elevation
     advantage grants the configured accuracy bonus. Verify critical hits trigger at the configured
     crit chance.
