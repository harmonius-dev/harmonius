# 13.26 — Minigame Framework

## Minigame Context

| ID        | Feature                                |
|-----------|----------------------------------------|
| F-13.26.1 | Minigame Session and Sandbox Context   |
| F-13.26.2 | Minigame World Presentation            |
| F-13.26.3 | Minigame Lifecycle and Result Contract |

1. **F-13.26.1** — A self-contained game context that runs within the outer game world as a "game
   within a game." When a player enters a minigame (interacting with an arcade cabinet, sitting at a
   card table, casting a fishing rod, approaching a puzzle lock), the engine creates an isolated
   minigame session with its own ECS world partition (F-1.1.34), input mapping context (F-6.2.2),
   camera (F-13.25.1), and UI layer (F-10.1.1). The outer game world optionally pauses, continues at
   reduced tick rate, or runs normally depending on the minigame configuration. The minigame session
   defines a bounded interaction surface with the outer world: it can read specified outer game
   state (player inventory, stats, currencies) and write results on completion (grant items, deduct
   currency, set quest flags) through a typed result contract. The outer world cannot directly
   mutate minigame state, and the minigame cannot access arbitrary outer world state — only what the
   contract permits. Minigame sessions are authored as self-contained project assets in the visual
   editor.
   - **Deps:** F-1.1.34 (Multiple World Support), F-6.2.2 (Input Mapping Contexts), F-13.25.1
     (Virtual Camera Framework), F-10.1.1 (Widget Tree)
2. **F-13.26.2** — Render the minigame within the outer game world using configurable presentation
   modes. **World- space mode**: the minigame renders on an in-world surface (arcade screen, card
   table, puzzle panel) using render-to-texture (F-2.3.8) — the player character remains visible
   interacting with the physical object. **Fullscreen overlay mode**: the minigame takes over the
   full screen with a themed UI frame, while the outer world is blurred or dimmed behind it.
   **Split-view mode**: the minigame occupies a portion of the screen (e.g., a fishing rod view in
   the lower half) while the outer world remains visible above. **Diegetic mode**: the minigame's
   elements are rendered as 3D objects in the world (chess pieces on a table, cards dealt on a
   surface) using the outer world's lighting and camera. Presentation mode is configured per
   minigame asset.
   - **Deps:** F-13.26.1, F-2.3.8 (Render-to-Texture), F-10.4.6 (World-Space UI)
3. **F-13.26.3** — Each minigame defines a typed result contract specifying: entry conditions
   (required items, minimum level, currency cost), input parameters (difficulty level, bet amount,
   opponent selection), output results (win/lose/draw, score, earned items, earned currency, time
   taken), and side effects on the outer world (set quest flag, unlock achievement, modify NPC
   relationship). The lifecycle phases are: **setup** (validate entry conditions, deduct costs,
   initialize minigame state), **play** (minigame runs until completion or player exit), **result**
   (display outcome, apply result contract to outer world), and **teardown** (destroy minigame world
   partition, restore outer world state). Players can quit mid-minigame; the result contract defines
   whether quitting counts as a loss, refunds the entry cost, or has no effect. Results integrate
   with the save system (F-13.3.1) — minigame high scores and completion counts persist.
   - **Deps:** F-13.26.1, F-13.9.1 (Inventory), F-13.7.6 (Currency), F-13.3.1 (Save System)

## Minigame Types

| ID         | Feature                             |
|------------|-------------------------------------|
| F-13.26.4  | Timing and Rhythm Minigames         |
| F-13.26.5a | Grid/Board Engine                   |
| F-13.26.5b | Match Detection Algorithms          |
| F-13.26.5c | Board Minigame AI                   |
| F-13.26.5d | Board Piece Animation and Cascading |
| F-13.26.6  | Physics Toy Minigames               |

1. **F-13.26.4** — A reusable timing/rhythm minigame template for QTE sequences, rhythm-based
   activities (playing instruments, dancing), and precision-timing challenges (fishing bite
   detection, lockpick sweet spot). The template provides: a timeline with beat markers synchronized
   to audio (F-5.4.5), input windows with configurable tolerance (perfect/great/good/miss
   thresholds), score accumulation with combo multipliers (F-13.24.3), visual feedback (note
   highway, shrinking circles, expanding rings), and a result grade. The timeline, note patterns,
   and audio tracks are authored as visual assets. Timing-based QTEs during cutscenes (F-13.5.1) use
   this same system with the cinematics sequencer driving the timeline.
   - **Deps:** F-13.26.1, F-5.4.5 (Beat Tracking), F-13.24.3 (Score and Combo), F-6.2.1 (Input
     Actions)
