# 13.21 — Turn-Based and Tactical Systems

### F-13.21.1 Tactical Grid System

Square or hexagonal grid overlay on the game world with per-cell data: traversability (walkable,
blocked, difficult terrain), elevation tier, cover values (none, half, full per edge), terrain type
(for movement cost), and occupancy. The grid is generated from world collision geometry or
hand-authored in the level editor. Grid cells are visualized with colored highlights during
gameplay: blue for reachable, red for enemy range, yellow for ability AoE. The grid system supports
multi-floor grids for buildings with stairs connecting elevation tiers.

- **Requirements:** R-13.21.1
- **Dependencies:** F-1.9.4 (Spatial Query), F-2.10.1 (ECS-to-Renderer Bridge)
- **Platform notes:** None

### F-13.21.2 Turn Manager and Initiative

Turn order resolution for turn-based games. Turn modes: round-robin (fixed order), initiative-based
(speed stat determines order each round), team-based (all of team A acts, then team B), and
phase-based (movement phase, action phase, end phase for all units). The turn manager maintains the
current actor, turn count, and phase. UI displays the turn order bar showing upcoming actors with
portraits. Turn transitions trigger camera focus on the active unit. AI and player units use the
same turn interface — player turns await input while AI turns execute behavior trees automatically.

- **Requirements:** R-13.21.2
- **Dependencies:** F-13.21.1, F-7.3.1 (Behavior Trees)
- **Platform notes:** None

### F-13.21.3 Action Point Movement and Abilities

Per-unit action point (AP) budget spent on movement and abilities each turn. Movement costs AP
proportional to distance and terrain difficulty. Each ability costs configurable AP. Units can split
their AP between movement and actions in any order (move, attack, move again if AP remains). The
movement range display highlights all reachable cells given remaining AP, with path preview on hover
showing AP cost. Ability targeting displays AoE shapes, range limits, and hit probability. End-turn
passes remaining AP (or saves it for overwatch). AP resets at the start of each unit's turn.

- **Requirements:** R-13.21.3
- **Dependencies:** F-13.21.2, F-13.21.1, F-13.10.1 (Abilities)
- **Platform notes:** None

### F-13.21.4 Grid Cover and Overwatch

Cover mechanics on the tactical grid. Cover objects (walls, crates, pillars) provide half or full
cover bonuses to defense. Cover is directional — only effective against attacks from the covered
direction. Flanking (attacking from a direction without cover) negates cover bonuses and may grant
accuracy/damage bonuses. Overwatch: a unit spends AP to enter overwatch stance, automatically firing
at the first enemy that moves within line-of-sight during the enemy turn. Overwatch shots use a
reduced accuracy modifier. Both cover values and overwatch state are visualized on the grid with
shield icons and overwatch cones.

- **Requirements:** R-13.21.4
- **Dependencies:** F-13.21.3, F-13.21.1
- **Platform notes:** None

### F-13.21.5 Hit Probability and Combat Resolution

Calculate and display hit probability before confirming attacks. Factors: base weapon accuracy,
range (distance penalty), cover (half/full defense bonus), elevation (height advantage bonus),
flanking (negate cover), active buffs/debuffs, and per-unit stats (accuracy, evasion). The
probability is displayed as a percentage in the targeting UI alongside damage preview (min-max
range). Combat resolution uses weighted random with the computed probability. Critical hits trigger
at a separate crit chance with bonus damage. Miss, graze (partial damage), hit, and critical are
distinguished with feedback animations and floating text.

- **Requirements:** R-13.21.5
- **Dependencies:** F-13.21.4, F-13.7.5 (Formula DSL), F-13.10.3 (Gameplay Effects)
- **Platform notes:** None
