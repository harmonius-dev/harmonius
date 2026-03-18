# User Stories -- 2.9 Post-Processing

## Stories

| ID          | Persona                  | Features | Requirements |
|-------------|--------------------------|----------|--------------|
| US-2.9.1.1  | environment artist (P-8) |          |              |
| US-2.9.1.2  | engine tester (P-27)     |          |              |
| US-2.9.2.1  | effects artist (P-12)    |          |              |
| US-2.9.2.2  | player (P-23)            |          |              |
| US-2.9.2.3  | engine tester (P-27)     |          |              |
| US-2.9.3.1  | player (P-23)            |          |              |
| US-2.9.3.2  | engine tester (P-27)     |          |              |
| US-2.9.4.1  | player (P-23)            |          |              |
| US-2.9.4.2  | engine developer (P-26)  |          |              |
| US-2.9.5.1  | technical artist (P-13)  |          |              |
| US-2.9.5.2  | technical artist (P-13)  |          |              |
| US-2.9.5.3  | engine tester (P-27)     |          |              |
| US-2.9.6.1  | effects artist (P-12)    |          |              |
| US-2.9.6.2  | engine tester (P-27)     |          |              |
| US-2.9.7.1  | effects artist (P-12)    |          |              |
| US-2.9.7.2  | engine tester (P-27)     |          |              |
| US-2.9.8.1  | player (P-23)            |          |              |
| US-2.9.8.2  | engine tester (P-27)     |          |              |
| US-2.9.9.1  | effects artist (P-12)    |          |              |
| US-2.9.9.2  | engine tester (P-27)     |          |              |
| US-2.9.10.1 | technical artist (P-13)  |          |              |
| US-2.9.10.2 | engine tester (P-27)     |          |              |
| US-2.9.11.1 | environment artist (P-8) |          |              |
| US-2.9.11.2 | engine tester (P-27)     |          |              |
| US-2.9.12.1 | game developer (P-15)    |          |              |
| US-2.9.12.2 | engine tester (P-27)     |          |              |
| US-2.9.13.1 | environment artist (P-8) |          |              |
| US-2.9.13.2 | environment artist (P-8) |          |              |
| US-2.9.13.3 | technical artist (P-13)  |          |              |
| US-2.9.13.4 | player (P-23)            |          |              |
| US-2.9.13.5 | engine developer (P-26)  |          |              |
| US-2.9.13.6 | engine tester (P-27)     |          |              |
| US-2.9.13.7 | engine tester (P-27)     |          |              |
| US-2.9.14.1 | technical artist (P-13)  |          |              |
| US-2.9.14.2 | technical artist (P-13)  |          |              |
| US-2.9.14.3 | environment artist (P-8) |          |              |
| US-2.9.14.4 | creative director (P-2)  |          |              |
| US-2.9.14.5 | game developer (P-15)    |          |              |
| US-2.9.14.6 | engine developer (P-26)  |          |              |
| US-2.9.14.7 | engine tester (P-27)     |          |              |
| US-2.9.14.8 | engine tester (P-27)     |          |              |
| US-2.9.14.9 | game designer (P-5)      |          |              |

1. **US-2.9.1.1** — I want bright-source bloom with a multi-pass blur chain that extracts pixels
   above a luminance threshold and composites the glow additively
   - **Acceptance:** neon signs, magic effects, and sunlit surfaces produce a cinematic light bleed
2. **US-2.9.1.2** — I want to verify that mobile uses 3-pass dual-kawase at quarter-res, Switch uses
   4-pass Gaussian at half-res, and desktop runs 6+ iterations
   - **Acceptance:** bloom cost scales correctly per platform
3. **US-2.9.2.1** — I want depth of field driven by real camera parameters (aperture, focal length,
   focus distance) with gather-based circular bokeh
   - **Acceptance:** cutscenes have cinematic focus pulls with correctly shaped bokeh highlights
4. **US-2.9.2.2** — I want near and far field depth-of-field blur during dialogue and cinematics
   - **Acceptance:** the focused character stands out clearly against a softly blurred background
5. **US-2.9.2.3** — I want to verify that Switch uses simplified separable DOF at half-res and
   mobile optionally uses lightweight Gaussian DOF at quarter-res
   - **Acceptance:** DOF effects remain available on lower platforms
