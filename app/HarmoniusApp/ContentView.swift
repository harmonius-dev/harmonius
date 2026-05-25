import SwiftUI

private let contentBackground = Color(red: 0.08, green: 0.09, blue: 0.12)

struct ContentView: View {
  @State private var metalViewReady = false

  var body: some View {
    VStack(spacing: 12) {
      Text("Harmonius")
        .font(.title.bold())
      Text("Metal 4 triangle (MTL4CommandQueue)")
        .font(.subheadline)
        .foregroundStyle(.secondary)
      MetalView(isReady: $metalViewReady)
        #if os(macOS)
          .frame(minWidth: 960, minHeight: 540)
        #endif
      Text("ready")
        .accessibilityIdentifier("metal-view-ready")
        .accessibilityHidden(!metalViewReady)
        .frame(width: 0, height: 0)
        .opacity(0)
    }
    .padding(24)
    .background(contentBackground)
    .accessibilityElement(children: .contain)
    .accessibilityIdentifier("snapshot-content")
  }
}

#Preview {
  ContentView()
}
