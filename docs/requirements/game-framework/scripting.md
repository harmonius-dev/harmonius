# R-13.4 — Gameplay Scripting Integration Requirements

## Visual Logic

1. **R-13.4.1** — The engine **SHALL** provide gameplay-specific logic graph integration points for
   ECS world access (entities, components, resources), gameplay event emission/handling, ability
   activation, state machine transitions, and AI blackboard read/write, with all gameplay scripting
   authored through the visual logic graph system and no text-based scripting language.
   - **Rationale:** Visual-only scripting enforces the no-code philosophy while providing full ECS
     integration.
   - **Verification:** Create a logic graph that reads a Health component, checks a threshold, and
     emits a "low health" event. Attach to an entity and verify it executes in the ECS schedule.
     Verify no text-based scripting API is exposed.

2. **R-13.4.2** — The engine **SHALL** support visual debugging of gameplay logic graphs with ECS
   query result inspection, component value watch expressions, event flow visualization, per-entity
   graph state comparison, and remote debugging over the network.
   - **Rationale:** Visual debugging enables designers to diagnose logic without code knowledge;
     remote debugging supports deployed builds.
   - **Verification:** Set a breakpoint in a logic graph. Trigger it and verify game pauses with
     correct component values displayed. Connect a remote debugger to a running instance and verify
     graph state is visible.

3. **R-13.4.3** — The engine **SHALL** support hot reload of gameplay logic graphs with changes
   applied immediately without game restart, preserving persistent state (local variables, coroutine
   positions) across reloads when layout is compatible, with clean restart on incompatible changes.
   - **Rationale:** Immediate hot reload enables rapid iteration; state preservation avoids replay.
   - **Verification:** Modify a graph while the game runs. Verify the change is visible within 1
     second. Verify local variables retain their values. Make an incompatible change and verify the
     graph restarts with a warning.
