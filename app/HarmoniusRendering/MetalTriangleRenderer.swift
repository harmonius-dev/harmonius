// HarmoniusShaderTypes is an internal C++ interop module (from Shaders/module.modulemap).
internal import HarmoniusShaderTypes
import Metal
import MetalKit

public final class MetalTriangleRenderer: NSObject, MTKViewDelegate {
  private let commandQueue: MTLCommandQueue
  private let renderPipelineState: MTLRenderPipelineState
  private let triangleVertexBuffer: MTLBuffer
  private let viewportSizeBuffer: MTLBuffer

  private var viewportWidth: UInt32 = 0
  private var viewportHeight: UInt32 = 0

  @MainActor
  public init?(view: MTKView) {
    guard let device = view.device else {
      return nil
    }
    guard
      let commandQueue = device.makeCommandQueue(),
      let defaultLibrary = device.makeDefaultLibrary()
    else {
      return nil
    }

    do {
      self.renderPipelineState = try Self.compileRenderPipeline(
        device: device,
        library: defaultLibrary,
        pixelFormat: view.colorPixelFormat
      )
    } catch {
      return nil
    }

    guard
      let triangleVertexBuffer = device.makeBuffer(
        length: MemoryLayout<TriangleData>.stride,
        options: .storageModeShared
      ),
      let viewportSizeBuffer = device.makeBuffer(
        length: MemoryLayout<hlslpp.uint2>.stride,
        options: .storageModeShared
      )
    else {
      return nil
    }

    self.commandQueue = commandQueue
    self.triangleVertexBuffer = triangleVertexBuffer
    self.viewportSizeBuffer = viewportSizeBuffer

    super.init()
    updateViewportSize(view.drawableSize)
  }

  @MainActor
  public func mtkView(_ view: MTKView, drawableSizeWillChange size: CGSize) {
    updateViewportSize(size)
  }

  @MainActor
  public func draw(in view: MTKView) {
    guard
      let drawable = view.currentDrawable,
      let renderPassDescriptor = view.currentRenderPassDescriptor,
      let commandBuffer = commandQueue.makeCommandBuffer()
    else {
      return
    }
    if let colorAttachment = renderPassDescriptor.colorAttachments[0] {
      colorAttachment.loadAction = .clear
      colorAttachment.clearColor = view.clearColor
    }

    guard
      let renderEncoder = commandBuffer.makeRenderCommandEncoder(
        descriptor: renderPassDescriptor
      )
    else {
      return
    }

    renderEncoder.label = "Snapshot triangle pass"
    renderEncoder.setRenderPipelineState(renderPipelineState)
    setViewport(for: renderEncoder)
    writeTriangleData()
    renderEncoder.setVertexBuffer(
      triangleVertexBuffer,
      offset: 0,
      index: Int(InputBufferIndexForVertexData.rawValue)
    )
    renderEncoder.setVertexBuffer(
      viewportSizeBuffer,
      offset: 0,
      index: Int(InputBufferIndexForViewportSize.rawValue)
    )
    renderEncoder.drawPrimitives(
      type: .triangle,
      vertexStart: 0,
      vertexCount: 3
    )
    renderEncoder.endEncoding()

    commandBuffer.present(drawable)
    commandBuffer.commit()
    commandBuffer.waitUntilCompleted()
  }

  private func updateViewportSize(_ size: CGSize) {
    guard
      size.width.isFinite,
      size.height.isFinite,
      size.width > 0,
      size.height > 0
    else {
      return
    }
    viewportWidth = UInt32(size.width)
    viewportHeight = UInt32(size.height)
    var viewportSize = hlslpp.uint2()
    viewportSize[0] = viewportWidth
    viewportSize[1] = viewportHeight
    withUnsafeBytes(of: &viewportSize) { bytes in
      viewportSizeBuffer.contents().copyMemory(
        from: bytes.baseAddress!,
        byteCount: bytes.count
      )
    }
  }

  private func setViewport(for encoder: MTLRenderCommandEncoder) {
    let viewport = MTLViewport(
      originX: 0,
      originY: 0,
      width: Double(viewportWidth),
      height: Double(viewportHeight),
      znear: 0,
      zfar: 1
    )
    encoder.setViewport(viewport)
  }

  private func writeTriangleData() {
    var triangle = TriangleGeometry.frameData()
    withUnsafeBytes(of: &triangle) { bytes in
      triangleVertexBuffer.contents().copyMemory(
        from: bytes.baseAddress!,
        byteCount: bytes.count
      )
    }
  }

  private static func compileRenderPipeline(
    device: MTLDevice,
    library: MTLLibrary,
    pixelFormat: MTLPixelFormat
  ) throws -> MTLRenderPipelineState {
    let descriptor = MTLRenderPipelineDescriptor()
    descriptor.label = "Harmonius Metal snapshot triangle pipeline"
    descriptor.vertexFunction = library.makeFunction(name: "vertexShader")
    descriptor.fragmentFunction = library.makeFunction(name: "fragmentShader")
    descriptor.colorAttachments[0].pixelFormat = pixelFormat
    return try device.makeRenderPipelineState(descriptor: descriptor)
  }
}
