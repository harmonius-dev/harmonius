# User Stories -- 8.5 Session Management

## US-8.5.1 Reconnect Without Losing Progress

**As a** player (P-23), **I want** to reconnect to my active session after a brief Wi-Fi dropout and
resume exactly where I left off (position, buffs, party), **so that** transient network issues do
not force me to relog or lose combat progress.

## US-8.5.2 Play with Friends on Other Platforms

**As a** player (P-23), **I want** to form a party with friends on PC, PlayStation, and Xbox and
queue for dungeons together, **so that** platform choice does not prevent me from playing with my
friends.

## US-8.5.3 Deploy Headless Servers in Containers

**As a** DevOps engineer (P-16), **I want** to run the game server as a headless Docker container
without GPU or display dependencies, **so that** I can deploy hundreds of server instances on
standard cloud compute (AWS ECS, Kubernetes) at minimal per-instance cost.

## US-8.5.4 Monitor Server Health from Load Balancer

**As a** DevOps engineer (P-16), **I want** the headless server to expose an HTTP health check
endpoint reporting player count, tick rate, and CPU/memory usage, **so that** the load balancer can
route new players to healthy servers and drain unhealthy ones.

## US-8.5.5 Fair Matches Based on Skill

**As a** player (P-23), **I want** to be matched against opponents of similar skill level using a
Glicko-2 rating system that accounts for uncertainty, **so that** matches feel challenging but fair
rather than one-sided stomps.

## US-8.5.6 Short Queue Times During Peak Hours

**As a** player (P-23), **I want** to find a match within 60 seconds during peak hours even if the
skill window is slightly wider, **so that** I spend more time playing and less time waiting.

## US-8.5.7 Graceful Server Shutdown Without Disconnects

**As a** player (P-23), **I want** the server to save my progress and migrate me to a replacement
server before shutting down, **so that** I do not lose progress or get disconnected during a patch
deployment.

## US-8.5.8 Log In Through My Platform Account

**As a** player (P-23), **I want** to authenticate using my Steam, PlayStation Network, Xbox Live,
or Apple Game Center account with support for multi-factor authentication, **so that** I can log in
securely without creating a separate game account.

## US-8.5.9 Form Parties That Persist Across Zones and Servers

**As a** player (P-23), **I want** my party to persist across zone transitions and server migrations
with role designations and ready checks, **so that** my group stays together regardless of where we
travel in the world.

## US-8.5.10 Link My Platform Accounts for Cross-Platform Progression

**As a** player (P-23), **I want** to link my Steam, PlayStation, and Xbox accounts to a single game
account, **so that** my characters, progression, friends list, and achievements carry across every
platform I play on.

## US-8.5.11 Scale Server Fleet Automatically for Launch Day

**As a** server admin (P-22), **I want** automatic cluster provisioning and recycling that scales
capacity based on population in real time, **so that** login queues are short on launch day and
infrastructure costs decrease when population drops.

## US-8.5.12 Perform Rolling Restarts for Patches

**As a** server admin (P-22), **I want** the cluster manager to drain and migrate sessions during
rolling restarts, **so that** patches deploy without disconnecting any player.

## US-8.5.13 Manage Login Queue Priorities During Peak Load

**As a** server admin (P-22), **I want** a managed login queue with priority tiers (subscribers,
founders), position display, estimated wait time, and reconnection position preservation,
**so that** players receive a fair and transparent waiting experience when servers are at capacity.

## US-8.5.14 Configure Headless Server via Environment Variables

**As a** DevOps engineer (P-16), **I want** the headless server to accept port, tick rate, max
players, map, and game mode via command-line arguments and environment variables, **so that** I can
configure each container instance declaratively in my Kubernetes manifests.

## US-8.5.15 Receive Queue Pop Notification on Mobile While Backgrounded

**As a** player (P-23), **I want** a push notification when my login queue or matchmaking queue
position is reached while the app is backgrounded on mobile, **so that** I do not miss my turn to
play.

## US-8.5.16 Test Login Flow Under Thousands of Concurrent Connections

**As an** engine tester (P-27), **I want** load tests that simulate thousands of concurrent login
and authentication attempts, **so that** I can verify the authentication service handles peak login
surges without rejecting legitimate players.

## US-8.5.17 Validate Cross-Play Matchmaking Respects Platform Rules

**As an** engine tester (P-27), **I want** integration tests that verify cross-play matchmaking
respects platform certification rules and opt-out preferences, **so that** console certification is
not blocked by matchmaking bugs.

## US-8.5.18 Design Matchmaking Queue Parameters

**As a** designer (P-5), **I want** to configure matchmaking parameters (rating window, widening
speed, group composition rules, region preferences) without code changes, **so that** I can tune the
queue experience based on player feedback and population data.
