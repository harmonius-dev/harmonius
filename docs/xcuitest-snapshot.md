# XCUITest snapshot rendering

See [testing.md](testing.md) for the UI snapshot workflow, baseline paths, and record commands.

## CUA verification

After building with `xcodebuild` (see [AGENTS.md](../AGENTS.md)), an agent with the CUA driver MCP
can confirm rendering outside the test runner:

1. Launch `build/xcodegen/Build/Products/Debug/HarmoniusApp.app`
   (`bundle_id`: `dev.harmonius.App`).
2. Poll accessibility until `metal-view` is visible.
3. Screenshot the app window and confirm the triangle is visible.

This step is local-only and is not part of CI.
