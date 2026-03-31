# 12.6 — DCC Tool Plugins

## Plugin SDK

| ID       | Feature |
|----------|---------|
| F-12.6.1 | Asset Pipeline Plugin SDK |
| F-12.6.2 | DCC Bridge Connection |

1. **F-12.6.1** — A C API with language bindings (Python, C++) that DCC tool plugins use to export
   assets directly to the engine's native binary format (F-12.7.1). The SDK provides functions to
   submit meshes, skeletons, animations, materials, textures, and scene hierarchies. The SDK handles
   versioning, compression, and transport over a local socket or shared memory channel.
   - **Deps:** F-12.7.1 (Universal Binary Asset Format), F-12.2.1 (Asset Processing)
   - **Platform:** SDK ships as a shared library (.dll/.dylib/.so) loadable by any DCC host.
2. **F-12.6.2** — Real-time bidirectional connection between a DCC tool and the running engine
   editor. Artists push mesh/material/animation changes and see results in the engine viewport
   within seconds without a full re-export. The engine sends camera position and selection state
   back to the DCC tool for synchronized viewports. Connection uses a local TCP socket with a
   compact binary delta protocol. available on mobile or console runtime.
   - **Deps:** F-12.6.1, F-12.4.1 (Hot Reload)
   - **Platform:** Desktop only. DCC tools run on Windows, macOS, or Linux workstations. Not

## Houdini Plugin

| ID       | Feature |
|----------|---------|
| F-12.6.3 | Houdini Engine Integration |
| F-12.6.4 | Houdini Mesh and Scene Export |
| F-12.6.5 | Houdini Procedural Placement Pipeline |

1. **F-12.6.3** — A Houdini Digital Asset (HDA) host that runs Houdini Engine in-process or
   out-of-process, enabling Houdini procedural graphs to execute within the engine editor. Artists
   author HDAs in Houdini and expose parameters in the engine's inspector. Output geometry,
   instances, curves, and attributes are converted to engine entities. Supports
   cook-on-parameter-change for interactive procedural workflows. a Houdini Engine license at edit
   time; baked output requires no runtime license.
   - **Deps:** F-12.6.1
   - **Platform:** Houdini Engine is a C API; wrapped via C FFI on the engine side. Requires
2. **F-12.6.4** — Export meshes with vertex attributes (position, normal, UV sets, color, custom
   attributes), skeleton hierarchies, skin weights, morph targets, and scene hierarchy directly from
   Houdini to the engine's native binary format (F-12.7.1) via the plugin SDK. Supports Houdini
   packed primitives as instanced meshes, curve primitives as engine splines, and volume primitives
   as 3D textures.
   - **Deps:** F-12.6.1, F-12.6.3
3. **F-12.6.5** — Connect Houdini scatter and placement nodes to the engine's procedural generation
   system (F-3.6). Houdini outputs point clouds with instance attributes (mesh ID, rotation, scale,
   custom tags); the engine converts these into ECS entities via the PCG spawning pipeline. Used for
   art-directed vegetation placement, prop scattering, and terrain detail authored in Houdini's
   procedural environment.
   - **Deps:** F-12.6.3, F-3.6.4 (PCG Spawning)

## Maya Plugin

| ID       | Feature |
|----------|---------|
| F-12.6.6 | Maya Direct Export Plugin |
| F-12.6.7 | Maya Animation and Rigging Export |

1. **F-12.6.6** — A Maya plugin (Python + C++ Maya API) that exports meshes, skeletons, animations,
   blend shapes, materials, cameras, and lights directly to the engine's native binary format
   (F-12.7.1) via the plugin SDK. The plugin reads Maya's internal DAG hierarchy and component data,
   converts to the engine's native format, and pushes over the DCC bridge or writes to an asset
   file. Supports incremental export of only changed nodes. Python layer exposes export commands in
   Maya's menu/shelf.
   - **Deps:** F-12.6.1
   - **Platform:** Uses Maya's C++ API (MFnMesh, MFnSkinCluster, etc.) wrapped in the plugin DLL.
