# User Stories -- Minigame Framework (13.26)

## US-13.26.1 Self-Contained Minigame Sessions
**As a** player, **I want** to enter minigames (card games, fishing, puzzles) as isolated
sessions within the game world that read my inventory and grant rewards on completion through
a defined contract, **so that** minigames feel integrated but cannot corrupt my main game
state.

## US-13.26.2 Reusable Minigame Templates
**As a** designer, **I want** reusable templates for timing/rhythm, board/grid, and physics
toy minigames with data-driven rules, AI opponents, and multiple presentation modes, **so
that** I can create diverse minigames through visual authoring without custom code per
minigame.

## US-13.26.3 Set Up a Grid-Based Card or Board Game
**As a** designer, **I want** an NxM grid engine with configurable cell types and turn-based
or real-time modes, **so that** I can create card games, match-3 puzzles, and chess-like
tactics as minigames without reimplementing grid logic.

## US-13.26.4 Define Custom Match Rules for Grid Games
**As a** designer, **I want** to author custom match detection rules (3-in-a-row, poker hands,
custom patterns) as logic graph assets, **so that** I can create unique puzzle mechanics
beyond the built-in algorithms.

## US-13.26.5 Play Against AI Opponents at Different Difficulties
**As a** player, **I want** AI opponents in board minigames with easy, medium, and hard
difficulty levels, **so that** I can enjoy challenging single-player board game sessions.

## US-13.26.6 Watch Tiles Cascade and Chain in Match-3 Games
**As a** player, **I want** matched tiles to animate away, remaining tiles to fall, and new
matches to trigger chain reactions, **so that** match-3 minigames feel satisfying and
dynamic.
