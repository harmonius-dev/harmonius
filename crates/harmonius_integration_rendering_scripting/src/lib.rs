//! Rendering ↔ scripting integration contracts for IR-3.5.x.
//!
//! Implements the API surface in
//! `docs/design/integration/rendering-scripting.md`: shader permutation keys,
//! material HLSL codegen, pipeline state handles, and rkyv-serialized GPU
//! payloads (`MaterialShaderOutput`, `CompiledEffect`).

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

mod codegen;
mod effect;
mod graph;
mod hlsl;
mod permutation;
mod pipeline;
mod shader_types;

pub use codegen::codegen_hlsl;
pub use effect::{AttributeLayout, CompiledEffect, CompiledKernel, OutputMode, ParamBufferLayout};
pub use graph::{GraphNodeKind, NodeId, ShaderGraphIr};
pub use hlsl::{HlslSourceHandle, MaterialShaderCache, MaterialShaderOutput};
pub use permutation::{permutation_lookup_index, permutation_table_from_keys};
pub use pipeline::{PipelineStateDesc, PipelineStateHandle, PipelineStateTable};
pub use shader_types::{
    CompileTarget, PermutationKey, PermutationSpan, RenderPath, ShaderFeatures, ShaderProfile,
    ShadingModel,
};
pub use smol_str::SmolStr;

/// Offline `(name, value)` pair for `dxc` `-D` defines (IR-3.5.1).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DefinePair {
    /// Macro name passed to the compiler.
    pub name: SmolStr,
    /// Macro expansion text.
    pub value: SmolStr,
}

/// Span into an offline arena holding [`DefinePair`] rows (IR-3.5.1).
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ArenaRef<T> {
    /// Byte offset of the first `DefinePair` in the arena.
    pub offset: u32,
    /// Number of `DefinePair` records in the span.
    pub len: u32,
    /// Marks logical type of the span for type safety at call sites.
    pub _marker: core::marker::PhantomData<T>,
}

/// Main-thread `dxc` invocation input (IR-3.5.2); never archived.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShaderCompileRequest {
    /// HLSL source handle resolved against [`MaterialShaderCache`].
    pub hlsl: HlslSourceHandle,
    /// Entry point function name.
    pub entry_point: SmolStr,
    /// Target shader model profile.
    pub profile: ShaderProfile,
    /// Macro defines stored in an offline arena.
    pub defines: ArenaRef<DefinePair>,
    /// Request DXIL bytecode output.
    pub output_dxil: bool,
    /// Request SPIR-V bytecode output.
    pub output_spirv: bool,
}
