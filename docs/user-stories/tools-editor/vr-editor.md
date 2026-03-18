# User Stories: VR Editor Mode

## F-15.22.1 VR Editor Mode

| ID           | Persona                  | Features | Requirements |
|--------------|--------------------------|----------|--------------|
| US-15.22.1.1 | designer (P-5)           |          |              |
| US-15.22.1.2 | level designer (P-6)     |          |              |
| US-15.22.1.3 | environment artist (P-8) |          |              |
| US-15.22.1.4 | engine developer (P-26)  |          |              |
| US-15.22.1.5 | designer (P-5)           |          |              |
| US-15.22.1.6 | engine developer (P-26)  |          |              |
| US-15.22.1.7 | level designer (P-6)     |          |              |

1. **US-15.22.1.1** — to enter a full VR editing mode from the desktop editor without restarting
   - **Acceptance:** I can switch between desktop and VR editing fluidly during a session
2. **US-15.22.1.2** — to walk through my level in VR at true player scale with stereoscopic
   rendering and head tracking
   - **Acceptance:** I can evaluate spatial proportions, corridor widths, and sightlines as the
     player will experience them
3. **US-15.22.1.3** — the VR viewport to render with the same lighting and materials as the desktop
   editor
   - **Acceptance:** I can evaluate visual quality immersively
4. **US-15.22.1.4** — changes made in VR to be immediately visible on the desktop monitor and vice
   versa
   - **Acceptance:** VR and desktop editing coexist without sync issues
5. **US-15.22.1.5** — support for both room-scale and seated VR configurations
   - **Acceptance:** I can use VR editing regardless of my physical workspace size
6. **US-15.22.1.6** — the VR editor to maintain 90 fps minimum (or the headset's native refresh
   rate)
   - **Acceptance:** users do not experience motion sickness during editing sessions
7. **US-15.22.1.7** — VR editing to work on Meta Quest (via Link), Valve Index, and Apple Vision Pro
   via OpenXR
   - **Acceptance:** I can use whichever headset my studio provides

## F-15.22.2 VR Spatial Editing

| ID           | Persona                  | Features | Requirements |
|--------------|--------------------------|----------|--------------|
| US-15.22.2.1 | designer (P-5)           |          |              |
| US-15.22.2.2 | level designer (P-6)     |          |              |
| US-15.22.2.3 | designer (P-5)           |          |              |
| US-15.22.2.4 | environment artist (P-8) |          |              |
| US-15.22.2.5 | designer (P-5)           |          |              |
| US-15.22.2.6 | engine developer (P-26)  |          |              |

1. **US-15.22.2.1** — to grab scene objects with motion controllers and move them with natural hand
   gestures
   - **Acceptance:** object placement feels intuitive and fast in VR
2. **US-15.22.2.2** — to grab an object with two controllers and pull apart or push together to
   scale it
   - **Acceptance:** I can resize objects using a natural two-hand gesture
3. **US-15.22.2.3** — trigger-based precision placement with snap-to-grid using the same increments
   as desktop gizmos
   - **Acceptance:** VR-placed objects align to the same grid as desktop-placed objects
4. **US-15.22.2.4** — controller haptic feedback on grab, release, and snap events
   - **Acceptance:** I get tactile confirmation of my editing actions
5. **US-15.22.2.5** — VR transform operations to integrate with the undo/redo stack
   - **Acceptance:** I can revert VR edits with the same undo mechanism as desktop editing
6. **US-15.22.2.6** — VR snap positions to match desktop snap positions numerically
   - **Acceptance:** objects placed in VR and desktop are interchangeable without drift

## F-15.22.3 VR Hand Tracking

| ID           | Persona                  | Features | Requirements |
|--------------|--------------------------|----------|--------------|
| US-15.22.3.1 | designer (P-5)           |          |              |
| US-15.22.3.2 | designer (P-5)           |          |              |
| US-15.22.3.3 | level designer (P-6)     |          |              |
| US-15.22.3.4 | environment artist (P-8) |          |              |
| US-15.22.3.5 | engine developer (P-26)  |          |              |

1. **US-15.22.3.1** — to select and grab objects with pinch gestures using bare hands on supported
   headsets
   - **Acceptance:** I can make quick edits without picking up controllers
2. **US-15.22.3.2** — an open palm gesture to activate the tool palette
   - **Acceptance:** I can switch tools with a natural hand gesture
3. **US-15.22.3.3** — pointing gestures to navigate menus and activate buttons
   - **Acceptance:** I can interact with VR UI elements without controllers
4. **US-15.22.3.4** — hand tracking to coexist with controller input and switch seamlessly when I
   set down or pick up controllers
   - **Acceptance:** I can use whichever input method is convenient
