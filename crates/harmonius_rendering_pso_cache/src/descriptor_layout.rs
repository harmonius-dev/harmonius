use crate::cache_error::CacheError;

/// A single shader resource binding inferred from bytecode.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ShaderBinding {
    /// Binding slot index.
    pub slot: u32,
    /// Descriptor count at the slot.
    pub count: u32,
}

/// Descriptor layout inferred once per shader bytecode blob.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DescriptorLayout {
    /// Ordered bindings.
    pub bindings: Vec<ShaderBinding>,
    /// Push constant range size in 32-bit words.
    pub push_constants: u32,
}

/// Shader container format for reflection selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShaderApi {
    /// Microsoft DXIL container.
    Dxil,
    /// Khronos SPIR-V module.
    Spirv,
    /// Apple Metal library.
    Metal,
}

/// Infers descriptor layout from bytecode using the active API.
///
/// This is a deterministic stub used by unit tests with sentinel fixtures; GPU
/// drivers are not consulted.
pub fn infer_descriptor_layout(
    bytecode: &[u8],
    api: ShaderApi,
) -> Result<DescriptorLayout, CacheError> {
    if bytecode.is_empty() {
        return Err(CacheError::Corrupted("empty bytecode"));
    }
    match api {
        ShaderApi::Dxil => infer_dxil(bytecode),
        ShaderApi::Spirv => infer_spirv(bytecode),
        ShaderApi::Metal => infer_metal(bytecode),
    }
}

fn infer_dxil(bytecode: &[u8]) -> Result<DescriptorLayout, CacheError> {
    if bytecode.starts_with(b"DXIL_TEST") {
        let slots = bytecode.get(9).copied().unwrap_or(0) as u32;
        let mut bindings = Vec::new();
        for slot in 0..slots {
            bindings.push(ShaderBinding { slot, count: 1 });
        }
        let push = bytecode.get(10).copied().unwrap_or(0) as u32;
        return Ok(DescriptorLayout {
            bindings,
            push_constants: push,
        });
    }
    Err(CacheError::Corrupted("unknown dxil fixture"))
}

fn infer_spirv(bytecode: &[u8]) -> Result<DescriptorLayout, CacheError> {
    if bytecode.len() >= 4 && bytecode[0..4] == [0x03, 0x02, 0x23, 0x07] {
        let slots = bytecode.get(4).copied().unwrap_or(0) as u32;
        let mut bindings = Vec::new();
        for slot in 0..slots {
            bindings.push(ShaderBinding { slot, count: 1 });
        }
        let push = bytecode.get(5).copied().unwrap_or(0) as u32;
        return Ok(DescriptorLayout {
            bindings,
            push_constants: push,
        });
    }
    Err(CacheError::Corrupted("unknown spirv fixture"))
}

fn infer_metal(bytecode: &[u8]) -> Result<DescriptorLayout, CacheError> {
    const PREFIX: &[u8] = b"MTL_SENT";
    if bytecode.starts_with(PREFIX) {
        let slots = bytecode
            .get(PREFIX.len())
            .copied()
            .unwrap_or(0) as u32;
        let mut bindings = Vec::new();
        for slot in 0..slots {
            bindings.push(ShaderBinding { slot, count: 1 });
        }
        let push = bytecode
            .get(PREFIX.len().saturating_add(1))
            .copied()
            .unwrap_or(0) as u32;
        return Ok(DescriptorLayout {
            bindings,
            push_constants: push,
        });
    }
    Err(CacheError::Corrupted("unknown metal fixture"))
}
