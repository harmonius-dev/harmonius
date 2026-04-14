//! Shader compilation cache keyed by graph + target (IR-5.1.5).

use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use crate::types::TargetPlatform;

/// Bytecode family stored in the CAS.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ShaderArtifactKind {
    Dxil,
    Spirv,
    Metallib,
}

/// Cache key: stable shader graph id + target.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ShaderCompileKey {
    pub graph_id: u64,
    pub target: TargetPlatform,
}

/// Counts external tool invocations for tests (IR-5.1.5.2).
#[derive(Debug, Default)]
pub struct ShaderToolInvocations {
    pub dxc: AtomicUsize,
    pub metal_converter: AtomicUsize,
}

/// In-process shader compile cache (fake `dxc` / `metal-shaderconverter`).
#[derive(Debug)]
pub struct ShaderCompileCache {
    map: HashMap<ShaderCompileKey, (ShaderArtifactKind, [u8; 32])>,
    pub invocations: Arc<ShaderToolInvocations>,
}

impl Default for ShaderCompileCache {
    fn default() -> Self {
        Self {
            map: HashMap::new(),
            invocations: Arc::new(ShaderToolInvocations::default()),
        }
    }
}

impl ShaderCompileCache {
    /// Creates an empty cache.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns cached `(kind, cas_key)` or compiles via `compile_fn` (increments tool counters).
    pub fn get_or_compile(
        &mut self,
        key: ShaderCompileKey,
        mut compile_fn: impl FnMut(&ShaderCompileKey) -> (ShaderArtifactKind, Vec<u8>),
    ) -> (ShaderArtifactKind, [u8; 32]) {
        if let Some(v) = self.map.get(&key).copied() {
            return v;
        }
        match key.target {
            TargetPlatform::Windows => {
                self.invocations.dxc.fetch_add(1, Ordering::SeqCst);
            }
            TargetPlatform::MacOS | TargetPlatform::IOS => {
                self.invocations.dxc.fetch_add(1, Ordering::SeqCst);
                self.invocations
                    .metal_converter
                    .fetch_add(1, Ordering::SeqCst);
            }
            TargetPlatform::Linux | TargetPlatform::Android => {
                self.invocations.dxc.fetch_add(1, Ordering::SeqCst);
            }
            TargetPlatform::ConsoleA | TargetPlatform::ConsoleB => {
                self.invocations.dxc.fetch_add(1, Ordering::SeqCst);
            }
        }
        let (kind, bytes) = compile_fn(&key);
        let cas_key = *blake3::hash(&bytes).as_bytes();
        self.map.insert(key, (kind, cas_key));
        (kind, cas_key)
    }
}
