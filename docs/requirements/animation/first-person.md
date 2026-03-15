# R-9.6 -- First-Person Animation Requirements

## R-9.6.1 First-Person Camera Controller

The engine **SHALL** provide a first-person camera with head-bob synchronized to locomotion speed,
landing impact proportional to fall distance, procedural lean/peek with lateral translation,
camera tilt on strafing and slopes, spring-damper driven motion, and a separate viewmodel FOV
independent of world FOV.

- **Derived from:** [F-9.6.1](../../features/animation/first-person.md)
- **Rationale:** A physically-grounded first-person camera with spring-damper dynamics produces
  immersive, natural camera motion that responds to all locomotion states.
- **Verification:** Walk the character at constant speed and verify head-bob frequency matches the
  locomotion gait cycle within 1 frame. Drop the character from 3 m and verify a downward camera
  snap occurs proportional to fall distance. Activate lean-peek and verify the camera translates
  laterally by the configured offset. Set world FOV to 110 and viewmodel FOV to 70 and verify the
  weapon renders without distortion while the world renders at the wider FOV.

## R-9.6.2 Procedural Weapon Sway and Bob

The engine **SHALL** move the weapon viewmodel in response to player input and locomotion using
spring physics with per-weapon configurable stiffness, damping, and mass, producing input sway,
locomotion bob, and sprint tilt.

- **Derived from:** [F-9.6.2](../../features/animation/first-person.md)
- **Rationale:** Per-weapon spring parameters give each weapon a distinct physical feel (heavy LMG
  vs. light pistol), and procedural generation avoids authoring sway animations per weapon.
- **Verification:** Move the mouse right at constant speed and verify the weapon rotates left
  (opposite direction) with a lagging follow. Walk at constant speed and verify vertical weapon
  bob amplitude matches the configured value within 5%. Enter sprint and verify the weapon tilts
  to the configured carry angle. Compare a heavy weapon (high mass) and light weapon (low mass)
  and verify the heavy weapon has visibly more sway lag.

## R-9.6.3 Procedural Recoil and ADS Animation

The engine **SHALL** generate non-repetitive recoil animation from recoil pattern data, applying
rotational and translational kicks to the weapon viewmodel via spring forces with recovery, and
smoothly interpolate between hip and sight alignment positions for ADS with reduced sway and bob
during ADS.

- **Derived from:** [F-9.6.3](../../features/animation/first-person.md)
- **Rationale:** Procedural recoil from pattern data produces unique per-shot visual feedback, and
  ADS interpolation enables sight alignment without per-weapon ADS animations.
- **Verification:** Fire 10 consecutive shots and verify each recoil kick differs from the
  previous by at least 10% in either rotation or translation magnitude. Enter ADS and verify the
  weapon interpolates from hip to sight position within the configured duration. Verify weapon sway
  amplitude during ADS is reduced to at most 30% of the hip-fire sway amplitude. Switch between
  iron sights and a mounted optic and verify the sight alignment position changes correctly.

## R-9.6.4 Weapon Equip, Inspect, and Dual Wield

The engine **SHALL** support equip/holster sequences with draw animations, weapon inspection
animations triggered by input, and dual wielding of two one-handed weapons with independent sway,
bob, fire, and reload per hand, supporting alternating, simultaneous, and independent fire modes.

- **Derived from:** [F-9.6.4](../../features/animation/first-person.md)
- **Rationale:** Full weapon handling (equip, inspect, dual wield) provides tactile feedback and
  visual variety essential for first-person game feel.
- **Verification:** Switch weapons and verify holster-then-draw plays in sequence with no frame
  where neither weapon is visible. Trigger inspect and verify the weapon rotates to show all
  sides. Equip two one-handed weapons and verify each hand has independent sway by moving the
  mouse and observing both hands lag independently. Fire in alternating mode and verify shots
  alternate left-right-left. Fire in simultaneous mode and verify both weapons fire on the same
  frame.
