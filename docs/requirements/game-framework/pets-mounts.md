# R-13.15 -- Pets, Companions, and Mounts Requirements

## Companion AI

1. **R-13.15.1** -- The engine **SHALL** provide AI-controlled companion entities with
   command-selected behavior tree subtrees, pathfinding-based follow with configurable distance,
   teleport catch-up, and AI-controlled ability activation from gameplay database stats.
   - **Rationale:** Behavior tree subtrees per command enable distinct modes; teleport catch-up
     prevents companion loss.
   - **Verification:** Issue "guard" and verify the companion holds position and attacks approaching
     enemies. Move beyond the teleport threshold and verify the companion teleports to the player.

2. **R-13.15.2** -- The engine **SHALL** provide configurable needs meters (hunger, happiness,
   cleanliness) on companion entities with drain rates, care action effects, mood thresholds, and
   mood-dependent behavior and animation selection.
   - **Rationale:** Needs meters are the engine primitive for any pet-care or creature maintenance
     system.
   - **Verification:** Set happiness to zero and verify the companion refuses commands. Feed the
     companion and verify happiness restores by the configured amount.

## Mount Controller

1. **R-13.15.3** -- The engine **SHALL** provide a mount controller that switches the character
   controller to mount-specific physics (speed, acceleration, turn rate), supports ground, flying,
   and aquatic movement modes, restricts abilities to a configured allowed set, and plays
   mount-specific enter/exit/combat animations.
   - **Rationale:** A mount controller is the engine primitive for any rideable entity with distinct
     locomotion.
   - **Verification:** Mount a flying mount and verify altitude limits are enforced. Attempt a
     disallowed ability while mounted and verify it is blocked.

## Taming Progress

1. **R-13.15.4** -- The engine **SHALL** provide a multi-attempt taming progress system with success
   probability affected by relative level, skill, and item quality, with configurable failure
   consequences (flee or attack) and per-species prerequisites.
   - **Rationale:** Taming progress is the engine primitive for any gradual creature acquisition
     system.
   - **Verification:** Feed a creature below the required food quality and verify zero taming
     progress. Complete taming and verify the creature becomes a companion or mount as configured.

## Life Stages and Evolution

1. **R-13.15.5** -- The engine **SHALL** support data-driven life stages (growth phases with stat
   and visual changes), evolution branching based on configurable conditions (diet, training,
   items), and breeding with trait inheritance from both parents.
   - **Rationale:** Life stages and evolution are the engine primitives for creature progression and
     specialization.
   - **Verification:** Advance a creature to the next life stage and verify stat and visual changes.
     Feed exclusively one diet and verify the correct evolution branch is selected. Breed two
     creatures and verify offspring traits within the configured range.
