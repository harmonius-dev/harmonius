//! rkyv-serialized GPU asset contracts (IR-5.2.1–IR-5.2.5).

use rkyv::{Archive, Deserialize, Serialize};

/// Target compilation platform.
#[derive(Archive, Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[rkyv(derive(Copy, Clone, Debug, Eq, PartialEq))]
#[repr(u8)]
pub enum TargetPlatform {
    WindowsD3D12 = 0,
    LinuxVulkan = 1,
    AndroidVulkan = 2,
    MacOsMetal = 3,
    IosMetal = 4,
}

/// Shader stage.
#[derive(Archive, Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[rkyv(derive(Copy, Clone, Debug, Eq, PartialEq))]
#[repr(u8)]
pub enum ShaderStage {
    Vertex = 0,
    Pixel = 1,
    Compute = 2,
    Mesh = 3,
    Amplification = 4,
}

/// GPU texture format.
#[derive(Archive, Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[rkyv(derive(Copy, Clone, Debug, Eq, PartialEq))]
#[repr(u8)]
pub enum GpuTextureFormat {
    Bc7Unorm = 0,
    Astc4x4Unorm = 1,
    Etc2Rgba8 = 2,
    Rgba8UnormFallback = 3,
}

/// Texture dimensionality.
#[derive(Archive, Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[rkyv(derive(Copy, Clone, Debug, Eq, PartialEq))]
#[repr(u8)]
pub enum TextureDimension {
    Tex2d = 0,
    Tex2dArray = 1,
    TexCube = 2,
    Tex3d = 3,
}

/// Single input/output parameter in shader reflection.
#[derive(Archive, Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[rkyv(derive(Copy, Clone, Debug, Eq, PartialEq), attr(repr(C)))]
#[repr(C)]
pub struct ShaderParam {
    pub name_hash: u64,
    pub register: u32,
    pub space: u32,
    pub component_count: u8,
}

/// Constant buffer binding descriptor.
#[derive(Archive, Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[rkyv(derive(Copy, Clone, Debug, Eq, PartialEq), attr(repr(C)))]
#[repr(C)]
pub struct CBufferBinding {
    pub name_hash: u64,
    pub register: u32,
    pub space: u32,
    pub size_bytes: u32,
}

/// Texture/UAV/Sampler binding descriptor.
#[derive(Archive, Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[rkyv(derive(Copy, Clone, Debug, Eq, PartialEq), attr(repr(C)))]
#[repr(C)]
pub struct TextureBinding {
    pub name_hash: u64,
    pub register: u32,
    pub space: u32,
    pub dimension: TextureDimension,
}

/// Reflection metadata consumed by the render pipeline.
#[derive(Archive, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[rkyv(derive(Debug, Eq, PartialEq), attr(repr(C, align(8))))]
#[repr(C, align(8))]
pub struct ShaderReflection {
    pub input_params: Vec<ShaderParam>,
    pub output_params: Vec<ShaderParam>,
    pub cbuffer_bindings: Vec<CBufferBinding>,
    pub texture_bindings: Vec<TextureBinding>,
    pub sampler_bindings: Vec<TextureBinding>,
    pub uav_bindings: Vec<TextureBinding>,
    pub push_constant_size: u32,
}

/// Shader compilation output stored in CAS.
#[derive(Archive, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[rkyv(derive(Debug, Eq, PartialEq), attr(repr(C, align(16))))]
#[repr(C, align(16))]
pub struct CompiledShader {
    pub source_hash: [u8; 32],
    pub platform: TargetPlatform,
    pub stage: ShaderStage,
    pub bytecode: Vec<u8>,
    pub reflection: ShaderReflection,
}

/// Axis-aligned bounds for a meshlet.
#[derive(Archive, Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
#[rkyv(derive(Copy, Clone, Debug, PartialEq), attr(repr(C)))]
#[repr(C)]
pub struct MeshletBounds {
    pub min: [f32; 3],
    pub max: [f32; 3],
}

/// Normal cone for meshlet backface culling.
#[derive(Archive, Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
#[rkyv(derive(Copy, Clone, Debug, PartialEq))]
#[repr(C)]
pub struct NormalCone {
    pub axis: [f32; 3],
    pub half_angle: f32,
}

/// GPU-ready meshlet buffer produced by `MeshProcessor`.
#[derive(Archive, Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
#[rkyv(derive(Copy, Clone, Debug, PartialEq), attr(repr(C)))]
#[repr(C)]
pub struct BakedMeshlet {
    pub vertex_offset: u32,
    pub vertex_count: u8,
    pub triangle_offset: u32,
    pub triangle_count: u8,
    pub bounds: MeshletBounds,
    pub normal_cone: NormalCone,
}

/// Compressed texture ready for GPU upload.
///
/// `data` is declared first so the rkyv archived layout keeps the payload slice
/// 16-byte aligned for BCn/ASTC loads (IR-5.2.4); other fields follow in a stable `repr(C)` block.
#[derive(Archive, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[rkyv(derive(Debug, Eq, PartialEq), attr(repr(C, align(16))))]
#[repr(C, align(16))]
pub struct BakedTexture {
    pub data: Vec<u8>,
    pub format: GpuTextureFormat,
    pub height: u32,
    pub mip_count: u8,
    pub mip_offsets: Vec<u64>,
    pub width: u32,
}

/// Placeholder vertex layout (filled by render pipeline integration).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VertexLayout;

/// Placeholder blend state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BlendState;

/// Placeholder depth/stencil state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DepthStencilState;

/// Placeholder rasterizer state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RasterizerState;

#[cfg(test)]
mod align_tests {
    use rkyv::access;
    use rkyv::rancor::Error;
    use rkyv::to_bytes;

    use super::*;

    fn empty_reflection() -> ShaderReflection {
        ShaderReflection {
            cbuffer_bindings: Vec::new(),
            input_params: Vec::new(),
            output_params: Vec::new(),
            push_constant_size: 0,
            sampler_bindings: Vec::new(),
            texture_bindings: Vec::new(),
            uav_bindings: Vec::new(),
        }
    }

    #[test]
    fn tc_ir_5_2_u1_rkyv_align_baked_texture_data() {
        let tex = BakedTexture {
            data: vec![0_u8; 64],
            format: GpuTextureFormat::Bc7Unorm,
            height: 4,
            mip_count: 1,
            mip_offsets: vec![0_u64],
            width: 4,
        };
        let bytes = to_bytes::<Error>(&tex).expect("serialize");
        let archived = access::<ArchivedBakedTexture, Error>(bytes.as_slice()).expect("access");
        let ptr = archived.data.as_ptr() as usize;
        assert_eq!(ptr % 16, 0, "BakedTexture.data must be 16-byte aligned");
    }

    #[test]
    fn tc_ir_5_2_u2_rkyv_align_compiled_shader_bytecode() {
        let sh = CompiledShader {
            bytecode: vec![0_u8; 64],
            platform: TargetPlatform::WindowsD3D12,
            reflection: empty_reflection(),
            source_hash: [1; 32],
            stage: ShaderStage::Vertex,
        };
        let bytes = to_bytes::<Error>(&sh).expect("serialize");
        let archived = access::<ArchivedCompiledShader, Error>(bytes.as_slice()).expect("access");
        let ptr = archived.bytecode.as_ptr() as usize;
        assert_eq!(
            ptr % 16,
            0,
            "CompiledShader.bytecode must be 16-byte aligned"
        );
    }
}
