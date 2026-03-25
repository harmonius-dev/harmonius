# User Personas

Personas representing every role in the Harmonius game engine ecosystem. Each persona informs
feature priorities, UI design, documentation tone, and user story authoring. Personas are grouped by
discipline.

---

## Game Development — Creative Leadership

### P-1 Game Executive / Studio Head

- **Role:** Funds and greenlights projects, evaluates engine ROI, manages studio P&L
- **Goals:** Ship profitable games on time, minimize engine licensing costs, reduce team ramp-up
  time, ensure the engine scales from indie prototypes to AAA productions
- **Pain points:** Unpredictable engine costs (per-seat, royalties), vendor lock-in, engine
  limitations discovered late in production, difficulty hiring engineers for niche engines
- **Engine touchpoints:** Launcher (project overview, team management), monetization dashboard,
  build pipeline CI/CD reports, certification compliance checker
- **Key features:** F-15.15.1 (Engine Version Management), F-13.23.3 (IAP Integration), F-15.14.3
  (Certification Checker), F-13.1.10 (Rust Plugin API)

### P-2 Creative Director

- **Role:** Defines the game's artistic vision, tone, and player experience across all disciplines
- **Goals:** Unified visual and narrative identity, rapid iteration on look and feel, ability to
  preview the full game experience without technical barriers
- **Pain points:** Disconnected tools (art vs. design vs. audio), inability to see final quality
  in-editor, slow iteration cycles between creative feedback and implementation
- **Engine touchpoints:** Editor viewport (visual quality preview), material editor, VFX editor,
  cinematic sequencer, photo mode, world-building tools
- **Key features:** F-15.3.1 (Material Graph), F-15.8.5 (Shader Graphs), F-13.5.1 (Sequencer),
  F-13.24.8 (Photo Mode), F-2.12.9 (Custom Materials)

### P-3 Gameplay Director

- **Role:** Designs and balances core gameplay systems — combat, progression, economy, pacing
- **Goals:** Rapid prototyping of gameplay mechanics, data-driven tuning without code changes,
  real-time balance adjustment, cross-system integration (combat + animation + VFX + audio)
- **Pain points:** Gameplay changes requiring programmer support, inability to test balance at
  scale, disconnected tuning spreadsheets from runtime behavior
- **Engine touchpoints:** Logic graph editor, gameplay databases, ability editor, stat tables,
  formula editor, profiler (gameplay timing)
- **Key features:** F-15.8.4 (Gameplay Logic Graphs), F-13.7.1 (Table Schema), F-13.10.1 (Ability
  Composition), F-13.7.5 (Visual Formula Nodes), F-13.12.2 (Talent Trees)

### P-4 Story Director / Narrative Designer

- **Role:** Crafts the game's narrative — main story, side quests, dialogue, lore, worldbuilding
- **Goals:** Author branching narratives without code, preview dialogue with voice acting and camera
  work, manage thousands of localized string keys, track quest dependencies
- **Pain points:** Dialogue tools disconnected from the game world, no in-context preview,
  difficulty managing narrative state across branching paths, localization complexity
- **Engine touchpoints:** Quest/dialogue editor, cinematic sequencer, localization editor, NPC
  simulation tools, logic graph (narrative state machines)
- **Key features:** F-13.6.1 (Quest System), F-13.6.4 (Branching Dialogue), F-15.13.1 (String Table
  Editor), F-13.19.1 (NPC Relationships), F-13.19.5 (Bark System)

---

## Game Development — Design

### P-5 Game Designer

- **Role:** Designs specific gameplay systems, levels, encounters, and player interactions
- **Goals:** Implement game mechanics visually (no code), test ideas quickly, balance difficulty,
  create interesting player choices
- **Pain points:** Needing programmers for simple mechanic changes, slow compile/test cycles, tools
  that don't expose enough control, inability to see system interactions
- **Engine touchpoints:** Logic graph editor, gameplay databases, ability editor, level editor,
  selection system, turn-based grid, fog of war tools
- **Key features:** F-15.8.4 (Logic Graphs), F-13.10.1 (Abilities), F-13.11.4 (Selection Presets),
  F-13.21.1 (Tactical Grid), F-13.20.1 (Fog of War)

### P-6 Level Designer

- **Role:** Builds game levels — layout, flow, pacing, encounter placement, environmental
  storytelling
- **Goals:** Fast terrain/geometry editing, intuitive placement tools, real-time lighting preview,
  playtesting within the editor, navigation mesh visualization
