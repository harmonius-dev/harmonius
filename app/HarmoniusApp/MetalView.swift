import Foundation
import HarmoniusRendering
import MetalKit
import SwiftUI

// MetalViewCoordinator drives the Metal 4 renderer from either platform.
// @MainActor: MTKView properties and delegate methods are main-actor-isolated.
@MainActor
final class MetalViewCoordinator: NSObject, MTKViewDelegate {
  private var renderer: Metal4TriangleRenderer?

  func configure(view: MTKView) {
    guard let device = view.device else { return }
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
    }
  }

  nonisolated func draw(in view: MTKView) {
    MainActor.assumeIsolated {
      renderer?.draw(in: view)
    }
  }
}

@MainActor
private func makeConfiguredMTKView(coordinator: MetalViewCoordinator) -> MTKView {
  let view = MTKView()
  view.device = MTLCreateSystemDefaultDevice()
  view.colorPixelFormat = .bgra8Unorm
  view.clearColor = MTLClearColor(red: 0, green: 0, blue: 0, alpha: 1)
  view.preferredFramesPerSecond = 60
  view.enableSetNeedsDisplay = false
  view.isPaused = false
  view.delegate = coordinator
  #if os(macOS)
    view.setAccessibilityElement(true)
    view.setAccessibilityIdentifier("metal-view")
  #else
    view.isAccessibilityElement = true
    view.accessibilityIdentifier = "metal-view"
  #endif
  coordinator.configure(view: view)
  return view
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

    func updateNSView(_ nsView: MTKView, context: Context) {}
  }
#elseif os(iOS)
  struct MetalView: UIViewRepresentable {
    func makeCoordinator() -> MetalViewCoordinator { MetalViewCoordinator() }

    func makeUIView(context: Context) -> MTKView {
      makeConfiguredMTKView(coordinator: context.coordinator)
    }

    func updateUIView(_ uiView: MTKView, context: Context) {}
  }
#endif
