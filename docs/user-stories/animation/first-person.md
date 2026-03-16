# User Stories -- 9.6 First-Person Animation

## F-9.6.1

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-9.6.1.1 | player (P-23) | I want head-bob synchronized to locomotion speed and gait, landing impact on ground contact, and procedural lean/peek when peeking around corners | first-person movement feels physically grounded and immersive | F-9.6.1 | R-9.6.1 |
| US-9.6.1.2 | game designer (P-5) | I want configurable stiffness, damping, and tilt parameters for first-person camera effects | I can tune the feel from heavy military (low bob, slow lean) to parkour (high bob, fast tilt) without code changes | F-9.6.1 | R-9.6.1 |
| US-9.6.1.3 | engine tester (P-27) | I want set a world FOV of 110 with a viewmodel FOV of 70 and verify that the weapon renders without distortion on all platforms | wide FOV does not stretch the weapon viewmodel | F-9.6.1 | R-9.6.1 |

## F-9.6.2

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-9.6.2.1 | player (P-23) | I want weapon sway and bob driven by per-weapon spring physics where a heavy LMG sways more than a light pistol | each weapon feels physically distinct during movement and aiming | F-9.6.2 | R-9.6.2 |
| US-9.6.2.2 | game designer (P-5) | I want sway stiffness, damping, mass, and bob amplitude exposed as per-weapon data assets that I edit in the editor | weapon feel is tunable per weapon type without programmer support | F-9.6.2 | R-9.6.2 |
| US-9.6.2.3 | engine tester (P-27) | I want trigger sprint and verify that the weapon rotates to a carry position only when movement speed exceeds the sprint threshold | sprint tilt transitions correctly based on movement state | F-9.6.2 | R-9.6.2 |

## F-9.6.3

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-9.6.3.1 | player (P-23) | I want procedural recoil that applies rotational and translational kick from recoil pattern data with spring-based recovery | sustained fire produces varied, non-repetitive weapon kick | F-9.6.3 | R-9.6.3 |
| US-9.6.3.2 | player (P-23) | I want ADS to interpolate the weapon from hip to sight alignment with reduced sway and minimized bob | aiming feels precise and stable | F-9.6.3 | R-9.6.3 |
| US-9.6.3.3 | engine tester (P-27) | I want toggle ADS with a magnified optic and verify that scope render-to-texture uses half-res on mobile and full-res on desktop | scope rendering stays within per-platform GPU budgets | F-9.6.3 | R-9.6.3 |

## F-9.6.4

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-9.6.4.1 | player (P-23) | I want equip/holster sequences that play draw animations when switching weapons with holster-then-draw cycling | weapon switching feels deliberate and weighty rather than instant | F-9.6.4 | R-9.6.4 |
| US-9.6.4.2 | player (P-23) | I want a dedicated inspect input that triggers a weapon inspection animation showing skins, engravings, and attachments | I can appreciate cosmetic customization in first person | F-9.6.4 | R-9.6.4 |
| US-9.6.4.3 | game designer (P-5) | I want dual wield fire modes (alternating, simultaneous, independent per trigger) | I can design distinct dual wield weapon classes with different firing behaviors | F-9.6.4 | R-9.6.4 |
| US-9.6.4.4 | engine tester (P-27) | I want enable dual wield and verify that mobile uses simplified viewmodel LOD for the off-hand weapon to control the doubled draw call cost | dual wield is playable on mobile | F-9.6.4 | R-9.6.4 |
