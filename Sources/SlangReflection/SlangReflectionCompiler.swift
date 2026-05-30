import Foundation
import SlangReflectionBridge

public enum SlangReflectionCompiler {
  public static func compile(
    sourcePath: String,
    metallibOutputPath: String,
    includeDirectory: String
  ) throws -> ShaderProgramModel {
    let output = hsrCompileSlangReflection(
      sourcePath,
      metallibOutputPath,
      includeDirectory
    )
    defer {
      hsrFreeSlangReflectionOutput(output)
    }

    guard output.status == 0 else {
      let diagnostics = output.diagnostics.map(String.init(cString:)) ?? "unknown Slang error"
      throw SlangReflectionCompilerError.slangFailed(diagnostics)
    }
    guard let json = output.json else {
      throw SlangReflectionCompilerError.missingReflectionJSON
    }

    let data = Data(String(cString: json).utf8)
    do {
      return try JSONDecoder().decode(ShaderProgramModel.self, from: data)
    } catch {
      throw SlangReflectionCompilerError.invalidReflectionJSON(error)
    }
  }
}

public enum SlangReflectionCompilerError: Error, CustomStringConvertible {
  case slangFailed(String)
  case missingReflectionJSON
  case invalidReflectionJSON(Error)

  public var description: String {
    switch self {
    case .slangFailed(let diagnostics):
      return diagnostics
    case .missingReflectionJSON:
      return "Slang reflection did not return JSON"
    case .invalidReflectionJSON(let error):
      return "Slang reflection JSON could not be decoded: \(error)"
    }
  }
}