- **Pain points:** Slow lightmap bakes, NavMesh rebuild delays, lack of gameplay preview in editor,
  difficulty managing large open-world zones
- **Engine touchpoints:** Level editor, world-building tools, terrain sculpting, foliage painting,
  NavMesh preview, trigger volume placement, streaming zone management
- **Key features:** F-15.2.1 (Entity Placement), F-15.6.1 (Terrain Sculpting), F-3.2.10 (Hybrid
  Voxel Terrain), F-7.1.9 (Background NavMesh), F-13.2.1 (Level Streaming)

### P-7 Prototyper

- **Role:** Rapidly builds gameplay prototypes to validate ideas before full production
- **Goals:** Minimum time from idea to playable prototype, reuse template systems, no code required,
  easy sharing of prototypes with team for feedback
- **Pain points:** Engine setup overhead, lack of genre templates, needing art assets to test
  gameplay, difficulty estimating production cost from prototype
- **Engine touchpoints:** Launcher (project creation wizard with templates), logic graph,
  placeholder assets, gameplay presets, deploy-to-device for playtesting
- **Key features:** F-15.15.3 (Project Creation Wizard), F-13.1.9 (Modular Systems), F-13.11.4
  (Genre Selection Presets), F-15.14.2 (Deploy-to-Device)

---

## Game Development — Art

### P-8 Environment Artist

- **Role:** Creates world environments — terrain, props, vegetation, architecture, lighting
- **Goals:** High visual quality with performance budgets, seamless DCC-to-engine pipeline,
  real-time preview of materials and lighting, LOD generation without quality loss
- **Pain points:** Texture memory limits, draw call budgets, manual LOD creation, long asset import
  times, material differences between DCC and engine
- **Engine touchpoints:** Material editor, world-building tools, foliage painter, DCC plugin
  (Maya/Blender/Houdini), asset browser, terrain tools, lighting probes
- **Key features:** F-12.6.1 (DCC Plugin SDK), F-15.3.1 (Material Editor), F-3.3.1 (Foliage
  Instancing), F-12.2.2 (LOD Generation), F-2.12.9 (Custom Materials)

### P-9 Character Artist

- **Role:** Creates character models — player characters, NPCs, creatures, equipment
- **Goals:** High-fidelity characters within poly/texture budgets, seamless cloth/hair preview,
  modular equipment system, morph target workflow, MetaHuman-level quality
- **Pain points:** Cloth/hair simulation instability, character LOD artifacts, equipment clipping,
  morph target authoring complexity, slow mesh merging
- **Engine touchpoints:** DCC plugins (ZBrush, Maya, Marvelous Designer), character customization
  preview, equipment socket editor, morph target editor, hair authoring
- **Key features:** F-13.8.1 (Facial Morphing), F-13.8.9 (Modular Mesh Parts), F-12.6.18 (ZBrush
  Plugin), F-12.6.10 (Marvelous Designer), F-2.8.1 (Strand Hair)

### P-10 Rigger

- **Role:** Builds skeleton hierarchies, skin weights, and constraint setups for characters and
  creatures
- **Goals:** Flexible rig definitions that work across retargeting, efficient bone hierarchies for
  GPU skinning, physics-ready joint configurations for ragdoll
- **Pain points:** Rig differences between DCC and engine, IK solver limitations, ragdoll
  instability, retargeting artifacts across body types
- **Engine touchpoints:** Animation editor (skeleton viewer, retargeting setup), DCC plugins (Maya,
  MotionBuilder), ragdoll configuration, IK chain definition
- **Key features:** F-9.1.8 (Retargeting), F-9.3.1 (IK Solvers), F-4.3.5 (Ragdoll), F-9.3.8
  (Multi-Skeleton Locomotion), F-9.3.10 (Attachment/Dismemberment)

### P-11 Character Animator

- **Role:** Creates character animations — locomotion, combat, cinematics, facial expressions
- **Goals:** Responsive animation state machines, smooth blending, animation events for gameplay
  integration, procedural animation for variety, motion matching for quality
- **Pain points:** State machine complexity, foot sliding, animation-gameplay desync, long iteration
  cycles for state machine tuning, lack of procedural variation
- **Engine touchpoints:** Animation editor (timeline, curve editor, state machine, blend space),
  animation preview viewport, DCC plugins (Maya, MotionBuilder), retargeting
