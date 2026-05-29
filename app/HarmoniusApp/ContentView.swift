import SwiftUI

#if os(macOS)
  import MetalKit
#endif

private let contentBackground = Color(red: 0.08, green: 0.09, blue: 0.12)

public struct ContentView: View {
  @State private var snapshotPointSize = HarmoniusLaunchOptions.snapshotCapturePixelSize

  public init() {}

  public var body: some View {
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
        width: snapshotPointSize.width,
        height: snapshotPointSize.height
      )
      .background(contentBackground)
      .accessibilityElement(children: .contain)
      .accessibilityIdentifier("snapshot-content")
      #if os(macOS)
        .background(SnapshotWindowConfigurator(pointSize: $snapshotPointSize))
      #endif
  }

  private var metalView: some View {
    MetalView()
  }
}

#if os(macOS)
  private struct SnapshotWindowConfigurator: NSViewRepresentable {
    @Binding var pointSize: CGSize

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
        let pixelRect = CGRect(
          origin: .zero,
          size: HarmoniusLaunchOptions.snapshotCapturePixelSize
        )
        let scaledPointSize = window.convertFromBacking(pixelRect).size
        pointSize = scaledPointSize
        window.minSize = scaledPointSize
        window.maxSize = scaledPointSize
        window.setContentSize(scaledPointSize)
        requestSnapshotDraws(in: window.contentView)
        DispatchQueue.main.async {
          requestSnapshotDraws(in: window.contentView)
        }
      }
      return view
    }

    func updateNSView(_ nsView: NSView, context: Context) {}
  }

  @MainActor
  private func requestSnapshotDraws(in view: NSView?) {
    guard let view else { return }
    if let metalView = view as? MTKView {
      requestSnapshotDraw(in: metalView)
    }
    for subview in view.subviews {
      requestSnapshotDraws(in: subview)
    }
  }
#endif

#Preview {
  ContentView()
}
