import CoreGraphics
import Foundation

enum HarmoniusLaunchOptions {
  static let snapshotModeArgument = "-HarmoniusSnapshotMode"
  static let snapshotScale: CGFloat = 1
  static let snapshotMetalPixelSize = CGSize(width: 1280, height: 720)

  static var isSnapshotMode: Bool {
    ProcessInfo.processInfo.arguments.contains(snapshotModeArgument)
  }

  #if os(macOS)
    static var snapshotMetalSize: CGSize {
      snapshotMetalSize(scale: snapshotScale)
    }

    static func snapshotMetalSize(scale: CGFloat) -> CGSize {
      let normalizedScale = max(scale, 1)
      return CGSize(
        width: snapshotMetalPixelSize.width / normalizedScale,
        height: snapshotMetalPixelSize.height / normalizedScale
      )
    }
  #else
    static let snapshotMetalSize = snapshotMetalPixelSize
  #endif
}
