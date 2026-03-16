# User Stories -- Minigame Framework (13.26)

## Minigame Session and Sandbox Context (F-13.26.1)

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-13.26.1.1 | player (P-23) | **As a** player (P-23), **I want** entering a minigame to create an isolated session with its own ECS world, input, camera, and UI, **so that** minigame state is separate from the outer game. |  | F-13.26.1 | R-13.26.1 |
| US-13.26.1.2 | player (P-23) | **As a** player (P-23), **I want** the outer game to optionally pause or continue at reduced tick rate during a minigame, **so that** the world behaves appropriately while I play. |  | F-13.26.1 | R-13.26.1 |
| US-13.26.1.3 | designer (P-5) | **As a** designer (P-5), **I want** a typed result contract defining entry conditions, input parameters, output results, and side effects, **so that** minigame integration is explicit. |  | F-13.26.1 | R-13.26.1 |
| US-13.26.1.4 | designer (P-5) | **As a** designer (P-5), **I want** minigame sessions authored as self-contained project assets in the visual editor, **so that** minigame creation requires no code. |  | F-13.26.1 | R-13.26.1 |
| US-13.26.1.5 | modder (P-24) | **As a** modder (P-24), **I want** to create custom minigame sessions using the same visual authoring tools, **so that** modded minigames integrate seamlessly. |  | F-13.26.1 | R-13.26.1 |
| US-13.26.1.6 | tester (P-27) | **As a** tester (P-27), **I want** to verify that the minigame cannot access arbitrary outer world state beyond the contract, **so that** isolation is enforced. |  | F-13.26.1 | R-13.26.1 |
| US-13.26.1.7 | tester (P-27) | **As a** tester (P-27), **I want** to verify that the outer game resumes correctly after minigame teardown, **so that** the outer world state is not corrupted. |  | F-13.26.1 | R-13.26.1 |

## Minigame World Presentation (F-13.26.2)

| US-13.26.2.1 | player (P-23) | **As a** player (P-23), **I want** minigames to render on in-world surfaces like arcade screens, **so that** minigames feel physically embedded in the world. |  | F-13.26.2 | R-13.26.2 |
| US-13.26.2.2 | player (P-23) | **As a** player (P-23), **I want** a fullscreen overlay mode for immersive minigames, **so that** complex minigames get full screen attention. |  | F-13.26.2 | R-13.26.2 |
| US-13.26.2.3 | player (P-23) | **As a** player (P-23), **I want** diegetic mode rendering minigame elements as 3D objects in the world, **so that** chess pieces on a table look natural. |  | F-13.26.2 | R-13.26.2 |
| US-13.26.2.4 | designer (P-5) | **As a** designer (P-5), **I want** to choose between world-space, fullscreen overlay, split-view, and diegetic presentation modes per minigame, **so that** I can match the presentation to the activity. |  | F-13.26.2 | R-13.26.2 |
| US-13.26.2.5 | tester (P-27) | **As a** tester (P-27), **I want** to verify that world-space mode renders correctly on the designated in-world surface, **so that** render-to-texture integration works. |  | F-13.26.2 | R-13.26.2 |

## Minigame Lifecycle and Result Contract (F-13.26.3)

| US-13.26.3.1 | player (P-23) | **As a** player (P-23), **I want** entry conditions validated before starting (required items, minimum level, currency cost), **so that** I know prerequisites upfront. |  | F-13.26.3 | R-13.26.3 |
| US-13.26.3.2 | player (P-23) | **As a** player (P-23), **I want** results displayed on completion with earned items, currency, and score, **so that** I see what I won. |  | F-13.26.3 | R-13.26.3 |
| US-13.26.3.3 | player (P-23) | **As a** player (P-23), **I want** quitting mid-minigame to follow the contract's quit rule (loss, refund, or no effect), **so that** quit behavior is predictable. |  | F-13.26.3 | R-13.26.3 |
| US-13.26.3.4 | designer (P-5) | **As a** designer (P-5), **I want** to define result contracts with entry conditions, output results, and side effects per minigame, **so that** each minigame's integration is explicit. |  | F-13.26.3 | R-13.26.3 |
| US-13.26.3.5 | modder (P-24) | **As a** modder (P-24), **I want** to define custom result contracts for modded minigames, **so that** mod minigames can grant rewards. |  | F-13.26.3 | R-13.26.3 |
| US-13.26.3.6 | tester (P-27) | **As a** tester (P-27), **I want** to verify that completing a minigame applies the result contract to the outer world, **so that** rewards are granted correctly. |  | F-13.26.3 | R-13.26.3 |
| US-13.26.3.7 | tester (P-27) | **As a** tester (P-27), **I want** to verify that high scores persist across sessions through the save system, **so that** minigame records are durable. |  | F-13.26.3 | R-13.26.3 |

