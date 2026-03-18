# User Stories -- 9.6 First-Person Animation

## F-9.6.1

| ID         | Persona              | Features | Requirements |
|------------|----------------------|----------|--------------|
| US-9.6.1.1 | player (P-23)        | F-9.6.1  | R-9.6.1      |
| US-9.6.1.2 | game designer (P-5)  | F-9.6.1  | R-9.6.1      |
| US-9.6.1.3 | engine tester (P-27) | F-9.6.1  | R-9.6.1      |

1. **US-9.6.1.1** — I want head-bob synchronized to locomotion speed and gait, landing impact on
   ground contact, and procedural lean/peek when peeking around corners
   - **Acceptance:** first-person movement feels physically grounded and immersive
2. **US-9.6.1.2** — I want configurable stiffness, damping, and tilt parameters for first-person
   camera effects
   - **Acceptance:** I can tune the feel from heavy military (low bob, slow lean) to parkour (high
     bob, fast tilt) without code changes
3. **US-9.6.1.3** — I want set a world FOV of 110 with a viewmodel FOV of 70 and verify that the
   weapon renders without distortion on all platforms
   - **Acceptance:** wide FOV does not stretch the weapon viewmodel

## F-9.6.2

| ID         | Persona              | Features | Requirements |
|------------|----------------------|----------|--------------|
| US-9.6.2.1 | player (P-23)        | F-9.6.2  | R-9.6.2      |
| US-9.6.2.2 | game designer (P-5)  | F-9.6.2  | R-9.6.2      |
| US-9.6.2.3 | engine tester (P-27) | F-9.6.2  | R-9.6.2      |

1. **US-9.6.2.1** — I want weapon sway and bob driven by per-weapon spring physics where a heavy LMG
   sways more than a light pistol
   - **Acceptance:** each weapon feels physically distinct during movement and aiming
2. **US-9.6.2.2** — I want sway stiffness, damping, mass, and bob amplitude exposed as per-weapon
   data assets that I edit in the editor
   - **Acceptance:** weapon feel is tunable per weapon type without programmer support
3. **US-9.6.2.3** — I want trigger sprint and verify that the weapon rotates to a carry position
   only when movement speed exceeds the sprint threshold
   - **Acceptance:** sprint tilt transitions correctly based on movement state

## F-9.6.3

| ID         | Persona              | Features | Requirements |
|------------|----------------------|----------|--------------|
| US-9.6.3.1 | player (P-23)        | F-9.6.3  | R-9.6.3      |
| US-9.6.3.2 | player (P-23)        | F-9.6.3  | R-9.6.3      |
| US-9.6.3.3 | engine tester (P-27) | F-9.6.3  | R-9.6.3      |

1. **US-9.6.3.1** — I want procedural recoil that applies rotational and translational kick from
   recoil pattern data with spring-based recovery
   - **Acceptance:** sustained fire produces varied, non-repetitive weapon kick
2. **US-9.6.3.2** — I want ADS to interpolate the weapon from hip to sight alignment with reduced
   sway and minimized bob
   - **Acceptance:** aiming feels precise and stable
3. **US-9.6.3.3** — I want toggle ADS with a magnified optic and verify that scope render-to-texture
   uses half-res on mobile and full-res on desktop
   - **Acceptance:** scope rendering stays within per-platform GPU budgets

## F-9.6.4

| ID         | Persona              | Features | Requirements |
|------------|----------------------|----------|--------------|
| US-9.6.4.1 | player (P-23)        | F-9.6.4  | R-9.6.4      |
| US-9.6.4.2 | player (P-23)        | F-9.6.4  | R-9.6.4      |
| US-9.6.4.3 | game designer (P-5)  | F-9.6.4  | R-9.6.4      |
| US-9.6.4.4 | engine tester (P-27) | F-9.6.4  | R-9.6.4      |

1. **US-9.6.4.1** — I want equip/holster sequences that play draw animations when switching weapons
   with holster-then-draw cycling
   - **Acceptance:** weapon switching feels deliberate and weighty rather than instant
2. **US-9.6.4.2** — I want a dedicated inspect input that triggers a weapon inspection animation
   showing skins, engravings, and attachments
   - **Acceptance:** I can appreciate cosmetic customization in first person
3. **US-9.6.4.3** — I want dual wield fire modes (alternating, simultaneous, independent per
   trigger)
   - **Acceptance:** I can design distinct dual wield weapon classes with different firing behaviors
4. **US-9.6.4.4** — I want enable dual wield and verify that mobile uses simplified viewmodel LOD
   for the off-hand weapon to control the doubled draw call cost
   - **Acceptance:** dual wield is playable on mobile
