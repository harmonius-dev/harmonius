# R-12.6 DCC Tool Plugins

## Plugin SDK

1. **R-12.6.1** — The engine **SHALL** provide a C API plugin SDK with Python and C++ language
   bindings that DCC tool plugins use to export meshes, skeletons, animations, materials, textures,
   and scene hierarchies to the engine's native binary format, with versioning, compression, and
   transport over a local socket or shared-memory channel.
   - **Rationale:** A single SDK with a stable C ABI allows every supported DCC tool to export
     directly to the native binary format without intermediate conversions.
   - **Verification:** Load the SDK shared library in a minimal C host, call the mesh-submit API
     with a triangle, and confirm a valid native binary asset is produced.
2. **R-12.6.2** — The engine **SHALL** maintain a real-time bidirectional TCP connection between a
   DCC tool plugin and the running engine editor that transmits asset deltas from the DCC tool and
   camera/selection state from the engine, with viewport updates completing within 2 seconds.
   - **Rationale:** Sub-second feedback between DCC tools and the engine viewport eliminates the
     full re-export cycle and accelerates artist iteration.
   - **Verification:** Modify a mesh in a connected DCC tool, push via DCC bridge, and confirm the
     engine viewport reflects the change within 2 seconds; disconnect and confirm graceful handling.

## Houdini

3. **R-12.6.3** — The engine **SHALL** host Houdini Digital Assets via the Houdini Engine C API,
   execute procedural graphs in-process or out-of-process, expose HDA parameters in the engine
   inspector, convert output geometry to engine entities, and re-cook on parameter change.
   - **Rationale:** Hosting Houdini Engine enables procedural content to execute inside the engine
     editor without manual export.
   - **Verification:** Load an HDA, change a parameter, and confirm the engine entity updates with
     re-cooked geometry; confirm no Houdini license is required for baked runtime output.
4. **R-12.6.4** — The engine **SHALL** accept Houdini exports containing vertex attributes, skeleton
   hierarchies, skin weights, morph targets, packed primitives as instanced meshes, curve primitives
   as splines, and volume primitives as 3D textures.
   - **Rationale:** Full-fidelity export from Houdini preserves all geometry data types, avoiding
     lossy intermediate conversions.
   - **Verification:** Export a Houdini scene with a skinned mesh, packed primitives, a curve, and a
     volume; confirm each type maps to its engine representation correctly.
5. **R-12.6.5** — The engine **SHALL** accept Houdini scatter/placement point clouds with instance
   attributes and convert them into ECS entities via the PCG spawning pipeline (F-3.6.4).
   - **Rationale:** Connecting Houdini's procedural placement to the engine's ECS spawning enables
     art-directed vegetation and prop scattering.
   - **Verification:** Export 1000 scatter instances; confirm 1000 ECS entities are spawned with
     correct transforms.

## Maya

6. **R-12.6.6** — The engine **SHALL** accept Maya plugin exports of meshes, skeletons, animations,
   blend shapes, materials, cameras, and lights with incremental export of only changed DAG nodes.
   - **Rationale:** Incremental export avoids full scene re-export and speeds up artist iteration.
   - **Verification:** Export a Maya scene; modify only a light and re-export incrementally; confirm
     only the light asset is updated.
7. **R-12.6.7** — The engine **SHALL** accept Maya animation exports with full curve data (Bezier
   tangents, step/linear/cubic interpolation), skin weights, IK metadata, and character rig bundles.
   - **Rationale:** Curve-based animation export preserves artistic intent and reduces asset size
     compared to baked per-frame samples.
   - **Verification:** Export a Maya animation clip with Bezier interpolation; confirm curve
     tangents match the source; export a character rig and confirm skeleton, bind pose, and morph
     targets are bundled.

## Blender

8. **R-12.6.8** — The engine **SHALL** accept Blender addon exports of meshes, armatures, shape
   keys, animations, materials (node-tree-to-engine-material mapping), and scene hierarchies with
   collections mapped to entity template hierarchies.
   - **Rationale:** A pure-Python Blender addon provides direct export without compiled extensions.
   - **Verification:** Export a Blender scene with two collections; confirm the engine imports them
     as entity template hierarchies.
