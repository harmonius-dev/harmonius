# Harmonius Go-to-Market Strategy

Strategic plan for bringing the Harmonius game engine from specification to market. Written for a
solo founder with AI-augmented development capacity (Claude). The engine has 1,204 features
specified across 15 domains and 1,501 requirements. No code has been written. No revenue.

## 1. Development Priorities

Development is organized as a dependency DAG of 87 feature-groups across 15 domains, detailed in
[domain-decomposition.md](domain-decomposition.md). The DAG replaces the traditional sequential tier
model with a structure that maximizes parallel agent development pipelines.

### Priority Principles

1. **Maximize parallelism:** Build foundational features across all domains as early as possible to
   unblock the most concurrent development pipelines.
2. **Preserve interoperability:** Lock down cross-domain contracts (component schemas, event types,
   API traits) before multiple agents build against them.
3. **DAG-driven ordering:** The build order emerges from the dependency graph. Feature-groups are
   built when their prerequisites are satisfied, not when their domain's "turn" arrives.
4. **Iterate foundations early:** Foundational products are built first so that future use cases can
   inform changes before dependent systems are too entrenched.

### Wave Summary

The DAG's topological sort produces 7 concurrency waves. Each wave contains all feature-groups whose
prerequisites are satisfied by completion of prior waves.

| Wave | Nodes | Max Concurrent Agents | Cumulative Complete |
|------|-------|-----------------------|---------------------|
| 0    | 3     | 3                     | 3                   |
| 1    | 6     | 6                     | 9                   |
| 2    | 9     | 9                     | 18                  |
| 3    | 12    | 12                    | 30                  |
| 4    | 19    | 19                    | 49                  |
| 5    | 20    | 20                    | 69                  |
| 6    | 18    | 18                    | 87                  |

1. **0** — Platform root (windowing, threading, OS)
2. **1** — Core Runtime (ECS, spatial index, reflection, events, memory, scene)
3. **2** — Infrastructure fan-out (GPU, asset import, physics, input, audio, animation, networking)
4. **3** — Intermediate systems (render graph, streaming, actions, spatial audio, state machines,
   NavMesh, replication, meshlets, widgets, particles)
5. **4** — Integration layer (core rendering, lighting, hot reload, AI, procedural anim, gameplay
   primitives, terrain, UI)
6. **5** — Advanced systems (advanced rendering, editor, logic graph, abilities, weapons,
   destruction, cloth, weather)
7. **6** — Capstone systems (procedural generation, MMO, genre-specific, CRDT collaboration)

### Critical Path

The longest chain through the DAG:

```text
Platform.Threading → CoreRuntime.ECS → Rendering.GPUAbstraction → Rendering.RenderGraph
  → Rendering.CoreRendering → Rendering.Lighting → Rendering.AdvancedRendering
    → GeometryWorld.ProceduralGeneration
```

**Critical path length: 8 feature-groups across 7 waves.** All other work proceeds in parallel
around this chain.

### Interoperability Contracts

These contracts must be defined and locked before Wave 2, as they are consumed by multiple
concurrent pipelines:

| Contract                        | Defined By                          |
|---------------------------------|-------------------------------------|
| Component trait + archetype API | CoreRuntime.ECS                     |
| Entity handle type              | CoreRuntime.ECS                     |
| System trait + scheduling API   | CoreRuntime.ECS                     |
| Event channel API               | CoreRuntime.EventsPlugins           |
| Spatial index query API         | CoreRuntime.SpatialIndex            |
| Serialization trait             | CoreRuntime.ReflectionSerialization |
| Async I/O trait                 | CoreRuntime.MemoryAsyncIO           |
| Transform component schema      | CoreRuntime.SceneTransforms         |
| GPU backend trait               | Rendering.GPUAbstraction            |
| Asset handle type               | ContentPipeline.AssetDatabase       |
| Render graph pass API           | Rendering.RenderGraph               |

