//! TC-2.2.13.1 — scoped encoder lifetime enforced at compile time.

#[test]
fn test_command_buffer_scope_lifetime() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/command_buffer_scope_lifetime.rs");
}