- **Key features:** F-9.4.1 (State Machine), F-9.4.8 (Blend Spaces), F-9.3.5 (Foot Placement),
  F-9.1.9 (Animation Events), F-9.4.10 (AI Animation Integration)

### P-12 Effects Artist (VFX Artist)

- **Role:** Creates visual effects — particles, explosions, magic, weather, destruction, UI effects
- **Goals:** Node-based VFX authoring (no code), real-time preview with performance stats,
  event-driven spawning from gameplay, LOD for performance scaling
- **Pain points:** VFX overdraw killing performance, lack of visual authoring tools, difficulty
  triggering effects from gameplay events, no LOD system for effects
- **Engine touchpoints:** VFX effect graph editor, particle system preview, screen effects, decal
  system, weather system, destruction VFX
- **Key features:** F-11.6.1 (Effect Graph Editor), F-11.6.4 (Event-Driven Spawning), F-11.6.5 (VFX
  LOD), F-11.1.1 (GPU Particles), F-11.5.1 (Debris Spawning)

### P-13 Technical Artist

- **Role:** Bridges art and engineering — shader authoring, pipeline optimization, tool scripting,
  performance profiling
- **Goals:** Custom shader creation without code, automated asset pipelines, performance analysis
  per asset, pipeline tools for batch operations
- **Pain points:** Shader debugging, performance regression from art changes, manual pipeline steps,
  lack of profiling tools accessible to non-programmers
- **Engine touchpoints:** Shader graph editor, material editor, profiler, asset processing pipeline,
  logic graph (tool scripting), Houdini integration
- **Key features:** F-15.8.5 (Shader Graphs), F-15.5.1 (CPU Profiler), F-15.5.3 (GPU Profiler),
  F-12.6.3 (Houdini Plugin), F-15.8.9 (Tool Automation Graphs)

---

## Game Development — Audio

### P-14 Audio Designer / Sound Designer

- **Role:** Creates and implements all game audio — sound effects, ambient, music, voice
- **Goals:** Spatial audio that matches the visual environment, adaptive music that responds to
  gameplay, efficient voice management for hundreds of simultaneous sounds
- **Pain points:** Audio occlusion not matching visuals, music transitions that feel abrupt, voice
  management in crowded scenes, lack of real-time audio debugging
- **Engine touchpoints:** Audio mixing graph, spatial audio tools, music system, DSP effects, DCC
  plugins (Substance for foley, audio middleware concepts)
- **Key features:** F-5.1.3 (Mixer Bus), F-5.2.5 (Occlusion), F-5.4.1 (Adaptive Music), F-5.3.8
  (Custom DSP Chains), F-15.8.8 (Audio Mixing Graphs)

---

## Game Development — Engine Engineering

### P-26 Game Engine Developer

- **Role:** Develops and maintains the Harmonius engine itself — core systems, rendering pipeline,
  physics, ECS, platform backends, and editor infrastructure
- **Goals:** Clean modular architecture, fast iteration (compile + test under 30 seconds),
  comprehensive test coverage, clear API boundaries between subsystems, documentation that stays in
  sync with code
- **Pain points:** Cross-platform debugging (Metal vs Vulkan vs D3D12), unsafe FFI boundaries
  (C ABI, `extern "C"`), ECS scheduling complexity, performance regressions from seemingly small
  changes, maintaining backward compatibility for plugins
- **Engine touchpoints:** All source code, GPU backend implementations, ECS internals, render graph
  compiler, physics solver, asset pipeline, platform abstraction layers, CI/CD pipeline, profiler,
  test framework
- **Key features:** F-1.1.1 (ECS Core), F-2.1.1 (GPU Backend Trait), F-2.2.1 (Render Graph), F-4.1.1
  (Physics Integration), F-1.8.1 (Async I/O), F-14.3.1 (Thread Pool), F-15.18.7 (Test Runner
  Infrastructure)

### P-27 Game Engine Tester / Engine QA

- **Role:** Tests the engine itself (not a specific game) — verifies that engine systems work
  correctly across all platforms, performs regression testing, validates performance budgets, and
  catches cross-system interaction bugs
- **Goals:** Automated test suites for every engine subsystem, screenshot comparison tests for
  rendering correctness, performance benchmark baselines, cross-platform CI that catches
  platform-specific failures, fuzzing for crash resilience
- **Pain points:** Combinatorial explosion of platform × GPU × feature configurations, flaky
  GPU-dependent tests, difficulty reproducing reported crashes, no standard for "correct" rendering
  output across GPU vendors, long full-test-suite run times
