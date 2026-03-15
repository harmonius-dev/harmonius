# User Stories -- 12.6 DCC Tool Plugins

## Plugin SDK (F-12.6.1)

## US-12.6.1 Export from Any DCC Tool via the Plugin SDK

**As a** designer (P-5), **I want** a C API plugin SDK with Python and C++ bindings that
DCC plugins use to export meshes, skeletons, animations, materials, and scenes directly to
the engine's native binary format, **so that** I can push assets from any supported tool
without intermediate format conversion.

## US-12.6.2 Extend the Plugin SDK for Custom DCC Tools

**As an** engine developer (P-26), **I want** the plugin SDK to ship as a shared library
(.dll/.dylib/.so) loadable by any DCC host, with versioning, compression, and transport
over local socket or shared memory, **so that** third-party DCC integrations can be built
without modifying the engine.

## Live Link (F-12.6.2)

## US-12.6.3 See Changes in the Engine Viewport Within Seconds

**As a** designer (P-5), **I want** a live link connection between my DCC tool and the
running engine that pushes mesh, material, and animation changes in real time, **so that**
I can iterate on assets with sub-second feedback without full re-export.

## US-12.6.4 Synchronize Viewports Between DCC Tool and Engine

**As a** designer (P-5), **I want** the engine to send camera position and selection state
back to my DCC tool via live link, **so that** both viewports stay synchronized and I can
navigate the scene from either application.

## US-12.6.5 Verify Live Link Latency Under 2 Seconds

**As an** engine tester (P-27), **I want** latency tests that measure the time from DCC tool
edit to engine viewport update via live link, verifying it stays under 2 seconds, **so
that** the interactive iteration target is validated.

## Houdini (F-12.6.3, F-12.6.4, F-12.6.5)

## US-12.6.6 Use Houdini Procedural Graphs in the Engine

**As a** designer (P-5), **I want** Houdini Digital Assets (HDAs) to run within the engine
editor with exposed parameters in the inspector and cook-on-parameter-change, **so that**
I can use Houdini procedural workflows interactively without switching applications.

## US-12.6.7 Export Meshes and Scenes from Houdini

**As a** designer (P-5), **I want** to export meshes with all vertex attributes, skeletons,
skin weights, morph targets, packed primitives as instances, curves as splines, and volumes
as 3D textures from Houdini, **so that** all Houdini output types transfer to the engine
without data loss.

## US-12.6.8 Scatter Vegetation and Props with Houdini

**As a** designer (P-5), **I want** Houdini scatter and placement nodes to output point
clouds with instance attributes that the engine converts to ECS entities via the PCG
pipeline, **so that** I can author art-directed placement in Houdini's procedural
environment.

## US-12.6.9 Verify Houdini Engine HDA Output Matches Reference

**As an** engine tester (P-27), **I want** automated tests that cook reference HDAs and
compare the output geometry, instances, and attributes against golden reference assets,
**so that** Houdini Engine integration regressions are caught in CI.

## Maya (F-12.6.6, F-12.6.7)

## US-12.6.10 Export Meshes and Animations from Maya

**As a** designer (P-5), **I want** a Maya plugin that exports meshes, skeletons,
animations, blend shapes, materials, cameras, and lights with incremental export of only
changed nodes, **so that** I can push Maya assets to the engine without full scene export.

## US-12.6.11 Export Full Animation Curves from Maya

**As a** designer (P-5), **I want** animation export with full curve data (Bezier tangents,
step/linear/cubic interpolation), skin weights, IK metadata, and character rig bundles,
**so that** Maya animation data transfers to the engine without quality loss from baked
per-frame samples.

## US-12.6.12 Verify Maya Plugin Exports Match Reference Assets

**As an** engine tester (P-27), **I want** automated tests that export reference Maya scenes
and compare the imported result against golden assets, verifying mesh data, skeleton
hierarchy, animation curves, and material properties, **so that** Maya plugin regressions
are caught in CI.

## Blender (F-12.6.8, F-12.6.9)

## US-12.6.13 Export Scenes from Blender with Material Conversion

**As a** designer (P-5), **I want** a Blender addon that exports meshes, armatures, shape
keys, animations, materials (Principled BSDF mapped to engine PBR), and collections as
prefab hierarchies, **so that** Blender users have a first-class export experience.

## US-12.6.14 Verify Blender Material Conversion Handles Common Setups

**As an** engine tester (P-27), **I want** tests that convert Principled BSDF node trees
with varying input configurations (all texture inputs, mixed constant/texture, custom
nodes) and verify correct engine material output, **so that** Blender material conversion
is reliable for common authoring patterns.

