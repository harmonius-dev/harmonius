import CoreGraphics
import Foundation

enum HarmoniusLaunchOptions {
  static let snapshotModeArgument = "-HarmoniusSnapshotMode"

  static var isSnapshotMode: Bool {
    ProcessInfo.processInfo.arguments.contains(snapshotModeArgument)
  }

  static let snapshotMetalSize = CGSize(width: 960, height: 540)
}
