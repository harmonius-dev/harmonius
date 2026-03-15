# R-12.6 DCC Tool Plugins

## R-12.6.1 Plugin SDK Asset Export
The engine **SHALL** provide a C API plugin SDK with Python and C++ language bindings that DCC
tool plugins use to export meshes, skeletons, animations, materials, textures, and scene
hierarchies to the engine's native binary format (R-12.7.1), with versioning, compression,
and transport over a local socket or shared-memory channel.
- **Derived from:** [F-12.6.1](../../features/content-pipeline/dcc-plugins.md)
- **Rationale:** A single SDK with a stable C ABI allows every supported DCC tool to export
  directly to the native binary format without intermediate file conversions.
- **Verification:** Load the SDK shared library in a minimal C host, call the mesh-submit
  API with a triangle, and confirm a valid native binary asset is produced with correct
  header, version, and content hash.

## R-12.6.2 Live Link Connection
The engine **SHALL** maintain a real-time bidirectional TCP connection between a DCC tool
plugin and the running engine editor that transmits asset deltas (mesh, material, animation
changes) from the DCC tool and camera/selection state from the engine, with viewport updates
completing within 2 seconds of a push.
- **Derived from:** [F-12.6.2](../../features/content-pipeline/dcc-plugins.md)
- **Rationale:** Sub-second feedback between DCC tools and the engine viewport eliminates the
  full re-export cycle and accelerates artist iteration.
- **Verification:** Modify a mesh in a connected DCC tool, push via live link, and confirm
  the engine viewport reflects the change within 2 seconds; disconnect the DCC tool and
  confirm the engine handles the dropped connection gracefully.

## R-12.6.3 Houdini Engine Integration
The engine **SHALL** host Houdini Digital Assets (HDAs) via the Houdini Engine C/C++ API,
execute procedural graphs in-process or out-of-process, expose HDA parameters in the
engine inspector, convert output geometry/instances/curves/attributes to engine entities,
and re-cook on parameter change.
- **Derived from:** [F-12.6.3](../../features/content-pipeline/dcc-plugins.md)
- **Rationale:** Hosting Houdini Engine enables procedural content authored in Houdini to
  execute directly inside the engine editor without manual export.
- **Verification:** Load an HDA that generates a grid mesh, change a parameter (grid size),
  and confirm the engine entity updates with the re-cooked geometry; confirm no Houdini
  Engine license is required for baked output at runtime.

## R-12.6.4 Houdini Mesh and Scene Export
The engine **SHALL** accept Houdini mesh exports containing vertex attributes (position,
normal, UV sets, color, custom attributes), skeleton hierarchies, skin weights, morph
targets, packed primitives as instanced meshes, curve primitives as engine splines, and
volume primitives as 3D textures, all in the engine's native binary format (R-12.7.1).
- **Derived from:** [F-12.6.4](../../features/content-pipeline/dcc-plugins.md)
- **Rationale:** Full-fidelity export from Houdini preserves all geometry data types that
  artists rely on, avoiding lossy intermediate format conversions.
- **Verification:** Export a Houdini scene containing a skinned mesh, packed primitives, a
  curve, and a volume; import into the engine and confirm each primitive type maps to its
  corresponding engine representation with correct attribute values.

## R-12.6.5 Houdini Procedural Placement Pipeline
The engine **SHALL** accept Houdini scatter/placement point clouds with instance attributes
(mesh ID, rotation, scale, custom tags) and convert them into ECS entities via the PCG
spawning pipeline (R-3.6).
- **Derived from:** [F-12.6.5](../../features/content-pipeline/dcc-plugins.md)
- **Rationale:** Connecting Houdini's procedural placement tools to the engine's ECS spawning
  pipeline enables art-directed vegetation and prop scattering without manual placement.
- **Verification:** Export a Houdini scatter node's point cloud with 1000 instances, import
  into the engine, and confirm 1000 ECS entities are spawned with correct transforms and
  mesh references.

