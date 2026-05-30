import Foundation
import SlangReflection
import SwiftBasicFormat
import SwiftSyntax
import SwiftSyntaxBuilder

public struct SwiftEmitter {
  private let mapper: TypeMapper

  public init(mapper: TypeMapper = TypeMapper()) {
    self.mapper = mapper
  }

  public func emit(
    _ program: ShaderProgramModel,
    metallibPath: String? = nil
  ) throws -> String {
    var declarations: [DeclSyntax] = [
      decl("import Foundation"),
      decl("import simd"),
    ]

    if let metallibPath {
      declarations.append(makeResourcePathExtension(metallibPath: metallibPath))
    }

    declarations.append(makeProgramNamespace(program))
    declarations.append(makeLayoutSupport())
    declarations.append(contentsOf: makeArrayWrappers(program))
    declarations.append(contentsOf: program.structs.map(makeShaderStruct))
    declarations.append(makeBindings(program.bindings))
    declarations.append(makeEntryPointMetadata())
    declarations.append(makeEntryPoints(program.entryPoints))
    declarations.append(makeArgumentBuffers(program.parameterBlocks))
    declarations.append(makeReflectionLayout(program.structs))
    declarations.append(makeValidation(program.structs))

    return formattedSource(from: declarations)
  }

  private func makeResourcePathExtension(metallibPath: String) -> DeclSyntax {
    let escapedPath = escapedSwiftString(metallibPath)
    return decl(
      """
      public extension HarmoniusShaderResources {
        static let defaultMetallibPath = "\(escapedPath)"

        static var defaultMetallibURL: URL {
          URL(fileURLWithPath: defaultMetallibPath)
        }
      }
      """
    )
  }

  private func makeProgramNamespace(_ program: ShaderProgramModel) -> DeclSyntax {
    decl(
      """
      public enum HarmoniusShaders {
        public static let moduleName = "\(escapedSwiftString(program.moduleName))"
      }
      """
    )
  }

  private func makeLayoutSupport() -> DeclSyntax {
    decl(
      """
      public struct ShaderFieldLayout: Equatable, Sendable {
        public var name: String
        public var offset: Int
        public var size: Int
        public var alignment: Int

        public init(name: String, offset: Int, size: Int, alignment: Int) {
          self.name = name
          self.offset = offset
          self.size = size
          self.alignment = alignment
        }
      }

      public struct ShaderStructLayout: Equatable, Sendable {
        public var name: String
        public var size: Int
        public var alignment: Int
        public var fields: [ShaderFieldLayout]

        public init(
          name: String,
          size: Int,
          alignment: Int,
          fields: [ShaderFieldLayout]
        ) {
          self.name = name
          self.size = size
          self.alignment = alignment
          self.fields = fields
        }
      }
      """
    )
  }

  private func makeArrayWrappers(_ program: ShaderProgramModel) -> [DeclSyntax] {
    mapper.arrayCounts(in: program).map(makeArrayWrapper)
  }

  private func makeArrayWrapper(count: Int) -> DeclSyntax {
    let storageType: String
    if count == 1 {
      storageType = "Element"
    } else {
      storageType = "(\(Array(repeating: "Element", count: count).joined(separator: ", ")))"
    }

    let getters = (0..<count).map { index in
      let access = count == 1 ? "values" : "values.\(index)"
      return "case \(index): return \(access)"
    }.joined(separator: "\n")
    let setters = (0..<count).map { index in
      let access = count == 1 ? "values" : "values.\(index)"
      return "case \(index): \(access) = newValue"
    }.joined(separator: "\n")

    return decl(
      """
      @frozen public struct ShaderArray\(count)<Element> {
        public var values: \(storageType)

        public init(_ values: \(storageType)) {
          self.values = values
        }

        public subscript(index: Int) -> Element {
          get {
            switch index {
            \(getters)
            default:
              preconditionFailure("Shader array index out of range")
            }
          }
          set {
            switch index {
            \(setters)
            default:
              preconditionFailure("Shader array index out of range")
            }
          }
        }
      }
      """
    )
  }

  private func makeShaderStruct(_ shaderStruct: ShaderStruct) -> DeclSyntax {
    let properties = shaderStruct.fields.map { field in
      "public var \(mapper.swiftIdentifier(field.name)): \(mapper.swiftType(for: field.type))"
    }.joined(separator: "\n")

    let initDeclaration: String
    if shaderStruct.fields.isEmpty {
      initDeclaration = "public init() {}"
    } else {
      let parameters = shaderStruct.fields.map { field in
        "\(mapper.swiftIdentifier(field.name)): \(mapper.swiftType(for: field.type))"
      }.joined(separator: ",\n")
      let assignments = shaderStruct.fields.map { field in
        let identifier = mapper.swiftIdentifier(field.name)
        return "self.\(identifier) = \(identifier)"
      }.joined(separator: "\n")
      initDeclaration = """
        public init(
        \(parameters)
        ) {
        \(assignments)
        }
        """
    }

    return decl(
      """
      @frozen public struct \(mapper.swiftTypeIdentifier(shaderStruct.name)) {
      \(properties)

      \(initDeclaration)
      }
      """
    )
  }

  private func makeBindings(_ bindings: [ShaderBinding]) -> DeclSyntax {
    let constants = bindings.sorted { lhs, rhs in
      (lhs.space, lhs.index, lhs.name) < (rhs.space, rhs.index, rhs.name)
    }.map { binding in
      "public static let \(mapper.swiftIdentifier(binding.name)) = \(binding.index)"
    }.joined(separator: "\n")

    return decl(
      """
      public enum ShaderBindings {
      \(constants)
      }
      """
    )
  }