## US-12.6.15 Apply Blender Modifiers at Export with Pre-Modifier Option

**As a** game developer (P-15), **I want** Blender's modifier stack applied at export time
with an option to export pre-modifier data for engine-side processing, **so that** artists
can choose whether to bake modifiers or defer them to the engine.

## Marvelous Designer (F-12.6.10, F-12.6.11)

## US-12.6.16 Export Clothing from Marvelous Designer

**As a** designer (P-5), **I want** to export garment meshes, UV layouts, seam data, and
fabric properties from Marvelous Designer with automatic skeleton binding, **so that**
cloth assets are ready for runtime simulation without manual skinning.

## US-12.6.17 Fit Garments to Characters with Different Body Shapes

**As a** designer (P-5), **I want** automatic garment-to-character fitting that computes
cloth attachment constraints and skin weight transfers for varying body proportions, **so
that** garments deform correctly with character body morph changes.

## US-12.6.18 Verify Garment Fitting Produces Valid Simulation Constraints

**As an** engine tester (P-27), **I want** tests that fit reference garments to characters
with varying body morphs and verify the cloth simulation constraints and collision bodies
are generated correctly, **so that** garment fitting does not produce simulation artifacts.

## Substance (F-12.6.12, F-12.6.13, F-12.6.14)

## US-12.6.19 Import Substance Materials with Adjustable Parameters

**As a** designer (P-5), **I want** to import .sbsar Substance materials and adjust exposed
parameters (color tint, wear, dirt) in the engine material editor without re-exporting from
Substance Designer, **so that** I can tune procedural materials in-engine.

## US-12.6.20 Import Substance Painter Texture Sets

**As a** designer (P-5), **I want** Substance Painter project import that extracts channel-
packed texture maps matching engine material conventions with UDIM support, **so that**
painted texture sets transfer to the engine correctly.

## US-12.6.21 Evaluate Substance Materials at Runtime

**As a** game developer (P-15), **I want** optional runtime .sbsar evaluation on a
background thread for dynamic material variation (weathering by region, damage response),
**so that** procedural materials can adapt to gameplay context with budgeted GPU cost.

## US-12.6.22 Verify Substance Material Parameter Changes Produce Correct Output

**As an** engine tester (P-27), **I want** tests that import reference .sbsar materials,
vary exposed parameters, and compare generated texture sets against baseline images, **so
that** Substance integration correctness is validated.

## Photoshop (F-12.6.15, F-12.6.16)

## US-12.6.23 Export Textures and UI Layouts from Photoshop

**As a** designer (P-5), **I want** a Photoshop plugin that exports textures with channel
packing, supports PSD round-trip re-export, live link, and layer-group-to-UI-widget
conversion, **so that** I can iterate on textures and UI mockups in Photoshop with instant
engine feedback.

## US-12.6.24 Convert Photoshop Layers to UI Widget Hierarchies

**As a** designer (P-5), **I want** Photoshop layer groups exported as engine UI widgets
with text properties, shape nine-slices, and visibility state mapping, **so that** UI
mockups designed in Photoshop transfer to the engine without manual recreation.

## US-12.6.25 Verify Photoshop Layer-to-UI Conversion

**As an** engine tester (P-27), **I want** tests that export reference Photoshop files with
text layers, shape layers, and visibility states, and verify the generated UI widget tree
matches the expected structure, **so that** Photoshop-to-UI conversion is reliable.

## Illustrator (F-12.6.17)

## US-12.6.26 Export Vector Assets from Illustrator

**As a** designer (P-5), **I want** Illustrator vector artwork exported as resolution-
independent vector graphics or rasterized atlases, with artboard-to-asset and symbol-to-
instance mapping, **so that** icons, HUD elements, and decals render crisply at any scale.

## US-12.6.27 Verify Illustrator Vector Export Fidelity

**As an** engine tester (P-27), **I want** tests that export reference Illustrator files
and verify vector paths, color swatches, and symbol instances convert correctly to the
engine's vector format, **so that** Illustrator export fidelity is validated.

## ZBrush (F-12.6.18, F-12.6.19)

## US-12.6.28 Export High-Poly Sculpts from ZBrush

**As a** designer (P-5), **I want** ZBrush sculpts exported with baked normal, displacement,
and cavity maps projected onto a low-poly mesh, polypaint as vertex colors or texture, and
multi-SubTool hierarchy, **so that** high-poly sculpt detail transfers to engine-ready
assets.

