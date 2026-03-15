# R-15.3 -- Material Editor User Stories

## US-15.3.1 Node-Based Material Graph

### US-15.3.1.1
As a **artist (P-8)**, I want a visual node graph editor for authoring materials
so that I can create shaders by connecting nodes without writing code.

### US-15.3.1.2
As a **artist (P-8)**, I want type-safe pin connections (scalar, vector, color, texture)
so that invalid connections are rejected before compilation.

### US-15.3.1.3
As a **artist (P-8)**, I want copy/paste of selected nodes with connections preserved
so that I can duplicate graph sections quickly.

### US-15.3.1.4
As a **artist (P-8)**, I want node grouping with collapsible regions
so that I can organize large material graphs into logical sections.

### US-15.3.1.5
As a **artist (P-8)**, I want comment nodes for annotating graph sections
so that other artists understand my material setup.

### US-15.3.1.6
As a **artist (P-8)**, I want minimap navigation for large graphs
so that I can orient within complex material networks quickly.

### US-15.3.1.7
As a **technical artist (P-13)**, I want the graph to compile to optimized GPU shader code
so that visual authoring produces performant shaders.

### US-15.3.1.8
As a **developer (P-15)**, I want material graph compilation errors to reference the
originating node
so that I can locate and fix shader issues from the graph view.

### US-15.3.1.9
As an **engine dev (P-26)**, I want material graphs implemented on the universal logic
graph runtime
so that the same graph infrastructure serves all visual authoring domains.

### US-15.3.1.10
As an **engine tester (P-27)**, I want to verify connecting incompatible pin types is
rejected at edit time
so that type safety is regression-tested.

---

## US-15.3.2 Material Functions and Subgraphs

### US-15.3.2.1
As a **artist (P-8)**, I want reusable material functions saved as assets
so that common patterns like triplanar mapping are authored once.

### US-15.3.2.2
As a **artist (P-8)**, I want material functions with input/output parameter pins
so that I can customize function behavior per-material.

### US-15.3.2.3
As a **technical artist (P-13)**, I want to update a shared material function in one place
so that all referencing materials receive the updated logic.

### US-15.3.2.4
As an **engine tester (P-27)**, I want to verify modifying a material function updates
all referencing materials
so that function propagation is regression-tested.

---

## US-15.3.3 Live Material Preview

### US-15.3.3.1
As a **artist (P-8)**, I want a real-time 3D preview on configurable meshes
so that I can see the material on sphere, cube, plane, or custom geometry.

### US-15.3.3.2
As a **artist (P-8)**, I want preview updates instantly when parameters change
so that I get immediate visual feedback during authoring.

### US-15.3.3.3
As a **artist (P-8)**, I want adjustable lighting in the preview
so that I can evaluate the material under different lighting conditions.

### US-15.3.3.4
As a **artist (P-8)**, I want split-view comparison of two material variants
so that I can evaluate A/B variations side by side.

### US-15.3.3.5
As an **engine tester (P-27)**, I want to verify preview updates within one frame after
a parameter change
so that preview latency meets the 16ms target.

---

## US-15.3.4 Shader Parameter Tweaking

### US-15.3.4.1
As a **artist (P-8)**, I want sliders for scalar material parameters
so that I can adjust values interactively in the inspector.

### US-15.3.4.2
As a **artist (P-8)**, I want color pickers for color parameters
so that I can select exact colors visually.

### US-15.3.4.3
As a **artist (P-8)**, I want curve editors for gradient parameters
so that I can define non-linear parameter distributions.

### US-15.3.4.4
As a **artist (P-8)**, I want parameter changes to apply without shader recompilation
so that iteration speed is not limited by compile time.

### US-15.3.4.5
As a **designer (P-5)**, I want to tweak material parameters on placed entities
so that I can adjust look in context without opening the material editor.

### US-15.3.4.6
As an **engine tester (P-27)**, I want to verify no shader recompilation occurs during
parameter-only changes
so that the uniform-only update path is regression-tested.

---

## US-15.3.5 Material Instances

### US-15.3.5.1
As a **artist (P-8)**, I want lightweight material instances that override parent params
so that I can create thousands of visual variations from one base material.

### US-15.3.5.2
As a **artist (P-8)**, I want instances to share the compiled shader program
so that GPU memory scales with uniform buffers, not shader count.

### US-15.3.5.3
As a **designer (P-5)**, I want to create material instances for per-entity tinting
so that identical objects can have distinct colors without new materials.

### US-15.3.5.4
As an **engine dev (P-26)**, I want material instances to avoid shader duplication
so that draw call batching is not fragmented by material variants.

### US-15.3.5.5
As an **engine tester (P-27)**, I want to verify 100 instances share one compiled shader
so that instance efficiency is regression-tested.

---

## US-15.3.6 Material Library and Browser

### US-15.3.6.1
As a **artist (P-8)**, I want a searchable library of all project materials
so that I can find existing materials instead of creating duplicates.

### US-15.3.6.2
As a **artist (P-8)**, I want thumbnail previews in the material browser
so that I can visually identify materials without opening each one.

### US-15.3.6.3
As a **artist (P-8)**, I want tagging and categorization of materials
so that I can organize materials by surface type or environment.

### US-15.3.6.4
As a **artist (P-8)**, I want drag-and-drop material application to meshes
so that I can assign materials directly from the library to the viewport.

### US-15.3.6.5
As a **artist (P-8)**, I want favorites for frequently used materials
so that I can access my most-used materials with one click.

### US-15.3.6.6
As a **technical artist (P-13)**, I want usage tracking showing which assets reference
a material
so that I can assess impact before modifying a shared material.

### US-15.3.6.7
As an **engine tester (P-27)**, I want to verify search by tag returns only matching
materials
so that library filtering is regression-tested.
