# R-15.3 -- Material Editor Requirements

## Node-Based Material Graph

### R-15.3.1 Node-Based Material Graph

The editor **SHALL** provide a visual node graph for material authoring with type-safe pin
connections (scalar, vector, color, texture), copy/paste, node grouping, comments, minimap
navigation, and compilation to optimized GPU shader code, built on the universal logic graph runtime
(F-15.8.1).

- **Derived from:**
  [F-15.3.1](../../features/tools-editor/material-editor.md)
- **Rationale:** No-code material authoring requires a visual graph with type safety to prevent
  invalid shader combinations.
- **Verification:** Unit test: connect incompatible pin types and verify rejection at edit time.
  Build a valid graph and verify compiled shader runs on GPU.

### R-15.3.2 Material Functions and Subgraphs

The editor **SHALL** support reusable material functions saved as assets with input/output parameter
pins, where modifying a shared function updates all referencing materials.

- **Derived from:**
  [F-15.3.2](../../features/tools-editor/material-editor.md)
- **Rationale:** Common patterns (triplanar mapping, detail blending) must be authored once and
  reused across all materials.
- **Verification:** Unit test: modify a shared function and verify all referencing materials reflect
  the update.

## Live Preview

### R-15.3.3 Live Material Preview

The editor **SHALL** render a real-time 3D material preview on configurable meshes with adjustable
lighting, updating within one frame (16 ms) after parameter changes, and supporting split-view
comparison of two variants.

- **Derived from:**
  [F-15.3.3](../../features/tools-editor/material-editor.md)
- **Rationale:** Immediate visual feedback is essential for iterative material authoring.
- **Verification:** Benchmark: change a parameter and measure frames until the preview updates.
  Verify update occurs within one frame.

### R-15.3.4 Shader Parameter Tweaking

The editor **SHALL** expose material parameters (colors, scalars, textures) in an inspector with
sliders, color pickers, and curve editors, applying changes instantly without shader recompilation.

- **Derived from:**
  [F-15.3.4](../../features/tools-editor/material-editor.md)
- **Rationale:** Parameter iteration must not be gated by compile time; uniform-only updates enable
  real-time tweaking.
- **Verification:** Unit test: change a scalar parameter and verify no shader recompilation occurs.
  Verify the viewport reflects the new value.

## Material Instances

### R-15.3.5 Material Instances

The editor **SHALL** support lightweight material instances that override parent parameters without
duplicating the compiled shader, sharing one shader program across all instances.

- **Derived from:**
  [F-15.3.5](../../features/tools-editor/material-editor.md)
- **Rationale:** Thousands of visual variations from a single base material keep GPU memory
  proportional to uniform buffers, not shader count.
- **Verification:** Unit test: create 100 instances from one parent and verify all share a single
  compiled shader.

## Material Library

### R-15.3.6 Material Library and Browser

The editor **SHALL** provide a searchable, taggable material library with thumbnail previews,
drag-and-drop application to meshes, favorites, and usage tracking showing which assets reference a
material.

- **Derived from:**
  [F-15.3.6](../../features/tools-editor/material-editor.md)
- **Rationale:** A searchable library prevents duplicate material creation and accelerates asset
  discovery.
- **Verification:** Unit test: tag a material, search by tag, and verify only matching materials are
  returned.

---

## User Story Traceability

User stories for this domain are maintained in
[user-stories/tools-editor/material-editor.md](../../user-stories/tools-editor/material-editor.md).
Requirements in this document are derived from those
user stories.
