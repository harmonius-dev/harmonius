# Testing

Harmonius uses SwiftPM for unit, render, snapshot, and Appium UI tests.
XcodeGen verifies Apple app bundles and provides app artifacts for Appium.

## Test Targets

| Target | Framework | Scope |
| --- | --- | --- |
| `HarmoniusUnitTests` | Swift Testing | Geometry and pure render data |
| `HarmoniusRenderTests` | Swift Testing + SnapshotTesting | Metal snapshots |
| `HarmoniusAppiumTests` | XCTest + Swift WebDriver | Appium smoke UI |
| `SwiftEmitterTests` | Swift Testing | Slang reflection model to Swift output |

## Prerequisites

- macOS 26 + Xcode 26 for Apple app and Metal coverage.
- `swift-format`, `jq`, `pkg-config`, XcodeGen, and Appium.
- vcpkg submodule initialized by `scripts/dev.sh bootstrap <platform>`.
- Appium `mac2` and `xcuitest` drivers from `scripts/dev.sh appium-bootstrap`.

## Local Commands

```bash
./scripts/dev.sh test-unit
./scripts/dev.sh test-codegen
./scripts/dev.sh test-render
./scripts/dev.sh test-render-record
./scripts/dev.sh test-ui-macos
./scripts/dev.sh test-ui-ios-simulator
./scripts/dev.sh test
```

Use `./scripts/dev.sh full-check` before opening a pull request. The full check
runs no-JS, line-length, JSON sorting, Swift formatting, package graph, build,
unit tests, shader code-generation tests, render tests, and `git diff --check`.

## Unit Tests

Unit tests live under `Tests/HarmoniusUnitTests/`. Files use Swift Testing with
`import Testing`, `@Test`, and `#expect`.

Current coverage:

1. `TriangleVertexLayout.maxFramesInFlight`.
2. `TriangleGeometry.frameData()` primary colors.
3. Vertex positions on the expected `240` radius circle.
4. Equilateral triangle side lengths.

## Render Snapshot Test

[HarmoniusRenderTests.swift](../Tests/HarmoniusRenderTests/HarmoniusRenderTests.swift)
uses Swift Testing and SnapshotTesting.

1. Create a real `MTLDevice`.
2. Load the SwiftPM-generated `default.metallib`.
3. Render the triangle into a real offscreen `MTLTexture`.
4. Convert that texture to an `NSImage`.
5. Compare the PNG at snapshot precision `0.98`.

Reference PNGs live under:

```text
Tests/HarmoniusRenderTests/__Snapshots__/HarmoniusRenderTests/
```

Snapshot dimensions stay explicit: render `960x540`, compare `480x270`.

## Record Or Refresh Baselines

```bash
./scripts/dev.sh test-render-record
```

Commit the updated PNG under the `__Snapshots__/` directory.

## Appium UI Tests

Appium replaces the Xcode UI test runner. Test code must stay in Swift and use
`swift-webdriver`; do not add JavaScript or TypeScript harnesses.

1. `test-ui-macos` builds the macOS bundle, starts Appium, and uses `mac2`.
2. `test-ui-ios-simulator` builds the simulator bundle and uses `xcuitest`.
3. Both flows assert a stable SwiftUI accessibility identifier.
4. Both flows capture a screenshot and always delete the Appium session.

## CI

The workflow in [.github/workflows/ci.yml](../.github/workflows/ci.yml) runs on
pull requests, `main` pushes, and manual dispatch.

1. `quality` runs package graph checks and `scripts/dev.sh full-check`.
2. `apple-tests` builds the macOS bundle and runs SwiftPM tests.
3. `apple-tests` runs Appium smoke tests for macOS and iOS simulator.
4. Release jobs archive iOS and macOS apps with `scripts/dev.sh archive`.

CI caches vcpkg installed trees across repeated runs. Release exports upload as
`ios-release-ipa` and `macos-release-pkg`.

The macOS release job expects these GitHub Actions secrets:

| Secret | Purpose |
| --- | --- |
| `APPLE_CERTIFICATE_P12_BASE64` | App certificate bundle |
| `APPLE_CERTIFICATE_PASSPHRASE` | App certificate passphrase |
| `APPLE_DISTRIBUTION_IDENTITY` | App signing identity |
| `APPLE_INSTALLER_DISTRIBUTION_IDENTITY` | Installer identity |
| `APPLE_TEAM_ID` | Apple Developer team |
| `ASC_ISSUER_ID` | App Store Connect issuer |
| `ASC_KEY_ID` | App Store Connect key |
| `ASC_KEY_P8_BASE64` | App Store Connect private key |
| `MACOS_PROVISIONING_PROFILE_BASE64` | Mac App Store profile |
| `MACOS_PROVISIONING_PROFILE_NAME` | Mac App Store profile name |

## App Icon Plan

The selected winner is the middleground controller mark from the final user-selected reference
image. It is a clean, borderless gamepad/controller silhouette split into three organic pieces by
thick white negative-space channels. The filled sections are desaturated sage green across the upper
bridge, muted warm orange/clay in the left grip, and dark charcoal in the right grip.

This completes the image-selection decision. The app icon source of truth is the saved Icon
Composer package at [`AppIcon.icon`](../Sources/HarmoniusApp/Resources/AppIcon.icon). The PDF
bridge was removed.

Implementation notes:

1. Use the final selected raster/reference image as the source of truth.
2. Vectorize into clean paths that preserve the exact silhouette, broad controller grips,
   soft upper bridge, large phase-aligned negative-space split channels, three-piece
   layout, rounded corners, and simple fill regions.
3. Remove any background or border. The final vector is only the controller silhouette
   pieces, with transparent outside/background and transparent negative-space gaps.
4. Preserve a WCAG-aware contrast relationship. Charcoal must remain clearly separated
   from the lighter sections; sage and clay stay desaturated but distinct, with enough
   lightness separation for small icon sizes.
5. Keep path count low and avoid texture, gradients, buttons, sticks, symbols,
   code marks, play icons, cubes, stones, and metallic effects.
6. Use Icon Composer to place the artwork into the app icon. Let the system/Liquid
   Glass background, lighting, and platform variants compose there instead of baking a
   border or background into the logo.
7. XcodeGen includes `AppIcon.icon` as an iOS resource outside `Assets.xcassets`.
8. The app icon build setting remains `AppIcon`, the filename without `.icon`.
9. Do not commit generated `AppIcon.appiconset` PNGs or flattened preview PNGs.

Icon Composer source:

1. Use macOS Sequoia 15.3 or newer.
2. Open `Sources/HarmoniusApp/Resources/AppIcon.icon` in Apple Icon Composer.
3. Save the package with File -> Save. Do not export app icon PNG sizes as source.
4. Keep `AppIcon.icon` next to `Assets.xcassets`, not inside the asset catalog.
5. Ensure the app target includes `AppIcon.icon` and the app icon name is `AppIcon`.
6. Keep the imported controller mark borderless with a transparent outside and gaps.
7. Use a light warm bone/ivory background so sage and clay remain the identity colors.
8. Keep approved translucency on with only a subtle neutral shadow for depth.
9. Keep source artwork tight; use Icon Composer layer scale for final icon padding.

Asset catalog:

1. Keep `Assets.xcassets` for non-icon assets such as `LaunchBackground`.
2. Do not generate app icon PNGs into `Assets.xcassets`.
