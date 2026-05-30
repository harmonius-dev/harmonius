#include "SlangReflectionBridge.h"

#include <slang-com-ptr.h>
#include <slang.h>

#include <algorithm>
#include <cstdlib>
#include <cstring>
#include <filesystem>
#include <fstream>
#include <set>
#include <sstream>
#include <string>
#include <string_view>
#include <vector>

namespace {

char* copyCString(const std::string& value) {
  auto* copy = static_cast<char*>(std::malloc(value.size() + 1));
  if (!copy) {
    return nullptr;
  }
  std::memcpy(copy, value.c_str(), value.size() + 1);
  return copy;
}

std::string blobString(slang::IBlob* blob) {
  if (!blob) {
    return {};
  }
  const auto* bytes = static_cast<const char*>(blob->getBufferPointer());
  return std::string(bytes, bytes + blob->getBufferSize());
}

HSRSlangReflectionOutput makeOutput(
  int status,
  const std::string& json,
  const std::string& diagnostics
) {
  return HSRSlangReflectionOutput{
    status,
    json.empty() ? nullptr : copyCString(json),
    diagnostics.empty() ? nullptr : copyCString(diagnostics),
  };
}

void appendQuoted(std::string& output, std::string_view value) {
  output.push_back('"');
  for (char character : value) {
    switch (character) {
    case '\\':
      output += "\\\\";
      break;
    case '"':
      output += "\\\"";
      break;
    case '\n':
      output += "\\n";
      break;
    case '\r':
      output += "\\r";
      break;
    case '\t':
      output += "\\t";
      break;
    default:
      output.push_back(character);
      break;
    }
  }
  output.push_back('"');
}

void appendKey(std::string& output, std::string_view key) {
  appendQuoted(output, key);
  output.push_back(':');
}

std::string readFile(const std::filesystem::path& path) {
  std::ifstream stream(path, std::ios::binary);
  std::ostringstream buffer;
  buffer << stream.rdbuf();
  return buffer.str();
}

bool writeBlob(const std::filesystem::path& path, slang::IBlob* blob) {
  if (!blob) {
    return false;
  }
  std::filesystem::create_directories(path.parent_path());
  std::ofstream stream(path, std::ios::binary);
  if (!stream) {
    return false;
  }
  stream.write(
    static_cast<const char*>(blob->getBufferPointer()),
    static_cast<std::streamsize>(blob->getBufferSize())
  );
  return static_cast<bool>(stream);
}

int checkedSize(size_t value) {
  if (value == SLANG_UNKNOWN_SIZE || value == SLANG_UNBOUNDED_SIZE) {
    return -1;
  }
  return static_cast<int>(value);
}

const char* scalarName(slang::TypeReflection::ScalarType type) {
  switch (type) {
  case slang::TypeReflection::Bool:
    return "bool";
  case slang::TypeReflection::Int32:
    return "int32";
  case slang::TypeReflection::UInt32:
    return "uint32";
  case slang::TypeReflection::Int64:
    return "int64";
  case slang::TypeReflection::UInt64:
    return "uint64";
  case slang::TypeReflection::Float16:
    return "float16";
  case slang::TypeReflection::Float32:
    return "float32";
  case slang::TypeReflection::Float64:
    return "float64";
  default:
    return "unknown";
  }
}

const char* stageName(SlangStage stage) {
  switch (stage) {
  case SLANG_STAGE_VERTEX:
    return "vertex";
  case SLANG_STAGE_FRAGMENT:
    return "fragment";
  case SLANG_STAGE_COMPUTE:
    return "compute";
  case SLANG_STAGE_HULL:
    return "hull";
  case SLANG_STAGE_DOMAIN:
    return "domain";
  case SLANG_STAGE_GEOMETRY:
    return "geometry";
  case SLANG_STAGE_MESH:
    return "mesh";
  case SLANG_STAGE_AMPLIFICATION:
    return "amplification";
  default:
    return "unknown";
  }
}

const char* categoryName(slang::ParameterCategory category) {
  switch (category) {
  case slang::ParameterCategory::ConstantBuffer:
    return "constantBuffer";
  case slang::ParameterCategory::ShaderResource:
    return "shaderResource";
  case slang::ParameterCategory::UnorderedAccess:
    return "unorderedAccess";
  case slang::ParameterCategory::SamplerState:
    return "samplerState";
  case slang::ParameterCategory::Uniform:
    return "uniform";
  case slang::ParameterCategory::DescriptorTableSlot:
    return "descriptorTableSlot";
  case slang::ParameterCategory::MetalArgumentBufferElement:
    return "metalArgumentBufferElement";
  default:
    return "unknown";
  }
}

const char* bindingTypeName(slang::BindingType type) {
  switch (type) {
  case slang::BindingType::Sampler:
    return "sampler";
  case slang::BindingType::Texture:
    return "texture";
  case slang::BindingType::ConstantBuffer:
    return "constantBuffer";
  case slang::BindingType::ParameterBlock:
    return "parameterBlock";
  case slang::BindingType::TypedBuffer:
    return "typedBuffer";
  case slang::BindingType::RawBuffer:
    return "rawBuffer";
  case slang::BindingType::CombinedTextureSampler:
    return "combinedTextureSampler";
  default:
    return "unknown";
  }
}

const char* typeKindName(slang::TypeReflection::Kind kind) {
  switch (kind) {
  case slang::TypeReflection::Kind::ConstantBuffer:
    return "constantBuffer";
  case slang::TypeReflection::Kind::ParameterBlock:
    return "parameterBlock";
  case slang::TypeReflection::Kind::Resource:
    return "resource";
  case slang::TypeReflection::Kind::SamplerState:
    return "sampler";
  case slang::TypeReflection::Kind::ShaderStorageBuffer:
    return "shaderStorageBuffer";
  case slang::TypeReflection::Kind::TextureBuffer:
    return "textureBuffer";
  default:
    return "unknown";
  }
}

int bindingIndex(slang::VariableLayoutReflection* parameter) {
  auto* typeLayout = parameter ? parameter->getTypeLayout() : nullptr;
  if (!parameter || !typeLayout) {
    return 0;
  }
  const slang::ParameterCategory categories[] = {
    slang::ParameterCategory::ConstantBuffer,
    slang::ParameterCategory::ShaderResource,
    slang::ParameterCategory::UnorderedAccess,
    slang::ParameterCategory::SamplerState,
    slang::ParameterCategory::DescriptorTableSlot,
    slang::ParameterCategory::MetalArgumentBufferElement,
  };
  int bestOffset = -1;
  for (auto category : categories) {
    auto offset = parameter->getOffset(category);
    if (offset != SLANG_UNKNOWN_SIZE && offset != SLANG_UNBOUNDED_SIZE) {
      bestOffset = std::max(bestOffset, static_cast<int>(offset));
    }
  }
  if (bestOffset >= 0) {
    return bestOffset;
  }
  return static_cast<int>(parameter->getBindingIndex());
}

int bindingSpace(slang::VariableLayoutReflection* parameter) {
  auto* typeLayout = parameter ? parameter->getTypeLayout() : nullptr;
  if (!parameter || !typeLayout) {
    return 0;
  }
  const slang::ParameterCategory categories[] = {
    slang::ParameterCategory::ConstantBuffer,
    slang::ParameterCategory::ShaderResource,
    slang::ParameterCategory::UnorderedAccess,
    slang::ParameterCategory::SamplerState,
    slang::ParameterCategory::DescriptorTableSlot,
    slang::ParameterCategory::MetalArgumentBufferElement,
  };
  int bestSpace = -1;
  for (auto category : categories) {
    auto space = parameter->getBindingSpace(category);
    if (space != SLANG_UNKNOWN_SIZE && space != SLANG_UNBOUNDED_SIZE) {
      bestSpace = std::max(bestSpace, static_cast<int>(space));
    }
  }
  if (bestSpace >= 0) {
    return bestSpace;
  }
  return static_cast<int>(parameter->getBindingSpace());
}

std::string safeName(const char* name, std::string_view fallback) {
  if (name && name[0] != '\0') {
    return name;
  }
  return std::string(fallback);
}

void appendTypeJSON(std::string& output, slang::TypeLayoutReflection* typeLayout) {
  if (!typeLayout) {
    output += "{\"kind\":\"unsupported\",\"name\":\"missing\"}";
    return;
  }

  switch (typeLayout->getKind()) {
  case slang::TypeReflection::Kind::Scalar:
    output += "{\"kind\":\"scalar\",\"element\":";
    appendQuoted(output, scalarName(typeLayout->getScalarType()));
    output.push_back('}');
    return;
  case slang::TypeReflection::Kind::Vector:
    output += "{\"kind\":\"vector\",\"element\":";
    appendQuoted(output, scalarName(typeLayout->getScalarType()));
    output += ",\"count\":";
    output += std::to_string(typeLayout->getType()->getElementCount());
    output.push_back('}');
    return;
  case slang::TypeReflection::Kind::Matrix:
    output += "{\"kind\":\"matrix\",\"element\":";
    appendQuoted(output, scalarName(typeLayout->getScalarType()));
    output += ",\"rows\":";
    output += std::to_string(typeLayout->getRowCount());
    output += ",\"columns\":";
    output += std::to_string(typeLayout->getColumnCount());
    output.push_back('}');
    return;
  case slang::TypeReflection::Kind::Array:
    output += "{\"kind\":\"array\",\"element\":";
    appendTypeJSON(output, typeLayout->getElementTypeLayout());
    output += ",\"count\":";
    output += std::to_string(typeLayout->getElementCount());
    output += ",\"stride\":";
    output += std::to_string(
      checkedSize(typeLayout->getElementStride(SLANG_PARAMETER_CATEGORY_UNIFORM))
    );
    output.push_back('}');
    return;
  case slang::TypeReflection::Kind::Struct:
    output += "{\"kind\":\"struct\",\"name\":";
    appendQuoted(output, safeName(typeLayout->getName(), "AnonymousStruct"));
    output.push_back('}');
    return;
  case slang::TypeReflection::Kind::ConstantBuffer:
    output += "{\"kind\":\"constantBuffer\",\"element\":";
    appendTypeJSON(output, typeLayout->getElementTypeLayout());
    output.push_back('}');
    return;
  case slang::TypeReflection::Kind::ParameterBlock:
    output += "{\"kind\":\"parameterBlock\",\"element\":";
    appendTypeJSON(output, typeLayout->getElementTypeLayout());
    output.push_back('}');
    return;
  case slang::TypeReflection::Kind::SamplerState:
    output += "{\"kind\":\"sampler\"}";
    return;
  case slang::TypeReflection::Kind::Resource:
    output += "{\"kind\":\"texture\",\"name\":";
    appendQuoted(output, safeName(typeLayout->getName(), "Texture"));
    output.push_back('}');
    return;
  case slang::TypeReflection::Kind::ShaderStorageBuffer:
  case slang::TypeReflection::Kind::TextureBuffer:
    output += "{\"kind\":\"resource\",\"name\":";
    appendQuoted(output, typeKindName(typeLayout->getKind()));
    output += ",\"element\":";
    appendTypeJSON(output, typeLayout->getElementTypeLayout());
    output.push_back('}');
    return;
  default:
    output += "{\"kind\":\"unsupported\",\"name\":";
    appendQuoted(output, safeName(typeLayout->getName(), "unsupported"));
    output.push_back('}');
    return;
  }
}

void appendFieldJSON(std::string& output, slang::VariableLayoutReflection* field) {
  auto* typeLayout = field ? field->getTypeLayout() : nullptr;
  output.push_back('{');
  appendKey(output, "name");
  appendQuoted(output, safeName(field ? field->getName() : nullptr, "field"));
  output += ",\"type\":";
  appendTypeJSON(output, typeLayout);
  output += ",\"offset\":";
  output += std::to_string(
    checkedSize(field ? field->getOffset(slang::ParameterCategory::Uniform) : 0)
  );
  output += ",\"size\":";
  output += std::to_string(
    checkedSize(typeLayout ? typeLayout->getSize(slang::ParameterCategory::Uniform) : 0)
  );
  output += ",\"alignment\":";
  output += std::to_string(
    checkedSize(typeLayout ? typeLayout->getAlignment(slang::ParameterCategory::Uniform) : 1)
  );
  output.push_back('}');
}

void appendStructJSON(
  std::string& output,
  slang::ProgramLayout* programLayout,
  slang::DeclReflection* declaration
) {
  auto* type = declaration->getType();
  auto* typeLayout = programLayout->getTypeLayout(
    type,
    slang::LayoutRules::DefaultStructuredBuffer
  );
  output.push_back('{');
  appendKey(output, "name");
  appendQuoted(output, safeName(declaration->getName(), "AnonymousStruct"));
  output += ",\"fields\":[";
  if (typeLayout) {
    for (unsigned index = 0; index < typeLayout->getFieldCount(); ++index) {
      if (index > 0) {
        output.push_back(',');
      }
      appendFieldJSON(output, typeLayout->getFieldByIndex(index));
    }
  }
  output += "],\"size\":";
  output += std::to_string(
    checkedSize(typeLayout ? typeLayout->getSize(slang::ParameterCategory::Uniform) : 0)
  );
  output += ",\"alignment\":";
  output += std::to_string(
    checkedSize(typeLayout ? typeLayout->getAlignment(slang::ParameterCategory::Uniform) : 1)
  );
  output.push_back('}');
}

void collectStructDecls(
  slang::DeclReflection* declaration,
  std::vector<slang::DeclReflection*>& structs,
  std::set<std::string>& seen
) {
  if (!declaration) {
    return;
  }
  if (declaration->getKind() == slang::DeclReflection::Kind::Struct) {
    auto name = safeName(declaration->getName(), "AnonymousStruct");
    if (seen.insert(name).second) {
      structs.push_back(declaration);
    }
  }
  for (unsigned index = 0; index < declaration->getChildrenCount(); ++index) {
    auto* child = declaration->getChild(index);
    auto kind = child ? child->getKind() : slang::DeclReflection::Kind::Unsupported;
    switch (kind) {
    case slang::DeclReflection::Kind::Module:
    case slang::DeclReflection::Kind::Namespace:
    case slang::DeclReflection::Kind::Generic:
    case slang::DeclReflection::Kind::Struct:
      collectStructDecls(child, structs, seen);
      break;
    default:
      break;
    }
  }
}

void appendStructsJSON(
  std::string& output,
  slang::ProgramLayout* programLayout,
  slang::DeclReflection* moduleReflection
) {
  std::vector<slang::DeclReflection*> structs;
  std::set<std::string> seen;
  collectStructDecls(moduleReflection, structs, seen);

  output.push_back('[');
  for (size_t index = 0; index < structs.size(); ++index) {
    if (index > 0) {
      output.push_back(',');
    }
    appendStructJSON(output, programLayout, structs[index]);
  }
  output.push_back(']');
}

void appendBindingJSON(std::string& output, slang::VariableLayoutReflection* parameter) {
  auto* typeLayout = parameter ? parameter->getTypeLayout() : nullptr;
  auto bindingType = typeLayout && typeLayout->getBindingRangeCount() > 0
    ? bindingTypeName(typeLayout->getBindingRangeType(0))
    : typeKindName(typeLayout ? typeLayout->getKind() : slang::TypeReflection::Kind::None);
  auto bindingCount = typeLayout && typeLayout->getBindingRangeCount() > 0
    ? typeLayout->getBindingRangeBindingCount(0)
    : 1;

  output.push_back('{');
  appendKey(output, "name");
  appendQuoted(output, safeName(parameter ? parameter->getName() : nullptr, "parameter"));
  output += ",\"type\":";
  appendTypeJSON(output, typeLayout);
  output += ",\"category\":";
  appendQuoted(output, categoryName(typeLayout ? typeLayout->getParameterCategory() : slang::None));
  output += ",\"bindingType\":";
  appendQuoted(output, bindingType);
  output += ",\"index\":";
  output += std::to_string(bindingIndex(parameter));
  output += ",\"space\":";
  output += std::to_string(bindingSpace(parameter));
  output += ",\"count\":";
  output += std::to_string(bindingCount > 0 ? bindingCount : 1);
  output.push_back('}');
}

void appendBindingsJSON(std::string& output, slang::ProgramLayout* programLayout) {
  output.push_back('[');
  for (unsigned index = 0; index < programLayout->getParameterCount(); ++index) {
    if (index > 0) {
      output.push_back(',');
    }
    appendBindingJSON(output, programLayout->getParameterByIndex(index));
  }
  output.push_back(']');
}

void appendEntryPointParameterJSON(
  std::string& output,
  slang::VariableLayoutReflection* parameter,
  int fallbackIndex
) {
  auto* typeLayout = parameter ? parameter->getTypeLayout() : nullptr;
  output.push_back('{');
  appendKey(output, "name");
  appendQuoted(
    output,
    safeName(
      parameter ? parameter->getName() : nullptr,
      "parameter" + std::to_string(fallbackIndex)
    )
  );
  output += ",\"type\":";
  appendTypeJSON(output, typeLayout);
  output += ",\"category\":";
  appendQuoted(output, categoryName(typeLayout ? typeLayout->getParameterCategory() : slang::None));
  output += ",\"offset\":";
  output += std::to_string(
    checkedSize(parameter ? parameter->getOffset(slang::ParameterCategory::Uniform) : 0)
  );
  output.push_back('}');
}

void appendEntryPointJSON(std::string& output, slang::EntryPointReflection* entryPoint) {
  output.push_back('{');
  appendKey(output, "name");
  appendQuoted(output, safeName(entryPoint ? entryPoint->getName() : nullptr, "entryPoint"));
  output += ",\"stage\":";
  appendQuoted(output, stageName(entryPoint ? entryPoint->getStage() : SLANG_STAGE_NONE));
  output += ",\"parameters\":[";
  if (entryPoint) {
    for (unsigned index = 0; index < entryPoint->getParameterCount(); ++index) {
      if (index > 0) {
        output.push_back(',');
      }
      appendEntryPointParameterJSON(
        output,
        entryPoint->getParameterByIndex(index),
        static_cast<int>(index)
      );
    }
  }
  output += "]}";
}

void appendEntryPointsJSON(std::string& output, slang::ProgramLayout* programLayout) {
  output.push_back('[');
  for (SlangUInt index = 0; index < programLayout->getEntryPointCount(); ++index) {
    if (index > 0) {
      output.push_back(',');
    }
    appendEntryPointJSON(output, programLayout->getEntryPointByIndex(index));
  }
  output.push_back(']');
}

void appendParameterBlocksJSON(std::string& output, slang::ProgramLayout* programLayout) {
  bool needsComma = false;
  output.push_back('[');
  for (unsigned index = 0; index < programLayout->getParameterCount(); ++index) {
    auto* parameter = programLayout->getParameterByIndex(index);
    auto* typeLayout = parameter ? parameter->getTypeLayout() : nullptr;
    if (!typeLayout || typeLayout->getKind() != slang::TypeReflection::Kind::ParameterBlock) {
      continue;
    }
    if (needsComma) {
      output.push_back(',');
    }
    needsComma = true;
    output.push_back('{');
    appendKey(output, "name");
    appendQuoted(output, safeName(parameter->getName(), "parameterBlock"));
    output += ",\"type\":";
    appendTypeJSON(output, typeLayout);
    output += ",\"index\":";
    output += std::to_string(bindingIndex(parameter));
    output += ",\"space\":";
    output += std::to_string(bindingSpace(parameter));
    output.push_back('}');
  }
  output.push_back(']');
}

std::string buildReflectionJSON(
  const std::string& moduleName,
  slang::ProgramLayout* programLayout,
  slang::DeclReflection* moduleReflection
) {
  std::string output;
  output.reserve(8192);
  output.push_back('{');
  appendKey(output, "moduleName");
  appendQuoted(output, moduleName);
  output += ",\"structs\":";
  appendStructsJSON(output, programLayout, moduleReflection);
  output += ",\"bindings\":";
  appendBindingsJSON(output, programLayout);
  output += ",\"entryPoints\":";
  appendEntryPointsJSON(output, programLayout);
  output += ",\"parameterBlocks\":";
  appendParameterBlocksJSON(output, programLayout);
  output.push_back('}');
  return output;
}

} // namespace

