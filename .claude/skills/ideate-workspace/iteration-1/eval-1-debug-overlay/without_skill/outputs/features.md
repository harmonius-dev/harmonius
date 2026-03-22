# 15.5 — Debug Overlay System (Proposed)

These features extend the existing profiling tools (F-15.5.x) with a rendering-focused debug overlay
system for in-viewport performance monitoring during play testing.

## Relationship to Existing Features

The existing F-15.5.6 (Stat Overlays) provides basic HUD overlays for FPS, frame time, draw calls,
triangle count, GPU memory, and CPU thread utilization. The features below extend that foundation
with deeper rendering diagnostics, real-time GPU pipeline visualization, and interactive overlay
controls designed for play-test workflows.

## Rendering Performance Overlay

| ID        | Feature                        | Requirements |
|-----------|--------------------------------|--------------|
| F-15.5.8  | Rendering Stats Panel          | R-15.5.8     |
| F-15.5.9  | GPU Pipeline Stage Breakdown   | R-15.5.9     |
| F-15.5.10 | Draw Call Inspector Overlay     | R-15.5.10    |
| F-15.5.11 | GPU Memory Breakdown Overlay   | R-15.5.11    |
| F-15.5.12 | Frame Time Graph Overlay       | R-15.5.12    |

1. **F-15.5.8** — Renders a compact, always-on-top overlay panel in the game viewport displaying
   core rendering performance metrics: FPS (instantaneous and averaged), frame time (CPU and GPU
   separately), total draw calls, triangle count, and visible entity count. The panel updates every
   frame and supports configurable averaging windows (1s, 5s, 30s). Color-codes metrics against
   configurable budget thresholds (green/yellow/red).
   - **Deps:** F-15.5.1, F-15.5.2, F-15.5.6
   - **Platform:** All platforms. Mobile uses a minimal single-line bar to conserve screen space.
     Desktop and console show the full multi-line panel.
2. **F-15.5.9** — Displays a per-frame GPU pipeline stage breakdown showing time spent in each
   render graph pass (shadow, depth prepass, G-buffer, lighting, post-processing, UI). Renders as a
   stacked horizontal bar in the viewport with each stage color-coded and labeled with millisecond
   timings. Tapping or hovering a stage expands detail showing shader dispatch counts and wavefront
   occupancy for that stage.
   - **Deps:** F-15.5.2, F-2.2.1
   - **Platform:** Desktop and console. Not available on mobile due to limited GPU timestamp query
     support on some tile-based GPUs. Apple M-series supports full breakdown via Metal GPU counters.
3. **F-15.5.10** — Overlays a per-material draw call breakdown showing how many draw calls and
   triangles each material batch contributes. Highlights the top N most expensive materials by draw
   call count or triangle count. Supports sorting by draw calls, triangles, or GPU time. Clicking a
   material entry highlights all objects using that material in the viewport with a wireframe tint.
   - **Deps:** F-15.5.8, F-2.3.7
   - **Platform:** Desktop and console. Not available on mobile.
4. **F-15.5.11** — Renders a GPU memory usage breakdown overlay showing total allocated VRAM,
   per-category consumption (textures, buffers, render targets, meshes), and current frame's
   transient allocation pressure. Displays a stacked bar chart with category colors and
   absolute/percentage labels. Flashes a warning when total GPU memory exceeds a configurable budget
   threshold.
   - **Deps:** F-15.5.3, F-2.1.1
   - **Platform:** All platforms. Mobile shows a simplified total-only bar. Desktop shows full
     per-category breakdown. Vulkan and Metal report via memory heap queries; D3D12 via DXGI budget
     APIs.
5. **F-15.5.12** — Renders a scrolling frame time graph in the viewport corner showing the last N
   frames (configurable, default 300) of CPU frame time, GPU frame time, and present latency as
   separate colored lines. Draws horizontal threshold lines at target frame budgets (16.6ms for 60
   FPS, 33.3ms for 30 FPS). Supports freeze-frame on a specific spike for inspection. Records graph
   data for CSV export alongside F-15.5.6 stat recording.
   - **Deps:** F-15.5.1, F-15.5.2
   - **Platform:** All platforms. Mobile renders at half the graph resolution to minimize overlay
     cost.

## Overlay Configuration and Interaction

| ID        | Feature                           | Requirements |
|-----------|-----------------------------------|--------------|
| F-15.5.13 | Overlay Preset Profiles           | R-15.5.13    |
| F-15.5.14 | Hotkey Toggle and Layout Control  | R-15.5.14    |
| F-15.5.15 | Budget Threshold Configuration    | R-15.5.15    |

1. **F-15.5.13** — Supports named overlay preset profiles that save and restore which overlays are
   visible, their positions, sizes, and threshold settings. Ships with built-in presets: "Minimal"
   (FPS only), "Rendering" (all rendering overlays), "Memory" (GPU memory focus), and "Full"
   (everything). Users can create, save, and share custom presets as JSON files.
   - **Platform:** All platforms.
2. **F-15.5.14** — All overlays are toggleable via configurable hotkeys. A master toggle key
   enables/disables all overlays at once. Individual overlays can be repositioned by dragging within
   the viewport. Overlay positions persist across sessions. Overlays render with adjustable opacity
   (25%-100%) to avoid obscuring gameplay.
   - **Platform:** All platforms. Mobile uses a tap-to-cycle gesture instead of hotkeys.
3. **F-15.5.15** — Provides a configuration panel for setting per-metric budget thresholds that
   drive the color-coding across all overlays. Thresholds are defined as green (within budget),
   yellow (warning), and red (over budget) ranges. Default thresholds are derived from the target
   platform's hardware tier. Custom thresholds persist in the project settings.
   - **Deps:** F-15.5.8
   - **Platform:** All platforms. Per-platform default thresholds account for mobile, console, and
     desktop hardware tiers.