6. **US-2.9.3.1** — I want per-pixel velocity-based motion blur that simulates camera shutter
   exposure during fast camera pans and character movement
   - **Acceptance:** motion feels smooth and cinematic rather than stroboscopic
7. **US-2.9.3.2** — I want to verify that mobile disables motion blur entirely, Switch uses
   camera-only blur with 4 samples at half-res, and desktop runs full per-pixel blur with 8 samples
   - **Acceptance:** motion blur cost is bounded per platform
8. **US-2.9.4.1** — I want automatic exposure adaptation that gradually brightens the view when I
   walk from a dark cave into sunlight
   - **Acceptance:** the transition feels like natural eye adjustment rather than an instant
     brightness snap
9. **US-2.9.4.2** — I want to measure compute cost for luminance histograms at average-only
   (mobile), 32-bin (Switch), 64-bin (desktop), and 128-bin (high-end)
   - **Acceptance:** auto exposure runs within budget on each tier
10. **US-2.9.5.1** — I want HDR-to-LDR tone mapping with ACES filmic curve followed by
    lift/gamma/gain color correction and 3D LUT application
    - **Acceptance:** I can achieve a specific cinematic look (film-like, desaturated, vibrant) per
      scene or camera
11. **US-2.9.5.2** — I want to preview HDR display output with the full tonemapping and color
    grading pipeline in the editor viewport
    - **Acceptance:** I can author HDR content with accurate visual feedback
12. **US-2.9.5.3** — I want to confirm that mobile uses 16x16x16 LUTs, desktop uses 32x32x32, and
    high-end uses 64x64x64
    - **Acceptance:** LUT memory scales correctly per platform
13. **US-2.9.6.1** — I want procedural film grain with configurable intensity, size, and luminance
    response
    - **Acceptance:** cutscenes and horror sequences have photographic texture that enhances
      atmosphere
14. **US-2.9.6.2** — I want to confirm that film grain is merged into the final composite pass on
    mobile to avoid an extra render target write
    - **Acceptance:** grain adds zero additional bandwidth cost
15. **US-2.9.7.1** — I want per-channel UV offsets applied radially from screen center with
    configurable intensity
    - **Acceptance:** the final image shows subtle RGB fringing that adds cinematic lens character
16. **US-2.9.7.2** — I want to confirm that chromatic aberration runs as a single-pass effect
    combined into the final composite on mobile
    - **Acceptance:** it adds negligible bandwidth cost
17. **US-2.9.8.1** — I want image-based lens flare with ghost and halo artifacts from bright sources
    - **Acceptance:** looking toward the sun or an explosion produces convincing optical effects
18. **US-2.9.8.2** — I want to verify that mobile uses sprite-based flare with max 2 sources, Switch
    shows 2 ghost layers with 4 sources, and desktop runs the full ghost/halo chain
    - **Acceptance:** flare complexity scales per platform
19. **US-2.9.9.1** — I want radial edge darkening with a configurable power curve
    - **Acceptance:** the player's eye is drawn toward the center of the frame for cinematic polish
20. **US-2.9.9.2** — I want to confirm that vignette is combined into the final composite pass on
    mobile with negligible additional GPU cost
    - **Acceptance:** the effect is free to use on every platform
21. **US-2.9.10.1** — I want to author custom fullscreen post-process materials that read scene
    textures (depth, stencil, normals, velocity) and output arbitrary color transformations
    - **Acceptance:** I can implement game-specific outlines, color distortion, and stylization
      without engine code changes
22. **US-2.9.10.2** — I want to confirm that mobile allows depth-only input with max 2 custom
    passes, Switch allows depth+normal with 4 passes, and desktop has unlimited passes with all
    scene inputs
    - **Acceptance:** custom post-process materials respect platform limits
23. **US-2.9.11.1** — I want per-region exposure adjustment using a downsampled luminance grid
    - **Acceptance:** a scene with a bright exterior visible through a dark interior preserves
      detail in both regions simultaneously
24. **US-2.9.11.2** — I want to verify that Switch uses 4x4 tile grid, desktop uses 16x16, and
    high-end uses 32x32 with smooth inter-tile blending
    - **Acceptance:** local exposure granularity matches platform capability
25. **US-2.9.12.1** — I want panini projection post-processing to reduce straight-line warping at
    wide field-of-view angles
    - **Acceptance:** first-person games at 110+ FOV look natural at screen edges while preserving
      center fidelity