## Timing and Rhythm Minigames (F-13.26.4)

| US-13.26.4.1 | player (P-23) | **As a** player (P-23), **I want** to play timing and rhythm challenges with beat markers synchronized to audio, **so that** musical minigames feel responsive. |  | F-13.26.4 | R-13.26.4 |
| US-13.26.4.2 | player (P-23) | **As a** player (P-23), **I want** input windows with perfect/great/good/miss thresholds, **so that** precision is rewarded with higher scores. |  | F-13.26.4 | R-13.26.4 |
| US-13.26.4.3 | designer (P-5) | **As a** designer (P-5), **I want** to author note patterns, audio tracks, and timing tolerances as visual assets, **so that** rhythm content is data-driven. |  | F-13.26.4 | R-13.26.4 |
| US-13.26.4.4 | designer (P-5) | **As a** designer (P-5), **I want** the timing template usable for QTE sequences during cutscenes, **so that** one system serves both minigames and cinematics. |  | F-13.26.4 | R-13.26.4 |
| US-13.26.4.5 | tester (P-27) | **As a** tester (P-27), **I want** to verify that perfect timing awards the maximum score multiplier, **so that** threshold scoring is correct. |  | F-13.26.4 | R-13.26.4 |

## Grid/Board Engine (F-13.26.5a)

| US-13.26.5 | player (P-23) | **As a** player (P-23), **I want** to play card games, board games, and match-3 puzzles on a configurable NxM grid, **so that** grid-based minigames feel polished. |  | F-13.26.5 | R-13.26.5 |
| US-13.26.5 | designer (P-5) | **As a** designer (P-5), **I want** configurable cell types, turn-based or real-time modes, and piece/card/tile management, **so that** the grid engine supports diverse game types. |  | F-13.26.5 | R-13.26.5 |
| US-13.26.5 | modder (P-24) | **As a** modder (P-24), **I want** to create custom board layouts and piece types as mod assets, **so that** modded board games use the same grid engine. |  | F-13.26.5 | R-13.26.5 |
| US-13.26.5 | tester (P-27) | **As a** tester (P-27), **I want** to verify that the board renders correctly in all presentation modes (world-space, fullscreen, diegetic), **so that** grid visuals work everywhere. |  | F-13.26.5 | R-13.26.5 |

## Match Detection Algorithms (F-13.26.5b)

| US-13.26.5 | player (P-23) | **As a** player (P-23), **I want** matches of 3-or-more in a row detected horizontally, vertically, and diagonally, **so that** match-3 mechanics work. |  | F-13.26.5 | R-13.26.5 |
| US-13.26.5 | designer (P-5) | **As a** designer (P-5), **I want** match detection to support poker hand evaluation and custom logic graph rules, **so that** diverse match mechanics are possible. |  | F-13.26.5 | R-13.26.5 |
| US-13.26.5 | modder (P-24) | **As a** modder (P-24), **I want** to define custom match rules as logic graph assets, **so that** modded puzzles can introduce new match mechanics. |  | F-13.26.5 | R-13.26.5 |
| US-13.26.5 | tester (P-27) | **As a** tester (P-27), **I want** to verify that diagonal match-3 detection works, **so that** all match directions are supported. |  | F-13.26.5 | R-13.26.5 |

## Board Minigame AI (F-13.26.5c)

| US-13.26.5 | player (P-23) | **As a** player (P-23), **I want** AI opponents with easy, medium, and hard difficulty levels, **so that** board minigames are challenging at every skill level. |  | F-13.26.5 | R-13.26.5 |
| US-13.26.5 | designer (P-5) | **As a** designer (P-5), **I want** to configure AI evaluation depth, randomness, and heuristic weights per difficulty tier, **so that** difficulty is tunable. |  | F-13.26.5 | R-13.26.5 |
| US-13.26.5 | tester (P-27) | **As a** tester (P-27), **I want** to verify that AI decision latency stays below the configured cap, **so that** AI turns do not cause perceptible delays. |  | F-13.26.5 | R-13.26.5 |

