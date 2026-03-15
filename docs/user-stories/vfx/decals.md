# User Stories — 11.2 Decals

## US-11.2.1 Leave Visible Marks on Every Surface During Combat
**As a** player, **I want** blood splatter, scorch marks, footprints, and damage decals to
persist on surfaces and characters after combat, **so that** the battlefield tells the story
of what happened and I get immediate visual feedback from every hit.

| Acceptance criteria | Features | Requirements |
|---|---|---|
| Deferred decals modifying G-buffer channels independently | F-11.2.1 | R-11.2.1 |
| Mesh decals for high-quality persistent markings | F-11.2.2 | R-11.2.2 |
| Atlas batching with LRU eviction under memory pressure | F-11.2.3 | R-11.2.3 |
| Priority stacking, blend modes, and lifecycle management | F-11.2.4 | R-11.2.4 |
| Procedural blood and damage decals from hit events | F-11.2.5 | R-11.2.5 |
| Surface-aware footprints and tire tracks | F-11.2.6 | R-11.2.6 |
| Max 2,048 active decals within 128 MB atlas budget | F-11.2.4 | R-X.13.2 |