9. **R-12.6.9** — The engine **SHALL** map Blender Principled BSDF inputs to the engine PBR material
   model, re-link texture inputs as engine assets, and emit warnings with fallback defaults for
   non-mappable nodes.
   - **Rationale:** Automatic material translation eliminates manual material recreation.
   - **Verification:** Export a Principled BSDF material; confirm correct texture bindings; add a
     non-mappable node and confirm a warning with fallback default.

## Marvelous Designer

10. **R-12.6.10** — The engine **SHALL** accept Marvelous Designer exports of garment meshes, UV
    layouts, seam data, and fabric properties, bind garments to character skeletons, and map fabric
    properties to the cloth simulation system (F-4.7.2).
    - **Rationale:** Direct garment export with fabric physics parameters enables runtime cloth
      simulation without manual property setup.
    - **Verification:** Export a garment; confirm skinning to a skeleton succeeds; confirm fabric
      properties drive cloth simulation correctly.
11. **R-12.6.11** — The engine **SHALL** automatically fit imported garments to character bodies by
    computing cloth constraints and skin weight transfers, propagate body morph changes, and
    auto-generate collision bodies.
    - **Rationale:** Automatic fitting eliminates per-character garment adjustment.
    - **Verification:** Fit a garment to two characters with different proportions; confirm
      constraints are computed; apply a body morph and confirm deformation; confirm collision bodies
      are generated.

## Substance

12. **R-12.6.12** — The engine **SHALL** import Substance Designer .sbsar archives, evaluate them at
    build time to produce engine-ready texture sets, and store exposed parameters in the material
    asset for in-engine adjustment.
    - **Rationale:** Build-time evaluation produces optimized textures while preserving parameter
      tweakability.
    - **Verification:** Import a .sbsar with exposed parameters; confirm all texture channels are
      produced; change a parameter and confirm textures regenerate.
13. **R-12.6.13** — The engine **SHALL** import Substance Painter .spp projects, extract
    channel-packed texture maps matching engine conventions, support UDIM tile sets, and allow
    configurable resolution at import time.
    - **Rationale:** Direct .spp import with correct packing removes the manual export and
      re-mapping step.
    - **Verification:** Import a .spp with UDIM tiles; confirm channel packing matches; re-import at
      a different resolution and confirm output changes.
14. **R-12.6.14** — The engine **SHALL** optionally evaluate .sbsar materials at runtime on a
    background thread with async GPU upload, gated by a per-material opt-in flag and a configurable
    evaluation budget.
    - **Rationale:** Runtime parametric evaluation enables dynamic visual variation without
      pre-baking every permutation.
    - **Verification:** Enable runtime evaluation; vary a parameter and confirm the texture updates
      within budget; confirm evaluation does not run when opt-in is disabled.

## Photoshop

15. **R-12.6.15** — The engine **SHALL** accept Photoshop plugin exports of textures, UI sprites,
    and layered compositions with configurable channel packing, PSD round-trip via source file
    references, and DCC bridge real-time texture push.
    - **Rationale:** Direct Photoshop export with channel packing and DCC bridge push keeps texture
      artists in their primary tool.
    - **Verification:** Export a PSD with channel-packed layers; confirm the engine asset matches
      packing; modify and re-export; confirm the update via stored source reference.
16. **R-12.6.16** — The engine **SHALL** convert Photoshop layer groups into engine UI widget
    hierarchies, mapping text layers to text components, shape layers to nine-slice or vector
    elements, and visibility to widget states.
    - **Rationale:** Automatic mockup conversion eliminates manual widget recreation.
    - **Verification:** Export a PSD with text, shape, and nested groups; confirm the widget
      hierarchy with correct properties.

## Illustrator

17. **R-12.6.17** — The engine **SHALL** accept Illustrator vector artwork as vector graphics assets
    or rasterized atlases, map artboards to individual assets, symbols to shared instances, and
    color swatches to engine palettes.
    - **Rationale:** Vector export preserves resolution independence while atlas rasterization
      handles complex artwork.
    - **Verification:** Export an Illustrator file with artboards, symbols, and swatches; confirm
      each produces the correct engine asset type.

## ZBrush