## R-12.6.6 Maya Direct Export
The engine **SHALL** accept Maya plugin exports of meshes, skeletons, animations, blend
shapes, materials, cameras, and lights in the engine's native binary format (R-12.7.1),
with support for incremental export of only changed DAG nodes.
- **Derived from:** [F-12.6.6](../../features/content-pipeline/dcc-plugins.md)
- **Rationale:** Direct export from Maya via the plugin SDK avoids intermediate formats and
  enables incremental workflows that export only what changed.
- **Verification:** Export a Maya scene with a skinned character and a light; modify only the
  light and re-export incrementally; confirm only the light asset is updated in the engine.

## R-12.6.7 Maya Animation and Rigging Export
The engine **SHALL** accept Maya animation exports containing full curve data (Bezier
tangents, step/linear/cubic interpolation) rather than baked per-frame samples, skin cluster
weights, IK handle metadata, constraint setups, and character rig bundles (skeleton + bind
pose + morph target set as a single asset).
- **Derived from:** [F-12.6.7](../../features/content-pipeline/dcc-plugins.md)
- **Rationale:** Exporting animation curves with tangent data instead of baked samples
  preserves artistic intent and reduces asset size.
- **Verification:** Export a Maya animation clip with Bezier interpolation, import into the
  engine, and confirm curve tangents match the source; export a character rig and confirm
  skeleton, bind pose, and morph targets are bundled as a single asset.

## R-12.6.8 Blender Direct Export
The engine **SHALL** accept Blender addon exports of meshes, armatures, shape keys,
animations, materials (node-tree-to-engine-material mapping), and scene hierarchies in
the engine's native binary format (R-12.7.1), with collections mapped to prefab
hierarchies and optional pre-modifier data export.
- **Derived from:** [F-12.6.8](../../features/content-pipeline/dcc-plugins.md)
- **Rationale:** A pure-Python Blender addon communicating via the SDK socket protocol
  provides direct export without requiring compiled extensions.
- **Verification:** Export a Blender scene with two collections containing meshes with
  modifiers; confirm the engine imports them as prefab hierarchies; export with modifiers
  applied and with pre-modifier data and confirm both modes produce valid assets.

## R-12.6.9 Blender Material Conversion
The engine **SHALL** map Blender Principled BSDF node-tree inputs (base color, metallic,
roughness, normal, emission) to the engine's PBR material model, re-link texture inputs as
engine texture assets, and emit warnings with fallback defaults for non-mappable nodes.
- **Derived from:** [F-12.6.9](../../features/content-pipeline/dcc-plugins.md)
- **Rationale:** Automatic material translation from Blender's shader graph to the engine's
  PBR model eliminates manual material recreation.
- **Verification:** Export a Blender material with Principled BSDF inputs; confirm engine
  material has correct texture bindings; add a non-mappable custom node and confirm a warning
  is emitted with a fallback default applied.

## R-12.6.10 Marvelous Designer Clothing Export
The engine **SHALL** accept Marvelous Designer exports of garment meshes, UV layouts, seam
data, and fabric material properties (thickness, stretch, bend stiffness) in the engine's
native binary format (R-12.7.1), bind garment meshes to character skeletons via the skinning
pipeline, and map fabric properties to the cloth simulation system (R-4.7).
- **Derived from:** [F-12.6.10](../../features/content-pipeline/dcc-plugins.md)
- **Rationale:** Direct garment export with fabric physics parameters enables runtime cloth
  simulation of designer-authored clothing without manual property setup.
- **Verification:** Export a Marvelous Designer garment, import into the engine, confirm
  skinning to a character skeleton succeeds, and confirm fabric properties drive the cloth
  simulation with correct stiffness and stretch values.

