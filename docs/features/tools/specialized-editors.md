# 15.21 -- Specialized Editors

## Entity Editing

| ID | Feature |
| ----------- | --------------- |
| F-15.21.1 | Entity Editor |

1. **F-15.21.1** — An inspector panel for viewing and editing entity components. Displays all
   components attached to the selected entity with type-appropriate property widgets. Supports
   adding and removing components via a searchable component palette, reordering components, and
   navigating the entity hierarchy tree. Changes integrate with the undo/redo stack (F-15.1.3) and
   collaborative editing (F-15.12.3).
   - **Deps:** F-15.1.4, F-15.1.3, F-7.6.1, F-1.1.1
   - **Platform:** Desktop only. Not available on mobile or console runtime.

## Graph Editors

| ID | Feature |
| ----------- | ------------------------ |
| F-15.21.2 | Animation Graph Editor |
| F-15.21.3 | Behavior Tree Editor |
| F-15.21.4 | State Machine Editor |

1. **F-15.21.2** — A visual graph editor for animation blending, transitions, and state machines.
   Nodes represent animation clips, blend nodes, IK targets, and procedural generators. Edges define
   blending weights and transition conditions. The editor provides real-time preview of the
   animation result on a character model, with scrubbing, slow motion, and frame stepping. Built on
   the logic graph runtime (F-15.8.1) with animation-specific node types.
   - **Deps:** F-15.8.7, F-15.8.1, F-9.4.1, F-15.20.3
   - **Platform:** Desktop only. Not available on mobile or console runtime.
2. **F-15.21.3** — A visual tree editor for AI behavior trees. Nodes represent tasks, conditions,
   decorators, and composite nodes (sequence, selector, parallel). The editor displays execution
   state overlays showing which nodes are active, succeeded, or failed during play mode. Supports
   subtree references for reusable behavior patterns and team-shared behavior libraries.
   - **Deps:** F-15.8.4, F-15.8.1, F-15.20.3
   - **Platform:** Desktop only. Not available on mobile or console runtime.
3. **F-15.21.4** — A general-purpose visual state machine editor reused across animation, AI,
   gameplay, and UI systems. States are represented as nodes; transitions as directed edges with
   condition expressions. The editor supports hierarchical state machines (states containing
   sub-machines), parallel regions, and history states. Execution visualization shows the current
   state and recent transition history during play mode.
   - **Deps:** F-15.8.1, F-15.20.3
   - **Platform:** Desktop only. Not available on mobile or console runtime.

## Data Table Editors

| ID | Feature |
| ----------- | ----------------------- |
| F-15.21.5 | Quest Editor |
| F-15.21.6 | Loot Table Editor |
| F-15.21.7 | Ability Ledger |
| F-15.21.8 | Equipment Stat Tables |
| F-15.21.9 | Price Ledger |

1. **F-15.21.5** — A visual quest graph editor for authoring quest chains with objectives,
   prerequisites, branching paths, and rewards. Each quest node displays its objectives, completion
   conditions, and reward summary. Edges represent prerequisites and unlock relationships. The
   editor validates quest graph connectivity, flags unreachable quests, and simulates quest
   progression for testing. Integrates with the dialogue system (F-13.6.4) and localization
   (F-15.13.1).
   - **Deps:** F-13.6.1, F-15.8.4, F-15.20.3
   - **Platform:** Desktop only. Not available on mobile or console runtime.
2. **F-15.21.6** — A probability-weighted loot table editor with hierarchical groups, weight
   sliders, and drop simulation. Tables define item pools with per-item weights, quantity ranges,
   and conditional modifiers (player level, difficulty, luck stat). The editor provides a simulation
   mode that rolls the table thousands of times and displays distribution histograms for balance
   verification. Supports nested sub-tables for tiered loot.
   - **Deps:** F-13.7.1, F-15.20.3
   - **Platform:** Desktop only. Not available on mobile or console runtime.
3. **F-15.21.7** — An ability definition editor for authoring abilities with cooldowns, resource
   costs, targeting rules, effects, and combo chains. Each ability entry displays a summary card
   with icon, cost, cooldown, and effect list. The editor supports ability categories, tag-based
   filtering, and comparison views for balance review. Links to VFX (F-11.6.1) and animation
   (F-9.4.1) assets for each ability.
   - **Deps:** F-13.10.1, F-13.7.1, F-15.20.3
   - **Platform:** Desktop only. Not available on mobile or console runtime.
4. **F-15.21.8** — An equipment statistics editor with columns for item stats, slot assignments, set
   bonuses, level requirements, and rarity tiers. The editor provides a comparison view for
   side-by-side stat evaluation, a balance heatmap highlighting outliers, and export to CSV for
   external analysis. Supports formula references (F-13.7.5) for computed stats.
   - **Deps:** F-13.7.1, F-13.8.9, F-15.20.3
   - **Platform:** Desktop only. Not available on mobile or console runtime.
5. **F-15.21.9** — An economy balancing editor with currency definitions, vendor price tables,
   exchange rates, and inflation modeling. The editor displays price curves over player progression,
   flags unsustainable sinks or faucets, and simulates economy flow over time. Supports multiple
   currency types and conversion rate matrices.
   - **Deps:** F-13.7.1, F-15.20.3
   - **Platform:** Desktop only. Not available on mobile or console runtime.

## Visual Graph Editors

| ID | Feature |
| ------------ | ---------------------------- |
| F-15.21.10 | Visual Effect Graph Editor |
| F-15.21.11 | Material Graph Editor |
| F-15.21.12 | Custom Logic Graph Nodes |

1. **F-15.21.10** — A node-based VFX authoring editor that wraps the existing effect graph system
   (F-11.6.1). Provides specialized nodes for particle emission, force fields, collision, rendering,
   and event triggers. The editor renders a real-time preview of the effect alongside the graph.
   Built as an in-engine feature editor (F-15.20.3) using the logic graph runtime (F-15.8.1) with
   VFX-specific node types and preview infrastructure.
   - **Deps:** F-11.6.1, F-15.8.1, F-15.20.3
   - **Platform:** Desktop only. Requires GPU for real-time VFX preview rendering.
2. **F-15.21.11** — A node-based material authoring editor that wraps the existing material graph
   system (F-15.3.1). Provides nodes for texture sampling, math operations, UV manipulation, and PBR
   parameter outputs. The editor renders a live material preview on a configurable mesh (sphere,
   cube, custom). Built as an in-engine feature editor (F-15.20.3) using the shader graph variant
   (F-15.8.5c).
   - **Deps:** F-15.3.1, F-15.8.5c, F-15.20.3
   - **Platform:** Desktop only. Requires GPU for real-time material preview.
3. **F-15.21.12** — An API for extending the logic graph with custom node types via the plugin
   system (F-15.20.1). Plugin authors define node inputs, outputs, parameter schemas, and execution
   logic. Custom nodes appear in the graph node palette alongside built-in nodes. The API supports
   node categories, documentation strings, and custom thumbnail icons. Custom nodes participate in
   graph compilation (F-15.8.12) and debugging (F-15.8.11).
   - **Deps:** F-15.8.10, F-15.8.1, F-15.20.1
   - **Platform:** Desktop only. Not available on mobile or console runtime.
