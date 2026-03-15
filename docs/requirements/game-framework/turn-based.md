# R-13.21 -- Turn-Based and Tactical Systems Requirements

## R-13.21.1 Tactical Grid System

The engine **SHALL** overlay a square or hexagonal grid on the game world with per-cell data for
traversability, elevation tier, directional cover values, terrain-type movement cost, and occupancy,
supporting multi-floor grids connected by stairs and rendering colored cell highlights for reachable
area, enemy range, and ability AoE during gameplay.

- **Derived from:** [F-13.21.1](../../features/game-framework/turn-based.md)
- **Rationale:** A data-rich tactical grid is the foundational spatial structure for turn-based
  combat, encoding all information needed for movement, cover, and ability targeting.
- **Verification:** Generate a hex grid on a test level with flat terrain, a wall, and two
  elevation tiers. Verify per-cell data: wall cell is blocked, stair cell connects tiers, flat
  cells have traversability true. Place a unit and verify blue highlight covers all reachable
  cells within movement range. Verify the grid renders correctly on both square and hex modes.

## R-13.21.2 Turn Manager and Initiative

The engine **SHALL** manage turn order using configurable modes (round-robin, initiative-based,
team-based, phase-based), maintain the current actor and turn count, display a turn order bar with
upcoming actor portraits, focus the camera on the active unit at turn transitions, and use the same
turn interface for both player and AI units.

- **Derived from:** [F-13.21.2](../../features/game-framework/turn-based.md)
- **Rationale:** A unified turn manager with multiple ordering modes supports diverse tactical
  game designs from XCOM-style team turns to Final Fantasy-style initiative.
- **Verification:** Configure four units with speed stats 10, 20, 15, 5 in initiative mode.
  Verify turn order is 20, 15, 10, 5. Switch to team-based mode with two teams of two; verify
  all of team A acts before team B. Verify the turn order UI displays correct portraits in
  order. Verify camera transitions to the active unit at each turn start.

## R-13.21.3 Action Point Movement and Abilities

The engine **SHALL** allocate per-unit action points (AP) each turn, deduct AP for movement
proportional to distance and terrain cost and for abilities at configurable AP costs, allow
interleaving movement and actions in any order within the AP budget, display reachable cells and
path cost on hover, and reset AP at the start of each unit's turn.

- **Derived from:** [F-13.21.3](../../features/game-framework/turn-based.md)
- **Rationale:** Action point budgets with free-form spending give players tactical flexibility
  to balance positioning and actions within each turn.
- **Verification:** Give a unit 6 AP on a grid where flat cells cost 1 AP and difficult terrain
  costs 2 AP. Move 3 flat cells (3 AP spent); verify 3 AP remain. Use a 2 AP ability; verify
  1 AP remains. Move 1 more flat cell; verify 0 AP remain and further actions are blocked.
  Start the next turn; verify AP resets to 6.

## R-13.21.4 Grid Cover and Overwatch

The engine **SHALL** apply directional half and full cover defense bonuses from grid objects, negate
cover for flanking attacks from uncovered directions, and support an overwatch stance consuming AP
that automatically fires at reduced accuracy on the first enemy moving within line-of-sight during
the enemy turn, with cover shields and overwatch cones visualized on the grid.

- **Derived from:** [F-13.21.4](../../features/game-framework/turn-based.md)
- **Rationale:** Cover and overwatch are core tactical mechanics that reward positioning and
  create meaningful decisions about movement risk during enemy turns.
- **Verification:** Place a unit behind half cover facing north. Attack from the north; verify
  the half cover defense bonus applies. Attack from the east (flanking); verify cover is negated.
  Place a unit in overwatch stance. Move an enemy through its line-of-sight; verify the overwatch
  shot triggers automatically at reduced accuracy. Verify cover icons and overwatch cones render
  on the grid.

## R-13.21.5 Hit Probability and Combat Resolution

The engine **SHALL** compute and display pre-confirmation hit probability from base accuracy, range
penalty, cover bonus, elevation advantage, flanking, buffs/debuffs, and per-unit stats, resolve
combat with weighted random producing miss, graze, hit, or critical outcomes, and display damage
preview and outcome feedback with distinct animations and floating text.

- **Derived from:** [F-13.21.5](../../features/game-framework/turn-based.md)
- **Rationale:** Transparent hit probability lets players make informed tactical decisions, while
  multi-tier outcomes (miss/graze/hit/crit) add variance that keeps combat dynamic.
- **Verification:** Set up an attack with known factors: 80% base accuracy, -10% range penalty,
  -20% half cover = 50% hit chance. Verify the UI displays 50%. Execute 1,000 attacks with this
  setup; verify the hit rate falls within the 95% confidence interval of 50%. Verify miss, graze,
  hit, and critical outcomes each produce distinct feedback animations.

## Non-Functional Requirements

### NFR-13.21.1 Tactical Grid Performance

Grid pathfinding for movement range display **SHALL** complete within 2ms for grids up to
100x100 cells. Hit probability computation **SHALL** complete within 0.1ms per attack.
Grid visualization (cell highlights, cover icons, overwatch cones) **SHALL** render within
0.5ms per frame for up to 10,000 visible cells.

- **Rationale:** Turn-based games require instant visual feedback when hovering over cells
  and previewing attacks. Pathfinding and probability must be fast for responsive UI.
- **Verification:** Generate a 100x100 grid and compute movement range for a unit with 6 AP.
  Verify pathfinding completes within 2ms. Compute hit probability for 100 simultaneous
  attack previews. Verify total time stays under 10ms. Render 10,000 highlighted cells and
  verify rendering time stays under 0.5ms.

### NFR-13.21.2 Turn Transition Responsiveness

Turn transitions (camera focus, UI update, AI turn start) **SHALL** complete within 500ms.
AI turn evaluation for a single unit **SHALL** complete within 200ms to maintain game pacing.
Player input during their turn **SHALL** be processed within 1 frame.

- **Rationale:** Slow turn transitions and AI evaluation frustrate players. Responsive turns
  maintain engagement in turn-based gameplay.
- **Verification:** Trigger a turn transition and measure time from turn end to next turn
  start including camera focus. Verify it completes within 500ms. Measure AI unit evaluation
  time and verify it stays under 200ms.
