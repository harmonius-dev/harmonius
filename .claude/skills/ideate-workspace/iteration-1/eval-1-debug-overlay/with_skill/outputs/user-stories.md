# Debug Overlay System User Stories

## Draw Call Breakdown Overlay

| ID         | Persona                  | Features  | Requirements |
|------------|--------------------------|-----------|--------------|
| US-15.5.8  | technical artist (P-13)  | F-15.5.8  | R-15.5.8     |
| US-15.5.9  | game designer (P-5)      | F-15.5.8  | R-15.5.8     |
| US-15.5.10 | engine tester (P-27)     | F-15.5.8  | R-15.5.8a    |

1. **US-15.5.8** — As a technical artist, I want to see draw calls broken down by render pass in a
   viewport overlay so that I can identify which passes are over budget and guide artists to
   optimize the right assets.
   - **Acceptance:** Overlay shows per-pass draw call counts<br>Passes exceeding threshold are
     highlighted<br> Overlay updates every frame without noticeable lag

2. **US-15.5.9** — As a game designer, I want to toggle a draw call overlay during play testing so
   that I can verify my level layouts stay within the performance budget before handing off to
   engineering.
   - **Acceptance:** Overlay activates via a single hotkey<br>Total draw call count is visible at a
     glance<br>Overlay does not obstruct core gameplay area

3. **US-15.5.10** — As an engine tester, I want to verify that the draw call overlay introduces
   negligible CPU overhead so that the overlay does not distort performance measurements.
   - **Acceptance:** CPU frame time delta under 0.1 ms with overlay on vs. off<br>Zero overhead when
     overlay is disabled<br>Test passes on all supported platforms

## GPU Memory Usage Overlay

| ID         | Persona                     | Features  | Requirements |
|------------|-----------------------------|-----------|--------------|
| US-15.5.11 | environment artist (P-8)    | F-15.5.9  | R-15.5.9     |
| US-15.5.12 | engine developer (P-26)     | F-15.5.9  | R-15.5.9a    |
| US-15.5.13 | engine tester (P-27)        | F-15.5.9  | R-15.5.9     |

1. **US-15.5.11** — As an environment artist, I want to see GPU memory usage categorized by resource
   type in the viewport so that I can stay within the texture memory budget while dressing a level.
   - **Acceptance:** Overlay shows texture, mesh, render target, and buffer categories<br>Budget bar
     turns red when over threshold<br>Values update within 1 second

2. **US-15.5.12** — As an engine developer, I want the GPU memory overlay to use platform-native
   memory query APIs so that reported values match driver-level accounting.
   - **Acceptance:** Values match DXGI, Vulkan memory_budget, or Metal currentAllocatedSize within
     5%<br>No fallback to manual tracking

3. **US-15.5.13** — As an engine tester, I want to verify that the GPU memory overlay accurately
   reflects allocations and deallocations so that I can trust the displayed values during regression
   testing.
   - **Acceptance:** Allocate a known-size resource and verify overlay reflects the increase<br>Free
     the resource and verify overlay reflects the decrease<br>Tolerance within 5%

## Frame Time Graph Overlay

| ID         | Persona                   | Features   | Requirements |
|------------|---------------------------|------------|--------------|
| US-15.5.14 | gameplay director (P-3)   | F-15.5.10  | R-15.5.10    |
| US-15.5.15 | QA lead (P-20)            | F-15.5.10  | R-15.5.10    |
| US-15.5.16 | engine tester (P-27)      | F-15.5.10  | R-15.5.10a   |

1. **US-15.5.14** — As a gameplay director, I want to see a scrolling frame time graph in the
   viewport so that I can spot hitches and frame time spikes while testing gameplay pacing.
   - **Acceptance:** Graph shows CPU and GPU frame time as separate lines<br>16.67 ms reference line
     is visible<br> Spikes are clearly identifiable

2. **US-15.5.15** — As a QA lead, I want to pause and scrub the frame time graph so that I can
   inspect individual frames when investigating a reported hitch.
   - **Acceptance:** Pause freezes the graph<br>Scrub reveals exact frame time values<br>Resume
     continues from the current frame

3. **US-15.5.16** — As an engine tester, I want to verify that the frame time graph overlay renders
   in under 0.05 ms of GPU time so that the overlay itself does not cause frame time jitter.
   - **Acceptance:** GPU timestamp query around overlay draw call reports under 0.05 ms<br>Test
     passes at 1080p and 4K

