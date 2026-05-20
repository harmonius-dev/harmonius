//! Compiles GLSL shaders to SPIR-V via the bundled `naga` crate.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use naga::back::spv::{self, PipelineOptions};
use naga::front::glsl::{Frontend, Options as GlslOptions};
use naga::valid::{Capabilities, ValidationFlags, Validator};
use naga::ShaderStage;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let shader_dir = manifest_dir.join("../../shaders");
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));

    for name in ["triangle.vert", "triangle.frag"] {
        let src = shader_dir.join(name);
        println!("cargo:rerun-if-changed={}", src.display());
        let dst = out_dir.join(format!("{name}.spv"));
        compile_shader(&src, &dst);
    }
}

fn compile_shader(src: &Path, dst: &Path) {
    let stage = shader_stage(src).unwrap_or_else(|| {
        panic!("unsupported shader extension: {}", src.display());
    });
    let source = fs::read_to_string(src).unwrap_or_else(|e| {
        panic!("failed to read shader {}: {e}", src.display());
    });

    let mut frontend = Frontend::default();
    let module = frontend
        .parse(&GlslOptions::from(stage), &source)
        .map_err(|errors| format_glsl_errors(src, &errors))
        .unwrap_or_else(|msg| panic!("{msg}"));

    let mut validator = Validator::new(ValidationFlags::all(), Capabilities::all());
    let info = validator
        .validate(&module)
        .map_err(|e| format!("validation failed for {}: {e}", src.display()))
        .unwrap_or_else(|msg| panic!("{msg}"));

    let pipeline = PipelineOptions {
        shader_stage: stage,
        entry_point: "main".into(),
    };
    let words = spv::write_vec(&module, &info, &spv::Options::default(), Some(&pipeline))
        .map_err(|e| format!("SPIR-V emission failed for {}: {e}", src.display()))
        .unwrap_or_else(|msg| panic!("{msg}"));

    write_spv(dst, &words);
}

fn shader_stage(path: &Path) -> Option<ShaderStage> {
    match path.extension()?.to_str()? {
        "vert" => Some(ShaderStage::Vertex),
        "frag" => Some(ShaderStage::Fragment),
        "comp" => Some(ShaderStage::Compute),
        _ => None,
    }
}

fn format_glsl_errors(path: &Path, errors: &naga::front::glsl::ParseErrors) -> String {
    let mut msg = format!("GLSL parse failed for {}:\n", path.display());
    for error in &errors.errors {
        msg.push_str(&format!("  {error:?}\n"));
    }
    msg
}

fn write_spv(dst: &Path, words: &[u32]) {
    let bytes: Vec<u8> = words.iter().flat_map(|word| word.to_le_bytes()).collect();
    fs::write(dst, bytes).unwrap_or_else(|e| {
        panic!("failed to write SPIR-V {}: {e}", dst.display());
    });
}
