# User Stories — 6.1 Device Abstraction

| ID         | Persona                  |
|------------|--------------------------|
| US-6.1.1.1 | game designer (P-5)      |
| US-6.1.1.2 | player (P-23)            |
| US-6.1.1.3 | engine developer (P-26)  |
| US-6.1.2.1 | player (P-23)            |
| US-6.1.2.2 | game designer (P-5)      |
| US-6.1.2.3 | engine developer (P-26)  |
| US-6.1.3.1 | player (P-23)            |
| US-6.1.3.2 | game designer (P-5)      |
| US-6.1.3.3 | engine developer (P-26)  |
| US-6.1.3.4 | game developer (P-15)    |
| US-6.1.4.1 | player (P-23)            |
| US-6.1.4.2 | game designer (P-5)      |
| US-6.1.4.3 | engine developer (P-26)  |
| US-6.1.5.1 | player (P-23)            |
| US-6.1.5.2 | player (P-23)            |
| US-6.1.5.3 | game designer (P-5)      |
| US-6.1.5.4 | engine developer (P-26)  |

1. **US-6.1.1.1** — **As a** game designer (P-5), **I want** to configure keyboard bindings in the
   editor using physical key positions, **so that** WASD movement works on any layout.
2. **US-6.1.1.2** — **As a** player (P-23), **I want** my chat input to produce correct characters
   for my language, **so that** I can communicate using my native keyboard.
3. **US-6.1.1.3** — **As an** engine developer (P-26), **I want** platform scancodes normalized to a
   common USB HID enumeration, **so that** input bindings are portable across OSes.
4. **US-6.1.2.1** — **As a** player (P-23), **I want** precise mouse aiming with sub-pixel deltas,
   **so that** camera and targeting feel smooth.
5. **US-6.1.2.2** — **As a** game designer (P-5), **I want** mouse sensitivity configurable in
   settings, **so that** players can tune aim speed to preference.
6. **US-6.1.2.3** — **As an** engine developer (P-26), **I want** trackpad continuous scroll and
   mouse wheel notch scroll normalized to a unified axis, **so that** gameplay code handles both
   identically.
7. **US-6.1.3.1** — **As a** player (P-23), **I want** any controller to work immediately when
   plugged in, **so that** I do not need to configure drivers.
8. **US-6.1.3.2** — **As a** game designer (P-5), **I want** to query controller capabilities (gyro,
   touchpad, adaptive triggers) at runtime, **so that** I can gate features by hardware.
9. **US-6.1.3.3** — **As an** engine developer (P-26), **I want** a unified gamepad abstraction over
   XInput, DualSense, and Switch Pro, **so that** gameplay systems are device-agnostic.
10. **US-6.1.3.4** — **As a** game developer (P-15), **I want** gyro-aim derived from Madgwick
    sensor fusion, **so that** motion-based targeting is stable.
11. **US-6.1.4.1** — **As a** player (P-23), **I want** virtual joystick and gesture camera controls
    on mobile, **so that** touch-based gameplay feels responsive.
12. **US-6.1.4.2** — **As a** game designer (P-5), **I want** touch and pen sensitivity configurable
    per device type, **so that** input is tuned for different screens.
13. **US-6.1.4.3** — **As an** engine developer (P-26), **I want** touch pressure normalized to
    0.0-1.0 across all backends, **so that** pressure-based features are consistent.
14. **US-6.1.5.1** — **As a** player (P-23), **I want** to switch between keyboard/mouse and gamepad
    without restarting, **so that** input transitions are seamless.
15. **US-6.1.5.2** — **As a** player (P-23), **I want** the game to pause or show fallback behavior
    when my controller disconnects, **so that** I am not penalized mid-session.
16. **US-6.1.5.3** — **As a** game designer (P-5), **I want** pause-on-disconnect behavior
    configurable per project, **so that** each game chooses its response.
17. **US-6.1.5.4** — **As an** engine developer (P-26), **I want** device enumeration on a
    background thread, **so that** hot-plug detection never blocks the game loop.

## Parent Stories

The 3-segment parent stories below are umbrella rollups for the refined 4-segment sub-stories listed
above. Each parent inherits the persona of its first sub-story and describes the umbrella capability
that the sub-stories refine.

| ID | Persona |
|----|---------|
| US-6.1.1 | game designer (P-5) |
| US-6.1.2 | player (P-23) |
| US-6.1.3 | player (P-23) |
| US-6.1.4 | player (P-23) |
| US-6.1.5 | player (P-23) |

1. **US-6.1.1** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-6.1.1.1 through US-6.1.1.3 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

2. **US-6.1.2** -- **As a** player (P-23), **I want** the capabilities defined in sub-stories
   US-6.1.2.1 through US-6.1.2.3 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

3. **US-6.1.3** -- **As a** player (P-23), **I want** the capabilities defined in sub-stories
   US-6.1.3.1 through US-6.1.3.4 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

4. **US-6.1.4** -- **As a** player (P-23), **I want** the capabilities defined in sub-stories
   US-6.1.4.1 through US-6.1.4.3 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

5. **US-6.1.5** -- **As a** player (P-23), **I want** the capabilities defined in sub-stories
   US-6.1.5.1 through US-6.1.5.4 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.
