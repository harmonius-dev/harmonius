# 9.6 — First-Person Animation

| ID      | Feature                               | Requirements |
|---------|---------------------------------------|--------------|
| F-9.6.1 | First-Person Camera Controller        | R-9.6.1      |
| F-9.6.2 | Procedural Weapon Sway and Bob        | R-9.6.2      |
| F-9.6.3 | Procedural Recoil and ADS Animation   | R-9.6.3      |
| F-9.6.4 | Weapon Equip, Inspect, and Dual Wield | R-9.6.4      |

1. **F-9.6.1** — Specialized camera system for first-person games with head-bob synchronized to
   locomotion speed, landing impact proportional to fall distance, procedural lean/peek, and camera
   tilt on strafing and slopes. Spring-damper driven effects with configurable stiffness and
   damping. Separate viewmodel FOV prevents weapon distortion at wide world FOVs. Integrates with
   the character controller (F-4.1.8).
   - **Deps:** F-4.1.8 (Character Controller), F-13.2.1 (Camera System)
   - **Platform:** Camera effects run on all platforms. Spring-damper complexity uniform. Separate
     viewmodel FOV supported on all tiers.
2. **F-9.6.2** — Weapon viewmodel moves in response to player input and locomotion via spring
   physics. Input sway creates a lagging follow effect. Locomotion bob oscillates synchronized to
   gait. Sprint tilt rotates weapon to carry position. All parameters are per-weapon data.
   - **Deps:** F-9.6.1, F-6.2.1 (Input Actions)
   - **Platform:** Lightweight spring systems on all platforms. No platform-specific scaling
     required.
3. **F-9.6.3** — Non-repetitive recoil from pattern data (F-13.16.3) via spring forces with
   recovery. ADS smoothly interpolates between hip and sight alignment positions. Multiple sight
   positions per weapon. Reduced sway and bob during ADS. Scope rendering uses render-to-texture for
   magnified optics.
   - **Deps:** F-9.6.2, F-13.16.3 (Recoil Patterns)
   - **Platform:** Lightweight spring systems on all platforms. Scope render-to-texture: mobile
     half-res, desktop full-res.
4. **F-9.6.4** — Equip/holster sequences with draw animations. Weapon inspection triggered by input.
   Dual wielding of two one-handed weapons with independent sway, bob, fire, and reload per hand.
   Fire modes: alternating, simultaneous, and independent.
   - **Deps:** F-9.6.2, F-9.4.1 (Animation State Machine)
   - **Platform:** Dual wield doubles draw call cost. Mobile may use simplified viewmodel LOD for
     off-hand weapon.
