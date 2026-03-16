# User Stories: Specialized Editors

## F-15.21.1 Entity Editor

## US-15.21.1.1 Designer Inspects Entity Components

**As a** designer (P-5), **I want** to select an entity and see all its components displayed with
type-appropriate property widgets, **so that** I can inspect and edit component data without
switching tools.

## US-15.21.1.2 Designer Adds Components via Palette

**As a** designer (P-5), **I want** to add components to an entity via a searchable component
palette, **so that** I can find and attach the right component type quickly.

## US-15.21.1.3 Developer Removes Components

**As a** developer (P-15), **I want** to remove components from an entity in the inspector with undo
support, **so that** I can experiment with different component configurations safely.

## US-15.21.1.4 Engine Dev Integrates with Collaboration

**As an** engine developer (P-26), **I want** entity editor changes to integrate with collaborative
editing (F-15.12.3), **so that** multiple users can edit different components on the same entity
simultaneously.

## F-15.21.2 Animation Graph Editor

## US-15.21.2.1 Animator Creates Blend Graph

**As a** character animator (P-11), **I want** to create a visual graph connecting animation clips
through blend nodes and transitions, **so that** I can author complex animation state machines
without writing code.

## US-15.21.2.2 Animator Previews in Real Time

**As a** character animator (P-11), **I want** real-time preview of the animation result on a
character model with scrubbing, slow motion, and frame stepping, **so that** I can evaluate blending
quality immediately.

## US-15.21.2.3 Designer Edits Transition Conditions

**As a** designer (P-5), **I want** to define transition conditions between animation states using
visual expressions, **so that** I can control when animations change based on gameplay variables.

## US-15.21.2.4 Engine Dev Validates Graph Compilation

**As an** engine developer (P-26), **I want** animation graphs to compile through the logic graph
runtime (F-15.8.1), **so that** animation blending uses the same optimized execution pipeline as
other graph types.

## F-15.21.3 Behavior Tree Editor

## US-15.21.3.1 Designer Builds AI Behavior

**As a** gameplay director (P-3), **I want** to build AI behavior trees visually with task,
condition, decorator, and composite nodes, **so that** I can design NPC decision-making without
writing code.

## US-15.21.3.2 Designer Debugs Live Behavior

**As a** designer (P-5), **I want** execution state overlays showing which behavior tree nodes are
active, succeeded, or failed during play mode, **so that** I can debug AI behavior by visual
inspection.

## US-15.21.3.3 Developer Reuses Subtree References

**As a** developer (P-15), **I want** subtree references for reusable behavior patterns and
team-shared behavior libraries, **so that** common AI patterns are authored once and reused across
multiple NPCs.

## F-15.21.4 State Machine Editor

## US-15.21.4.1 Gameplay Director Creates State Machine

**As a** gameplay director (P-3), **I want** a general-purpose visual state machine editor with
states as nodes and transitions as directed edges with condition expressions, **so that** I can
model game state logic visually.

## US-15.21.4.2 Animator Reuses for Animation States

**As a** character animator (P-11), **I want** the state machine editor to be reusable for animation
state machines, **so that** animation and gameplay state machines use the same editing workflow.

## US-15.21.4.3 Designer Nests Hierarchical States

**As a** designer (P-5), **I want** hierarchical state machines with sub-machines, parallel regions,
and history states, **so that** I can model complex systems without a flat state explosion.

## F-15.21.5 Quest Editor

## US-15.21.5.1 Story Director Authors Quest Chain

**As a** story director (P-4), **I want** to author quest chains in a visual graph with quest nodes
showing objectives, prerequisites, and rewards, **so that** I can see the entire quest flow at a
glance.

## US-15.21.5.2 Designer Validates Quest Graph

**As a** designer (P-5), **I want** the editor to validate quest graph connectivity and flag
unreachable quests, **so that** I catch orphaned content before playtesting.

## US-15.21.5.3 Designer Simulates Progression

**As a** designer (P-5), **I want** to simulate quest progression in the editor to test prerequisite
chains, **so that** I can verify quest unlock order without playing through the entire game.

## US-15.21.5.4 Story Writer Links Dialogue

**As a** story writer (P-17), **I want** quest nodes linked to dialogue (F-13.6.4) and localization
(F-15.13.1) entries, **so that** quest text stays in sync with narrative content.

## F-15.21.6 Loot Table Editor

## US-15.21.6.1 Designer Configures Drop Weights

**As a** designer (P-5), **I want** to configure probability-weighted loot tables with item pools,
per-item weights, and quantity ranges, **so that** I can control drop rates precisely.

## US-15.21.6.2 Designer Simulates Drops

**As a** designer (P-5), **I want** to run thousands of simulation rolls and see distribution
histograms, **so that** I can verify loot balance statistically before committing.

