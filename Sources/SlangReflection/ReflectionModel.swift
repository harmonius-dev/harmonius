import Foundation

public struct ShaderProgramModel: Codable, Equatable, Sendable {
  public var moduleName: String
  public var structs: [ShaderStruct]
  public var bindings: [ShaderBinding]
  public var entryPoints: [ShaderEntryPoint]
  public var parameterBlocks: [ShaderParameterBlock]

  public init(
    moduleName: String,
    structs: [ShaderStruct] = [],
    bindings: [ShaderBinding] = [],
    entryPoints: [ShaderEntryPoint] = [],
    parameterBlocks: [ShaderParameterBlock] = []
  ) {
    self.moduleName = moduleName
    self.structs = structs
    self.bindings = bindings
    self.entryPoints = entryPoints
    self.parameterBlocks = parameterBlocks
  }
}

public struct ShaderStruct: Codable, Equatable, Sendable {
  public var name: String
  public var fields: [ShaderField]
  public var size: Int
  public var alignment: Int

  public init(
    name: String,
    fields: [ShaderField],
    size: Int,
    alignment: Int
  ) {
    self.name = name
    self.fields = fields
    self.size = size
    self.alignment = alignment
  }
}

public struct ShaderField: Codable, Equatable, Sendable {
  public var name: String
  public var type: ShaderType
  public var offset: Int
  public var size: Int
  public var alignment: Int

  public init(
    name: String,
    type: ShaderType,
    offset: Int,
    size: Int,
    alignment: Int
  ) {
    self.name = name
    self.type = type
    self.offset = offset
    self.size = size
    self.alignment = alignment
  }
}

public indirect enum ShaderType: Codable, Equatable, Sendable {
  case scalar(ScalarKind)
  case vector(element: ScalarKind, count: Int)
  case matrix(element: ScalarKind, rows: Int, columns: Int)
  case array(element: ShaderType, count: Int, stride: Int)
  case `struct`(String)
  case constantBuffer(element: ShaderType)
  case parameterBlock(element: ShaderType)
  case texture(name: String)
  case sampler
  case resource(name: String, element: ShaderType?)
  case unsupported(String)

  private enum CodingKeys: String, CodingKey {
    case kind
    case element
    case count
    case stride
    case rows
    case columns
    case name
  }

  private enum Kind: String, Codable {
    case scalar
    case vector
    case matrix
    case array
    case `struct`
    case constantBuffer
    case parameterBlock
    case texture
    case sampler
    case resource
    case unsupported
  }

  public init(from decoder: Decoder) throws {
    let container = try decoder.container(keyedBy: CodingKeys.self)
    let kind = try container.decode(Kind.self, forKey: .kind)

    switch kind {
    case .scalar:
      self = .scalar(try container.decode(ScalarKind.self, forKey: .element))
    case .vector:
      self = .vector(
        element: try container.decode(ScalarKind.self, forKey: .element),
        count: try container.decode(Int.self, forKey: .count)
      )
    case .matrix:
      self = .matrix(
        element: try container.decode(ScalarKind.self, forKey: .element),
        rows: try container.decode(Int.self, forKey: .rows),
        columns: try container.decode(Int.self, forKey: .columns)
      )
    case .array:
      self = .array(
        element: try container.decode(ShaderType.self, forKey: .element),
        count: try container.decode(Int.self, forKey: .count),
        stride: try container.decode(Int.self, forKey: .stride)
      )
    case .struct:
      self = .struct(try container.decode(String.self, forKey: .name))
    case .constantBuffer:
      self = .constantBuffer(
        element: try container.decode(ShaderType.self, forKey: .element)
      )
    case .parameterBlock:
      self = .parameterBlock(
        element: try container.decode(ShaderType.self, forKey: .element)
      )
    case .texture:
      self = .texture(name: try container.decode(String.self, forKey: .name))
    case .sampler:
      self = .sampler
    case .resource:
      self = .resource(
        name: try container.decode(String.self, forKey: .name),
        element: try container.decodeIfPresent(ShaderType.self, forKey: .element)
      )
    case .unsupported:
      self = .unsupported(try container.decode(String.self, forKey: .name))
    }
  }

  public func encode(to encoder: Encoder) throws {
    var container = encoder.container(keyedBy: CodingKeys.self)
    switch self {
    case .scalar(let element):
      try container.encode(Kind.scalar, forKey: .kind)
      try container.encode(element, forKey: .element)
    case .vector(let element, let count):
      try container.encode(Kind.vector, forKey: .kind)
      try container.encode(element, forKey: .element)
      try container.encode(count, forKey: .count)
    case .matrix(let element, let rows, let columns):
      try container.encode(Kind.matrix, forKey: .kind)
      try container.encode(element, forKey: .element)
      try container.encode(rows, forKey: .rows)
      try container.encode(columns, forKey: .columns)
    case .array(let element, let count, let stride):
      try container.encode(Kind.array, forKey: .kind)
      try container.encode(element, forKey: .element)
      try container.encode(count, forKey: .count)
      try container.encode(stride, forKey: .stride)
    case .struct(let name):
      try container.encode(Kind.struct, forKey: .kind)
      try container.encode(name, forKey: .name)
    case .constantBuffer(let element):
      try container.encode(Kind.constantBuffer, forKey: .kind)
      try container.encode(element, forKey: .element)
    case .parameterBlock(let element):
      try container.encode(Kind.parameterBlock, forKey: .kind)
      try container.encode(element, forKey: .element)
    case .texture(let name):
      try container.encode(Kind.texture, forKey: .kind)
      try container.encode(name, forKey: .name)
    case .sampler:
      try container.encode(Kind.sampler, forKey: .kind)
    case .resource(let name, let element):
      try container.encode(Kind.resource, forKey: .kind)
      try container.encode(name, forKey: .name)
      try container.encodeIfPresent(element, forKey: .element)
    case .unsupported(let name):
      try container.encode(Kind.unsupported, forKey: .kind)
      try container.encode(name, forKey: .name)
    }
  }
}

