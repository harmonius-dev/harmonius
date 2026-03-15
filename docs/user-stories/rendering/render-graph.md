# Render Graph User Stories

## US-2.2.1 Declarative Pass Authoring

**As a** graphics programmer, **I want** to declare render passes with typed resource I/O and
have the graph compiler automatically insert barriers, schedule queues, and alias memory,
**so that** I can add new rendering features without manually managing GPU synchronization.

## US-2.2.2 Quality Tier Scalability

**As a** designer, **I want** the render graph to automatically cull low-priority passes when
the frame budget is exceeded, **so that** the game maintains a stable frame rate across
hardware tiers without manual per-platform configuration.
