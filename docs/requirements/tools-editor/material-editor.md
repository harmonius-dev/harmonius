# R-15.3 -- Material Editor Requirements

## Node-Based Material Graph

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.3.1 | The editor **SHALL** provide a visual node graph for material authoring with type-safe pin connections (scalar, vector, color, texture), copy/paste, node grouping, comments, minimap navigation, and compilation to optimized GPU shader code, built on the universal logic graph runtime (F-15.8.1). | [F-15.3.1](../../features/tools-editor/material-editor.md) | No-code material authoring requires a visual graph with type safety to prevent invalid shader combinations. | Unit test: connect incompatible pin types and verify rejection at edit time. Build a valid graph and verify compiled shader runs on GPU. |
| R-15.3.2 | The editor **SHALL** support reusable material functions saved as assets with input/output parameter pins, where modifying a shared function updates all referencing materials. | [F-15.3.2](../../features/tools-editor/material-editor.md) | Common patterns (triplanar mapping, detail blending) must be authored once and reused across all materials. | Unit test: modify a shared function and verify all referencing materials reflect the update. |

## Live Preview

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.3.3 | The editor **SHALL** render a real-time 3D material preview on configurable meshes with adjustable lighting, updating within one frame (16 ms) after parameter changes, and supporting split-view comparison of two variants. | [F-15.3.3](../../features/tools-editor/material-editor.md) | Immediate visual feedback is essential for iterative material authoring. | Benchmark: change a parameter and measure frames until the preview updates. Verify update occurs within one frame. |
| R-15.3.4 | The editor **SHALL** expose material parameters (colors, scalars, textures) in an inspector with sliders, color pickers, and curve editors, applying changes instantly without shader recompilation. | [F-15.3.4](../../features/tools-editor/material-editor.md) | Parameter iteration must not be gated by compile time; uniform-only updates enable real-time tweaking. | Unit test: change a scalar parameter and verify no shader recompilation occurs. Verify the viewport reflects the new value. |

## Material Instances

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.3.5 | The editor **SHALL** support lightweight material instances that override parent parameters without duplicating the compiled shader, sharing one shader program across all instances. | [F-15.3.5](../../features/tools-editor/material-editor.md) | Thousands of visual variations from a single base material keep GPU memory proportional to uniform buffers, not shader count. | Unit test: create 100 instances from one parent and verify all share a single compiled shader. |

## Material Library

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|--------------|-----------|--------------|
| R-15.3.6 | The editor **SHALL** provide a searchable, taggable material library with thumbnail previews, drag-and-drop application to meshes, favorites, and usage tracking showing which assets reference a material. | [F-15.3.6](../../features/tools-editor/material-editor.md) | A searchable library prevents duplicate material creation and accelerates asset discovery. | Unit test: tag a material, search by tag, and verify only matching materials are returned. |

---

## User Story Traceability

User stories for this domain are maintained in
[user-stories/tools-editor/material-editor.md](../../user-stories/tools-editor/material-editor.md).
Requirements in this document are derived from those user stories.
