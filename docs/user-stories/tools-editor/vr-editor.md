# User Stories: VR Editor Mode

## F-15.22.1 VR Editor Mode

## US-15.22.1.1 Designer Enters VR Mode

**As a** designer (P-5), **I want** to enter a full VR editing mode from the desktop editor without
restarting, **so that** I can switch between desktop and VR editing fluidly during a session.

## US-15.22.1.2 Level Designer Experiences Player Scale

**As a** level designer (P-6), **I want** to walk through my level in VR at true player scale with
stereoscopic rendering and head tracking, **so that** I can evaluate spatial proportions, corridor
widths, and sightlines as the player will experience them.

## US-15.22.1.3 Environment Artist Previews Lighting

**As an** environment artist (P-8), **I want** the VR viewport to render with the same lighting and
materials as the desktop editor, **so that** I can evaluate visual quality immersively.

## US-15.22.1.4 Engine Dev Syncs VR and Desktop

**As an** engine developer (P-26), **I want** changes made in VR to be immediately visible on the
desktop monitor and vice versa, **so that** VR and desktop editing coexist without sync issues.

## US-15.22.1.5 Designer Uses Room-Scale or Seated

**As a** designer (P-5), **I want** support for both room-scale and seated VR configurations,
**so that** I can use VR editing regardless of my physical workspace size.

## US-15.22.1.6 Engine Dev Maintains VR Frame Rate

