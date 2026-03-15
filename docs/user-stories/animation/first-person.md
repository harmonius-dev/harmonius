# User Stories -- 9.6 First-Person Animation

## US-9.6.1.1 Feel Natural Head Bob During First-Person Movement

**As a** player (P-23), **I want** head-bob synchronized to locomotion speed and gait, landing
impact on ground contact, and procedural lean/peek when peeking around corners, **so that**
first-person movement feels physically grounded and immersive.

## US-9.6.1.2 Configure Camera Spring-Damper Parameters Per Game

**As a** game designer (P-5), **I want** configurable stiffness, damping, and tilt parameters
for first-person camera effects, **so that** I can tune the feel from heavy military (low bob,
slow lean) to parkour (high bob, fast tilt) without code changes.

## US-9.6.1.3 Validate Separate Viewmodel FOV on All Platforms

**As an** engine tester (P-27), **I want** to set a world FOV of 110 with a viewmodel FOV of 70
and verify that the weapon renders without distortion on all platforms, **so that** wide FOV
does not stretch the weapon viewmodel.

## US-9.6.2.1 Feel Each Weapon's Weight Through Sway and Bob

**As a** player (P-23), **I want** weapon sway and bob driven by per-weapon spring physics where
a heavy LMG sways more than a light pistol, **so that** each weapon feels physically distinct
during movement and aiming.

## US-9.6.2.2 Tune Per-Weapon Sway Parameters as Data Assets

**As a** game designer (P-5), **I want** sway stiffness, damping, mass, and bob amplitude
exposed as per-weapon data assets that I edit in the editor, **so that** weapon feel is tunable
per weapon type without programmer support.

## US-9.6.2.3 Validate Sprint Tilt Activates at Sprint Speed Threshold

**As an** engine tester (P-27), **I want** to trigger sprint and verify that the weapon rotates
to a carry position only when movement speed exceeds the sprint threshold, **so that** sprint
tilt transitions correctly based on movement state.

## US-9.6.3.1 Generate Non-Repetitive Recoil From Pattern Data

**As a** player (P-23), **I want** procedural recoil that applies rotational and translational
kick from recoil pattern data with spring-based recovery, **so that** sustained fire produces
varied, non-repetitive weapon kick.

## US-9.6.3.2 Smoothly Transition to Aim-Down-Sights Position

**As a** player (P-23), **I want** ADS to interpolate the weapon from hip to sight alignment
with reduced sway and minimized bob, **so that** aiming feels precise and stable.

## US-9.6.3.3 Validate Scope Render-to-Texture Resolution Per Platform

**As an** engine tester (P-27), **I want** to toggle ADS with a magnified optic and verify that
scope render-to-texture uses half-res on mobile and full-res on desktop, **so that** scope
rendering stays within per-platform GPU budgets.

## US-9.6.4.1 See Weapon Draw and Holster Animations When Switching

**As a** player (P-23), **I want** equip/holster sequences that play draw animations when
switching weapons with holster-then-draw cycling, **so that** weapon switching feels deliberate
and weighty rather than instant.

## US-9.6.4.2 Inspect Weapons to Admire Skins and Attachments

**As a** player (P-23), **I want** a dedicated inspect input that triggers a weapon inspection
animation showing skins, engravings, and attachments, **so that** I can appreciate cosmetic
customization in first person.

## US-9.6.4.3 Fire Dual-Wielded Weapons Independently Per Hand

**As a** game designer (P-5), **I want** dual wield fire modes (alternating, simultaneous,
independent per trigger), **so that** I can design distinct dual wield weapon classes with
different firing behaviors.

## US-9.6.4.4 Validate Dual Wield Draw Call Cost on Mobile

**As an** engine tester (P-27), **I want** to enable dual wield and verify that mobile uses
simplified viewmodel LOD for the off-hand weapon to control the doubled draw call cost, **so
that** dual wield is playable on mobile.
