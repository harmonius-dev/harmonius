# User Stories: Specialized Editors

## F-15.21.1 Entity Editor

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.21.1.1 | designer (P-5) | to select an entity and see all its components displayed with type-appropriate property widgets | I can inspect and edit component data without switching tools |  |  |
| US-15.21.1.2 | designer (P-5) | to add components to an entity via a searchable component palette | I can find and attach the right component type quickly |  |  |
| US-15.21.1.3 | developer (P-15) | to remove components from an entity in the inspector with undo support | I can experiment with different component configurations safely |  |  |
| US-15.21.1.4 | engine developer (P-26) | entity editor changes to integrate with collaborative editing (F-15.12.3) | multiple users can edit different components on the same entity simultaneously |  |  |

## F-15.21.2 Animation Graph Editor

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.21.2.1 | character animator (P-11) | to create a visual graph connecting animation clips through blend nodes and transitions | I can author complex animation state machines without writing code |  |  |
| US-15.21.2.2 | character animator (P-11) | real-time preview of the animation result on a character model with scrubbing, slow motion, and frame stepping | I can evaluate blending quality immediately |  |  |
| US-15.21.2.3 | designer (P-5) | to define transition conditions between animation states using visual expressions | I can control when animations change based on gameplay variables |  |  |
| US-15.21.2.4 | engine developer (P-26) | animation graphs to compile through the logic graph runtime (F-15.8.1) | animation blending uses the same optimized execution pipeline as other graph types |  |  |

## F-15.21.3 Behavior Tree Editor

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.21.3.1 | gameplay director (P-3) | to build AI behavior trees visually with task, condition, decorator, and composite nodes | I can design NPC decision-making without writing code |  |  |
| US-15.21.3.2 | designer (P-5) | execution state overlays showing which behavior tree nodes are active, succeeded, or failed during play mode | I can debug AI behavior by visual inspection |  |  |
| US-15.21.3.3 | developer (P-15) | subtree references for reusable behavior patterns and team-shared behavior libraries | common AI patterns are authored once and reused across multiple NPCs |  |  |

## F-15.21.4 State Machine Editor

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.21.4.1 | gameplay director (P-3) | a general-purpose visual state machine editor with states as nodes and transitions as directed edges with condition expressions | I can model game state logic visually |  |  |
| US-15.21.4.2 | character animator (P-11) | the state machine editor to be reusable for animation state machines | animation and gameplay state machines use the same editing workflow |  |  |
| US-15.21.4.3 | designer (P-5) | hierarchical state machines with sub-machines, parallel regions, and history states | I can model complex systems without a flat state explosion |  |  |

## F-15.21.5 Quest Editor

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.21.5.1 | story director (P-4) | to author quest chains in a visual graph with quest nodes showing objectives, prerequisites, and rewards | I can see the entire quest flow at a glance |  |  |
| US-15.21.5.2 | designer (P-5) | the editor to validate quest graph connectivity and flag unreachable quests | I catch orphaned content before playtesting |  |  |
| US-15.21.5.3 | designer (P-5) | to simulate quest progression in the editor to test prerequisite chains | I can verify quest unlock order without playing through the entire game |  |  |
| US-15.21.5.4 | story writer (P-17) | quest nodes linked to dialogue (F-13.6.4) and localization (F-15.13.1) entries | quest text stays in sync with narrative content |  |  |

## F-15.21.6 Loot Table Editor

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.21.6.1 | designer (P-5) | to configure probability-weighted loot tables with item pools, per-item weights, and quantity ranges | I can control drop rates precisely |  |  |
| US-15.21.6.2 | designer (P-5) | to run thousands of simulation rolls and see distribution histograms | I can verify loot balance statistically before committing |  |  |
| US-15.21.6.3 | gameplay director (P-3) | conditional modifiers on loot tables (player level, difficulty, luck stat) | loot scales with player progression and game settings |  |  |

## F-15.21.7 Ability Ledger

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.21.7.1 | designer (P-5) | to define abilities with cooldowns, resource costs, targeting rules, effects, and combo chains in a dedicated editor | ability data is structured and validated |  |  |
| US-15.21.7.2 | gameplay director (P-3) | comparison views showing multiple abilities side by side with costs, cooldowns, and effects | I can evaluate ability balance across the roster |  |  |
| US-15.21.7.3 | designer (P-5) | each ability entry linked to VFX (F-11.6.1) and animation (F-9.4.1) assets | ability definitions are self-contained with all associated visual references |  |  |

## F-15.21.8 Equipment Stat Tables

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.21.8.1 | designer (P-5) | to edit equipment stats in a tabular editor with columns for damage, armor, slot, set bonus, level requirement, and rarity | I can author and update hundreds of items efficiently |  |  |
| US-15.21.8.2 | gameplay director (P-3) | a balance heatmap that highlights stat outliers across the item table | I can identify over-powered or under-powered equipment at a glance |  |  |
| US-15.21.8.3 | developer (P-15) | to export equipment stat tables to CSV for external analysis in spreadsheet tools | the data team can run balance models outside the editor |  |  |

## F-15.21.9 Price Ledger

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.21.9.1 | designer (P-5) | to define multiple currency types with exchange rates and conversion matrices | I can model multi-currency economies |  |  |
| US-15.21.9.2 | gameplay director (P-3) | to simulate economy flow over time and see price curves over player progression | I can detect inflation, unsustainable sinks, and faucet imbalances |  |  |
| US-15.21.9.3 | designer (P-5) | a vendor price table editor with buy/sell prices per item per vendor | I can balance merchant economies across the game world |  |  |

## F-15.21.10 Visual Effect Graph Editor

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.21.10.1 | VFX artist (P-12) | a node-based VFX authoring editor with specialized nodes for particle emission, force fields, collision, and rendering | I can build effects visually without code |  |  |
| US-15.21.10.2 | VFX artist (P-12) | real-time preview of the effect rendered alongside the graph | I see the result of parameter changes immediately |  |  |
| US-15.21.10.3 | designer (P-5) | event trigger nodes in the VFX graph | effects respond to gameplay events like ability activation, damage, or environment changes |  |  |

## F-15.21.11 Material Graph Editor

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.21.11.1 | tech artist (P-13) | a node-based material authoring editor with nodes for texture sampling, math, UV manipulation, and PBR outputs | I can create materials visually without writing shader code |  |  |
| US-15.21.11.2 | tech artist (P-13) | a live material preview rendered on a configurable mesh (sphere, cube, custom) | I can evaluate PBR parameter changes in real time |  |  |
| US-15.21.11.3 | environment artist (P-8) | to adjust PBR parameter nodes (roughness, metallic, normal) and see the preview update instantly | I can match reference materials accurately |  |  |

## F-15.21.12 Custom Logic Graph Nodes

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.21.12.1 | developer (P-15) | to define custom logic graph node types via the plugin API with inputs, outputs, parameter schemas, and execution logic | I can add domain-specific nodes for my game's systems |  |  |
| US-15.21.12.2 | designer (P-5) | custom nodes to appear in the graph node palette alongside built-in nodes with categories and documentation | I can use plugin-provided nodes without distinguishing them from engine nodes |  |  |
| US-15.21.12.3 | extension developer (P-25) | to provide custom thumbnail icons and documentation strings for my custom nodes | users can identify and understand my nodes visually in the palette |  |  |
