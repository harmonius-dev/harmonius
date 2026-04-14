//! Emits `target_platform_console` cfgs consumed by `harmonius_core`.

use std::env;

fn main() {
    let target = env::var("TARGET").unwrap_or_default();
    if let Some(platform) = console_target::console_platform_from_target(&target) {
        println!("cargo:rustc-cfg=target_platform_console=\"{platform}\"");
    }
}
