# User Stories: Material Editor

## F-15.3.1 Node-Based Material Graph

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.3.1.1 | artist (P-8) | a visual node graph editor for authoring materials by connecting math, texture, and parameter nodes with type-safe pins | I can create complex materials without writing shader code |  |  |
| US-15.3.1.2 | tech artist (P-13) | copy/paste, node grouping, comments, and minimap navigation in the material graph | I can organize and navigate large material graphs with dozens of nodes efficiently |  |  |
| US-15.3.1.3 | designer (P-5) | type-safe pin validation that prevents invalid connections (e.g., connecting a color to a scalar) | I catch material errors at edit time rather than at runtime |  |  |
| US-15.3.1.4 | engine developer (P-26) | the material graph to compile to optimized GPU shader code | visually authored materials achieve the same performance as hand-written shaders |  |  |

## F-15.3.2 Material Functions and Subgraphs

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.3.2.1 | tech artist (P-13) | to create reusable subgraphs (material functions) encapsulating common patterns like triplanar mapping and detail normal blending | I can share proven material techniques across multiple materials without duplicating nodes |  |  |
| US-15.3.2.2 | artist (P-8) | to reference material functions saved as assets from multiple materials | updating a single function automatically improves all materials that use it |  |  |
| US-15.3.2.3 | developer (P-15) | material functions to support input/output parameter pins with default values | function consumers can customize behavior without editing the function internals |  |  |
| US-15.3.2.4 | engine tester (P-27) | to verify that updating a material function propagates correctly to all referencing materials without introducing compilation errors | shared functions can be safely improved without breaking dependent materials |  |  |

## F-15.3.3 Live Material Preview

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.3.3.1 | artist (P-8) | a real-time 3D preview of my material on configurable meshes (sphere, cube, plane, custom asset) with adjustable lighting | I can evaluate the material's appearance on different surface shapes |  |  |
| US-15.3.3.2 | creative director (P-2) | split-view comparison of two material variants side by side | I can make informed decisions about which material direction to approve |  |  |
| US-15.3.3.3 | tech artist (P-13) | the preview to update instantly as I change graph connections and parameters | I can iterate rapidly without waiting for shader recompilation |  |  |
| US-15.3.3.4 | designer (P-5) | to preview materials on the actual in-scene mesh by selecting it in the viewport | I can evaluate how the material looks in its final game context |  |  |

## F-15.3.4 Shader Parameter Tweaking

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.3.4.1 | artist (P-8) | all material parameters (colors, scalars, textures, toggles) exposed in an inspector panel with sliders, color pickers, and curve editors | I can iterate on material look without opening the node graph |  |  |
| US-15.3.4.2 | designer (P-5) | parameter changes applied instantly to all viewports without recompilation | I can rapidly tune material appearance during level design |  |  |
| US-15.3.4.3 | creative director (P-2) | to adjust base color, roughness, and metallic sliders with instant viewport feedback | I can direct material look during art reviews without technical knowledge of the node graph |  |  |
| US-15.3.4.4 | engine developer (P-26) | parameter changes to update uniform values without recompiling the shader program | tweaking is always interactive regardless of shader complexity |  |  |

## F-15.3.5 Material Instances

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.3.5.1 | artist (P-8) | to create material instances that override specific parameters of a parent material without duplicating the shader | I can produce thousands of visual variations (armor tints, weathering levels) with minimal memory overhead |  |  |
| US-15.3.5.2 | designer (P-5) | material instances to share the compiled shader program and change only uniform values | adding color variations to props does not increase draw call count |  |  |
| US-15.3.5.3 | tech artist (P-13) | to see how many instances derive from a given parent material | I can evaluate the instancing hierarchy and identify consolidation opportunities |  |  |
| US-15.3.5.4 | engine tester (P-27) | to verify that all instances of a parent material share the same compiled shader program at runtime | the instancing system delivers its promised performance benefits |  |  |

## F-15.3.6 Material Library and Browser

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.3.6.1 | artist (P-8) | a searchable, filterable library of all materials and material instances with thumbnail previews | I can quickly find and apply existing materials instead of creating duplicates |  |  |
| US-15.3.6.2 | designer (P-5) | to drag-and-drop materials from the library onto meshes in the viewport | I can apply materials visually without navigating the inspector |  |  |
| US-15.3.6.3 | tech artist (P-13) | usage tracking showing which assets reference a material | I can safely deprecate unused materials and identify high-impact materials worth optimizing |  |  |
| US-15.3.6.4 | creative director (P-2) | materials organized by tags and categories with favorites support | I can browse the material palette by visual theme during art direction reviews |  |  |
