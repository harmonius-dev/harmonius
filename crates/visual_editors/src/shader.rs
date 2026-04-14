//! Shader graph emission and offline compile hooks.

/// HLSL text bundle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HlslSource(pub String);

/// Minimal vertex shader emission for unit tests.
pub fn emit_shader_graph_main() -> HlslSource {
    HlslSource(
        "float4 main() : SV_Position { return float4(0,0,0,1); }\n".to_string(),
    )
}

/// Records a DXC invocation outcome without calling the external tool.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DxcCompileResult {
    pub ok: bool,
    pub stage: ShaderStage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaderStage {
    Vertex,
    Fragment,
    Compute,
}

/// Stub DXC compile: succeeds when source mentions `SV_Position`.
pub fn compile_with_dxc_stub(source: &HlslSource, stage: ShaderStage) -> DxcCompileResult {
    DxcCompileResult {
        ok: source.0.contains("SV_Position"),
        stage,
    }
}

/// PBR material template header.
pub fn material_pbr_template() -> HlslSource {
    HlslSource(
        "// PBR template\nfloat3 albedo; float metallic; float roughness;\n".to_string(),
    )
}
