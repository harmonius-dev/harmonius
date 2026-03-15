# User Stories -- 14.5 Platform Services

## US-14.5.1 Earn Achievements on Every Storefront

**As a** player (P-23), **I want** achievements to unlock on Steam, PlayStation, and Xbox when I
reach MMO milestones (first raid clear, max level), **so that** my accomplishments are visible on my
platform profile regardless of where I play.

## US-14.5.2 Compete on Leaderboards Across Platforms

**As a** player (P-23), **I want** to submit and view ranked scores (DPS rankings, PvP ladders,
speedrun timers) through platform leaderboard APIs, **so that** I can compare my performance with
friends and the global community.

## US-14.5.3 Show Friends What I Am Playing

**As a** player (P-23), **I want** my platform status to show my current zone, party size, and
activity type, **so that** friends on my platform's social UI can see what I am doing and join me.

## US-14.5.4 Talk to My Party Through Platform Voice

**As a** player (P-23), **I want** the game to bridge with my platform's party voice system (Steam
Voice, PlayStation, Xbox Game Chat 2), **so that** voice chat works automatically with my party
without manual channel configuration.

## US-14.5.5 Keep My Settings Across Machines via Cloud Save

**As a** player (P-23), **I want** key bindings, UI layouts, and add-on configurations saved to
platform cloud storage, **so that** my preferences are preserved when I switch machines or
reinstall.

## US-14.5.6 Access Only the Content I Own

**As a** player (P-23), **I want** the game to verify my owned entitlements (base game, expansions,
DLC, subscription) at login and periodically during play, **so that** content access is gated
correctly without requiring a restart for new purchases.

## US-14.5.7 Write Platform Service Code Once

**As a** game developer (P-15), **I want** unified abstractions for achievements, leaderboards, rich
presence, voice, cloud storage, and entitlements, **so that** gameplay code integrates with platform
services without branching per platform.

## US-14.5.8 Handle Deferred Achievement Unlocks Gracefully

**As an** engine developer (P-26), **I want** achievement unlocks to be queued and retried when the
platform service is temporarily unavailable (offline play, network hiccup), **so that** no
achievement is lost due to transient connectivity issues.

## US-14.5.9 Integrate Platform-Specific Authentication SDKs

**As an** engine developer (P-26), **I want** each platform's identity provider integration (Steam,
PSN, Xbox Live, Apple Game Center) encapsulated behind the unified service layer, **so that**
first-party authentication SDK requirements are met without leaking platform details to gameplay
code.

## US-14.5.10 Pass Console Certification Requirements

**As a** game developer (P-15), **I want** the engine to enforce console certification compliance
(suspend/resume, system UI overlays, controller disconnect prompts, safe-area rendering,
accessibility mandates), **so that** the game passes PlayStation and Xbox certification without
platform-specific fixes in gameplay code.

## US-14.5.11 Gate Unavailable DLC Content in the Storefront

**As a** designer (P-5), **I want** unavailable content to be hidden or clearly marked with a
purchase redirect to the platform storefront, **so that** players understand what they can access
and certification requirements for DLC presentation are met.

## US-14.5.12 Respect Platform Cloud Storage Quotas

**As a** DevOps engineer (P-16), **I want** the cloud save system to respect per-platform file size
and total quota limits, **so that** saves do not fail or get rejected during console certification
testing.

## US-14.5.13 Verify Achievements Unlock on All Platforms

**As an** engine tester (P-27), **I want** automated tests that trigger achievement unlock,
leaderboard submission, and rich presence updates on each platform API, verifying correct results,
**so that** platform service regressions are caught before certification.

## US-14.5.14 Verify Console Suspend and Resume Behavior

**As an** engine tester (P-27), **I want** tests that simulate console suspend, resume, and memory
pressure events, verifying the engine saves state, releases resources within platform deadlines, and
restores cleanly, **so that** certification-blocking issues are caught in automated testing.

## US-14.5.15 Throttle Leaderboard Queries to Avoid Rate Limits

**As an** engine developer (P-26), **I want** leaderboard query results cached and requests
throttled to respect platform-imposed rate limits, **so that** frequent UI refreshes do not cause
API throttling or certification failures.

## US-14.5.16 Verify Cross-Play Voice Integration

**As an** engine tester (P-27), **I want** tests that verify platform voice chat bridging works for
cross-play parties using Vivox as fallback when platform voice is unavailable, **so that** voice
communication works reliably in all party configurations.

## US-14.5.17 Keep My Settings When Switching PCs

**As a** player (P-23), **I want** my graphics settings, keybindings, and audio volumes to sync to
the cloud and restore automatically when I log in on a different PC, **so that** I don't have to
reconfigure everything when I play on my laptop vs my desktop.

## US-14.5.18 Resolve Settings Conflict After Offline Play

**As a** player (P-23), **I want** a clear dialog showing which settings differ when my local and
cloud preferences diverge after offline play on two machines, **so that** I can choose which version
to keep rather than losing my carefully tuned keybindings.

## US-14.5.19 Recover From Corrupted Settings

**As a** game developer (P-15), **I want** the preferences API to use atomic writes and return
defaults for missing keys, **so that** a crash mid-save never corrupts the preferences file and
missing entries don't cause errors.

## US-14.5.20 Manage Downloaded Content Cache

**As a** player (P-23), **I want** to see how much disk space downloaded DLC, mods, and streaming
content are using and clear specific categories from the settings menu, **so that** I can free disk
space without uninstalling the game.

## US-14.5.21 Cache Evicts Automatically When Full

**As an** engine developer (P-26), **I want** the local cache manager to evict least-recently-used
entries by category priority when the cache exceeds its configured maximum, **so that** the game
never fails a download due to disk-full errors.

## US-14.5.22 Reduce Rebuild Times With Local Cache

**As a** game developer (P-15), **I want** compiled assets, shader bytecode, and logic graph
bytecode cached locally in `.harmonius/cache/` with hash-based invalidation, **so that** incremental
builds take seconds instead of minutes for unchanged assets.

## US-14.5.23 Fall Back to Network Cache When Local Misses

**As a** DevOps engineer (P-16), **I want** the build system to check the local cache first, then
the shared S3-backed network cache, before doing a full rebuild, **so that** team members benefit
from each other's cached builds without duplicating compilation.

## US-14.5.24 Eliminate Shader Compilation Stutter

**As a** player (P-23), **I want** the game to cache compiled pipeline states to disk so that shader
compilation stutter only happens once per material, **so that** I don't experience hitches every
time I enter a new area or encounter a new effect.

## US-14.5.25 Ship Pre-Built PSO Cache With the Game

**As a** DevOps engineer (P-16), **I want** the cook process to generate a pre-built PSO cache from
a reference playthrough that ships with the game, **so that** players experience zero shader stutter
on first launch with no warmup needed.

## US-14.5.26 Verify PSO Cache Invalidates on Driver Update

**As an** engine tester (P-27), **I want** tests that verify PSO cache entries are invalidated when
the GPU driver version changes, **so that** players don't load stale bytecode that could cause
rendering artifacts after updating drivers.

## US-14.5.27 Clean Up Temp Files After a Crash

**As a** player (P-23), **I want** the engine to clean up leftover temp files from previous crashed
sessions on startup, **so that** temporary data doesn't accumulate and waste disk space over time.

## US-14.5.28 Temp Files Never Contain Important Data

**As an** engine developer (P-26), **I want** the temp file system to guarantee that all temp data
is recreatable from non-temp sources via RAII handles that auto-delete on drop, **so that** deleting
the entire temp directory never causes data loss.
