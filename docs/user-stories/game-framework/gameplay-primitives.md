# User Stories — 13.1 Gameplay Primitives

## F-13.1.1 Game Mode State Machine

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-13.1.1.1 | developer (P-15) | I want to create a new game mode asset that defines win/loss conditions, scoring rules, and phase transitions so that I can configure how a match plays out through the plugin API |  | F-13.1.1 | R-13.1.1 |
| US-13.1.1.2 | designer (P-5) | I want to set player count limits, team composition rules, and respawn policies per game mode asset in the visual editor so that each mode enforces its own participation constraints automatically |  | F-13.1.1 | R-13.1.1 |
| US-13.1.1.3 | designer (P-5) | I want to nest sub-mode state machines inside a parent mode (e.g., boss phases inside a raid) so that encounters have their own rules while inheriting the session's top-level constraints |  | F-13.1.1 | R-13.1.1 |
| US-13.1.1.4 | developer (P-15) | I want to transition between game modes (lobby, match, post-match) at runtime via logic graph events so that the game flow progresses through phases automatically without hard-coded transitions |  | F-13.1.1 | R-13.1.1 |
| US-13.1.1.5 | engine developer (P-26) | I want to register a new game mode type via the plugin system with declared dependencies so that third-party modes integrate into the state machine without modifying the engine core |  | F-13.1.1 | R-13.1.1 |
| US-13.1.1.6 | engine tester (P-27) | I want to attempt every invalid state transition in a game mode graph and verify all are rejected so that the state machine enforces its declared topology |  | F-13.1.1 | R-13.1.1 |
| US-13.1.1.7 | engine tester (P-27) | I want to cycle through all game mode transitions 100 times and verify zero memory growth so that mode switching does not cause memory leaks in long play sessions |  | F-13.1.1 | R-13.1.1 |
| US-13.1.1.8 | engine tester (P-27) | I want to validate that every mode declared in a game mode asset has at least one valid transition path from the initial state so that no orphaned modes exist that players can never reach |  | F-13.1.1 | R-13.1.1 |

## F-13.1.2 Game State Manager

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-13.1.2.1 | designer (P-5) | I want to configure the sequence of top-level states (main menu, character select, loading, in-game, paused, shutdown) with transition triggers so that each state automatically loads the correct resources, UI layers, and input contexts |  | F-13.1.2 | R-13.1.2 |
| US-13.1.2.2 | developer (P-15) | I want to the game state manager to synchronize state authoritatively between client and server so that instanced content like dungeons keeps all participants in the same phase |  | F-13.1.2 | R-13.1.2 |
| US-13.1.2.3 | developer (P-15) | I want to the state manager to handle disconnected and reconnecting states gracefully so that players resume their session at the correct game phase after a network interruption |  | F-13.1.2 | R-13.1.2 |
| US-13.1.2.4 | engine developer (P-26) | I want to implement the state lifecycle so transitions trigger resource loading/unloading, UI layer swaps, and input context changes atomically so that no partial state is ever visible to the player or game systems |  | F-13.1.2 | R-13.1.2 |
| US-13.1.2.5 | engine tester (P-27) | I want to trigger rapid state transitions (pause/unpause, menu open/close) at 60 Hz and verify no state corruption so that fast player inputs cannot put the game into an invalid state |  | F-13.1.2 | R-13.1.2 |
| US-13.1.2.6 | engine tester (P-27) | I want to verify that exiting any game state releases all resources loaded by that state's enter logic so that state transitions never accumulate stale resources |  | F-13.1.2 | R-13.1.2 |

## F-13.1.3 Player Controller

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-13.1.3.1 | designer (P-5) | I want to define input context transitions (exploration, combat, mounted, vehicle, cinematic) in the visual editor so that the player controller automatically remaps inputs when context changes |  | F-13.1.3 | R-13.1.3 |
| US-13.1.3.2 | designer (P-5) | I want to select a targeting mode (tab-target, action-target, soft-lock) per game mode asset so that the controller's targeting behavior matches the game's combat style |  | F-13.1.3 | R-13.1.3 |
| US-13.1.3.3 | developer (P-15) | I want to the player controller to mediate between the input action system and the controlled pawn entity so that input actions translate to gameplay actions through a single well-defined pathway |  | F-13.1.3 | R-13.1.3 |
| US-13.1.3.4 | developer (P-15) | I want to the controller to queue ability inputs during the global cooldown so that the next ability fires immediately when GCD expires without requiring frame-perfect timing |  | F-13.1.3 | R-13.1.3 |
| US-13.1.3.5 | engine developer (P-26) | I want to the controller to manage camera ownership and UI focus arbitration so that only one system drives the camera at any time and focus conflicts are resolved deterministically |  | F-13.1.3 | R-13.1.3 |
| US-13.1.3.6 | engine tester (P-27) | I want to exercise every input context transition path and verify no input leaks between contexts so that exploration inputs never fire during cinematics and vice versa |  | F-13.1.3 | R-13.1.3 |