## Overlay Customization

| ID         | Persona                  | Features   | Requirements |
|------------|--------------------------|------------|--------------|
| US-15.5.17 | technical artist (P-13)  | F-15.5.11  | R-15.5.11    |
| US-15.5.18 | game developer (P-15)    | F-15.5.11  | R-15.5.11    |
| US-15.5.19 | engine tester (P-27)     | F-15.5.11  | R-15.5.11a   |

1. **US-15.5.17** — As a technical artist, I want to arrange overlay panels by dragging them to
   specific screen positions so that I can build a profiling layout that fits my workflow.
   - **Acceptance:** Panels snap to corners and edges<br>Custom layout persists after editor
     restart<br>Opacity is adjustable per panel

2. **US-15.5.18** — As a game developer, I want to save and load named overlay presets so that I can
   switch between a rendering-focused layout and a networking-focused layout without reconfiguring
   panels each time.
   - **Acceptance:** Save creates a named preset in project settings<br>Load restores all panel
     positions and visibility<br>Presets are shareable via version control

3. **US-15.5.19** — As an engine tester, I want to verify that overlay configuration changes apply
   within a single frame so that live tuning is responsive during testing.
   - **Acceptance:** Toggle visibility via hotkey and observe change in the next frame<br>No stall
     or recompilation on config change

## Overlay Data Recording

| ID         | Persona                 | Features   | Requirements |
|------------|-------------------------|------------|--------------|
| US-15.5.20 | DevOps engineer (P-16)  | F-15.5.12  | R-15.5.12    |
| US-15.5.21 | QA lead (P-20)          | F-15.5.12  | R-15.5.12    |
| US-15.5.22 | engine tester (P-27)    | F-15.5.12  | R-15.5.12a   |

1. **US-15.5.20** — As a DevOps engineer, I want CI test runs to automatically record overlay stats
   to a JSON file so that I can detect performance regressions between builds.
   - **Acceptance:** CLI flag enables recording without GUI<br>Output file is valid JSON parseable
     by jq<br>Each row contains timestamp and frame number

2. **US-15.5.21** — As a QA lead, I want to start and stop overlay recording via hotkey during
   manual play testing so that I can capture performance data for specific gameplay scenarios
   without engineering support.
   - **Acceptance:** Hotkey starts recording with on-screen indicator<br>Hotkey stops recording and
     saves file<br>File contains all active overlay stat columns

3. **US-15.5.22** — As an engine tester, I want to verify that overlay recording adds less than 0.05
   ms of CPU overhead so that recorded sessions reflect true performance.
   - **Acceptance:** CPU frame time delta under 0.05 ms with recording on vs. off<br>Async writes
     confirmed via profiler<br>No file I/O on the game thread

## Render Pass Heatmap Overlay

| ID         | Persona                  | Features   | Requirements |
|------------|--------------------------|------------|--------------|
| US-15.5.23 | technical artist (P-13)  | F-15.5.13  | R-15.5.13    |
| US-15.5.24 | effects artist (P-12)    | F-15.5.13  | R-15.5.13    |
| US-15.5.25 | engine tester (P-27)     | F-15.5.13  | R-15.5.13a   |

1. **US-15.5.23** — As a technical artist, I want to enable a shader complexity heatmap in the
   viewport so that I can identify which materials are most expensive and prioritize shader
   optimization.
   - **Acceptance:** Heatmap colors pixels by estimated ALU cost<br>Hot areas are clearly
     distinguishable from cool areas<br>Mode is togglable via hotkey

2. **US-15.5.24** — As an effects artist, I want to see an overdraw heatmap so that I can verify my
   particle effects do not cause excessive overdraw in gameplay scenarios.
   - **Acceptance:** Overdraw count is visualized per pixel<br> Transparent particles contribute to
     overdraw count<br> Heatmap is visible without disabling VFX

3. **US-15.5.25** — As an engine tester, I want to verify that the heatmap overlay adds less than
   1.0 ms of GPU time at 1080p so that it remains usable as a diagnostic tool.
   - **Acceptance:** GPU timestamp delta under 1.0 ms on a mid-range desktop GPU<br>Test passes at
     1080p resolution<br>Heatmap renders correctly on all GPU backends