## US-12.6.29 Generate LODs from ZBrush Sculpts

**As a** designer (P-5), **I want** automatic decimation of ZBrush sculpts to engine-ready
poly counts with per-LOD normal map bakes, **so that** a single sculpt produces a complete
LOD chain without manual decimation.

## US-12.6.30 Verify ZBrush Normal Map Bake Quality

**As an** engine tester (P-27), **I want** tests that export reference ZBrush sculpts and
compare baked normal maps against golden references (PSNR thresholds), **so that** the
high-poly-to-low-poly bake pipeline produces visually correct results.

## MotionBuilder (F-12.6.20, F-12.6.21)

## US-12.6.31 Export Motion Capture Data from MotionBuilder

**As a** designer (P-5), **I want** mocap data, cleaned animations, and character rigs
exported from MotionBuilder with full curve data and batch export of multiple takes, **so
that** motion capture sessions transfer to the engine efficiently.

## US-12.6.32 Stream Live Mocap to the Engine for Preview

**As a** designer (P-5), **I want** real-time mocap streaming from MotionBuilder applied to
engine characters with full materials and lighting, **so that** directors see capture
results on final characters during recording sessions.

## US-12.6.33 Verify MotionBuilder Curve Export Matches Source Data

**As an** engine tester (P-27), **I want** tests that export reference animation takes from
MotionBuilder and verify the imported curve data (keyframes, tangents, interpolation modes)
matches the source within floating-point tolerance, **so that** animation fidelity is
preserved.

## Git Integration (F-12.6.22, F-12.6.23, F-12.6.24)

## US-12.6.34 Lock Source Files Automatically When Editing

**As a** designer (P-5), **I want** DCC plugins to automatically acquire Git LFS locks on
source files when I open them and release locks on save and close, **so that** my team
avoids merge conflicts on binary source files.

## US-12.6.35 View Review Comments Inside My DCC Tool

**As a** designer (P-5), **I want** review comments from the engine editor and pull requests
displayed in a panel within my DCC tool with reply and resolve actions, **so that** I can
respond to feedback without switching applications.

## US-12.6.36 See Git Status from Any DCC Tool

**As a** designer (P-5), **I want** a consistent status dashboard in every DCC plugin
showing branch, lock status, pending changes, review comments, and one-click commit, push,
pull, lock, unlock, and PR actions, **so that** I have the same version control experience
regardless of which tool I am using.

## US-12.6.37 Verify Git LFS Lock Workflow in Multi-User Scenarios

**As an** engine tester (P-27), **I want** tests that simulate two users attempting to lock
the same file and verify correct lock acquisition, conflict notification, and lock release
behavior, **so that** the collaborative locking workflow is reliable.

## US-12.6.38 Verify Review Comments Sync Across DCC Plugins

**As an** engine tester (P-27), **I want** tests that post a review comment in the engine
editor and verify it appears in the DCC plugin panel within the sync interval, **so that**
cross-tool comment synchronization is reliable.

## Shared Features (F-12.6.25, F-12.6.26)

## US-12.6.39 Map Materials Across DCC Tools Consistently

**As an** engine developer (P-26), **I want** a configurable material translation layer that
maps each DCC tool's material system to the engine's PBR model with fallback defaults for
unmapped nodes, **so that** material import works consistently regardless of the source tool.

## US-12.6.40 Run DCC Exports in CI Headless Mode

**As a** DevOps engineer (P-16), **I want** all DCC plugins to support headless batch export
via command-line invocation (hython, mayapy, blender --background), **so that** CI pipelines
re-export modified source files and push them through the asset processing pipeline
automatically.

## US-12.6.41 Verify Headless Batch Export Produces Identical Output

**As an** engine tester (P-27), **I want** tests that compare headless batch export output
against interactive export output for the same source files and verify byte-identical
results, **so that** CI builds produce the same assets as artist workstations.

## US-12.6.42 Verify DCC-Agnostic Material Mapping Consistency

**As an** engine tester (P-27), **I want** tests that import the same material from Maya,
Blender, and Substance using the material mapping table and verify the engine output matches
within tolerance, **so that** the DCC-agnostic mapping produces consistent results.

## US-12.6.43 Manage DCC Tool Licenses in CI

**As a** DevOps engineer (P-16), **I want** CI build manifests to track which DCC tools and
license types (Houdini Engine, Maya Batch) are required per source file, **so that** build
machines have the correct licenses provisioned for headless export.
