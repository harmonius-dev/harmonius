# User Stories -- 12.4 Hot Reload

## US-12.4.1 See Texture Changes in Under 2 Seconds

**As a** designer (P-5), **I want** modified textures to re-import, reprocess, and swap in
the running engine via descriptor heap update in under 2 seconds, **so that** I can iterate
on textures with near-instant visual feedback.

## US-12.4.2 See Shader Changes Without Restarting

**As a** game developer (P-15), **I want** shader source or graph changes to trigger
recompilation of affected permutations and PSO swap at the next frame boundary, **so that**
I can iterate on shaders without application restarts.

## US-12.4.3 See Logic Graph Changes While Running

**As a** designer (P-5), **I want** logic graph hot reload that recompiles and patches
running graph instances with updated bytecode, preserving persistent state when the
variable layout is compatible, **so that** I can iterate on gameplay logic without
restarting the play session.

## US-12.4.4 See UI Changes Without Resetting the Interface

**As a** designer (P-5), **I want** UI layout, style sheet, and widget template changes to
rebuild and re-render the affected subtree in place, preserving scroll positions, focus
state, and animation progress, **so that** UI iteration does not reset my view.

## US-12.4.5 Detect File Changes Automatically for Hot Reload

**As an** engine developer (P-26), **I want** a file watcher daemon using platform-native
APIs (ReadDirectoryChangesW, FSEvents, inotify) with debounce and deduplication, **so
that** source asset changes are detected within milliseconds and dispatched to the correct
re-import handler.

## US-12.4.6 Re-Import Only Changed Sub-Assets

**As an** engine developer (P-26), **I want** partial re-import that processes only modified
sub-assets (e.g., a single animation clip from a multi-asset export), **so that** hot
reload latency is minimized for large composite assets.

## US-12.4.7 Sync Editor Changes to Connected Runtimes

**As a** game developer (P-15), **I want** a bidirectional sync channel between the editor
and connected runtime instances that streams asset changes, entity transforms, and material
tweaks to the runtime and streams camera position and performance counters back, **so that**
I can see live previews of editor changes on target devices.

## US-12.4.8 See Shader Errors as an Overlay Without Losing the Previous Shader

**As a** game developer (P-15), **I want** shader compilation errors displayed as a viewport
overlay while the previous valid shader remains active, **so that** a syntax error does not
leave the viewport blank while I fix the issue.

## US-12.4.9 Hot Reload Applies at Safe Sync Points Only

**As an** engine developer (P-26), **I want** hot reload operations to apply only at frame
sync points (not mid-frame), **so that** in-flight rendering and simulation are never
interrupted by a mid-frame asset swap.

## US-12.4.10 Verify Hot Reload Does Not Leak Resources

**As an** engine tester (P-27), **I want** automated tests that repeatedly hot reload
textures, meshes, shaders, and logic graphs and verify GPU and CPU memory usage does not
grow unboundedly, **so that** hot reload does not introduce memory leaks during extended
authoring sessions.

## US-12.4.11 Verify Logic Graph Hot Reload Preserves State

**As an** engine tester (P-27), **I want** tests that modify a running logic graph asset,
trigger hot reload, and verify persistent state (local variables, coroutine positions) is
preserved when the variable layout is compatible and reset when incompatible, **so that**
state preservation logic is correct.

## US-12.4.12 Benchmark Hot Reload Latency per Asset Type

**As an** engine tester (P-27), **I want** benchmarks that measure hot reload latency for
textures (< 2s), meshes (< 3s), logic graphs (< 500ms), and shaders (< 5s), **so that**
latency targets are validated and regressions are caught.

## US-12.4.13 Verify Editor-Runtime Sync Under Multiple Connected Devices

**As an** engine tester (P-27), **I want** tests that connect multiple runtime instances
to a single editor and verify all receive synchronized asset changes, **so that** multi-
device preview workflows are reliable.

## US-12.4.14 Test Hot Reload for All DCC Plugin Live Link Scenarios

**As an** engine tester (P-27), **I want** integration tests that push changes through each
DCC plugin's live link and verify the engine hot reloads the affected assets correctly,
**so that** the end-to-end live link workflow is validated per DCC tool.

## US-12.4.15 Hot Reload Mesh Assets with Atomic Pointer Swap

**As an** engine developer (P-26), **I want** mesh and material hot reload to use atomic
pointer replacement behind a frame fence, **so that** in-flight draw calls complete with
the old asset while subsequent frames use the new one without visual glitches.

## US-12.4.16 Verify UI Hot Reload Preserves Scroll and Focus State

**As an** engine tester (P-27), **I want** tests that hot reload UI layouts while scroll
positions, focus targets, and animations are active, verifying all are preserved after
reload, **so that** UI state preservation is validated.
