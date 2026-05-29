import HarmoniusShaderResources
internal import HarmoniusShaders
// HarmoniusShaders is an internal C++ interop module.
import Metal
import MetalKit
import QuartzCore

#if os(iOS) && targetEnvironment(simulator)
  public final class Metal4TriangleRenderer: NSObject, MTKViewDelegate {
    public static func isSupported(on device: MTLDevice) -> Bool {
      false
    }

    @MainActor
    public init?(view: MTKView) {
      NSLog("Metal 4 rendering is unavailable in the iOS simulator SDK.")
      return nil
    }

    @MainActor
    public func mtkView(_ view: MTKView, drawableSizeWillChange size: CGSize) {}

    @MainActor
    public func draw(in view: MTKView) {}
  }
#else
  public final class Metal4TriangleRenderer: NSObject, MTKViewDelegate {
    private let device: MTLDevice
    private let commandQueue: any MTL4CommandQueue
    private let commandBuffer: any MTL4CommandBuffer
    private let commandAllocators: [any MTL4CommandAllocator]
    private let argumentTable: any MTL4ArgumentTable
    private let residencySet: any MTLResidencySet
    private let sharedEvent: MTLSharedEvent
    private let viewportSizeBuffer: MTLBuffer
    private let triangleVertexBuffers: [MTLBuffer]
    private let renderPipelineState: any MTLRenderPipelineState
    private let defaultLibrary: MTLLibrary

    private var frameNumber: UInt64 = 0
    private var viewportWidth: UInt32 = 0
    private var viewportHeight: UInt32 = 0

    public static func isSupported(on device: MTLDevice) -> Bool {
      device.supportsFamily(.metal4)
    }

    @MainActor
    public init?(view: MTKView) {
      guard let device = view.device else {
        return nil
      }
      guard device.supportsFamily(.metal4) else {
        return nil
      }
      guard let commandQueue = device.makeMTL4CommandQueue(),
        let commandBuffer = device.makeCommandBuffer(),
        let defaultLibrary = try? HarmoniusShaderResources.makeDefaultLibrary(
          device: device
        )
      else {
        return nil
      }

      self.device = device
      self.commandQueue = commandQueue
      self.commandBuffer = commandBuffer
      self.defaultLibrary = defaultLibrary

      do {
        triangleVertexBuffers = try Self.makeTriangleBuffers(
          device: device,
          count: TriangleVertexLayout.maxFramesInFlight
        )
        argumentTable = try Self.makeArgumentTable(device: device)
        residencySet = try Self.makeResidencySet(device: device)
        commandAllocators = try Self.makeCommandAllocators(
          device: device,
          count: TriangleVertexLayout.maxFramesInFlight
        )
        viewportSizeBuffer = device.makeBuffer(
          length: MemoryLayout<HarmoniusUInt2>.stride,
          options: .storageModeShared
        )!
        renderPipelineState = try Self.compileRenderPipeline(
          device: device,
          library: defaultLibrary,
          pixelFormat: view.colorPixelFormat
        )
      } catch {
        return nil
      }

      sharedEvent = device.makeSharedEvent()!
      sharedEvent.signaledValue = frameNumber

      residencySet.addAllocation(viewportSizeBuffer)
      for buffer in triangleVertexBuffers {
        residencySet.addAllocation(buffer)
      }
      residencySet.commit()

      commandQueue.addResidencySet(residencySet)
      if let layer = view.layer as? CAMetalLayer {
        commandQueue.addResidencySet(layer.residencySet)
      }

      super.init()
      updateViewportSize(view.drawableSize)
    }

    @MainActor
    public func mtkView(_ view: MTKView, drawableSizeWillChange size: CGSize) {
      updateViewportSize(size)
    }

    @MainActor
    public func draw(in view: MTKView) {
      guard let drawable = view.currentDrawable,
        let renderPassDescriptor = view.currentMTL4RenderPassDescriptor
      else {
        return
      }
      if let colorAttachment = renderPassDescriptor.colorAttachments[0] {
        colorAttachment.loadAction = .clear
        colorAttachment.clearColor = view.clearColor
      }

      frameNumber &+= 1
      let frameIndex = Int(frameNumber % UInt64(TriangleVertexLayout.maxFramesInFlight))

      if frameNumber > UInt64(TriangleVertexLayout.maxFramesInFlight) {
        let earlierFrame = frameNumber - UInt64(TriangleVertexLayout.maxFramesInFlight)
        let signaled = sharedEvent.wait(
          untilSignaledValue: earlierFrame,
          timeoutMS: 10
        )
        if !signaled {
          NSLog("Timed out waiting for frame \(earlierFrame)")
        }
      }

      let frameAllocator = commandAllocators[frameIndex]
      frameAllocator.reset()
      commandBuffer.beginCommandBuffer(allocator: frameAllocator)
      commandBuffer.label = "Frame \(frameNumber)"

      guard
        let renderEncoder = commandBuffer.makeRenderCommandEncoder(
          descriptor: renderPassDescriptor,
          options: MTL4RenderEncoderOptions()
        )
      else {
        return
      }

      renderEncoder.label = "Triangle pass \(frameNumber)"
      renderEncoder.setRenderPipelineState(renderPipelineState)
      setViewport(for: renderEncoder)

      let vertexBuffer = triangleVertexBuffers[frameIndex]
      writeTriangleData(to: vertexBuffer)

      argumentTable.setAddress(
        vertexBuffer.gpuAddress,
        index: Int(InputBufferIndexForVertexData.rawValue)
      )
      argumentTable.setAddress(
        viewportSizeBuffer.gpuAddress,
        index: Int(InputBufferIndexForViewportSize.rawValue)
      )
      renderEncoder.setArgumentTable(argumentTable, stages: .vertex)

      renderEncoder.drawPrimitives(
        primitiveType: .triangle,
        vertexStart: 0,
        vertexCount: 3
      )
      renderEncoder.endEncoding()
      commandBuffer.endCommandBuffer()

      commandQueue.waitForDrawable(drawable)
      commandQueue.commit([commandBuffer])
      commandQueue.signalDrawable(drawable)
      drawable.present()
      commandQueue.signalEvent(sharedEvent, value: frameNumber)
    }

    private func updateViewportSize(_ size: CGSize) {
      guard size.width.isFinite, size.height.isFinite, size.width > 0, size.height > 0 else {
        return
      }
      viewportWidth = UInt32(size.width)
      viewportHeight = UInt32(size.height)
      var viewportSize = HarmoniusMakeUInt2(viewportWidth, viewportHeight)
      withUnsafeBytes(of: &viewportSize) { bytes in
        viewportSizeBuffer.contents().copyMemory(
          from: bytes.baseAddress!,
          byteCount: bytes.count
        )
      }
    }

    private func setViewport(for encoder: any MTL4RenderCommandEncoder) {
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

    private func writeTriangleData(to buffer: MTLBuffer) {
      var triangle = TriangleGeometry.frameData()
      withUnsafeBytes(of: &triangle) { bytes in
        buffer.contents().copyMemory(
          from: bytes.baseAddress!,
          byteCount: bytes.count
        )
      }
    }

    private static func makeTriangleBuffers(
      device: MTLDevice,
      count: Int
    ) throws -> [MTLBuffer] {
      try (0..<count).map { _ in
        guard
          let buffer = device.makeBuffer(
            length: MemoryLayout<TriangleData>.stride,
            options: .storageModeShared
          )
        else {
          throw RendererError.resourceCreationFailed("triangle buffer")
        }
        return buffer
      }
    }

    private static func makeArgumentTable(device: MTLDevice) throws -> any MTL4ArgumentTable {
      let descriptor = MTL4ArgumentTableDescriptor()
      descriptor.maxBufferBindCount = 2
      return try device.makeArgumentTable(descriptor: descriptor)
    }

    private static func makeResidencySet(device: MTLDevice) throws -> any MTLResidencySet {
      let descriptor = MTLResidencySetDescriptor()
      return try device.makeResidencySet(descriptor: descriptor)
    }

    private static func makeCommandAllocators(
      device: MTLDevice,
      count: Int
    ) throws -> [any MTL4CommandAllocator] {
      try (0..<count).map { _ in
        guard let allocator = device.makeCommandAllocator() else {
          throw RendererError.resourceCreationFailed("command allocator")
        }
        return allocator
      }
    }

    private static func compileRenderPipeline(
      device: MTLDevice,
      library: MTLLibrary,
      pixelFormat: MTLPixelFormat
    ) throws -> any MTLRenderPipelineState {
      try Metal4PipelineSupport.createRenderPipeline(
        device: device,
        pixelFormat: pixelFormat,
        library: library
      )
    }
  }

  private enum RendererError: Error {
    case resourceCreationFailed(String)
  }
#endif
