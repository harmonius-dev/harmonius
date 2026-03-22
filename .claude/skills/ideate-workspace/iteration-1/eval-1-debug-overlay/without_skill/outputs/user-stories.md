# User Stories — Debug Overlay System (Proposed)

## F-15.5.8 Rendering Stats Panel

| ID           | Persona                 | Features | Requirements |
|--------------|-------------------------|----------|--------------|
| US-15.5.8.1  | game designer (P-5)     | F-15.5.8 | R-15.5.8     |
| US-15.5.8.2  | tech artist (P-13)      | F-15.5.8 | R-15.5.8     |
| US-15.5.8.3  | developer (P-15)        | F-15.5.8 | R-15.5.8     |
| US-15.5.8.4  | engine tester (P-27)    | F-15.5.8 | R-15.5.8     |
| US-15.5.8.5  | QA lead (P-20)          | F-15.5.8 | R-15.5.8     |

1. **US-15.5.8.1** — I want to see FPS and frame time directly in the viewport while play testing a
   level
   - **Acceptance:** I can immediately notice when a gameplay area causes performance drops without
     leaving play mode
2. **US-15.5.8.2** — I want the rendering stats panel to color-code metrics against budget
   thresholds so I can spot violations at a glance
   - **Acceptance:** metrics turning red alert me to performance problems without reading exact
     numbers
3. **US-15.5.8.3** — I want to see draw call count and triangle count alongside FPS to correlate
   rendering load with frame rate
   - **Acceptance:** I can determine whether low FPS is caused by too many draw calls or too much
     geometry
4. **US-15.5.8.4** — I want to verify that the rendering stats panel reports accurate FPS and frame
   time values by comparing against an external measurement tool
   - **Acceptance:** overlay values match external profiler readings within 5% tolerance
5. **US-15.5.8.5** — I want the stats panel to show averaged metrics over configurable windows so I
   can assess sustained performance
   - **Acceptance:** I can distinguish between occasional spikes and sustained performance problems

## F-15.5.9 GPU Pipeline Stage Breakdown

| ID           | Persona                 | Features | Requirements |
|--------------|-------------------------|----------|--------------|
| US-15.5.9.1  | tech artist (P-13)      | F-15.5.9 | R-15.5.9     |
| US-15.5.9.2  | engine developer (P-26) | F-15.5.9 | R-15.5.9     |
| US-15.5.9.3  | effects artist (P-12)   | F-15.5.9 | R-15.5.9     |
| US-15.5.9.4  | engine tester (P-27)    | F-15.5.9 | R-15.5.9     |

1. **US-15.5.9.1** — I want to see how much GPU time each render pass consumes as a stacked bar in
   the viewport
   - **Acceptance:** I can identify whether shadows, lighting, or post-processing is the rendering
     bottleneck
2. **US-15.5.9.2** — I want to expand a render pass in the GPU breakdown to see shader dispatch
   counts and wavefront occupancy
   - **Acceptance:** I can diagnose whether a pass is ALU-bound or bandwidth-bound without opening a
     vendor profiler
3. **US-15.5.9.3** — I want to see the post-processing pass cost in real time while adjusting VFX
   parameters
   - **Acceptance:** I can tune effect quality to stay within the GPU budget during live iteration
4. **US-15.5.9.4** — I want to verify that the GPU pipeline breakdown stage timings sum to the total
   GPU frame time
   - **Acceptance:** stage timings are consistent and account for at least 90% of total GPU frame
     time

## F-15.5.10 Draw Call Inspector Overlay

| ID            | Persona                 | Features  | Requirements |
|---------------|-------------------------|-----------|--------------|
| US-15.5.10.1  | tech artist (P-13)      | F-15.5.10 | R-15.5.10    |
| US-15.5.10.2  | env artist (P-8)        | F-15.5.10 | R-15.5.10    |
| US-15.5.10.3  | engine developer (P-26) | F-15.5.10 | R-15.5.10    |
| US-15.5.10.4  | engine tester (P-27)    | F-15.5.10 | R-15.5.10    |

1. **US-15.5.10.1** — I want to see which materials contribute the most draw calls so I can
   consolidate expensive material setups
   - **Acceptance:** the overlay ranks materials by draw call count and I can identify the top
     offenders
2. **US-15.5.10.2** — I want to click a material in the draw call overlay and see all objects using
   it highlighted in the viewport
   - **Acceptance:** I can visually locate which placed props are using the expensive material for
     optimization
3. **US-15.5.10.3** — I want to sort the draw call inspector by GPU time per material to find the
   most expensive shaders
   - **Acceptance:** I can prioritize shader optimization by actual GPU cost rather than draw call
     count alone
4. **US-15.5.10.4** — I want to verify that the draw call inspector reports draw call counts
   matching the GPU indirect draw buffer
   - **Acceptance:** overlay data is trustworthy for optimization decisions

## F-15.5.11 GPU Memory Breakdown Overlay

| ID            | Persona                 | Features  | Requirements |
|---------------|-------------------------|-----------|--------------|
| US-15.5.11.1  | tech artist (P-13)      | F-15.5.11 | R-15.5.11    |
| US-15.5.11.2  | env artist (P-8)        | F-15.5.11 | R-15.5.11    |
| US-15.5.11.3  | developer (P-15)        | F-15.5.11 | R-15.5.11    |
| US-15.5.11.4  | engine tester (P-27)    | F-15.5.11 | R-15.5.11    |

