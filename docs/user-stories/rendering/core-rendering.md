# User Stories -- 2.3 Core Rendering

## US-2.3.1.1 Preview Direct Lighting Across All Light Types

**As a** environment artist (P-8), **I want** to place point, spot, and directional lights in a
scene and see physically-based attenuation in real time, **so that** I can evaluate lighting
composition and color without waiting for a bake pass.

## US-2.3.1.2 Validate Light Buffer Correctness Across Forward and Deferred Paths

**As an** engine tester (P-27), **I want** to run automated comparison tests of the unified light
buffer output between the forward and deferred rendering paths, **so that** I can verify that both
paths produce identical lighting results across all light types.

## US-2.3.1.3 Profile Per-Tile Light Counts Under Stress

**As an** engine developer (P-26), **I want** to query per-tile light assignment counts during a
stress test with 256 overlapping lights, **so that** I can detect tile overflow and validate
platform-specific tile caps (16 on mobile, 32 on Switch, 256 on desktop).

## US-2.3.2.1 Render Dense Forests Without CPU Bottlenecks

**As a** environment artist (P-8), **I want** GPU frustum culling at meshlet granularity to
automatically exclude off-screen foliage, **so that** I can build dense vegetation scenes without
worrying about manual culling volumes or draw call budgets.

## US-2.3.2.2 Verify Meshlet Frustum Culling Produces No Visual Gaps

**As an** engine tester (P-27), **I want** to sweep the camera through a fully populated test scene
at 1-degree increments and compare against a brute-force reference render, **so that** I can confirm
no meshlets are incorrectly culled near frustum boundaries.

## US-2.3.3.1 Eliminate Wasted Rasterization on Solid Objects

**As an** engine developer (P-26), **I want** meshlet-level normal cone culling to skip backfacing
meshlet groups before rasterization, **so that** GPU fragment workload is reduced by 30-50% on solid
closed meshes.

## US-2.3.3.2 Validate Backface Culling Does Not Affect Two-Sided Materials

**As an** engine tester (P-27), **I want** to verify that meshlets flagged as two-sided (foliage,
curtains) are excluded from normal cone culling, **so that** double-sided materials render correctly
from all viewing angles.

## US-2.3.4.1 Maintain Frame Rate in Dense Interior Scenes

**As a** game developer (P-15), **I want** two-phase HZB occlusion culling to automatically reject
geometry hidden behind walls and furniture, **so that** interior scenes with heavy overdraw stay
within the GPU frame budget without manual occlusion volumes.

## US-2.3.4.2 Confirm Phase-2 Retest Recovers Newly Revealed Geometry

**As an** engine tester (P-27), **I want** to record a fast camera pan that reveals previously
occluded geometry and verify that phase-2 HZB retesting renders it within the same frame,
**so that** players never see single-frame pop-in when turning quickly.

## US-2.3.4.3 Debug HZB Mip Chain Accuracy

**As an** engine developer (P-26), **I want** to visualize the HZB mip chain as a buffer overlay
alongside the final depth buffer, **so that** I can diagnose conservative occlusion errors when
objects are incorrectly culled at distance.

## US-2.3.5.1 Set Up an Orthographic Camera for a Top-Down RPG

**As a** game designer (P-5), **I want** to configure an orthographic projection camera with
adjustable width and height, **so that** I can preview and iterate on a top-down game layout without
perspective distortion.

## US-2.3.5.2 Verify Orthographic Shadow Map Generation

**As an** engine tester (P-27), **I want** to render shadow maps using orthographic projection and
compare shadow coverage against a known reference, **so that** directional light shadow cascades
produce correct results in orthographic views.

## US-2.3.6.1 Maintain Depth Precision at Extreme Distances

**As a** game developer (P-15), **I want** reverse-Z perspective projection with an infinite far
plane, **so that** distant terrain and skybox elements have sub-pixel depth precision without
z-fighting artifacts.

## US-2.3.6.2 Test Reverse-Z Across All GPU Backends

**As an** engine tester (P-27), **I want** to run depth precision tests on Metal, D3D12, and Vulkan
to verify that reverse-Z clears depth to 0 correctly on each backend, **so that** depth comparison
produces identical results across platforms.

## US-2.3.7.1 Draw Thousands of Material Variants With Minimal Draw Calls

