# OS Services

Clipboard, dialogs, IME, platform services, and console SDKs.

## What it covers

- Clipboard read/write (text and images) via async API.
- Native file/folder picker dialogs with file-type filters.
- System toast notifications and optional tray icon.
- Drag-and-drop with MIME type and file extension validation.
- Keyboard layout queries with dead-key handling and IME support for CJK text input.
- Achievements and leaderboards bridged to platform services (Steam, PlayStation, Xbox, Apple,
  Google Play).
- Cloud saves and entitlements from platform stores.
- Account linking for cross-platform progression.
- Anti-cheat integration (Easy Anti-Cheat, Warhawk, etc.).
- Server-side player data storage and cross-platform account service.

## Concepts

### Async Platform APIs

Clipboard operations, file dialogs, and platform queries are exposed as async functions. Even if the
underlying OS call is synchronous, the engine treats it as async (delegated to a background thread)
to prevent blocking the game loop. All operations have timeout and structured error handling.

### Platform Services

Steam, PlayStation, Xbox Live, Apple Game Center, Google Play, Nintendo, and EOS provide
achievements, leaderboards, cloud storage, and matchmaking. The engine unifies these under
trait-based abstractions, with platform-specific backends compiled in per target. At runtime on PC,
the engine auto-detects the active launcher and uses the appropriate backend.

### Cross-Platform Progression

A server-side canonical record stores player progression. Platform-specific identities (Steam ID,
PSN, Xbox Live ID) link to a single game account. Cross-buy allows a player to claim a game on
multiple platforms and retain their save and progress.

## How it fits

- See [crash-and-telemetry](./crash-and-telemetry.md) for platform-native crash reporting and
  telemetry integration.
- See [networking](../networking/infrastructure-and-anti-cheat.md) for online features and
  anti-cheat.
- Integrates with [game-framework](../game-framework/world-management.md) for cloud save triggers
  and progression persistence.
