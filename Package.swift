// swift-tools-version: 6.2

import PackageDescription

let package = Package(
  name: "Harmonius",
  platforms: [
    .iOS(.v26),
    .macOS(.v26),
  ],
  products: [
    .library(
      name: "HarmoniusAppCore",
      targets: ["HarmoniusAppCore"]
    ),
    .library(
      name: "HarmoniusRendering",
      targets: ["HarmoniusRendering"]
    ),
    .library(
      name: "HarmoniusShaders",
      targets: ["HarmoniusShaders"]
    ),
  ],
  dependencies: [
    .package(
      url: "https://github.com/pointfreeco/swift-snapshot-testing.git",
      exact: "1.19.2"
    ),
    .package(
      url: "https://github.com/thebrowsercompany/swift-webdriver.git",
      revision: "eb79abde86888ef086ca56c0c7ccd09a03757c2d"
    ),
  ],
  targets: [
    .systemLibrary(
      name: "CCGLTF",
      path: "Sources/CCGLTF",
      pkgConfig: "cgltf"
    ),
    .systemLibrary(
      name: "CMeshOptimizer",
      path: "Sources/CMeshOptimizer",
      pkgConfig: "meshoptimizer"
    ),
    .systemLibrary(
      name: "HarmoniusShaderTypes",
      path: "app/Shaders",
      pkgConfig: "hlslpp"
    ),
    .target(
      name: "HarmoniusShaders",
      dependencies: [],
      path: "Sources/HarmoniusShaders",
      plugins: [
        .plugin(name: "HarmoniusShaderPlugin")
      ]
    ),
    .target(
      name: "HarmoniusRendering",
      dependencies: [
        "CCGLTF",
        "CMeshOptimizer",
        "HarmoniusShaderTypes",
        "HarmoniusShaders",
      ],
      path: "app/HarmoniusRendering",
      swiftSettings: [
        .interoperabilityMode(.Cxx)
      ],
      linkerSettings: [
        .linkedFramework("CoreGraphics"),
        .linkedFramework("Metal"),
        .linkedFramework("MetalKit"),
        .linkedFramework("QuartzCore"),
      ]
    ),
    .target(
      name: "HarmoniusAppCore",
      dependencies: [
        "HarmoniusRendering"
      ],
      path: "app/HarmoniusApp",
      exclude: [
        "HarmoniusApp.entitlements",
        "HarmoniusApp.swift",
      ],
      swiftSettings: [
        .interoperabilityMode(.Cxx)
      ],
      linkerSettings: [
        .linkedFramework("AppKit", .when(platforms: [.macOS])),
        .linkedFramework("CoreGraphics"),
        .linkedFramework("Metal"),
        .linkedFramework("MetalKit"),
        .linkedFramework("QuartzCore"),
        .linkedFramework("SwiftUI"),
        .linkedFramework("UIKit", .when(platforms: [.iOS])),
      ]
    ),
    .executableTarget(
      name: "HarmoniusShaderTool",
      path: "Sources/HarmoniusShaderTool"
    ),
    .plugin(
      name: "HarmoniusShaderPlugin",
      capability: .buildTool(),
      dependencies: [
        "HarmoniusShaderTool"
      ],
      path: "Plugins/HarmoniusShaderPlugin"
    ),
    .testTarget(
      name: "HarmoniusUnitTests",
      dependencies: [
        "HarmoniusRendering"
      ],
      path: "Tests/HarmoniusUnitTests",
      swiftSettings: [
        .interoperabilityMode(.Cxx)
      ]
    ),
    .testTarget(
      name: "HarmoniusRenderTests",
      dependencies: [
        "HarmoniusRendering",
        "HarmoniusShaders",
        .product(
          name: "SnapshotTesting",
          package: "swift-snapshot-testing"
        ),
      ],
      path: "Tests/HarmoniusRenderTests",
      exclude: [
        "__Snapshots__"
      ],
      swiftSettings: [
        .interoperabilityMode(.Cxx)
      ],
      linkerSettings: [
        .linkedFramework("AppKit"),
        .linkedFramework("CoreGraphics"),
        .linkedFramework("Metal"),
      ]
    ),
    .testTarget(
      name: "HarmoniusAppiumTests",
      dependencies: [
        .product(
          name: "WebDriver",
          package: "swift-webdriver"
        )
      ],
      path: "Tests/HarmoniusAppiumTests"
    ),
  ],
  cxxLanguageStandard: .cxx2b
)