2. **F-12.6.7** — Export animation clips (skeletal, blendshape, camera) to the engine's native
   binary format (F-12.7.1) with full curve data (Bezier tangents, step/linear/cubic interpolation)
   rather than baked per-frame samples. Export skin cluster weights, IK handles (as metadata for
   engine IK setup), and constraint setups. Character rigs export skeleton + bind pose + morph
   target set as a single character asset.
   - **Deps:** F-12.6.6

## Blender Plugin

| ID       | Feature |
|----------|---------|
| F-12.6.8 | Blender Direct Export Addon |
| F-12.6.9 | Blender Material to Engine Material Conversion |

1. **F-12.6.8** — A Blender addon (Python, using bpy API) that exports meshes, armatures, shape
   keys, animations, materials (node tree to engine material mapping), and scene hierarchy directly
   to the engine's native binary format (F-12.7.1). Supports Blender's collections as engine entity
   template hierarchies. Addon registers as an export operator and a DCC bridge panel in Blender's
   sidebar. Blender's modifier stack is applied at export time with an option to export pre-modifier
   data for engine-side processing. the plugin SDK's socket protocol.
   - **Deps:** F-12.6.1
   - **Platform:** Pure Python addon; no compiled extensions needed. Communicates with engine via
2. **F-12.6.9** — Map Blender's Principled BSDF node tree to the engine's PBR material model.
   Texture inputs (base color, metallic, roughness, normal, emission) are extracted and re-linked as
   engine texture assets. Non-mappable nodes (custom Blender shaders) emit warnings with fallback
   defaults. Material conversion runs automatically on export and on DCC bridge push.
   - **Deps:** F-12.6.8

## Marvelous Designer

| ID        | Feature |
|-----------|---------|
| F-12.6.10 | Marvelous Designer Clothing Export |
| F-12.6.11 | Garment-to-Character Fitting |

1. **F-12.6.10** — Export clothing and fabric assets from Marvelous Designer directly to the
   engine's native binary format (F-12.7.1) via the plugin SDK. The plugin extracts garment meshes,
   UV layouts, seam data, and fabric material properties (thickness, stretch, bend stiffness).
   Garment meshes are bound to the character skeleton using the engine's skinning pipeline. Fabric
   properties map to the cloth simulation system (F-4.7.2) for runtime simulation of exported
   garments.
   - **Deps:** F-12.6.1 (Plugin SDK), F-4.7.2 (Cloth), F-13.8.9 (Modular Parts)
2. **F-12.6.11** — Automatically fit imported Marvelous Designer garments to character bodies with
   varying proportions. The fitting system reads the garment's rest shape and the target character's
   body mesh, then computes cloth attachment constraints and skin weight transfers. Body morph
   propagation (F-13.8.4) ensures garments deform with character body changes. Collision bodies are
   auto-generated from the character mesh for cloth simulation.
   - **Deps:** F-12.6.10, F-13.8.4 (Morph Propagation)

## Adobe Substance

| ID        | Feature |
|-----------|---------|
| F-12.6.12 | Substance Material Import |
| F-12.6.13 | Substance Painter Project Import |
| F-12.6.14 | Runtime Substance Material Evaluation |

1. **F-12.6.12** — Import Substance Designer .sbsar (Substance Archive) materials and evaluate them
   at build time to produce engine-ready texture sets (base color, normal, metallic, roughness, AO,
   height, emissive). Exposed parameters (color tints, pattern scale, wear amount, dirt level) are
   stored in the material asset and adjustable in the engine's material editor without re-exporting
   from Substance Designer. licensed separately; baked textures require no runtime license.
   - **Deps:** F-12.6.1 (Plugin SDK)
   - **Platform:** Uses the Substance Engine C API for .sbsar evaluation. Substance Engine is
2. **F-12.6.13** — Import Substance Painter projects (.spp) and extract baked texture maps with
   per-texel-set channel packing matching the engine's material conventions. The importer reads the
   painter export preset and maps channels (BaseColor, Roughness, Metallic, Normal, Height,
   Emissive) to engine material slots. UDIM tile sets are supported for multi-UV-tile assets.
   Texture resolution is configurable at import time independently of the painter project
   resolution.
   - **Deps:** F-12.6.12
