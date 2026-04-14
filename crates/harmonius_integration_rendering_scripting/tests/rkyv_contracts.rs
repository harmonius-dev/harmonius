//! rkyv alignment + `check_bytes` coverage (TC-IR-3.5.U1, TC-IR-3.5.U2, TC-IR-3.5.U3).

use harmonius_integration_rendering_scripting::{
    CompiledEffect, HlslSourceHandle, MaterialShaderOutput, PermutationSpan, ShaderFeatures,
    ShadingModel,
};
use rkyv::rancor::Failure;
use rkyv::{from_bytes, to_bytes};

#[test]
fn tc_ir_3_5_u1_material_shader_output_rkyv_ptr_aligns_16() {
    let v = MaterialShaderOutput {
        hlsl: HlslSourceHandle {
            index: 1,
            generation: 2,
        },
        shading_model: ShadingModel::DefaultLit,
        features: ShaderFeatures::NORMAL_MAP,
        permutations: PermutationSpan {
            offset: 0,
            length: 1,
        },
        content_hash: 0xC0FFEE,
    };
    let bytes = to_bytes::<Failure>(&v).expect("serialize");
    assert_eq!(bytes.as_ptr() as usize % 16, 0);
}

#[test]
fn tc_ir_3_5_u2_compiled_effect_rkyv_ptr_aligns_16() {
    let v = CompiledEffect::test_fixture();
    let bytes = to_bytes::<Failure>(&v).expect("serialize");
    assert_eq!(bytes.as_ptr() as usize % 16, 0);
}

#[test]
fn tc_ir_3_5_u3_rkyv_check_bytes_roundtrip_material() {
    let v = MaterialShaderOutput {
        hlsl: HlslSourceHandle {
            index: 0,
            generation: 0,
        },
        shading_model: ShadingModel::Foliage,
        features: ShaderFeatures::ALL_NAMED_BITS,
        permutations: PermutationSpan {
            offset: 7,
            length: 3,
        },
        content_hash: 42,
    };
    let bytes = to_bytes::<Failure>(&v).expect("serialize");
    let back: MaterialShaderOutput =
        from_bytes::<MaterialShaderOutput, Failure>(bytes.as_slice()).expect("deserialize");
    assert_eq!(v, back);
}

#[test]
fn tc_ir_3_5_u3_rkyv_check_bytes_roundtrip_effect() {
    let v = CompiledEffect::test_fixture();
    let bytes = to_bytes::<Failure>(&v).expect("serialize");
    let back: CompiledEffect =
        from_bytes::<CompiledEffect, Failure>(bytes.as_slice()).expect("deserialize");
    assert_eq!(v, back);
}

#[test]
fn tc_ir_3_5_1_i2_unlit_model_roundtrips_in_archive() {
    let v = MaterialShaderOutput {
        hlsl: HlslSourceHandle {
            index: 0,
            generation: 0,
        },
        shading_model: ShadingModel::Unlit,
        features: ShaderFeatures::NONE,
        permutations: PermutationSpan {
            offset: 0,
            length: 0,
        },
        content_hash: 0,
    };
    let bytes = to_bytes::<Failure>(&v).expect("serialize");
    let back: MaterialShaderOutput =
        from_bytes::<MaterialShaderOutput, Failure>(bytes.as_slice()).expect("deserialize");
    assert_eq!(back.shading_model, ShadingModel::Unlit);
}
