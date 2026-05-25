import SwiftUI

struct ContentView: View {
  var body: some View {
    VStack(spacing: 12) {
      Text("Harmonius")
        .font(.title.bold())
      Text("Metal 4 triangle (MTL4CommandQueue)")
        .font(.subheadline)
        .foregroundStyle(.secondary)
      MetalView()
        #if os(macOS)
          .frame(minWidth: 960, minHeight: 540)
        #endif
    }
    .padding(24)
  }
}

#Preview {
  ContentView()
}