1. **Component trait + archetype API** — All domains
2. **Entity handle type** — All domains
3. **System trait + scheduling API** — All domains
4. **Event channel API** — All domains
5. **Spatial index query API** — Physics, Rendering, Networking, AI, Audio, GameFramework
6. **Serialization trait** — Networking, ContentPipeline, GameFramework
7. **Async I/O trait** — ContentPipeline, Platform
8. **Transform component schema** — Rendering, Physics, Animation, Audio
9. **GPU backend trait** — ContentPipeline, VFX, UI2D
10. **Asset handle type** — All domains
11. **Render graph pass API** — VFX, UI2D, all Rendering subgroups

### Key Differences from Sequential Tiers

| Aspect             | Old (Sequential Tiers)       |
|--------------------|------------------------------|
| Structure          | 5 sequential tiers by domain |
| Ordering unit      | Entire domain                |
| Max parallelism    | 1–4 agents                   |
| Cross-domain edges | Implicit, within-tier only   |
| Physics starts     | After all of Tier 0          |
| AI starts          | After Tier 2                 |
| Editor starts      | After Tier 1                 |

1. **Structure** — 7 waves by prerequisite satisfaction
2. **Ordering unit** — Feature-group within a domain
3. **Max parallelism** — 3→6→9→12→19→20→18 agents
4. **Cross-domain edges** — Explicit, any-to-any across all levels
5. **Physics starts** — As soon as ECS + spatial index are ready
6. **AI starts** — Navigation starts as soon as spatial index + physics queries are ready
7. **Editor starts** — After widget framework, rendering, hot reload, and reflection

---

## 2. MVP Definition — "First Playable"

### Target Demo: 3D Action/Adventure

**Why this genre:**

| Factor             |
|--------------------|
| Market breadth     |
| Technical showcase |
| Visual appeal      |
| Moderate scope     |
| Benchmarkable      |

1. **Market breadth** — Action/adventure is the most popular genre on Steam
2. **Technical showcase** — Exercises rendering, physics, animation, input, audio, and UI
   simultaneously
3. **Visual appeal** — Screenshots and trailers attract developers to engines
4. **Moderate scope** — Smaller than RPG (no inventory/quests), larger than puzzle (shows more
   systems)
5. **Benchmarkable** — Easy to compare against Unreal/Unity/Godot demos

### "First Playable" Feature List

The demo: A third-person character navigates a small environment with enemies, combat, pickups, and
a death/respawn loop. Think "Crash Bandicoot level" — 3 minutes of gameplay.

| System    |
|-----------|
| ECS       |
| Scene     |
| Rendering |
| Physics   |
| Input     |
| Animation |
| Audio     |
| UI        |
| Camera    |
| Assets    |
| Platform  |

1. **ECS** — Archetype storage, entities, queries, systems, events, commands
   - **Feature IDs:** F-1.1.1, F-1.1.11, F-1.1.17, F-1.1.25, F-1.1.32, F-1.5.1
2. **Scene** — Entity hierarchy, transform propagation
   - **Feature IDs:** F-1.2.1, F-1.2.4
3. **Rendering** — GPU backend, render graph, forward+, PBR, shadows, bloom
   - **Feature IDs:** F-2.1.*, F-2.2.*, F-2.4.1-4, F-2.9.1
4. **Physics** — Rigid body, collision, character controller, raycasting
   - **Feature IDs:** F-4.1.1-4, F-4.1.8, F-4.2.1-7, F-4.4.1
5. **Input** — Keyboard, mouse, gamepad, action mapping
   - **Feature IDs:** F-6.1.1-3, F-6.2.1-4
6. **Animation** — Skeletal, blending, state machine, root motion
   - **Feature IDs:** F-9.1.1-3, F-9.1.6, F-9.4.1-2
7. **Audio** — Sound source, listener, spatial, streaming
   - **Feature IDs:** F-5.1.1-5, F-5.2.1-2
