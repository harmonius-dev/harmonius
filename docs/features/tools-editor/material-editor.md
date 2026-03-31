# 15.3 — Material Editor

## Node Graph

| ID | Feature |
| ---------- | ---------------------------------- |
| F-15.3.1 | Node-Based Material Graph |
| F-15.3.2 | Material Functions and Subgraphs |

1. **F-15.3.1** — Provides a visual node graph editor for authoring materials by connecting math,
   texture, and parameter nodes. The graph compiles to optimized GPU shader code and validates
   connections with type-safe pins (scalar, vector, color, texture). Supports copy/paste, node
   grouping, comments, and minimap navigation for large graphs used by material artists.
   - **Deps:** F-2.1.1, F-15.1.1
   - **Platform:** Desktop only. Not available on mobile or console runtime.
2. **F-15.3.2** — Enables reusable subgraphs (material functions) that encapsulate common patterns
   such as triplanar mapping, detail normal blending, or parallax occlusion. Functions are saved as
   assets, referenced by multiple materials, and updated in a single place. Supports input/output
   parameter pins with default values.
   - **Deps:** F-15.3.1
   - **Platform:** Desktop only. Not available on mobile or console runtime.

## Preview and Iteration

| ID | Feature |
| ---------- | --------------------------- |
| F-15.3.3 | Live Material Preview |
| F-15.3.4 | Shader Parameter Tweaking |

1. **F-15.3.3** — Renders a real-time 3D preview of the material on configurable meshes (sphere,
   cube, plane, custom asset) with adjustable lighting. The preview updates instantly as parameters
   and graph connections change, providing immediate visual feedback. Supports split-view comparison
   of two material variants side by side.
   - **Deps:** F-15.3.1, F-2.3.1
   - **Platform:** Desktop only. Not available on mobile or console runtime.
2. **F-15.3.4** — Exposes all material parameters (colors, scalars, textures, toggles) in an
   inspector panel with sliders, color pickers, and curve editors. Parameter changes apply instantly
   to all viewports without recompilation. Allows content creators to iterate on material look
   without editing the node graph.
   - **Deps:** F-15.3.1
   - **Platform:** Desktop only. Not available on mobile or console runtime.

## Material Instancing and Library

| ID | Feature |
| ---------- | ------------------------------ |
| F-15.3.5 | Material Instances |
| F-15.3.6 | Material Library and Browser |

1. **F-15.3.5** — Creates lightweight material instances that override specific parameters of a
   parent material without duplicating the shader. Instances share the compiled shader program and
   change only uniform values, enabling thousands of visual variations (e.g., armor tints,
   weathering levels) from a single base material. Essential for MMO-scale asset diversity.
   - **Deps:** F-15.3.1, F-15.3.4
   - **Platform:** Desktop only. Not available on mobile or console runtime.
2. **F-15.3.6** — Provides a searchable, filterable library of all materials and material instances
   in the project with thumbnail previews. Supports tagging, categorization, favorites, and usage
   tracking (which assets reference a material). Drag-and-drop from the library applies materials to
   meshes in the viewport or assigns them in the inspector.
   - **Deps:** F-15.3.5, F-15.1.1
   - **Platform:** Desktop only. Not available on mobile or console runtime.
