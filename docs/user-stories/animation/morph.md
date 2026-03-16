# User Stories -- 9.2 Morph Targets

## F-9.2.1

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-9.2.1.1 | engine developer (P-26) | I want GPU compute accumulation of weighted morph target deltas (position and normal offsets) with sparse delta storage | arbitrary active targets per mesh are evaluated with minimal memory bandwidth | F-9.2.1 | R-9.2.1 |
| US-9.2.1.2 | character artist (P-9) | I want assign morph targets to face meshes and drive them with weight sliders in the editor | I can create unique facial features (nose width, jaw shape, cheekbone height) for character customization | F-9.2.1 | R-9.2.1 |
| US-9.2.1.3 | engine tester (P-27) | I want verify that mobile supports 8-16 active morph targets per mesh, Switch 16-32, and desktop 64+ | morph target count respects per-platform GPU budgets | F-9.2.1 | R-9.2.1 |

## F-9.2.2

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-9.2.2.1 | character artist (P-9) | I want corrective blend shapes that auto-activate from joint angle combination rules (elbow bend past 120 degrees) | extreme poses look correct without manual per-frame intervention | F-9.2.2 | R-9.2.2 |
| US-9.2.2.2 | engine tester (P-27) | I want verify that corrective blend shapes are disabled on mobile for non-hero characters under budget pressure | base skinning quality is acceptable when correctives are skipped | F-9.2.2 | R-9.2.2 |

## F-9.2.3

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-9.2.3.1 | character animator (P-11) | I want a facial animation system with standardized action units compatible with performance capture | captured facial performances map directly to in-engine blend shapes without manual retargeting | F-9.2.3 | R-9.2.3 |
| US-9.2.3.2 | game designer (P-5) | I want real-time facial animation supporting hundreds of visible NPC faces with unique expressions | city environments feel alive with diverse, emoting characters | F-9.2.3 | R-9.2.3 |
| US-9.2.3.3 | engine tester (P-27) | I want verify that mobile supports 16-24 face action units and desktop supports 52+ (ARKit-compatible) | facial expression fidelity scales with platform capability | F-9.2.3 | R-9.2.3 |

## F-9.2.4

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-9.2.4.1 | environment artist (P-8) | I want bake complex deformations (tentacles, fluid surfaces, foliage sway) into vertex animation textures sampled in the vertex shader | decorative animations play with zero CPU cost | F-9.2.4 | R-9.2.4 |
| US-9.2.4.2 | game developer (P-15) | I want distant crowd characters to use VAT playback instead of full skeletal evaluation | animation cost for far characters drops to a single texture sample per vertex | F-9.2.4 | R-9.2.4 |
| US-9.2.4.3 | engine tester (P-27) | I want verify that mobile uses half-resolution VAT textures and desktop uses full resolution | VAT memory scales with platform capability | F-9.2.4 | R-9.2.4 |

## F-9.2.5

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-9.2.5.1 | game developer (P-15) | I want async I/O morph target streaming that loads delta buffers only for currently visible characters and evicts unused targets via LRU | MMO-scale character customization fits in GPU memory | F-9.2.5 | R-9.2.5 |
| US-9.2.5.2 | engine tester (P-27) | I want fill morph target memory and verify that LRU eviction correctly unloads the least recently used targets and reloads them on demand | streaming never causes allocation failures | F-9.2.5 | R-9.2.5 |
| US-9.2.5.3 | engine developer (P-26) | I want confirm that morph target streaming uses IOCP on Windows, GCD on macOS, and io_uring on Linux | streaming uses the specified platform-native async I/O path | F-9.2.5 | R-9.2.5 |