8. **UI** — Widget tree, text, health bar, layout
   - **Feature IDs:** F-10.1.1, F-10.1.4, F-10.2.1, F-10.2.3
9. **Camera** — Third-person camera, collision avoidance
   - **Feature IDs:** F-13.25.1-5
10. **Assets** — Mesh import, texture import, shader compilation
    - **Feature IDs:** F-12.1.1-2, F-12.2.7
11. **Platform** — Windowing, threading, async I/O, logging
    - **Feature IDs:** F-14.1.1, F-14.3.1, F-14.3.5, F-14.4.4

**Total: ~55 features for the demo** (4.6% of all specified features). These span Waves 0–4 of the
DAG.

---

## 3. Revenue Model

### Revenue Sources

| Source               | Type                   | Price Point        |
|----------------------|------------------------|--------------------|
| Core engine + editor | Free + open source     | $0                 |
| Cloud collaboration  | SaaS subscription      | $10/user/month     |
| AI assistant         | Premium subscription   | $20/user/month     |
| Asset marketplace    | Transaction commission | 12% of sale price  |
| Enterprise tier      | SaaS subscription      | $50/user/month     |
| Game royalty         | Revenue share          | 5% above $1M gross |
| Shared build cache   | Usage-based cloud      | $0.01/build-minute |
| Dedicated hosting    | Managed infrastructure | Custom pricing     |

1. **Core engine + editor** — Always
2. **Cloud collaboration** — After ToolsEditor.AdvancedTools (Wave 6)
3. **AI assistant** — After ToolsEditor.AdvancedTools (Wave 6)
4. **Asset marketplace** — After ContentPipeline.DCCPlugins (Wave 6)
5. **Enterprise tier** — After Networking.Replication (Wave 3) + collaboration
6. **Game royalty** — After first game ships
7. **Shared build cache** — After ContentPipeline.AssetDatabase (Wave 2) + cloud infra
8. **Dedicated hosting** — After Networking.MMO (Wave 6)

### Path to $10K MRR

| Scenario | Users Needed |
|----------|-------------|
| Cloud collaboration only ($10/mo) | 1,000 paying users |
| AI assistant only ($20/mo) | 500 paying users |
| Enterprise only ($50/user/mo) | 200 seats (4-8 studios) |
| Blended (most likely) | ~400 users across tiers |

### Pricing Philosophy

- **Core engine is always free and open source.** Non-negotiable for Rust community adoption and
  competitive positioning against Unity's pricing controversies.
- **Revenue comes from cloud services, not the engine itself.** Collaboration, AI assistance, asset
  marketplace, and build infrastructure are all cloud-hosted value-adds.
- **No royalty below $1M.** Indie developers pay nothing until they succeed. This removes the
  biggest friction point in engine adoption.

---

## 4. Marketing Strategy

### Phase 1: Community Building (during Waves 0–1)

**Goal:** Establish Harmonius as a credible, exciting project in the Rust gamedev ecosystem.

| Channel                       | Expected Outcome        |
|-------------------------------|-------------------------|
| Personal blog, dev.to         | 5,000-10,000 readers    |
| r/rust_gamedev, r/rust        | Community discussion    |
| YouTube, Twitch               | 100-500 regular viewers |
| Bevy Discord, Rust Gamedev WG | Name recognition        |
| Blog, Hacker News             | Front page post         |
| GitHub wiki, blog             | Developer trust         |

1. **"Building a Game Engine in Rust with AI" blog series** — "Building a Game Engine in Rust with
   AI" blog series
2. **Post Rust ECS benchmarks vs Bevy/flecs** — Post Rust ECS benchmarks vs Bevy/flecs
3. **Livestream development sessions with Claude** — Livestream development sessions with Claude
4. **Engage Rust gamedev Discord servers** — Engage Rust gamedev Discord servers
5. **Write "Why Rust for Game Engines" technical deep-dive** — Write "Why Rust for Game Engines"
   technical deep-dive