3. **F-12.6.14** — Optionally evaluate .sbsar materials at runtime for dynamic material variation —
   e.g., a stone wall that adjusts weathering based on world region, or character skin that responds
   to damage. The Substance Engine runs on a background thread; output textures are uploaded to GPU
   asynchronously. Runtime evaluation is opt-in per material and budgeted to avoid frame spikes.
   - **Deps:** F-12.6.12, F-1.8.1 (Async I/O)
   - **Platform:** Requires Substance Engine runtime license for shipping titles.

## Adobe Photoshop

| ID        | Feature |
|-----------|---------|
| F-12.6.15 | Photoshop Direct Export Plugin |
| F-12.6.16 | Photoshop Layer-to-UI Asset Pipeline |

1. **F-12.6.15** — A Photoshop plugin (UXP/CEP JavaScript + C++ native module) that exports
   textures, UI sprites, and layered compositions directly to the engine's native binary format
   (F-12.7.1). Exports individual layers or flattened output with channel packing (e.g., metallic in
   R, roughness in G, AO in B) matching the engine's material conventions. Supports PSD round-trip —
   the engine stores a reference to the source PSD so re-export updates the engine asset without
   manual reimport. DCC Bridge pushes texture changes to the engine viewport in real time. versions.
   Native C++ module handles binary asset serialization.
   - **Deps:** F-12.6.1, F-12.6.2 (DCC Bridge)
   - **Platform:** Uses Adobe UXP plugin API (Photoshop 2022+). Legacy CEP fallback for older
2. **F-12.6.16** — Export Photoshop layer groups as engine UI widget hierarchies. Text layers become
   UI text components with font, size, and color preserved. Shape layers become nine-slice or vector
   elements. Layer visibility maps to widget visibility states. Designers iterate on UI mockups in
   Photoshop and push to the engine without manual recreation of widget trees.
   - **Deps:** F-12.6.15, F-10.1.1 (Widget Framework)

## Adobe Illustrator

| ID        | Feature |
|-----------|---------|
| F-12.6.17 | Illustrator Vector Asset Export |

1. **F-12.6.17** — Export Illustrator vector artwork (icons, HUD elements, decals, signage) to the
   engine's native binary format (F-12.7.1) as vector graphics assets or rasterized texture atlases.
   SVG paths are converted to the engine's vector rendering format (F-10.4.3) for
   resolution-independent UI. Artboards map to individual assets; symbols map to shared instances.
   Color swatches export as engine color palette assets for consistent theming. UI coordinates with
   configurable DPI target.
   - **Deps:** F-12.6.1, F-10.4.3 (Vector Graphics)
   - **Platform:** Uses Adobe UXP plugin API. Illustrator's coordinate system is mapped to engine

## ZBrush

| ID        | Feature |
|-----------|---------|
| F-12.6.18 | ZBrush High-Poly to Engine Pipeline |
| F-12.6.19 | ZBrush Decimation and LOD Generation |

1. **F-12.6.18** — Export ZBrush sculpts (ZTL/ZPR) to the engine's native binary format (F-12.7.1)
   via GoZ bridge or file-based export. The plugin extracts the highest subdivision mesh, generates
   normal/displacement/cavity maps by projecting detail onto a lower subdivision level, and pushes
   both the low-poly mesh and baked maps to the engine. Polypaint exports as vertex colors or baked
   to texture. Supports multi-part models (SubTools) as separate engine mesh assets with preserved
   hierarchy. exchange directory with file-watch trigger.
   - **Deps:** F-12.6.1, F-12.2.1 (Asset Processing)
   - **Platform:** Uses ZBrush GoZ C++ plugin interface. GoZ bridge communicates via shared file
2. **F-12.6.19** — Automatically decimate ZBrush high-poly sculpts to engine-ready poly counts using
   ZRemesher or Decimation Master settings configured in the plugin. Generate multiple LOD levels
   from a single sculpt with corresponding normal map bakes per LOD. LOD chain is exported as a
   single engine mesh asset with LOD transitions configured automatically.
   - **Deps:** F-12.6.18, F-12.2.2 (LOD Generation)

## MotionBuilder

| ID        | Feature |
|-----------|---------|
| F-12.6.20 | MotionBuilder Animation Export |
| F-12.6.21 | MotionBuilder Live Mocap Streaming |

