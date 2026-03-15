# User Stories — 4.7 Soft Body & Cloth

## F-4.7.1 Position-Based Dynamics Solver

**US-4.7.1a** As a gameplay programmer, I want the XPBD solver to run within the ECS
schedule with no separate physics world so that cloth and soft body simulation integrates
cleanly with all other ECS systems and queries.

## F-4.7.2 Cloth Simulation

**US-4.7.2a** As a designer, I want to attach cloth instances to skeleton bones so that
capes, banners, and sails follow animated characters and environmental anchors naturally.

**US-4.7.2b** As a player, I want capes and banners to drape, swing, and fold realistically
so that cloth-wearing characters and environmental fabric look alive.

## F-4.7.3 Cloth Self-Collision

**US-4.7.3a** As a designer, I want to enable self-collision on cloth entities so that
draped garments fold over themselves correctly instead of clipping through.

**US-4.7.3b** As a QA engineer, I want to verify that no two non-adjacent cloth particles
are closer than the configured thickness so that self-collision enforcement is measurably
correct.

## F-4.7.4 Two-Way Rigid Body Coupling

**US-4.7.4a** As a player, I want a heavy curtain draped over a light box to push the box
down so that cloth and rigid bodies interact physically rather than cloth being a
visual-only overlay.

**US-4.7.4b** As a gameplay programmer, I want two-way coupling implemented through
standard ECS component queries so that cloth-rigid-body interaction requires no custom
collision world.

## F-4.7.5 Wind Interaction

**US-4.7.5a** As a designer, I want to place directional, point, and vortex wind source
entities so that flags ripple, banners flutter, and sails billow according to localized
wind conditions.

**US-4.7.5b** As a player, I want to see cloth react to wind in real time — flags snapping
in a storm, banners calming when wind dies — so that the world feels weather-responsive.

## F-4.7.6 Cloth Tearing

**US-4.7.6a** As a player, I want sails and banners to tear when overstressed so that
battle damage to fabric is visible and destructible cloth adds to environmental
storytelling.

**US-4.7.6b** As a designer, I want to configure a tear-strain threshold per cloth instance
so that some fabrics tear easily (old sails) while others resist tearing (heavy tarps).

## F-4.7.7 Cloth Level of Detail

**US-4.7.7a** As a gameplay programmer, I want cloth LOD to reduce particle count and solver
iterations based on camera distance so that distant cloth consumes near-zero simulation
budget.

**US-4.7.7b** As a player, I want LOD transitions on cloth to be visually smooth so that
flags and capes do not pop or jitter as I move closer or further away.