## F-13.1.4 Pawn and Character System

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-13.1.4.1 | designer (P-5) | I want to set up a character's stats, equipment slots, faction, and level data as component data in the visual editor so that character templates are authored without code |  | F-13.1.4 | R-13.1.4 |
| US-13.1.4.2 | developer (P-15) | I want to transfer possession between pawns (mind control, spectator mode, vehicle mounting) by reassigning the controller entity so that gameplay mechanics requiring body-swapping work through a single API |  | F-13.1.4 | R-13.1.4 |
| US-13.1.4.3 | engine developer (P-26) | I want to the pawn and character system to handle thousands of NPC characters coexisting with player characters in ECS so that MMO-scale populations are supported without architectural changes |  | F-13.1.4 | R-13.1.4 |
| US-13.1.4.4 | engine tester (P-27) | I want to verify that possessing a pawn, then releasing it, returns both the pawn and the original character to their correct states so that possession cycles never corrupt entity data |  | F-13.1.4 | R-13.1.4 |

## F-13.1.5 Gameplay Ability System

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-13.1.5.1 | designer (P-5) | I want to create an ability asset in the visual editor defining activation conditions, costs, targeting rules, cast time, and channeling behavior so that I can design abilities without writing code |  | F-13.1.5 | R-13.1.5 |
| US-13.1.5.2 | designer (P-5) | I want to assign abilities to shared cooldown groups and configure per-ability and global cooldowns so that ability pacing matches the intended combat rhythm |  | F-13.1.5 | R-13.1.5 |
| US-13.1.5.3 | developer (P-15) | I want to the ability system to validate all activation requests on the server (costs, cooldowns, conditions) so that cheating clients cannot bypass ability constraints |  | F-13.1.5 | R-13.1.5 |
| US-13.1.5.4 | engine developer (P-26) | I want to the ability framework to be fully data-driven with no hard-coded ability types so that new abilities are added as assets without engine code changes |  | F-13.1.5 | R-13.1.5 |
| US-13.1.5.5 | engine tester (P-27) | I want to simulate 200ms latency and verify that queued abilities fire in correct order with no dropped inputs so that ability queueing works reliably for remote players |  | F-13.1.5 | R-13.1.5 |
| US-13.1.5.6 | engine tester (P-27) | I want to send rapid ability activation requests during cooldown and verify all are rejected so that the cooldown system cannot be exploited |  | F-13.1.5 | R-13.1.5 |

## F-13.1.6 Gameplay Effect System

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-13.1.6.1 | designer (P-5) | I want to create an effect asset defining stat modifications, duration, stacking rules, and tick intervals so that buffs, debuffs, and DoTs are configured visually |  | F-13.1.6 | R-13.1.6 |
| US-13.1.6.2 | designer (P-5) | I want to set stacking rules (refresh, stack count, diminishing returns) per effect type so that multiple applications of the same effect behave as intended by game design |  | F-13.1.6 | R-13.1.6 |
| US-13.1.6.3 | developer (P-15) | I want to choose between snapshot (freeze stats at application) and dynamic (re-evaluate stats each tick) per effect so that each effect type uses the appropriate stat binding mode |  | F-13.1.6 | R-13.1.6 |
| US-13.1.6.4 | engine developer (P-26) | I want to the effect system to resolve conflicting CCs using priority ordering and diminishing returns so that competing stuns, roots, and silences produce deterministic outcomes |  | F-13.1.6 | R-13.1.6 |
| US-13.1.6.5 | engine tester (P-27) | I want to apply a 10-second effect and verify it expires within 1 frame of the target duration across varying frame rates so that effect durations are frame-rate independent |  | F-13.1.6 | R-13.1.6 |
| US-13.1.6.6 | engine tester (P-27) | I want to apply an effect beyond its max stack count and verify additional stacks are rejected or refresh duration per config so that stacking rules cannot be bypassed |  | F-13.1.6 | R-13.1.6 |
| US-13.1.6.7 | engine tester (P-27) | I want to kill an entity with active effects and verify all effects are cleanly removed with no dangling references so that dead entities do not leak effect state |  | F-13.1.6 | R-13.1.6 |

## F-13.1.7 Damage Model

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-13.1.7.1 | designer (P-5) | I want to configure the damage pipeline stages (attack power scaling, armor mitigation, crit, block, parry, dodge, absorb shields) per damage school so that combat math matches the game's design without code changes |  | F-13.1.7 | R-13.1.7 |
| US-13.1.7.2 | designer (P-5) | I want to create damage schools (physical, fire, frost, shadow) with per-school resistances so that elemental combat interactions work as designed |  | F-13.1.7 | R-13.1.7 |
| US-13.1.7.3 | developer (P-15) | I want to the damage model to produce damage events consumed by the health system, combat log, floating combat text, and network replication so that all downstream systems react to a single authoritative damage computation |  | F-13.1.7 | R-13.1.7 |
| US-13.1.7.4 | engine developer (P-26) | I want to allow plugins to insert custom stages into the damage pipeline so that games with unique combat math can extend the model without forking the engine |  | F-13.1.7 | R-13.1.7 |
| US-13.1.7.5 | engine tester (P-27) | I want to compute damage for identical inputs 1000 times and verify identical outputs so that the damage pipeline is fully deterministic |  | F-13.1.7 | R-13.1.7 |
| US-13.1.7.6 | engine tester (P-27) | I want to test damage with each mitigation stage (armor, resist, block, parry, dodge, absorb) active individually and combined so that no stage is skipped or double-applied |  | F-13.1.7 | R-13.1.7 |

