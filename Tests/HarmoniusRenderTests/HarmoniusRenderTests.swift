import Foundation
import HarmoniusRendering
import HarmoniusShaderResources
import Metal
import Testing
import simd

private let renderSize = RenderSize(width: 1920, height: 1080)

@Test func triangleRendersIntoTexture() throws {
  ShaderValidation.verifyAll()
  #expect(ShaderBindings.maxBufferBindCount == 2)

  let texture = try renderTriangleTexture()
  #expect(texture.width == renderSize.width)
  #expect(texture.height == renderSize.height)
  #expect(texture.pixelFormat == .bgra8Unorm)

  let pixels = try readPixels(from: texture)
  #expect(pixels.contains { $0.hasVisibleColor })
  #expect(pixels[pixelIndex(x: 0, y: 0)].isOpaqueBlack)
  #expect(pixels[pixelIndex(x: renderSize.width / 2, y: renderSize.height / 2)].hasVisibleColor)
}

private struct RenderSize {
  let width: Int
  let height: Int
}

private enum RenderSnapshotError: Error {
  case commandBufferFailed(String)
  case missingFeature(String)
  case missingResource(String)
  case resourceCreationFailed(String)
}

private struct Pixel {
  let blue: UInt8
  let green: UInt8
  let red: UInt8
  let alpha: UInt8

  var hasVisibleColor: Bool {
    red > 0 || green > 0 || blue > 0
  }

  var isOpaqueBlack: Bool {
    red == 0 && green == 0 && blue == 0 && alpha == 255
  }
}

private func renderTriangleTexture() throws -> MTLTexture {
  guard let device = MTLCreateSystemDefaultDevice() else {
    throw RenderSnapshotError.missingResource("Metal device")
  }
  return try renderTriangleTexture(device: device)
}

private func renderTriangleTexture(device: MTLDevice) throws -> MTLTexture {
  guard device.supportsFamily(.metal4) else {
    throw RenderSnapshotError.missingFeature("Metal 4")
  }
  guard let commandQueue = device.makeMTL4CommandQueue(),
    let commandBuffer = device.makeCommandBuffer(),
    let commandAllocator = device.makeCommandAllocator(),
    let completionEvent = device.makeSharedEvent()
  else {
    throw RenderSnapshotError.resourceCreationFailed("Metal 4 command resources")
  }

  let library = try makeShaderLibrary(device: device)
  let pipelineState = try makePipelineState(device: device, library: library)
  let texture = try makeRenderTexture(device: device)
  let triangleBuffer = try makeTriangleBuffer(device: device)
  let viewportBuffer = try makeViewportBuffer(device: device)
  let argumentTable = try makeArgumentTable(device: device)
  let passDescriptor = makeRenderPassDescriptor(texture: texture)

  completionEvent.signaledValue = 0
  commandAllocator.reset()
  commandBuffer.beginCommandBuffer(allocator: commandAllocator)
  guard
    let encoder = commandBuffer.makeRenderCommandEncoder(
      descriptor: passDescriptor,
      options: MTL4RenderEncoderOptions()
    )
  else {
    throw RenderSnapshotError.resourceCreationFailed("render encoder")
  }

  encoder.setRenderPipelineState(pipelineState)
  encoder.setViewport(
    MTLViewport(
      originX: 0,
      originY: 0,
      width: Double(renderSize.width),
      height: Double(renderSize.height),
      znear: 0,
      zfar: 1
    )
  )
  argumentTable.setAddress(
    triangleBuffer.gpuAddress,
    index: ShaderBindings.vertexData
  )
  argumentTable.setAddress(
    viewportBuffer.gpuAddress,
    index: ShaderBindings.viewportSize
  )
  encoder.setArgumentTable(argumentTable, stages: .vertex)
  encoder.drawPrimitives(
    primitiveType: .triangle,
    vertexStart: 0,
    vertexCount: 3
  )
  encoder.endEncoding()
  commandBuffer.endCommandBuffer()

  commandQueue.commit([commandBuffer])
  commandQueue.signalEvent(completionEvent, value: 1)
  if !completionEvent.wait(untilSignaledValue: 1, timeoutMS: 1_000) {
    throw RenderSnapshotError.commandBufferFailed("timed out")
  }
  return texture
}

private func makeShaderLibrary(device: MTLDevice) throws -> MTLLibrary {
  if let metallibPath = ProcessInfo.processInfo.environment["HARMONIUS_METALLIB_PATH"] {
    return try device.makeLibrary(URL: URL(fileURLWithPath: metallibPath))
  }
  return try HarmoniusShaderResources.makeDefaultLibrary(device: device)
}

