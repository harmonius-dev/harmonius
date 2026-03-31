# User Stories — 6.2 Input Actions & Mapping

| ID          | Persona                 |
|-------------|-------------------------|
| US-6.2.1.1  | game designer (P-5)     |
| US-6.2.1.2  | player (P-23)           |
| US-6.2.1.3  | engine developer (P-26) |
| US-6.2.2.1  | game designer (P-5)     |
| US-6.2.2.2  | player (P-23)           |
| US-6.2.2.3  | engine developer (P-26) |
| US-6.2.3.1  | game designer (P-5)     |
| US-6.2.3.2  | player (P-23)           |
| US-6.2.3.3  | engine developer (P-26) |
| US-6.2.4.1  | game designer (P-5)     |
| US-6.2.4.2  | player (P-23)           |
| US-6.2.4.3  | game designer (P-5)     |
| US-6.2.5.1  | player (P-23)           |
| US-6.2.5.2  | player (P-23)           |
| US-6.2.5.3  | game designer (P-5)     |
| US-6.2.6.1  | player (P-23)           |
| US-6.2.6.2  | game designer (P-5)     |
| US-6.2.6.3  | engine developer (P-26) |
| US-6.2.7.1  | game developer (P-15)   |
| US-6.2.7.2  | game designer (P-5)     |
| US-6.2.8.1  | game designer (P-5)     |
| US-6.2.8.2  | player (P-23)           |
| US-6.2.8.3  | engine developer (P-26) |
| US-6.2.9.1  | player (P-23)           |
| US-6.2.9.2  | game designer (P-5)     |
| US-6.2.10.1 | player (P-23)           |
| US-6.2.10.2 | game designer (P-5)     |
| US-6.2.10.3 | engine developer (P-26) |
| US-6.2.11.1 | player (P-23)           |
| US-6.2.11.2 | player (P-23)           |
| US-6.2.11.3 | engine developer (P-26) |

1. **US-6.2.1.1** — **As a** game designer (P-5), **I want** to define boolean, axis 1D, 2D, and 3D
   actions in the editor, **so that** gameplay logic is decoupled from devices.
2. **US-6.2.1.2** — **As a** player (P-23), **I want** the same game actions to work on keyboard,
   gamepad, and touch, **so that** I can use any device seamlessly.
3. **US-6.2.1.3** — **As an** engine developer (P-26), **I want** type enforcement that rejects
   mismatched source-to-action bindings at load time, **so that** silent value truncation bugs are
   caught early.
4. **US-6.2.2.1** — **As a** game designer (P-5), **I want** to create named mapping contexts
   (OnFoot, Mounted, UIMenu) in the visual editor, **so that** input bindings are grouped by game
   mode.
5. **US-6.2.2.2** — **As a** player (P-23), **I want** controls to automatically adapt when entering
   menus, vehicles, or combat, **so that** the right bindings are always active.
6. **US-6.2.2.3** — **As an** engine developer (P-26), **I want** contexts on a priority-ordered
   stack where higher-priority contexts consume inputs first, **so that** modal overlays work
   correctly.
7. **US-6.2.3.1** — **As a** game designer (P-5), **I want** dead zone, response curve, and
   sensitivity modifiers configurable per binding, **so that** each action has tuned stick feel.
8. **US-6.2.3.2** — **As a** player (P-23), **I want** per-player sensitivity options, **so that** I
   can customize stick feel to preference.
9. **US-6.2.3.3** — **As an** engine developer (P-26), **I want** composable modifier chains applied
   in a defined order, **so that** input processing is predictable.
10. **US-6.2.4.1** — **As a** game designer (P-5), **I want** trigger conditions (pressed, hold,
    tap, chord, combo) settable per action, **so that** each ability has appropriate activation.
11. **US-6.2.4.2** — **As a** player (P-23), **I want** holding a button to channel abilities and
    tapping to cast instantly, **so that** input matches the action type.
12. **US-6.2.4.3** — **As a** game designer (P-5), **I want** longer tap/hold thresholds on mobile,
    **so that** finger imprecision is accommodated.
13. **US-6.2.5.1** — **As a** player (P-23), **I want** to rebind any action to any compatible input
    at runtime, **so that** my controls are fully customizable.
14. **US-6.2.5.2** — **As a** player (P-23), **I want** rebindings to persist across sessions,
    **so that** my customizations are never lost.
15. **US-6.2.5.3** — **As a** game designer (P-5), **I want** platform-specific default bindings
    shipped per controller type, **so that** out-of-box experience is good.
16. **US-6.2.6.1** — **As a** player (P-23), **I want** button prompts to show my controller's icons
    (Xbox A, PlayStation Cross), **so that** UI guidance matches my device.
17. **US-6.2.6.2** — **As a** game designer (P-5), **I want** glyph atlases as swappable assets,
    **so that** custom icon packs are possible.
18. **US-6.2.6.3** — **As an** engine developer (P-26), **I want** glyphs to update within one frame
    of a device change, **so that** prompts are always current.
19. **US-6.2.7.1** — **As a** game developer (P-15), **I want** to record input events to a binary
    stream and replay them deterministically, **so that** I can create automated regression tests.
20. **US-6.2.7.2** — **As a** game designer (P-5), **I want** ghost playback of recorded inputs for
    tutorials, **so that** players see example movements.
21. **US-6.2.8.1** — **As a** game designer (P-5), **I want** to author combo trees as visual graph
    assets with timing windows, **so that** complex combos are designed visually.
22. **US-6.2.8.2** — **As a** player (P-23), **I want** to execute directional combo chains
    (quarter-circle, dragon punch), **so that** skilled input produces powerful attacks.
23. **US-6.2.8.3** — **As an** engine developer (P-26), **I want** stick, D-pad, and WASD normalized
    to the same cardinal/diagonal inputs, **so that** combos work on any device.
24. **US-6.2.9.1** — **As a** player (P-23), **I want** to queue my next attack during recovery
    frames, **so that** combat chains feel responsive.
25. **US-6.2.9.2** — **As a** game designer (P-5), **I want** cancel windows defined per ability as
    frame ranges with priority rules, **so that** action cancelling is precisely tunable.
26. **US-6.2.10.1** — **As a** player (P-23), **I want** aim assist helping me target accurately on
    gamepad, **so that** controller aiming is competitive.
27. **US-6.2.10.2** — **As a** game designer (P-5), **I want** aim assist strength configurable per
    weapon type and disableable per game mode, **so that** balance is tunable.
28. **US-6.2.10.3** — **As an** engine developer (P-26), **I want** a low-pass filter that reduces
    stick jitter by 80% without adding over 16 ms latency, **so that** worn controllers remain
    usable.
29. **US-6.2.11.1** — **As a** player (P-23), **I want** to navigate all UI with D-pad and stick
    without a mouse, **so that** I can use menus on console.
30. **US-6.2.11.2** — **As a** player (P-23), **I want** stick angle to select items in radial
    menus, **so that** quick-select wheels work on gamepad.
31. **US-6.2.11.3** — **As an** engine developer (P-26), **I want** seamless switching between
    gamepad and mouse mid-interaction, **so that** mixed input works without focus loss.
