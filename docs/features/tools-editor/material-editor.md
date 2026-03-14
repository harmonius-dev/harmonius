# 15.3 — Material Editor

## Node Graph

### F-15.3.1 Node-Based Material Graph

Provides a visual node graph editor for authoring materials by connecting math, texture, and
parameter nodes. The graph compiles to optimized GPU shader code and validates connections with
type-safe pins (scalar, vector, color, texture). Supports copy/paste, node grouping, comments,
and minimap navigation for large graphs used by material artists.

- **Requirements:** R-15.3.1
- **Dependencies:** F-8.1.1, F-15.1.1
- **Platform notes:** None

### F-15.3.2 Material Functions and Subgraphs

Enables reusable subgraphs (material functions) that encapsulate common patterns such as
triplanar mapping, detail normal blending, or parallax occlusion. Functions are saved as
assets, referenced by multiple materials, and updated in a single place. Supports input/output
parameter pins with default values.

- **Requirements:** R-15.3.2
- **Dependencies:** F-15.3.1
- **Platform notes:** None

## Preview and Iteration

### F-15.3.3 Live Material Preview

Renders a real-time 3D preview of the material on configurable meshes (sphere, cube, plane,
custom asset) with adjustable lighting. The preview updates instantly as parameters and graph
connections change, providing immediate visual feedback. Supports split-view comparison of two
material variants side by side.

- **Requirements:** R-15.3.3
- **Dependencies:** F-15.3.1, F-2.3.1
- **Platform notes:** None

### F-15.3.4 Shader Parameter Tweaking

Exposes all material parameters (colors, scalars, textures, toggles) in an inspector panel
with sliders, color pickers, and curve editors. Parameter changes apply instantly to all
viewports without recompilation. Allows content creators to iterate on material look without
editing the node graph.

- **Requirements:** R-15.3.4
- **Dependencies:** F-15.3.1
- **Platform notes:** None

## Material Instancing and Library

### F-15.3.5 Material Instances

Creates lightweight material instances that override specific parameters of a parent material
without duplicating the shader. Instances share the compiled shader program and change only
uniform values, enabling thousands of visual variations (e.g., armor tints, weathering levels)
from a single base material. Essential for MMO-scale asset diversity.

- **Requirements:** R-15.3.5
- **Dependencies:** F-15.3.1, F-15.3.4
- **Platform notes:** None

### F-15.3.6 Material Library and Browser

Provides a searchable, filterable library of all materials and material instances in the
project with thumbnail previews. Supports tagging, categorization, favorites, and usage
tracking (which assets reference a material). Drag-and-drop from the library applies materials
to meshes in the viewport or assigns them in the inspector.

- **Requirements:** R-15.3.6
- **Dependencies:** F-15.3.5, F-15.1.1
- **Platform notes:** None
