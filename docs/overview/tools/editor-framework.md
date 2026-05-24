# Editor Framework

Editor UI, property inspection, and workflow tools.

## What it covers

- Scene editor: hierarchical entity editing and visualization.
- Property inspector: editing entity components and properties.
- Undo/redo system: reverting changes and reapplying actions.
- Selection and gizmos: selecting and transforming entities.
- Asset browser: searching and loading assets.
- Project settings: game configuration and metadata.
- Build settings: target platform, performance options.
- Keyboard shortcuts and customization: editor workflow automation.
- Project structure: organizing assets and scenes.
- Preferences: editor appearance and behavior.

## Concepts

### Scene Editing

Scene editor displays world hierarchy as a tree: root contains scenes, scenes contain entities,
entities contain components. Clicking entities in tree selects them. 3D viewport shows scene
spatially. Drag-and-drop hierarchically reparents entities. Prefabs (reusable entity templates) can
be dragged into scenes to instantiate. Editing a prefab updates all instances.

### Property Inspector

Property inspector displays selected entity's components and properties. Editing a property (name,
position, material) updates the entity. Different property types have different editors: color
properties show color picker; numeric properties show sliders or text input; enum properties show
dropdown. Custom property types register custom editors.

### Undo and Redo

Undo system records every change: create entity, set position, add component. Undo reverts the last
change; redo reapplies. Undo history persists across save/load (optional). Complex operations
group into single undo steps: moving multiple entities and changing properties appears as one undo.

### Gizmos and Selection

Gizmos are visual handles in 3D viewport: move gizmo (arrows for X, Y, Z translation), rotate gizmo
(arcs), scale gizmo (cubes). Clicking gizmo handles drags to edit: drag X arrow to move along X
axis. Selection mode (translate, rotate, scale) changes gizmo type.

### Asset Browser and Project Organization

Asset browser displays project files hierarchically. Drag asset into scene to create entity. Search
filters assets by name or type. Favorites pin frequently-used assets. Projects organize code,
assets, and scenes in consistent structure: Assets/, Code/, Scenes/, Docs/.

## How it fits

- See [visual-editors.md](./visual-editors.md) for specialized visual tools.
- See [profiling-and-collaboration.md](./profiling-and-collaboration.md) for debugging and
  performance tools.
- See [../core-runtime/authoring-runtime.md](../core-runtime/authoring-runtime.md) for
  plugin and extension systems.
