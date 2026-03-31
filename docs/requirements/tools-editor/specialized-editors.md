# R-15.21 -- Specialized Editor Requirements

## Requirements

1. **R-15.21.1** — The engine **SHALL** provide an entity inspector with component display,
   searchable palette, hierarchy navigation, and undo/redo integration.
   - **Rationale:** Entity inspection is the most common editing operation.
   - **Verification:** Add a component, undo, and verify it is removed.

2. **R-15.21.2** — The engine **SHALL** provide a visual animation graph editor with
   blend/transition nodes, real-time preview, and scrubbing.
   - **Rationale:** Visual animation authoring is required for the no-code workflow.
   - **Verification:** Create a blend node and verify the preview shows interpolated output.

3. **R-15.21.3** — The engine **SHALL** provide a behavior tree editor with execution state overlays
   and subtree references.
   - **Rationale:** AI behavior authoring requires visual debugging during play mode.
   - **Verification:** Run a behavior tree and verify active node highlighting.

4. **R-15.21.4** — The engine **SHALL** provide a general-purpose state machine editor with
   hierarchical states, parallel regions, and execution visualization.
   - **Rationale:** State machines are used across animation, AI, gameplay, and UI systems.
   - **Verification:** Create a hierarchical state machine and verify sub-state entry.

5. **R-15.21.5** — The engine **SHALL** provide a quest graph editor with objectives, prerequisites,
   branching, rewards, and graph validation.
   - **Rationale:** Quest authoring requires visual graph tools with connectivity validation.
   - **Verification:** Create a disconnected quest and verify the validator flags it.

6. **R-15.21.6** — The engine **SHALL** provide a loot table editor with probability weights, nested
   sub-tables, and distribution simulation.
   - **Rationale:** Loot balance requires statistical simulation.
   - **Verification:** Simulate 10,000 drops and verify the histogram matches configured weights.

7. **R-15.21.7** — The engine **SHALL** provide an ability definition editor with cooldowns, costs,
   targeting, effects, combos, and comparison views.
   - **Rationale:** Ability authoring and balance review must be visual and data-driven.
   - **Verification:** Compare two abilities and verify the comparison view shows correct stat
     differences.

8. **R-15.21.8** — The engine **SHALL** provide an equipment stat editor with balance heatmaps and
   CSV export.
   - **Rationale:** Equipment balance across tiers requires visual outlier detection.
   - **Verification:** Import a stat table and verify the heatmap highlights outlier values.

9. **R-15.21.9** — The engine **SHALL** provide an economy editor with price curves, inflation
   modeling, and unsustainable flow detection.
   - **Rationale:** Economic balance must be modeled before live deployment.
   - **Verification:** Create a faucet exceeding sinks and verify the editor flags the imbalance.

10. **R-15.21.10** — The engine **SHALL** provide a node-based Effect Graph editor with real-time
    preview and event trigger nodes.
    - **Rationale:** VFX authoring is visual; real-time preview enables fast iteration.
    - **Verification:** Create a particle emitter node and verify the preview renders particles.

11. **R-15.21.11** — The engine **SHALL** provide a node-based material graph editor with PBR
    outputs and live viewport preview on configurable meshes.
    - **Rationale:** Material authoring is a primary visual workflow.
    - **Verification:** Connect an albedo output and verify the preview updates.

12. **R-15.21.12** — The engine **SHALL** provide a plugin API for custom logic graph node types
    that participate in compilation and debugging.
    - **Rationale:** Extensible node types enable project-specific logic.
    - **Verification:** Register a custom node, use it in a graph, compile, and verify it executes
      correctly.