- **Engine touchpoints:** Test runner infrastructure (F-15.18.7), CI/CD pipeline (F-15.18.6),
  profiler (F-15.5.1), crash reporting (F-14.4.1), input recording (F-6.2.7), replay system
  (F-8.6.1), screenshot comparison, platform deploy-to-device (F-15.14.2)
- **Key features:** F-15.18.7 (Test Runners), F-15.18.6 (CI/CD Pipeline), F-6.2.7 (Input Recording),
  F-14.4.1 (Crash Handler), F-15.5.1 (Profiler), F-15.14.2 (Deploy-to-Device)

---

## Game Development — Gameplay Engineering

### P-15 Game Developer / Gameplay Programmer

- **Role:** Implements gameplay systems, game-specific features, and custom engine extensions in
  Rust
- **Goals:** Clean ECS API, type-safe component access, fast compile times, hot reload for
  iteration, access to all engine subsystems through the plugin API
- **Pain points:** Opaque engine internals, long rebuild times, ABI instability across engine
  versions, difficulty debugging cross-system interactions
- **Engine touchpoints:** Rust plugin API, ECS world access, engine source code, profiler, debugger,
  CI/CD pipeline
- **Key features:** F-13.1.10 (Rust Plugin API), F-1.1.1 (ECS), F-1.6.1 (Plugin System), F-15.5.1
  (CPU Profiler), F-12.4.4 (Logic Graph Hot Reload)

### P-16 DevOps / Build Engineer

- **Role:** Manages CI/CD pipelines, build infrastructure, deployment, and automated testing
- **Goals:** Reproducible builds, automated testing, fast CI turnaround, multi-platform packaging,
  store submission automation
- **Pain points:** Platform-specific build failures, long asset cooking times, flaky tests, store
  submission rejected for certification issues
- **Engine touchpoints:** CLI build tools, shared build cache, certification checker, store
  submission pipeline, crash aggregation dashboard, automated testing
- **Key features:** F-15.14.1 (Platform Packaging), F-15.14.8 (Store Distribution), F-15.11.1
  (Shared Build Cache), F-15.14.3 (Certification Checker), F-15.11.7 (CI Population)

---

## Game Development — Writing and Localization

### P-17 Story Writer

- **Role:** Writes dialogue, quest text, item descriptions, lore entries, and narrative scripts
- **Goals:** Write directly in the engine's dialogue editor, see text in context with character
  models and environments, manage branching paths visually, collaborate with voice actors
- **Pain points:** Writing in external documents disconnected from the game, text overflow in UI
  elements, localization breaking text flow, no preview of text with voice acting
- **Engine touchpoints:** Dialogue editor, quest editor, string table editor, localization preview,
  NPC bark editor, cutscene sequencer
- **Key features:** F-13.6.4 (Branching Dialogue), F-15.13.1 (String Table Editor), F-15.13.2
  (Localization Preview), F-13.19.5 (Bark System), F-13.5.1 (Sequencer)

### P-18 Localization Specialist

- **Role:** Manages translation workflows, validates localized content, ensures cultural
  appropriateness
- **Goals:** Efficient string management, pseudo-localization for testing, RTL support,
  platform-specific localization compliance, translation memory
- **Pain points:** Missing translations blocking builds, text overflow in localized UI, no context
  for translators, RTL layout bugs
- **Engine touchpoints:** Localization editor, string table, translation workflow integration,
  pseudo-localization mode, localization validation report
- **Key features:** F-15.13.1 (String Tables), F-15.13.2 (Preview/Validation), F-15.13.3
  (Translation Workflow), F-10.1.9 (Localization Hooks)

---

## Game Development — QA and Testing

### P-19 Game Tester / QA Engineer

- **Role:** Tests game builds for bugs, performance issues, compliance failures, and user experience
  problems
- **Goals:** Reproducible bug reports with input recordings, automated regression testing,
  performance benchmarks, platform-specific testing tools
- **Pain points:** Non-reproducible bugs, no automated visual regression, slow test iteration,
  difficulty testing networked gameplay, accessibility testing tools
- **Engine touchpoints:** Input recording/playback, crash reports, profiler, automated test
  framework, deploy-to-device, replay system, accessibility checker
- **Key features:** F-6.2.7 (Input Recording), F-14.4.1 (Crash Handler), F-15.5.1 (Profiler),
  F-8.6.1 (Replay Recording), F-10.6.7 (WCAG Compliance)

