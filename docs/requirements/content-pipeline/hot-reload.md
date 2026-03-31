# R-12.4 Hot Reload

## File Watching

1. **R-12.4.1** — The engine **SHALL** monitor source asset directories for file creation,
   modification, deletion, and rename events using platform-native file system notification APIs,
   debouncing and deduplicating detected changes before dispatching them to re-import or
   recompilation handlers. File change detection delegates to the platform filesystem change
   notification system (R-14.6.5).
   - **Rationale:** Platform-native file watching provides sub-second change detection with minimal
     CPU overhead, enabling interactive authoring workflows without polling.
   - **Verification:** Create, modify, rename, and delete files in a watched directory; verify the
     watcher dispatches exactly one event per logical change after debouncing; verify latency from
     file write to event dispatch is under 500 ms on all platforms.

## Asset Hot Reload

2. **R-12.4.2** — The engine **SHALL** re-import and reprocess only the affected asset and its
   dependents when a source asset changes, then patch the in-memory runtime representation without
   restarting the application, using descriptor heap updates for textures and atomic pointer
   replacement behind a frame fence for meshes and materials.
   - **Rationale:** In-place asset swapping preserves live game state during iteration, eliminating
     restart latency for content creators.
   - **Verification:** Load a scene with a textured mesh; modify the source texture; verify the
     runtime texture updates within 2 seconds without restart; verify no rendering artifacts during
     the swap; verify dependent materials also update.
3. **R-12.4.3** — The engine **SHALL** detect shader source or graph changes, recompile affected
   permutations, and swap pipeline state objects at the next frame boundary, displaying compilation
   errors as a viewport overlay while the previous valid shader remains active.
   - **Rationale:** Iterative shader authoring requires immediate visual feedback without restarts,
     and graceful error handling prevents broken rendering during edits.
   - **Verification:** Modify a shader with a valid change and verify the PSO updates within one
     frame boundary; introduce a syntax error and verify the error overlay appears while the
     previous shader continues rendering.
4. **R-12.4.4** — The engine **SHALL** hot-reload logic graph assets within 500 ms of file change
   detection, preserving execution state when the variable layout is unchanged. Incompatible
   variable layout changes **SHALL** trigger a clean restart of the affected graph instance.
   - **Rationale:** Preserving graph execution state eliminates the need to reproduce gameplay
     scenarios after every logic change.
   - **Verification:** Spawn entities with logic graph behavior; modify the graph and trigger hot
     reload; verify state is preserved within 500 ms; modify the variable layout and verify the
     graph instance restarts cleanly.
5. **R-12.4.5** — The engine **SHALL** detect changes to UI layout definitions, style sheets, and
   widget templates, then rebuild and re-render the affected UI subtree in place, preserving scroll
   positions, focus state, and animation progress.
   - **Rationale:** Preserving UI interaction state during hot reload prevents disruptive resets
     that interrupt the designer's workflow.
   - **Verification:** Render a UI with a scrolled list and an active animation; modify the style
     sheet; verify scroll position, focused element, and animation progress are preserved; verify
     only the affected subtree is rebuilt.

## Partial Re-Import and Sync

6. **R-12.4.6** — The engine **SHALL** re-import only modified sub-assets within a composite source
   file rather than the entire file, using content hashing to identify changed sub-assets and skip
   unchanged ones.
   - **Rationale:** Partial re-import reduces hot reload latency for large composite assets from
     minutes to seconds.
   - **Verification:** Import a composite asset with 10 animation clips; modify one clip; verify
     only the modified clip is reprocessed; measure re-import time and verify it is under 20% of a
     full re-import.
7. **R-12.4.7** — The engine **SHALL** maintain a bidirectional sync channel between the editor
   process and connected runtime instances, streaming asset changes from editor to runtime and
   runtime state (camera position, performance counters) from runtime to editor in real time.
   - **Rationale:** Bidirectional sync enables live preview of editor changes in the running game
     and live telemetry feedback without manual refresh.
   - **Verification:** Connect an editor to a runtime; adjust a material parameter in the editor and
     verify the runtime reflects it within 100 ms; move the camera in the runtime and verify the
     editor displays the updated position within 100 ms; verify graceful reconnection after
     transient disconnection.
