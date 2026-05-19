//! SPIR-V shader module loading.

use ash::vk;

use crate::backend::vulkan::{VulkanContext, VulkanError};

/// Loaded SPIR-V shader module.
pub struct ShaderModule {
    pub(crate) module: vk::ShaderModule,
}

impl ShaderModule {
    /// Create a shader module from SPIR-V words.
    pub fn from_spirv(ctx: &VulkanContext, spirv: &[u32]) -> Result<Self, VulkanError> {
        if spirv.first() != Some(&0x0723_0203) {
            return Err(VulkanError::Api(
                "invalid SPIR-V: missing magic number 0x07230203".into(),
            ));
        }
        let create_info = vk::ShaderModuleCreateInfo::default().code(spirv);
        let module = unsafe {
            ctx.device()
                .create_shader_module(&create_info, None)
                .map_err(|e| VulkanError::Api(e.to_string()))?
        };
        Ok(Self { module })
    }
}