## F-13.1.8 Death, Respawn, and Encounter Reset

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-13.1.8.1 | designer (P-5) | I want to place respawn points (graveyards, checkpoints, instance entrances) and configure respawn timers with repeated-death debuffs so that death penalties match the intended difficulty curve |  | F-13.1.8 | R-13.1.8 |
| US-13.1.8.2 | developer (P-15) | I want to the death system to handle ragdoll activation, death state, spirit/ghost phase, and resurrection by other players as state machine transitions so that death flows through well-defined phases |  | F-13.1.8 | R-13.1.8 |
| US-13.1.8.3 | engine developer (P-26) | I want to the respawn system to select the nearest valid respawn point considering faction, phase, and instance boundaries so that players always respawn at a sensible location |  | F-13.1.8 | R-13.1.8 |
| US-13.1.8.4 | engine tester (P-27) | I want to trigger a wipe during each boss phase and verify that all encounter state (boss HP, add spawns, phase counters) resets to initial values so that encounter resets are complete and consistent |  | F-13.1.8 | R-13.1.8 |
| US-13.1.8.5 | engine tester (P-27) | I want to verify respawn timers fire within 100ms of the configured duration across varying frame rates so that respawn timing is precise and frame-rate independent |  | F-13.1.8 | R-13.1.8 |
| US-13.1.8.6 | engine tester (P-27) | I want to die repeatedly and verify that death debuffs stack and expire according to configured rules so that repeated-death penalties work as designed |  | F-13.1.8 | R-13.1.8 |

## F-13.1.9 Modular System Registration

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-13.1.9.1 | designer (P-5) | I want to enable or disable gameplay systems (combat, inventory, quests, AI) in the project settings so that the editor UI only shows tools relevant to my game and irrelevant systems are excluded |  | F-13.1.9 | R-13.1.9 |
| US-13.1.9.2 | designer (P-5) | I want to see which systems depend on which others when enabling a system so that I understand why enabling combat also enables physics and animation |  | F-13.1.9 | R-13.1.9 |
| US-13.1.9.3 | developer (P-15) | I want to register gameplay systems through the plugin system with declared dependencies so that each system is independently toggleable per project |  | F-13.1.9 | R-13.1.9 |
| US-13.1.9.4 | engine developer (P-26) | I want to disabled systems to be excluded from compilation, reducing binary size and runtime memory so that minimal projects do not carry the weight of unused systems |  | F-13.1.9 | R-13.1.9 |
| US-13.1.9.5 | engine tester (P-27) | I want to enable a system with deep dependency chains and verify all transitive dependencies are automatically enabled so that the dependency resolver handles multi-level chains correctly |  | F-13.1.9 | R-13.1.9 |
| US-13.1.9.6 | engine tester (P-27) | I want to disable a system and verify that all its panels, inspectors, and tools are hidden from the editor UI so that disabled systems leave no UI artifacts |  | F-13.1.9 | R-13.1.9 |

## F-13.1.10 Rust Plugin API for Developers

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-13.1.10.1 | developer (P-15) | I want to use the plugin template generator to scaffold a new Rust plugin project with build configuration and boilerplate so that I can start extending the engine without manual project setup |  | F-13.1.10 | R-13.1.10 |
| US-13.1.10.2 | developer (P-15) | I want to register custom ECS components, systems, and queries through the plugin API so that my game-specific logic runs within the engine's ECS schedule |  | F-13.1.10 | R-13.1.10 |
| US-13.1.10.3 | developer (P-15) | I want to add custom inspector panels, viewport overlays, and menu items through the plugin API so that my game's editor tools integrate seamlessly with the engine editor |  | F-13.1.10 | R-13.1.10 |
| US-13.1.10.4 | engine developer (P-26) | I want to design a C-compatible ABI layer for plugins so the loader validates ABI compatibility before loading so that plugins built against different engine versions fail safely |  | F-13.1.10 | R-13.1.10 |
| US-13.1.10.5 | engine developer (P-26) | I want to implement plugin loading using dlopen on macOS/Linux and LoadLibrary on Windows so that plugins work consistently across all supported platforms |  | F-13.1.10 | R-13.1.10 |
| US-13.1.10.6 | engine tester (P-27) | I want to load a plugin compiled against a different engine version and verify it is rejected with a clear error message so that ABI mismatches never cause crashes |  | F-13.1.10 | R-13.1.10 |
| US-13.1.10.7 | engine tester (P-27) | I want to load and unload a plugin 100 times and verify zero memory leaks and no dangling component registrations so that plugin lifecycle management is robust |  | F-13.1.10 | R-13.1.10 |
