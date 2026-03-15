# User Stories -- 2.9 Post-Processing

## US-2.9.1.1 Add Bloom Glow to Bright Light Sources

**As a** environment artist (P-8), **I want** bright-source bloom with a multi-pass blur chain
that extracts pixels above a luminance threshold and composites the glow additively, **so that**
neon signs, magic effects, and sunlit surfaces produce a cinematic light bleed.

## US-2.9.1.2 Validate Bloom Blur Iteration Count Per Platform

**As an** engine tester (P-27), **I want** to verify that mobile uses 3-pass dual-kawase at
quarter-res, Switch uses 4-pass Gaussian at half-res, and desktop runs 6+ iterations, **so
that** bloom cost scales correctly per platform.

## US-2.9.2.1 Set Up Cinematic Depth of Field for Cutscenes

**As a** effects artist (P-12), **I want** depth of field driven by real camera parameters
(aperture, focal length, focus distance) with gather-based circular bokeh, **so that** cutscenes
have cinematic focus pulls with correctly shaped bokeh highlights.

## US-2.9.2.2 See Background Blur That Isolates the Focused Subject

**As a** player (P-23), **I want** near and far field depth-of-field blur during dialogue and
cinematics, **so that** the focused character stands out clearly against a softly blurred
background.

## US-2.9.2.3 Test DOF Fallback to Gaussian on Switch and Mobile

**As an** engine tester (P-27), **I want** to verify that Switch uses simplified separable DOF
at half-res and mobile optionally uses lightweight Gaussian DOF at quarter-res, **so that** DOF
effects remain available on lower platforms.

## US-2.9.3.1 See Camera and Object Motion Blur During Fast Movement

**As a** player (P-23), **I want** per-pixel velocity-based motion blur that simulates camera
shutter exposure during fast camera pans and character movement, **so that** motion feels smooth
and cinematic rather than stroboscopic.

## US-2.9.3.2 Validate Motion Blur Sample Count Per Platform

**As an** engine tester (P-27), **I want** to verify that mobile disables motion blur entirely,
Switch uses camera-only blur with 4 samples at half-res, and desktop runs full per-pixel blur
with 8 samples, **so that** motion blur cost is bounded per platform.

## US-2.9.4.1 Adapt Scene Exposure Automatically When Exiting a Dark Cave

**As a** player (P-23), **I want** automatic exposure adaptation that gradually brightens the
view when I walk from a dark cave into sunlight, **so that** the transition feels like natural
eye adjustment rather than an instant brightness snap.

## US-2.9.4.2 Profile Luminance Histogram Resolution Per Platform

**As an** engine developer (P-26), **I want** to measure compute cost for luminance histograms
at average-only (mobile), 32-bin (Switch), 64-bin (desktop), and 128-bin (high-end), **so
that** auto exposure runs within budget on each tier.

## US-2.9.5.1 Apply Cinematic Color Grading With LUTs

**As a** technical artist (P-13), **I want** HDR-to-LDR tone mapping with ACES filmic curve
followed by lift/gamma/gain color correction and 3D LUT application, **so that** I can achieve
a specific cinematic look (film-like, desaturated, vibrant) per scene or camera.

## US-2.9.5.2 Preview HDR10 and Dolby Vision Output in the Editor

**As a** technical artist (P-13), **I want** to preview HDR display output with the full
tonemapping and color grading pipeline in the editor viewport, **so that** I can author HDR
content with accurate visual feedback.

## US-2.9.5.3 Validate LUT Size Per Platform

**As an** engine tester (P-27), **I want** to confirm that mobile uses 16x16x16 LUTs, desktop
uses 32x32x32, and high-end uses 64x64x64, **so that** LUT memory scales correctly per
platform.

## US-2.9.6.1 Add Film Grain for Cinematic Texture

**As a** effects artist (P-12), **I want** procedural film grain with configurable intensity,
size, and luminance response, **so that** cutscenes and horror sequences have photographic
texture that enhances atmosphere.

## US-2.9.6.2 Verify Film Grain Is Combined Into Final Composite on Mobile

**As an** engine tester (P-27), **I want** to confirm that film grain is merged into the final
composite pass on mobile to avoid an extra render target write, **so that** grain adds zero
additional bandwidth cost.

## US-2.9.7.1 Simulate Lens Chromatic Aberration at Screen Edges

