#[test]
fn test_codegen_rejects_sensitive_field() {
    let cases = trybuild::TestCases::new();
    cases.pass("tests/compile_pass/pii_ok.rs");
    cases.compile_fail("tests/compile_fail/sensitive_field.rs");
}
