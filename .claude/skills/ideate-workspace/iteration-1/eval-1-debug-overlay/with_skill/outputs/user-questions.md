# Questions for User Review

These are the questions I would ask at each review checkpoint in the ideate skill workflow.

---

## Phase 2 Review: Proposed Features

I identified this idea as belonging to domain 15 (Tools & Editor), area 15.5 (Profiling Tools).
Feature F-15.5.6 (Stat Overlays) already exists and covers basic HUD stats like FPS and entity
count. The proposed features extend this into a richer debug overlay system.

### Proposed features (F-15.5.8 through F-15.5.13)

1. **F-15.5.8 Draw Call Breakdown Overlay** — Per-pass draw call counts with threshold highlighting
2. **F-15.5.9 GPU Memory Usage Overlay** — Categorized VRAM consumption with budget alerts
3. **F-15.5.10 Frame Time Graph Overlay** — Scrolling CPU/GPU frame time chart with pause and scrub
4. **F-15.5.11 Overlay Layout Customization** — Drag-and-drop panel arrangement with named presets
5. **F-15.5.12 Overlay Data Recording and Export** — CSV/JSON recording for post-session and CI
   analysis
6. **F-15.5.13 Render Pass Heatmap Overlay** — Per-pixel overdraw, shader complexity, and triangle
   density visualization

### Questions I would ask

1. Which of the proposed features do you want to keep? Are any unnecessary or out of scope for now?

2. F-15.5.6 already covers basic stat overlays (FPS, draw calls, GPU memory, etc.). Should F-15.5.8
   and F-15.5.9 be treated as expansions of F-15.5.6, or are they distinct enough to warrant
   separate feature IDs?

3. Should the overlay system support VR-specific rendering (e.g., world-space stat panels)? I
   included this for F-15.5.10 but want to confirm priority.

4. The heatmap overlay (F-15.5.13) is a suggested addition you did not explicitly mention. It
   complements the draw call and GPU memory overlays by showing WHERE performance problems occur on
   screen. Is this in scope?

5. Are there additional stats you want in the overlays beyond FPS, draw calls, and GPU memory?
   Examples:
   - Triangle count per pass
   - Shader compilation stalls
   - Texture streaming pending count
   - Physics step time
   - Audio voice count

6. Should overlay presets (F-15.5.11) be per-user or per-project? I defaulted to per-project
   (shareable via version control).

---

## Phase 4 Review: Requirements and User Stories

### Questions I would ask

1. Are the performance overhead budgets realistic?
   - Draw call overlay: < 0.1 ms CPU
   - Frame time graph: < 0.05 ms GPU
   - Recording: < 0.05 ms CPU
   - Heatmap: < 1.0 ms GPU at 1080p

2. The GPU memory overlay (R-15.5.9a) requires platform-native APIs (DXGI, VK_EXT_memory_budget,
   Metal). Should there be a fallback for platforms where these extensions are unavailable, or
   should unsupported platforms simply hide the overlay?

3. I selected these personas for user stories:
   - Technical artist (P-13) — primary profiling user
   - Game designer (P-5) — uses overlays during play testing
   - Environment artist (P-8) — GPU memory awareness
   - Effects artist (P-12) — overdraw awareness
   - Gameplay director (P-3) — frame time monitoring
   - QA lead (P-20) — regression detection and investigation
   - DevOps engineer (P-16) — CI integration
   - Engine developer (P-26) — API correctness
   - Engine tester (P-27) — verification of all features

Are any personas missing? Should the game player (P-23) have a story for toggling a simplified FPS
counter in shipped builds?

4. Should any features have additional non-functional requirements? For example:
   - Maximum memory footprint of the overlay system itself
   - Font size and readability at different resolutions
   - Color-blind accessible heatmap palettes

5. The recording feature (F-15.5.12) outputs CSV or JSON. Should there be a requirement for a
   specific schema version or backward compatibility guarantee for CI tooling?
