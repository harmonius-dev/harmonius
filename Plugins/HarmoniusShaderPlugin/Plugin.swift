import Foundation
import PackagePlugin

@main
struct HarmoniusShaderPlugin: BuildToolPlugin {
  func createBuildCommands(
    context: PluginContext,
    target: Target
  ) throws -> [Command] {
    let tool = try context.tool(named: "HarmoniusShaderTool")
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
        executable: tool.url,
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