18. **R-12.6.18** — The engine **SHALL** accept ZBrush sculpt exports via GoZ bridge, extract the
    highest-subdivision mesh, generate baked normal/displacement/cavity maps, export polypaint as
    vertex colors or texture, and preserve SubTool hierarchy.
    - **Rationale:** Automated detail-map baking delivers game-ready meshes directly from ZBrush.
    - **Verification:** Export a multi-SubTool sculpt with polypaint; confirm baked maps and vertex
      colors; confirm SubTools map to separate mesh assets.
19. **R-12.6.19** — The engine **SHALL** decimate ZBrush high-poly sculpts to engine-ready poly
    counts, generate multiple LOD levels with per-LOD normal map bakes, and export the LOD chain as
    a single mesh asset.
    - **Rationale:** Generating the full LOD chain from a single sculpt ensures consistent detail
      reduction.
    - **Verification:** Export a sculpt with 3 LOD levels; confirm each LOD has expected poly count
      and normal map; confirm LOD transitions are configured.

## MotionBuilder

20. **R-12.6.20** — The engine **SHALL** accept MotionBuilder exports of skeleton hierarchies,
    animation takes with full curve data, and character definitions, with batch export support.
    - **Rationale:** Curve-based animation export from MotionBuilder preserves mocap cleanup edits.
    - **Verification:** Batch-export 5 takes; confirm all are imported with correct curve data;
      confirm skeletons match the source.
21. **R-12.6.21** — The engine **SHALL** receive real-time skeleton pose data from MotionBuilder
    over the DCC bridge at capture frame rate and apply poses to character entities in the viewport.
    - **Rationale:** Live mocap preview on final characters enables directors to evaluate
      performance during capture.
    - **Verification:** Stream mocap at 60 fps; confirm the character reflects poses with no more
      than 2 frames of latency.

## Git Integration

22. **R-12.6.22** — The engine **SHALL** integrate Git LFS locking into every DCC plugin so that
    opening a source file acquires a lock, a status indicator shows lock state, and saving/closing
    commits, pushes, and releases the lock.
    - **Rationale:** Automatic lock-on-open prevents concurrent edits to binary source files that
      cannot be merged.
    - **Verification:** Open a source file in a DCC plugin and confirm a lock is acquired; attempt
      from a second user and confirm the lock holder is displayed; save and close and confirm the
      lock is released.
23. **R-12.6.23** — The engine **SHALL** display asset review comments in a dedicated panel within
    each DCC plugin, supporting thread viewing, replying, and marking resolved, synchronized via the
    collaboration cloud service (F-15.12.7).
    - **Rationale:** Surfacing review feedback inside the DCC tool eliminates context-switching.
    - **Verification:** Post a comment in the engine editor; open the asset in a DCC plugin and
      confirm the comment appears; reply from the DCC plugin and confirm it syncs.
24. **R-12.6.24** — The engine **SHALL** provide a consistent UI panel across all DCC plugins
    displaying branch, lock status, last commit, pending changes, review comment count, and build
    cache status, with one-click version control actions.
    - **Rationale:** A unified dashboard gives artists the same experience regardless of which tool
      they use.
    - **Verification:** Open the dashboard in Maya, Blender, and Photoshop; confirm identical
      information for the same asset; perform a commit from each and confirm success.

## Shared Features

25. **R-12.6.25** — The engine **SHALL** translate each DCC tool's material system to the engine PBR
    model via a configurable mapping table, convert texture semantics, fall back to defaults for
    missing textures, and support extensibility for custom shaders.
    - **Rationale:** A shared material translation layer ensures consistent PBR results across all
      DCC tools.
    - **Verification:** Export a material from Maya, Blender, and Houdini; confirm equivalent engine
      materials; add a custom mapping and confirm it applies.
26. **R-12.6.26** — The engine **SHALL** support headless batch export via command-line invocation
    of all supported DCC tools, enabling CI pipelines to re-export modified source files and
    validate results using build manifests.
    - **Rationale:** Headless batch export in CI catches broken assets before they reach artists.
    - **Verification:** Batch-export 10 source files across 3 DCC tools; confirm all pass
      validation; modify one and confirm only it is re-exported on the next build.
