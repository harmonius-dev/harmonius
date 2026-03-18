# User Stories — 3.1 Meshlet Pipeline

## Stories

| ID       | Persona          | Features | Requirements |
|----------|------------------|----------|--------------|
| US-3.1.1 | technical artist |          |              |
| US-3.1.2 | player           |          |              |
| US-3.1.3 | technical artist |          |              |
| US-3.1.4 | player           |          |              |
| US-3.1.5 | player           |          |              |
| US-3.1.6 | level designer   |          |              |
| US-3.1.7 | technical artist |          |              |
| US-3.1.8 | player           |          |              |

1. **US-3.1.1** — I want imported meshes to be automatically decomposed into meshlet DAGs with
   bounding metadata
   - **Acceptance:** the GPU-driven pipeline can perform fine-grained LOD selection and culling
     without manual LOD authoring
2. **US-3.1.2** — I want objects hidden behind large occluders to appear instantly when the occluder
   moves
   - **Acceptance:** I never see one-frame pop-in artifacts during gameplay
3. **US-3.1.3** — I want per-meshlet and per-triangle culling to discard backfacing,
   frustum-outside, and sub-pixel geometry
   - **Acceptance:** draw cost scales with visible complexity rather than total scene complexity
4. **US-3.1.4** — I want the engine to fall back to indirect draw when my GPU lacks mesh shader
   support
   - **Acceptance:** I get GPU-driven culling benefits on older hardware
5. **US-3.1.5** — I want meshes to transition smoothly between detail levels using screen-space
   error thresholds with dithered crossfade
   - **Acceptance:** I never see abrupt LOD pops as I move through the world
6. **US-3.1.6** — I want meshlet pages to stream from disk based on camera proximity and
   screen-space contribution
   - **Acceptance:** I can build worlds with virtually unlimited geometry without hitting memory
     limits
7. **US-3.1.7** — I want a visibility buffer that defers material evaluation to a fullscreen compute
   pass
   - **Acceptance:** only visible pixels are shaded and the renderer scales to millions of unique
     materials
8. **US-3.1.8** — I want the meshlet pipeline to keep frame time within budget even in scenes with
   millions of triangles
   - **Acceptance:** I experience consistent frame rates in dense open-world areas