1. **US-15.5.11.1** — I want to see a per-category GPU memory breakdown (textures, meshes, render
   targets, buffers) in the viewport during play testing
   - **Acceptance:** I can identify which resource category is consuming the most VRAM without
     opening the memory profiler
2. **US-15.5.11.2** — I want a warning flash when GPU memory exceeds the budget threshold while I
   place assets in a level
   - **Acceptance:** I am alerted before GPU memory exhaustion causes visual artifacts or driver
     thrashing
3. **US-15.5.11.3** — I want to see transient GPU allocation pressure per frame alongside the
   persistent memory breakdown
   - **Acceptance:** I can detect per-frame allocation spikes that may cause GPU memory
     fragmentation
4. **US-15.5.11.4** — I want to verify that the GPU memory overlay reports values consistent with
   the platform memory API (Vulkan heap queries, Metal resource sizes, DXGI budget)
   - **Acceptance:** reported values match platform API queries within 5% tolerance

## F-15.5.12 Frame Time Graph Overlay

| ID            | Persona                 | Features  | Requirements |
|---------------|-------------------------|-----------|--------------|
| US-15.5.12.1  | developer (P-15)        | F-15.5.12 | R-15.5.12    |
| US-15.5.12.2  | tech artist (P-13)      | F-15.5.12 | R-15.5.12    |
| US-15.5.12.3  | game tester (P-19)      | F-15.5.12 | R-15.5.12    |
| US-15.5.12.4  | engine tester (P-27)    | F-15.5.12 | R-15.5.12    |

1. **US-15.5.12.1** — I want a scrolling frame time graph showing CPU and GPU frame times over the
   last 300 frames
   - **Acceptance:** I can see frame time history and identify periodic hitches or frame pacing
     issues
2. **US-15.5.12.2** — I want horizontal threshold lines at 16.6ms and 33.3ms drawn on the frame time
   graph
   - **Acceptance:** I can instantly see whether the game is meeting 60 FPS or 30 FPS targets
3. **US-15.5.12.3** — I want to freeze the frame time graph on a spike frame and inspect the exact
   timing values
   - **Acceptance:** I can capture and report specific frame time spikes in bug reports
4. **US-15.5.12.4** — I want to export the frame time graph data to CSV for automated regression
   analysis
   - **Acceptance:** CI pipelines can compare frame time data across builds to detect performance
     regressions

## F-15.5.13 Overlay Preset Profiles

| ID            | Persona                 | Features  | Requirements |
|---------------|-------------------------|-----------|--------------|
| US-15.5.13.1  | game designer (P-5)     | F-15.5.13 | R-15.5.13    |
| US-15.5.13.2  | tech artist (P-13)      | F-15.5.13 | R-15.5.13    |
| US-15.5.13.3  | QA lead (P-20)          | F-15.5.13 | R-15.5.13    |

1. **US-15.5.13.1** — I want to switch to a "Minimal" overlay preset that shows only FPS so it does
   not distract during play testing
   - **Acceptance:** one action switches from full overlays to a non-intrusive FPS counter
2. **US-15.5.13.2** — I want to create and save a custom overlay preset with my preferred layout and
   thresholds
   - **Acceptance:** I can restore my preferred overlay setup instantly across sessions and share it
     with teammates
3. **US-15.5.13.3** — I want to distribute a standard overlay preset to the QA team as a shared JSON
   file
   - **Acceptance:** all testers use identical overlay settings for consistent performance reporting

## F-15.5.14 Hotkey Toggle and Layout Control

| ID            | Persona                 | Features  | Requirements |
|---------------|-------------------------|-----------|--------------|
| US-15.5.14.1  | developer (P-15)        | F-15.5.14 | R-15.5.14    |
| US-15.5.14.2  | game designer (P-5)     | F-15.5.14 | R-15.5.14    |
| US-15.5.14.3  | game tester (P-19)      | F-15.5.14 | R-15.5.14    |

1. **US-15.5.14.1** — I want to press a single hotkey to toggle all debug overlays on or off during
   play testing
   - **Acceptance:** I can instantly hide overlays when I need an unobstructed view and restore them
     with one keypress
2. **US-15.5.14.2** — I want to drag overlays to reposition them in the viewport without overlapping
   important game UI
   - **Acceptance:** overlay positions persist across sessions so I do not have to rearrange them
     each time
3. **US-15.5.14.3** — I want to adjust overlay opacity so I can see through them to the game content
   underneath
   - **Acceptance:** setting opacity to 25% makes overlays readable but does not obscure the game
     scene

## F-15.5.15 Budget Threshold Configuration

| ID            | Persona                 | Features  | Requirements |
|---------------|-------------------------|-----------|--------------|
| US-15.5.15.1  | tech artist (P-13)      | F-15.5.15 | R-15.5.15    |
| US-15.5.15.2  | developer (P-15)        | F-15.5.15 | R-15.5.15    |
| US-15.5.15.3  | engine tester (P-27)    | F-15.5.15 | R-15.5.15    |

1. **US-15.5.15.1** — I want to set custom budget thresholds for draw calls and GPU memory that
   match our project's target hardware
   - **Acceptance:** overlay color-coding reflects our project's actual performance budgets, not
     generic defaults
2. **US-15.5.15.2** — I want per-platform default thresholds that auto-adjust when I switch the
   target platform
   - **Acceptance:** switching from desktop to mobile updates all budget thresholds to
     mobile-appropriate values
3. **US-15.5.15.3** — I want to verify that budget threshold color-coding transitions correctly at
   exact boundary values
   - **Acceptance:** a metric at exactly the yellow threshold displays yellow, not green or red