5. **US-15.22.3.5** — gesture recognition to achieve at least 95% accuracy for pinch, grab, open
   palm, and point gestures
   - **Acceptance:** hand tracking is reliable for editing tasks

## F-15.22.4 VR Editor UI

| ID           | Persona                  | Features | Requirements |
|--------------|--------------------------|----------|--------------|
| US-15.22.4.1 | designer (P-5)           |          |              |
| US-15.22.4.2 | level designer (P-6)     |          |              |
| US-15.22.4.3 | designer (P-5)           |          |              |
| US-15.22.4.4 | environment artist (P-8) |          |              |
| US-15.22.4.5 | engine developer (P-26)  |          |              |

1. **US-15.22.4.1** — floating editor panels positioned in world space that I can pin, move, and
   dismiss with grab gestures
   - **Acceptance:** I can arrange my workspace in VR space around me
2. **US-15.22.4.2** — a radial menu on each controller providing quick access to tools (select,
   move, rotate, scale, paint, sculpt)
   - **Acceptance:** I can switch tools rapidly without navigating panel menus
3. **US-15.22.4.3** — text input via a virtual keyboard or voice dictation in VR
   - **Acceptance:** I can rename entities, enter values, and search without removing my headset
4. **US-15.22.4.4** — UI text to scale automatically based on panel distance from my head so it
   remains readable from 0.5 to 3.0 meters
   - **Acceptance:** I do not need to lean in to read panel content
5. **US-15.22.4.5** — gaze-activated widgets on headsets with eye tracking
   - **Acceptance:** users can interact with UI elements by looking at them on supported hardware

## F-15.22.5 VR Collaboration

| ID           | Persona                  | Features | Requirements |
|--------------|--------------------------|----------|--------------|
| US-15.22.5.1 | designer (P-5)           |          |              |
| US-15.22.5.2 | level designer (P-6)     |          |              |
| US-15.22.5.3 | environment artist (P-8) |          |              |
| US-15.22.5.4 | designer (P-5)           |          |              |
| US-15.22.5.5 | engine developer (P-26)  |          |              |

1. **US-15.22.5.1** — to see other users' head and hand avatars in VR space during collaborative
   sessions
   - **Acceptance:** I know where my teammates are and what they are doing
2. **US-15.22.5.2** — cursor trails visualizing each collaborator's recent edit activity
   - **Acceptance:** I can see what areas they have been working on
3. **US-15.22.5.3** — voice chat audio spatialized to each collaborator's avatar position
   - **Acceptance:** I can tell who is speaking and where they are in the scene
4. **US-15.22.5.4** — VR collaboration to integrate with the presence system (F-15.12.3)
   - **Acceptance:** I see the same collaborator list and status as the desktop editor
5. **US-15.22.5.5** — avatar rendering to use simplified meshes (under 5,000 triangles each) with
   rendering overhead below 1 ms for 8 simultaneous avatars
   - **Acceptance:** collaboration does not cause VR frame drops

## F-15.22.6 VR User/AI Tracking

| ID           | Persona                  | Features | Requirements |
|--------------|--------------------------|----------|--------------|
| US-15.22.6.1 | designer (P-5)           |          |              |
| US-15.22.6.2 | level designer (P-6)     |          |              |
| US-15.22.6.3 | designer (P-5)           |          |              |
| US-15.22.6.4 | environment artist (P-8) |          |              |
| US-15.22.6.5 | engine developer (P-26)  |          |              |
| US-15.22.6.6 | designer (P-5)           |          |              |

1. **US-15.22.6.1** — to select a collaborator from the presence list and enter a follow mode that
   mirrors their camera position with a configurable offset
   - **Acceptance:** I can observe their work without manual camera navigation
2. **US-15.22.6.2** — to follow an AI agent (F-15.12.12) in VR to observe its automated editing
   workflow
   - **Acceptance:** I can understand what the AI is doing and intervene if needed
3. **US-15.22.6.3** — the followed user's selections, placements, and tool changes highlighted with
   visual indicators
   - **Acceptance:** I can see exactly what actions they are performing
4. **US-15.22.6.4** — follow mode for mentoring new team members
   - **Acceptance:** a mentor can observe a junior artist's work and provide real-time guidance in
     VR
5. **US-15.22.6.5** — follow mode to apply a 1-2 frame latency buffer to smooth camera motion and
   damp sudden movements
   - **Acceptance:** following another user does not cause VR discomfort
6. **US-15.22.6.6** — to exit follow mode instantly by pressing any controller button
   - **Acceptance:** I can regain control of my viewpoint at any time without delay