**As an** engine developer (P-26), **I want** GPU-driven instance compaction to batch surviving
opaque meshlets by material into contiguous indirect draws, **so that** a scene with 10,000 material
instances renders with fewer than 100 draw calls.

## US-2.3.7.2 Author Unique Props Without Performance Anxiety

**As a** environment artist (P-8), **I want** the instancing system to automatically batch my placed
props by shared material, **so that** I can scatter hundreds of unique asset variants across a level
without manually managing draw call counts.

## US-2.3.7.3 Verify Transparent Objects Render in Correct Depth Order

**As an** engine tester (P-27), **I want** to place overlapping transparent objects at various
depths and confirm they render in back-to-front sorted order, **so that** transparency compositing
is visually correct without sorting artifacts.

## US-2.3.8.1 Route Any Pass Output to an Off-Screen Texture

**As an** engine developer (P-26), **I want** to declare a render-to-texture target in the render
graph and have barriers inserted automatically between write and read, **so that** I can chain
multi-pass effects without manual synchronization.

## US-2.3.8.2 Render Security Camera Feeds as Material Inputs

**As a** game designer (P-5), **I want** to render a secondary camera view to a texture and display
it on an in-world monitor mesh, **so that** players can see security camera feeds as interactive
gameplay elements.

## US-2.3.9.1 Preview Reflection Probes With Dynamic Cubemaps

**As a** environment artist (P-8), **I want** to place a reflection probe that re-renders selected
cubemap faces each frame, **so that** reflections update dynamically when I move objects in the
scene without a manual rebake.

## US-2.3.9.2 Validate Cubemap Face Rendering Correctness

**As an** engine tester (P-27), **I want** to render a test scene to all six cubemap faces and
verify seamless stitching at face edges, **so that** cubemap-based reflections and IBL prefiltering
have no visible seam artifacts.

## US-2.3.10.1 Create a Portal Effect Using Scene Capture

**As a** effects artist (P-12), **I want** to render the scene from a second camera into a texture
and apply it to a portal mesh with distortion, **so that** players see a real-time view of the
destination through the portal surface.

## US-2.3.10.2 Verify Scene Capture Resolution Scaling Per Platform

**As an** engine tester (P-27), **I want** to confirm that scene captures respect platform limits
(quarter-resolution on mobile, half on Switch, configurable on desktop), **so that** scene captures
do not exceed per-platform GPU budgets.

## US-2.3.11.1 Play Without Frame Drops During Boss Fights

**As a** player (P-23), **I want** the game to automatically lower its internal resolution when the
frame rate dips during intense combat, **so that** gameplay remains responsive without visible
stuttering.

## US-2.3.11.2 Tune Dynamic Resolution Min/Max Bounds Per Platform

**As a** technical artist (P-13), **I want** to configure per-platform minimum and maximum render
resolution percentages with a GPU timing feedback loop, **so that** I can balance visual quality and
performance targets for each hardware tier.

## US-2.3.11.3 Validate DRS Feedback Loop Stability

**As an** engine tester (P-27), **I want** to run a frame-time stress test and verify that dynamic
resolution converges to a stable percentage without oscillating, **so that** players do not
experience distracting resolution flickering.

## US-2.3.12.1 Author Skin Materials With Subsurface Scatter Profiles

**As a** character artist (P-9), **I want** to assign a subsurface scattering profile to skin
materials and preview the scatter radius in real time, **so that** I can achieve realistic skin
translucency for hero characters without guessing at parameter values.

## US-2.3.12.2 Test SSS Fallback Quality on Lower Platforms

**As an** engine tester (P-27), **I want** to compare screen-space SSS output on desktop against the
preintegrated LUT fallback on mobile, **so that** skin rendering remains visually acceptable across
all hardware tiers.

## US-2.3.13.1 Use Alpha Cutouts for Vegetation Without Shadow Gaps

**As a** environment artist (P-8), **I want** alpha-tested geometry to participate in shadow map
rendering with a configurable alpha threshold, **so that** fences and vegetation cast correct
shadows without manual shadow proxy meshes.

## US-2.3.13.2 Validate Alpha Cutout Impact on Tile-Based GPUs

**As an** engine developer (P-26), **I want** to profile alpha-test overdraw on tile-based mobile
GPUs and verify that distant vegetation falls back to opaque proxies, **so that** alpha cutouts do
not break hidden surface removal and cause bandwidth spikes on mobile.
