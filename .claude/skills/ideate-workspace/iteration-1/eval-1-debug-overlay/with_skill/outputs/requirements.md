# R-15.5 — Debug Overlay System Requirements

## Rendering Performance Overlays

| ID         | Derived From                                      |
|------------|---------------------------------------------------|
| R-15.5.8   | [F-15.5.8](../../features/tools-editor/profiling-tools.md) |
| R-15.5.8a  | [F-15.5.8](../../features/tools-editor/profiling-tools.md) |
| R-15.5.9   | [F-15.5.9](../../features/tools-editor/profiling-tools.md) |
| R-15.5.9a  | [F-15.5.9](../../features/tools-editor/profiling-tools.md) |
| R-15.5.10  | [F-15.5.10](../../features/tools-editor/profiling-tools.md)|
| R-15.5.10a | [F-15.5.10](../../features/tools-editor/profiling-tools.md)|

1. **R-15.5.8** — The engine **SHALL** display a draw call breakdown overlay categorized by render
   pass (shadow, opaque, transparent, post-process, UI) with per-pass and total counts updated every
   frame.
   - **Rationale:** Artists and designers need immediate feedback on draw call distribution to
     optimize scene complexity during play testing.
   - **Verification:** Enable overlay in a scene with known pass counts. Verify displayed counts
     match GPU profiler pass data within 1% tolerance.

2. **R-15.5.8a** — The draw call breakdown overlay **SHALL** introduce less than 0.1 ms of CPU
   overhead per frame when active and zero overhead when disabled.
   - **Rationale:** Debug overlays must not distort the performance characteristics they measure.
   - **Verification:** Benchmark a 10,000 draw call scene with overlay on and off. Measure delta in
     CPU frame time. Delta must be under 0.1 ms.

3. **R-15.5.9** — The engine **SHALL** display a GPU memory usage overlay showing current
   consumption categorized by resource type (textures, meshes, render targets, buffers, shader
   programs) and total VRAM budget.
   - **Rationale:** GPU memory exhaustion causes hitching and crashes. Real-time visibility prevents
     artists from exceeding budgets unknowingly.
   - **Verification:** Load scenes with known asset sizes. Verify overlay totals match platform
     memory query APIs within 5% tolerance.

4. **R-15.5.9a** — The GPU memory overlay **SHALL** update at least once per second and query memory
   usage using platform-native APIs (DXGI QueryVideoMemoryInfo, Vulkan VK_EXT_memory_budget, Metal
   currentAllocatedSize).
   - **Rationale:** Stale data misleads optimization decisions. Platform-native APIs provide
     accurate reporting.
   - **Verification:** Allocate and free a 256 MB texture. Verify the overlay reflects the change
     within 1 second.

5. **R-15.5.10** — The engine **SHALL** display a scrolling frame time graph showing CPU frame time,
   GPU frame time, and present latency as separate lines over the last N frames (configurable,
   default 300), with horizontal reference lines at 16.67 ms and 8.33 ms.
   - **Rationale:** A time-series view reveals hitches and regressions that instantaneous FPS
     counters miss.
   - **Verification:** Inject synthetic frame time spikes. Verify spikes appear in the graph at the
     correct frame positions and amplitudes.

6. **R-15.5.10a** — The frame time graph overlay **SHALL** render in under 0.05 ms of GPU time using
   a single draw call with a line-strip or instanced-quad approach.
   - **Rationale:** The graph itself must not introduce frame time jitter.
   - **Verification:** Measure GPU time of the overlay draw call via timestamp queries. Must be
     under 0.05 ms.

## Overlay Customization and Recording

| ID         | Derived From                                       |
|------------|----------------------------------------------------|
| R-15.5.11  | [F-15.5.11](../../features/tools-editor/profiling-tools.md)|
| R-15.5.11a | [F-15.5.11](../../features/tools-editor/profiling-tools.md)|
| R-15.5.12  | [F-15.5.12](../../features/tools-editor/profiling-tools.md)|
| R-15.5.12a | [F-15.5.12](../../features/tools-editor/profiling-tools.md)|

1. **R-15.5.11** — The engine **SHALL** provide a configuration interface for overlay panels
   supporting visibility toggle, screen position (corner or edge anchor), size, opacity, and update
   frequency, with named presets that persist in project settings.
   - **Rationale:** Different roles need different stat views. Presets avoid reconfiguring overlays
     per session.
   - **Verification:** Create a preset with 3 panels in custom positions. Save, reload project, and
     verify all panel positions and settings are restored.

2. **R-15.5.11a** — Overlay configuration changes **SHALL** apply within a single frame with no
   stall or recompilation.
   - **Rationale:** Live tuning of overlay layout must feel instant during play testing.
   - **Verification:** Toggle overlay visibility via hotkey. Measure frames between input and visual
     change. Must be 0 or 1 frames.

3. **R-15.5.12** — The engine **SHALL** record all active overlay statistics to a structured CSV or
   JSON file with per-frame rows containing timestamp, frame number, and all stat values. Recording
   starts and stops via hotkey or CLI command.
   - **Rationale:** Post-session analysis and CI regression detection require persistent, parseable
     performance data.
   - **Verification:** Record 1000 frames. Open the output file and verify it contains 1000 data
     rows with all expected columns. Parse with jq or a CSV library without errors.

4. **R-15.5.12a** — Overlay recording **SHALL** introduce less than 0.05 ms of CPU overhead per
   frame, using buffered async writes to avoid blocking the game thread.
   - **Rationale:** Recording must not perturb the measurements.
   - **Verification:** Benchmark with and without recording active. CPU frame time delta must be
     under 0.05 ms.

## Render Pass Heatmap

| ID         | Derived From                                       |
|------------|----------------------------------------------------|
| R-15.5.13  | [F-15.5.13](../../features/tools-editor/profiling-tools.md)|
| R-15.5.13a | [F-15.5.13](../../features/tools-editor/profiling-tools.md)|

1. **R-15.5.13** — The engine **SHALL** provide a per-pixel heatmap overlay with selectable modes:
   overdraw count, shader complexity (estimated ALU cost), and triangle density per screen tile.
   - **Rationale:** Localized hotspots are invisible in aggregate stats. Per-pixel visualization
     pinpoints problem areas.
   - **Verification:** Place a known-overdraw object (10 layers). Verify the overdraw heatmap shows
     10x intensity at that screen location.

2. **R-15.5.13a** — The heatmap overlay **SHALL** render in under 1.0 ms of additional GPU time at
   1080p resolution.
   - **Rationale:** Heatmap is a diagnostic tool that should not dominate the frame budget.
   - **Verification:** Enable heatmap at 1080p. Measure additional GPU time via timestamp queries.
     Must be under 1.0 ms on a mid-range desktop GPU (e.g., RTX 4060).