**As an** engine developer (P-26), **I want** the VR editor to maintain 90 fps minimum (or the
headset's native refresh rate), **so that** users do not experience motion sickness during editing
sessions.

## US-15.22.1.7 Level Designer Tests on Multiple Headsets

**As a** level designer (P-6), **I want** VR editing to work on Meta Quest (via Link), Valve Index,
and Apple Vision Pro via OpenXR, **so that** I can use whichever headset my studio provides.

## F-15.22.2 VR Spatial Editing

## US-15.22.2.1 Designer Grabs and Moves Objects

**As a** designer (P-5), **I want** to grab scene objects with motion controllers and move them with
natural hand gestures, **so that** object placement feels intuitive and fast in VR.

## US-15.22.2.2 Level Designer Scales with Two Hands

**As a** level designer (P-6), **I want** to grab an object with two controllers and pull apart or
push together to scale it, **so that** I can resize objects using a natural two-hand gesture.

## US-15.22.2.3 Designer Snaps to Grid in VR

**As a** designer (P-5), **I want** trigger-based precision placement with snap-to-grid using the
same increments as desktop gizmos, **so that** VR-placed objects align to the same grid as
desktop-placed objects.

## US-15.22.2.4 Environment Artist Feels Haptic Feedback

**As an** environment artist (P-8), **I want** controller haptic feedback on grab, release, and snap
events, **so that** I get tactile confirmation of my editing actions.

## US-15.22.2.5 Designer Undoes VR Transforms

**As a** designer (P-5), **I want** VR transform operations to integrate with the undo/redo stack,
**so that** I can revert VR edits with the same undo mechanism as desktop editing.

## US-15.22.2.6 Engine Dev Tests Cross-Mode Consistency

**As an** engine developer (P-26), **I want** VR snap positions to match desktop snap positions
numerically, **so that** objects placed in VR and desktop are interchangeable without drift.

## F-15.22.3 VR Hand Tracking

## US-15.22.3.1 Designer Edits with Bare Hands

**As a** designer (P-5), **I want** to select and grab objects with pinch gestures using bare hands
on supported headsets, **so that** I can make quick edits without picking up controllers.

## US-15.22.3.2 Designer Opens Tool Palette by Palm

**As a** designer (P-5), **I want** an open palm gesture to activate the tool palette, **so that** I
can switch tools with a natural hand gesture.

## US-15.22.3.3 Level Designer Points to Navigate Menus

**As a** level designer (P-6), **I want** pointing gestures to navigate menus and activate buttons,
**so that** I can interact with VR UI elements without controllers.

## US-15.22.3.4 Environment Artist Switches Seamlessly

**As an** environment artist (P-8), **I want** hand tracking to coexist with controller input and
switch seamlessly when I set down or pick up controllers, **so that** I can use whichever input
method is convenient.

## US-15.22.3.5 Engine Dev Ensures Gesture Accuracy

**As an** engine developer (P-26), **I want** gesture recognition to achieve at least 95% accuracy
for pinch, grab, open palm, and point gestures, **so that** hand tracking is reliable for editing
tasks.

## F-15.22.4 VR Editor UI

## US-15.22.4.1 Designer Uses Floating Panels

**As a** designer (P-5), **I want** floating editor panels positioned in world space that I can pin,
move, and dismiss with grab gestures, **so that** I can arrange my workspace in VR space around me.

## US-15.22.4.2 Level Designer Uses Radial Menu

**As a** level designer (P-6), **I want** a radial menu on each controller providing quick access to
tools (select, move, rotate, scale, paint, sculpt), **so that** I can switch tools rapidly without
navigating panel menus.

## US-15.22.4.3 Designer Types with Virtual Keyboard

**As a** designer (P-5), **I want** text input via a virtual keyboard or voice dictation in VR,
**so that** I can rename entities, enter values, and search without removing my headset.

## US-15.22.4.4 Environment Artist Reads Panels at Distance

**As an** environment artist (P-8), **I want** UI text to scale automatically based on panel
distance from my head so it remains readable from 0.5 to 3.0 meters, **so that** I do not need to
lean in to read panel content.

## US-15.22.4.5 Engine Dev Activates Gaze Widgets

**As an** engine developer (P-26), **I want** gaze-activated widgets on headsets with eye tracking,
**so that** users can interact with UI elements by looking at them on supported hardware.

## F-15.22.5 VR Collaboration

## US-15.22.5.1 Designer Sees Collaborator Avatars

**As a** designer (P-5), **I want** to see other users' head and hand avatars in VR space during
collaborative sessions, **so that** I know where my teammates are and what they are doing.

## US-15.22.5.2 Level Designer Sees Edit Trails

**As a** level designer (P-6), **I want** cursor trails visualizing each collaborator's recent edit
activity, **so that** I can see what areas they have been working on.

## US-15.22.5.3 Environment Artist Hears Spatial Audio

**As an** environment artist (P-8), **I want** voice chat audio spatialized to each collaborator's
avatar position, **so that** I can tell who is speaking and where they are in the scene.

## US-15.22.5.4 Designer Joins VR from Presence List

**As a** designer (P-5), **I want** VR collaboration to integrate with the presence system
(F-15.12.3), **so that** I see the same collaborator list and status as the desktop editor.

## US-15.22.5.5 Engine Dev Maintains Frame Rate with Avatars

**As an** engine developer (P-26), **I want** avatar rendering to use simplified meshes (under 5,000
triangles each) with rendering overhead below 1 ms for 8 simultaneous avatars, **so that**
collaboration does not cause VR frame drops.

## F-15.22.6 VR User/AI Tracking

## US-15.22.6.1 Designer Follows Collaborator

**As a** designer (P-5), **I want** to select a collaborator from the presence list and enter a
follow mode that mirrors their camera position with a configurable offset, **so that** I can observe
their work without manual camera navigation.

## US-15.22.6.2 Level Designer Follows AI Agent

**As a** level designer (P-6), **I want** to follow an AI agent (F-15.12.12) in VR to observe its
automated editing workflow, **so that** I can understand what the AI is doing and intervene if
needed.

## US-15.22.6.3 Designer Sees Followed User Actions

**As a** designer (P-5), **I want** the followed user's selections, placements, and tool changes
highlighted with visual indicators, **so that** I can see exactly what actions they are performing.

## US-15.22.6.4 Environment Artist Mentors in VR

**As an** environment artist (P-8), **I want** follow mode for mentoring new team members,
**so that** a mentor can observe a junior artist's work and provide real-time guidance in VR.

## US-15.22.6.5 Engine Dev Smooths Follow Camera

**As an** engine developer (P-26), **I want** follow mode to apply a 1-2 frame latency buffer to
smooth camera motion and damp sudden movements, **so that** following another user does not cause VR
discomfort.

## US-15.22.6.6 Designer Exits Follow Mode Instantly

**As a** designer (P-5), **I want** to exit follow mode instantly by pressing any controller button,
**so that** I can regain control of my viewpoint at any time without delay.
