import CoreGraphics
import Foundation

enum HarmoniusLaunchOptions {
  static let snapshotModeArgument = "-HarmoniusSnapshotMode"
  static let snapshotMetalSize = CGSize(width: 1280, height: 720)

  static var isSnapshotMode: Bool {
    ProcessInfo.processInfo.arguments.contains(snapshotModeArgument)
  }
}
