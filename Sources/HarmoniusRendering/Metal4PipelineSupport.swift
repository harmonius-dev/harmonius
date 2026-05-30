import HarmoniusShaderResources
import Metal

#if !(os(iOS) && targetEnvironment(simulator))
  enum Metal4PipelineSupport {
    static func createRenderPipeline(
      device: MTLDevice,
      pixelFormat: MTLPixelFormat,
      library: MTLLibrary
    ) throws -> any MTLRenderPipelineState {
      let compiler = try device.makeCompiler(descriptor: MTL4CompilerDescriptor())

      let descriptor = MTL4RenderPipelineDescriptor()
      descriptor.label = "Harmonius Metal 4 triangle pipeline"
      descriptor.colorAttachments[0].pixelFormat = pixelFormat

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
  }
#endif
