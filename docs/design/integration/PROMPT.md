# Integration Design — Prompt

Use this prompt verbatim to start a new Claude Code conversation in plan mode. Enter plan mode
first, then paste this prompt.

---

Create integration design documents for the Harmonius game engine. All integration designs go in
`docs/design/integration/`.

## Phase 1: Audit and discover all integration pairs

Read every design document in `docs/design/` (55 files across 17 domains). For each, find the
"Cross-subsystem integration" or "Cross-Cutting Dependencies" table. Collect every integration pair
mentioned.

Then identify UNLISTED pairs that logically need integration. Consider every combination of:

- Core Runtime: ECS, scene/transforms, events/plugins, spatial index, game loop, memory/async-io,
  algorithms
- Platform: threading, windowing, platform-services
- Data Systems: directed graphs, data tables, attributes/effects, containers/slots
- Simulation: grids/volumes, spatial awareness, timelines, event logs
- Domain: rendering (core, pipeline, effects, styles, 2d), animation (skeletal, procedural, state
  machine), vfx (effects, particles), ai (behavior, navigation, steering-crowds), physics
  (foundation, advanced, constraints), audio, content-pipeline (asset-pipeline, asset-processing),
  input, networking (transport, services, infrastructure), geometry (world, procedural)
- Game Framework: camera, save system, scripting (logic graph system)
- UI: ui-framework
- Tools: editor-core, visual-editors, level-world, profiler, team-tools, build-deploy,
  content-services

Unlisted pairs to especially consider:

- Audio to Physics (collision sounds, material-based impact audio)
- Audio to Spatial awareness (obstruction/occlusion queries)
- Audio to Camera (listener position)
- Networking to Save system (server-authoritative saves)
- Networking to ECS (state replication, delta compression)
- Input to UI (focus management, gestures, gamepad navigation)
- Input to Camera (mouse look, gamepad orbit, VR head tracking)
- Animation to Physics (ragdoll transition, cloth, soft body)
- Animation to VFX (particle spawn on animation events)
- Animation to Audio (footstep sounds synchronized to walk cycle)
- Rendering to Physics (debug visualization of colliders, BVH)
- Rendering to UI (render graph passes, viewport occlusion by HUD)
- Rendering to VFX (render graph compute passes, froxel injection)
- AI to Navigation (pathfinding, flow fields, steering)
- AI to Spatial awareness (perception, threat assessment)
- AI to Data tables (NPC behavior data, ability definitions)
- AI to Event logs (NPC memory of witnessed events)
- Content pipeline to Build system (asset baking per platform)
- Content pipeline to Rendering (shader compilation, texture formats)
- ECS to Scripting (codegen'd systems, component read/write)
- Scripting to Rendering (custom shaders from logic graphs)
- Scripting to Data tables (formula columns as logic graphs)
- Timelines to Animation (property animation, cutscene playback)
- Timelines to Camera (cinematic camera tracks)
- Timelines to Audio (music cues, voice-over timing)
- Timelines to Dialogue (subtitle synchronization)
- Grids/Volumes to Rendering (fog of war, voxel GI, SDF shadows)
- Grids/Volumes to Physics (heightmap collision, voxel collision)
- Grids/Volumes to AI (influence maps, flow fields)
- Save system to every subsystem (what components are saveable)
- Editor to every subsystem (property inspection, debug viz)
- Localization to UI, dialogue, data tables, subtitles, editor

## Phase 2: Create high-level integration design FIRST

Create `docs/design/integration/high-level.md` — the architectural glue document. This shows:

1. All 15+ subsystems in a single Mermaid diagram with data flow edges
2. Per-frame data flow: which subsystem produces what data at which game loop phase, and which
   subsystem consumes it
3. Thread ownership map: which thread owns each subsystem's data
4. Frame-boundary handoff points between simulation and render threads
5. The codegen/middleman .dylib as the compilation surface connecting all subsystems
6. Performance budget allocation across subsystems per frame

## Phase 3: Create per-pair integration design documents

For each identified pair, create a document using the document-author agent (one subagent per
document). Each document:

- File: `docs/design/integration/<subsystem-a>-<subsystem-b>.md`
- Sections: Overview, Data exchanged (types/formats), Direction, Mechanism (ECS query, event
  channel, render graph resource, etc.), Game loop phase, Frame-boundary handoff, Thread ownership,
  Error handling, Performance budget, Test cases
- Cross-reference the two parent design documents
- Follow all rules in `docs/design/CLAUDE.md`

## Phase 4: Commit and push

After all integration documents are created, git add all new files, commit with a descriptive
message, and push.

## Key constraints (from docs/design/constraints.md)

- Everything compiles to Rust (visual graphs codegen .rs source)
- Zero reflection (codegen via middleman .dylib)
- No async/await in engine (sync + platform-native I/O)
- rkyv for binary serialization (no serde, no RON, no BSN)
- Custom job system (crossbeam-deque, no Rayon, no Tokio)
- QUIC for all networking (no TCP, no HTTPS)
- Localization is core-runtime (LocalizedStringId everywhere)
- No server-side proprietary (all services OSS)
- Case analysis (match) not conditional (if/else)
- Assets on disk, code in binary (include_bytes! only for inline table/graph data)
- Customer-owned AI API keys (engine is thin client)