### P-20 QA Lead

- **Role:** Plans test strategies, manages QA team, tracks bug metrics, signs off on release
  readiness
- **Goals:** Test coverage dashboards, automated smoke tests per build, certification pre-check,
  crash trend analysis, performance regression detection
- **Pain points:** Manual test overhead, no visibility into automated test coverage, late discovery
  of certification blockers
- **Engine touchpoints:** Crash aggregation dashboard, certification checker, CI test reports,
  profiler benchmarks, build pipeline status
- **Key features:** F-15.14.3 (Certification Checker), F-14.4.3 (Crash Aggregation), F-15.11.7 (CI
  Population), F-15.5.6 (Stat Overlays)

---

## Game Development — Production and Management

### P-21 Community Manager

- **Role:** Manages player community, moderates user-generated content, handles player feedback
- **Goals:** Mod moderation tools, player reporting dashboard, chat moderation, community content
  curation, event management
- **Pain points:** Toxic content in mods, no tools for reviewing UGC, difficulty communicating with
  players about issues, manual moderation at scale
- **Engine touchpoints:** Mod moderation dashboard, chat channel management, player reporting,
  in-game event tools
- **Key features:** F-15.16.6 (Mod Moderation), F-5.5.8 (Voice Chat Channels), F-8.8.5 (Rate
  Limiting), F-13.23.2 (Daily/Weekly Challenges)

### P-22 Server Administrator

- **Role:** Operates live game servers — deployment, scaling, monitoring, incident response
- **Goals:** Auto-scaling server clusters, real-time monitoring dashboards, graceful degradation
  under load, zero-downtime updates, anti-cheat enforcement
- **Pain points:** Server crashes without diagnostics, manual scaling, no visibility into player
  distribution, inability to hotfix without downtime
- **Engine touchpoints:** Server deployment tools, MMO infrastructure management, monitoring/
  telemetry, anti-cheat dashboard, load balancer configuration
- **Key features:** F-8.7.6 (Auto-Scaling), F-8.7.3 (Dynamic Shard Splitting), F-8.8.1 (Server-Side
  Cheat Detection), F-14.4.3 (Crash Aggregation), F-3.6.62 (Server-Side Universe Generation)

---

## End Users

### P-23 Game Player

- **Role:** Plays the shipped game
- **Goals:** Fun, responsive gameplay, stable performance, fair multiplayer, accessible controls,
  mod support for extended content
- **Pain points:** Lag, crashes, unfair cheaters, inaccessible controls, long load times, pay-to-win
  monetization, lack of mod support
- **Engine touchpoints:** Runtime only — all engine features manifest as gameplay quality,
  performance, stability, and accessibility
- **Key features:** F-4.1.8 (Character Controller responsiveness), F-8.8.1 (Anti-Cheat), F-10.6.1
  (Screen Reader), F-15.16.5 (Mod Workshop), F-13.24.4 (Game Feel)

### P-24 Modder

- **Role:** Creates community content — new levels, items, gameplay mechanics, total conversions
- **Goals:** Access to powerful creation tools with safety constraints, easy mod distribution,
  community feedback on published mods, ability to monetize popular mods
- **Pain points:** Limited modding tools, no documentation, mod conflicts, breaking changes between
  game updates, no moderation feedback loop
- **Engine touchpoints:** Mod SDK (subset of editor), mod packaging tools, Steam Workshop, mod
  browser, constraint system
- **Key features:** F-15.16.1 (Mod SDK), F-15.16.2 (Mod Constraints), F-15.16.5 (Workshop
  Integration), F-15.16.4 (Sandboxed Loading)

---

## Ecosystem — Third-Party Developers

### P-25 Extension / Plugin / Asset Store Developer

- **Role:** Builds and sells reusable engine extensions, plugins, assets, and tools through the
  asset marketplace
- **Goals:** Stable plugin API across engine versions, large addressable market, fair revenue
  sharing, automated compatibility testing, discoverability in the store
- **Pain points:** ABI breaks between engine versions, small market, high store commission, piracy,
  no automated testing against new engine versions, support burden
- **Engine touchpoints:** Rust plugin API, asset store publisher dashboard, compatibility testing
  tools, documentation/API reference, store listing management
- **Key features:** F-13.1.10 (Rust Plugin API), F-15.17.x (Asset Store — to be added), F-15.15.2
  (Automatic Project Upgrades), F-12.7.1 (Binary Asset Format)
