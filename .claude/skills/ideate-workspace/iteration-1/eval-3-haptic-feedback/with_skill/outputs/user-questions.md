# Questions for User Review

## Phase 2 Questions — Feature Confirmation

These questions would be asked after presenting the proposed features, before writing final feature
documents.

### Feature Selection

1. I proposed 7 new features (2 runtime, 5 editor). Which of these do you want to keep?
   - F-6.4.6 Haptic Channel Mixer
   - F-6.4.7 Haptic Effect Hot Reload
   - F-15.21.13 Haptic Waveform Editor
   - F-15.21.14 Haptic Live Preview
   - F-15.21.15 Haptic Effect Template Library
   - F-15.21.16 Per-Platform Haptic Testing
   - F-15.21.17 Haptic Debug Overlay

### Suggested Expansions

2. I suggested two features you may not have considered:
   - **Haptic Channel Mixer** (F-6.4.6) — concurrent effect blending so overlapping haptics compose
     instead of stomping. Is this useful for your vision?
   - **Haptic Debug Overlay** (F-15.21.17) — runtime visualization of active haptic state. Do you
     want this for development builds?

### Missing Features

3. Are there any capabilities missing from this list? Some possibilities I considered but did not
   include:
   - **Haptic A/B testing** — compare two haptic variants side by side with designer voting
   - **Haptic accessibility settings** — player-facing intensity scaling or disable toggle
   - **Haptic recording** — record and replay haptic output for regression testing
   - **Networked haptic preview** — preview on a remote device over the network (e.g., preview
     Switch haptics on a dev kit)

### Scope Clarification

4. For the template library (F-15.21.15), should templates be engine-provided only, or should they
   also be downloadable from the asset store? This affects whether we need an asset store
   integration feature.

5. For the waveform editor (F-15.21.13), should it support importing haptic data from external
   formats (e.g., Apple Core Haptics AHAP files, or third-party haptic authoring tools)? This would
   add an import/export feature.

## Phase 4 Questions — Requirements and Stories Review

These questions would be asked after presenting the generated requirements and user stories.

### Requirements Coverage

6. Are the performance targets realistic and measurable?
   - Mixer: 50 us for 8 effects on 4 channels
   - Hot reload: 200 ms detection-to-output
   - Preview latency: 30 ms play-to-output
   - Editor: 60 fps with 200 keyframes on 4 tracks
   - Simulation accuracy: 5% amplitude tolerance
   - Debug overlay: under 0.5 ms overhead

7. Are any requirements too vague or missing verification criteria? The simulation accuracy
   requirement (R-15.21.16a) at 5% tolerance may be difficult to verify without specialized haptic
   measurement hardware. Should we relax this to a visual comparison test instead?

### Persona Coverage

8. Are the right personas selected for each user story? I used these personas across the stories:
   - Designer (P-5) — primary authoring workflow
   - Engine developer (P-26) — ECS integration, API design
   - Player (P-23) — end-user tactile experience
   - Engine tester (P-27) — performance and correctness
   - QA tester (P-19) — functional testing
   - Audio designer (P-14) — audio-haptic sync
   - Prototyper (P-7) — rapid template usage

9. Should any features have stories from additional personas? For example:
   - **Creative director (P-2)** for overall haptic vision?
   - **Technical artist (P-13)** for haptic pipeline tools?
   - **Modder (P-24)** for mod-authored haptic effects?

### Non-Functional Requirements

10. Should any features have additional non-functional requirements? I included NFRs for:
    - Mixer performance (R-6.4.6a)
    - Editor rendering performance (R-15.21.13a)
    - Preview latency (R-15.21.14a)
    - Simulation accuracy (R-15.21.16a)

    Missing NFR candidates:
    - Memory budget for loaded haptic assets?
    - Maximum haptic asset file size?
    - Template library load time?
