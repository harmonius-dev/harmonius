import HarmoniusAppCore
import SwiftUI

@main
public struct HarmoniusApp: App {
  private let uiConfiguration = HarmoniusUIConfiguration()

  public init() {}

  public var body: some Scene {
    WindowGroup {
      ContentView(configuration: uiConfiguration)
    }
    #if os(macOS)
      .defaultSize(
        width: CGFloat(macOSDefaultWindowPointSize.width),
        height: CGFloat(macOSDefaultWindowPointSize.height)
      )
    #endif
  }

  private var macOSDefaultWindowPointSize: HarmoniusPointSize {
    uiConfiguration.macOSWindowPointSize
      ?? HarmoniusUIConfiguration.defaultMacOSWindowPointSize
  }
}
