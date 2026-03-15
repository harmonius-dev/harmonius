# User Stories: Shared Build Cache

## US-15.11.1 Eliminate Redundant Builds

**As a** developer, **I want** compiled assets to be uploaded to a shared cache keyed by
content hash so that other developers download cached results instead of rebuilding, **so
that** each unique asset version is built at most once across the entire organization.

## US-15.11.2 Fast New Developer Onboarding

**As a** DevOps engineer, **I want** a fresh repository clone to fetch all compiled assets,
shaders, and graph bytecode from the shared cache on first editor launch, **so that** new
team members start working in minutes instead of waiting hours for a full build.
