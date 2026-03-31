# 2.13 — First-Person Rendering

## Viewmodel Pipeline

| ID      | Feature                         |
|---------|---------------------------------|
| F-2.13.1 | Viewmodel Render Layer        |
| F-2.13.2 | Viewmodel FOV Independence    |
| F-2.13.3 | Viewmodel Depth Compositing   |
| F-2.13.4 | Viewmodel Lighting Integration |

1. **F-2.13.1** — Dedicated render layer for first-person viewmodel meshes (arms, weapons, tools).
   Viewmodel geometry is submitted to a separate render pass that clears its own depth buffer,
   ensuring viewmodel always renders in front of world geometry regardless of world depth values.
   The render graph (F-2.2.1) schedules the viewmodel pass after the main scene pass. Entities opt
   into the viewmodel layer via a `ViewmodelLayer` component.
   - **Deps:** F-2.2.1 (Render Graph), F-2.10.1 (Render Proxy Extraction)
   - **Platform:** All platforms. No tier-specific scaling; viewmodel pass has fixed low cost.
2. **F-2.13.2** — Viewmodel renders with its own projection matrix using a configurable FOV
   independent of the world camera FOV. Prevents weapon distortion at wide world FOVs (e.g.,
   100-120). FOV transitions (ADS zoom, sprint widening, scope magnification) smoothly interpolate
   both world and viewmodel FOVs simultaneously with independent curves and durations. The viewmodel
   projection also uses a separate near-clip plane, allowing arms to render closer to the camera
   than world geometry without clipping.
   - **Deps:** F-2.10.4 (View and Camera Setup), F-9.6.1 (FP Camera Controller)
   - **Platform:** All platforms. Near-clip adjustment critical on mobile where depth precision is
     lower (16-bit depth).
3. **F-2.13.3** — Composites the viewmodel color and depth over the scene output. Viewmodel pixels
   always win over scene pixels, but viewmodel geometry self-occludes correctly using its own depth
   buffer. The compositor writes viewmodel depth into the final depth buffer at a fixed near value
   so that post-processing effects (DOF, SSAO) treat the viewmodel as foreground. Transparent
   viewmodel surfaces (glass sights, holographic elements) blend correctly with the scene behind
   them.
   - **Deps:** F-2.13.1, F-2.13.2
   - **Platform:** All platforms. Mobile uses a single resolve pass to minimize bandwidth.
4. **F-2.13.4** — Viewmodel receives the same lighting environment as the world scene. In deferred
   rendering, the viewmodel forward pass samples the scene's light buffer, shadow maps, and
   reflection probes. In forward rendering, the viewmodel pass binds the same light list.
   Screen-space effects (SSR, SSAO) apply to viewmodel surfaces where visible. Dynamic lights near
   the camera (muzzle flash, flashlight) illuminate both viewmodel and world geometry consistently.
   - **Deps:** F-2.13.1, F-2.5.1 (Punctual Lights), F-2.5.6 (Reflection Probes)
   - **Platform:** Mobile: viewmodel may sample a reduced light count (max 4 dynamic lights) to stay
     within per-pixel ALU budget.

## Post-Processing and Shadows

| ID       | Feature                          |
|----------|----------------------------------|
| F-2.13.5 | Viewmodel Post-Processing Control |
| F-2.13.6 | Viewmodel Shadow Casting         |
| F-2.13.7 | Viewmodel Motion Vectors         |

1. **F-2.13.5** — Per-pass opt-in/out for viewmodel geometry in the post-processing pipeline.
   Viewmodel can skip motion blur (arms moving at input speed would smear), depth of field
   (viewmodel is always in focus), and bloom (prevent weapon glow blowout) while still receiving
   tonemapping, color grading, and film grain. Each post-processing pass checks a per-pixel stencil
   flag to include or exclude viewmodel fragments.
   - **Deps:** F-2.13.3, F-2.9.1 (Post-Processing Pipeline)
   - **Platform:** All platforms. Stencil test adds no measurable cost on modern GPUs.
2. **F-2.13.6** — First-person arms and weapons cast shadows into the world scene. The viewmodel
   depth buffer is projected into shadow map space and merged with world shadow casters.
   Configurable per entity: some viewmodel meshes (e.g., transparent holographic sights) can opt out
   of shadow casting. Shadow resolution for the viewmodel uses the closest cascade.
   - **Deps:** F-2.13.1, F-2.5.2 (Cascaded Shadow Maps)
   - **Platform:** Mobile: viewmodel shadow casting is optional (can be disabled for GPU budget).
     Desktop/console: always enabled.
3. **F-2.13.7** — The viewmodel pass writes per-pixel motion vectors into the motion vector buffer
   using the viewmodel's own projection and transform deltas. This enables TAA to handle viewmodel
   edges correctly and frame generation (F-2.6.6) to reconstruct viewmodel motion independently from
   world motion. Without correct viewmodel motion vectors, weapon edges ghost under TAA.
   - **Deps:** F-2.13.1, F-2.6.1 (TAA)
   - **Platform:** All platforms. Essential for TAA quality on all tiers.

## Platform-Specific Extensions

| ID       | Feature                         |
|----------|---------------------------------|
| F-2.13.8 | VR Stereo Viewmodel Rendering   |
| F-2.13.9 | Viewmodel Stencil Masking       |

1. **F-2.13.8** — In VR, viewmodel geometry renders with a separate near-clip plane per eye to
   prevent hand/weapon intersection with the stereo near plane. The viewmodel projection accounts
   for the per-eye offset so that arms appear at the correct stereoscopic depth. Single-pass stereo
   instancing renders both eye views in one draw call where supported. Hand tracking integration
   positions viewmodel meshes at tracked hand transforms.
   - **Deps:** F-2.13.1, F-2.13.2, F-6.5.1 (VR Input)
   - **Platform:** VR only. Requires stereo instancing support (Vulkan multiview / Metal layer
     rendering).
2. **F-2.13.9** — Stencil-based masking for viewmodel regions. Scope lenses render the magnified
   scene inside a stencil cutout on the viewmodel mesh. Looking down at the character body clips
   viewmodel arms against the body mesh stencil to prevent interpenetration. VFX (muzzle flash,
   shell ejection) can write to the viewmodel stencil layer for correct occlusion ordering.
   - **Deps:** F-2.13.1, F-2.13.3
   - **Platform:** All platforms. Stencil operations are zero additional cost.
