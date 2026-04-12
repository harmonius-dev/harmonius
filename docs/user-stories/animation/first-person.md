# User Stories -- 9.6 First-Person Animation

## Camera and Weapon

| ID          | Persona                    |
|-------------|----------------------------|
| US-9.6.1.1  | character animator (P-11)  |
| US-9.6.1.2  | engine developer (P-26)    |
| US-9.6.1.3  | game developer (P-15)      |
| US-9.6.2.1  | character animator (P-11)  |
| US-9.6.2.2  | technical artist (P-13)    |
| US-9.6.2.3  | game developer (P-15)      |
| US-9.6.3.1  | character animator (P-11)  |
| US-9.6.3.2  | engine developer (P-26)    |
| US-9.6.3.3  | game developer (P-15)      |
| US-9.6.4.1  | character animator (P-11)  |
| US-9.6.4.2  | character animator (P-11)  |
| US-9.6.4.3  | game developer (P-15)      |
| US-9.6.4.4  | engine developer (P-26)    |

1. **US-9.6.1.1** -- **As a** character animator (P-11), **I want** a first-person camera with
   head-bob, landing impact, procedural lean, and slope tilt driven by spring-damper physics,
   **so that** first-person movement feels physical and immersive.

2. **US-9.6.1.2** -- **As an** engine developer (P-26), **I want** a separate viewmodel FOV that
   prevents weapon distortion at wide world FOVs, **so that** first-person weapons render correctly
   regardless of world FOV setting.

3. **US-9.6.2.1** -- **As a** character animator (P-11), **I want** weapon viewmodel sway,
   locomotion bob, and sprint tilt driven by per-weapon spring data, **so that** each weapon feels
   distinct during movement.

4. **US-9.6.2.2** -- **As a** technical artist (P-13), **I want** all sway and bob parameters
   defined as per-weapon data assets, **so that** tuning does not require code changes.

5. **US-9.6.3.1** -- **As a** character animator (P-11), **I want** non-repetitive recoil from
   pattern data with spring-based recovery and smooth ADS transitions, **so that** weapon firing
   feels unique and aim-down-sights is fluid.

6. **US-9.6.3.2** -- **As an** engine developer (P-26), **I want** scope rendering using
   render-to-texture for magnified optics, **so that** scoped weapons show real magnified views of
   the scene.

7. **US-9.6.4.1** -- **As a** character animator (P-11), **I want** equip, holster, and weapon
   inspection animations, **so that** weapon handling has visible physical presence.

8. **US-9.6.1.3** -- **As a** game developer (P-15), **I want** head-bob frequency synchronized to
   locomotion speed and landing impact proportional to fall distance, **so that** camera motion
   matches gameplay state without manual tuning.

9. **US-9.6.2.3** -- **As a** game developer (P-15), **I want** sprint tilt to activate at sprint
   speed and deactivate on stop, **so that** weapon carry position responds to movement state
   automatically.

10. **US-9.6.3.3** -- **As a** game developer (P-15), **I want** ADS to interpolate smoothly between
hip and sight positions with reduced sway and bob, **so that** aiming feels precise and responsive.

11. **US-9.6.4.1** -- **As a** character animator (P-11), **I want** equip, holster, and weapon

    inspection animations, **so that** weapon handling has visible physical presence.

12. **US-9.6.4.2** -- **As a** character animator (P-11), **I want** dual wielding with independent
sway, bob, fire, and reload per hand, **so that** two-weapon combat feels responsive and distinct.

13. **US-9.6.4.3** -- **As a** game developer (P-15), **I want** alternating, simultaneous, and
    independent fire modes for dual wielding, **so that** different weapon combinations have
    distinct gameplay feel.

14. **US-9.6.4.4** -- **As an** engine developer (P-26), **I want** independent per-hand spring
    systems for dual wield, **so that** each hand's sway, bob, and recoil are computed separately
    without interference.
