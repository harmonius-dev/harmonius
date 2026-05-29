import Foundation

enum ShaderToolError: Error, CustomStringConvertible {
  case invalidArguments([String])
  case missingSlangCompiler([String])
  case processFailed(Int32)
  case unsupportedRuntime

  var description: String {
    switch self {
    case .invalidArguments(let arguments):
      return "expected source, output, generated Swift, package root, include dir: \(arguments)"
    case .missingSlangCompiler(let candidates):
      return "could not find slangc. Tried: \(candidates.joined(separator: ", "))"
    case .processFailed(let status):
      return "slangc failed with exit status \(status)"
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

  let sourcePath = arguments[0]
  let outputPath = arguments[1]
  let generatedSwiftPath = arguments[2]
  let packageRoot = arguments[3]
  let includeDir = arguments[4]

  func existingSlangCompiler() throws -> String {
    let fileManager = FileManager.default
    var candidates: [String] = []
    if let explicitPath = ProcessInfo.processInfo.environment["HARMONIUS_SLANGC"] {
      candidates.append(explicitPath)
    }
    candidates.append(
      "\(packageRoot)/build/macos/vcpkg_installed/arm64-osx/tools/shader-slang/slangc"
    )
    candidates.append(
      "\(packageRoot)/vcpkg_installed/arm64-osx/tools/shader-slang/slangc"
    )

    for candidate in candidates where fileManager.isExecutableFile(atPath: candidate) {
      return candidate
    }
    throw ShaderToolError.missingSlangCompiler(candidates)
  }

  let outputURL = URL(fileURLWithPath: outputPath)
  try FileManager.default.createDirectory(
    at: outputURL.deletingLastPathComponent(),
    withIntermediateDirectories: true
  )

  let slangc = try existingSlangCompiler()
  let process = Process()
  process.executableURL = URL(fileURLWithPath: slangc)
  process.arguments = [
    "-target",
    "metallib",
    "-I\(includeDir)",
    "-o",
    outputPath,
    sourcePath,
  ]
  try process.run()
  process.waitUntilExit()
  guard process.terminationStatus == 0 else {
    throw ShaderToolError.processFailed(process.terminationStatus)
  }

  let escapedOutputPath =
    outputPath
    .replacingOccurrences(of: "\\", with: "\\\\")
    .replacingOccurrences(of: "\"", with: "\\\"")
  let generatedSwift = """
    import Foundation

    public extension HarmoniusShaderResources {
      static let defaultMetallibPath = "\(escapedOutputPath)"

      static var defaultMetallibURL: URL {
        URL(fileURLWithPath: defaultMetallibPath)
      }
    }

    """

  try generatedSwift.write(
    to: URL(fileURLWithPath: generatedSwiftPath),
    atomically: true,
    encoding: .utf8
  )
#else
  throw ShaderToolError.unsupportedRuntime
#endif
