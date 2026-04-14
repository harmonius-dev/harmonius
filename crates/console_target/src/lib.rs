//! Maps `TARGET` triple fragments to `target_platform_console` cfg values.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

/// Returns the console platform key for `rustc` `--cfg target_platform_console="…"`, if any.
#[must_use]
pub fn console_platform_from_target(target: &str) -> Option<&'static str> {
    if target.contains("ps5") {
        return Some("ps5");
    }
    if target.contains("xbox-gdk") {
        return Some("xbox");
    }
    if target.contains("nintendo") {
        return Some("switch");
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_script_sets_console_cfg_ps5() {
        assert_eq!(console_platform_from_target("aarch64-sie-ps5"), Some("ps5"));
    }

    #[test]
    fn test_build_script_sets_console_cfg_xbox() {
        assert_eq!(
            console_platform_from_target("x86_64-pc-windows-msvc-xbox-gdk"),
            Some("xbox")
        );
    }

    #[test]
    fn test_build_script_sets_console_cfg_switch() {
        assert_eq!(
            console_platform_from_target("aarch64-nintendo-switch-freertos"),
            Some("switch")
        );
    }

    #[test]
    fn test_desktop_target_has_no_console_cfg() {
        for triple in [
            "x86_64-unknown-linux-gnu",
            "aarch64-apple-darwin",
            "x86_64-pc-windows-msvc",
        ] {
            assert_eq!(console_platform_from_target(triple), None);
        }
    }
}
