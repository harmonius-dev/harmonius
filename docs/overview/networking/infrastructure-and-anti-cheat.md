# Infrastructure and Anti-Cheat

Server hosting, matchmaking, and cheat prevention.

## What it covers

- Server deployment: cloud hosting, bare metal, containerization.
- Matchmaking: pairing compatible players into matches.
- Skill rating systems: ranking players (ELO, trueskill).
- Ranked and casual modes: different queuing for competitive and social play.
- Anti-cheat systems: detecting and preventing cheating.
- Cheat detection: server-side validation of client actions.
- Input validation: verifying inputs are within valid range.
- Behavioral detection: identifying suspicious play patterns.
- Ban systems: temporary and permanent account suspension.
- Reporting and review: investigating player reports.

## Concepts

### Deployment Architecture

Server hosting options range from cloud providers (Amazon, Google) to bare-metal dedicated servers.
Cloud hosting provides flexibility and scaling. Containerization (via Docker) simplifies deployment
and scaling. Load balancers distribute clients across multiple servers. Auto-scaling adds servers
during high load, removes when demand drops.

### Matchmaking

Matchmaking systems find compatible players for teams or matches. Random matchmaking pairs any
players; skill-based matchmaking pairs similar skill levels. Ranking systems (ELO, TrueSkill) track
player skill. Ranked queues use skill ranking; casual queues use random. Party systems group friends
into matches.

### Anti-Cheat Validation

Server validates all client actions: hitscans verify shot geometry; movement validates speed is
achievable. Input validation rejects inputs outside valid range (acceleration too high, position
teleport too far). Behavioral detection identifies suspicious patterns: impossible reaction times,
100% accuracy, wall-hack behavior (looking through walls). Ban systems suspend accounts, temporarily
or permanently. Reports from players trigger investigation and potential action.

### Cheat Detection Methods

Client-side anti-cheat monitors process memory, detects injected code. Server-side validation
catches client exploits: client can't cheat speed or position because server validates. Replay
review examines replays of flagged players, identifying impossible play.

## How it fits

- See [transport-and-replication.md](./transport-and-replication.md) for network transmission
  validation.
- See [session-and-prediction.md](./session-and-prediction.md) for input and state validation.
- See [communication-and-replay.md](./communication-and-replay.md) for replay review and
  evidence.
