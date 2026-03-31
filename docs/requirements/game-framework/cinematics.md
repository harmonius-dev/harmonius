# R-13.5 — Cinematics Requirements

## Cinematics Editor

1. **R-13.5.1** — The engine **SHALL** provide a multi-track timeline system for authoring
   cinematics with tracks for camera, animation, audio, VFX, lighting, gameplay triggers, and UI
   overlays, supporting nested sub-sequences and deterministic evaluation regardless of framerate.
   - **Rationale:** Deterministic multi-track evaluation ensures cinematics play identically across
     hardware and framerate variations.
   - **Verification:** Author a cinematic with 6 tracks. Play at 30 fps and 120 fps and verify
     identical output at matching timestamps. Nest a sub-sequence and verify it plays inline.

2. **R-13.5.2** — The engine **SHALL** provide cinematic camera modes (fixed, tracking, orbit, dolly
   zoom, handheld shake) with depth-of-field overrides, focal target tracking, and configurable
   blend transitions.
   - **Rationale:** Varied camera modes enable directors to create engaging shots without code.
   - **Verification:** Create shots using each mode and verify correct behavior. Verify DoF override
     changes focus per shot. Verify blend transitions interpolate between modes.

3. **R-13.5.3** — The engine **SHALL** support spline camera paths (Catmull-Rom, Bezier) with
   configurable speed, acceleration, look-at targets, and branching paths selectable by gameplay
   conditions.
   - **Rationale:** Spline paths enable precise camera movement authored in the world.
   - **Verification:** Place a spline path and verify camera follows at configured speed. Set a
     gameplay condition and verify the correct branch is selected.

## Actor Integration

4. **R-13.5.4** — The engine **SHALL** blend actors between gameplay and cinematic poses with
   per-actor blend-in/blend-out durations, partial body overrides, and background NPC ambient
   behavior continuation.
   - **Rationale:** Seamless blending prevents visible pops at cutscene boundaries.
   - **Verification:** Start a cutscene and verify actors blend smoothly from gameplay pose. Verify
     partial body override allows lower-body locomotion during upper-body cinematic gesture.

5. **R-13.5.5** — The engine **SHALL** fire dialogue events at timeline cue points triggering
   voice-over, localized subtitles with speaker identification, and lip-sync animation, with
   branching support via the dialogue system.
   - **Rationale:** Timeline-driven dialogue ensures audio, text, and animation synchronize
     precisely.
   - **Verification:** Place a dialogue cue and verify voice, subtitle, and lip-sync trigger at the
     correct time. Verify localized text displays correctly.

## Player Control

6. **R-13.5.6** — The engine **SHALL** support cutscene skip (applying all side effects),
   fast-forward (2x, 4x), and pause with configurable multiplayer policies (consensus,
   majority-vote, host-only).
   - **Rationale:** Player control over cutscene playback respects player time without
     desynchronizing game state.
   - **Verification:** Skip a cutscene and verify all gameplay side effects (quest updates, item
     grants) are applied. Fast-forward at 4x and verify triggers fire at correct positions. Pause
     and verify frozen state.

7. **R-13.5.7** — The engine **SHALL** render configurable letterbox bars (2.39:1, 2.00:1, 1.85:1)
   with animated reveal/hide, coordinating HUD suppression and input suppression during cinematic
   mode.
   - **Rationale:** Letterboxing signals cinematic mode and suppresses irrelevant UI.
   - **Verification:** Trigger letterboxing and verify bars animate in at the configured aspect
     ratio. Verify HUD elements hide and gameplay inputs are suppressed.