1. **F-12.6.20** — Export motion capture data, cleaned animations, and character rigs from
   MotionBuilder directly to the engine's native binary format (F-12.7.1) via the plugin SDK. The
   plugin reads MotionBuilder's internal scene representation, extracts skeleton hierarchies,
   animation takes with full curve data (not baked samples), and character definitions. Supports
   batch export of multiple takes from a single session. custom device for DCC bridge streaming of
   real-time mocap data.
   - **Deps:** F-12.6.1
   - **Platform:** Uses MotionBuilder's Open Reality SDK (C++ ORSDK). Plugin registers as a
2. **F-12.6.21** — Stream motion capture data from MotionBuilder to the engine in real time for live
   preview. The plugin sends skeleton pose data at capture frame rate over the DCC bridge
   connection. The engine applies incoming poses to character entities in the viewport, enabling
   directors to see mocap results on final characters with full materials and lighting during
   capture sessions.
   - **Deps:** F-12.6.20, F-12.6.2 (DCC Bridge)

## Git Integration for DCC Plugins

| ID        | Feature |
|-----------|---------|
| F-12.6.22 | DCC Plugin Git LFS Lock Workflow |
| F-12.6.23 | DCC Plugin Review Comment Viewer |
| F-12.6.24 | DCC Plugin Asset Status Dashboard |

1. **F-12.6.22** — Every DCC plugin integrates with Git LFS locking. When an artist opens a source
   file (PSD, .ma, .blend, .ztl, .zprj, .hip, .spp, .ai) from the engine's asset browser or from
   within the DCC tool, the plugin automatically acquires a Git LFS lock on that file. A status
   indicator in the DCC tool shows lock state (locked by you, locked by another user, unlocked). On
   save and close, the plugin commits the changes, pushes, and releases the lock. If another user
   holds the lock, the plugin shows who and offers to request the lock. command-line Git
   installation required in the DCC tool environment.
   - **Deps:** F-15.10.2 (Git LFS Management), F-12.6.1
   - **Platform:** Git operations use libgit2 via the plugin SDK's shared library. No
2. **F-12.6.23** — Each DCC plugin displays review comments (F-15.12.13) attached to the asset being
   edited. Comments from the engine editor, pull request reviews, and collaborator feedback appear
   in a dedicated panel within the DCC tool. Artists can view threads, read feedback, mark comments
   as resolved, and reply — all without switching to the engine editor or a web browser. Comments
   sync via the collaboration cloud service (F-15.12.7).
   - **Deps:** F-15.12.13 (Asset Comments), F-15.12.7 (Cloud Service), F-12.6.1
3. **F-12.6.24** — A shared UI panel (consistent across all DCC plugins) showing: current Git
   branch, file lock status, last commit info, pending changes, review comments count, and build
   cache status for the current asset. One-click actions for commit, push, pull, lock, unlock, and
   open PR. The dashboard provides the same version control experience regardless of whether the
   artist is in Maya, Blender, Photoshop, or any other supported tool. panels for Blender, UXP
   panels for Adobe tools, IMGUI for ZBrush/Houdini).
   - **Deps:** F-12.6.22, F-12.6.23, F-15.10.1 (Git Integration)
   - **Platform:** Dashboard UI rendered via the DCC tool's native panel API (Qt for Maya, bpy

## Shared Features

| ID        | Feature |
|-----------|---------|
| F-12.6.25 | DCC-Agnostic Material Mapping |
| F-12.6.26 | Batch Export and CI Integration |

1. **F-12.6.25** — A material translation layer that maps each DCC tool's material system to the
   engine's PBR material model via a configurable mapping table. Texture semantics (diffuse/albedo,
   specular/ metallic, glossiness/roughness) are translated between conventions. Missing textures
   fall back to defaults. The mapping table is extensible for custom DCC shaders and engine material
   types.
   - **Deps:** F-12.6.1
2. **F-12.6.26** — All DCC plugins support headless batch export via command-line invocation
   (Houdini hython, Maya mayapy, Blender --background, Photoshop --headless). Asset builds in CI
   pipelines invoke DCC tools to re-export modified source files, push them through the asset
   processing pipeline, and validate results. Build manifests track which source files map to which
   DCC tool and export preset. licenses are available for headless rendering.
   - **Deps:** F-12.6.1, F-12.3.4 (Incremental Builds)
   - **Platform:** Requires DCC tool licenses on CI build machines. Houdini Engine and Maya Batch
