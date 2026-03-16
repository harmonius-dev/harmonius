# User Stories: VR Editor Mode

## F-15.22.1 VR Editor Mode

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.22.1.1 | designer (P-5) | to enter a full VR editing mode from the desktop editor without restarting | I can switch between desktop and VR editing fluidly during a session |  |  |
| US-15.22.1.2 | level designer (P-6) | to walk through my level in VR at true player scale with stereoscopic rendering and head tracking | I can evaluate spatial proportions, corridor widths, and sightlines as the player will experience them |  |  |
| US-15.22.1.3 | environment artist (P-8) | the VR viewport to render with the same lighting and materials as the desktop editor | I can evaluate visual quality immersively |  |  |
| US-15.22.1.4 | engine developer (P-26) | changes made in VR to be immediately visible on the desktop monitor and vice versa | VR and desktop editing coexist without sync issues |  |  |
| US-15.22.1.5 | designer (P-5) | support for both room-scale and seated VR configurations | I can use VR editing regardless of my physical workspace size |  |  |
| US-15.22.1.6 | engine developer (P-26) | the VR editor to maintain 90 fps minimum (or the headset's native refresh rate) | users do not experience motion sickness during editing sessions |  |  |
| US-15.22.1.7 | level designer (P-6) | VR editing to work on Meta Quest (via Link), Valve Index, and Apple Vision Pro via OpenXR | I can use whichever headset my studio provides |  |  |

## F-15.22.2 VR Spatial Editing

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.22.2.1 | designer (P-5) | to grab scene objects with motion controllers and move them with natural hand gestures | object placement feels intuitive and fast in VR |  |  |
| US-15.22.2.2 | level designer (P-6) | to grab an object with two controllers and pull apart or push together to scale it | I can resize objects using a natural two-hand gesture |  |  |
| US-15.22.2.3 | designer (P-5) | trigger-based precision placement with snap-to-grid using the same increments as desktop gizmos | VR-placed objects align to the same grid as desktop-placed objects |  |  |
| US-15.22.2.4 | environment artist (P-8) | controller haptic feedback on grab, release, and snap events | I get tactile confirmation of my editing actions |  |  |
| US-15.22.2.5 | designer (P-5) | VR transform operations to integrate with the undo/redo stack | I can revert VR edits with the same undo mechanism as desktop editing |  |  |
| US-15.22.2.6 | engine developer (P-26) | VR snap positions to match desktop snap positions numerically | objects placed in VR and desktop are interchangeable without drift |  |  |

## F-15.22.3 VR Hand Tracking

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.22.3.1 | designer (P-5) | to select and grab objects with pinch gestures using bare hands on supported headsets | I can make quick edits without picking up controllers |  |  |
| US-15.22.3.2 | designer (P-5) | an open palm gesture to activate the tool palette | I can switch tools with a natural hand gesture |  |  |
| US-15.22.3.3 | level designer (P-6) | pointing gestures to navigate menus and activate buttons | I can interact with VR UI elements without controllers |  |  |
| US-15.22.3.4 | environment artist (P-8) | hand tracking to coexist with controller input and switch seamlessly when I set down or pick up controllers | I can use whichever input method is convenient |  |  |
| US-15.22.3.5 | engine developer (P-26) | gesture recognition to achieve at least 95% accuracy for pinch, grab, open palm, and point gestures | hand tracking is reliable for editing tasks |  |  |

## F-15.22.4 VR Editor UI

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.22.4.1 | designer (P-5) | floating editor panels positioned in world space that I can pin, move, and dismiss with grab gestures | I can arrange my workspace in VR space around me |  |  |
| US-15.22.4.2 | level designer (P-6) | a radial menu on each controller providing quick access to tools (select, move, rotate, scale, paint, sculpt) | I can switch tools rapidly without navigating panel menus |  |  |
| US-15.22.4.3 | designer (P-5) | text input via a virtual keyboard or voice dictation in VR | I can rename entities, enter values, and search without removing my headset |  |  |
| US-15.22.4.4 | environment artist (P-8) | UI text to scale automatically based on panel distance from my head so it remains readable from 0.5 to 3.0 meters | I do not need to lean in to read panel content |  |  |
| US-15.22.4.5 | engine developer (P-26) | gaze-activated widgets on headsets with eye tracking | users can interact with UI elements by looking at them on supported hardware |  |  |

## F-15.22.5 VR Collaboration

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.22.5.1 | designer (P-5) | to see other users' head and hand avatars in VR space during collaborative sessions | I know where my teammates are and what they are doing |  |  |
| US-15.22.5.2 | level designer (P-6) | cursor trails visualizing each collaborator's recent edit activity | I can see what areas they have been working on |  |  |
| US-15.22.5.3 | environment artist (P-8) | voice chat audio spatialized to each collaborator's avatar position | I can tell who is speaking and where they are in the scene |  |  |
| US-15.22.5.4 | designer (P-5) | VR collaboration to integrate with the presence system (F-15.12.3) | I see the same collaborator list and status as the desktop editor |  |  |
| US-15.22.5.5 | engine developer (P-26) | avatar rendering to use simplified meshes (under 5,000 triangles each) with rendering overhead below 1 ms for 8 simultaneous avatars | collaboration does not cause VR frame drops |  |  |

## F-15.22.6 VR User/AI Tracking

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.22.6.1 | designer (P-5) | to select a collaborator from the presence list and enter a follow mode that mirrors their camera position with a configurable offset | I can observe their work without manual camera navigation |  |  |
| US-15.22.6.2 | level designer (P-6) | to follow an AI agent (F-15.12.12) in VR to observe its automated editing workflow | I can understand what the AI is doing and intervene if needed |  |  |
| US-15.22.6.3 | designer (P-5) | the followed user's selections, placements, and tool changes highlighted with visual indicators | I can see exactly what actions they are performing |  |  |
| US-15.22.6.4 | environment artist (P-8) | follow mode for mentoring new team members | a mentor can observe a junior artist's work and provide real-time guidance in VR |  |  |
| US-15.22.6.5 | engine developer (P-26) | follow mode to apply a 1-2 frame latency buffer to smooth camera motion and damp sudden movements | following another user does not cause VR discomfort |  |  |
| US-15.22.6.6 | designer (P-5) | to exit follow mode instantly by pressing any controller button | I can regain control of my viewpoint at any time without delay |  |  |
