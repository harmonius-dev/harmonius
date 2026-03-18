# User Stories: Specialized Editors

## F-15.21.1 Entity Editor

| ID           | Persona                 | Features | Requirements |
|--------------|-------------------------|----------|--------------|
| US-15.21.1.1 | designer (P-5)          |          |              |
| US-15.21.1.2 | designer (P-5)          |          |              |
| US-15.21.1.3 | developer (P-15)        |          |              |
| US-15.21.1.4 | engine developer (P-26) |          |              |

1. **US-15.21.1.1** — to select an entity and see all its components displayed with type-appropriate
   property widgets
   - **Acceptance:** I can inspect and edit component data without switching tools
2. **US-15.21.1.2** — to add components to an entity via a searchable component palette
   - **Acceptance:** I can find and attach the right component type quickly
3. **US-15.21.1.3** — to remove components from an entity in the inspector with undo support
   - **Acceptance:** I can experiment with different component configurations safely
4. **US-15.21.1.4** — entity editor changes to integrate with collaborative editing (F-15.12.3)
   - **Acceptance:** multiple users can edit different components on the same entity simultaneously

## F-15.21.2 Animation Graph Editor

| ID           | Persona                   | Features | Requirements |
|--------------|---------------------------|----------|--------------|
| US-15.21.2.1 | character animator (P-11) |          |              |
| US-15.21.2.2 | character animator (P-11) |          |              |
| US-15.21.2.3 | designer (P-5)            |          |              |
| US-15.21.2.4 | engine developer (P-26)   |          |              |

1. **US-15.21.2.1** — to create a visual graph connecting animation clips through blend nodes and
   transitions
   - **Acceptance:** I can author complex animation state machines without writing code
2. **US-15.21.2.2** — real-time preview of the animation result on a character model with scrubbing,
   slow motion, and frame stepping
   - **Acceptance:** I can evaluate blending quality immediately
3. **US-15.21.2.3** — to define transition conditions between animation states using visual
   expressions
   - **Acceptance:** I can control when animations change based on gameplay variables
4. **US-15.21.2.4** — animation graphs to compile through the logic graph runtime (F-15.8.1)
   - **Acceptance:** animation blending uses the same optimized execution pipeline as other graph
     types

## F-15.21.3 Behavior Tree Editor

| ID           | Persona                 | Features | Requirements |
|--------------|-------------------------|----------|--------------|
| US-15.21.3.1 | gameplay director (P-3) |          |              |
| US-15.21.3.2 | designer (P-5)          |          |              |
| US-15.21.3.3 | developer (P-15)        |          |              |

1. **US-15.21.3.1** — to build AI behavior trees visually with task, condition, decorator, and
   composite nodes
   - **Acceptance:** I can design NPC decision-making without writing code
2. **US-15.21.3.2** — execution state overlays showing which behavior tree nodes are active,
   succeeded, or failed during play mode
   - **Acceptance:** I can debug AI behavior by visual inspection
3. **US-15.21.3.3** — subtree references for reusable behavior patterns and team-shared behavior
   libraries
   - **Acceptance:** common AI patterns are authored once and reused across multiple NPCs

## F-15.21.4 State Machine Editor

| ID           | Persona                   | Features | Requirements |
|--------------|---------------------------|----------|--------------|
| US-15.21.4.1 | gameplay director (P-3)   |          |              |
| US-15.21.4.2 | character animator (P-11) |          |              |
| US-15.21.4.3 | designer (P-5)            |          |              |

1. **US-15.21.4.1** — a general-purpose visual state machine editor with states as nodes and
   transitions as directed edges with condition expressions
   - **Acceptance:** I can model game state logic visually
2. **US-15.21.4.2** — the state machine editor to be reusable for animation state machines
   - **Acceptance:** animation and gameplay state machines use the same editing workflow
3. **US-15.21.4.3** — hierarchical state machines with sub-machines, parallel regions, and history
   states
   - **Acceptance:** I can model complex systems without a flat state explosion

## F-15.21.5 Quest Editor

| ID           | Persona              | Features | Requirements |
|--------------|----------------------|----------|--------------|
| US-15.21.5.1 | story director (P-4) |          |              |
| US-15.21.5.2 | designer (P-5)       |          |              |
| US-15.21.5.3 | designer (P-5)       |          |              |
| US-15.21.5.4 | story writer (P-17)  |          |              |

1. **US-15.21.5.1** — to author quest chains in a visual graph with quest nodes showing objectives,
   prerequisites, and rewards
   - **Acceptance:** I can see the entire quest flow at a glance
2. **US-15.21.5.2** — the editor to validate quest graph connectivity and flag unreachable quests
   - **Acceptance:** I catch orphaned content before playtesting
3. **US-15.21.5.3** — to simulate quest progression in the editor to test prerequisite chains
   - **Acceptance:** I can verify quest unlock order without playing through the entire game
4. **US-15.21.5.4** — quest nodes linked to dialogue (F-13.6.4) and localization (F-15.13.1) entries
   - **Acceptance:** quest text stays in sync with narrative content

## F-15.21.6 Loot Table Editor

| ID           | Persona                 | Features | Requirements |
|--------------|-------------------------|----------|--------------|
| US-15.21.6.1 | designer (P-5)          |          |              |
| US-15.21.6.2 | designer (P-5)          |          |              |
| US-15.21.6.3 | gameplay director (P-3) |          |              |

1. **US-15.21.6.1** — to configure probability-weighted loot tables with item pools, per-item
   weights, and quantity ranges
   - **Acceptance:** I can control drop rates precisely
