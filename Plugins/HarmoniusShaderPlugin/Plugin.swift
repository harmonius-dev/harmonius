import Foundation
import PackagePlugin

@main
struct HarmoniusShaderPlugin: BuildToolPlugin {
  func createBuildCommands(
    context: PluginContext,
    target: Target
  ) throws -> [Command] {
    let toolPath = ProcessInfo.processInfo.environment["HARMONIUS_SHADER_TOOL"] ?? ""
    guard !toolPath.isEmpty else {
      throw HarmoniusShaderPluginError.missingShaderTool
    }
    let tool = URL(fileURLWithPath: toolPath)
    let packageDirectory = context.package.directoryURL
    let shaderDirectory = packageDirectory.appendingPathComponent(
      "Sources/HarmoniusShaders"
    )
    let source = shaderDirectory.appendingPathComponent("Triangle.slang")
    let output = context.pluginWorkDirectoryURL.appendingPathComponent(
      "default.metallib"
    )
    let generatedSwift = context.pluginWorkDirectoryURL.appendingPathComponent(
      "ShaderTypes.swift"
    )

    return [
      .buildCommand(
        displayName: "Compile and reflect Triangle.slang",
        executable: tool,
        arguments: [
          source.path,
          output.path,
          generatedSwift.path,
          packageDirectory.path,
          shaderDirectory.path,
        ],
        inputFiles: [
          source
        ],
        outputFiles: [
          output,
          generatedSwift,
        ]
      )
    ]
  }
}

enum HarmoniusShaderPluginError: Error, CustomStringConvertible {
  case missingShaderTool

  var description: String {
    "HARMONIUS_SHADER_TOOL must point to the host HarmoniusShaderTool executable"
  }
}
