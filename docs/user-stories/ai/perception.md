# User Stories -- 7.6 Perception

## US-7.6.1 Sneak Past Guards Using Line of Sight
**As a** player, **I want** guards to have realistic vision cones with line-of-sight checks,
**so that** I can sneak behind them or hide behind walls and feel rewarded for using cover
and positioning.

## US-7.6.2 NPCs Forget Targets Over Time
**As an** AI designer, **I want** NPC perception confidence to decay over time when a target
is no longer visible, **so that** NPCs search briefly, then give up and return to patrol
rather than tracking the player indefinitely.

## US-7.6.3 Stay Downwind to Avoid Detection
**As a** player, **I want** predator AI to track me by scent that drifts with the wind,
**so that** I can use wind direction strategically — staying downwind to avoid detection or
using rain to wash away my scent trail.

## US-7.6.4 Configure Scent Persistence per Creature
**As an** AI designer, **I want** to configure how long different scent types persist (blood
trails last hours, footstep scent fades in minutes), **so that** tracking difficulty varies
by situation and creature type without writing code.

## US-7.6.5 Tracking Dogs Follow Footprints in Snow
**As a** player, **I want** enemy tracking dogs to follow my footprints through snow and mud,
**so that** I have to think about which surfaces I walk on to avoid leaving a trail, adding
tension and strategic depth to stealth gameplay.

## US-7.6.6 AI Investigates Suspicious Sounds
**As a** player, **I want** guards to investigate suspicious sounds by walking to the source
and looking around, **so that** I can throw distractions to lure them away from their posts
and create openings to sneak past.

## US-7.6.7 AI Coordinates Investigation to Avoid Redundancy
**As an** AI designer, **I want** only one nearby guard to investigate a sound while others
hold position, **so that** AI feels coordinated and realistic rather than every guard
abandoning their post for the same noise.

## US-7.6.8 Wolves Hunt in Packs Using Shared Tracking
**As a** player, **I want** wolf packs to share tracking information — when one wolf spots me,
all pack members update their pursuit direction, **so that** pack hunting feels coordinated
and dangerous, forcing me to break contact with the entire pack, not just one wolf.

## US-7.6.9 AI Seamlessly Switches Between Senses
**As a** player, **I want** a pursuing enemy to switch from chasing me by sight to tracking
my footprints when I break line of sight, then switch to following my scent when footprints
end on hard ground, **so that** escape requires countering multiple detection methods and
the pursuit feels relentless and intelligent.

## US-7.6.10 Blood Trails Lead Enemies to Injured Players
**As a** player, **I want** enemies to find me by following the blood drops I leave when
injured, **so that** getting hurt has consequences beyond HP loss — I need to bandage wounds
to stop leaving evidence, adding survival depth to combat encounters.
