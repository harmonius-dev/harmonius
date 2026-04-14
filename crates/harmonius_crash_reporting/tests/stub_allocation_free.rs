#[test]
fn test_stub_is_signal_safe_noalloc() {
    let src = include_str!("../src/stub_fault_path.rs");
    for pat in [
        "alloc::",
        "std::alloc",
        "Box::",
        "Vec::new",
        "format!",
        "String::",
    ] {
        assert!(
            !src.contains(pat),
            "stub fault path must not reference heap APIs ({pat})"
        );
    }
}
