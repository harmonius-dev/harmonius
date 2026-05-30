import Foundation
import SlangReflection
import SwiftEmitter

enum ShaderToolError: Error, CustomStringConvertible {
  case invalidArguments([String])
  case unsupportedRuntime

  var description: String {
    switch self {
    case .invalidArguments(let arguments):
      return "expected source, output, generated Swift, package root, include dir: \(arguments)"
    case .unsupportedRuntime:
      return "HarmoniusShaderTool must run on the package host platform"
    }
  }
}

#if os(macOS) || os(Linux)
  let arguments = Array(CommandLine.arguments.dropFirst())
  guard arguments.count == 5 else {
    throw ShaderToolError.invalidArguments(arguments)
  }

  let sourcePath = URL(fileURLWithPath: arguments[0]).standardizedFileURL.path
  let outputPath = URL(fileURLWithPath: arguments[1]).standardizedFileURL.path
  let generatedSwiftPath = URL(fileURLWithPath: arguments[2]).standardizedFileURL.path
  let packageRoot = URL(fileURLWithPath: arguments[3]).standardizedFileURL.path
  let includeDir = URL(fileURLWithPath: arguments[4]).standardizedFileURL.path

  _ = packageRoot

  let outputURL = URL(fileURLWithPath: outputPath)
  try FileManager.default.createDirectory(
    at: outputURL.deletingLastPathComponent(),
    withIntermediateDirectories: true
  )

  let program = try SlangReflectionCompiler.compile(
    sourcePath: sourcePath,
    metallibOutputPath: outputPath,
    includeDirectory: includeDir
  )
  let generatedSwift = try SwiftEmitter().emit(
    program,
    metallibPath: outputPath
  )
  try generatedSwift.write(
    to: URL(fileURLWithPath: generatedSwiftPath),
    atomically: true,
    encoding: .utf8
  )
#else
  throw ShaderToolError.unsupportedRuntime
#endif
