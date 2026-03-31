# R-6.2 — Input Actions & Mapping Requirements

## Action Definitions

1. **R-6.2.1** — The engine **SHALL** define input actions as strongly typed values (boolean, axis
   1D, axis 2D, axis 3D) with type enforcement at binding load time that rejects mismatched
   source-to-action bindings.
   - **Rationale:** Type safety prevents silent value truncation that causes hard-to-debug gameplay
     bugs.
   - **Verification:** Bind a boolean action to keyboard, gamepad, and touch. Assert all three
     produce the same value. Bind axis 2D to boolean and assert load-time diagnostic error.

2. **R-6.2.2** — The engine **SHALL** produce identical `ActionState` values for the same semantic
   input regardless of source device, so gameplay systems never branch on device type.
   - **Rationale:** Device-agnostic gameplay logic is required for the no-code engine constraint.
   - **Verification:** Bind a "Move" axis 2D to WASD, gamepad stick, and touch joystick. Push all to
     max forward. Assert equivalent values within 1%.

## Input Mapping Contexts

3. **R-6.2.3** — The engine **SHALL** maintain mapping contexts on a priority-ordered stack where
   higher-priority contexts are evaluated first. When a context's `consumes_input` flag is true,
   matched inputs **SHALL NOT** pass to lower-priority contexts.
   - **Rationale:** Modal overlays must capture inputs before combat or movement contexts.
   - **Verification:** Push Combat (priority 10) and UIMenu (priority 20) both binding Escape. Press
     Escape. Assert only UIMenu fires.

4. **R-6.2.4** — The engine **SHALL** pass unbound inputs through to lower-priority contexts,
   allowing movement while a UI overlay captures menu-specific keys.
   - **Rationale:** Players expect to move while an inventory overlay is open.
   - **Verification:** Push UIMenu (Escape only) and OnFoot (WASD). Press W. Assert movement fires.
     Press Escape. Assert UIMenu fires.

5. **R-6.2.5** — The engine **SHALL** support authoring mapping contexts and their bindings entirely
   in the visual editor with no code required.
   - **Rationale:** No-code engine constraint requires all input configuration via visual tools.
   - **Verification:** Create a context in the editor, add 5 bindings, save, reload, and verify all
     bindings are functional at runtime.

## Modifiers

6. **R-6.2.6** — The engine **SHALL** apply composable modifier chains in defined order: dead zone,
   response curve, swizzle, negate, scalar, smoothing, acceleration. Each modifier **SHALL** be
   configurable per binding.
   - **Rationale:** Different controllers and contexts require different input processing.
   - **Verification:** Chain radial dead zone (0.15), exponential curve, and scalar (2.0). Feed 0.10
     and assert zero. Feed 0.50 and verify within 0.1%.

7. **R-6.2.7** — The engine **SHALL** support both axial and radial dead zone modes. Values below
   threshold map to zero; values above are remapped to [0.0, 1.0].
   - **Rationale:** Axial dead zones prevent per-axis drift; radial dead zones prevent
     magnitude-based drift.
   - **Verification:** Apply radial dead zone 0.15 to magnitude 0.10. Assert output is (0, 0). Apply
     to 0.50. Assert output is 0.412.

8. **R-6.2.8** — The engine **SHALL** support linear, exponential, and S-curve response curves as
   modifier stages.
   - **Rationale:** Exponential gives fine control at low input; S-curves give fine control at both
     extremes.
   - **Verification:** Apply exponential (exponent 2.0) to input 0.5. Assert output is 0.25.

## Triggers

9. **R-6.2.9** — The engine **SHALL** support trigger conditions: pressed, released, hold, tap,
   pulse, chord, and combo.
   - **Rationale:** Diverse activation patterns are essential for combat systems.
   - **Verification:** Test each trigger type: pressed fires on down frame only; hold fires after
     duration; chord fires when all inputs active; combo fires on correct ordered sequence within
     time window.

10. **R-6.2.10** — The engine **SHALL** allow designers to set trigger conditions per action binding
    in the visual editor with preview of trigger timing.
    - **Rationale:** No-code constraint; designers must tune trigger timing without code.
    - **Verification:** Set hold trigger with 500 ms in editor. Play and hold for 600 ms. Assert
      fires.

## Rebinding

11. **R-6.2.11** — The engine **SHALL** allow runtime rebinding of any action to any compatible
    input with conflict detection within overlapping contexts, presenting swap, unbind, or cancel
    options.
    - **Rationale:** Rebinding is a core accessibility requirement; conflict detection prevents
      unusable configurations.
    - **Verification:** Rebind two actions to the same key. Assert conflict detected with resolution
      options. Select swap and assert bindings are swapped.

12. **R-6.2.12** — The engine **SHALL** persist rebinding changes to storage within 100 ms and
    restore all rebindings within 50 ms during startup.
    - **Rationale:** Players expect rebindings to survive crashes; fast restore prevents startup
      delay.
    - **Verification:** Rebind 20 actions. Assert writes complete within 100 ms. Restart. Assert
      restored within 50 ms.