2. **F-13.26.5a** — A reusable NxM grid engine for card games, board games, match-3 puzzles,
   chess-like tactics, and tile-based challenges. Configurable cell types and rules, turn-based or
   real-time play modes, and piece/card/tile entity management. Board layout and piece types are
   authored as visual assets. The board renders in any minigame presentation mode (world-space cards
   on a table, fullscreen puzzle screen, diegetic chess board).
   - **Deps:** F-13.26.1, F-13.26.2 (Presentation)
3. **F-13.26.5b** — Pattern-matching algorithms for grid-based minigames: 3-or-more in a row
   (horizontal, vertical, diagonal), poker hand evaluation, and custom rule functions authored as
   logic graphs. Match detection runs after each move or on a configurable trigger. Detected matches
   produce a match result consumed by scoring and cascade systems.
   - **Deps:** F-13.26.5a, F-15.8.4 (Logic Graphs)
4. **F-13.26.5c** — AI opponents for board and grid minigames using the utility AI system (F-7.4.1).
   At least three difficulty tiers (easy, medium, hard) with configurable evaluation depth,
   randomness, and heuristic weights. AI decision latency is capped to prevent perceptible delays
   between turns.
   - **Deps:** F-13.26.5a, F-7.4.1 (Utility AI)
5. **F-13.26.5d** — Animated piece movement, placement, and removal on the grid. Gravity/cascading
   for falling-tile games: matched pieces are removed, remaining pieces fall to fill gaps, and new
   matches are checked recursively. Cascade animations play sequentially with configurable timing
   per step. Win/loss condition evaluation runs after all cascades resolve.
   - **Deps:** F-13.26.5a, F-13.26.5b
6. **F-13.26.6** — A reusable physics-driven minigame template for skill-based activities: fishing
   (cast, reel, tension management), ball-throwing games (aim, power, trajectory), crane/claw
   machines (joystick control of a physics-simulated claw), pinball, and physics puzzles (Rube
   Goldberg contraptions, ball mazes). The template provides: a physics sandbox using the engine's
   physics system (F-4.1.1) with configurable gravity, constraints, and interaction rules; analog
   input mapping for nuanced control (rod tension, throw power, claw position); skill-check
   resolution based on timing and input precision; and progressive difficulty scaling. Physics
   parameters, object layouts, and scoring rules are authored as visual assets.
   - **Deps:** F-13.26.1, F-4.1.1 (Rigid Body ECS), F-4.3.1 (Joints), F-6.2.1 (Input Actions)

## Multiplayer and Integration

| ID        | Feature                        |
|-----------|--------------------------------|
| F-13.26.7 | Multiplayer Minigame Sessions  |
| F-13.26.8 | Minigame Library and Discovery |

1. **F-13.26.7** — Minigame sessions support multiple players — both local (split-screen or
   shared-screen) and networked. Networked minigame sessions replicate minigame state through the
   networking system (F-8.2.1) with the same server-authoritative model as the outer game.
   Turn-based minigames (card games, board games) use the turn manager (F-13.21.2) for turn
   synchronization. Real-time minigames (racing, fishing competitions) use prediction and rollback
   (F-8.4.1) for responsive play. Spectators can watch minigame sessions in progress. Minigame
   matchmaking uses the session system (F-8.5.1) for competitive minigames with ratings.
   - **Deps:** F-13.26.1, F-8.2.1 (State Replication), F-8.4.1 (Prediction), F-8.5.1 (Lobby System),
     F-13.21.2 (Turn Manager)
2. **F-13.26.8** — A runtime minigame registry that tracks all available minigames in the project
   with metadata: name, description, category (timing, board, physics, custom), player count range,
   average duration, difficulty rating, and unlock conditions. Players discover minigames through
   world interactions (finding an arcade cabinet, being challenged by an NPC, reaching a quest
   objective). A collectible minigame menu (accessible from the pause menu or a dedicated in-world
   location like an arcade hall) displays all discovered minigames with high scores, completion
   counts, and achievement progress. Minigames can be replayed from the menu without returning to
   the original world location. The registry integrates with the achievement system (F-13.12.6) for
   minigame- specific achievements (beat all minigames, achieve S rank, win 10 multiplayer matches).
   - **Deps:** F-13.26.1, F-13.12.6 (Achievements), F-13.3.1 (Save System), F-10.3.1 (HUD Widgets)
