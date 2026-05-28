import SwiftUI

private let contentBackground = Color(red: 0.08, green: 0.09, blue: 0.12)

struct ContentView: View {
  var body: some View {
    Group {
      if HarmoniusLaunchOptions.isSnapshotMode {
        snapshotContent
      } else {
        standardContent
      }
    }
  }

  private var standardContent: some View {
    VStack(spacing: 12) {
      Text("Harmonius")
        .font(.title.bold())
      Text("Metal 4 triangle (MTL4CommandQueue)")
        .font(.subheadline)
        .foregroundStyle(.secondary)
      metalView
    }
    .padding(24)
    .background(contentBackground)
    .accessibilityElement(children: .contain)
    .accessibilityIdentifier("snapshot-content")
  }

  private var snapshotContent: some View {
    metalView
      .frame(
        width: HarmoniusLaunchOptions.snapshotMetalSize.width,
        height: HarmoniusLaunchOptions.snapshotMetalSize.height
      )
      .background(contentBackground)
      .accessibilityElement(children: .contain)
      .accessibilityIdentifier("snapshot-content")
      #if os(macOS)
        .background(SnapshotWindowConfigurator())
      #endif
  }

  private var metalView: some View {
    MetalView()
  }
}

#if os(macOS)
  private struct SnapshotWindowConfigurator: NSViewRepresentable {
    func makeNSView(context: Context) -> NSView {
      let view = NSView(frame: .zero)
      DispatchQueue.main.async {
        guard let window = view.window else {
          return
        }
        window.titleVisibility = .hidden
        window.titlebarAppearsTransparent = true
        window.isOpaque = true
        window.backgroundColor = NSColor(contentBackground)
        window.hasShadow = false
        window.styleMask.remove(.resizable)
        let pointSize = HarmoniusLaunchOptions.snapshotMetalSize
        window.minSize = pointSize
        window.maxSize = pointSize
        window.setContentSize(pointSize)
      }
      return view
    }

    func updateNSView(_ nsView: NSView, context: Context) {}
  }
#endif

#Preview {
  ContentView()
}
