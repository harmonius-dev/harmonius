use std::path::PathBuf;

use crate::{BufferHandle, ExecutionPlan, ShaderHandle};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResourceData {
    path: PathBuf,
    handle: BufferHandle,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShaderData {
    path: PathBuf,
    handle: ShaderHandle,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BackendMessage {
    LoadResourceData(ResourceData),
    PrepareShaderData(ShaderData),
    SubmitFrame(ExecutionPlan),
}
