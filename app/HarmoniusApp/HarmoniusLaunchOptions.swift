import CoreGraphics
import Foundation

enum HarmoniusLaunchOptions {
  static let snapshotModeArgument = "-HarmoniusSnapshotMode"
  static let snapshotScale: CGFloat = 2
  static let snapshotMetalPixelSize = CGSize(width: 960, height: 540)
  static let snapshotMetalSize = CGSize(width: 480, height: 270)

  static var isSnapshotMode: Bool {
    ProcessInfo.processInfo.arguments.contains(snapshotModeArgument)
  }
}
