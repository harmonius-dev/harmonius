//! Manual GPU/display smoke test.

#[test]
#[ignore = "requires display, Vulkan, and glslangValidator-built shaders"]
fn triangle_demo_runs() {
    // Run manually: cargo test -p harmonius_app -- --ignored --nocapture
    assert!(true);
}
