# User Stories — 6.4 Haptics & Feedback

| ID         | Persona                 |
|------------|-------------------------|
| US-6.4.1.1 | player (P-23)           |
| US-6.4.1.2 | game designer (P-5)     |
| US-6.4.1.3 | engine developer (P-26) |
| US-6.4.2.1 | player (P-23)           |
| US-6.4.2.2 | game designer (P-5)     |
| US-6.4.2.3 | engine developer (P-26) |
| US-6.4.3.1 | player (P-23)           |
| US-6.4.3.2 | game designer (P-5)     |
| US-6.4.3.3 | engine developer (P-26) |
| US-6.4.4.1 | player (P-23)           |
| US-6.4.4.2 | game designer (P-5)     |
| US-6.4.4.3 | engine developer (P-26) |
| US-6.4.5.1 | game designer (P-5)     |
| US-6.4.5.2 | player (P-23)           |
| US-6.4.5.3 | engine developer (P-26) |

1. **US-6.4.1.1** — **As a** player (P-23), **I want** distinct rumble for combat hits, ability
   impacts, and mount locomotion, **so that** actions have tactile presence.
2. **US-6.4.1.2** — **As a** game designer (P-5), **I want** to author rumble patterns as keyframe
   sequences with attack, sustain, and decay, **so that** haptic assets are designed visually.
3. **US-6.4.1.3** — **As an** engine developer (P-26), **I want** motor intensity normalized to
   0.0-1.0 across all controller backends, **so that** patterns feel consistent on every device.
4. **US-6.4.2.1** — **As a** player (P-23), **I want** trigger resistance when drawing a bow,
   **so that** archery feels physical through the controller.
5. **US-6.4.2.2** — **As a** game designer (P-5), **I want** to configure adaptive trigger effect
   modes and parameters per trigger in the editor, **so that** trigger feel is tunable per activity.
6. **US-6.4.2.3** — **As an** engine developer (P-26), **I want** adaptive trigger effects to
   degrade to no-op on controllers without support, **so that** non-DualSense users are unaffected.
7. **US-6.4.3.1** — **As a** player (P-23), **I want** footsteps to feel different on stone vs grass
   vs sand through HD haptics, **so that** surface type is tactile.
8. **US-6.4.3.2** — **As a** game designer (P-5), **I want** to author HD haptic waveforms in the
   visual editor, **so that** fine haptic assets are tuned visually.
9. **US-6.4.3.3** — **As an** engine developer (P-26), **I want** a common waveform format with
   platform-specific conversion at the backend, **so that** HD haptics are cross-platform.
10. **US-6.4.4.1** — **As a** player (P-23), **I want** explosions to produce matching tactile
    feedback automatically, **so that** every blast is felt through the controller.
11. **US-6.4.4.2** — **As a** game designer (P-5), **I want** frequency band and amplitude
    parameters configurable for audio-driven haptics, **so that** haptic generation is tunable per
    effect.
12. **US-6.4.4.3** — **As an** engine developer (P-26), **I want** audio-to-haptic latency under 10
    ms, **so that** desynchronization is imperceptible.
13. **US-6.4.5.1** — **As a** game designer (P-5), **I want** named profiles combining rumble,
    adaptive triggers, and HD haptics into reusable assets, **so that** haptic design is unified and
    composable.
14. **US-6.4.5.2** — **As a** player (P-23), **I want** some haptic feedback on any controller,
    **so that** switching devices does not lose all feedback.
15. **US-6.4.5.3** — **As an** engine developer (P-26), **I want** fallback chain ordering validated
    at build time, **so that** every profile produces feedback on every controller class.
