# R-13.26 — Minigame Framework Requirements

## R-13.26.1 Minigame Session and Sandbox Context

The engine **SHALL** create an isolated ECS world partition for each minigame session with its own
input mapping context, camera, and UI layer. The minigame **SHALL** interact with the outer game
world exclusively through a typed result contract that defines readable outer state and writable
result fields. The minigame **SHALL NOT** access outer world state beyond the contract. The outer
world **SHALL** optionally pause, reduce tick rate, or continue normally as configured per minigame
asset. Session creation and teardown **SHALL** complete within 500ms.

- **Derived from:** [F-13.26.1](../../features/game-framework/minigames.md)
- **Rationale:** Isolation prevents minigames from corrupting outer game state or introducing
  unintended dependencies. The result contract makes data flow explicit and auditable.
- **Verification:** Enter a minigame; verify it cannot read ECS components outside its contract.
  Verify outer world pause/resume behavior matches configuration. Measure session creation time.

## R-13.26.2 Minigame World Presentation

The engine **SHALL** support four presentation modes for minigames: world-space (render-to- texture
on an in-world surface), fullscreen overlay (themed UI frame over dimmed outer world), split-view
(minigame in a screen region with outer world visible), and diegetic (minigame elements as 3D
objects in the outer world). Presentation mode **SHALL** be configurable per minigame asset without
code changes.

- **Derived from:** [F-13.26.2](../../features/game-framework/minigames.md)
- **Rationale:** Different minigame types require different presentation styles — a card game
  renders on a table surface, a fishing rod view splits the screen, a puzzle takes over fullscreen.
- **Verification:** Create a test minigame in each presentation mode; verify correct rendering with
  no outer world artifacts (no UI bleed, correct depth sorting, proper input routing).

## R-13.26.3 Minigame Lifecycle and Result Contract

The engine **SHALL** enforce a four-phase minigame lifecycle (setup, play, result, teardown) with
typed result contracts. Entry conditions **SHALL** be validated before the play phase begins. Result
contract outputs **SHALL** be applied to the outer world atomically on the result phase — no partial
application on crash or quit. Player quit mid-minigame **SHALL** execute the contract's quit policy
(loss, refund, or no-effect) as configured. Minigame high scores and completion counts **SHALL**
persist through the save system.

- **Derived from:** [F-13.26.3](../../features/game-framework/minigames.md)
- **Rationale:** Typed contracts prevent minigames from producing unexpected side effects. Atomic
  result application prevents item duplication or currency exploits from interrupted sessions.
- **Verification:** Complete a minigame; verify results are applied. Crash during play phase; verify
  no partial results are applied. Quit mid-game; verify quit policy executes correctly.

## R-13.26.4 Timing and Rhythm Minigames

The engine **SHALL** provide a reusable timing/rhythm template with audio-synchronized beat markers,
configurable input tolerance windows (perfect: +/-30ms, great: +/-60ms, good: +/-100ms as defaults),
combo multiplier scoring, and letter/star grade output. Input latency from audio output to hit
detection **SHALL** be under 16ms. The template **SHALL** support QTE sequences driven by the
cinematics sequencer.

- **Derived from:** [F-13.26.4](../../features/game-framework/minigames.md)
- **Rationale:** Timing precision is critical for rhythm games. Sub-frame input latency ensures
  player inputs feel responsive and fair.
- **Verification:** Play a rhythm sequence at 120 BPM; verify hit detection windows match configured
  tolerances. Measure audio-to-input latency; verify under 16ms.

## R-13.26.5a Grid/Board Engine

The engine **SHALL** provide a reusable NxM grid engine with configurable cell types, turn-based or
real-time modes, and piece/card/tile entity management. Board layout and piece types **SHALL** be
authored as visual assets. The board **SHALL** render in all supported minigame presentation modes.

- **Derived from:** [F-13.26.5a](../../features/game-framework/minigames.md)
- **Rationale:** A reusable grid engine avoids reimplementing board logic for each grid-based
  minigame (card games, match-3, chess variants, tile puzzles).
- **Verification:** Create a 4x4 grid and an 8x8 grid; verify correct cell layout and piece
  placement. Switch between turn-based and real-time modes; verify correct behavior.

## R-13.26.5b Match Detection Algorithms

The engine **SHALL** provide match detection algorithms (3-or-more in a row, poker hands) and
support custom rule functions authored as logic graph assets. Match detection **SHALL** produce
typed results consumed by scoring and cascade systems.

- **Derived from:** [F-13.26.5b](../../features/game-framework/minigames.md)
- **Rationale:** Reusable match detection covers the most common grid-based game patterns while
  logic graph extensibility supports custom rules.
- **Verification:** Create a match-3 minigame; verify 3-in-a-row detection (horizontal, vertical,
  diagonal). Author a custom match rule via logic graph; verify it triggers.

## R-13.26.5c Board Minigame AI

