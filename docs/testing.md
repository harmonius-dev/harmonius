# Testing

Harmonius uses CMake/CTest for Swift Testing execution. XcodeGen still verifies
the app bundle, but tests are not generated as Xcode-owned bundles.

## Test Targets

| Target | Framework | Scope |
| ------ | --------- | ----- |
| `HarmoniusUnitTests` | Swift Testing | Geometry and pure rendering data |
| `HarmoniusRenderTests` | Swift Testing + SnapshotTesting | Real Metal texture snapshots |

CMake builds production artifacts, Swift Testing executables, and the
Point-Free SnapshotTesting library used by render snapshots.

## Prerequisites

- macOS 26 + Xcode 26 (Metal 4, Swift 6.3)
- [XcodeGen](https://github.com/yonaskolb/XcodeGen) (`brew install xcodegen`)
- Ninja (`brew install ninja`)

## Run locally

Configure and build CMake, then run CTest.

```bash
cmake --preset macos-debug
cmake --build --preset macos-debug
ctest --test-dir build/macos --output-on-failure
```

Verify the Xcode app bundle separately:

```bash
xcodegen generate
xcodebuild build \
  -project Harmonius.xcodeproj \
  -scheme HarmoniusApp \
  -destination "platform=macOS" \
  -derivedDataPath build/xcodegen
```

## Unit tests

Unit tests are colocated with source under `app/HarmoniusRendering/`. Files
ending in `Tests.swift` use Swift Testing (`import Testing`, `@Test`, and
`#expect`).

Current coverage:

1. `TriangleVertexLayout.maxFramesInFlight`
2. `TriangleGeometry.frameData()` vertex colors
3. Vertex positions on the expected circle radius
4. Equilateral triangle side lengths

Add a new `@Test` function in a `*Tests.swift` file next to the code under
test. Public API under test must be marked `public` in the CMake-built module.

## Render Snapshot Test

[HarmoniusRenderTests.swift](../app/HarmoniusApp/HarmoniusRenderTests.swift)
uses Swift Testing and
[swift-snapshot-testing](https://github.com/pointfreeco/swift-snapshot-testing).

1. Create a real `MTLDevice`.
2. Load the Slang-built `default.metallib`.
3. Render the triangle into a real offscreen `MTLTexture`.
4. Convert that texture to an `NSImage`.
5. Call `assertSnapshot(of:as:named:)` at precision `0.98`.

Reference PNGs live under `app/HarmoniusApp/__Snapshots__/HarmoniusRenderTests/`.
SnapshotTesting names files `{testFunction}.{named}.png`.

### Record Or Refresh Baselines

```bash
SNAPSHOT_RECORD=1 ctest --test-dir build/macos --output-on-failure
```

Commit the updated PNG under `__Snapshots__/`.

## CI

The workflow in [.github/workflows/ci.yml](../.github/workflows/ci.yml) runs on every pull request
and `main` push:

1. `format` selects Xcode 26 and lints Swift files with `swift-format` on `macos-26`.
2. `macos-tests` builds CMake targets, runs CTest, and builds the Xcode app.
3. `deploy-ios` archives `HarmoniusApp`, exports an IPA, and uploads it to App Store Connect
   on successful `main` pushes from `macos-26`.
4. `deploy-macos` archives `HarmoniusApp`, exports a Mac App Store package, uploads it to App Store
   Connect, and waits for processing on successful `main` pushes from `macos-26`.

CI caches the vcpkg installed tree across repeated runs. Release exports upload
as `ios-release-ipa` and `macos-release-pkg`.

The macOS release job expects these GitHub Actions secrets:

| Secret | Purpose |
| ------ | ------- |
| `APPLE_CERTIFICATE_P12_BASE64` | App signing certificate bundle |
| `APPLE_CERTIFICATE_PASSPHRASE` | Distribution certificate passphrase |
| `APPLE_DISTRIBUTION_IDENTITY` | App signing identity |
| `APPLE_INSTALLER_CERTIFICATE_P12_BASE64` | Installer certificate bundle |
| `APPLE_INSTALLER_CERTIFICATE_PASSPHRASE` | Installer certificate passphrase |
| `APPLE_INSTALLER_DISTRIBUTION_IDENTITY` | Installer signing identity selector |
| `APPLE_TEAM_ID` | Apple Developer Program team |
| `ASC_ISSUER_ID` | App Store Connect API issuer |
| `ASC_KEY_ID` | App Store Connect API key |
| `ASC_KEY_P8_BASE64` | App Store Connect API private key |
| `MACOS_PROVISIONING_PROFILE_BASE64` | Mac App Store provisioning profile |
| `MACOS_PROVISIONING_PROFILE_NAME` | Mac App Store profile name |

## App icon plan

The selected winner is the middleground controller mark from the final user-selected reference
image. It is a clean, borderless gamepad/controller silhouette split into three organic pieces by
thick white negative-space channels. The filled sections are desaturated sage green across the upper
bridge, muted warm orange/clay in the left grip, and dark charcoal in the right grip.

This completes the image-selection decision. The app icon source of truth is the saved Icon Composer
package at [`AppIcon.icon`](../app/Resources/AppIcon.icon). The PDF bridge was removed.

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
2. Open `app/AssetSources/AppIcon.icon` in Apple Icon Composer.
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

## CMake artifact paths

If CMake output layout changes, update [project.yml](../project.yml):

| Artifact | Default path |
| -------- | ------------ |
| HarmoniusApp bundle | `build/macos/app/HarmoniusApp` |
| HarmoniusRendering archive | `build/macos/app/libHarmoniusRendering.a` |
| Swift module (import path) | `build/macos/app/` |