2. **US-15.21.6.2** — to run thousands of simulation rolls and see distribution histograms
   - **Acceptance:** I can verify loot balance statistically before committing
3. **US-15.21.6.3** — conditional modifiers on loot tables (player level, difficulty, luck stat)
   - **Acceptance:** loot scales with player progression and game settings

## F-15.21.7 Ability Ledger

| ID           | Persona                 | Features | Requirements |
|--------------|-------------------------|----------|--------------|
| US-15.21.7.1 | designer (P-5)          |          |              |
| US-15.21.7.2 | gameplay director (P-3) |          |              |
| US-15.21.7.3 | designer (P-5)          |          |              |

1. **US-15.21.7.1** — to define abilities with cooldowns, resource costs, targeting rules, effects,
   and combo chains in a dedicated editor
   - **Acceptance:** ability data is structured and validated
2. **US-15.21.7.2** — comparison views showing multiple abilities side by side with costs,
   cooldowns, and effects
   - **Acceptance:** I can evaluate ability balance across the roster
3. **US-15.21.7.3** — each ability entry linked to VFX (F-11.6.1) and animation (F-9.4.1) assets
   - **Acceptance:** ability definitions are self-contained with all associated visual references

## F-15.21.8 Equipment Stat Tables

| ID           | Persona                 | Features | Requirements |
|--------------|-------------------------|----------|--------------|
| US-15.21.8.1 | designer (P-5)          |          |              |
| US-15.21.8.2 | gameplay director (P-3) |          |              |
| US-15.21.8.3 | developer (P-15)        |          |              |

1. **US-15.21.8.1** — to edit equipment stats in a tabular editor with columns for damage, armor,
   slot, set bonus, level requirement, and rarity
   - **Acceptance:** I can author and update hundreds of items efficiently
2. **US-15.21.8.2** — a balance heatmap that highlights stat outliers across the item table
   - **Acceptance:** I can identify over-powered or under-powered equipment at a glance
3. **US-15.21.8.3** — to export equipment stat tables to CSV for external analysis in spreadsheet
   tools
   - **Acceptance:** the data team can run balance models outside the editor

## F-15.21.9 Price Ledger

| ID           | Persona                 | Features | Requirements |
|--------------|-------------------------|----------|--------------|
| US-15.21.9.1 | designer (P-5)          |          |              |
| US-15.21.9.2 | gameplay director (P-3) |          |              |
| US-15.21.9.3 | designer (P-5)          |          |              |

1. **US-15.21.9.1** — to define multiple currency types with exchange rates and conversion matrices
   - **Acceptance:** I can model multi-currency economies
2. **US-15.21.9.2** — to simulate economy flow over time and see price curves over player
   progression
   - **Acceptance:** I can detect inflation, unsustainable sinks, and faucet imbalances
3. **US-15.21.9.3** — a vendor price table editor with buy/sell prices per item per vendor
   - **Acceptance:** I can balance merchant economies across the game world

## F-15.21.10 Visual Effect Graph Editor

| ID            | Persona           | Features | Requirements |
|---------------|-------------------|----------|--------------|
| US-15.21.10.1 | VFX artist (P-12) |          |              |
| US-15.21.10.2 | VFX artist (P-12) |          |              |
| US-15.21.10.3 | designer (P-5)    |          |              |

1. **US-15.21.10.1** — a node-based VFX authoring editor with specialized nodes for particle
   emission, force fields, collision, and rendering
   - **Acceptance:** I can build effects visually without code
2. **US-15.21.10.2** — real-time preview of the effect rendered alongside the graph
   - **Acceptance:** I see the result of parameter changes immediately
3. **US-15.21.10.3** — event trigger nodes in the VFX graph
   - **Acceptance:** effects respond to gameplay events like ability activation, damage, or
     environment changes

## F-15.21.11 Material Graph Editor

| ID            | Persona                  | Features | Requirements |
|---------------|--------------------------|----------|--------------|
| US-15.21.11.1 | tech artist (P-13)       |          |              |
| US-15.21.11.2 | tech artist (P-13)       |          |              |
| US-15.21.11.3 | environment artist (P-8) |          |              |

1. **US-15.21.11.1** — a node-based material authoring editor with nodes for texture sampling, math,
   UV manipulation, and PBR outputs
   - **Acceptance:** I can create materials visually without writing shader code
2. **US-15.21.11.2** — a live material preview rendered on a configurable mesh (sphere, cube,
   custom)
   - **Acceptance:** I can evaluate PBR parameter changes in real time
3. **US-15.21.11.3** — to adjust PBR parameter nodes (roughness, metallic, normal) and see the
   preview update instantly
   - **Acceptance:** I can match reference materials accurately

## F-15.21.12 Custom Logic Graph Nodes

| ID            | Persona                    | Features | Requirements |
|---------------|----------------------------|----------|--------------|
| US-15.21.12.1 | developer (P-15)           |          |              |
| US-15.21.12.2 | designer (P-5)             |          |              |
| US-15.21.12.3 | extension developer (P-25) |          |              |

1. **US-15.21.12.1** — to define custom logic graph node types via the plugin API with inputs,
   outputs, parameter schemas, and execution logic
   - **Acceptance:** I can add domain-specific nodes for my game's systems
2. **US-15.21.12.2** — custom nodes to appear in the graph node palette alongside built-in nodes
   with categories and documentation
   - **Acceptance:** I can use plugin-provided nodes without distinguishing them from engine nodes
3. **US-15.21.12.3** — to provide custom thumbnail icons and documentation strings for my custom
   nodes
   - **Acceptance:** users can identify and understand my nodes visually in the palette
