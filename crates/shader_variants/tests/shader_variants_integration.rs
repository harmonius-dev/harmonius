//! Integration scenarios for shader variant bundles and resolution.

use shader_variants::{
    LodLevel, OnDemandCompiler, PermutationKey, RenderPath, ShaderApi, ShaderFeatures,
    ShaderResolver, ShadingModel, VariantBundle, VariantBundleWriter,
};

struct BytesCompiler {
    payload: Vec<u8>,
}

impl OnDemandCompiler for BytesCompiler {
    fn compile(&mut self, _: PermutationKey) -> Result<Vec<u8>, shader_variants::VariantError> {
        Ok(self.payload.clone())
    }
}

fn mat_key(m: u32, bits: u32) -> PermutationKey {
    PermutationKey {
        shading_model: ShadingModel::VARIANTS[(m as usize) % ShadingModel::VARIANTS.len()],
        features: ShaderFeatures { bits },
        render_path: RenderPath::Forward,
        lod: LodLevel::High,
    }
}

#[test]
fn test_full_build_precompile_pak() {
    let dir = tempfile::tempdir().unwrap();
    let pak = dir.path().join("out.pak");
    let mut w = VariantBundleWriter::new(ShaderApi::D3D12);
    for m in 0..5u32 {
        w.push_variant(mat_key(m, 0), vec![m as u8, 1, 2], m as u64);
    }
    w.write_to_path(&pak).unwrap();
    let bundle = VariantBundle::open_mmap(&pak).unwrap();
    for m in 0..5u32 {
        let rec = bundle.get_record(&mat_key(m, 0)).expect("variant present");
        assert_eq!(bundle.slice(rec)[0], m as u8);
    }
}

#[test]
fn test_runtime_resolve_precompiled_hit() {
    let dir = tempfile::tempdir().unwrap();
    let pak = dir.path().join("hit.pak");
    let mut w = VariantBundleWriter::new(ShaderApi::D3D12);
    let k = mat_key(0, 7);
    w.push_variant(k, vec![0xAA, 0xBB], 1);
    w.write_to_path(&pak).unwrap();
    let bundle = VariantBundle::open_mmap(&pak).unwrap();
    let mut resolver = ShaderResolver::new(
        bundle,
        BytesCompiler {
            payload: vec![0xEE],
        },
    );
    let got = resolver.resolve(k).unwrap();
    assert_eq!(got, &[0xAA, 0xBB]);
    assert_eq!(*resolver.metrics().hit_counts().get(&k).unwrap(), 1);
}

#[test]
fn test_runtime_resolve_on_demand_fallback() {
    let dir = tempfile::tempdir().unwrap();
    let pak = dir.path().join("miss.pak");
    let mut w = VariantBundleWriter::new(ShaderApi::D3D12);
    w.push_variant(mat_key(0, 0), vec![1], 0);
    w.write_to_path(&pak).unwrap();
    let bundle = VariantBundle::open_mmap(&pak).unwrap();
    let mut resolver = ShaderResolver::new(
        bundle,
        BytesCompiler {
            payload: vec![0xDE, 0xAD],
        },
    );
    let missing = mat_key(3, 9);
    let got = resolver.resolve(missing).unwrap();
    assert_eq!(got, &[0xDE, 0xAD]);
    assert!(resolver.metrics().miss_log().contains(&missing));
}

#[test]
fn test_pak_cross_backend_build() {
    let dir = tempfile::tempdir().unwrap();
    let k = mat_key(1, 2);
    for (name, api) in [
        ("d3d12", ShaderApi::D3D12),
        ("vk", ShaderApi::Vulkan),
        ("mtl", ShaderApi::Metal),
    ] {
        let path = dir.path().join(format!("{name}.pak"));
        let mut w = VariantBundleWriter::new(api);
        w.push_variant(k, vec![api as u8, 2, 3], 0);
        w.write_to_path(&path).unwrap();
        let b = VariantBundle::open_mmap(&path).unwrap();
        let rec = b.get_record(&k).unwrap();
        assert_eq!(rec.api, api);
    }
}
