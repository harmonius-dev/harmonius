import Foundation
import SlangReflection

public struct TypeMapper {
  public init() {}

  public func swiftType(for type: ShaderType) -> String {
    switch type {
    case .scalar(let scalar):
      return swiftScalarType(for: scalar)
    case .vector(let element, let count):
      return "SIMD\(count)<\(swiftScalarType(for: element))>"
    case .matrix(let element, let rows, let columns):
      return swiftMatrixType(element: element, rows: rows, columns: columns)
    case .array(let element, let count, _):
      return "ShaderArray\(count)<\(swiftType(for: element))>"
    case .struct(let name):
      return swiftTypeIdentifier(name)
    case .constantBuffer(let element), .parameterBlock(let element):
      return swiftType(for: element)
    case .texture(let name), .resource(let name, _), .unsupported(let name):
      return "Never /* \(name) */"
    case .sampler:
      return "Never /* sampler */"
    }
  }

  public func arrayCounts(in program: ShaderProgramModel) -> [Int] {
    let counts = program.structs.reduce(into: Set<Int>()) { partialResult, shaderStruct in
      for field in shaderStruct.fields {
        collectArrayCounts(from: field.type, into: &partialResult)
      }
    }
    return counts.sorted()
  }

  public func swiftIdentifier(_ name: String) -> String {
    let candidate = sanitized(name, allowLeadingDigit: false)
    if swiftKeywords.contains(candidate) {
      return "`\(candidate)`"
    }
    return candidate
  }

  public func swiftTypeIdentifier(_ name: String) -> String {
    let candidate = sanitized(name, allowLeadingDigit: false)
    if candidate.first?.isLowercase == true {
      return candidate.prefix(1).uppercased() + candidate.dropFirst()
    }
    return candidate
  }

  public func layoutIdentifier(for typeName: String) -> String {
    let typeIdentifier = swiftTypeIdentifier(typeName)
    return typeIdentifier.prefix(1).lowercased() + typeIdentifier.dropFirst()
  }

  private func swiftScalarType(for scalar: ScalarKind) -> String {
    switch scalar {
    case .bool:
      return "Bool"
    case .int32:
      return "Int32"
    case .uint32:
      return "UInt32"
    case .int64:
      return "Int64"
    case .uint64:
      return "UInt64"
    case .float16:
      return "Float16"
    case .float32:
      return "Float"
    case .float64:
      return "Double"
    case .unknown:
      return "Never /* unknown scalar */"
    }
  }

  private func swiftMatrixType(element: ScalarKind, rows: Int, columns: Int) -> String {
    switch (element, rows, columns) {
    case (.float32, 2, 2):
      return "simd_float2x2"
    case (.float32, 3, 3):
      return "simd_float3x3"
    case (.float32, 4, 4):
      return "simd_float4x4"
    default:
      let scalar = swiftScalarType(for: element)
      return "Never /* matrix<\(scalar), \(rows), \(columns)> */"
    }
  }

  private func collectArrayCounts(from type: ShaderType, into counts: inout Set<Int>) {
    switch type {
    case .array(let element, let count, _):
      counts.insert(count)
      collectArrayCounts(from: element, into: &counts)
    case .constantBuffer(let element), .parameterBlock(let element):
      collectArrayCounts(from: element, into: &counts)
    case .resource(_, let element):
      if let element {
        collectArrayCounts(from: element, into: &counts)
      }
    default:
      break
    }
  }

  private func sanitized(_ name: String, allowLeadingDigit: Bool) -> String {
    var result = ""
    for scalar in name.unicodeScalars {
      if CharacterSet.alphanumerics.contains(scalar) || scalar == "_" {
        result.unicodeScalars.append(scalar)
      } else {
        result.append("_")
      }
    }
    if result.isEmpty {
      result = "value"
    }
    if !allowLeadingDigit, result.first?.isNumber == true {
      result = "_\(result)"
    }
    return result
  }

  private let swiftKeywords: Set<String> = [
    "Any", "Type", "Self", "actor", "as", "associatedtype", "async", "await",
    "break", "case", "catch", "class", "continue", "default", "defer", "deinit",
    "do", "else", "enum", "extension", "fallthrough", "false", "fileprivate",
    "for", "func", "guard", "if", "import", "in", "init", "inout", "internal",
    "is", "let", "nil", "nonisolated", "open", "operator", "private", "protocol",
    "public", "repeat", "rethrows", "return", "self", "some", "static", "struct",
    "subscript", "super", "switch", "throw", "throws", "true", "try", "typealias",
    "var", "where", "while",
  ]
}
