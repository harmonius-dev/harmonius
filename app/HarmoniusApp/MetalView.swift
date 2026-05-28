import Foundation
import HarmoniusRendering
import MetalKit
import SwiftUI
#if os(iOS)
  import UIKit
#endif

// MetalViewCoordinator drives the Metal 4 renderer from either platform.
// @MainActor: MTKView properties and delegate methods are main-actor-isolated.
@MainActor
final class MetalViewCoordinator: NSObject, MTKViewDelegate {
  private var renderer: Metal4TriangleRenderer?
  private var snapshotRenderer: MetalTriangleRenderer?

  func configure(view: MTKView) {
    guard let device = view.device else { return }
    if HarmoniusLaunchOptions.isSnapshotMode {
      snapshotRenderer = MetalTriangleRenderer(view: view)
      return
    }
    guard device.supportsFamily(.metal4) else {
      NSLog("Metal 4 is required for Harmonius.")
      return
    }
    guard let renderer = Metal4TriangleRenderer(view: view) else { return }
    self.renderer = renderer
  }

  nonisolated func mtkView(_ view: MTKView, drawableSizeWillChange size: CGSize) {
    MainActor.assumeIsolated {
      renderer?.mtkView(view, drawableSizeWillChange: size)
      snapshotRenderer?.mtkView(view, drawableSizeWillChange: size)
    }
  }

  nonisolated func draw(in view: MTKView) {
    MainActor.assumeIsolated {
      renderer?.draw(in: view)
      snapshotRenderer?.draw(in: view)
    }
  }
}

@MainActor
private func makeConfiguredMTKView(coordinator: MetalViewCoordinator) -> MTKView {
  let device = MTLCreateSystemDefaultDevice()
  let isSnapshotMode = HarmoniusLaunchOptions.isSnapshotMode
  let frame =
    isSnapshotMode
    ? CGRect(origin: .zero, size: currentSnapshotPointSize())
    : .zero
  let view = MTKView(frame: frame, device: device)
  view.colorPixelFormat = .bgra8Unorm
  view.clearColor = MTLClearColor(red: 0, green: 0, blue: 0, alpha: 1)
  view.preferredFramesPerSecond = 60
  view.enableSetNeedsDisplay = isSnapshotMode
  view.isPaused = isSnapshotMode
  if isSnapshotMode {
    view.drawableSize = HarmoniusLaunchOptions.snapshotMetalPixelSize
    view.framebufferOnly = false
  }
  view.delegate = coordinator
  #if os(macOS)
    if isSnapshotMode {
      view.wantsLayer = true
      view.layer?.contentsScale = HarmoniusLaunchOptions.snapshotScale
    }
    view.setAccessibilityElement(true)
    view.setAccessibilityIdentifier("metal-view")
  #else
    if isSnapshotMode {
      view.contentScaleFactor = HarmoniusLaunchOptions.snapshotScale
    }
    view.isAccessibilityElement = true
    view.accessibilityIdentifier = "metal-view"
  #endif
  coordinator.configure(view: view)
  DispatchQueue.main.async {
    requestSnapshotDraw(in: view)
  }
  return view
}

private func currentSnapshotPointSize() -> CGSize {
  #if os(macOS)
    let pixelRect = CGRect(
      origin: .zero,
      size: HarmoniusLaunchOptions.snapshotCapturePixelSize
    )
    return NSScreen.main?.convertRectFromBacking(pixelRect).size
      ?? HarmoniusLaunchOptions.snapshotCapturePixelSize
  #elseif os(iOS)
    let scale = UIScreen.main.scale
    return CGSize(
      width: HarmoniusLaunchOptions.snapshotCapturePixelSize.width / scale,
      height: HarmoniusLaunchOptions.snapshotCapturePixelSize.height / scale
    )
  #else
    return HarmoniusLaunchOptions.snapshotCapturePixelSize
  #endif
}

@MainActor
func requestSnapshotDraw(in view: MTKView) {
  guard HarmoniusLaunchOptions.isSnapshotMode else { return }
  view.drawableSize = HarmoniusLaunchOptions.snapshotMetalPixelSize
  #if os(macOS)
    view.wantsLayer = true
    view.layer?.contentsScale = HarmoniusLaunchOptions.snapshotScale
    view.needsDisplay = true
  #else
    view.setNeedsDisplay()
  #endif
  view.draw()
}

// ---------------------------------------------------------------------------
// Platform view wrappers
// ---------------------------------------------------------------------------

#if os(macOS)
  struct MetalView: NSViewRepresentable {
    func makeCoordinator() -> MetalViewCoordinator { MetalViewCoordinator() }

    func makeNSView(context: Context) -> MTKView {
      makeConfiguredMTKView(coordinator: context.coordinator)
    }

    func updateNSView(_ nsView: MTKView, context: Context) {
      requestSnapshotDraw(in: nsView)
    }
  }
#elseif os(iOS)
  struct MetalView: UIViewRepresentable {
    func makeCoordinator() -> MetalViewCoordinator { MetalViewCoordinator() }

    func makeUIView(context: Context) -> MTKView {
      makeConfiguredMTKView(coordinator: context.coordinator)
    }

    func updateUIView(_ uiView: MTKView, context: Context) {
      requestSnapshotDraw(in: uiView)
    }
  }
#endif
