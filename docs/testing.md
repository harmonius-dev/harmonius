# Testing

Harmonius uses SwiftPM and Swift Testing for unit, render, and Appium UI tests.
XcodeGen verifies Apple app bundles and provides app artifacts for Appium.

## Test Targets

| Target | Framework | Scope |
| --- | --- | --- |
| `HarmoniusUnitTests` | Swift Testing | Geometry and pure render data |
| `SwiftEmitterTests` | Swift Testing | Slang reflection model to Swift output |
| `HarmoniusRenderTests` | Swift Testing | Metal texture validation |
| `HarmoniusAppiumTests` | Swift Testing + Swift WebDriver | UI snapshots |

## Prerequisites

- macOS 26 + Xcode 26 for Apple app and Metal coverage.
- `swift-format`, `jq`, `pkg-config`, XcodeGen, and Appium.
- vcpkg submodule initialized by `scripts/dev.sh bootstrap <platform>`.
- Appium macOS and iOS drivers from `scripts/dev.sh appium-bootstrap`.

## Local Commands

```bash
./scripts/dev.sh test-unit
./scripts/dev.sh test-codegen
./scripts/dev.sh test-render
./scripts/dev.sh test-ui-macos
./scripts/dev.sh test-ui-ios-simulator
./scripts/dev.sh test-ui-record
./scripts/dev.sh test
```

Use `./scripts/dev.sh full-check` before opening a pull request. The full check
runs no-JS, no-XCTest, line-length, JSON sorting, Swift formatting, package
graph, build, unit tests, shader code-generation tests, render tests, and
`git diff --check`.

## Unit Tests

Unit tests live under `Tests/HarmoniusUnitTests/`. Files use Swift Testing with
`import Testing`, `@Test`, and `#expect`.

Current coverage:

1. `TriangleVertexLayout.maxFramesInFlight`.
2. `TriangleGeometry.frameData()` primary colors.
3. Vertex positions on the expected `240` radius circle.
4. Equilateral triangle side lengths.

## Render Texture Test

[HarmoniusRenderTests.swift](../Tests/HarmoniusRenderTests/HarmoniusRenderTests.swift)
uses Swift Testing only.

1. Create a real `MTLDevice`.
2. Load the SwiftPM-generated `default.metallib`.
3. Render the triangle into a real offscreen `MTLTexture`.
4. Read texture pixels directly.
5. Assert texture size, pixel format, nonblank output, and expected samples.

Render tests must not create, record, or compare screenshots.

## Appium UI Snapshots

Appium UI tests are the only screenshot snapshot tests. Test code must stay in
Swift Testing and use `swift-webdriver`; do not add JavaScript or TypeScript
harnesses.

Baselines live under:

```text
Tests/HarmoniusAppiumTests/__Snapshots__/
```

Record or refresh baselines with:

```bash
./scripts/dev.sh test-ui-record-macos
./scripts/dev.sh test-ui-record-ios-simulator
```

1. macOS snapshots resize the app window to `960x540` points.
2. macOS snapshot locations include only the host display DPI scale.
3. iOS simulator snapshot locations include only the simulator device/runtime.
4. iOS screenshots are captured at the native app size and must show the rendered
   triangle.
5. Screenshots are never scaled before recording or comparison.
6. Both flows assert `snapshot-content` and `metal-view`, capture a screenshot,
   compare it with the baseline, and always delete the Appium session.

## CI

The workflow in [.github/workflows/ci.yml](../.github/workflows/ci.yml) runs on
pull requests, `main` pushes, and manual dispatch.

1. `quality` runs package graph checks and `scripts/dev.sh full-check`.
2. `apple-tests` builds the macOS bundle and runs SwiftPM tests.
3. `apple-tests` runs Appium UI snapshot tests for macOS and iOS simulator.
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
