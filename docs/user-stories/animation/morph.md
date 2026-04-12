# User Stories -- 9.2 Morph Targets

## Blend Shapes and Correctives

| ID          | Persona                    |
|-------------|----------------------------|
| US-9.2.1.1  | engine developer (P-26)    |
| US-9.2.1.2  | character animator (P-11)  |
| US-9.2.1.3  | technical artist (P-13)    |
| US-9.2.2.1  | rigger (P-10)              |
| US-9.2.2.2  | character animator (P-11)  |

1. **US-9.2.1.1** -- **As an** engine developer (P-26), **I want** GPU compute shaders that
   accumulate weighted morph target deltas with sparse storage, **so that** blend shapes are applied
   before skeletal skinning with minimal memory bandwidth.

2. **US-9.2.1.2** -- **As a** character animator (P-11), **I want** to blend an arbitrary number of
   morph targets per mesh with adjustable weights, **so that** character customization combines
   multiple shape variations smoothly.

3. **US-9.2.1.3** -- **As a** technical artist (P-13), **I want** morph target evaluation to
   complete within 0.5 ms for 16 active targets, **so that** facial animation and body customization
   stay within GPU budget.

4. **US-9.2.2.1** -- **As a** rigger (P-10), **I want** corrective blend shapes driven by joint
   angles to activate automatically, **so that** deformation artifacts at extreme poses are fixed
   without manual intervention.

5. **US-9.2.2.2** -- **As a** character animator (P-11), **I want** to author corrective shapes as
   difference-from-expected deltas with combination rules, **so that** corrections trigger at
   precise joint angle thresholds.

## Facial Animation

| ID          | Persona                    |
|-------------|----------------------------|
| US-9.2.3.1  | character animator (P-11)  |
| US-9.2.3.2  | character animator (P-11)  |
| US-9.2.3.3  | engine developer (P-26)    |

1. **US-9.2.3.1** -- **As a** character animator (P-11), **I want** facial blend shapes driven
   through standardized face action units, **so that** performance capture data maps directly to
   in-engine facial animation.

2. **US-9.2.3.2** -- **As a** character animator (P-11), **I want** both curve-driven keyframe
   animation and real-time parameter input for lip sync, **so that** facial animation supports both
   cinematic and runtime-driven expressions.

3. **US-9.2.3.3** -- **As an** engine developer (P-26), **I want** facial blend shapes to scale per
   platform tier with distant NPCs disabling facial animation on mobile, **so that** hundreds of
   visible characters are animated within budget.

## Per-Vertex Animation and Streaming

| ID          | Persona                    |
|-------------|----------------------------|
| US-9.2.4.1  | technical artist (P-13)    |
| US-9.2.4.2  | engine developer (P-26)    |
| US-9.2.4.3  | engine developer (P-26)    |
| US-9.2.5.1  | engine developer (P-26)    |
| US-9.2.5.2  | technical artist (P-13)    |
| US-9.2.5.3  | game developer (P-15)      |

1. **US-9.2.4.1** -- **As a** technical artist (P-13), **I want** to bake complex deformations into
   vertex animation textures sampled in the vertex shader, **so that** fluid surfaces, tentacles,
   and foliage sway play back with zero CPU cost.

2. **US-9.2.4.2** -- **As an** engine developer (P-26), **I want** VAT playback to be GPU-only with
   each animation frame stored as a texel row, **so that** playback is lightweight on all platforms.

3. **US-9.2.4.3** -- **As an** engine developer (P-26), **I want** VAT CPU frame time to show zero
   contribution from playback, **so that** VAT animations are verifiably free of CPU cost.

4. **US-9.2.5.1** -- **As an** engine developer (P-26), **I want** morph target delta buffers
   streamed from disk on demand using async I/O with LRU eviction, **so that** only targets needed
   for visible characters consume memory.

5. **US-9.2.5.2** -- **As a** technical artist (P-13), **I want** morph target streaming to support
   MMO-scale character customization, **so that** hundreds of unique characters load their shape
   data without exceeding memory budgets.

6. **US-9.2.5.3** -- **As a** game developer (P-15), **I want** morph targets streamed in before a
   character becomes visible, **so that** players never see default-shape pop-in during gameplay.