## Board Piece Animation and Cascading (F-13.26.5d)

| US-13.26.5 | player (P-23) | **As a** player (P-23), **I want** matched tiles to animate away, remaining tiles to fall, and new matches to trigger cascading chain reactions, **so that** match-3 feels dynamic. |  | F-13.26.5 | R-13.26.5 |
| US-13.26.5 | designer (P-5) | **As a** designer (P-5), **I want** cascade animations with configurable timing per step, **so that** cascade pacing is tunable. |  | F-13.26.5 | R-13.26.5 |
| US-13.26.5 | tester (P-27) | **As a** tester (P-27), **I want** to verify that win/loss evaluation runs only after all cascades resolve, **so that** cascading does not prematurely end the game. |  | F-13.26.5 | R-13.26.5 |

## Physics Toy Minigames (F-13.26.6)

| US-13.26.6.1 | player (P-23) | **As a** player (P-23), **I want** to play physics-driven activities like fishing, ball throwing, and crane machines, **so that** skill-based minigames feel physical. |  | F-13.26.6 | R-13.26.6 |
| US-13.26.6.2 | player (P-23) | **As a** player (P-23), **I want** analog input for nuanced control (rod tension, throw power, claw position), **so that** physics minigames reward precision. |  | F-13.26.6 | R-13.26.6 |
| US-13.26.6.3 | designer (P-5) | **As a** designer (P-5), **I want** to configure physics parameters, interaction rules, and scoring per physics minigame, **so that** skill-based activities are data-driven. |  | F-13.26.6 | R-13.26.6 |
| US-13.26.6.4 | tester (P-27) | **As a** tester (P-27), **I want** to verify that throw power scales with input magnitude, **so that** analog input mapping is correct. |  | F-13.26.6 | R-13.26.6 |

## Multiplayer Minigame Sessions (F-13.26.7)

| US-13.26.7.1 | player (P-23) | **As a** player (P-23), **I want** to play minigames with other players locally or online, **so that** minigames support social play. |  | F-13.26.7 | R-13.26.7 |
| US-13.26.7.2 | player (P-23) | **As a** player (P-23), **I want** spectators to watch minigame sessions in progress, **so that** I can observe competitive matches. |  | F-13.26.7 | R-13.26.7 |
| US-13.26.7.3 | designer (P-5) | **As a** designer (P-5), **I want** turn-based minigames to use the turn manager for synchronization and real-time ones to use prediction and rollback, **so that** networking matches the minigame type. |  | F-13.26.7 | R-13.26.7 |
| US-13.26.7.4 | tester (P-27) | **As a** tester (P-27), **I want** to verify that networked minigame state is server-authoritative, **so that** cheating is prevented. |  | F-13.26.7 | R-13.26.7 |

## Minigame Library and Discovery (F-13.26.8)

| US-13.26.8.1 | player (P-23) | **As a** player (P-23), **I want** to discover minigames through world interactions and access them from a collectible menu, **so that** minigames are explorable. |  | F-13.26.8 | R-13.26.8 |
| US-13.26.8.2 | player (P-23) | **As a** player (P-23), **I want** the minigame menu to display high scores, completion counts, and achievement progress, **so that** I can track my minigame performance. |  | F-13.26.8 | R-13.26.8 |
| US-13.26.8.3 | player (P-23) | **As a** player (P-23), **I want** to replay discovered minigames from the menu without returning to the world location, **so that** replaying is convenient. |  | F-13.26.8 | R-13.26.8 |
| US-13.26.8.4 | designer (P-5) | **As a** designer (P-5), **I want** a runtime registry tracking available minigames with metadata (category, player count, difficulty), **so that** discovery and sorting are data-driven. |  | F-13.26.8 | R-13.26.8 |
| US-13.26.8.5 | modder (P-24) | **As a** modder (P-24), **I want** modded minigames to register in the same library, **so that** community minigames are discoverable alongside official ones. |  | F-13.26.8 | R-13.26.8 |
| US-13.26.8.6 | tester (P-27) | **As a** tester (P-27), **I want** to verify that minigame achievements integrate with the achievement system, **so that** minigame-specific achievements unlock correctly. |  | F-13.26.8 | R-13.26.8 |
