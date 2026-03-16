# 15.22 -- VR Editor Mode

## Immersive Editing

### F-15.22.1 VR Editor Mode

A full editor experience rendered inside a VR headset with stereoscopic rendering, head tracking,
and 6-DoF motion controller input. The VR editor shares the same world state as the desktop editor
so changes are visible in both views simultaneously. Supports room-scale and seated configurations.
Enters and exits VR mode without restarting the editor. Built on the existing VR subsystem (F-6.5.1)
and OpenXR runtime.

- **Requirements:** R-15.22.1
- **Dependencies:** F-15.1.9, F-6.5.1, F-6.5.2
- **Platform notes:** Requires OpenXR-compatible headset. Desktop mirror view uses the existing
  editor rendering pipeline. Tested on Meta Quest (via Link), Valve Index, and Apple Vision Pro.

### F-15.22.2 VR Spatial Editing

Direct manipulation of scene objects using motion controllers. Grab, move, rotate, and scale objects
with natural hand gestures. Supports single-hand grab for translation and rotation, two-hand grab
for scaling, and trigger-based precision placement with snap-to-grid. Controller haptic feedback
confirms grab, release, and snap events. Transform operations integrate with the undo/redo stack
(F-15.1.3).

- **Requirements:** R-15.22.2
- **Dependencies:** F-15.22.1, F-15.1.5, F-15.1.3, F-6.5.2
- **Platform notes:** Haptic intensity scales with controller capability. Quest controllers, Index
  knuckles, and Vision Pro hand tracking all supported.

### F-15.22.3 VR Hand Tracking

Edit with bare hands on headsets that support optical hand tracking. Pinch gestures select and grab
objects. Open palm activates the tool palette. Pointing gestures navigate menus and activate
buttons. Hand tracking coexists with controller input and switches seamlessly when controllers are
set down or picked up. Gesture recognition uses the OpenXR hand tracking extension.

- **Requirements:** R-15.22.3
- **Dependencies:** F-15.22.1, F-6.5.1
- **Platform notes:** Requires `XR_EXT_hand_tracking` support. Available on Meta Quest 2+, Apple
  Vision Pro. Not available on Valve Index without Ultraleap accessory.

### F-15.22.4 VR Editor UI

A spatial UI system for VR with floating panels, radial menus, and gaze-activated widgets. Panels
are positioned in world space and can be pinned, moved, or dismissed with grab gestures. A radial
menu on each controller provides quick access to tools (select, move, rotate, scale, paint, sculpt).
Text input uses a virtual keyboard or voice dictation. UI elements scale with distance to maintain
readability.

- **Requirements:** R-15.22.4
- **Dependencies:** F-15.22.1, F-15.1.1, F-14.1.1
- **Platform notes:** UI rendering uses the same widget framework (F-14.1.1) with a VR-specific
  layout adapter. Gaze activation requires eye tracking on supported headsets.

### F-15.22.5 VR Collaboration

See other users' avatars and cursors in VR space during collaborative editing sessions. Each remote
user is represented by a head avatar and hand models showing their current position, orientation,
and active tool. Cursor trails visualize recent edit activity. Voice chat audio is spatialized to
each user's avatar position. Integrates with the collaboration presence system (F-15.12.3) and voice
chat (F-15.12.10).

- **Requirements:** R-15.22.5
- **Dependencies:** F-15.22.1, F-15.12.3, F-15.12.10
- **Platform notes:** Avatar rendering uses simplified meshes to maintain VR frame rate. Spatial
  audio uses the engine audio system (F-5.2.5).

### F-15.22.6 VR User/AI Tracking

Follow another user's or AI agent's viewpoint in VR. Select a collaborator from the presence list
and enter a follow mode that mirrors their camera position and orientation with a configurable
offset. The followed user's actions (selection, placement, tool changes) are highlighted with visual
indicators. AI agents (F-15.12.12) can be followed to observe automated editing workflows. Useful
for mentoring, review sessions, and understanding AI agent behavior.

- **Requirements:** R-15.22.6
- **Dependencies:** F-15.22.1, F-15.22.5, F-15.12.12
- **Platform notes:** Follow mode adds a small latency buffer (1-2 frames) to smooth camera motion
  and prevent VR discomfort from sudden movements.