6. **Publish architecture decision records (ADRs) publicly** — Publish architecture decision records
   (ADRs) publicly

**Unique angle:** "The first game engine built collaboratively by a human and AI." Livestreaming the
development process with Claude is genuinely novel and attracts attention from both the gamedev and
AI communities.

### Phase 2: Developer Preview (during Waves 2–3)

**Goal:** Get the engine into the hands of early adopters who will stress-test it and provide
feedback.

| Action | Channel | Expected Outcome |
|--------|---------|------------------|
| Editor alpha with invitation system | Direct outreach | 50-100 alpha testers |
| Partner with 3-5 indie developers for beta | Indie dev communities | Beta partner agreements |
| Demo game built during development | YouTube, Steam page | 10,000+ views |
| Submit talk to RustConf | Conference | 200-500 attendees |
| Submit talk to GDC (Indie Summit) | Conference | Industry visibility |
| Weekly dev log with screenshots/GIFs | Twitter/X, Mastodon, Reddit | 2,000-5,000 followers |

**Beta partner criteria:**

- Small indie team (1-5 developers)
- Willing to build a real game on Harmonius
- Comfortable with Rust or willing to use no-code logic graphs
- Will provide weekly feedback
- Gets free enterprise tier for life in exchange

### Phase 3: Public Launch (during Waves 4–5)

**Goal:** Transition from private beta to public availability.

| Action | Channel | Expected Outcome |
|--------|---------|------------------|
| Free tier available to all developers | Website, GitHub | 1,000+ downloads in first period |
| Showcase games from beta partners | YouTube, Steam | Social proof |
| Tutorial series: "Your First Game in Harmonius" | YouTube, docs | 50,000+ views |
| Documentation blitz: API reference + guides | docs.harmonius.dev | Complete beginner docs |
| Logic graph showcase: "No code required" | YouTube, Reddit | Attract non-programmer audience |

### Phase 4: Ecosystem Growth (Wave 6+)

**Goal:** Build a self-sustaining ecosystem where the community creates value for each other.

| Action | Channel | Expected Outcome |
|--------|---------|------------------|
| Asset marketplace reaches critical mass | Platform | 1,000+ assets |
| Enterprise tier launched for studios | Direct sales | 2-5 studio contracts |
| Education partnerships | Universities, bootcamps | 10+ curriculum adoptions |
| Annual "HarmoniusCon" community event | Online/hybrid | 500+ attendees |
| Game jam sponsorship | itch.io, Ludum Dare | Community games |
| Contributor program | GitHub | 20+ external contributors |

---

## 5. Competitive Positioning

### Market Landscape

| Engine        | License                              | Language            | Editor             |
|---------------|--------------------------------------|---------------------|--------------------|
| Unreal        | Source-available, 5% royalty >$1M    | C++ / Blueprints    | Full               |
| Unity         | Proprietary, runtime fee controversy | C#                  | Full               |
| Godot         | MIT open source                      | GDScript / C# / C++ | Full               |
| Bevy          | MIT + Apache 2.0                     | Rust                | None (code only)   |
| O3DE          | Apache 2.0                           | C++                 | Full               |
| **Harmonius** | **Open source core**                 | **Rust**            | **Full (planned)** |

| Engine | Strengths | Weaknesses |
|--------|-----------|------------|
| Unreal | AAA quality, Blueprints, market share | C++ complexity, build times, heavy |
| Unity | Ecosystem, asset store, mobile | Trust erosion, performance ceiling, pricing |
| Godot | Free, lightweight, growing community | Limited AAA capability, 3D is catching up |
| Bevy | Rust, ECS-first, growing fast | No editor, no visual scripting, early stage |
| O3DE | Amazon backing, modular | Complex, slow development, small community |
| **Harmonius** | **See below** | **Does not exist yet** |

### Head-to-Head Positioning

**vs Unreal Engine:**

