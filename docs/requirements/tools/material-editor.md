# R-15.3 -- Material Editor Requirements

## Requirements

1. **R-15.3.1** — The engine **SHALL** provide a node-based material graph editor with type-safe
   pins, copy/paste, node grouping, comments, and minimap navigation that compiles to optimized GPU
   shader code.
   - **Rationale:** Visual material authoring is the primary workflow for all material creation in a
     no-code engine.
   - **Verification:** Create a material graph with 20+ nodes, verify type errors on mismatched
     connections, and confirm the compiled shader renders correctly.

2. **R-15.3.2** — The engine **SHALL** support reusable material function subgraphs saved as assets
   with input/output parameter pins and default values.
   - **Rationale:** Shared functions eliminate duplication of common patterns like triplanar
     mapping.
   - **Verification:** Create a function, reference it from two materials, modify the function, and
     verify both materials update.

3. **R-15.3.3** — The engine **SHALL** render a real-time 3D material preview on configurable meshes
   with adjustable lighting and split-view comparison of two variants.
   - **Rationale:** Immediate visual feedback reduces material iteration time from minutes to
     seconds.
   - **Verification:** Change a material parameter and verify the preview updates within one frame.

4. **R-15.3.4** — The engine **SHALL** expose all material parameters in an inspector panel with
   instant application to viewports without recompilation.
   - **Rationale:** Parameter tweaking must be fast enough for real-time artistic iteration.
   - **Verification:** Adjust a color parameter via slider and verify the viewport reflects the
     change immediately.

5. **R-15.3.5** — The engine **SHALL** support lightweight material instances that override specific
   parent parameters while sharing the compiled shader program.
   - **Rationale:** Instances enable thousands of visual variations without duplicating shader
     permutations.
   - **Verification:** Create 100 instances with different tint colors and verify they all reference
     the same compiled shader.

6. **R-15.3.6** — The engine **SHALL** provide a searchable, filterable material library with
   thumbnail previews, tags, usage tracking, and drag-and-drop assignment.
   - **Rationale:** Discoverability and quick assignment reduce time spent searching for existing
     materials.
   - **Verification:** Tag a material, search by tag, and verify drag-and-drop onto a mesh applies
     the material.
