# 15.5 — Debug Overlay System (Proposed Features)

> **Note:** F-15.5.6 (Stat Overlays) already exists and covers basic HUD stat rendering. These
> proposed features extend the debug overlay system with rendering-specific diagnostics, GPU memory
> visualization, customization, and recording capabilities.

## Rendering Performance Overlays

| ID        | Feature                          | Requirements          |
|-----------|----------------------------------|-----------------------|
| F-15.5.8  | Draw Call Breakdown Overlay      | R-15.5.8, R-15.5.8a  |
| F-15.5.9  | GPU Memory Usage Overlay         | R-15.5.9, R-15.5.9a  |
| F-15.5.10 | Frame Time Graph Overlay         | R-15.5.10, R-15.5.10a|

1. **F-15.5.8** — Displays a per-frame breakdown of draw calls categorized by render pass (shadow,
   opaque, transparent, post-process, UI). Shows total draw call count, per-pass counts, batching
   efficiency percentage, and instancing utilization. Color codes passes that exceed configurable
   thresholds. Helps artists and designers identify which scene elements generate excessive draw
   calls during play testing.
   - **Deps:** F-15.5.6 (Stat Overlays), F-2.2.1 (Render Graph)
   - **Platform:** All platforms. Mobile shows a simplified single-column layout.

2. **F-15.5.9** — Visualizes real-time GPU memory consumption as a stacked bar or mini treemap in
   the viewport corner. Categories include textures, meshes, render targets, buffers, and shader
   programs. Shows total VRAM budget, current usage, and peak usage for the session. Alerts when
   usage exceeds a configurable percentage of the GPU memory budget.
   - **Deps:** F-15.5.6 (Stat Overlays), F-15.5.3 (Memory Profiler)
   - **Platform:** Desktop and console. Metal reports via currentAllocatedSize. Vulkan uses
     VK_EXT_memory_budget. D3D12 uses DXGI QueryVideoMemoryInfo.

3. **F-15.5.10** — Renders a scrolling frame time graph directly in the viewport showing the last N
   frames (configurable, default 300). Displays CPU frame time, GPU frame time, and present latency
   as separate color-coded lines. Marks target frame time thresholds (16.67 ms for 60 FPS, 8.33 ms
   for 120 FPS) as horizontal reference lines. Supports pause and scrub to inspect individual
   frames.
   - **Deps:** F-15.5.6 (Stat Overlays), F-15.5.1 (Frame Profiler)
   - **Platform:** All platforms. VR renders to a world-space quad attached to the left controller.

## Overlay Customization and Recording

| ID        | Feature                           | Requirements           |
|-----------|-----------------------------------|------------------------|
| F-15.5.11 | Overlay Layout Customization      | R-15.5.11, R-15.5.11a |
| F-15.5.12 | Overlay Data Recording and Export | R-15.5.12, R-15.5.12a |

1. **F-15.5.11** — Allows users to configure which overlay panels are visible, their screen position
   (corner or edge-anchored), size, opacity, and update frequency. Supports saving and loading named
   overlay presets. Presets can be shared across team members via project settings. Provides a
   drag-and-drop arrangement mode accessible from the editor toolbar.
   - **Deps:** F-15.5.6 (Stat Overlays), F-15.1.1 (Editor Framework)
   - **Platform:** Editor-side customization on desktop. Runtime overlays on all platforms use
     presets configured in the editor.

2. **F-15.5.12** — Records all overlay statistics to a structured file (CSV or JSON) for
   post-session analysis. Recording starts and stops via hotkey or editor command. Each row includes
   a timestamp, frame number, and all active stat values. Supports automated recording triggered by
   CI test runs for performance regression detection.
   - **Deps:** F-15.5.6 (Stat Overlays), F-1.8.1 (Async I/O)
   - **Platform:** All platforms. On mobile, writes to app sandbox storage. On CI, output path is
     configurable via CLI argument.

## Suggested Additional Features

| ID        | Feature                             | Requirements           |
|-----------|-------------------------------------|------------------------|
| F-15.5.13 | Render Pass Heatmap Overlay         | R-15.5.13, R-15.5.13a |

1. **F-15.5.13** — Overlays a per-pixel heatmap on the viewport showing which render pass
   contributes the most GPU time to each screen region. Modes include overdraw visualization, shader
   complexity coloring, and triangle density. Helps artists locate hotspots that cause performance
   drops in specific viewport areas.
   - **Deps:** F-15.5.2 (GPU Profiler), F-2.2.1 (Render Graph)
   - **Platform:** Desktop and console only. Requires GPU timestamp queries per draw call, which may
     not be available on all mobile GPUs.
