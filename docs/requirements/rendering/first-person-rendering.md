# R-2.13 -- First-Person Rendering Requirements

## Viewmodel Pipeline

1. **R-2.13.1** — The engine **SHALL** render viewmodel meshes in a dedicated layer with its own
   depth buffer, scheduled after the main scene pass via the render graph, ensuring viewmodel always
   renders in front of world geometry.
   - **Rationale:** Separate depth prevents viewmodel clipping against world geometry at any depth.
   - **Verification:** Place a weapon in front of a wall at distance. Verify viewmodel renders in
     front. Verify the render graph schedules the pass after scene.

2. **R-2.13.2** — The engine **SHALL** render viewmodel with an independent FOV and near-clip plane,
   supporting smooth FOV transitions for ADS zoom, sprint widening, and scope magnification.
   - **Rationale:** Independent FOV prevents weapon distortion at wide world FOV settings.
   - **Verification:** Set world FOV to 120. Verify viewmodel is undistorted. Trigger ADS zoom.
     Verify smooth FOV interpolation for both world and viewmodel.

3. **R-2.13.3** — The engine **SHALL** composite viewmodel color and depth over the scene, writing
   viewmodel depth at a fixed near value so DOF and SSAO treat viewmodel as foreground, with correct
   transparent surface blending.
   - **Rationale:** Depth compositing ensures post-processing treats the viewmodel correctly.
   - **Verification:** Enable DOF. Verify viewmodel stays in focus. Render a glass sight. Verify
     scene shows through the transparent surface.

## Lighting and Post-Processing

4. **R-2.13.4** — The engine **SHALL** light the viewmodel from the same lighting environment as the
   world scene, including shadow maps, reflection probes, and dynamic lights near the camera.
   - **Rationale:** Consistent lighting prevents the viewmodel from appearing disconnected from the
     world.
   - **Verification:** Fire a muzzle flash. Verify both viewmodel arms and nearby walls are
     illuminated. Verify viewmodel samples the scene light buffer.

5. **R-2.13.5** — The engine **SHALL** allow per-pass opt-in and opt-out for viewmodel in
   post-processing, controlled by a per-pixel stencil flag.
   - **Rationale:** Viewmodel should skip motion blur and DOF but receive tonemapping and color
     grading.
   - **Verification:** Enable motion blur. Verify viewmodel is excluded via stencil. Verify
     tonemapping is applied to viewmodel.

6. **R-2.13.6** — The engine **SHALL** support viewmodel shadow casting into the world scene and
   viewmodel motion vectors for TAA and frame generation.
   - **Rationale:** Shadows ground the viewmodel; motion vectors prevent TAA ghosting on weapon
     edges.
   - **Verification:** Cast a shadow from viewmodel arms. Verify shadow on the ground. Verify TAA
     handles weapon edges without ghosting.

## Platform Extensions

7. **R-2.13.7** — The engine **SHALL** support VR stereo viewmodel rendering with per-eye near-clip,
   single-pass stereo instancing, and hand tracking integration.
   - **Rationale:** VR viewmodel requires correct stereoscopic depth and tracked hand positioning.
   - **Verification:** Render in VR. Verify weapon at correct stereoscopic depth. Verify hand
     tracking positions the viewmodel mesh.

8. **R-2.13.8** — The engine **SHALL** support stencil-based viewmodel masking for scope lenses,
   body clipping, and VFX occlusion ordering.
   - **Rationale:** Stencil masking enables scope zoom views and correct VFX occlusion against the
     viewmodel.
   - **Verification:** Look through a scope. Verify magnified scene inside the stencil cutout.
     Verify muzzle flash occludes correctly against the weapon mesh.
