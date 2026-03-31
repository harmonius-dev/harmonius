# R-13.14 -- Building and Survival Requirements

## Snap Placement

1. **R-13.14.1** -- The engine **SHALL** provide a socket-based snap placement system with
   configurable attachment points, rotation increments, placement validation, and ghost preview with
   validity indication.
   - **Rationale:** Socket-based snapping provides predictable alignment while data-driven rules
     enable per-game customization.
   - **Verification:** Place a wall piece and verify it snaps to valid sockets. Attempt placement in
     a collision overlap and verify rejection with red ghost preview.

2. **R-13.14.2** -- The engine **SHALL** support multi-phase construction with configurable
   duration, material costs, visual stage transitions, and cancellation with partial resource
   refunds.
   - **Rationale:** Phased construction enables survival and strategy game loops where building is
     an investment.
   - **Verification:** Place a building and verify visual transitions at configured thresholds.
     Cancel at 50% and verify the configured refund fraction is returned.

3. **R-13.14.3** -- The engine **SHALL** compute structural stability by propagating support values
   from grounded foundations, with cascade collapse when stability reaches zero, computed
   incrementally on piece add, remove, or damage.
   - **Rationale:** Incremental computation avoids per-frame cost for large structures while
     enabling dynamic collapse.
   - **Verification:** Remove a mid-structure support and verify dependent pieces above threshold
     distance collapse. Verify stability recomputation under 1 ms for a 500-piece structure.

4. **R-13.14.4** -- The engine **SHALL** support data-driven material tier upgrades,
   damage-proportional repair, and configurable real-time decay.
   - **Rationale:** Tier upgrades and decay create long-term building maintenance loops driven by
     data rather than code.
   - **Verification:** Upgrade a wood piece to stone and verify mesh, HP, and stability change. Set
     decay rate and verify HP decreases over configured intervals.

## Housing and Furniture

1. **R-13.14.5** -- The engine **SHALL** support instanced housing zones with visitor permissions,
   furniture placement with separate grid/free placement, and data-driven functional effects per
   furniture type.
   - **Rationale:** Housing with permissions provides personalized spaces; functional furniture
     enables designer-authored benefits.
   - **Verification:** Set a housing instance to private and verify a non-permitted visitor cannot
     enter. Place a bed and verify the respawn point updates.

## Consumable Resource Pools

1. **R-13.14.6** -- The engine **SHALL** provide configurable consumable resource pools with drain
   rates, restoration values, activity multipliers, environmental modifiers, and threshold-triggered
   debuffs via the gameplay effect system.
   - **Rationale:** Consumable resource pools are the engine primitive underlying hunger, thirst,
     warmth, and stamina systems.
   - **Verification:** Configure a pool with drain rate 1/s. Verify depletion in the expected time.
     Verify threshold debuff triggers.

## Harvestable Nodes

1. **R-13.14.7** -- The engine **SHALL** provide data-driven harvestable resource nodes with
   configurable yield, gather time, tool requirement, HP depletion, respawn timer, and procedural
   distribution rules.
   - **Rationale:** Harvestable nodes are the engine primitive for any gather-and-collect gameplay
     loop.
   - **Verification:** Configure a node requiring a pickaxe. Verify gathering without the pickaxe
     fails. Deplete the node and verify respawn after the configured timer.

## Growth Simulation

1. **R-13.14.8** -- The engine **SHALL** provide a multi-stage growth simulation with configurable
   stage durations, maintenance requirements, neglect penalties, and environmental modifiers.
   - **Rationale:** Growth simulation is the engine primitive for farming, crop growth, and
     time-progression systems.
   - **Verification:** Configure 4-stage growth with watering required. Skip watering and verify
     withering after the grace period.

## Trait Inheritance

1. **R-13.14.9** -- The engine **SHALL** support data-driven trait inheritance for bred entities
   with configurable genetic rules, parent trait blending, and random variation within bounds.
   - **Rationale:** Trait inheritance is the engine primitive for any breeding or genetic
     combination system.
   - **Verification:** Breed two entities with distinct traits and verify offspring traits within
     the configured variation range.
