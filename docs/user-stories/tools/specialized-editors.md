# User Stories -- 15.21 Specialized Editors

## Stories

| ID            | Persona                 |
|---------------|-------------------------|
| US-15.21.1.1  | game designer (P-5)     |
| US-15.21.1.2  | engine developer (P-26) |
| US-15.21.2.1  | technical artist (P-13) |
| US-15.21.2.2  | game designer (P-5)     |
| US-15.21.3.1  | game designer (P-5)     |
| US-15.21.3.2  | engine developer (P-26) |
| US-15.21.4.1  | game designer (P-5)     |
| US-15.21.4.2  | technical artist (P-13) |
| US-15.21.5.1  | game designer (P-5)     |
| US-15.21.5.2  | level designer (P-6)    |
| US-15.21.6.1  | game designer (P-5)     |
| US-15.21.6.2  | game designer (P-5)     |
| US-15.21.7.1  | game designer (P-5)     |
| US-15.21.7.2  | game designer (P-5)     |
| US-15.21.8.1  | game designer (P-5)     |
| US-15.21.8.2  | technical artist (P-13) |
| US-15.21.9.1  | game designer (P-5)     |
| US-15.21.9.2  | game designer (P-5)     |
| US-15.21.10.1 | technical artist (P-13) |
| US-15.21.10.2 | game designer (P-5)     |
| US-15.21.11.1 | technical artist (P-13) |
| US-15.21.11.2 | game designer (P-5)     |
| US-15.21.12.1 | extension developer (P-25) |
| US-15.21.12.2 | engine developer (P-26) |

1. **US-15.21.1.1** — **As a** game designer (P-5), **I want** an entity inspector displaying all
   components with a searchable component palette, **so that** I can edit entities visually.

2. **US-15.21.1.2** — **As a** engine developer (P-26), **I want** the entity editor to integrate
   with undo/redo and CRDT collaboration, **so that** edits are safe and sharable.

3. **US-15.21.2.1** — **As a** technical artist (P-13), **I want** a visual animation graph editor
   with real-time preview on a character model, **so that** I author blend trees and state machines
   visually.

4. **US-15.21.2.2** — **As a** game designer (P-5), **I want** scrubbing, slow motion, and frame
   stepping in the animation preview, **so that** I can inspect motion quality.

5. **US-15.21.3.1** — **As a** game designer (P-5), **I want** a behavior tree editor with execution
   state overlays, **so that** I can see which nodes are active during play mode.

6. **US-15.21.3.2** — **As a** engine developer (P-26), **I want** subtree references for reusable
   behavior patterns, **so that** AI libraries are composable.

7. **US-15.21.4.1** — **As a** game designer (P-5), **I want** a general-purpose state machine
   editor reused across animation, AI, gameplay, and UI, **so that** all state machines have a
   consistent editing experience.

8. **US-15.21.4.2** — **As a** technical artist (P-13), **I want** hierarchical state machines with
   parallel regions and history states, **so that** complex behaviors are expressible.

9. **US-15.21.5.1** — **As a** game designer (P-5), **I want** a visual quest graph editor with
   objectives, prerequisites, and reward summaries, **so that** I author quest chains visually.

10. **US-15.21.5.2** — **As a** level designer (P-6), **I want** quest graph validation flagging
    unreachable quests, **so that** I catch design errors early.

11. **US-15.21.6.1** — **As a** game designer (P-5), **I want** a loot table editor with probability
    weights and drop simulation histograms, **so that** I can balance loot distribution.

12. **US-15.21.6.2** — **As a** game designer (P-5), **I want** nested sub-tables for tiered loot,
    **so that** drop pools are composable.

13. **US-15.21.7.1** — **As a** game designer (P-5), **I want** an ability definition editor with
    cooldowns, resource costs, targeting rules, and combo chains, **so that** I author abilities
    visually.

14. **US-15.21.7.2** — **As a** game designer (P-5), **I want** comparison views for ability balance
    review, **so that** outliers are visible at a glance.

15. **US-15.21.8.1** — **As a** game designer (P-5), **I want** an equipment stat editor with
    balance heatmaps, **so that** I can spot stat outliers across item tiers.

