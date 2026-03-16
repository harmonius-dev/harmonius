# User Stories -- Minigame Framework (13.26)

## Minigame Session and Sandbox Context (F-13.26.1)

## US-13.26.1.1

**As a** player (P-23), **I want** entering a minigame to create an isolated session with its own
ECS world, input, camera, and UI, **so that** minigame state is separate from the outer game.

## US-13.26.1.2

**As a** player (P-23), **I want** the outer game to optionally pause or continue at reduced tick
rate during a minigame, **so that** the world behaves appropriately while I play.

## US-13.26.1.3

**As a** designer (P-5), **I want** a typed result contract defining entry conditions, input
parameters, output results, and side effects, **so that** minigame integration is explicit.

## US-13.26.1.4

**As a** designer (P-5), **I want** minigame sessions authored as self-contained project assets in
the visual editor, **so that** minigame creation requires no code.

## US-13.26.1.5

**As a** modder (P-24), **I want** to create custom minigame sessions using the same visual
authoring tools, **so that** modded minigames integrate seamlessly.

## US-13.26.1.6

**As a** tester (P-27), **I want** to verify that the minigame cannot access arbitrary outer world
state beyond the contract, **so that** isolation is enforced.

## US-13.26.1.7

**As a** tester (P-27), **I want** to verify that the outer game resumes correctly after minigame
teardown, **so that** the outer world state is not corrupted.

## Minigame World Presentation (F-13.26.2)

## US-13.26.2.1

**As a** player (P-23), **I want** minigames to render on in-world surfaces like arcade screens,
**so that** minigames feel physically embedded in the world.

## US-13.26.2.2

**As a** player (P-23), **I want** a fullscreen overlay mode for immersive minigames, **so that**
complex minigames get full screen attention.

## US-13.26.2.3

**As a** player (P-23), **I want** diegetic mode rendering minigame elements as 3D objects in the
world, **so that** chess pieces on a table look natural.

## US-13.26.2.4

**As a** designer (P-5), **I want** to choose between world-space, fullscreen overlay, split-view,
and diegetic presentation modes per minigame, **so that** I can match the presentation to the
activity.

## US-13.26.2.5

**As a** tester (P-27), **I want** to verify that world-space mode renders correctly on the
designated in-world surface, **so that** render-to-texture integration works.

## Minigame Lifecycle and Result Contract (F-13.26.3)

## US-13.26.3.1

**As a** player (P-23), **I want** entry conditions validated before starting (required items,
minimum level, currency cost), **so that** I know prerequisites upfront.

## US-13.26.3.2

**As a** player (P-23), **I want** results displayed on completion with earned items, currency, and
score, **so that** I see what I won.

## US-13.26.3.3

**As a** player (P-23), **I want** quitting mid-minigame to follow the contract's quit rule (loss,
refund, or no effect), **so that** quit behavior is predictable.

## US-13.26.3.4

**As a** designer (P-5), **I want** to define result contracts with entry conditions, output
results, and side effects per minigame, **so that** each minigame's integration is explicit.

## US-13.26.3.5

**As a** modder (P-24), **I want** to define custom result contracts for modded minigames,
**so that** mod minigames can grant rewards.

## US-13.26.3.6

**As a** tester (P-27), **I want** to verify that completing a minigame applies the result contract
to the outer world, **so that** rewards are granted correctly.

## US-13.26.3.7

**As a** tester (P-27), **I want** to verify that high scores persist across sessions through the
save system, **so that** minigame records are durable.

## Timing and Rhythm Minigames (F-13.26.4)

## US-13.26.4.1

**As a** player (P-23), **I want** to play timing and rhythm challenges with beat markers
synchronized to audio, **so that** musical minigames feel responsive.

## US-13.26.4.2

**As a** player (P-23), **I want** input windows with perfect/great/good/miss thresholds,
**so that** precision is rewarded with higher scores.

## US-13.26.4.3

**As a** designer (P-5), **I want** to author note patterns, audio tracks, and timing tolerances as
visual assets, **so that** rhythm content is data-driven.

## US-13.26.4.4

**As a** designer (P-5), **I want** the timing template usable for QTE sequences during cutscenes,
**so that** one system serves both minigames and cinematics.

## US-13.26.4.5

**As a** tester (P-27), **I want** to verify that perfect timing awards the maximum score
multiplier, **so that** threshold scoring is correct.

## Grid/Board Engine (F-13.26.5a)

## US-13.26.5a.1

**As a** player (P-23), **I want** to play card games, board games, and match-3 puzzles on a
configurable NxM grid, **so that** grid-based minigames feel polished.

## US-13.26.5a.2

**As a** designer (P-5), **I want** configurable cell types, turn-based or real-time modes, and
piece/card/tile management, **so that** the grid engine supports diverse game types.