26. **US-2.9.12.2** — I want to confirm that panini projection runs as a lightweight single-pass UV
    remap with negligible cost on all platforms
    - **Acceptance:** it can be enabled without performance concerns
27. **US-2.9.13.1** — I want to enable screen-space cavity/curvature on a character close-up so that
    wrinkles, pores, and skin folds are darkened in concavities and brightened on ridges
    - **Acceptance:** fine surface detail is visible without additional geometry or texture work
28. **US-2.9.13.2** — I want to adjust the cavity sample radius (in world units) so the effect
    matches my scene's scale -- small radius for character skin detail, larger radius for
    architectural crevices
    - **Acceptance:** the darkening and brightening appear at the correct spatial frequency for the
      content
29. **US-2.9.13.3** — I want a "cavity only" debug view that isolates the cavity map as a grayscale
    overlay
    - **Acceptance:** I can precisely tune ridge brightness and valley darkness intensity without
      guessing how the effect interacts with scene lighting
30. **US-2.9.13.4** — I want screen-space cavity to reveal panel lines on vehicles, seams between
    armor plates, and fabric folds on characters
    - **Acceptance:** surfaces look more detailed and three-dimensional without impacting frame rate
31. **US-2.9.13.5** — I want to implement the screen-space cavity effect as a compute shader that
    samples the normal buffer at configurable pixel-radius and world-radius offsets, computes
    per-pixel normal differences, and outputs a single-channel cavity map composited
    multiplicatively over the lit scene
    - **Acceptance:** the effect integrates cleanly into the post-processing pipeline
32. **US-2.9.13.6** — I want to benchmark the cavity effect at 4, 8, 16, and 32 samples at 1080p on
    reference hardware and verify the 16-sample configuration completes in under 0.5ms
    - **Acceptance:** the default desktop quality tier stays within the per-effect budget
33. **US-2.9.13.7** — I want to confirm that the cavity effect is disabled by default on mobile
    platforms (since the normal buffer is expensive on TBDR GPUs) and that enabling it falls back to
    curvature-only mode at half-res with 4 samples
    - **Acceptance:** mobile performance is protected
34. **US-2.9.14.1** — I want to build a custom color grading post-process by connecting Scene Color
    input through Curves, Channel Mixer, and LUT Apply nodes to a Final Color output in the
    post-process graph editor
    - **Acceptance:** I can author a unique color look without writing any shader code
35. **US-2.9.14.2** — I want to mark graph node inputs as "exposed parameters" so they appear in the
    inspector with per-camera or per-volume overrides
    - **Acceptance:** I can reuse the same post-process graph with different settings for gameplay
      cameras, cinematic cameras, and security-camera views
36. **US-2.9.14.3** — I want to create a stylized outline effect by connecting Scene Depth and Scene
    Normals through Sobel Edge and Threshold nodes, blending the result over Scene Color
    - **Acceptance:** the game has a hand-drawn visual style without modifying any materials
37. **US-2.9.14.4** — I want to see custom post-process graphs applied live in the editor viewport
    with immediate visual feedback as nodes are connected and parameters are adjusted
    - **Acceptance:** I can direct the visual style interactively without waiting for builds
38. **US-2.9.14.5** — I want to insert a Custom HLSL Snippet node into a post-process graph to
    implement a proprietary screen-space effect (e.g., radar pulse, damage vignette, scanner
    overlay)
    - **Acceptance:** I can combine visual scripting convenience with low-level shader control for
      game-specific needs
39. **US-2.9.14.6** — I want the post-process graph compiler to produce an optimized sequence of GPU
    compute dispatches with dead-node elimination, constant folding, and pass merging, inserted into
    the render graph at the configured injection point
    - **Acceptance:** custom post-process graphs execute efficiently alongside built-in effects
40. **US-2.9.14.7** — I want to modify a post-process graph asset on disk and verify that the
    running engine hot-reloads the changes within one frame without a restart
    - **Acceptance:** artists get instant iteration on post-process effects
41. **US-2.9.14.8** — I want to apply 5 custom post-process graphs on a mobile device and verify
    that only the first 4 execute while the 5th is skipped with a warning log
    - **Acceptance:** mobile GPU budgets are protected from unbounded custom post-processing
42. **US-2.9.14.9** — I want to enable or disable specific custom post-process graphs per camera
    - **Acceptance:** in split-screen multiplayer each player's viewport can have different visual
      treatments (e.g., a poisoned player sees a green tint while others do not)