**As a** effects artist (P-12), **I want** per-channel UV offsets applied radially from screen
center with configurable intensity, **so that** the final image shows subtle RGB fringing that
adds cinematic lens character.

## US-2.9.7.2 Verify Chromatic Aberration Combined With Composite Pass

**As an** engine tester (P-27), **I want** to confirm that chromatic aberration runs as a
single-pass effect combined into the final composite on mobile, **so that** it adds negligible
bandwidth cost.

## US-2.9.8.1 See Lens Flare From the Sun and Bright Explosions

**As a** player (P-23), **I want** image-based lens flare with ghost and halo artifacts from
bright sources, **so that** looking toward the sun or an explosion produces convincing optical
effects.

## US-2.9.8.2 Validate Flare Element Count Per Platform

**As an** engine tester (P-27), **I want** to verify that mobile uses sprite-based flare with
max 2 sources, Switch shows 2 ghost layers with 4 sources, and desktop runs the full ghost/halo
chain, **so that** flare complexity scales per platform.

## US-2.9.9.1 Add Edge Vignette for Cinematic Framing

**As a** effects artist (P-12), **I want** radial edge darkening with a configurable power
curve, **so that** the player's eye is drawn toward the center of the frame for cinematic
polish.

## US-2.9.9.2 Verify Vignette Negligible Cost on All Platforms

**As an** engine tester (P-27), **I want** to confirm that vignette is combined into the final
composite pass on mobile with negligible additional GPU cost, **so that** the effect is free to
use on every platform.

## US-2.9.10.1 Create Custom Outline and Distortion Post-Process Effects

**As a** technical artist (P-13), **I want** to author custom fullscreen post-process materials
that read scene textures (depth, stencil, normals, velocity) and output arbitrary color
transformations, **so that** I can implement game-specific outlines, color distortion, and
stylization without engine code changes.

## US-2.9.10.2 Validate Post-Process Material Scene Input Access Per Platform

**As an** engine tester (P-27), **I want** to confirm that mobile allows depth-only input with
max 2 custom passes, Switch allows depth+normal with 4 passes, and desktop has unlimited passes
with all scene inputs, **so that** custom post-process materials respect platform limits.

## US-2.9.11.1 Preserve Detail in High-Contrast Scenes With Local Exposure

**As a** environment artist (P-8), **I want** per-region exposure adjustment using a
downsampled luminance grid, **so that** a scene with a bright exterior visible through a dark
interior preserves detail in both regions simultaneously.

## US-2.9.11.2 Validate Local Exposure Tile Grid Resolution

**As an** engine tester (P-27), **I want** to verify that Switch uses 4x4 tile grid, desktop
uses 16x16, and high-end uses 32x32 with smooth inter-tile blending, **so that** local exposure
granularity matches platform capability.

## US-2.9.12.1 Correct Wide-FOV Distortion With Panini Projection

**As a** game developer (P-15), **I want** panini projection post-processing to reduce
straight-line warping at wide field-of-view angles, **so that** first-person games at 110+ FOV
look natural at screen edges while preserving center fidelity.

## US-2.9.12.2 Verify Panini Projection Has Negligible GPU Cost

**As an** engine tester (P-27), **I want** to confirm that panini projection runs as a
lightweight single-pass UV remap with negligible cost on all platforms, **so that** it can be
enabled without performance concerns.

## US-2.9.13.1 Enable Cavity to Enhance Character Wrinkle Detail

**As an** environment artist (P-8), **I want** to enable screen-space cavity/curvature on a
character close-up so that wrinkles, pores, and skin folds are darkened in concavities and
brightened on ridges, **so that** fine surface detail is visible without additional geometry
or texture work.

## US-2.9.13.2 Adjust Cavity Radius to Match Scene Scale

**As an** environment artist (P-8), **I want** to adjust the cavity sample radius (in world
units) so the effect matches my scene's scale -- small radius for character skin detail,
larger radius for architectural crevices, **so that** the darkening and brightening appear at
the correct spatial frequency for the content.

## US-2.9.13.3 Use Debug View to Tune Cavity Intensity

**As a** technical artist (P-13), **I want** a "cavity only" debug view that isolates the
cavity map as a grayscale overlay, **so that** I can precisely tune ridge brightness and
valley darkness intensity without guessing how the effect interacts with scene lighting.

## US-2.9.13.4 See Enhanced Surface Detail on Vehicles and Armor

