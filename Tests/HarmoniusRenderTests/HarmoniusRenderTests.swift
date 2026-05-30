import AppKit
import CoreGraphics
import Foundation
import HarmoniusRendering
import HarmoniusShaderResources
import Metal
import SnapshotTesting
import Testing
import simd

private let renderSize = SnapshotSize(width: 960, height: 540)
private let snapshotSize = SnapshotSize(width: 480, height: 270)

@Test func testTriangleRendersSnapshot() throws {
  let image = try renderTriangleSnapshot()
  let record: SnapshotTestingConfiguration.Record =
    ProcessInfo.processInfo.environment["SNAPSHOT_RECORD"] == "1" ? .all : .missing

  withSnapshotTesting(record: record) {
    assertSnapshot(
      of: image,
      as: .image(precision: 0.98),
      named: "triangle"
    )
  }
}

private struct SnapshotSize {
  let width: Int
  let height: Int
}

private enum RenderSnapshotError: Error {
  case commandBufferFailed(String)
  case missingFeature(String)
  case missingResource(String)
  case resourceCreationFailed(String)
}

private func renderTriangleSnapshot() throws -> NSImage {
  guard let device = MTLCreateSystemDefaultDevice() else {
    throw RenderSnapshotError.missingResource("Metal device")
  }
  let texture = try renderTriangleTexture(device: device)
  let image = try makeImage(from: texture)
  return try resizedImage(image, to: snapshotSize)
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
  descriptor.label = "Harmonius render snapshot pipeline"
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
  descriptor.maxBufferBindCount = 2
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

private func makeImage(from texture: MTLTexture) throws -> CGImage {
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
  return try makeImage(width: width, height: height, bytes: pixels)
}

private func resizedImage(_ image: CGImage, to size: SnapshotSize) throws -> NSImage {
  let bytesPerPixel = 4
  let bytesPerRow = size.width * bytesPerPixel
  var pixels = [UInt8](repeating: 0, count: bytesPerRow * size.height)
  let contextCreated = pixels.withUnsafeMutableBytes { bytes in
    guard
      let context = makeBitmapContext(
        width: size.width,
        height: size.height,
        bytesPerRow: bytesPerRow,
        bytes: bytes.baseAddress!
      )
    else {
      return false
    }

    context.interpolationQuality = .high
    context.draw(image, in: CGRect(x: 0, y: 0, width: size.width, height: size.height))
    return true
  }
  if !contextCreated {
    throw RenderSnapshotError.resourceCreationFailed("resize context")
  }

  let cgImage = try makeImage(width: size.width, height: size.height, bytes: pixels)
  return NSImage(cgImage: cgImage, size: NSSize(width: size.width, height: size.height))
}

private func makeImage(width: Int, height: Int, bytes: [UInt8]) throws -> CGImage {
  let bytesPerPixel = 4
  let bytesPerRow = width * bytesPerPixel
  let data = Data(bytes)
  guard let provider = CGDataProvider(data: data as CFData) else {
    throw RenderSnapshotError.resourceCreationFailed("image data provider")
  }
  guard
    let image = CGImage(
      width: width,
      height: height,
      bitsPerComponent: 8,
      bitsPerPixel: bytesPerPixel * 8,
      bytesPerRow: bytesPerRow,
      space: CGColorSpaceCreateDeviceRGB(),
      bitmapInfo: bitmapInfo,
      provider: provider,
      decode: nil,
      shouldInterpolate: false,
      intent: .defaultIntent
    )
  else {
    throw RenderSnapshotError.resourceCreationFailed("CGImage")
  }
  return image
}

private func makeBitmapContext(
  width: Int,
  height: Int,
  bytesPerRow: Int,
  bytes: UnsafeMutableRawPointer
) -> CGContext? {
  CGContext(
    data: bytes,
    width: width,
    height: height,
    bitsPerComponent: 8,
    bytesPerRow: bytesPerRow,
    space: CGColorSpaceCreateDeviceRGB(),
    bitmapInfo: bitmapInfo.rawValue
  )
}

private let bitmapInfo = CGBitmapInfo(rawValue: CGImageAlphaInfo.premultipliedFirst.rawValue)
  .union(.byteOrder32Little)