16. **US-15.21.8.2** — **As a** technical artist (P-13), **I want** CSV export for external
    analysis, **so that** I can run custom balance simulations.

17. **US-15.21.9.1** — **As a** game designer (P-5), **I want** an economy editor with price curves
    and inflation modeling, **so that** I can balance currency flows over progression.

18. **US-15.21.9.2** — **As a** game designer (P-5), **I want** the economy editor to flag
    unsustainable sinks or faucets, **so that** economic imbalances are caught early.

19. **US-15.21.10.1** — **As a** technical artist (P-13), **I want** a node-based Effect Graph
    editor with real-time VFX preview, **so that** I author particle effects visually.

20. **US-15.21.10.2** — **As a** game designer (P-5), **I want** event trigger nodes in the Effect
    Graph editor, **so that** effects respond to gameplay events.

21. **US-15.21.11.1** — **As a** technical artist (P-13), **I want** a node-based material graph
    editor with live preview on configurable meshes, **so that** I create and iterate on materials
    visually.

22. **US-15.21.11.2** — **As a** game designer (P-5), **I want** PBR output nodes (albedo, normal,
    roughness, metallic), **so that** materials integrate with the rendering pipeline.

23. **US-15.21.12.1** — **As a** extension developer (P-25), **I want** an API for extending the
    logic graph with custom node types via the plugin system, **so that** my nodes appear alongside
    built-in ones.

24. **US-15.21.12.2** — **As a** engine developer (P-26), **I want** custom nodes to participate in
    graph compilation and debugging, **so that** they are first-class citizens.

## Parent Stories

The 3-segment parent stories below are umbrella rollups for the refined 4-segment sub-stories listed
above. Each parent inherits the persona of its first sub-story and describes the umbrella capability
that the sub-stories refine.

| ID | Persona |
|----|---------|
| US-15.21.1 | game designer (P-5) |
| US-15.21.10 | technical artist (P-13) |
| US-15.21.11 | technical artist (P-13) |
| US-15.21.12 | extension developer (P-25) |
| US-15.21.2 | technical artist (P-13) |
| US-15.21.3 | game designer (P-5) |
| US-15.21.4 | game designer (P-5) |
| US-15.21.5 | game designer (P-5) |
| US-15.21.6 | game designer (P-5) |
| US-15.21.7 | game designer (P-5) |
| US-15.21.8 | game designer (P-5) |
| US-15.21.9 | game designer (P-5) |

1. **US-15.21.1** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-15.21.1.1 through US-15.21.1.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

2. **US-15.21.10** -- **As a** technical artist (P-13), **I want** the capabilities defined in
   sub-stories US-15.21.10.1 through US-15.21.10.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

3. **US-15.21.11** -- **As a** technical artist (P-13), **I want** the capabilities defined in
   sub-stories US-15.21.11.1 through US-15.21.11.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

4. **US-15.21.12** -- **As a** extension developer (P-25), **I want** the capabilities defined in
   sub-stories US-15.21.12.1 through US-15.21.12.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

5. **US-15.21.2** -- **As a** technical artist (P-13), **I want** the capabilities defined in
   sub-stories US-15.21.2.1 through US-15.21.2.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

6. **US-15.21.3** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-15.21.3.1 through US-15.21.3.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

7. **US-15.21.4** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-15.21.4.1 through US-15.21.4.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

8. **US-15.21.5** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-15.21.5.1 through US-15.21.5.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

9. **US-15.21.6** -- **As a** game designer (P-5), **I want** the capabilities defined in
   sub-stories US-15.21.6.1 through US-15.21.6.2 combined into a single umbrella feature,
   **so that** I have a coherent parent story covering the refined child stories.

10. **US-15.21.7** -- **As a** game designer (P-5), **I want** the capabilities defined in
    sub-stories
US-15.21.7.1 through US-15.21.7.2 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

11. **US-15.21.8** -- **As a** game designer (P-5), **I want** the capabilities defined in
    sub-stories
US-15.21.8.1 through US-15.21.8.2 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

12. **US-15.21.9** -- **As a** game designer (P-5), **I want** the capabilities defined in
    sub-stories
US-15.21.9.1 through US-15.21.9.2 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.
