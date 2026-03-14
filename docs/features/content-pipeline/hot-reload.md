# 12.4 — Hot Reload

## File Watching

### F-12.4.1 File Watcher

A background daemon monitors source asset directories for file creation, modification, deletion, and
rename events using platform-native file system notification APIs. Detected changes are debounced,
deduplicated, and dispatched to the appropriate re-import or recompilation handlers. Essential for
interactive content authoring workflows with sub-second feedback.

- **Requirements:** R-12.4.1
- **Dependencies:** None
- **Platform notes:** Uses ReadDirectoryChangesW on Windows, FSEvents on macOS, inotify on Linux.

## Asset Hot Reload

### F-12.4.2 Asset Hot Reload

When a source asset changes, the pipeline re-imports and reprocesses only the affected asset and its
dependents, then patches the in-memory runtime representation without restarting the application.
Textures swap via descriptor heap updates; meshes and materials swap via atomic pointer replacement
behind a frame fence.

- **Requirements:** R-12.4.2
- **Dependencies:** F-12.4.1, F-12.2.8, F-12.3.3
- **Platform notes:** None

### F-12.4.3 Shader Hot Reload

Detect shader source or graph changes, recompile affected permutations, and swap pipeline state
objects at the next frame boundary. Compilation errors are displayed as an overlay in the viewport
while the previous valid shader remains active. Supports iterative shader authoring without
application restarts.

- **Requirements:** R-12.4.3
- **Dependencies:** F-12.4.1, F-12.2.7
- **Platform notes:** None

### F-12.4.4 Script Hot Reload

Detect script source changes, recompile the affected module, and hot-swap the function pointers or
WASM module in the scripting runtime without restarting gameplay. Entity state is serialized before
reload and deserialized after, preserving the live game state across code changes.

- **Requirements:** R-12.4.4
- **Dependencies:** F-12.4.1
- **Platform notes:** None

### F-12.4.5 UI Hot Reload

Detect changes to UI layout definitions, style sheets, and widget templates, then rebuild and
re-render the affected UI subtree in place. Preserves scroll positions, focus state, and animation
progress so that UI iteration does not reset the interface to its initial state.

- **Requirements:** R-12.4.5
- **Dependencies:** F-12.4.1
- **Platform notes:** None

## Partial Re-Import and Sync

### F-12.4.6 Partial Re-Import

When a source file changes, re-import only the modified sub-assets rather than the entire file. For
example, modifying a single animation clip in a multi-animation glTF file re-imports only that clip.
Partial re-import reduces hot reload latency for large composite assets common in MMO content.

- **Requirements:** R-12.4.6
- **Dependencies:** F-12.4.1, F-12.3.2
- **Platform notes:** None

### F-12.4.7 Editor-Runtime Synchronization

A bidirectional sync channel between the editor process and one or more connected runtime instances.
Asset changes made in the editor (material parameter tweaks, entity transforms, light adjustments)
are streamed to the runtime in real time, and runtime state (camera position, performance counters)
is streamed back to the editor for live preview.

- **Requirements:** R-12.4.7
- **Dependencies:** F-12.4.2
- **Platform notes:** None