## R-12.6.11 Garment-to-Character Fitting
The engine **SHALL** automatically fit imported garments to character bodies by computing
cloth attachment constraints and skin weight transfers from the garment rest shape and
target body mesh, propagate body morph changes to garment deformation, and auto-generate
collision bodies from the character mesh for cloth simulation.
- **Derived from:** [F-12.6.11](../../features/content-pipeline/dcc-plugins.md)
- **Rationale:** Automatic fitting eliminates manual per-character garment adjustment and
  ensures garments deform correctly across body variations.
- **Verification:** Fit a garment to two characters with different body proportions; confirm
  attachment constraints are computed for both; apply a body morph and confirm the garment
  deforms accordingly; confirm collision bodies are generated.

## R-12.6.12 Substance Material Import
The engine **SHALL** import Substance Designer .sbsar archives, evaluate them at build time
to produce engine-ready texture sets (base color, normal, metallic, roughness, AO, height,
emissive), and store exposed parameters (color tints, pattern scale, wear, dirt) in the
material asset for adjustment in the engine's material editor without re-export.
- **Derived from:** [F-12.6.12](../../features/content-pipeline/dcc-plugins.md)
- **Rationale:** Build-time evaluation of parametric Substance materials produces optimized
  texture sets while preserving parameter tweakability in the engine editor.
- **Verification:** Import a .sbsar with exposed parameters, build, and confirm all texture
  channels are produced; change a parameter in the material editor and confirm textures
  regenerate without re-exporting from Substance Designer.

## R-12.6.13 Substance Painter Project Import
The engine **SHALL** import Substance Painter .spp projects, extract baked texture maps with
channel packing matching the engine's material conventions, support UDIM tile sets for
multi-UV-tile assets, and allow configurable texture resolution at import time.
- **Derived from:** [F-12.6.13](../../features/content-pipeline/dcc-plugins.md)
- **Rationale:** Direct .spp import with correct channel packing removes the manual export
  and re-mapping step from the texture artist workflow.
- **Verification:** Import a .spp project with UDIM tiles; confirm channel packing matches
  engine conventions; re-import at a different resolution and confirm output resolution
  changes independently of the painter project resolution.

## R-12.6.14 Runtime Substance Material Evaluation
The engine **SHALL** optionally evaluate .sbsar materials at runtime on a background thread
with asynchronous GPU texture upload, gated by a per-material opt-in flag and a configurable
evaluation budget to prevent frame spikes.
- **Derived from:** [F-12.6.14](../../features/content-pipeline/dcc-plugins.md)
- **Rationale:** Runtime parametric material evaluation enables dynamic visual variation
  (weathering, damage) without pre-baking every permutation.
- **Verification:** Enable runtime evaluation on a material, vary a parameter at runtime,
  and confirm the texture updates without exceeding the configured budget or causing frame
  drops; confirm evaluation does not run when the opt-in flag is disabled.

## R-12.6.15 Photoshop Direct Export
The engine **SHALL** accept Photoshop plugin exports of textures, UI sprites, and layered
compositions in the engine's native binary format (R-12.7.1) with configurable channel
packing, PSD round-trip via source file references, and live-link real-time texture push.
- **Derived from:** [F-12.6.15](../../features/content-pipeline/dcc-plugins.md)
- **Rationale:** Direct Photoshop export with channel packing and live-link push keeps
  texture artists in their primary tool without manual reimport.
- **Verification:** Export a PSD with channel-packed layers; confirm the engine asset matches
  the packing configuration; modify the PSD and re-export; confirm the engine asset updates
  via the stored source reference without manual reimport.

## R-12.6.16 Photoshop Layer-to-UI Pipeline
The engine **SHALL** convert Photoshop layer groups into engine UI widget hierarchies, map
text layers to text components (preserving font, size, color), shape layers to nine-slice or
vector elements, and layer visibility to widget visibility states.
- **Derived from:** [F-12.6.16](../../features/content-pipeline/dcc-plugins.md)
- **Rationale:** Automatic conversion of Photoshop mockups to engine widget trees eliminates
  manual widget recreation and keeps designers iterating in Photoshop.
