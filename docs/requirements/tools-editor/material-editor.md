# R-15.3 — Material Editor Requirements

## Node Graph

### R-15.3.1 Node-Based Material Graph

The engine **SHALL** provide a visual node graph editor for material authoring with type-safe
pin connections (scalar, vector, color, texture), compilation to optimized GPU shader code, and
graph management features including copy/paste, node grouping, comments, and minimap navigation.
The material node graph is implemented as a specialized view of the universal logic graph system
(R-15.8.5). The material editor provides a domain-specific UX layer (material preview, parameter
sliders, shader compilation feedback) on top of the shared graph runtime.

- **Derived from:** [F-15.3.1](../../features/tools-editor/material-editor.md)
- **Rationale:** A visual node graph enables material artists to author shaders without writing code
  while type-safe pins prevent invalid connections that would produce shader compilation errors.
- **Verification:** Integration test: create a material graph connecting a texture sample node to a
  base color output. Compile and verify valid shader code is generated. Attempt to connect a scalar
  output to a texture input and verify the connection is rejected. Verify copy/paste duplicates
  selected nodes with correct connections. Verify node grouping collapses and expands correctly.

### R-15.3.2 Material Functions and Subgraphs

The engine **SHALL** support reusable material function assets (subgraphs) with input/output
parameter pins and default values, referenceable by multiple materials and updatable from a
single source.

- **Derived from:** [F-15.3.2](../../features/tools-editor/material-editor.md)
- **Rationale:** Shared material functions (triplanar mapping, detail normal blending) eliminate
  duplication and ensure consistency when updating common patterns across many materials.
- **Verification:** Unit test: create a material function with two inputs and one output. Reference
  it from two materials with different input values and verify both compile correctly. Modify the
  function's internal logic and verify both referencing materials produce updated shader output
  without manual intervention.

## Preview and Iteration

### R-15.3.3 Live Material Preview

The engine **SHALL** render a real-time 3D preview of the material on configurable meshes (sphere,
cube, plane, custom asset) with adjustable lighting, updating instantly as parameters and graph
connections change, and supporting split-view comparison of two material variants.

- **Derived from:** [F-15.3.3](../../features/tools-editor/material-editor.md)
- **Rationale:** Instant visual feedback during material authoring eliminates the compile-and-check
  cycle, and side-by-side comparison helps artists evaluate variations efficiently.
- **Verification:** Integration test: open the material preview on a sphere, change a color
  parameter, and verify the preview updates within one frame. Switch the preview mesh to a custom
  asset and verify the material renders correctly. Activate split-view with two material variants
  and verify both previews render simultaneously with independent parameters.

### R-15.3.4 Shader Parameter Tweaking

The engine **SHALL** expose all material parameters (colors, scalars, textures, toggles) in an
inspector panel with appropriate controls (sliders, color pickers, curve editors), applying
changes instantly to all viewports without shader recompilation.

- **Derived from:** [F-15.3.4](../../features/tools-editor/material-editor.md)
- **Rationale:** Parameter tweaking without recompilation enables rapid artist iteration on material
  appearance directly in the scene context.
- **Verification:** Integration test: open a material in the inspector, adjust a scalar slider, and
  verify the change is visible in all open viewports within one frame. Verify no shader
  recompilation occurs by monitoring GPU pipeline state. Toggle a boolean parameter and verify the
  material variant switches instantly.

## Non-Functional Requirements

### R-15.3.NF1 Material Preview Update Latency

Live material preview updates **SHALL** appear within one frame (under 16ms) after a parameter
change. Shader recompilation triggered by graph topology changes **SHALL** complete within 2
seconds for materials with up to 200 nodes. The material library browser **SHALL** render thumbnail
previews for up to 1,000 materials within 5 seconds of opening.

- **Derived from:** F-15.3.1 through F-15.3.6 (all material editor features).
- **Rationale:** Material artists iterate rapidly on parameter values and expect instant visual
  feedback. Slow shader recompilation or preview lag interrupts the creative flow.
- **Verification:** Change a color parameter in the material inspector; measure time to preview
  update and assert under 16ms. Modify the graph topology of a 200-node material; measure
  recompilation time and assert under 2 seconds. Open the material library with 1,000 materials and
  assert all thumbnails render within 5 seconds.

## Material Instancing and Library

### R-15.3.5 Material Instances

The engine **SHALL** support lightweight material instances that override specific parameters of a
parent material without duplicating the compiled shader, sharing the shader program and changing
only uniform values.

- **Derived from:** [F-15.3.5](../../features/tools-editor/material-editor.md)
- **Rationale:** Shader sharing across instances enables thousands of visual variations (armor tints,
  weathering levels) without multiplying shader permutations or GPU memory usage.
- **Verification:** Unit test: create 100 material instances from one parent, each overriding a
  different color parameter. Verify all instances reference the same compiled shader handle. Verify
  each instance renders with its unique color. Verify GPU memory usage does not scale linearly
  with instance count beyond uniform buffer growth.

### R-15.3.6 Material Library and Browser

The engine **SHALL** provide a searchable, filterable material library with thumbnail previews,
tagging, categorization, favorites, usage tracking, and drag-and-drop application of materials
to meshes in the viewport or inspector.

- **Derived from:** [F-15.3.6](../../features/tools-editor/material-editor.md)
- **Rationale:** A centralized library with search and usage tracking prevents duplicate materials
  and enables artists to discover and reuse existing assets efficiently.
- **Verification:** Integration test: add three materials with distinct tags. Search by tag and
  verify only matching materials appear. Verify thumbnail previews render correctly. Drag a
  material from the library onto a mesh and verify the material is assigned. Query usage tracking
  and verify it reports the correct list of assets referencing the material.
