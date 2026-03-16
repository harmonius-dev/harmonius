# 9.6 — First-Person Animation

## F-9.6.1 First-Person Camera Controller

Specialized camera system for first-person games with head-bob synchronized to locomotion speed and
gait, landing impact (downward snap on ground contact proportional to fall distance), procedural
lean/peek (camera translates laterally when peeking around corners), and camera tilt on strafing and
slopes. Camera effects are driven by spring-damper systems with configurable stiffness and damping,
producing smooth, natural motion. Separate viewmodel FOV (weapon rendered at a different FOV than
the world) prevents weapon distortion at wide world FOVs. The camera integrates with the character
controller (F-4.1.8) for ground state, velocity, and orientation data.

- **Requirements:** R-9.6.1
- **Dependencies:** F-4.1.8 (Character Controller), F-13.2.1 (Camera System)
- **Platform notes:** Camera effects (head-bob, lean, tilt) run on all platforms. Spring-damper
  complexity uniform. Separate viewmodel FOV supported on all tiers.

### F-9.6.2 Procedural Weapon Sway and Bob

Weapon viewmodel moves in response to player input and locomotion. Input sway: the weapon rotates
and translates opposite to mouse/stick movement using spring physics, creating a lagging follow
effect. Locomotion bob: the weapon oscillates vertically and horizontally synchronized to
walking/running gait with configurable amplitude and frequency curves per movement speed. Sprint
tilt: the weapon rotates to a carry position during sprint. All sway and bob parameters are
per-weapon data — a heavy LMG sways more than a light pistol. The spring system uses configurable
stiffness, damping, and mass for physically-grounded feel.

- **Requirements:** R-9.6.2
- **Dependencies:** F-9.6.1, F-6.2.1 (Input Actions)
- **Platform notes:** Weapon sway and bob are lightweight spring systems on all platforms. No
  platform-specific scaling required.

### F-9.6.3 Procedural Recoil and ADS Animation

Non-repetitive recoil animation generated procedurally from recoil pattern data (F-13.16.3). Each
shot applies a rotational and translational kick to the weapon viewmodel via spring forces, with
recovery pulling back toward the rest position. ADS (aim-down-sights) smoothly interpolates the
weapon from hip position to the sight alignment position using configurable animation curves.
Multiple sight positions are supported per weapon (iron sights, mounted optic, canted sight)
selected by input toggle. During ADS, weapon sway is reduced and bob is minimized. Scope rendering
uses render-to-texture for magnified optics with scope shadow vignette.

- **Requirements:** R-9.6.3
- **Dependencies:** F-9.6.2, F-13.16.3 (Recoil Patterns)
- **Platform notes:** Recoil and ADS are lightweight spring systems on all platforms. Scope
  render-to-texture resolution: mobile half-res, desktop full-res.

### F-9.6.4 Weapon Equip, Inspect, and Dual Wield

Procedural and animated equip/holster sequences: weapon slides in from off-screen with a draw
animation, cycling between weapons plays holster-then-draw. Weapon inspection: a dedicated input
triggers an inspect animation where the weapon rotates and tilts for the player to admire (showing
skins, attachments, engravings). Dual wielding: two one-handed weapons held simultaneously with
independent sway, bob, fire, and reload per hand. Each hand has its own spring system. Dual wield
fire modes include: alternating (left-right-left), simultaneous (both fire together), and
independent (each hand fires on its own trigger). All animations are authored as assets or driven
procedurally from weapon data.

- **Requirements:** R-9.6.4
- **Dependencies:** F-9.6.2, F-9.4.1 (Animation State Machine)
- **Platform notes:** Dual wield renders two viewmodels — draw call cost doubles. Mobile may use
  simplified viewmodel LOD for the off-hand weapon.
