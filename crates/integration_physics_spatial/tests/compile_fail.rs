#[test]
fn tc_ir_3_9_2_u1_ai_cannot_name_physics_bvh_type() {
    let cases = trybuild::TestCases::new();
    cases.compile_fail("tests/compile_fail/ai_imports_physics_bvh.rs");
}
