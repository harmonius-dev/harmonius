# User Stories — 4.6 Destruction & Fracture

## F-4.6.1 Voronoi Fracture Generation

**US-4.6.1a** As a designer, I want to generate fracture patterns at build time using
Voronoi decomposition with random, impact-directed, and artist-guided seeding so that
destruction looks varied and controllable without runtime mesh generation cost.

## F-4.6.2 Pre-Fractured Mesh Authoring

**US-4.6.2a** As a designer, I want to import pre-fractured meshes from DCC tools so that
hero destruction objects (castle walls, towers, bridges) break exactly the way I intend.

**US-4.6.2b** As a gameplay programmer, I want fracture assets to store fragment geometry,
connectivity, and joint thresholds in a single file so that the content pipeline handles
destruction data without special-case loading code.

## F-4.6.3 Runtime Fracture and Activation

**US-4.6.3a** As a player, I want destructible walls to shatter when hit hard enough so
that combat and environmental interaction feel dynamic and impactful.

**US-4.6.3b** As a gameplay programmer, I want fracture activation to be budgeted per frame
so that large-scale destruction events (siege scenarios, explosions) do not cause frame-rate
hitches.

## F-4.6.4 Progressive Damage Model

**US-4.6.4a** As a player, I want to see cracks and deformation appear on damaged objects
before they fully break so that I can gauge how close a wall or structure is to collapse.

**US-4.6.4b** As a designer, I want to configure multiple visual damage stages with
threshold values so that destruction has readable, telegraphed progression.

## F-4.6.5 Stress Propagation and Structural Collapse

**US-4.6.5a** As a player, I want a building to cascade-collapse when I destroy its load-
bearing pillars so that structural destruction feels realistic and strategic.

**US-4.6.5b** As a designer, I want to mark anchor points (grounded supports) on structures
so that the engine automatically computes which fragments collapse when supports are
removed.

## F-4.6.6 Debris Simulation and Lifecycle

**US-4.6.6a** As a gameplay programmer, I want debris entities to despawn after a
configurable lifetime and enforce a global debris cap so that destruction-heavy scenes
stay within memory and performance budgets.

**US-4.6.6b** As a player, I want debris fragments to tumble and settle realistically after
a structure breaks, then fade out naturally so that destruction aftermath looks believable
without permanent clutter.

## F-4.6.7 Debris Pooling and LOD

**US-4.6.7a** As a gameplay programmer, I want debris entities to be recycled from an object
pool so that rapid destruction sequences avoid allocation-driven frame hitches.

**US-4.6.7b** As a QA engineer, I want to verify that distant debris loses its `RigidBody`
and `Collider` components and that pooling reduces allocation count by at least 80% so that
LOD and pooling are working as specified.