private func makePipelineState(
  device: MTLDevice,
  library: MTLLibrary
) throws -> MTLRenderPipelineState {
  let compiler = try device.makeCompiler(descriptor: MTL4CompilerDescriptor())
  let descriptor = MTL4RenderPipelineDescriptor()
  descriptor.label = "Harmonius render validation pipeline"
  descriptor.colorAttachments[0].pixelFormat = .bgra8Unorm

  let vertexFunction = MTL4LibraryFunctionDescriptor()
  vertexFunction.library = library
  vertexFunction.name = ShaderEntryPoints.vertexShader
  descriptor.vertexFunctionDescriptor = vertexFunction

  let fragmentFunction = MTL4LibraryFunctionDescriptor()
  fragmentFunction.library = library
  fragmentFunction.name = ShaderEntryPoints.fragmentShader
  descriptor.fragmentFunctionDescriptor = fragmentFunction

  return try compiler.makeRenderPipelineState(
    descriptor: descriptor,
    compilerTaskOptions: nil
  )
}

private func makeArgumentTable(device: MTLDevice) throws -> any MTL4ArgumentTable {
  let descriptor = MTL4ArgumentTableDescriptor()
  descriptor.maxBufferBindCount = ShaderBindings.maxBufferBindCount
  return try device.makeArgumentTable(descriptor: descriptor)
}

private func makeRenderTexture(device: MTLDevice) throws -> MTLTexture {
  let descriptor = MTLTextureDescriptor.texture2DDescriptor(
    pixelFormat: .bgra8Unorm,
    width: renderSize.width,
    height: renderSize.height,
    mipmapped: false
  )
  descriptor.storageMode = .shared
  descriptor.usage = [.renderTarget, .shaderRead]
  guard let texture = device.makeTexture(descriptor: descriptor) else {
    throw RenderSnapshotError.resourceCreationFailed("render texture")
  }
  return texture
}

private func makeTriangleBuffer(device: MTLDevice) throws -> MTLBuffer {
  var triangle = TriangleGeometry.frameData()
  let length = MemoryLayout<TriangleData>.stride
  let buffer = withUnsafeBytes(of: &triangle) { bytes in
    device.makeBuffer(
      bytes: bytes.baseAddress!,
      length: length,
      options: .storageModeShared
    )
  }
  guard let buffer else {
    throw RenderSnapshotError.resourceCreationFailed("triangle buffer")
  }
  return buffer
}

private func makeViewportBuffer(device: MTLDevice) throws -> MTLBuffer {
  var viewport = SIMD2<UInt32>(
    UInt32(renderSize.width),
    UInt32(renderSize.height)
  )
  let length = MemoryLayout<SIMD2<UInt32>>.stride
  let buffer = withUnsafeBytes(of: &viewport) { bytes in
    device.makeBuffer(
      bytes: bytes.baseAddress!,
      length: length,
      options: .storageModeShared
    )
  }
  guard let buffer else {
    throw RenderSnapshotError.resourceCreationFailed("viewport buffer")
  }
  return buffer
}

private func makeRenderPassDescriptor(texture: MTLTexture) -> MTL4RenderPassDescriptor {
  let descriptor = MTL4RenderPassDescriptor()
  let colorAttachment = descriptor.colorAttachments[0]
  colorAttachment?.texture = texture
  colorAttachment?.loadAction = .clear
  colorAttachment?.storeAction = .store
  colorAttachment?.clearColor = MTLClearColor(red: 0, green: 0, blue: 0, alpha: 1)
  return descriptor
}

private func readPixels(from texture: MTLTexture) throws -> [Pixel] {
  let width = texture.width
  let height = texture.height
  let bytesPerPixel = 4
  let bytesPerRow = width * bytesPerPixel
  var pixels = [UInt8](repeating: 0, count: bytesPerRow * height)
  pixels.withUnsafeMutableBytes { bytes in
    texture.getBytes(
      bytes.baseAddress!,
      bytesPerRow: bytesPerRow,
      from: MTLRegionMake2D(0, 0, width, height),
      mipmapLevel: 0
    )
  }

  return stride(from: 0, to: pixels.count, by: bytesPerPixel).map { offset in
    Pixel(
      blue: pixels[offset],
      green: pixels[offset + 1],
      red: pixels[offset + 2],
      alpha: pixels[offset + 3]
    )
  }
}

private func pixelIndex(x: Int, y: Int) -> Int {
  y * renderSize.width + x
}
