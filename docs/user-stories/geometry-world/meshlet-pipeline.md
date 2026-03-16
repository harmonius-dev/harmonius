# User Stories — 3.1 Meshlet Pipeline

## US-3.1.1 Import Geometry as Meshlets

**As a** technical artist, **I want** imported meshes to be automatically decomposed into meshlet
DAGs with bounding metadata, **so that** the GPU-driven pipeline can perform fine-grained LOD
selection and culling without manual LOD authoring.

## US-3.1.2 Eliminate Disocclusion Artifacts

**As a** player, **I want** objects hidden behind large occluders to appear instantly when the
occluder moves, **so that** I never see one-frame pop-in artifacts during gameplay.

## US-3.1.3 Cull Invisible Geometry Automatically

**As a** technical artist, **I want** per-meshlet and per-triangle culling to discard backfacing,
frustum-outside, and sub-pixel geometry, **so that** draw cost scales with visible complexity rather
than total scene complexity.

## US-3.1.4 Support Hardware Without Mesh Shaders

**As a** player, **I want** the engine to fall back to indirect draw when my GPU lacks mesh shader
support, **so that** I get GPU-driven culling benefits on older hardware.

## US-3.1.5 Smooth LOD Transitions

**As a** player, **I want** meshes to transition smoothly between detail levels using screen-space
error thresholds with dithered crossfade, **so that** I never see abrupt LOD pops as I move through
the world.

## US-3.1.6 Stream Meshlet Data On Demand

**As a** level designer, **I want** meshlet pages to stream from disk based on camera proximity and
screen-space contribution, **so that** I can build worlds with virtually unlimited geometry without
hitting memory limits.

## US-3.1.7 Defer Material Shading to Visible Pixels

**As a** technical artist, **I want** a visibility buffer that defers material evaluation to a
fullscreen compute pass, **so that** only visible pixels are shaded and the renderer scales to
millions of unique materials.

## US-3.1.8 Maintain Frame Budget Under Dense Geometry

**As a** player, **I want** the meshlet pipeline to keep frame time within budget even in scenes
with millions of triangles, **so that** I experience consistent frame rates in dense open-world
areas.
