# User Stories -- 9.2 Morph Targets

## F-9.2.1

| ID         | Persona                 | Features | Requirements |
|------------|-------------------------|----------|--------------|
| US-9.2.1.1 | engine developer (P-26) | F-9.2.1  | R-9.2.1      |
| US-9.2.1.2 | character artist (P-9)  | F-9.2.1  | R-9.2.1      |
| US-9.2.1.3 | engine tester (P-27)    | F-9.2.1  | R-9.2.1      |

1. **US-9.2.1.1** — I want GPU compute accumulation of weighted morph target deltas (position and
   normal offsets) with sparse delta storage
   - **Acceptance:** arbitrary active targets per mesh are evaluated with minimal memory bandwidth
2. **US-9.2.1.2** — I want assign morph targets to face meshes and drive them with weight sliders in
   the editor
   - **Acceptance:** I can create unique facial features (nose width, jaw shape, cheekbone height)
     for character customization
3. **US-9.2.1.3** — I want verify that mobile supports 8-16 active morph targets per mesh, Switch
   16-32, and desktop 64+
   - **Acceptance:** morph target count respects per-platform GPU budgets

## F-9.2.2

| ID         | Persona                | Features | Requirements |
|------------|------------------------|----------|--------------|
| US-9.2.2.1 | character artist (P-9) | F-9.2.2  | R-9.2.2      |
| US-9.2.2.2 | engine tester (P-27)   | F-9.2.2  | R-9.2.2      |

1. **US-9.2.2.1** — I want corrective blend shapes that auto-activate from joint angle combination
   rules (elbow bend past 120 degrees)
   - **Acceptance:** extreme poses look correct without manual per-frame intervention
2. **US-9.2.2.2** — I want verify that corrective blend shapes are disabled on mobile for non-hero
   characters under budget pressure
   - **Acceptance:** base skinning quality is acceptable when correctives are skipped

## F-9.2.3

| ID         | Persona                   | Features | Requirements |
|------------|---------------------------|----------|--------------|
| US-9.2.3.1 | character animator (P-11) | F-9.2.3  | R-9.2.3      |
| US-9.2.3.2 | game designer (P-5)       | F-9.2.3  | R-9.2.3      |
| US-9.2.3.3 | engine tester (P-27)      | F-9.2.3  | R-9.2.3      |

1. **US-9.2.3.1** — I want a facial animation system with standardized action units compatible with
   performance capture
   - **Acceptance:** captured facial performances map directly to in-engine blend shapes without
     manual retargeting
2. **US-9.2.3.2** — I want real-time facial animation supporting hundreds of visible NPC faces with
   unique expressions
   - **Acceptance:** city environments feel alive with diverse, emoting characters
3. **US-9.2.3.3** — I want verify that mobile supports 16-24 face action units and desktop supports
   52+ (ARKit-compatible)
   - **Acceptance:** facial expression fidelity scales with platform capability

## F-9.2.4

| ID         | Persona                  | Features | Requirements |
|------------|--------------------------|----------|--------------|
| US-9.2.4.1 | environment artist (P-8) | F-9.2.4  | R-9.2.4      |
| US-9.2.4.2 | game developer (P-15)    | F-9.2.4  | R-9.2.4      |
| US-9.2.4.3 | engine tester (P-27)     | F-9.2.4  | R-9.2.4      |

1. **US-9.2.4.1** — I want bake complex deformations (tentacles, fluid surfaces, foliage sway) into
   vertex animation textures sampled in the vertex shader
   - **Acceptance:** decorative animations play with zero CPU cost
2. **US-9.2.4.2** — I want distant crowd characters to use VAT playback instead of full skeletal
   evaluation
   - **Acceptance:** animation cost for far characters drops to a single texture sample per vertex
3. **US-9.2.4.3** — I want verify that mobile uses half-resolution VAT textures and desktop uses
   full resolution
   - **Acceptance:** VAT memory scales with platform capability

## F-9.2.5

| ID         | Persona                 | Features | Requirements |
|------------|-------------------------|----------|--------------|
| US-9.2.5.1 | game developer (P-15)   | F-9.2.5  | R-9.2.5      |
| US-9.2.5.2 | engine tester (P-27)    | F-9.2.5  | R-9.2.5      |
| US-9.2.5.3 | engine developer (P-26) | F-9.2.5  | R-9.2.5      |

1. **US-9.2.5.1** — I want async I/O morph target streaming that loads delta buffers only for
   currently visible characters and evicts unused targets via LRU
   - **Acceptance:** MMO-scale character customization fits in GPU memory
2. **US-9.2.5.2** — I want fill morph target memory and verify that LRU eviction correctly unloads
   the least recently used targets and reloads them on demand
   - **Acceptance:** streaming never causes allocation failures
3. **US-9.2.5.3** — I want confirm that morph target streaming uses IOCP on Windows, GCD on macOS,
   and io_uring on Linux
   - **Acceptance:** streaming uses the specified platform-native async I/O path
