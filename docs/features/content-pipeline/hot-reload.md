# 12.4 — Hot Reload

## File Watching

| ID       | Feature      |
|----------|--------------|
| F-12.4.1 | File Watcher |

1. **F-12.4.1** — A background daemon monitors source asset directories for file creation,
   modification, deletion, and rename events using platform-native file system notification APIs.
   Detected changes are debounced, deduplicated, and dispatched to the appropriate re-import or
   recompilation handlers. Essential for interactive content authoring workflows with sub-second
   feedback.
   - **Platform:** Uses ReadDirectoryChangesW on Windows, FSEvents on macOS, inotify on Linux.

## Asset Hot Reload

| ID       | Feature                |
|----------|------------------------|
| F-12.4.2 | Asset Hot Reload       |
| F-12.4.3 | Shader Hot Reload      |
| F-12.4.4 | Logic Graph Hot Reload |
| F-12.4.5 | UI Hot Reload          |

1. **F-12.4.2** — When a source asset changes, the pipeline re-imports and reprocesses only the
   affected asset and its dependents, then patches the in-memory runtime representation without
   restarting the application. Textures swap via descriptor pool/set updates; meshes and materials
   swap via atomic pointer replacement behind a frame fence.
   - **Deps:** F-12.4.1, F-12.2.8, F-12.3.3
2. **F-12.4.3** — Detect shader source or graph changes, recompile affected permutations, and swap
   pipeline state objects at the next frame boundary. Compilation errors are displayed as an overlay
   in the viewport while the previous valid shader remains active. Supports iterative shader
   authoring without application restarts.
   - **Deps:** F-12.4.1, F-12.2.7
3. **F-12.4.4** — Hot reload of logic graph assets. When a logic graph asset is modified in the
   editor, the runtime recompiles the affected graph through the shared build cache (F-15.11.3) and
   patches running graph instances with updated bytecode. Persistent state (local variables,
   coroutine positions) is preserved when the variable layout is compatible. Incompatible changes
   trigger a clean restart of the affected graph instance.
   - **Deps:** F-12.4.1, F-15.8.1 (Logic Graph Runtime), F-15.8.12 (Graph Compilation)
4. **F-12.4.5** — Detect changes to UI layout definitions, style sheets, and widget templates, then
   rebuild and re-render the affected UI subtree in place. Preserves scroll positions, focus state,
   and animation progress so that UI iteration does not reset the interface to its initial state.
   - **Deps:** F-12.4.1

## Partial Re-Import and Sync

| ID       | Feature                        |
|----------|--------------------------------|
| F-12.4.6 | Partial Re-Import              |
| F-12.4.7 | Editor-Runtime Synchronization |

1. **F-12.4.6** — When a source file changes, re-import only the modified sub-assets rather than the
   entire file. For example, modifying a single animation clip in a multi-asset DCC export
   re-imports only that clip. Partial re-import reduces hot reload latency for large composite
   assets.
   - **Deps:** F-12.4.1, F-12.3.2
2. **F-12.4.7** — A bidirectional sync channel between the editor process and one or more connected
   runtime instances. Asset changes made in the editor (material parameter tweaks, entity
   transforms, light adjustments) are streamed to the runtime in real time, and runtime state
   (camera position, performance counters) is streamed back to the editor for live preview.
   - **Deps:** F-12.4.2
