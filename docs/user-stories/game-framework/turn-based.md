# User Stories -- Turn-Based and Tactical Systems (13.21)

## Tactical Grid and Turns

| ID            | Persona                 |
|---------------|-------------------------|
| US-13.21.1.1  | game designer (P-5)     |
| US-13.21.1.2  | game developer (P-15)   |
| US-13.21.1.3  | player (P-23)           |
| US-13.21.2.1  | game designer (P-5)     |
| US-13.21.2.2  | game developer (P-15)   |
| US-13.21.2.3  | player (P-23)           |
| US-13.21.3.1  | game designer (P-5)     |
| US-13.21.3.2  | player (P-23)           |
| US-13.21.3.3  | player (P-23)           |

1. **US-13.21.1.1** -- **As a** game designer (P-5), **I want** to choose between square and
   hexagonal grids per level with auto-generation from collision geometry or hand-authoring,
   **so that** grid type matches the game's tactical style.

2. **US-13.21.1.2** -- **As a** game developer (P-15), **I want** per-cell data for traversability,
   elevation, cover values, terrain type, and occupancy, **so that** the grid encodes all tactical
   information.

3. **US-13.21.1.3** -- [game-specific] **As a** player (P-23), **I want** the grid to display
   colored highlights showing reachable cells (blue), enemy range (red), and ability AoE (yellow),
   **so that** I plan moves visually.

4. **US-13.21.2.1** -- **As a** game designer (P-5), **I want** to choose between round-robin,
   initiative-based, team-based, and phase-based turn modes, **so that** the turn system fits my
   game design.

5. **US-13.21.2.2** -- **As a** game developer (P-15), **I want** AI and player units to use the
   same turn interface, **so that** AI turns execute identically to player turns.

6. **US-13.21.2.3** -- [game-specific] **As a** player (P-23), **I want** a turn order bar showing
   upcoming actors with portraits, **so that** I know who acts next.

7. **US-13.21.3.1** -- **As a** game designer (P-5), **I want** to configure AP costs per ability
   and movement cost per terrain type, **so that** tactical economy is data-driven.

8. **US-13.21.3.2** -- [game-specific] **As a** player (P-23), **I want** to split AP between
   movement and actions in any order, **so that** I move, attack, then move again if AP remains.

9. **US-13.21.3.3** -- [game-specific] **As a** player (P-23), **I want** movement range displayed
   as highlighted reachable cells with path preview and AP cost on hover, **so that** I plan moves
   before committing.

## Cover, Overwatch, and Combat

| ID            | Persona                 |
|---------------|-------------------------|
| US-13.21.4.1  | game designer (P-5)     |
| US-13.21.4.2  | game developer (P-15)   |
| US-13.21.4.3  | player (P-23)           |
| US-13.21.4.4  | player (P-23)           |
| US-13.21.5.1  | game designer (P-5)     |
| US-13.21.5.2  | player (P-23)           |
| US-13.21.5.3  | player (P-23)           |

1. **US-13.21.4.1** -- **As a** game designer (P-5), **I want** cover values (half, full)
   configurable per object and per edge direction with overwatch accuracy modifiers, **so that** I
   design diverse cover layouts.

2. **US-13.21.4.2** -- **As a** game developer (P-15), **I want** cover to be directional so
   flanking negates cover bonuses and optionally grants accuracy bonuses, **so that** positional
   tactics matter.

3. **US-13.21.4.3** -- [game-specific] **As a** player (P-23), **I want** cover objects to provide
   defense bonuses from the covered direction, **so that** positioning behind cover reduces damage.

4. **US-13.21.4.4** -- [game-specific] **As a** player (P-23), **I want** to spend AP to enter
   overwatch and automatically fire at the first enemy that moves in line of sight, **so that** I
   control space on the enemy turn.

5. **US-13.21.5.1** -- **As a** game designer (P-5), **I want** hit probability computed from weapon
   accuracy, range, cover, elevation, flanking, buffs, and unit stats, **so that** tactical factors
   create meaningful variance.

6. **US-13.21.5.2** -- [game-specific] **As a** player (P-23), **I want** hit probability displayed
   as a percentage before confirming attacks, **so that** I make informed attack decisions.

7. **US-13.21.5.3** -- [game-specific] **As a** player (P-23), **I want** miss, graze, hit, and
   critical results distinguished with feedback animations and floating text, **so that** each
   outcome feels distinct.
