# R-12.4 -- Hot Reload Requirements

## File Watching

| ID       | Derived From                                              |
|----------|-----------------------------------------------------------|
| R-12.4.1 | [F-12.4.1](../../features/content-pipeline/hot-reload.md) |

1. **R-12.4.1** — The engine **SHALL** monitor source asset directories for file creation,
   modification, deletion, and rename events using platform-native file system notification APIs
   (ReadDirectoryChangesW on Windows, FSEvents on macOS, inotify on Linux), debouncing and
   deduplicating detected changes before dispatching them to re-import or recompilation handlers.
   File change detection delegates to the platform filesystem change notification system (R-14.6.5).
   This requirement specifies the asset-pipeline-specific debouncing, batching, and re-import
   triggering logic on top of the platform notifications.
   - **Rationale:** Platform-native file watching provides sub-second change detection with minimal
     CPU overhead, enabling interactive authoring workflows without polling.
   - **Verification:** Integration test per platform: create, modify, rename, and delete files in a
     watched directory. Verify the watcher dispatches exactly one event per logical change after
     debouncing. Verify no spurious duplicate events are delivered for rapid successive writes.
     Verify latency from file write to event dispatch is under 500 ms on all platforms.

## Asset Hot Reload

| ID       | Derived From                                              |
|----------|-----------------------------------------------------------|
| R-12.4.2 | [F-12.4.2](../../features/content-pipeline/hot-reload.md) |
| R-12.4.3 | [F-12.4.3](../../features/content-pipeline/hot-reload.md) |
| R-12.4.4 | [F-12.4.4](../../features/content-pipeline/hot-reload.md) |
| R-12.4.5 | [F-12.4.5](../../features/content-pipeline/hot-reload.md) |

1. **R-12.4.2** — The engine **SHALL** re-import and reprocess only the affected asset and its
   dependents when a source asset changes, then patch the in-memory runtime representation without
   restarting the application, using descriptor heap updates for textures and atomic pointer
   replacement behind a frame fence for meshes and materials.
   - **Rationale:** In-place asset swapping preserves live game state during iteration, eliminating
     application restart latency for content creators.
   - **Verification:** Integration test: load a scene with a textured mesh, modify the source
     texture file, and verify the runtime texture updates within 2 seconds without application
     restart. Verify no rendering artifacts occur during the swap by capturing a frame before and
     after the fence. Verify dependent assets (materials referencing the texture) also update.
2. **R-12.4.3** — The engine **SHALL** detect shader source or graph changes, recompile affected
   permutations, and swap pipeline state objects at the next frame boundary, displaying compilation
   errors as a viewport overlay while the previous valid shader remains active.
   - **Rationale:** Iterative shader authoring requires immediate visual feedback without
     application restarts, and graceful error handling prevents broken rendering during edits.
   - **Verification:** Integration test: modify a shader source file with a valid change and verify
     the pipeline state object updates within one frame boundary. Introduce a syntax error and
     verify the error overlay appears while the previous valid shader continues rendering. Verify
     only affected permutations are recompiled, not the entire shader library.
3. **R-12.4.4** — The engine **SHALL** hot-reload logic graph assets within 500 ms of file change
   detection, preserving execution state when the variable layout is unchanged. The runtime
   recompiles the affected graph through the shared build cache and patches running instances with
   updated bytecode. Incompatible variable layout changes **SHALL** trigger a clean restart of the
   affected graph instance.
   - **Rationale:** Preserving graph execution state across hot reloads eliminates the need to
     manually reproduce gameplay scenarios after every logic change.
   - **Verification:** Integration test: spawn entities with logic graph behavior and mutable state.
     Modify the logic graph asset, trigger hot reload, and verify entity state (local variables,
     coroutine positions) is preserved within 500 ms. Modify the variable layout and verify the
     graph instance restarts cleanly. Verify no memory leaks from the replaced bytecode.
4. **R-12.4.5** — The engine **SHALL** detect changes to UI layout definitions, style sheets, and
   widget templates, then rebuild and re-render the affected UI subtree in place, preserving scroll
   positions, focus state, and animation progress.
   - **Rationale:** Preserving UI interaction state during hot reload prevents disruptive resets
     that interrupt the designer's workflow.
   - **Verification:** Integration test: render a UI with a scrolled list and an active animation.
     Modify the style sheet, trigger reload, and verify the scroll position, focused element, and
     animation progress are preserved. Verify only the affected subtree is rebuilt, not the entire
     UI tree.

## Partial Re-Import and Sync

| ID       | Derived From                                              |
|----------|-----------------------------------------------------------|
| R-12.4.6 | [F-12.4.6](../../features/content-pipeline/hot-reload.md) |
| R-12.4.7 | [F-12.4.7](../../features/content-pipeline/hot-reload.md) |

1. **R-12.4.6** — The engine **SHALL** re-import only modified sub-assets within a composite source
   file rather than the entire file, using content hashing to identify changed sub-assets and skip
   unchanged ones.
   - **Rationale:** Partial re-import reduces hot reload latency for large composite assets such as
     multi-animation DCC exports from minutes to seconds.
   - **Verification:** Integration test: import a composite asset containing 10 animation clips.
     Modify one clip and trigger re-import. Verify only the modified clip is reprocessed by checking
     import timestamps or content hashes of the other 9 clips remain unchanged. Measure re-import
     time and verify it is under 20% of a full re-import.
2. **R-12.4.7** — The engine **SHALL** maintain a bidirectional sync channel between the editor
   process and connected runtime instances, streaming asset changes (material parameters, entity
   transforms, light adjustments) from editor to runtime and runtime state (camera position,
   performance counters) from runtime to editor in real time.
   - **Rationale:** Bidirectional sync enables live preview of editor changes in the running game
     and live telemetry feedback in the editor without manual refresh.
   - **Verification:** Integration test: connect an editor instance to a running runtime. Adjust a
     material parameter in the editor and verify the runtime reflects the change within 100 ms. Move
     the camera in the runtime and verify the editor displays the updated position within 100 ms.
     Verify the channel reconnects gracefully after a transient disconnection.
