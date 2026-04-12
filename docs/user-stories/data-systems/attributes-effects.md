# Attributes and Effects User Stories

## Meters

| ID        | Persona                 |
|-----------|-------------------------|
| US-16.1.1 | game developer (P-15)   |
| US-16.1.2 | game designer (P-5)     |
| US-16.1.3 | engine developer (P-26) |

1. **US-16.1.1** — **As a** game developer (P-15), **I want** a generic bounded Meter primitive for
   health, stamina, mana, hunger, and reputation, **so that** I can model all bounded numeric
   resources without building a custom tracker per game.
2. **US-16.1.2** — **As a** game designer (P-5), **I want** to configure rising and falling
   thresholds on meters that fire events or apply effects on crossing, **so that** I can drive
   low-health warnings and reputation tier transitions from data.
3. **US-16.1.3** — **As an** engine developer (P-26), **I want** 1,000 active meters to evaluate
   within 0.5 ms per frame, **so that** meter updates never become the frame-budget bottleneck for
   large entity populations.

## Attributes

| ID        | Persona                 |
|-----------|-------------------------|
| US-16.1.4 | game designer (P-5)     |
| US-16.1.5 | engine developer (P-26) |

1. **US-16.1.4** — **As a** game designer (P-5), **I want** to author attribute schemas with typed
   fields and per-field min/max in the editor, **so that** I can define character stat blocks
   without writing code.
2. **US-16.1.5** — **As an** engine developer (P-26), **I want** 10,000 attribute reads to complete
   within 0.1 ms, **so that** gameplay systems can query stats dozens of times per entity per frame
   without perf impact.

## Modifiers

| ID        | Persona                 |
|-----------|-------------------------|
| US-16.1.6 | game designer (P-5)     |

1. **US-16.1.6** — **As a** game designer (P-5), **I want** layered modifier stacks combining flat,
   percent, and override operations in deterministic priority order, **so that** buffs, debuffs, and
   equipment bonuses compose predictably without custom aggregation code.

## Effects

| ID         | Persona                 |
|------------|-------------------------|
| US-16.1.7  | game designer (P-5)     |
| US-16.1.8  | game designer (P-5)     |
| US-16.1.9  | gamer (P-23)            |
| US-16.1.10 | engine developer (P-26) |

1. **US-16.1.7** — **As a** game designer (P-5), **I want** to author effects with instant,
   duration, periodic, and infinite types that apply modifiers to meters and attributes, **so that**
   I can create buffs, debuffs, and status effects from data.
2. **US-16.1.8** — **As a** game designer (P-5), **I want** to configure how multiple buff effects
   stack (additive versus highest-wins versus non-stacking), **so that** balance tuning is
   data-driven without touching engine code.
3. **US-16.1.9** — **As a** gamer (P-23), **I want** visible VFX indicators on my character for
   every active buff and debuff, **so that** I can tell at a glance which effects are influencing my
   stats.
4. **US-16.1.10** — **As an** engine developer (P-26), **I want** up to 64 concurrent effects per
   entity to process within 0.1 ms per tick, **so that** complex builds with many stacked effects do
   not blow the frame budget.

## Conditions

| ID         | Persona                 |
|------------|-------------------------|
| US-16.1.11 | game designer (P-5)     |
| US-16.1.12 | game developer (P-15)   |

1. **US-16.1.11** — **As a** game designer (P-5), **I want** to compose boolean condition expression
   trees (And, Or, Not) to gate effect application and threshold actions, **so that** I can build
   complex gating logic visually without writing code.
2. **US-16.1.12** — **As a** game developer (P-15), **I want** to subscribe to applied, ticked,
   expired, and removed effect events through the ECS event channel, **so that** UI, audio, and
   achievement systems can react to effect lifecycle without polling.
