# User Stories: Material Editor

## F-15.3.1 Node-Based Material Graph

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-15.3.1.1 | artist (P-8)            |          |              |
| US-15.3.1.2 | tech artist (P-13)      |          |              |
| US-15.3.1.3 | designer (P-5)          |          |              |
| US-15.3.1.4 | engine developer (P-26) |          |              |

1. **US-15.3.1.1** — a visual node graph editor for authoring materials by connecting math, texture,
   and parameter nodes with type-safe pins
   - **Acceptance:** I can create complex materials without writing shader code
2. **US-15.3.1.2** — copy/paste, node grouping, comments, and minimap navigation in the material
   graph
   - **Acceptance:** I can organize and navigate large material graphs with dozens of nodes
     efficiently
3. **US-15.3.1.3** — type-safe pin validation that prevents invalid connections (e.g., connecting a
   color to a scalar)
   - **Acceptance:** I catch material errors at edit time rather than at runtime
4. **US-15.3.1.4** — the material graph to compile to optimized GPU shader code
   - **Acceptance:** visually authored materials achieve the same performance as hand-written
     shaders

## F-15.3.2 Material Functions and Subgraphs

| ID          | Persona              | Features | Requirements |
|-------------|----------------------|----------|--------------|
| US-15.3.2.1 | tech artist (P-13)   |          |              |
| US-15.3.2.2 | artist (P-8)         |          |              |
| US-15.3.2.3 | developer (P-15)     |          |              |
| US-15.3.2.4 | engine tester (P-27) |          |              |

1. **US-15.3.2.1** — to create reusable subgraphs (material functions) encapsulating common patterns
   like triplanar mapping and detail normal blending
   - **Acceptance:** I can share proven material techniques across multiple materials without
     duplicating nodes
2. **US-15.3.2.2** — to reference material functions saved as assets from multiple materials
   - **Acceptance:** updating a single function automatically improves all materials that use it
3. **US-15.3.2.3** — material functions to support input/output parameter pins with default values
   - **Acceptance:** function consumers can customize behavior without editing the function
     internals
4. **US-15.3.2.4** — to verify that updating a material function propagates correctly to all
   referencing materials without introducing compilation errors
   - **Acceptance:** shared functions can be safely improved without breaking dependent materials

## F-15.3.3 Live Material Preview

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-15.3.3.1 | artist (P-8)            |          |              |
| US-15.3.3.2 | creative director (P-2) |          |              |
| US-15.3.3.3 | tech artist (P-13)      |          |              |
| US-15.3.3.4 | designer (P-5)          |          |              |

1. **US-15.3.3.1** — a real-time 3D preview of my material on configurable meshes (sphere, cube,
   plane, custom asset) with adjustable lighting
   - **Acceptance:** I can evaluate the material's appearance on different surface shapes
2. **US-15.3.3.2** — split-view comparison of two material variants side by side
   - **Acceptance:** I can make informed decisions about which material direction to approve
3. **US-15.3.3.3** — the preview to update instantly as I change graph connections and parameters
   - **Acceptance:** I can iterate rapidly without waiting for shader recompilation
4. **US-15.3.3.4** — to preview materials on the actual in-scene mesh by selecting it in the
   viewport
   - **Acceptance:** I can evaluate how the material looks in its final game context

## F-15.3.4 Shader Parameter Tweaking

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-15.3.4.1 | artist (P-8)            |          |              |
| US-15.3.4.2 | designer (P-5)          |          |              |
| US-15.3.4.3 | creative director (P-2) |          |              |
| US-15.3.4.4 | engine developer (P-26) |          |              |

1. **US-15.3.4.1** — all material parameters (colors, scalars, textures, toggles) exposed in an
   inspector panel with sliders, color pickers, and curve editors
   - **Acceptance:** I can iterate on material look without opening the node graph
2. **US-15.3.4.2** — parameter changes applied instantly to all viewports without recompilation
   - **Acceptance:** I can rapidly tune material appearance during level design
3. **US-15.3.4.3** — to adjust base color, roughness, and metallic sliders with instant viewport
   feedback
   - **Acceptance:** I can direct material look during art reviews without technical knowledge of
     the node graph
4. **US-15.3.4.4** — parameter changes to update uniform values without recompiling the shader
   program
   - **Acceptance:** tweaking is always interactive regardless of shader complexity

## F-15.3.5 Material Instances

| ID          | Persona              | Features | Requirements |
|-------------|----------------------|----------|--------------|
| US-15.3.5.1 | artist (P-8)         |          |              |
| US-15.3.5.2 | designer (P-5)       |          |              |
| US-15.3.5.3 | tech artist (P-13)   |          |              |
| US-15.3.5.4 | engine tester (P-27) |          |              |

1. **US-15.3.5.1** — to create material instances that override specific parameters of a parent
   material without duplicating the shader
   - **Acceptance:** I can produce thousands of visual variations (armor tints, weathering levels)
     with minimal memory overhead
2. **US-15.3.5.2** — material instances to share the compiled shader program and change only uniform
   values
   - **Acceptance:** adding color variations to props does not increase draw call count
3. **US-15.3.5.3** — to see how many instances derive from a given parent material
   - **Acceptance:** I can evaluate the instancing hierarchy and identify consolidation
     opportunities
4. **US-15.3.5.4** — to verify that all instances of a parent material share the same compiled
   shader program at runtime
   - **Acceptance:** the instancing system delivers its promised performance benefits

## F-15.3.6 Material Library and Browser

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-15.3.6.1 | artist (P-8)            |          |              |
| US-15.3.6.2 | designer (P-5)          |          |              |
| US-15.3.6.3 | tech artist (P-13)      |          |              |
| US-15.3.6.4 | creative director (P-2) |          |              |

1. **US-15.3.6.1** — a searchable, filterable library of all materials and material instances with
   thumbnail previews
   - **Acceptance:** I can quickly find and apply existing materials instead of creating duplicates
2. **US-15.3.6.2** — to drag-and-drop materials from the library onto meshes in the viewport
   - **Acceptance:** I can apply materials visually without navigating the inspector
3. **US-15.3.6.3** — usage tracking showing which assets reference a material
   - **Acceptance:** I can safely deprecate unused materials and identify high-impact materials
     worth optimizing
4. **US-15.3.6.4** — materials organized by tags and categories with favorites support
   - **Acceptance:** I can browse the material palette by visual theme during art direction reviews
