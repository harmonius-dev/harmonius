# User Stories -- 8.2 State Replication

## US-8.2.1 See Only Relevant Entities

**As a** game developer (P-15), **I want** each client to receive replication updates only for
entities within its area of interest, **so that** per-client bandwidth stays within budget even in
zones with hundreds of thousands of entities.

## US-8.2.2 Deploy Server Updates Without Kicking Players

**As a** server admin (P-22), **I want** component schema versioning to allow clients on the
previous build to decode replicated data from the updated server, **so that** I can perform rolling
server updates without disconnecting any player.

## US-8.2.3 Minimize Bandwidth for State Updates

**As an** engine developer (P-26), **I want** delta-compressed property replication using per-client
baseline tracking and XOR-based differencing, **so that** only changed fields are transmitted and
MMO-scale worlds with thousands of updating entities fit within each client's bandwidth budget.

## US-8.2.4 Control What Each Player Sees Based on Game Rules

**As a** game developer (P-15), **I want** per-property replication conditions (owner-only,
team-only, public) and distance-based detail tiers, **so that** sensitive data like enemy cooldowns
is hidden from opponents and distant entities consume less bandwidth.

## US-8.2.5 Prioritize Critical Entities Under Bandwidth Pressure

**As an** engine developer (P-26), **I want** a priority scheduler that allocates the per-connection
bandwidth budget based on relevancy score, entity type, and staleness, **so that** the player's
target and nearby hostiles always update first while low-priority distant NPCs fill remaining
budget.

## US-8.2.6 Reduce Server Load from Idle Entities

**As a** server admin (P-22), **I want** entities that have not changed for a configurable period to
enter dormancy and consume zero replication bandwidth, **so that** the 60-80% of idle entities in a
typical MMO zone do not waste server CPU or network resources.

## US-8.2.7 Experience Smooth Gameplay in Large-Scale PvP

**As a** player (P-23), **I want** replication to prioritize nearby players and hostile targets even
during a 500-player guild siege, **so that** I can see and react to threats around me without lag
even when the server is handling massive entity counts.

## US-8.2.8 Keep Mobile Players in Sync on Limited Bandwidth

**As a** player (P-23), **I want** the server to adapt replication precision and frequency for my
mobile connection (500 Kbps), **so that** I can play on cellular data without excessive data usage
or desyncs.

## US-8.2.9 Support Mobile Clients Running Older App Versions

**As a** server admin (P-22), **I want** schema versioning to handle mobile clients running older
app versions due to app store update delays, **so that** all players can connect regardless of
whether they have updated to the latest build.

## US-8.2.10 Verify Replication Correctness Under High Entity Counts

**As an** engine tester (P-27), **I want** automated tests that spawn thousands of entities with
varying update rates and verify delta-compressed replication delivers correct state to each client,
**so that** I can catch replication bugs before they reach production.

## US-8.2.11 Verify AOI Filtering Does Not Leak Sensitive Data

**As an** engine tester (P-27), **I want** test scenarios that verify owner-only and team-only
properties are never sent to unauthorized clients, **so that** competitive integrity is maintained
and data leaks are caught during development.

## US-8.2.12 Always See Party Members Regardless of Distance

**As a** player (P-23), **I want** party members, guild members in my zone, and raid bosses to
always be replicated regardless of distance, **so that** I can track my group and encounter targets
on the minimap and in the UI at all times.
