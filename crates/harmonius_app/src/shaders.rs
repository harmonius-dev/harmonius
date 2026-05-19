//! Build-time compiled SPIR-V shaders.

use std::path::PathBuf;

/// Load SPIR-V words from `OUT_DIR` for `shader_base` (e.g. `triangle.vert`).
pub fn load_spirv(shader_base: &str) -> Vec<u32> {
    let out_dir = PathBuf::from(env!("OUT_DIR"));
    let path = out_dir.join(format!("{shader_base}.spv"));
    let bytes = std::fs::read(&path).unwrap_or_else(|e| {
        panic!("missing compiled shader {}: {e}", path.display());
    });
    assert_eq!(
        bytes.len() % 4,
        0,
        "SPIR-V file size must be a multiple of 4"
    );
    let words: Vec<u32> = bytes
        .chunks_exact(4)
        .map(|chunk| u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
        .collect();
    words
}