| Dimension | Unreal | Harmonius |
|-----------|--------|-----------|
| Language | C++ (unsafe, slow builds) | Rust (safe, fast iteration with hot reload) |
| Visual scripting | Blueprints (mature) | Logic graphs (planned, similar scope) |
| Licensing | 5% royalty >$1M | Free core, cloud services revenue |
| Build system | UnrealBuildTool (slow) | Cargo (fast, incremental) |
| Editor | Decades of polish | New, modern architecture |
| Planet-scale | Limited (World Partition) | Native universe pipeline |
| Collaboration | Perforce-dependent | Built-in CRDT real-time editing |

**vs Unity:**

| Dimension | Unity | Harmonius |
|-----------|-------|-----------|
| Pricing | Runtime fee, trust broken | Free core, transparent cloud pricing |
| Performance | C# with Burst/DOTS | Native Rust, zero-cost abstractions |
| ECS | DOTS (bolted on) | ECS-first architecture from day one |
| Open source | No (proprietary) | Yes (core engine) |
| Mod support | Third-party solutions | Built-in mod ecosystem |

**vs Godot:**

| Dimension | Godot | Harmonius |
|-----------|-------|-----------|
| Scope | General-purpose indie | General-purpose, indie through AAA |
| Networking | Basic, no MMO support | Built-in MMO infrastructure |
| Procedural gen | Limited | Planet-scale universe generation |
| Collaboration | Git only | Real-time CRDT + Git integration |
| Performance | GDScript is slow, C# is better | Rust native performance |
| Asset pipeline | Basic import | Intermediate format import pipeline |

**vs Bevy:**

| Dimension | Bevy | Harmonius |
|-----------|------|-----------|
| ECS | bevy_ecs (excellent) | Similar scope, relationships built in |
| Editor | None (code only) | Full visual editor with no-code authoring |
| Visual scripting | None | Logic graph system |
| Asset pipeline | bevy_asset (basic) | Full import pipeline, hot reload, versioning |
| Scope | Growing organically | Complete engine specification from day one |
| Collaboration | None | Real-time collaborative editing |

**vs O3DE (Amazon):**

| Dimension | O3DE | Harmonius |
|-----------|------|-----------|
| Backing | Amazon (declining interest) | Solo founder + AI (lean, focused) |
| Architecture | Legacy C++ (Lumberyard DNA) | Clean Rust from scratch |
| Community | Small, shrinking | New, building |
| Development pace | Slow | AI-accelerated |

### One-Sentence Pitch

> "The first AI-accelerated, Rust-native game engine with no-code authoring, planet-scale procedural
> generation, and built-in MMO infrastructure — open source at its core."

### Positioning Statement (Long Form)

Harmonius is for developers who want Unreal's ambition with Godot's openness, built on Rust's safety
guarantees. It is the only engine designed from the ground up for real-time collaboration,
AI-assisted development, and planet-scale procedural worlds — without writing a single line of code.

---

## 6. Biggest Risks

| #  | Likelihood | Impact |
|----|------------|--------|
| 1  | High       | Fatal  |
| 2  | High       | Fatal  |
| 3  | Medium     | High   |
| 4  | Medium     | High   |
| 5  | Medium     | Medium |
| 6  | Medium     | High   |
| 7  | Low        | Medium |
| 8  | Medium     | High   |
| 9  | Low        | High   |
| 10 | Medium     | Medium |

1. **1** — Never reaching feature parity with established engines
   - **Mitigation:** Prioritize ruthlessly. Focus on the ~278 features needed for "shippable," not
     all 1,204. Use the DAG to cut low-value capstone features without affecting the critical path.
2. **2** — Solo developer burnout
   - **Mitigation:** Use Claude AI as a force multiplier. Automate everything possible. The DAG
     model enables parallel agent work, reducing bottleneck on any single person.
3. **3** — No early adopter traction
   - **Mitigation:** Build community before asking them to adopt a full engine. Make foundational
     modules genuinely useful standalone.
