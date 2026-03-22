---
name: warn-use-bun
enabled: true
event: bash
pattern: \b(npm|npx|pnpm|yarn|vite)\s+
action: warn
---

**Use bun instead.**

This project uses bun exclusively as the JS/TS runtime and package manager. Replace:

- `npm install` → `bun install`
- `npx` → `bunx`
- `npm run` → `bun run`
- `npm test` → `bun test`
