# R-15.21 -- Specialized Editor Requirements

## Entity Editing

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.21.1 | The editor **SHALL** provide an inspector panel that displays all components on the selected entity with type-appropriate property widgets, supports adding and removing components via a searchable palette, and integrates with undo/redo (F-15.1.3) and collaborative editing (F-15.12.3). | [F-15.21.1](../../features/tools-editor/specialized-editors.md) | Entity inspection is the primary editing interface for all ECS-based content. | Integration test: select an entity, add a component via the palette, undo the addition, and verify the component is removed. |

## Graph Editors

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.21.2 | The editor **SHALL** provide a visual graph editor for animation blending, transitions, and state machines with real-time preview on a character model, scrubbing, slow motion, and frame stepping. The graph editor **SHALL** use animation-specific node types built on the logic graph runtime (F-15.8.1). | [F-15.21.2](../../features/tools-editor/specialized-editors.md) | Animation state machines are too complex for text-based editing; visual graphs with live preview accelerate iteration. | Integration test: create a blend between two animation clips and verify the preview renders the blended result. |
| R-15.21.3 | The editor **SHALL** provide a visual tree editor for AI behavior trees with execution state overlays (active, succeeded, failed), subtree references for reusable patterns, and team-shared behavior libraries. | [F-15.21.3](../../features/tools-editor/specialized-editors.md) | AI behavior trees require visual debugging to understand decision flow at runtime. | Integration test: create a behavior tree, run it in play mode, and verify active node highlighting. |
| R-15.21.4 | The editor **SHALL** provide a general-purpose visual state machine editor with hierarchical states, parallel regions, history states, and execution visualization during play mode. The editor **SHALL** be reusable across animation, AI, gameplay, and UI systems. | [F-15.21.4](../../features/tools-editor/specialized-editors.md) | A shared state machine editor avoids duplicating visual editing infrastructure across subsystems. | Integration test: create a hierarchical state machine with a sub-machine and verify execution visualization shows the current state during play mode. |

## Data Table Editors

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.21.5 | The editor **SHALL** provide a visual quest graph editor with quest nodes showing objectives, completion conditions, and rewards. The editor **SHALL** validate graph connectivity, flag unreachable quests, and simulate quest progression for testing. The editor **SHALL** integrate with dialogue (F-13.6.4) and localization (F-15.13.1). | [F-15.21.5](../../features/tools-editor/specialized-editors.md) | Quest chains require graph visualization for dependency tracking and completeness validation. | Integration test: create a quest graph with an unreachable quest and verify the editor flags it. |
| R-15.21.6 | The editor **SHALL** provide a loot table editor with probability-weighted item pools, quantity ranges, conditional modifiers (player level, difficulty, luck), nested sub-tables, and a simulation mode that rolls thousands of times and displays distribution histograms. | [F-15.21.6](../../features/tools-editor/specialized-editors.md) | Loot balance requires statistical simulation to verify probability distributions before playtesting. | Integration test: create a loot table, run 10,000 simulations, and verify the histogram matches the configured weights within a 5% tolerance. |
| R-15.21.7 | The editor **SHALL** provide an ability definition editor with cooldowns, resource costs, targeting rules, effects, combo chains, summary cards, category filtering, and comparison views for balance review. The editor **SHALL** link to VFX (F-11.6.1) and animation (F-9.4.1) assets per ability. | [F-15.21.7](../../features/tools-editor/specialized-editors.md) | Ability definitions span multiple subsystems; a unified editor accelerates balance iteration. | Integration test: create two abilities with different costs and verify comparison view displays both side by side. |
| R-15.21.8 | The editor **SHALL** provide an equipment statistics editor with stat columns, slot assignments, set bonuses, level requirements, rarity tiers, comparison views, balance heatmaps, and CSV export. The editor **SHALL** support formula references (F-13.7.5) for computed stats. | [F-15.21.8](../../features/tools-editor/specialized-editors.md) | Equipment balance requires tabular views with outlier detection across hundreds of items. | Integration test: create equipment entries with outlier stats and verify the balance heatmap highlights them. |
| R-15.21.9 | The editor **SHALL** provide an economy balancing editor with currency definitions, vendor price tables, exchange rates, price-over-progression curves, sink/faucet analysis, and time simulation. The editor **SHALL** support multiple currency types and conversion rate matrices. | [F-15.21.9](../../features/tools-editor/specialized-editors.md) | Game economies require simulation to prevent inflation and balance currency flows. | Integration test: define two currencies with an exchange rate and verify the simulation detects an unsustainable faucet. |

## Visual Graph Editors

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.21.10 | The editor **SHALL** provide a node-based VFX authoring editor wrapping the effect graph system (F-11.6.1) with real-time preview alongside the graph. The editor **SHALL** provide VFX-specific nodes for particle emission, force fields, collision, rendering, and event triggers. | [F-15.21.10](../../features/tools-editor/specialized-editors.md) | VFX authoring requires immediate visual feedback while editing graph parameters. | Integration test: add a particle emission node and verify the preview renders particles in real time. |
| R-15.21.11 | The editor **SHALL** provide a node-based material authoring editor wrapping the material graph system (F-15.3.1) with live material preview on a configurable mesh. The editor **SHALL** provide nodes for texture sampling, math operations, UV manipulation, and PBR parameter outputs. | [F-15.21.11](../../features/tools-editor/specialized-editors.md) | Material creation requires real-time preview to evaluate PBR parameter changes. | Integration test: connect a texture sample node to the base color output and verify the preview mesh updates in real time. |
| R-15.21.12 | The plugin API **SHALL** support defining custom logic graph node types with inputs, outputs, parameter schemas, execution logic, node categories, documentation strings, and custom thumbnail icons. Custom nodes **SHALL** appear in the graph node palette alongside built-in nodes and participate in graph compilation (F-15.8.12) and debugging (F-15.8.11). | [F-15.21.12](../../features/tools-editor/specialized-editors.md) | Studios need domain-specific graph nodes without modifying the engine. | Integration test: register a custom node via plugin, add it to a graph, compile the graph, and verify execution produces the expected output. |

---

## User Story Traceability

User stories for this domain are maintained in
[user-stories/tools-editor/specialized-editors.md](../../user-stories/tools-editor/specialized-editors.md).
Requirements in this document are derived from those user stories.
