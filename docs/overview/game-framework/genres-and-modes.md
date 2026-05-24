# Genres and Modes

Game mode specializations, genre-specific systems, and gameplay variants.

## What it covers

- First-person shooters: gunplay, aim assist, killstreaks.
- Third-person action: combat combos, blocking, parrying.
- RPGs: character classes, skill trees, stat-based progression.
- Strategy: turn-based or real-time tactics, resource gathering.
- Puzzle games: logic challenges, spatial reasoning.
- Rhythm games: timing-based mechanics, score metrics.
- Roguelikes: procedural generation, permadeath, meta-progression.
- Multiplayer modes: deathmatch, team modes, cooperative.
- PvP and PvE: competitive and cooperative gameplay.
- Accessibility modes: difficulty assists, alternative control schemes.

## Concepts

### Genre-Specific Systems

Genre determines core gameplay loop: FPS loop (encounter enemies, aim, shoot, win); RPG loop
(explore, encounter, gain experience, level up); puzzle loop (analyze puzzle, attempt solution, win
or retry). Each genre specializes its systems: FPS emphasizes aim assist and hitreg; RPG emphasizes
stat progression. Core game loop remains consistent: challenge → overcome → progress.

### Multiplayer Modes

Deathmatch is FFA (free-for-all): last player standing wins. Team modes: two teams fight. Capture
the flag: teams capture opponent flag and bring to base. Cooperative modes: players team against
enemy AI. Progression systems track wins/losses, compute rankings (Elo, TrueSkill). Matchmaking
pairs players of similar skill.

### Permadeath and Roguelikes

Permadeath: character death ends the run (start fresh from beginning). Permadeath raises stakes:
every decision matters. Meta-progression unlocks persist across runs: defeat 10 enemies with axe,
unlock axe upgrade. Meta-progression motivates continued play despite permadeath frustration.

### Accessibility Modes

Difficulty assists reduce challenge: lower enemy damage, increase player damage, autoaim. Story
mode prioritizes narrative over combat. Assists enable less-skilled or disabled players to experience
content. Time assists extend or remove time limits. Motion sickness assists reduce camera movement
speed.

### Specialization Hooks

Genre systems hook into core engine: ability system drives RPG skills, physics enables destructible
environments, AI powers enemy behavior. Genre-specific UI: FPS HUD shows crosshair and ammo; RPG
HUD shows experience bar and inventory. Genre-specific audio: FPS emphasizes gunshot/impact sounds;
RPG emphasizes music and dialogue.

## How it fits

- See [gameplay-features.md](./gameplay-features.md) for ability and progression systems.
- See [authoring-and-scripting.md](./authoring-and-scripting.md) for mode-specific scripting.
- See [../networking/infrastructure-and-anti-cheat.md](../networking/infrastructure-and-anti-cheat.md)
  for ranked systems and matchmaking.