**As a** player (P-23), **I want** screen-space cavity to reveal panel lines on vehicles,
seams between armor plates, and fabric folds on characters, **so that** surfaces look more
detailed and three-dimensional without impacting frame rate.

## US-2.9.13.5 Implement the Cavity Compute Shader

**As an** engine developer (P-26), **I want** to implement the screen-space cavity effect as
a compute shader that samples the normal buffer at configurable pixel-radius and world-radius
offsets, computes per-pixel normal differences, and outputs a single-channel cavity map
composited multiplicatively over the lit scene, **so that** the effect integrates cleanly
into the post-processing pipeline.

## US-2.9.13.6 Benchmark Cavity at Different Sample Counts

**As an** engine tester (P-27), **I want** to benchmark the cavity effect at 4, 8, 16, and
32 samples at 1080p on reference hardware and verify the 16-sample configuration completes
in under 0.5ms, **so that** the default desktop quality tier stays within the per-effect
budget.

## US-2.9.13.7 Verify Cavity Disabled on Mobile by Default

**As an** engine tester (P-27), **I want** to confirm that the cavity effect is disabled by
default on mobile platforms (since the normal buffer is expensive on TBDR GPUs) and that
enabling it falls back to curvature-only mode at half-res with 4 samples, **so that** mobile
performance is protected.

## US-2.9.14.1 Create a Custom Color Grading Effect With the Graph Editor

**As a** technical artist (P-13), **I want** to build a custom color grading post-process by
connecting Scene Color input through Curves, Channel Mixer, and LUT Apply nodes to a Final
Color output in the post-process graph editor, **so that** I can author a unique color look
without writing any shader code.

## US-2.9.14.2 Expose Parameters for Per-Camera Adjustment

**As a** technical artist (P-13), **I want** to mark graph node inputs as "exposed
parameters" so they appear in the inspector with per-camera or per-volume overrides, **so
that** I can reuse the same post-process graph with different settings for gameplay cameras,
cinematic cameras, and security-camera views.

## US-2.9.14.3 Add a Painterly Edge-Detection Outline as a Post-Process

**As an** environment artist (P-8), **I want** to create a stylized outline effect by
connecting Scene Depth and Scene Normals through Sobel Edge and Threshold nodes, blending
the result over Scene Color, **so that** the game has a hand-drawn visual style without
modifying any materials.

## US-2.9.14.4 Preview a Stylized Post-Process Look in the Viewport

**As a** creative director (P-2), **I want** to see custom post-process graphs applied live
in the editor viewport with immediate visual feedback as nodes are connected and parameters
are adjusted, **so that** I can direct the visual style interactively without waiting for
builds.

## US-2.9.14.5 Inject a Custom HLSL Snippet Node for a Game-Specific Effect

**As a** game developer (P-15), **I want** to insert a Custom HLSL Snippet node into a
post-process graph to implement a proprietary screen-space effect (e.g., radar pulse, damage
vignette, scanner overlay), **so that** I can combine visual scripting convenience with
low-level shader control for game-specific needs.

## US-2.9.14.6 Compile the Post-Process Graph to Render Graph Passes

**As an** engine developer (P-26), **I want** the post-process graph compiler to produce an
optimized sequence of GPU compute dispatches with dead-node elimination, constant folding,
and pass merging, inserted into the render graph at the configured injection point, **so
that** custom post-process graphs execute efficiently alongside built-in effects.

## US-2.9.14.7 Verify Hot-Reload of Custom Post-Process Graphs

**As an** engine tester (P-27), **I want** to modify a post-process graph asset on disk and
verify that the running engine hot-reloads the changes within one frame without a restart,
**so that** artists get instant iteration on post-process effects.

## US-2.9.14.8 Verify Mobile Cap of 4 Simultaneous Custom Passes

**As an** engine tester (P-27), **I want** to apply 5 custom post-process graphs on a mobile
device and verify that only the first 4 execute while the 5th is skipped with a warning log,
**so that** mobile GPU budgets are protected from unbounded custom post-processing.

## US-2.9.14.9 Toggle Custom Post-Process Effects Per Camera for Split-Screen

**As a** game designer (P-5), **I want** to enable or disable specific custom post-process
graphs per camera, **so that** in split-screen multiplayer each player's viewport can have
different visual treatments (e.g., a poisoned player sees a green tint while others do not).
