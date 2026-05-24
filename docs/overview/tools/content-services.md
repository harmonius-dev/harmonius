# Content Services

Mod support, marketplace, and user-generated content.

## What it covers

- Mod SDK: APIs and tools for mod creation.
- Mod distribution: marketplace for sharing mods.
- Mod loading: plugin system and security sandboxing.
- Workshop integration: Steam Workshop and similar platforms.
- Asset marketplace: selling cosmetics and content.
- DLC and expansions: distributing additional content.
- User-generated content: player-created maps, skins, modes.
- Content moderation: reviewing and approving content.
- Creator programs: revenue sharing with content creators.
- Telemetry for content: tracking mod and DLC usage.

## Concepts

### Mod SDK and Distribution

Mod SDK provides APIs for modders: accessing game data, spawning entities, hooking events. Modders
create mod packages (folder with scripts and assets). Distribution platforms (Steam Workshop,
custom launcher) host mods. Players subscribe to mods; launcher downloads and loads them at startup.

### Security and Sandboxing

Untrusted mod code could compromise security. Sandboxing runs mods in isolated context: mods can't
access filesystem outside mod directory, can't crash game core. Signed mods from trusted creators
get higher privileges. Content filters remove malicious or offensive mods.

### Workshop Integration

Steam Workshop and similar platforms integrate mod publishing: modders upload mod package; players
subscribe. Subscribed mods automatically update. Workshop handles distribution and storage. Creators
earn revenue share from mod purchases.

### DLC and Expansions

DLC packages new content: maps, items, campaigns. DLC distributes via app store or in-game
marketplace. DLC metadata (name, description, price) appears in shop. Players purchase and download
DLC. DLC code gates content to owners: non-owners see locked UI, get error if accessing DLC content.

### Creator Programs and Revenue

Creator programs enable creators (YouTubers, streamers) to monetize: viewers purchase cosmetics, or
percentage of purchases goes to creator. Creator dashboards show revenue, viewers, performance
metrics. Tiered partnerships offer better terms for high-performing creators.

## How it fits

- See [cloud-and-deployment.md](./cloud-and-deployment.md) for distribution infrastructure.
- See [../core-runtime/authoring-runtime.md](../core-runtime/authoring-runtime.md) for
  plugin systems and hot reload.
- See [../game-framework/authoring-and-scripting.md](../game-framework/authoring-and-scripting.md)
  for scripting APIs.
