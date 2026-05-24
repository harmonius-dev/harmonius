# Visual Editors

Specialized visual tools for content creation and authoring.

## What it covers

- Material editor: graphical material parameter authoring.
- Shader editor: node-based shader graph authoring.
- Particle effect editor: visual particle system design.
- Animation editor: keyframing and timeline authoring.
- State machine editor: visual behavior tree and state machine editing.
- Terrain editor: sculpting and painting heightmaps.
- Lightmap baker: pre-baking static lighting.
- Prefab editor: isolated prefab editing.
- Timeline/sequencer: cinematic and cutscene editing.
- Dialogue editor: branching conversation tree authoring.

## Concepts

### Visual Graph Editors

Material editor uses node-based graphs: input nodes (texture samplers, constants) connect to output
nodes (final color). Connecting nodes computes material properties. Shader editor similarly creates
node graphs compiling to shaders. Visual graphs avoid writing code; artists author via nodes.

### Timeline Editors

Timeline editors display events (animations, camera changes, audio) on a horizontal time axis. Each
track represents an event type. Scrubbing the timeline preview events at that time. Keyframing
records animation frames; inbetween frames interpolate. Cinematic timelines control camera,
characters, and effects for cutscenes.

### Terrain Sculpting

Terrain editor displays heightmap. Sculpting brushes (raise, lower, smooth) modify heights. Painting
assigns materials (grass, rock) to terrain. Undo/redo tracks terrain changes. Real-time preview
shows sculpted terrain in main editor viewport.

### Prefab Isolation

Prefab editor opens a prefab in isolation: edit components, add/remove children without affecting
instances in scenes. Changes apply to all instances. Instance overrides allow per-instance
customization while maintaining base prefab structure.

## How it fits

- See [editor-framework.md](./editor-framework.md) for core editor functionality.
- See [profiling-and-collaboration.md](./profiling-and-collaboration.md) for performance and
  collaboration tools.
- See [../rendering/effects-and-styles.md](../rendering/effects-and-styles.md) for material and
  shader concepts.
