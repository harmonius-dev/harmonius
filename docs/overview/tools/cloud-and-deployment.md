# Cloud and Deployment

Cloud infrastructure, CI/CD, and production deployment.

## What it covers

- Build pipelines: automating game builds for platforms.
- Testing automation: running tests on every change.
- Artifact management: storing and versioning builds.
- Deployment automation: shipping builds to players.
- A/B testing: comparing game versions with different settings.
- Analytics collection: gathering telemetry data.
- Crash reporting: collecting and analyzing crash data.
- Remote configuration: updating game settings without rebuilding.
- Content delivery networks: distributing builds geographically.
- Cloud saves: synchronizing player data across devices.

## Concepts

### Build Pipelines

CI/CD pipelines automate building: code change triggers build (compile, link). Build succeeds or
fails; failures block merge. Successful builds run tests. Passing tests create artifacts (binaries).
Artifacts deploy to staging environment for QA. QA approval deploys to production. This automation
catches errors early and enables rapid iteration.

### Testing Automation

Automated tests (unit, integration, smoke) run on every change. Failing tests block merge. Tests
exercise gameplay systems: ability cooldowns, damage calculations, level progression. Deterministic
replay tests ensure physics and AI produce consistent results.

### Analytics and Telemetry

Telemetry collects gameplay data: player actions, performance metrics, crashes. Analytics dashboard
visualizes data: playtime per level, ability usage, crash rates. A/B testing compares two game
versions: group A plays normal; group B plays with modified difficulty. Metrics show which version
players prefer.

### Remote Configuration

Game downloads configuration on startup: balance parameters (enemy damage), feature flags (enable
new ability). Changing configuration redeployment unnecessary; update config and restart game.
Feature flags enable gradual rollout: enable feature for 10% of players, gradually increase to 100%.

### Deployment and CDN

Builds deploy to staging servers first (QA environment). After QA approval, deploy to production
servers. Content delivery networks (CDNs) distribute builds geographically: players download from
nearest CDN for low latency. Cloud saves synchronize player data to cloud servers, enabling
multi-device play.

## How it fits

- See [profiling-and-collaboration.md](./profiling-and-collaboration.md) for performance
  metrics and monitoring.
- See [content-services.md](./content-services.md) for content distribution and services.
- See [../networking/infrastructure-and-anti-cheat.md](../networking/infrastructure-and-anti-cheat.md)
  for server deployment and matchmaking.
