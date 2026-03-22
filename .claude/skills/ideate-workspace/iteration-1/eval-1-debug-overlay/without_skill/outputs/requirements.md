# R-15.5 — Debug Overlay System Requirements (Proposed)

These requirements extend the existing profiling tools requirements (R-15.5.x) for the rendering
debug overlay system.

## Rendering Performance Overlay

| ID        | Derived From |
|-----------|--------------|
| R-15.5.8  | F-15.5.8     |
| R-15.5.9  | F-15.5.9     |
| R-15.5.10 | F-15.5.10    |
| R-15.5.11 | F-15.5.11    |
| R-15.5.12 | F-15.5.12    |

1. **R-15.5.8** — The engine **SHALL** render a compact overlay panel in the game viewport
   displaying FPS (instantaneous and averaged), CPU frame time, GPU frame time, draw call count,
   triangle count, and visible entity count, updating every frame with configurable averaging
   windows (1s, 5s, 30s), color-coded against per-metric budget thresholds (green/yellow/red).
   - **Rationale:** Developers need immediate visual feedback on rendering performance during play
     testing without switching to a separate profiler window.
   - **Verification:** Run a test scene and verify all six metrics display non-zero values. Toggle
     averaging windows and verify FPS values smooth appropriately. Set a draw call budget below the
     current count and verify the metric turns red.
2. **R-15.5.9** — The engine **SHALL** display a per-frame GPU pipeline stage breakdown as a stacked
   horizontal bar in the viewport, showing time per render graph pass (shadow, depth prepass,
   G-buffer, lighting, post-processing, UI) with millisecond labels, and **SHALL** support expanding
   a stage to show shader dispatch counts and wavefront occupancy.
   - **Rationale:** Identifying which render pass is the bottleneck requires per-pass GPU timing
     visible during gameplay.
   - **Verification:** Render a scene with all major passes active. Verify the stacked bar sums to
     within 10% of total GPU frame time. Expand the lighting pass and verify dispatch count matches
     the number of light evaluation dispatches in the render graph.
3. **R-15.5.10** — The engine **SHALL** overlay a per-material draw call breakdown showing draw call
   count and triangle count per material batch, sorted by configurable criteria (draw calls,
   triangles, GPU time), highlighting the top N most expensive materials, and **SHALL** support
   clicking a material entry to highlight all objects using that material in the viewport.
   - **Rationale:** Material batching efficiency directly affects draw call count; visualizing
     per-material cost identifies optimization targets.
   - **Verification:** Render a scene with 10 distinct materials. Verify the overlay lists all 10
     with correct draw call counts matching the GPU indirect draw buffer. Click a material and
     verify all instances highlight with a wireframe tint.
4. **R-15.5.11** — The engine **SHALL** render a GPU memory usage breakdown overlay showing total
   allocated VRAM, per-category consumption (textures, buffers, render targets, meshes), and
   transient allocation pressure, and **SHALL** flash a warning when total GPU memory exceeds a
   configurable budget threshold.
   - **Rationale:** GPU memory exhaustion causes driver-level thrashing or crashes; real-time
     visibility prevents exceeding hardware limits during content authoring and play testing.
   - **Verification:** Load a scene with known texture, mesh, and render target sizes. Verify
     per-category values sum to the reported total within 5%. Set a budget below the current total
     and verify the warning flashes.
5. **R-15.5.12** — The engine **SHALL** render a scrolling frame time graph showing the last N
   frames (configurable, default 300) of CPU frame time, GPU frame time, and present latency as
   separate lines, with horizontal threshold lines at target frame budgets, and **SHALL** support
   freeze-frame inspection and CSV export.
   - **Rationale:** A historical frame time graph reveals intermittent hitches and frame pacing
     issues invisible in instantaneous metrics.
   - **Verification:** Introduce a deliberate 50ms stall every 60 frames. Verify the graph shows
     periodic spikes at the correct interval. Freeze the graph on a spike and verify the tooltip
     shows the correct frame time value. Export to CSV and verify the spike frames appear in the
     data.

## Overlay Configuration and Interaction

| ID        | Derived From |
|-----------|--------------|
| R-15.5.13 | F-15.5.13    |
| R-15.5.14 | F-15.5.14    |
| R-15.5.15 | F-15.5.15    |

1. **R-15.5.13** — The engine **SHALL** support named overlay preset profiles that save and restore
   overlay visibility, positions, sizes, and threshold settings, shipping with built-in presets
   (Minimal, Rendering, Memory, Full) and allowing user-created presets stored as JSON files.
   - **Rationale:** Different testing scenarios require different overlay configurations; presets
     eliminate repetitive manual setup.
   - **Verification:** Create a custom preset with specific overlay positions and thresholds.
     Restart the editor, load the preset, and verify all settings are restored exactly. Verify
     built-in presets activate the correct overlay combinations.
2. **R-15.5.14** — The engine **SHALL** support hotkey toggling of all overlays individually and
   collectively via a master toggle, drag-to-reposition overlays within the viewport with position
   persistence across sessions, and adjustable overlay opacity (25%-100%).
   - **Rationale:** Overlays must not obstruct gameplay during play testing; quick toggling and
     repositioning keeps them useful without being intrusive.
   - **Verification:** Press the master toggle hotkey and verify all overlays hide. Press again and
     verify they restore. Drag an overlay to a new position, restart the session, and verify the
     position persists. Set opacity to 25% and verify the overlay is visually semi-transparent.
3. **R-15.5.15** — The engine **SHALL** provide per-metric budget threshold configuration with
   green/yellow/red ranges that drive color-coding across all overlays, with platform-derived
   defaults and project-level persistence for custom thresholds.
   - **Rationale:** Budget thresholds vary by target platform and project; configurable thresholds
     ensure the overlay is useful across hardware tiers.
   - **Verification:** Set a custom FPS threshold of 30/45/60 for green/yellow/red. Run a scene at
     40 FPS and verify the FPS metric displays in yellow. Change the target platform and verify the
     default thresholds update accordingly.

## Non-Functional Requirements

| ID         |
|------------|
| NFR-15.5.8 |
| NFR-15.5.9 |

1. **NFR-15.5.8** — The combined rendering overhead of all active debug overlays **SHALL NOT**
   exceed 0.5ms of GPU time and 0.25ms of CPU time per frame at 1080p on target desktop hardware.
   - **Rationale:** Debug overlays that significantly affect frame time distort the metrics they are
     displaying, making them unreliable.
   - **Verification:** Enable all overlays simultaneously. Measure GPU and CPU time with and without
     overlays on a reference scene. Verify the delta is under 0.5ms GPU and 0.25ms CPU.
2. **NFR-15.5.9** — Debug overlay rendering **SHALL** use a dedicated render pass injected after the
   final post-processing pass and before present, ensuring overlays do not interfere with scene
   rendering or post-processing effects.
   - **Rationale:** Overlays rendered mid-pipeline could affect post-processing (bloom, DoF) or be
     occluded by UI elements.
   - **Verification:** Enable bloom and DoF post-processing. Verify overlay text is not affected by
     bloom glow or depth-of-field blur. Verify overlays render on top of all scene content and game
     UI.
