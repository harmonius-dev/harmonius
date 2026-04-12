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

4. **R-13.4.4** — The engine **SHALL** compile logic graphs to Rust source (CPU targets) or HLSL
   source (GPU targets) through a unified compiler with shared optimization passes.
   - **Rationale:** A unified compiler ensures consistent optimization and error reporting across
     CPU and GPU graph targets.
   - **Verification:** Compile a graph targeting CPU and verify Rust output. Compile a shader graph
     targeting GPU and verify HLSL output. Verify both share the same constant-folding pass.

5. **R-13.4.5** — The engine **SHALL** enforce sandbox constraints on user-authored logic graphs,
   preventing unsafe code, raw pointers, and unbounded loops, with budget checks inserted at loop
   back-edges.
   - **Rationale:** Sandboxing prevents user graphs from crashing the engine or consuming unbounded
     resources.
   - **Verification:** Author a graph with an infinite loop, compile, run, and verify the budget
     check terminates execution with a BudgetExhausted error.

6. **R-13.4.6** — The engine **SHALL** handle logic graph runtime errors (missing component,
   division by zero, budget exhaustion, type mismatch, empty query) gracefully by transitioning the
   graph instance to an error state without crashing.
   - **Rationale:** Runtime errors in user logic must not crash the engine or corrupt game state.
   - **Verification:** Author a graph that queries a missing component, run it, and verify the graph
     enters an error state with a diagnostic message.

7. **R-13.4.7** — The engine **SHALL** support three variable scopes for logic graphs: local (reset
   each execution), graph (persistent per instance), and entity (mapped to ECS components).
   - **Rationale:** Different storage lifetimes serve different gameplay patterns (transient
     computation vs. persistent state).
   - **Verification:** Create variables in each scope, execute the graph twice, and verify local
     resets, graph persists, and entity maps to the correct component.

8. **R-13.4.8** — The engine **SHALL** support coroutine wait conditions including next-frame,
   frame-count delay, time delay, and event-based resume.
   - **Rationale:** Multi-frame gameplay sequences require diverse wait conditions beyond simple
     frame delays.
   - **Verification:** Author a coroutine with a 0.5-second time delay, run it, and verify it
     resumes after approximately 0.5 seconds of game time.

9. **R-13.4.9** — The engine **SHALL** emit ECS events when a graph instance completes, errors, or
   hot-reloads, enabling gameplay systems to react to graph lifecycle changes.
   - **Rationale:** Graph lifecycle events enable cleanup, retry logic, and UI feedback tied to
     graph execution state.
   - **Verification:** Register an event listener, run a graph to completion, and verify a
     GraphEvent::Completed event fires.

10. **R-13.4.10** — The engine **SHALL** provide per-graph execution timing in the profiler, with
    zone names from logic graph entry points.
    - **Rationale:** Per-graph profiling enables designers to identify expensive graphs without
      engine-level profiling knowledge.
    - **Verification:** Run 5 graphs simultaneously, open the profiler, and verify each graph
      appears as a named zone with correct timing.