- **Verification:** Export a PSD with text layers, shape layers, and nested groups; confirm
  the engine produces a matching widget hierarchy with correct font properties, nine-slice
  regions, and visibility states.

## R-12.6.17 Illustrator Vector Asset Export
The engine **SHALL** accept Illustrator vector artwork exports as vector graphics assets (for
resolution-independent UI rendering via R-10.4) or rasterized texture atlases, map artboards
to individual assets, symbols to shared instances, and color swatches to engine color palette
assets.
- **Derived from:** [F-12.6.17](../../features/content-pipeline/dcc-plugins.md)
- **Rationale:** Vector export preserves resolution independence for UI elements while atlas
  rasterization provides a fallback for complex artwork.
- **Verification:** Export an Illustrator file with multiple artboards, symbols, and color
  swatches; confirm each artboard produces a separate asset, symbols produce shared
  instances, and swatches map to engine color palettes.

## R-12.6.18 ZBrush High-Poly Export
The engine **SHALL** accept ZBrush sculpt exports via GoZ bridge or file-based export,
extract the highest-subdivision mesh, generate normal/displacement/cavity maps by projecting
onto a lower subdivision level, export polypaint as vertex colors or baked texture, and
preserve SubTool hierarchy as separate mesh assets.
- **Derived from:** [F-12.6.18](../../features/content-pipeline/dcc-plugins.md)
- **Rationale:** Automated detail-map baking from high-poly sculpts eliminates the manual
  bake step and delivers game-ready meshes directly from ZBrush.
- **Verification:** Export a multi-SubTool ZBrush sculpt with polypaint; confirm the engine
  receives a low-poly mesh with baked normal/displacement/cavity maps and vertex colors;
  confirm SubTools map to separate engine mesh assets.

## R-12.6.19 ZBrush Decimation and LOD Generation
The engine **SHALL** decimate ZBrush high-poly sculpts to engine-ready poly counts using
plugin-configured ZRemesher or Decimation Master settings, generate multiple LOD levels with
per-LOD normal map bakes, and export the LOD chain as a single mesh asset with automatic LOD
transition configuration.
- **Derived from:** [F-12.6.19](../../features/content-pipeline/dcc-plugins.md)
- **Rationale:** Generating the full LOD chain from a single sculpt at export time ensures
  consistent detail reduction and eliminates manual LOD authoring.
- **Verification:** Export a ZBrush sculpt with 3 LOD levels configured; confirm each LOD
  has the expected poly count and a corresponding normal map; confirm LOD transitions are
  configured in the resulting mesh asset.

## R-12.6.20 MotionBuilder Animation Export
The engine **SHALL** accept MotionBuilder exports of skeleton hierarchies, animation takes
with full curve data (not baked samples), and character definitions in the engine's native
binary format (R-12.7.1), with support for batch export of multiple takes from a single
session.
- **Derived from:** [F-12.6.20](../../features/content-pipeline/dcc-plugins.md)
- **Rationale:** Curve-based animation export from MotionBuilder preserves mocap cleanup
  edits and avoids data bloat from per-frame baking.
- **Verification:** Export 5 animation takes in a single batch from MotionBuilder; confirm
  all 5 are imported with correct curve data; confirm skeleton hierarchies match the source
  character definition.

## R-12.6.21 MotionBuilder Live Mocap Streaming
The engine **SHALL** receive real-time skeleton pose data from MotionBuilder over the live
link connection at capture frame rate and apply incoming poses to character entities in the
engine viewport during capture sessions.
- **Derived from:** [F-12.6.21](../../features/content-pipeline/dcc-plugins.md)
- **Rationale:** Live mocap preview on final characters with full materials and lighting
  enables directors to evaluate performance during capture.
