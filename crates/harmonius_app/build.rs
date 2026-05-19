//! Compiles GLSL shaders to SPIR-V via `glslangValidator`.

use std::env;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Command;

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
    let validator = which_glslang_validator();
    let status = Command::new(&validator)
        .args([
            "-V",
            src.to_str().expect("utf8 path"),
            "-o",
            dst.to_str().expect("utf8 path"),
        ])
        .status()
        .unwrap_or_else(|e| {
            panic!(
                "failed to run glslangValidator ({validator:?}): {e}\n\
                 Install the Vulkan SDK or glslang-tools and ensure glslangValidator is on PATH."
            );
        });
    if !status.success() {
        panic!(
            "glslangValidator failed for {} (exit {:?})",
            src.display(),
            status.code()
        );
    }
}

fn which_glslang_validator() -> String {
    if let Ok(path) = env::var("GLSLANG_VALIDATOR") {
        if Path::new(&path).is_file() {
            return path;
        }
    }
    for candidate in ["glslangValidator", "glslangValidator.exe"] {
        if Command::new(OsStr::new(candidate))
            .arg("--version")
            .output()
            .is_ok()
        {
            return candidate.to_string();
        }
    }
    panic!(
        "glslangValidator not found on PATH. Set GLSLANG_VALIDATOR or install Vulkan SDK / \
         glslang-tools."
    );
}
