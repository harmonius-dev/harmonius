import CoreGraphics
import Foundation

#if os(macOS)
  import AppKit
#endif

enum HarmoniusLaunchOptions {
  static let snapshotModeArgument = "-HarmoniusSnapshotMode"
  static let snapshotMetalPixelSize = CGSize(width: 960, height: 540)

  static var isSnapshotMode: Bool {
    ProcessInfo.processInfo.arguments.contains(snapshotModeArgument)
  }

  #if os(macOS)
    static var snapshotMetalSize: CGSize {
      let scale = max(NSScreen.main?.backingScaleFactor ?? 1, 1)
      return CGSize(
        width: snapshotMetalPixelSize.width / scale,
        height: snapshotMetalPixelSize.height / scale
      )
    }
  #else
    static let snapshotMetalSize = snapshotMetalPixelSize
  #endif
}