  private func makeEntryPointMetadata() -> DeclSyntax {
    decl(
      """
      public struct ShaderEntryPointMetadata: Equatable, Sendable {
        public var name: String
        public var stage: String

        public init(name: String, stage: String) {
          self.name = name
          self.stage = stage
        }
      }
      """
    )
  }

  private func makeEntryPoints(_ entryPoints: [ShaderEntryPoint]) -> DeclSyntax {
    let constants = entryPoints.map { entryPoint in
      """
      public static let \(mapper.swiftIdentifier(entryPoint.name)) =
        "\(escapedSwiftString(entryPoint.name))"
      """
    }.joined(separator: "\n")
    let metadata = entryPoints.map { entryPoint in
      """
      ShaderEntryPointMetadata(
        name: "\(escapedSwiftString(entryPoint.name))",
        stage: "\(entryPoint.stage.rawValue)"
      )
      """
    }.joined(separator: ",\n")

    return decl(
      """
      public enum ShaderEntryPoints {
      \(constants)

        public static let all: [ShaderEntryPointMetadata] = [
      \(metadata)
        ]
      }
      """
    )
  }

  private func makeArgumentBuffers(_ blocks: [ShaderParameterBlock]) -> DeclSyntax {
    guard !blocks.isEmpty else {
      return decl("public enum ShaderArgumentBuffers {}")
    }

    let wrappers = blocks.map { block in
      """
      public struct \(mapper.swiftTypeIdentifier(block.name)) {
        public static let binding = \(block.index)
        public static let space = \(block.space)
      }
      """
    }.joined(separator: "\n")

    return decl(
      """
      public enum ShaderArgumentBuffers {
      \(wrappers)
      }
      """
    )
  }

  private func makeReflectionLayout(_ structs: [ShaderStruct]) -> DeclSyntax {
    let constants = structs.map { shaderStruct in
      let fields = shaderStruct.fields.map { field in
        """
        ShaderFieldLayout(
          name: "\(escapedSwiftString(field.name))",
          offset: \(field.offset),
          size: \(field.size),
          alignment: \(field.alignment)
        )
        """
      }.joined(separator: ",\n")

      return """
          public static let \(mapper.layoutIdentifier(for: shaderStruct.name)) =
            ShaderStructLayout(
              name: "\(escapedSwiftString(shaderStruct.name))",
              size: \(shaderStruct.size),
              alignment: \(shaderStruct.alignment),
              fields: [
        \(fields)
              ]
            )
        """
    }.joined(separator: "\n")

    return decl(
      """
      public enum ShaderReflectionLayout {
      \(constants)
      }
      """
    )
  }

  private func makeValidation(_ structs: [ShaderStruct]) -> DeclSyntax {
    let structChecks = structs.map { shaderStruct in
      let typeName = mapper.swiftTypeIdentifier(shaderStruct.name)
      let layoutName = mapper.layoutIdentifier(for: shaderStruct.name)
      return "verify(\(typeName).self, reflectedLayout: ShaderReflectionLayout.\(layoutName))"
    }.joined(separator: "\n")

    let fieldChecks = structs.flatMap { shaderStruct in
      shaderStruct.fields.map { field in
        let typeName = mapper.swiftTypeIdentifier(shaderStruct.name)
        let fieldName = mapper.swiftIdentifier(field.name)
        let escapedName = escapedSwiftString(field.name)
        return """
          verifyField(
            \(typeName).self,
            \\.\(fieldName),
            name: "\(escapedName)",
            offset: \(field.offset)
          )
          """
      }
    }.joined(separator: "\n")

    return decl(
      """
      public enum ShaderValidation {
        public static func verifyAll() {
      \(structChecks)
      \(fieldChecks)
        }

        public static func verify<T>(
          _ type: T.Type,
          reflectedLayout: ShaderStructLayout
        ) {
          precondition(
            MemoryLayout<T>.size == reflectedLayout.size,
            "\\(T.self) size does not match reflected Slang layout"
          )
          precondition(
            MemoryLayout<T>.alignment == reflectedLayout.alignment,
            "\\(T.self) alignment does not match reflected Slang layout"
          )
        }

        public static func verifyField<T>(
          _ type: T.Type,
          _ keyPath: PartialKeyPath<T>,
          name: String,
          offset: Int
        ) {
          guard let actual = MemoryLayout<T>.offset(of: keyPath) else {
            preconditionFailure("\\(T.self).\\(name) has no stable stored offset")
          }
          precondition(
            actual == offset,
            "\\(T.self).\\(name) offset does not match reflected Slang layout"
          )
        }
      }
      """
    )
  }

  private func formattedSource(from declarations: [DeclSyntax]) -> String {
    let items = declarations.map { declaration in
      CodeBlockItemSyntax(item: .decl(declaration))
    }
    let source = SourceFileSyntax(
      statements: CodeBlockItemListSyntax(items),
      endOfFileToken: .endOfFileToken()
    )
    return BasicFormat().rewrite(source).description
  }

  private func decl(_ source: String) -> DeclSyntax {
    DeclSyntax(stringLiteral: source)
  }

  private func escapedSwiftString(_ value: String) -> String {
    value
      .replacingOccurrences(of: "\\", with: "\\\\")
      .replacingOccurrences(of: "\"", with: "\\\"")
      .replacingOccurrences(of: "\n", with: "\\n")
      .replacingOccurrences(of: "\r", with: "\\r")
      .replacingOccurrences(of: "\t", with: "\\t")
  }
}
