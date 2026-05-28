import SwiftUI

private let contentBackground = Color(red: 0.08, green: 0.09, blue: 0.12)

struct ContentView: View {
  @State private var metalViewReady = false

  var body: some View {
    Group {
      if HarmoniusLaunchOptions.isSnapshotMode {
        snapshotContent
      } else {
        standardContent
      }
    }
    #if os(macOS)
      .background(SnapshotWindowConfigurator())
    #endif
  }

  private var standardContent: some View {
    VStack(spacing: 12) {
      Text("Harmonius")
        .font(.title.bold())
      Text("Metal 4 triangle (MTL4CommandQueue)")
        .font(.subheadline)
        .foregroundStyle(.secondary)
      metalView
      readinessMarker
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
      .overlay {
        readinessMarker
      }
  }

  private var metalView: some View {
    MetalView(isReady: $metalViewReady)
      #if os(macOS)
        .frame(
          minWidth: HarmoniusLaunchOptions.snapshotMetalSize.width,
          minHeight: HarmoniusLaunchOptions.snapshotMetalSize.height
        )
      #endif
  }

  private var readinessMarker: some View {
    Text("ready")
      .accessibilityIdentifier("metal-view-ready")
      .accessibilityHidden(!metalViewReady)
      .frame(width: 0, height: 0)
      .opacity(0)
  }
}

#if os(macOS)
  private struct SnapshotWindowConfigurator: NSViewRepresentable {
    func makeNSView(context: Context) -> NSView {
      let view = NSView(frame: .zero)
      DispatchQueue.main.async {
        guard HarmoniusLaunchOptions.isSnapshotMode, let window = view.window else {
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
