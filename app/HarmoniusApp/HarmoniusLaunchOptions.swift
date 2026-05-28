import CoreGraphics
import Foundation

#if os(macOS)
  import AppKit
#endif

enum HarmoniusLaunchOptions {
  static let snapshotModeArgument = "-HarmoniusSnapshotMode"
  static let snapshotMetalPixelSize = CGSize(width: 1920, height: 1080)

  static var isSnapshotMode: Bool {
    ProcessInfo.processInfo.arguments.contains(snapshotModeArgument)
  }

  #if os(macOS)
    static var snapshotMetalSize: CGSize {
      snapshotMetalSize(backingScaleFactor: NSScreen.main?.backingScaleFactor ?? 1)
    }

    static func snapshotMetalSize(backingScaleFactor scale: CGFloat) -> CGSize {
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
