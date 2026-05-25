# XCUITest snapshot rendering

See [testing.md](testing.md) for the full testing guide (unit tests, UI tests, and CI).

This document remains as a focused reference for the UI snapshot workflow.

## Snapshot test

[HarmoniusRenderTests.swift](../app/HarmoniusApp/HarmoniusRenderTests.swift) launches `Harmonius`,
waits for `metal-view-ready`, screenshots the window, and compares against a reference PNG via
[SnapshotImageTesting.swift](../app/HarmoniusApp/SnapshotImageTesting.swift).

Reference images live under `app/HarmoniusApp/__Snapshots__/HarmoniusRenderTests/`.

### Record or refresh baselines

```bash
SNAPSHOT_RECORD=1 xcodebuild test \
  -project Harmonius.xcodeproj \
  -scheme HarmoniusApp \
  -only-testing:HarmoniusUITests \
  -destination "platform=macOS" \
  -derivedDataPath build/xcodegen
```

Commit the updated PNG under `__Snapshots__/`.

## CUA verification

After building with `xcodebuild` (see [AGENTS.md](../AGENTS.md)), an agent with the CUA driver MCP
can confirm rendering outside the test runner:

1. Launch `build/xcodegen/Build/Products/Debug/HarmoniusApp.app`
   (`bundle_id`: `dev.harmonius.App`).
2. Poll accessibility until `metal-view-ready` is visible.
3. Screenshot the app window and confirm the triangle is visible.

This step is local-only and is not part of CI.