4. **4** — Rust gamedev ecosystem matures faster (Bevy ships an editor)
   - **Mitigation:** Differentiate on no-code, collaboration, and planet-scale. These are features
     Bevy is not pursuing.
5. **5** — Unreal/Unity/Godot copy key differentiators
   - **Mitigation:** Move fast on collaboration (CRDT) and planet-scale generation. These require
     deep architectural decisions that incumbents cannot retrofit easily.
6. **6** — Cloud service costs exceed revenue
   - **Mitigation:** Conservative free tier with usage limits. Usage-based pricing for compute. No
     always-on infrastructure until revenue justifies it.
7. **7** — Rust compile times frustrate users
   - **Mitigation:** Invest in incremental compilation. Use dynamic linking in dev mode. Hot reload
     for gameplay logic. Logic graphs bypass Rust compilation entirely.
8. **8** — GPU backend complexity (Vulkan)
   - **Mitigation:** Vulkan-only from day one on all platforms via `ash` and GLSL→SPIR-V via
     `glslc`.
9. **9** — No-code logic graphs are too slow for real games
   - **Mitigation:** Compile logic graphs to native Rust at build time. Interpret only in the editor
     for hot reload.
10. **10** — Asset marketplace never reaches critical mass
    - **Mitigation:** Seed the marketplace with first-party assets. Partner with asset creators
      early. Make publishing frictionless.

### Risk Heatmap

|                       | Low Impact | Medium Impact        | Fatal Impact                      |
|-----------------------|------------|----------------------|-----------------------------------|
| **High Likelihood**   |            |                      | Feature parity (#1), Burnout (#2) |
| **Medium Likelihood** |            | Incumbents copy (#5) |                                   |
| **Low Likelihood**    |            | Compile times (#7)   |                                   |

2. ****Medium Likelihood**** — No traction (#3), Bevy matures (#4), Cloud costs (#6), GPU complexity
   (#8), Marketplace (#10)
3. ****Low Likelihood**** — Logic graph perf (#9)

### Top 3 Risk Mitigations

#### Risk #1: Never reaching feature parity

- Accept that 100% feature parity is impossible and unnecessary
- The DAG model lets you cut entire Wave 6 capstone features without affecting Waves 0–5
- Use the 80/20 rule: 80% of users need 20% of features
- Track features shipped per wave; adjust scope based on velocity

#### Risk #2: Solo developer burnout

- Sustainable pace: 30 hours/week of focused development, not 60
- The DAG enables parallel AI agent work — the human focuses on design and architecture
- Celebrate milestones publicly (wave completions, demo videos)
- Budget for hiring a second developer when revenue hits $5K MRR

#### Risk #3: No early adopter traction

- Track leading indicators: GitHub stars, Discord members, crate downloads, blog post views
- If foundational modules see no adoption, reassess scope
- Consider pivoting to a specific niche (e.g., "the Rust MMO engine" instead of "general purpose")

---

## 7. Unique Differentiators Ranked by Market Impact

| Rank | Feature Area | Why It Matters | Feature IDs | DAG Wave |
|------|-------------|----------------|-------------|----------|
| 1 | Planet-scale procedural generation | No other engine does this | F-3.6.34–F-3.6.64 | 6 |
| 2 | Real-time CRDT collaboration | Google Docs for game dev | F-15.12.1–F-15.12.14 | 6 |
| 3 | No-code logic graph | Full gameplay authoring without code | F-15.8.1–F-15.8.14 | 5 |
| 4 | AI editor assistant | LLM-powered development | F-15.9.1–F-15.9.10 | 6 |
| 5 | Mod support ecosystem | Community-driven content | F-15.16.1–F-15.16.6 | 6 |
| 6 | MMO infrastructure | Built-in, not bolted on | F-8.7.1–F-8.7.8 | 6 |
| 7 | Voxel terrain system | Destructible worlds | F-3.2.9–F-3.2.14 | 6 |