extern "C" HSRSlangReflectionOutput hsrCompileSlangReflection(
  const char* sourcePath,
  const char* metallibPath,
  const char* includeDirectory
) {
  if (!sourcePath || !metallibPath || !includeDirectory) {
    return makeOutput(1, {}, "missing source, metallib, or include path");
  }

  try {
    const std::filesystem::path source(sourcePath);
    const std::filesystem::path metallib(metallibPath);
    const auto moduleName = source.stem().string();
    const auto sourceText = readFile(source);
    if (sourceText.empty()) {
      return makeOutput(1, {}, "shader source is empty or unreadable");
    }

    Slang::ComPtr<slang::IGlobalSession> globalSession;
    auto result = slang::createGlobalSession(globalSession.writeRef());
    if (SLANG_FAILED(result)) {
      return makeOutput(1, {}, "failed to create Slang global session");
    }

    slang::TargetDesc target = {};
    target.format = SLANG_METAL_LIB;

    std::vector<std::string> searchPathStorage = {
      source.parent_path().string(),
      std::string(includeDirectory),
    };
    std::vector<const char*> searchPaths;
    for (const auto& path : searchPathStorage) {
      searchPaths.push_back(path.c_str());
    }

    slang::SessionDesc sessionDesc = {};
    sessionDesc.targets = &target;
    sessionDesc.targetCount = 1;
    sessionDesc.searchPaths = searchPaths.data();
    sessionDesc.searchPathCount = static_cast<SlangInt>(searchPaths.size());

    Slang::ComPtr<slang::ISession> session;
    result = globalSession->createSession(sessionDesc, session.writeRef());
    if (SLANG_FAILED(result)) {
      return makeOutput(1, {}, "failed to create Slang session");
    }

    Slang::ComPtr<slang::IBlob> diagnostics;
    auto* module = session->loadModuleFromSourceString(
      moduleName.c_str(),
      source.string().c_str(),
      sourceText.c_str(),
      diagnostics.writeRef()
    );
    if (!module) {
      return makeOutput(1, {}, blobString(diagnostics.get()));
    }

    std::vector<Slang::ComPtr<slang::IEntryPoint>> entryPoints;
    std::vector<slang::IComponentType*> components = {module};
    const auto entryPointCount = module->getDefinedEntryPointCount();
    for (SlangInt32 index = 0; index < entryPointCount; ++index) {
      Slang::ComPtr<slang::IEntryPoint> entryPoint;
      diagnostics.setNull();
      result = module->getDefinedEntryPoint(index, entryPoint.writeRef());
      if (SLANG_FAILED(result)) {
        return makeOutput(1, {}, "failed to get Slang entry point");
      }
      components.push_back(entryPoint.get());
      entryPoints.push_back(entryPoint);
    }

    Slang::ComPtr<slang::IComponentType> composite;
    diagnostics.setNull();
    result = session->createCompositeComponentType(
      components.data(),
      static_cast<SlangInt>(components.size()),
      composite.writeRef(),
      diagnostics.writeRef()
    );
    if (SLANG_FAILED(result)) {
      return makeOutput(1, {}, blobString(diagnostics.get()));
    }

    Slang::ComPtr<slang::IComponentType> linkedProgram;
    diagnostics.setNull();
    result = composite->link(linkedProgram.writeRef(), diagnostics.writeRef());
    if (SLANG_FAILED(result)) {
      return makeOutput(1, {}, blobString(diagnostics.get()));
    }

    diagnostics.setNull();
    auto* programLayout = linkedProgram->getLayout(0, diagnostics.writeRef());
    if (!programLayout) {
      return makeOutput(1, {}, blobString(diagnostics.get()));
    }

    Slang::ComPtr<slang::IBlob> targetCode;
    diagnostics.setNull();
    result = linkedProgram->getTargetCode(0, targetCode.writeRef(), diagnostics.writeRef());
    if (SLANG_FAILED(result)) {
      return makeOutput(1, {}, blobString(diagnostics.get()));
    }
    if (!writeBlob(metallib, targetCode.get())) {
      return makeOutput(1, {}, "failed to write Metal library");
    }

    auto json = buildReflectionJSON(moduleName, programLayout, module->getModuleReflection());
    return makeOutput(0, json, blobString(diagnostics.get()));
  } catch (const std::exception& error) {
    return makeOutput(1, {}, error.what());
  }
}

extern "C" void hsrFreeSlangReflectionOutput(HSRSlangReflectionOutput output) {
  std::free(const_cast<char*>(output.json));
  std::free(const_cast<char*>(output.diagnostics));
}