## US-13.26.5a.3

**As a** modder (P-24), **I want** to create custom board layouts and piece types as mod assets,
**so that** modded board games use the same grid engine.

## US-13.26.5a.4

**As a** tester (P-27), **I want** to verify that the board renders correctly in all presentation
modes (world-space, fullscreen, diegetic), **so that** grid visuals work everywhere.

## Match Detection Algorithms (F-13.26.5b)

## US-13.26.5b.1

**As a** player (P-23), **I want** matches of 3-or-more in a row detected horizontally, vertically,
and diagonally, **so that** match-3 mechanics work.

## US-13.26.5b.2

**As a** designer (P-5), **I want** match detection to support poker hand evaluation and custom
logic graph rules, **so that** diverse match mechanics are possible.

## US-13.26.5b.3

**As a** modder (P-24), **I want** to define custom match rules as logic graph assets, **so that**
modded puzzles can introduce new match mechanics.

## US-13.26.5b.4

**As a** tester (P-27), **I want** to verify that diagonal match-3 detection works, **so that** all
match directions are supported.

## Board Minigame AI (F-13.26.5c)

## US-13.26.5c.1

**As a** player (P-23), **I want** AI opponents with easy, medium, and hard difficulty levels,
**so that** board minigames are challenging at every skill level.

## US-13.26.5c.2

**As a** designer (P-5), **I want** to configure AI evaluation depth, randomness, and heuristic
weights per difficulty tier, **so that** difficulty is tunable.

## US-13.26.5c.3

**As a** tester (P-27), **I want** to verify that AI decision latency stays below the configured
cap, **so that** AI turns do not cause perceptible delays.

## Board Piece Animation and Cascading (F-13.26.5d)

## US-13.26.5d.1

**As a** player (P-23), **I want** matched tiles to animate away, remaining tiles to fall, and new
matches to trigger cascading chain reactions, **so that** match-3 feels dynamic.

## US-13.26.5d.2

**As a** designer (P-5), **I want** cascade animations with configurable timing per step,
**so that** cascade pacing is tunable.

## US-13.26.5d.3

**As a** tester (P-27), **I want** to verify that win/loss evaluation runs only after all cascades
resolve, **so that** cascading does not prematurely end the game.

## Physics Toy Minigames (F-13.26.6)

## US-13.26.6.1

**As a** player (P-23), **I want** to play physics-driven activities like fishing, ball throwing,
and crane machines, **so that** skill-based minigames feel physical.

## US-13.26.6.2

**As a** player (P-23), **I want** analog input for nuanced control (rod tension, throw power, claw
position), **so that** physics minigames reward precision.

## US-13.26.6.3

**As a** designer (P-5), **I want** to configure physics parameters, interaction rules, and scoring
per physics minigame, **so that** skill-based activities are data-driven.

## US-13.26.6.4

**As a** tester (P-27), **I want** to verify that throw power scales with input magnitude,
**so that** analog input mapping is correct.

## Multiplayer Minigame Sessions (F-13.26.7)

## US-13.26.7.1

**As a** player (P-23), **I want** to play minigames with other players locally or online,
**so that** minigames support social play.

## US-13.26.7.2

**As a** player (P-23), **I want** spectators to watch minigame sessions in progress, **so that** I
can observe competitive matches.

## US-13.26.7.3

**As a** designer (P-5), **I want** turn-based minigames to use the turn manager for synchronization
and real-time ones to use prediction and rollback, **so that** networking matches the minigame type.

## US-13.26.7.4

**As a** tester (P-27), **I want** to verify that networked minigame state is server-authoritative,
**so that** cheating is prevented.

## Minigame Library and Discovery (F-13.26.8)

## US-13.26.8.1

**As a** player (P-23), **I want** to discover minigames through world interactions and access them
from a collectible menu, **so that** minigames are explorable.

## US-13.26.8.2

**As a** player (P-23), **I want** the minigame menu to display high scores, completion counts, and
achievement progress, **so that** I can track my minigame performance.

## US-13.26.8.3

**As a** player (P-23), **I want** to replay discovered minigames from the menu without returning to
the world location, **so that** replaying is convenient.

## US-13.26.8.4

**As a** designer (P-5), **I want** a runtime registry tracking available minigames with metadata
(category, player count, difficulty), **so that** discovery and sorting are data-driven.

## US-13.26.8.5

**As a** modder (P-24), **I want** modded minigames to register in the same library, **so that**
community minigames are discoverable alongside official ones.

## US-13.26.8.6

**As a** tester (P-27), **I want** to verify that minigame achievements integrate with the
achievement system, **so that** minigame-specific achievements unlock correctly.