The engine **SHALL** provide AI opponents for board minigames via the utility AI system with at
least 3 difficulty levels that produce measurably different win rates (easy: <30%, medium: ~50%,
hard: >70% against a random-play baseline).

- **Derived from:** [F-13.26.5c](../../features/game-framework/minigames.md)
- **Rationale:** Scalable AI difficulty ensures minigames are accessible to casual players while
  challenging experienced ones.
- **Verification:** Play 100 games against each AI difficulty; verify win rate ranges match
  configured targets.

## R-13.26.5d Board Piece Animation and Cascading

The engine **SHALL** animate piece movement, placement, and removal on the grid, with
gravity/cascading for falling-tile games that recursively checks for new matches after cascade
resolution. Win/loss evaluation **SHALL** run after all cascades resolve.

- **Derived from:** [F-13.26.5d](../../features/game-framework/minigames.md)
- **Rationale:** Animated cascading is core to match-3 gameplay and must resolve fully before
  evaluating win/loss conditions.
- **Verification:** Create a match-3 minigame; verify matched pieces are removed, remaining pieces
  fall, and new matches trigger recursively. Verify win/loss evaluation runs only after all cascades
  complete.

## R-13.26.6 Physics Toy Minigames

The engine **SHALL** provide a reusable physics sandbox template using the engine's rigid body
system with configurable gravity, constraints, and scoring rules. Analog input **SHALL** map to
physics forces with configurable sensitivity curves. Skill-check outcomes **SHALL** be deterministic
given identical input sequences for replay fairness.

- **Derived from:** [F-13.26.6](../../features/game-framework/minigames.md)
- **Rationale:** Physics-driven minigames (fishing, claw machines, ball games) require consistent
  physics behavior for fair skill expression.
- **Verification:** Create a fishing minigame; verify analog input controls rod tension smoothly.
  Record and replay an input sequence; verify identical physics outcome.

## R-13.26.7 Multiplayer Minigame Sessions

The engine **SHALL** support multiplayer minigame sessions with local (shared-screen, split- screen)
and networked play. Turn-based minigames **SHALL** use the turn manager for synchronization.
Real-time minigames **SHALL** use prediction and rollback. Network latency up to 150ms **SHALL NOT**
cause desynchronization in turn-based modes. Spectators **SHALL** be able to observe active minigame
sessions.

- **Derived from:** [F-13.26.7](../../features/game-framework/minigames.md)
- **Rationale:** Multiplayer minigames (card games against other players, competitive fishing) are a
  major engagement driver and require the same networking quality as the outer game.
- **Verification:** Play a networked card game with 150ms simulated latency; verify turn resolution
  is correct. Verify a spectator sees the same board state as players.

## R-13.26.8 Minigame Library and Discovery

The engine **SHALL** maintain a runtime registry of all available minigames with metadata (name,
category, player count, duration, difficulty, unlock status). Discovered minigames **SHALL** be
replayable from a collectible menu without returning to the world location. Per-minigame high
scores, completion counts, and achievement progress **SHALL** persist across sessions.

- **Derived from:** [F-13.26.8](../../features/game-framework/minigames.md)
- **Rationale:** A centralized minigame library increases replay value and enables collection
  mechanics (complete all minigames, beat all high scores).
- **Verification:** Discover a minigame in-world; verify it appears in the menu. Replay from the
  menu; verify the same minigame loads. Set a high score; restart the game; verify it persists.

## Non-Functional Requirements

### NFR-13.26.1 Minigame Session Isolation

Minigame ECS world partitions **SHALL** be fully isolated from the outer world partition. No
cross-partition component access **SHALL** be possible outside the typed result contract. Session
creation **SHALL** complete within 500ms and teardown within 200ms. Memory allocated by the minigame
partition **SHALL** be fully reclaimed on teardown with zero leakage.

- **Rationale:** Isolation prevents minigames from corrupting outer game state. Fast
  creation/teardown avoids perceptible loading delays. Zero leakage prevents memory growth from
  repeated minigame sessions.
- **Verification:** Create and destroy 100 minigame sessions in sequence. Measure memory before and
  after and verify zero net allocation growth. Attempt to access an outer-world component from
  within the minigame and verify it fails. Measure creation and teardown times.

### NFR-13.26.2 Timing Minigame Input Precision

Rhythm and timing minigame input latency from audio output to hit detection **SHALL** be under 16ms.
Input timestamp precision **SHALL** be sub-millisecond. Frame-rate variations **SHALL NOT** affect
hit detection window accuracy by more than 1ms.

- **Rationale:** Rhythm games require frame-precise input detection. Any latency or jitter in hit
  windows makes the game feel unfair and unresponsive.
- **Verification:** Play a rhythm sequence at 120 BPM. Measure audio-to-input detection latency and
  verify it is under 16ms. Vary frame rate between 30 and 120 fps and verify hit window accuracy
  remains within 1ms tolerance.