public enum ScalarKind: String, Codable, Equatable, Sendable {
  case bool
  case int32
  case uint32
  case int64
  case uint64
  case float16
  case float32
  case float64
  case unknown
}

public struct ShaderBinding: Codable, Equatable, Sendable {
  public var name: String
  public var type: ShaderType
  public var category: String
  public var bindingType: String
  public var index: Int
  public var space: Int
  public var count: Int

  public init(
    name: String,
    type: ShaderType,
    category: String,
    bindingType: String,
    index: Int,
    space: Int = 0,
    count: Int = 1
  ) {
    self.name = name
    self.type = type
    self.category = category
    self.bindingType = bindingType
    self.index = index
    self.space = space
    self.count = count
  }
}

public struct ShaderParameterBlock: Codable, Equatable, Sendable {
  public var name: String
  public var type: ShaderType
  public var index: Int
  public var space: Int

  public init(
    name: String,
    type: ShaderType,
    index: Int,
    space: Int = 0
  ) {
    self.name = name
    self.type = type
    self.index = index
    self.space = space
  }
}

public struct ShaderEntryPoint: Codable, Equatable, Sendable {
  public var name: String
  public var stage: ShaderStage
  public var parameters: [ShaderEntryPointParameter]

  public init(
    name: String,
    stage: ShaderStage,
    parameters: [ShaderEntryPointParameter] = []
  ) {
    self.name = name
    self.stage = stage
    self.parameters = parameters
  }
}

public struct ShaderEntryPointParameter: Codable, Equatable, Sendable {
  public var name: String
  public var type: ShaderType
  public var category: String
  public var offset: Int

  public init(
    name: String,
    type: ShaderType,
    category: String,
    offset: Int
  ) {
    self.name = name
    self.type = type
    self.category = category
    self.offset = offset
  }
}

public enum ShaderStage: String, Codable, Equatable, Sendable {
  case vertex
  case fragment
  case compute
  case hull
  case domain
  case geometry
  case mesh
  case amplification
  case unknown
}
