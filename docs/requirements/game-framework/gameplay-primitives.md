# R-13.1 — Gameplay Primitives Requirements

## Game Modes and State

1. **R-13.1.1** — The engine **SHALL** provide a hierarchical state machine for game modes governing
   rules, scoring, phase transitions, player count, team composition, and respawn policies, with
   nested sub-modes for encounters.
   - **Rationale:** Hierarchical state machines enable mode composition without duplication.
   - **Verification:** Define a mode with 3 phases. Verify transitions fire on conditions. Define a
     sub-mode inside phase 2 and verify independent state tracking.

2. **R-13.1.2** — The engine **SHALL** manage top-level game state lifecycle (menu, loading,
   in-game, paused, disconnected) with transitions triggering resource loading, UI swaps, and input
   context changes.
   - **Rationale:** Centralized state management ensures all systems respond to lifecycle
     transitions consistently.
   - **Verification:** Transition from menu to in-game and verify resources load, UI swaps, and
     input context changes. Disconnect and verify graceful fallback to disconnected state.

## Player Control

3. **R-13.1.3** — The engine **SHALL** provide a player controller routing input to the possessed
   pawn with input context switching (exploration, combat, vehicle, cinematic) and camera ownership.
   - **Rationale:** Input context switching prevents unintended actions when gameplay mode changes.
   - **Verification:** Switch from exploration to combat context and verify input bindings change.
     Possess a vehicle and verify camera ownership transfers.

4. **R-13.1.4** — The engine **SHALL** separate pawns (any possessable entity) from characters
   (pawns with stats, equipment, and faction) supporting possession transfer for vehicles,
   spectating, and mind control.
   - **Rationale:** Pawn/character separation enables possession mechanics without duplicating
     gameplay logic.
   - **Verification:** Possess a vehicle pawn and verify the character entity retains its
     components. Return to the character and verify all state is preserved.

## Ability Framework and Effects

5. **R-13.1.5** — The engine **SHALL** provide a data-driven ability framework where each ability
   defines activation conditions, costs, targeting rules, cast time, channeling, and interrupt
   priority, with server-authoritative validation.
   - **Rationale:** Data-driven abilities enable designer authoring without code; server authority
     prevents exploits.
   - **Verification:** Define an ability with mana cost and cooldown. Activate and verify cost
     deduction and cooldown start. Attempt activation without mana and verify rejection.

6. **R-13.1.6** — The engine **SHALL** apply stat modifications, buffs, debuffs, damage-over-time,
   healing-over-time, and crowd control as gameplay effects with duration, stacking rules, and
   priority ordering.
   - **Rationale:** A unified effect system avoids per-mechanic special cases.
   - **Verification:** Apply a duration buff and verify it expires. Apply stacking effects and
     verify rules. Apply competing crowd controls and verify priority ordering.

## Damage and Death

7. **R-13.1.7** — The engine **SHALL** compute damage through a configurable pipeline of modifiers
   (scaling, mitigation, crit, dodge, absorb) with multiple damage types and per-type resistances,
   producing events consumed by health, combat log, and replication.
   - **Rationale:** A configurable pipeline enables genre-specific damage formulas without hardcoded
     math.
   - **Verification:** Apply physical damage with armor mitigation and verify reduced output. Apply
     fire damage with fire resistance and verify separate reduction. Verify damage event is received
     by combat log.

8. **R-13.1.8** — The engine **SHALL** handle death transitions (ragdoll, death state, respawn
   selection) and encounter resets (health restore, phase reset) as configurable state transitions.
   - **Rationale:** Configurable death and reset enable varied recovery rules per game mode.
   - **Verification:** Kill a character and verify ragdoll activates. Verify respawn at the selected
     point after timer. Trigger encounter reset and verify all entity states are restored.

## Modular Systems and Plugins

9. **R-13.1.9** — The engine **SHALL** register every gameplay system through the plugin system with
   declared dependencies, independently enableable per project, with dependency validation at load.
   - **Rationale:** Modular registration reduces binary size and editor clutter for unused systems.
   - **Verification:** Disable combat system and verify editor hides combat panels. Enable combat
     and verify physics and animation dependencies are auto-enabled. Verify disabled systems are
     excluded from compilation.

10. **R-13.1.10** — The engine **SHALL** expose a stable Rust plugin API for ECS world access, asset
    hooks, editor extension points, and rendering hooks, with ABI validation on load and a plugin
    template generator.
    - **Rationale:** A stable API enables third-party extensions without engine forks.
    - **Verification:** Load a plugin compiled against engine v1.0 in engine v1.1 and verify ABI
      validation. Load an incompatible plugin and verify rejection. Run the template generator and
      verify a compilable project is produced.
