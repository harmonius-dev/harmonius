# R-13.17 -- Traversal and Interaction Requirements

## Interaction System

1. **R-13.17.1** -- The engine **SHALL** provide a data-driven interaction system with look-at
   raycast or proximity detection, configurable interaction types (instant, channeled, automatic),
   required items, logic graph execution, and radial menu for multiple interactions.
   - **Rationale:** A generic interaction system is the engine primitive for all object interaction
     in any game genre.
   - **Verification:** Configure a channeled interaction and verify it cancels if the player moves.
     Configure a radial menu with two options and verify both are selectable.

2. **R-13.17.2** -- The engine **SHALL** provide configurable door entities with swing types, locked
   state with key requirement, breakable HP, auto-close timer, access permissions, and AI-aware
   pathfinding integration.
   - **Rationale:** Doors are a fundamental interactable primitive used across virtually all game
     genres.
   - **Verification:** Lock a door and verify an NPC without a key pathfinds around it. Verify an
     NPC with a key opens it.

3. **R-13.17.3** -- The engine **SHALL** support physics object pickup, carry, and throw with
   configurable hold distance, spring stiffness, throw strength, and weight-based movement penalty.
   - **Rationale:** Physics pickup is the engine primitive for environmental puzzle and
     physics-based interaction systems.
   - **Verification:** Pick up a heavy object and verify movement speed reduction. Throw it and
     verify force matches the configured throw strength.

## Traversal Detection

1. **R-13.17.4** -- The engine **SHALL** detect traversal opportunities via shape casts and classify
   geometry by dimensions (vault, mantle, wall-run, balance beam, slide-under) with configurable
   thresholds and support for auto-detection or explicit tagging.
   - **Rationale:** Traversal detection is the engine primitive for any parkour, platforming, or
     movement-based gameplay.
   - **Verification:** Place a 0.5 m obstacle and verify vault classification. Place a 1.5 m
     obstacle and verify mantle. Verify traversal is blocked when stamina is insufficient.

## Climbing and Vertical Traversal

1. **R-13.17.5** -- The engine **SHALL** provide IK-driven free-climb on tagged surfaces with grip
   points, stamina drain, rest points, ladder entities with no stamina cost, and ledge grab with
   shimmy and pull-up.
   - **Rationale:** Climbing primitives enable vertical traversal with configurable difficulty and
     risk.
   - **Verification:** Deplete stamina while climbing and verify the character falls. Use a ladder
     and verify no stamina drain. Grab a ledge and verify lateral shimmy.

## Aquatic Locomotion

1. **R-13.17.6** -- The engine **SHALL** transition the character controller to swim locomotion on
   water volume entry with configurable speed, buoyancy, oxygen meter, drowning damage, and
   underwater visual effects.
   - **Rationale:** Swim locomotion is the engine primitive for any aquatic environment traversal.
   - **Verification:** Enter a water volume and verify locomotion switches. Submerge and verify
     oxygen drains. Deplete oxygen and verify drowning damage.

## Rope and Cable Traversal

1. **R-13.17.7** -- The engine **SHALL** provide grapple-pull, grapple- swing (pendulum physics),
   and zipline traversal with configurable range, speed, and swing parameters, using rope/cable
   physics (F-4.3.4) for simulation.
   - **Rationale:** Rope traversal is the engine primitive for grappling hook and zipline gameplay
     mechanics.
   - **Verification:** Fire a grappling hook beyond range and verify it fails. Fire within range and
     verify the character is pulled toward the anchor.

## Interaction Framework

1. **R-13.17.8** -- The engine **SHALL** resolve interaction priority when multiple objects are in
   range using configurable scoring (distance, alignment, designer priority, recency penalty).
   - **Rationale:** Priority resolution prevents ambiguous interaction when multiple targets
     overlap.
   - **Verification:** Place two interactables and verify the closer one receives focus. Set a
     designer override on the farther one and verify it wins.

2. **R-13.17.9** -- The engine **SHALL** support multi-step interaction chains with conditional
   branching, partial completion tracking, and per-step events.
   - **Rationale:** Interaction chains enable complex environmental puzzles and multi-phase
     interactions.
   - **Verification:** Interrupt a chain at step 2 and verify it resumes at step 2 on
     re-interaction.

3. **R-13.17.10** -- The engine **SHALL** support cooperative interactions requiring multiple
   participants with role assignments, synchronized animations, and cross-client synchronization
   (F-8.2.1).
   - **Rationale:** Cooperative interactions enable multiplayer puzzles and team-based environmental
     challenges.
   - **Verification:** Start a 2-player interaction and verify it cancels if one participant leaves
     range.

4. **R-13.17.11** -- The engine **SHALL** persist interaction state (completed interactions,
   remaining uses, cooldown timers, chain progress) across save/load cycles via the save system
   (F-13.3.1).
   - **Rationale:** State persistence ensures half-completed chains and cooldowns survive session
     boundaries.
   - **Verification:** Save mid-chain, load, and verify the chain resumes at the correct step.

5. **R-13.17.12** -- The engine **SHALL** evaluate context-sensitive interaction options based on
   character state, equipped items, quest progress, time of day, and environmental conditions via
   logic graph predicates.
   - **Rationale:** Context-sensitive interactions enable dynamic world objects that respond to
     player state.
   - **Verification:** Equip a key mid-approach and verify the "unlock" option appears dynamically.

## Interaction Feedback and Networking

1. **R-13.17.13** -- The engine **SHALL** provide configurable multi- sensory interaction feedback
   (outline, ambient sound, haptic) with intensity scaling by distance to the focused interactable.
   - **Rationale:** Multi-sensory feedback signals interaction availability without UI clutter.
   - **Verification:** Approach an interactable and verify outline intensity increases with
     proximity.

2. **R-13.17.14** -- The engine **SHALL** support remote (projectile, thrown object, ability) and
   area (pressure plate, proximity sensor) triggers feeding into the same interaction pipeline as
   direct input.
   - **Rationale:** Unified trigger sources enable consistent interaction execution regardless of
     activation method.
   - **Verification:** Shoot a switch with a projectile and verify the interaction fires.

3. **R-13.17.15** -- The engine **SHALL** replicate interaction state and animations across clients
   with server-authoritative validation, client prediction on start, and reconciliation on
   confirmation (F-8.2.1, F-8.4.1).
   - **Rationale:** Server authority with prediction ensures consistent and responsive networked
     interactions.
   - **Verification:** Request an interaction from a client and verify the server validates range
     and requirements before execution.

4. **R-13.17.16** -- The engine **SHALL** allow AI agents to evaluate and execute interactions using
   the same system as players, with scoring against behavior tree goals and respect for requirements
   and cooldowns.
   - **Rationale:** Shared interaction systems ensure NPC behavior is consistent with player
     capabilities.
   - **Verification:** Verify an NPC navigates to and executes an interaction using the same
     pipeline as a player.

5. **R-13.17.17** -- The engine **SHALL** bind interaction types to animation montages with IK
   targets for hand placement, body orientation, and approach/exit animations, configurable per
   interactable without code.
   - **Rationale:** Animation binding enables new interactable types with custom animations from
     data alone.
   - **Verification:** Create a new interactable with custom IK targets and verify hand placement on
     the mesh surface.
