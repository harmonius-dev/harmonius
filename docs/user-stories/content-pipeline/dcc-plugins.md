# User Stories -- 12.6 DCC Tool Plugins

## Plugin SDK (F-12.6.1)

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-12.6.1 | designer (P-5) | a C API plugin SDK with Python and C++ bindings that DCC plugins use to export meshes, skeletons, animations, materials, and scenes directly to the engine's native binary format | I can push assets from any supported tool without intermediate format conversion |  |  |
| US-12.6.2 | engine developer (P-26) | the plugin SDK to ship as a shared library (.dll/.dylib/.so) loadable by any DCC host, with versioning, compression, and transport over local socket or shared memory | third-party DCC integrations can be built without modifying the engine |  |  |

## Live Link (F-12.6.2)

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-12.6.3 | designer (P-5) | a live link connection between my DCC tool and the running engine that pushes mesh, material, and animation changes in real time | I can iterate on assets with sub-second feedback without full re-export |  |  |
| US-12.6.4 | designer (P-5) | the engine to send camera position and selection state back to my DCC tool via live link | both viewports stay synchronized and I can navigate the scene from either application |  |  |
| US-12.6.5 | engine tester (P-27) | latency tests that measure the time from DCC tool edit to engine viewport update via live link, verifying it stays under 2 seconds | the interactive iteration target is validated |  |  |

## Houdini (F-12.6.3, F-12.6.4, F-12.6.5)

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-12.6.6 | designer (P-5) | Houdini Digital Assets (HDAs) to run within the engine editor with exposed parameters in the inspector and cook-on-parameter-change | I can use Houdini procedural workflows interactively without switching applications |  |  |
| US-12.6.7 | designer (P-5) | to export meshes with all vertex attributes, skeletons, skin weights, morph targets, packed primitives as instances, curves as splines, and volumes as 3D textures from Houdini | all Houdini output types transfer to the engine without data loss |  |  |
| US-12.6.8 | designer (P-5) | Houdini scatter and placement nodes to output point clouds with instance attributes that the engine converts to ECS entities via the PCG pipeline | I can author art-directed placement in Houdini's procedural environment |  |  |
| US-12.6.9 | engine tester (P-27) | automated tests that cook reference HDAs and compare the output geometry, instances, and attributes against golden reference assets | Houdini Engine integration regressions are caught in CI |  |  |

## Maya (F-12.6.6, F-12.6.7)

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-12.6.10 | designer (P-5) | a Maya plugin that exports meshes, skeletons, animations, blend shapes, materials, cameras, and lights with incremental export of only changed nodes | I can push Maya assets to the engine without full scene export |  |  |
| US-12.6.11 | designer (P-5) | animation export with full curve data (Bezier tangents, step/linear/cubic interpolation), skin weights, IK metadata, and character rig bundles | Maya animation data transfers to the engine without quality loss from baked per-frame samples |  |  |
| US-12.6.12 | engine tester (P-27) | automated tests that export reference Maya scenes and compare the imported result against golden assets, verifying mesh data, skeleton hierarchy, animation curves, and material properties | Maya plugin regressions are caught in CI |  |  |

## Blender (F-12.6.8, F-12.6.9)

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-12.6.13 | designer (P-5) | a Blender addon that exports meshes, armatures, shape keys, animations, materials (Principled BSDF mapped to engine PBR), and collections as prefab hierarchies | Blender users have a first-class export experience |  |  |
| US-12.6.14 | engine tester (P-27) | tests that convert Principled BSDF node trees with varying input configurations (all texture inputs, mixed constant/texture, custom nodes) and verify correct engine material output | Blender material conversion is reliable for common authoring patterns |  |  |
| US-12.6.15 | game developer (P-15) | Blender's modifier stack applied at export time with an option to export pre-modifier data for engine-side processing | artists can choose whether to bake modifiers or defer them to the engine |  |  |

## Marvelous Designer (F-12.6.10, F-12.6.11)

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-12.6.16 | designer (P-5) | to export garment meshes, UV layouts, seam data, and fabric properties from Marvelous Designer with automatic skeleton binding | cloth assets are ready for runtime simulation without manual skinning |  |  |
| US-12.6.17 | designer (P-5) | automatic garment-to-character fitting that computes cloth attachment constraints and skin weight transfers for varying body proportions | garments deform correctly with character body morph changes |  |  |
| US-12.6.18 | engine tester (P-27) | tests that fit reference garments to characters with varying body morphs and verify the cloth simulation constraints and collision bodies are generated correctly | garment fitting does not produce simulation artifacts |  |  |

## Substance (F-12.6.12, F-12.6.13, F-12.6.14)

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-12.6.19 | designer (P-5) | to import .sbsar Substance materials and adjust exposed parameters (color tint, wear, dirt) in the engine material editor without re-exporting from Substance Designer | I can tune procedural materials in-engine |  |  |
| US-12.6.20 | designer (P-5) | Substance Painter project import that extracts channel- packed texture maps matching engine material conventions with UDIM support | painted texture sets transfer to the engine correctly |  |  |
| US-12.6.21 | game developer (P-15) | optional runtime .sbsar evaluation on a background thread for dynamic material variation (weathering by region, damage response) | procedural materials can adapt to gameplay context with budgeted GPU cost |  |  |
| US-12.6.22 | engine tester (P-27) | tests that import reference .sbsar materials, vary exposed parameters, and compare generated texture sets against baseline images | Substance integration correctness is validated |  |  |

