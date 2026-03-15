# User Stories -- 9.2 Morph Targets

## US-9.2.1.1 Accumulate Blend Shape Deltas on GPU for Facial Deformation

**As an** engine developer (P-26), **I want** GPU compute accumulation of weighted morph target
deltas (position and normal offsets) with sparse delta storage, **so that** arbitrary active targets
per mesh are evaluated with minimal memory bandwidth.

## US-9.2.1.2 Author Character Face Customization With Morph Sliders

**As a** character artist (P-9), **I want** to assign morph targets to face meshes and drive them
with weight sliders in the editor, **so that** I can create unique facial features (nose width, jaw
shape, cheekbone height) for character customization.

## US-9.2.1.3 Validate Active Morph Target Count Per Platform

**As an** engine tester (P-27), **I want** to verify that mobile supports 8-16 active morph targets
per mesh, Switch 16-32, and desktop 64+, **so that** morph target count respects per-platform GPU
budgets.

## US-9.2.2.1 Fix Elbow Deformation Artifacts With Corrective Shapes

**As a** character artist (P-9), **I want** corrective blend shapes that auto-activate from joint
angle combination rules (elbow bend past 120 degrees), **so that** extreme poses look correct
without manual per-frame intervention.

## US-9.2.2.2 Validate Corrective Shapes Disable on Mobile Under Budget

**As an** engine tester (P-27), **I want** to verify that corrective blend shapes are disabled on
mobile for non-hero characters under budget pressure, **so that** base skinning quality is
acceptable when correctives are skipped.

## US-9.2.3.1 Drive Facial Animation From Performance Capture Data

**As a** character animator (P-11), **I want** a facial animation system with standardized action
units compatible with performance capture, **so that** captured facial performances map directly to
in-engine blend shapes without manual retargeting.

## US-9.2.3.2 Display Unique NPC Expressions Across Crowded City Hubs

**As a** game designer (P-5), **I want** real-time facial animation supporting hundreds of visible
NPC faces with unique expressions, **so that** city environments feel alive with diverse, emoting
characters.

## US-9.2.3.3 Validate Facial Action Unit Count Per Platform

**As an** engine tester (P-27), **I want** to verify that mobile supports 16-24 face action units
and desktop supports 52+ (ARKit-compatible), **so that** facial expression fidelity scales with
platform capability.

## US-9.2.4.1 Play Environmental Animations From Vertex Animation Textures

**As a** environment artist (P-8), **I want** to bake complex deformations (tentacles, fluid
surfaces, foliage sway) into vertex animation textures sampled in the vertex shader, **so that**
decorative animations play with zero CPU cost.

## US-9.2.4.2 Use VAT for Distant Crowd Character Animation

**As a** game developer (P-15), **I want** distant crowd characters to use VAT playback instead of
full skeletal evaluation, **so that** animation cost for far characters drops to a single texture
sample per vertex.

## US-9.2.4.3 Validate VAT Texture Resolution Per Platform

**As an** engine tester (P-27), **I want** to verify that mobile uses half-resolution VAT textures
and desktop uses full resolution, **so that** VAT memory scales with platform capability.

## US-9.2.5.1 Stream Morph Targets On Demand for Visible Characters

**As a** game developer (P-15), **I want** async I/O morph target streaming that loads delta buffers
only for currently visible characters and evicts unused targets via LRU, **so that** MMO-scale
character customization fits in GPU memory.

## US-9.2.5.2 Validate LRU Eviction Under Memory Pressure

**As an** engine tester (P-27), **I want** to fill morph target memory and verify that LRU eviction
correctly unloads the least recently used targets and reloads them on demand, **so that** streaming
never causes allocation failures.

## US-9.2.5.3 Verify Platform-Native Async I/O for Morph Streaming

**As an** engine developer (P-26), **I want** to confirm that morph target streaming uses IOCP on
Windows, GCD on macOS, and io_uring on Linux, **so that** streaming uses the specified
platform-native async I/O path.
