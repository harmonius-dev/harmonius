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
      name: "HarmoniusShaderResources",
      targets: ["HarmoniusShaderResources"]
    ),
    .library(
      name: "SlangReflection",
      targets: ["SlangReflection"]
    ),
    .library(
      name: "SwiftEmitter",
      targets: ["SwiftEmitter"]
    ),
  ],
  dependencies: [
    .package(
      url: "https://github.com/ajmcclary/swift-snapshot-testing.git",
      revision: "67ce8c189de56737485d6ab46a95a4f6f53b98a2"
    ),
    .package(
      url: "https://github.com/swiftlang/swift-syntax.git",
      exact: "603.0.1"
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
      name: "CSlang",
      path: "Sources/CSlang",
      pkgConfig: "shader-slang"
    ),
    .target(
      name: "SlangReflectionBridge",
      dependencies: [
        "CSlang"
      ],
      path: "Sources/SlangReflectionBridge",
      publicHeadersPath: "include"
    ),
    .target(
      name: "SlangReflection",
      dependencies: [
        "SlangReflectionBridge"
      ],
      path: "Sources/SlangReflection"
    ),
    .target(
      name: "SwiftEmitter",
      dependencies: [
        "SlangReflection",
        .product(
          name: "SwiftBasicFormat",
          package: "swift-syntax"
        ),
        .product(
          name: "SwiftSyntax",
          package: "swift-syntax"
        ),
        .product(
          name: "SwiftSyntaxBuilder",
          package: "swift-syntax"
        ),
      ],
      path: "Sources/SwiftEmitter"
    ),
    .target(
      name: "HarmoniusShaderResources",
      dependencies: [],
      path: "Sources/HarmoniusShaderResources",
      plugins: [
        .plugin(name: "HarmoniusShaderPlugin")
      ]
    ),
    .target(
      name: "HarmoniusRendering",
      dependencies: [
        "CCGLTF",
        "CMeshOptimizer",
        "HarmoniusShaderResources",
      ],
      path: "Sources/HarmoniusRendering",
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
      path: "Sources/HarmoniusAppCore",
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
      dependencies: [
        "SlangReflection",
        "SwiftEmitter",
      ],
      path: "Sources/HarmoniusShaderTool"
    ),
    .plugin(
      name: "HarmoniusShaderPlugin",
      capability: .buildTool(),
      path: "Plugins/HarmoniusShaderPlugin"
    ),
    .testTarget(
      name: "HarmoniusUnitTests",
      dependencies: [
        "HarmoniusRendering"
      ],
      path: "Tests/HarmoniusUnitTests"
    ),
    .testTarget(
      name: "HarmoniusRenderTests",
      dependencies: [
        "HarmoniusRendering",
        "HarmoniusShaderResources",
      ],
      path: "Tests/HarmoniusRenderTests",
      linkerSettings: [
        .linkedFramework("AppKit"),
        .linkedFramework("CoreGraphics"),
        .linkedFramework("Metal"),
      ]
    ),
    .testTarget(
      name: "SwiftEmitterTests",
      dependencies: [
        "SlangReflection",
        "SwiftEmitter",
      ],
      path: "Tests/SwiftEmitterTests"
    ),
    .testTarget(
      name: "HarmoniusAppiumTests",
      dependencies: [
        .product(
          name: "SnapshotTesting",
          package: "swift-snapshot-testing"
        ),
        .product(
          name: "WebDriver",
          package: "swift-webdriver"
        ),
      ],
      path: "Tests/HarmoniusAppiumTests",
      exclude: [
        "__Snapshots__"
      ],
      linkerSettings: [
        .linkedFramework("CoreGraphics"),
        .linkedFramework("ImageIO"),
      ]
    ),
  ],
  cxxLanguageStandard: .cxx2b
)