## US-15.21.6.3 Gameplay Director Adds Conditional Modifiers

**As a** gameplay director (P-3), **I want** conditional modifiers on loot tables (player level,
difficulty, luck stat), **so that** loot scales with player progression and game settings.

## F-15.21.7 Ability Ledger

## US-15.21.7.1 Designer Defines Abilities

**As a** designer (P-5), **I want** to define abilities with cooldowns, resource costs, targeting
rules, effects, and combo chains in a dedicated editor, **so that** ability data is structured and
validated.

## US-15.21.7.2 Gameplay Director Compares Abilities

**As a** gameplay director (P-3), **I want** comparison views showing multiple abilities side by
side with costs, cooldowns, and effects, **so that** I can evaluate ability balance across the
roster.

## US-15.21.7.3 Designer Links VFX and Animation

**As a** designer (P-5), **I want** each ability entry linked to VFX (F-11.6.1) and animation
(F-9.4.1) assets, **so that** ability definitions are self-contained with all associated visual
references.

## F-15.21.8 Equipment Stat Tables

## US-15.21.8.1 Designer Edits Item Stats

**As a** designer (P-5), **I want** to edit equipment stats in a tabular editor with columns for
damage, armor, slot, set bonus, level requirement, and rarity, **so that** I can author and update
hundreds of items efficiently.

## US-15.21.8.2 Gameplay Director Spots Outliers

**As a** gameplay director (P-3), **I want** a balance heatmap that highlights stat outliers across
the item table, **so that** I can identify over-powered or under-powered equipment at a glance.

## US-15.21.8.3 Developer Exports to CSV

**As a** developer (P-15), **I want** to export equipment stat tables to CSV for external analysis
in spreadsheet tools, **so that** the data team can run balance models outside the editor.

## F-15.21.9 Price Ledger

## US-15.21.9.1 Designer Defines Currencies

**As a** designer (P-5), **I want** to define multiple currency types with exchange rates and
conversion matrices, **so that** I can model multi-currency economies.

## US-15.21.9.2 Gameplay Director Simulates Economy

**As a** gameplay director (P-3), **I want** to simulate economy flow over time and see price curves
over player progression, **so that** I can detect inflation, unsustainable sinks, and faucet
imbalances.

## US-15.21.9.3 Designer Sets Vendor Prices

**As a** designer (P-5), **I want** a vendor price table editor with buy/sell prices per item per
vendor, **so that** I can balance merchant economies across the game world.

## F-15.21.10 Visual Effect Graph Editor

## US-15.21.10.1 VFX Artist Authors Effects Visually

**As a** VFX artist (P-12), **I want** a node-based VFX authoring editor with specialized nodes for
particle emission, force fields, collision, and rendering, **so that** I can build effects visually
without code.

## US-15.21.10.2 VFX Artist Previews in Real Time

**As a** VFX artist (P-12), **I want** real-time preview of the effect rendered alongside the graph,
**so that** I see the result of parameter changes immediately.

## US-15.21.10.3 Designer Triggers Effects from Events

**As a** designer (P-5), **I want** event trigger nodes in the VFX graph, **so that** effects
respond to gameplay events like ability activation, damage, or environment changes.

## F-15.21.11 Material Graph Editor

## US-15.21.11.1 Tech Artist Authors Materials Visually

**As a** tech artist (P-13), **I want** a node-based material authoring editor with nodes for
texture sampling, math, UV manipulation, and PBR outputs, **so that** I can create materials
visually without writing shader code.

## US-15.21.11.2 Tech Artist Previews on Mesh

**As a** tech artist (P-13), **I want** a live material preview rendered on a configurable mesh
(sphere, cube, custom), **so that** I can evaluate PBR parameter changes in real time.

## US-15.21.11.3 Artist Adjusts PBR Parameters

**As an** environment artist (P-8), **I want** to adjust PBR parameter nodes (roughness, metallic,
normal) and see the preview update instantly, **so that** I can match reference materials
accurately.

## F-15.21.12 Custom Logic Graph Nodes

## US-15.21.12.1 Developer Defines Custom Nodes

**As a** developer (P-15), **I want** to define custom logic graph node types via the plugin API
with inputs, outputs, parameter schemas, and execution logic, **so that** I can add domain-specific
nodes for my game's systems.

## US-15.21.12.2 Designer Uses Custom Nodes in Graph

**As a** designer (P-5), **I want** custom nodes to appear in the graph node palette alongside
built-in nodes with categories and documentation, **so that** I can use plugin-provided nodes
without distinguishing them from engine nodes.

## US-15.21.12.3 Extension Dev Provides Thumbnails

**As an** extension developer (P-25), **I want** to provide custom thumbnail icons and documentation
strings for my custom nodes, **so that** users can identify and understand my nodes visually in the
palette.