13. **R-6.2.13** — The engine **SHALL** reject rebinding attempts to platform-reserved keys with a
    diagnostic message.
    - **Rationale:** Platform certification requires reserved keys to remain functional.
    - **Verification:** Attempt to rebind to PS button. Assert rebinding is rejected with message.

## Button Glyphs

14. **R-6.2.14** — The engine **SHALL** resolve input action bindings to platform-specific button
    icons and update all displayed glyphs within one frame of an active device change.
    - **Rationale:** Console certification requires correct platform-native button icons at all
      times.
    - **Verification:** Switch from keyboard to Xbox gamepad. Assert glyph for "Jump" updates from
      "Space" to "A" within one frame.

15. **R-6.2.15** — The engine **SHALL** support swappable glyph atlas assets for custom controller
    icon packs.
    - **Rationale:** Games need branded or stylized button icons.
    - **Verification:** Load custom glyph atlas. Assert glyphs resolve to icons from the custom
      atlas.

## Recording and Playback

16. **R-6.2.16** — The engine **SHALL** record all input events to a binary stream and play them
    back deterministically, producing identical game state on replay. Playback **SHALL** support
    0.5x to 4.0x speed and frame stepping.
    - **Rationale:** Deterministic replay enables testing, bug reproduction, and tutorial ghost
      playback.
    - **Verification:** Record 30 seconds of input. Play back and compare game state hash. Assert
      identical. Verify speed control and frame stepping.

## Combos

17. **R-6.2.17** — The engine **SHALL** support combo input trees as visual graph assets with
    directional motion sequences, branching states, configurable leniency windows, and input
    buffering.
    - **Rationale:** Fighting-game-grade combos require structured input recognition; visual
      authoring serves the no-code constraint.
    - **Verification:** Author a quarter-circle-forward combo. Input within window. Assert fires.
      Input with timeout. Assert resets to root.

18. **R-6.2.18** — The engine **SHALL** normalize directional inputs from sticks, D-pad, and WASD to
    identical cardinal and diagonal directions.
    - **Rationale:** Combos must feel identical regardless of input method.
    - **Verification:** Input quarter-circle-forward via stick, D-pad, and WASD. Assert all three
      trigger the same combo.

## Input Buffering

19. **R-6.2.19** — The engine **SHALL** buffer the most recent input during active ability frames
    with configurable duration (100-200 ms) and execute it when the current action enters a cancel
    window or completes.
    - **Rationale:** Input buffering enables responsive action combat without frame-perfect timing.
    - **Verification:** Trigger a 500 ms ability with cancel at 300-500 ms. Press dodge at 250 ms.
      Assert dodge executes at 300 ms.

20. **R-6.2.20** — The engine **SHALL** resolve buffered input conflicts using priority ordering
    (dodge > attack > movement).
    - **Rationale:** The highest-priority action should execute when multiple inputs arrive during
      recovery.
    - **Verification:** Buffer dodge and attack in the same window. Assert dodge executes.

## Advanced Filtering

21. **R-6.2.21** — The engine **SHALL** provide a low-pass filter modifier that reduces analog stick
    jitter by at least 80% without adding more than 16 ms of latency.
    - **Rationale:** Worn controllers produce jitter that degrades camera control.
    - **Verification:** Feed jittery input with 50 ms smoothing. Assert variance reduced by 80%.
      Assert latency under 16 ms.

22. **R-6.2.22** — The engine **SHALL** support acceleration curves with configurable ramp-up time,
    max multiplier, and decay rate.
    - **Rationale:** Acceleration enables slow precision and fast turns.
    - **Verification:** Set acceleration at 2x max. Assert output at max velocity equals 2x base
      sensitivity.

23. **R-6.2.23** — The engine **SHALL** support aim assist with magnetism, friction, and snap, each
    configurable per weapon type and game mode. Aim assist **SHALL** be disableable per game mode.
    - **Rationale:** Gamepad players need aim assist for competitive parity; it must be disableable
      for PvP.
    - **Verification:** Enable magnetism. Assert crosshair deflects toward target. Disable. Assert
      no deflection.

## Gamepad UI Navigation

24. **R-6.2.24** — The engine **SHALL** support full UI navigability via gamepad using directional
    focus, virtual cursor, and action mapping modes. All hover-dependent behaviors **SHALL**
    activate on focus when gamepad is active.
    - **Rationale:** Console certification requires controller navigation; no-code engine requires
      it without scripting.
    - **Verification:** Navigate all UI screens with gamepad. Assert every widget is reachable.
      Verify tooltips appear on focus.

25. **R-6.2.25** — The engine **SHALL** support switching between gamepad and mouse mid-interaction
    without losing UI focus state.
    - **Rationale:** Players frequently switch devices; the transition must be invisible.
    - **Verification:** Navigate to a UI element via gamepad. Switch to mouse. Assert focus is
      retained.
