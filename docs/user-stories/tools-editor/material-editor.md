# User Stories: Material Editor

## F-15.3.1 Node-Based Material Graph

## US-15.3.1.1 Artist Authors Material Visually
**As an** artist (P-8), **I want** a visual node graph editor for authoring materials by connecting
math, texture, and parameter nodes with type-safe pins, **so that** I can create complex materials
without writing shader code.

## US-15.3.1.2 Tech Artist Uses Minimap Navigation
**As a** tech artist (P-13), **I want** copy/paste, node grouping, comments, and minimap navigation
in the material graph, **so that** I can organize and navigate large material graphs with dozens of
nodes efficiently.

## US-15.3.1.3 Designer Validates Pin Connections
**As a** designer (P-5), **I want** type-safe pin validation that prevents invalid connections
(e.g., connecting a color to a scalar), **so that** I catch material errors at edit time rather than
at runtime.

## US-15.3.1.4 Engine Dev Compiles to GPU Shader
**As an** engine developer (P-26), **I want** the material graph to compile to optimized GPU shader
code, **so that** visually authored materials achieve the same performance as hand-written shaders.

## F-15.3.2 Material Functions and Subgraphs

## US-15.3.2.1 Tech Artist Creates Reusable Subgraph
**As a** tech artist (P-13), **I want** to create reusable subgraphs (material functions)
encapsulating common patterns like triplanar mapping and detail normal blending, **so that** I can
share proven material techniques across multiple materials without duplicating nodes.

## US-15.3.2.2 Artist References Shared Function
**As an** artist (P-8), **I want** to reference material functions saved as assets from multiple
materials, **so that** updating a single function automatically improves all materials that use it.

## US-15.3.2.3 Developer Defines Function Parameters
**As a** developer (P-15), **I want** material functions to support input/output parameter pins with
default values, **so that** function consumers can customize behavior without editing the function
internals.

## US-15.3.2.4 Engine Tester Validates Function Updates
**As an** engine tester (P-27), **I want** to verify that updating a material function propagates
correctly to all referencing materials without introducing compilation errors, **so that** shared
functions can be safely improved without breaking dependent materials.

## F-15.3.3 Live Material Preview

## US-15.3.3.1 Artist Previews on Custom Mesh
**As an** artist (P-8), **I want** a real-time 3D preview of my material on configurable meshes
(sphere, cube, plane, custom asset) with adjustable lighting, **so that** I can evaluate the
material's appearance on different surface shapes.

## US-15.3.3.2 Creative Director Compares Variants
**As a** creative director (P-2), **I want** split-view comparison of two material variants side by
side, **so that** I can make informed decisions about which material direction to approve.

## US-15.3.3.3 Tech Artist Iterates with Instant Feedback
**As a** tech artist (P-13), **I want** the preview to update instantly as I change graph
connections and parameters, **so that** I can iterate rapidly without waiting for shader
recompilation.

## US-15.3.3.4 Designer Previews in Context
**As a** designer (P-5), **I want** to preview materials on the actual in-scene mesh by selecting it
in the viewport, **so that** I can evaluate how the material looks in its final game context.

## F-15.3.4 Shader Parameter Tweaking

## US-15.3.4.1 Artist Tweaks Parameters Without Graph
**As an** artist (P-8), **I want** all material parameters (colors, scalars, textures, toggles)
exposed in an inspector panel with sliders, color pickers, and curve editors, **so that** I can
iterate on material look without opening the node graph.

## US-15.3.4.2 Designer Applies Changes Instantly
**As a** designer (P-5), **I want** parameter changes applied instantly to all viewports without
recompilation, **so that** I can rapidly tune material appearance during level design.

## US-15.3.4.3 Creative Director Reviews Color Palette
**As a** creative director (P-2), **I want** to adjust base color, roughness, and metallic sliders
with instant viewport feedback, **so that** I can direct material look during art reviews without
technical knowledge of the node graph.

## US-15.3.4.4 Engine Dev Ensures Hot Reload
**As an** engine developer (P-26), **I want** parameter changes to update uniform values without
recompiling the shader program, **so that** tweaking is always interactive regardless of shader
complexity.

## F-15.3.5 Material Instances

## US-15.3.5.1 Artist Creates Lightweight Variation
**As an** artist (P-8), **I want** to create material instances that override specific parameters of
a parent material without duplicating the shader, **so that** I can produce thousands of visual
variations (armor tints, weathering levels) with minimal memory overhead.

## US-15.3.5.2 Designer Tints Without New Shader
**As a** designer (P-5), **I want** material instances to share the compiled shader program and
change only uniform values, **so that** adding color variations to props does not increase draw call
count.

## US-15.3.5.3 Tech Artist Audits Instance Count
**As a** tech artist (P-13), **I want** to see how many instances derive from a given parent
material, **so that** I can evaluate the instancing hierarchy and identify consolidation
opportunities.

## US-15.3.5.4 Engine Tester Validates Shader Sharing
**As an** engine tester (P-27), **I want** to verify that all instances of a parent material share
the same compiled shader program at runtime, **so that** the instancing system delivers its promised
performance benefits.

## F-15.3.6 Material Library and Browser

## US-15.3.6.1 Artist Searches Material Library
**As an** artist (P-8), **I want** a searchable, filterable library of all materials and material
instances with thumbnail previews, **so that** I can quickly find and apply existing materials
instead of creating duplicates.

## US-15.3.6.2 Designer Drags Material to Viewport
**As a** designer (P-5), **I want** to drag-and-drop materials from the library onto meshes in the
viewport, **so that** I can apply materials visually without navigating the inspector.

## US-15.3.6.3 Tech Artist Tracks Material Usage
**As a** tech artist (P-13), **I want** usage tracking showing which assets reference a material,
**so that** I can safely deprecate unused materials and identify high-impact materials worth
optimizing.

## US-15.3.6.4 Creative Director Browses by Category
**As a** creative director (P-2), **I want** materials organized by tags and categories with
favorites support, **so that** I can browse the material palette by visual theme during art
direction reviews.
