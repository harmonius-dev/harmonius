import SlangReflection
import SwiftEmitter
import Testing

@Test func emitsStructs() throws {
  let source = try SwiftEmitter().emit(
    ShaderProgramModel(
      moduleName: "Triangle",
      structs: [
        ShaderStruct(
          name: "CameraData",
          fields: [
            ShaderField(
              name: "viewProj",
              type: .matrix(element: .float32, rows: 4, columns: 4),
              offset: 0,
              size: 64,
              alignment: 16
            ),
            ShaderField(
              name: "eyePos",
              type: .vector(element: .float32, count: 3),
              offset: 64,
              size: 16,
              alignment: 16
            ),
          ],
          size: 80,
          alignment: 16
        )
      ]
    )
  )

  #expect(source.contains("public struct CameraData"))
  #expect(source.contains("public var viewProj: simd_float4x4"))
  #expect(source.contains("public var eyePos: SIMD3<Float>"))
}

@Test func emitsNestedStructs() throws {
  let source = try SwiftEmitter().emit(
    ShaderProgramModel(
      moduleName: "Nested",
      structs: [
        ShaderStruct(
          name: "VertexData",
          fields: [
            ShaderField(
              name: "position",
              type: .vector(element: .float32, count: 2),
              offset: 0,
              size: 8,
              alignment: 4
            )
          ],
          size: 8,
          alignment: 4
        ),
        ShaderStruct(
          name: "TriangleData",
          fields: [
            ShaderField(
              name: "vertex0",
              type: .struct("VertexData"),
              offset: 0,
              size: 8,
              alignment: 4
            )
          ],
          size: 8,
          alignment: 4
        ),
      ]
    )
  )

  #expect(source.contains("public var vertex0: VertexData"))
}

@Test func emitsArrayWrappers() throws {
  let source = try SwiftEmitter().emit(
    ShaderProgramModel(
      moduleName: "Arrays",
      structs: [
        ShaderStruct(
          name: "Weights",
          fields: [
            ShaderField(
              name: "values",
              type: .array(
                element: .scalar(.float32),
                count: 3,
                stride: 4
              ),
              offset: 0,
              size: 12,
              alignment: 4
            )
          ],
          size: 12,
          alignment: 4
        )
      ]
    )
  )

  #expect(source.contains("public struct ShaderArray3<Element>"))
  #expect(source.contains("public var values: ShaderArray3<Float>"))
}

@Test func emitsResourceBindings() throws {
  let source = try SwiftEmitter().emit(
    ShaderProgramModel(
      moduleName: "Resources",
      bindings: [
        ShaderBinding(
          name: "albedo",
          type: .texture(name: "Texture2D"),
          category: "shaderResource",
          bindingType: "texture",
          index: 0
        ),
        ShaderBinding(
          name: "linearSampler",
          type: .sampler,
          category: "samplerState",
          bindingType: "sampler",
          index: 1
        ),
      ]
    )
  )

  #expect(source.contains("public static let albedo = 0"))
  #expect(source.contains("public static let linearSampler = 1"))
}

@Test func emitsEntryPointMetadata() throws {
  let source = try SwiftEmitter().emit(
    ShaderProgramModel(
      moduleName: "Entries",
      entryPoints: [
        ShaderEntryPoint(name: "vsMain", stage: .vertex),
        ShaderEntryPoint(name: "fsMain", stage: .fragment),
      ]
    )
  )

  #expect(source.contains("public static let vsMain"))
  #expect(source.contains("ShaderEntryPointMetadata"))
  #expect(source.contains("stage: \"vertex\""))
}