- **Verification:** Stream mocap data from MotionBuilder at 60 fps; confirm the engine
  character entity reflects the incoming poses in real time with no more than 2 frames of
  latency.

## R-12.6.22 DCC Plugin Git LFS Lock Workflow
The engine **SHALL** integrate Git LFS locking into every DCC plugin so that opening a
source file acquires a lock, a status indicator shows lock state (locked by you, locked by
another user, unlocked), and saving and closing commits, pushes, and releases the lock, with
lock conflict notification identifying the current holder.
- **Derived from:** [F-12.6.22](../../features/content-pipeline/dcc-plugins.md)
- **Rationale:** Automatic lock-on-open prevents concurrent edits to binary source files that
  cannot be merged, reducing lost work.
- **Verification:** Open a source file in a DCC plugin and confirm a Git LFS lock is
  acquired; attempt to open the same file from a second user and confirm the lock holder is
  displayed; save, close, and confirm the lock is released after push.

## R-12.6.23 DCC Plugin Review Comment Viewer
The engine **SHALL** display asset review comments from the engine editor, pull request
reviews, and collaborator feedback in a dedicated panel within each DCC plugin, supporting
thread viewing, replying, and marking comments as resolved, synchronized via the
collaboration cloud service.
- **Derived from:** [F-12.6.23](../../features/content-pipeline/dcc-plugins.md)
- **Rationale:** Surfacing review feedback inside the DCC tool eliminates context-switching
  to the engine editor or a web browser during art iteration.
- **Verification:** Post a review comment on an asset in the engine editor; open the asset
  in a DCC plugin and confirm the comment appears; reply from the DCC plugin and confirm the
  reply appears in the engine editor.

## R-12.6.24 DCC Plugin Asset Status Dashboard
The engine **SHALL** provide a consistent UI panel across all DCC plugins displaying current
Git branch, file lock status, last commit info, pending changes, review comment count, and
build cache status, with one-click actions for commit, push, pull, lock, unlock, and open PR.
- **Derived from:** [F-12.6.24](../../features/content-pipeline/dcc-plugins.md)
- **Rationale:** A unified version control dashboard across all DCC tools gives artists a
  consistent experience regardless of which tool they are working in.
- **Verification:** Open the dashboard in Maya, Blender, and Photoshop plugins; confirm all
  three display the same branch, lock status, and commit info for the same asset; perform a
  commit from each and confirm success.

## R-12.6.25 DCC-Agnostic Material Mapping
The engine **SHALL** translate each DCC tool's material system to the engine's PBR material
model via a configurable mapping table that converts texture semantics (diffuse/albedo,
specular/metallic, glossiness/roughness) between conventions, falls back to defaults for
missing textures, and is extensible for custom DCC shaders and engine material types.
- **Derived from:** [F-12.6.25](../../features/content-pipeline/dcc-plugins.md)
- **Rationale:** A shared material translation layer ensures consistent PBR results across
  all DCC tools without per-tool special-case logic.
- **Verification:** Export a material with identical PBR properties from Maya, Blender, and
  Houdini; confirm all three produce equivalent engine materials; add a custom mapping entry
  and confirm it is applied on the next export.

## R-12.6.26 Batch Export and CI Integration
The engine **SHALL** support headless batch export via command-line invocation of all
supported DCC tools (hython, mayapy, blender --background, photoshop --headless), enabling
CI pipelines to re-export modified source files, process them through the asset pipeline, and
validate results using build manifests that track source-file-to-DCC-tool-and-preset mappings.
- **Derived from:** [F-12.6.26](../../features/content-pipeline/dcc-plugins.md)
- **Rationale:** Headless batch export in CI ensures every committed source file is
  re-exported and validated automatically, catching broken assets before they reach artists.
- **Verification:** Run a CI job that batch-exports 10 source files across 3 DCC tools;
  confirm all assets are produced and pass validation; modify one source file and confirm
  only that file is re-exported on the next incremental build.