## Photoshop (F-12.6.15, F-12.6.16)

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-12.6.23 | designer (P-5) | a Photoshop plugin that exports textures with channel packing, supports PSD round-trip re-export, live link, and layer-group-to-UI-widget conversion | I can iterate on textures and UI mockups in Photoshop with instant engine feedback |  |  |
| US-12.6.24 | designer (P-5) | Photoshop layer groups exported as engine UI widgets with text properties, shape nine-slices, and visibility state mapping | UI mockups designed in Photoshop transfer to the engine without manual recreation |  |  |
| US-12.6.25 | engine tester (P-27) | tests that export reference Photoshop files with text layers, shape layers, and visibility states, and verify the generated UI widget tree matches the expected structure | Photoshop-to-UI conversion is reliable |  |  |

## Illustrator (F-12.6.17)

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-12.6.26 | designer (P-5) | Illustrator vector artwork exported as resolution- independent vector graphics or rasterized atlases, with artboard-to-asset and symbol-to- instance mapping | icons, HUD elements, and decals render crisply at any scale |  |  |
| US-12.6.27 | engine tester (P-27) | tests that export reference Illustrator files and verify vector paths, color swatches, and symbol instances convert correctly to the engine's vector format | Illustrator export fidelity is validated |  |  |

## ZBrush (F-12.6.18, F-12.6.19)

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-12.6.28 | designer (P-5) | ZBrush sculpts exported with baked normal, displacement, and cavity maps projected onto a low-poly mesh, polypaint as vertex colors or texture, and multi-SubTool hierarchy | high-poly sculpt detail transfers to engine-ready assets |  |  |
| US-12.6.29 | designer (P-5) | automatic decimation of ZBrush sculpts to engine-ready poly counts with per-LOD normal map bakes | a single sculpt produces a complete LOD chain without manual decimation |  |  |
| US-12.6.30 | engine tester (P-27) | tests that export reference ZBrush sculpts and compare baked normal maps against golden references (PSNR thresholds) | the high-poly-to-low-poly bake pipeline produces visually correct results |  |  |

## MotionBuilder (F-12.6.20, F-12.6.21)

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-12.6.31 | designer (P-5) | mocap data, cleaned animations, and character rigs exported from MotionBuilder with full curve data and batch export of multiple takes | motion capture sessions transfer to the engine efficiently |  |  |
| US-12.6.32 | designer (P-5) | real-time mocap streaming from MotionBuilder applied to engine characters with full materials and lighting | directors see capture results on final characters during recording sessions |  |  |
| US-12.6.33 | engine tester (P-27) | tests that export reference animation takes from MotionBuilder and verify the imported curve data (keyframes, tangents, interpolation modes) matches the source within floating-point tolerance | animation fidelity is preserved |  |  |

## Git Integration (F-12.6.22, F-12.6.23, F-12.6.24)

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-12.6.34 | designer (P-5) | DCC plugins to automatically acquire Git LFS locks on source files when I open them and release locks on save and close | my team avoids merge conflicts on binary source files |  |  |
| US-12.6.35 | designer (P-5) | review comments from the engine editor and pull requests displayed in a panel within my DCC tool with reply and resolve actions | I can respond to feedback without switching applications |  |  |
| US-12.6.36 | designer (P-5) | a consistent status dashboard in every DCC plugin showing branch, lock status, pending changes, review comments, and one-click commit, push, pull, lock, unlock, and PR actions | I have the same version control experience regardless of which tool I am using |  |  |
| US-12.6.37 | engine tester (P-27) | tests that simulate two users attempting to lock the same file and verify correct lock acquisition, conflict notification, and lock release behavior | the collaborative locking workflow is reliable |  |  |
| US-12.6.38 | engine tester (P-27) | tests that post a review comment in the engine editor and verify it appears in the DCC plugin panel within the sync interval | cross-tool comment synchronization is reliable |  |  |

## Shared Features (F-12.6.25, F-12.6.26)

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-12.6.39 | engine developer (P-26) | a configurable material translation layer that maps each DCC tool's material system to the engine's PBR model with fallback defaults for unmapped nodes | material import works consistently regardless of the source tool |  |  |
| US-12.6.40 | DevOps engineer (P-16) | all DCC plugins to support headless batch export via command-line invocation (hython, mayapy, blender --background) | CI pipelines re-export modified source files and push them through the asset processing pipeline automatically |  |  |
| US-12.6.41 | engine tester (P-27) | tests that compare headless batch export output against interactive export output for the same source files and verify byte-identical results | CI builds produce the same assets as artist workstations |  |  |
| US-12.6.42 | engine tester (P-27) | tests that import the same material from Maya, Blender, and Substance using the material mapping table and verify the engine output matches within tolerance | the DCC-agnostic mapping produces consistent results |  |  |
| US-12.6.43 | DevOps engineer (P-16) | CI build manifests to track which DCC tools and license types (Houdini Engine, Maya Batch) are required per source file | build machines have the correct licenses provisioned for headless export |  |  |
