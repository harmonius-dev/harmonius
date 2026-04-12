# Event Logs User Stories

## Bounded Log Primitive

| ID        | Persona                 |
|-----------|-------------------------|
| US-17.1.1 | game developer (P-15)   |
| US-17.1.2 | engine developer (P-26) |

1. **US-17.1.1** — **As a** game developer (P-15), **I want** a generic bounded EventLog primitive
   with configurable capacity and automatic eviction of oldest entries, **so that** I can model NPC
   memory, gossip, threat tables, and combat logs with one type.
2. **US-17.1.2** — **As an** engine developer (P-26), **I want** per-entry memory bounded to 256
   bytes, **so that** thousands of NPC memory logs fit within mobile memory budgets.

## Decay

| ID        | Persona                 |
|-----------|-------------------------|
| US-17.1.3 | game designer (P-5)     |
| US-17.1.4 | engine developer (P-26) |

1. **US-17.1.3** — **As a** game designer (P-5), **I want** log entries to decay accuracy over
   configurable curves, **so that** NPC memories of old events fade realistically instead of
   vanishing at a fixed timeout.
2. **US-17.1.4** — **As an** engine developer (P-26), **I want** the decay pass across 1,000 logs to
   complete within 2 ms, **so that** per-tick simulation budgets are never blown by memory updates.

## Propagation

| ID        | Persona                 |
|-----------|-------------------------|
| US-17.1.5 | game designer (P-5)     |
| US-17.1.6 | engine developer (P-26) |

1. **US-17.1.5** — **As a** game designer (P-5), **I want** log entries to propagate between nearby
   entities with configurable accuracy loss per hop, **so that** gossip rumors degrade as they
   spread between NPCs.
2. **US-17.1.6** — **As an** engine developer (P-26), **I want** propagation from one source to 50
   neighbors to finish within 0.5 ms, **so that** crowd-scale rumor spread fits in per-tick budget.

## Threshold Triggers

| ID        | Persona             |
|-----------|---------------------|
| US-17.1.7 | game designer (P-5) |

1. **US-17.1.7** — **As a** game designer (P-5), **I want** NPCs to become alert after witnessing 3
   hostile events in 60 seconds, **so that** repeated aggression can escalate NPC behavior through a
   data-driven threshold.

## Persistence

| ID        | Persona      |
|-----------|--------------|
| US-17.1.8 | gamer (P-23) |

1. **US-17.1.8** — **As a** gamer (P-23), **I want** NPC memory and reputation to persist across
   save/load with bit-identical state, **so that** my long-term consequences survive quitting the
   game.
