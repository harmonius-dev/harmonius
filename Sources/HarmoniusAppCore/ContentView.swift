import SwiftUI

private let contentBackground = Color(red: 0.08, green: 0.09, blue: 0.12)

public struct ContentView: View {
  private let configuration: HarmoniusUIConfiguration

  public init(configuration: HarmoniusUIConfiguration = HarmoniusUIConfiguration()) {
    self.configuration = configuration
  }

  public var body: some View {
    #if os(macOS)
      if let size = configuration.macOSWindowPointSize {
        metalContent
          .frame(width: CGFloat(size.width), height: CGFloat(size.height))
      } else {
        let defaultSize = HarmoniusUIConfiguration.defaultMacOSWindowPointSize
        metalContent
          .frame(
            minWidth: CGFloat(defaultSize.width),
            minHeight: CGFloat(defaultSize.height)
          )
      }
    #else
      metalContent
    #endif
  }

  private var metalContent: some View {
    MetalView()
      .background(contentBackground)
      .accessibilityElement(children: .contain)
      .accessibilityIdentifier("snapshot-content")
  }
}

#Preview {
  ContentView()
}
