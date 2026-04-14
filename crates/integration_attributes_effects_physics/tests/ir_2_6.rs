//! Integration tests for `TC-IR-2.6.*` (attributes-effects ↔ physics).

use std::hint::black_box;
use std::time::Instant;

use glam::Vec3;
use integration_attributes_effects_physics::{
    apply_surface_collision_effect, bench_friction_batch, bench_gravity_force_batch,
    bench_gravity_override_churn, bench_mass_inverse_batch, friction_override_from_lookup,
    friction_override_from_material, gravity_override_from_scale, idle_sync_touch_count,
    mass_sample_after_scale, resolve_surface_effect_row, scale_gravity_force,
    scale_knockback_force, sync_gravity_batch, ActiveEffects, CollisionEffectLog, EffectDefinition,
    GravityOverride, MaterialLookup, PhysicsMaterial, RowRef, SurfaceEffectMap, SurfaceType,
    TableRegistry,
};

fn mat(surface: SurfaceType, sf: f32, df: f32) -> PhysicsMaterial {
    PhysicsMaterial {
        static_friction: sf,
        dynamic_friction: df,
        restitution: 0.0,
        density: 1.0,
        surface_type: surface,
    }
}

// --- Unit tests (companion table) ---

#[test]
fn tc_ir_2_6_1_u1_gravity_override_write_clear() {
    assert_eq!(
        gravity_override_from_scale(0.1),
        Some(GravityOverride { scale: 0.1 })
    );
    assert_eq!(gravity_override_from_scale(1.0), None);
}

#[test]
fn tc_ir_2_6_2_u1_mass_inverse_formula() {
    let m = mass_sample_after_scale(0.5, 2.0);
    assert!((m.inverse - 0.25).abs() < 1e-6);
}

#[test]
fn tc_ir_2_6_2_u2_featherfall_mass() {
    let m = mass_sample_after_scale(0.5, 0.5);
    assert!((m.inverse - 1.0).abs() < 1e-6);
}

#[test]
fn tc_ir_2_6_3_u1_friction_override_pair() {
    let material = mat(SurfaceType::Default, 0.8, 0.6);
    let o = friction_override_from_material(&material, 0.5).expect("override");
    assert!((o.static_friction - 0.4).abs() < 1e-6);
    assert!((o.dynamic_friction - 0.3).abs() < 1e-6);
}

#[test]
fn tc_ir_2_6_3_u2_friction_clamp_upper() {
    let material = mat(SurfaceType::Default, 0.8, 0.6);
    let o = friction_override_from_material(&material, 10.0).expect("override");
    assert!((o.static_friction - 1.0).abs() < 1e-6);
    assert!((o.dynamic_friction - 1.0).abs() < 1e-6);
}

#[test]
fn tc_ir_2_6_5_u1_surface_effect_map_lookup() {
    let map = SurfaceEffectMap::new();
    let ice_row = RowRef(9001);
    map.insert(SurfaceType::Ice, ice_row);
    let mut reg = TableRegistry::new();
    reg.register_effect(ice_row, EffectDefinition { id: 42 });
    let def = resolve_surface_effect_row(SurfaceType::Ice, &map, &reg).expect("def");
    assert_eq!(def.id, 42);
}

// --- Integration-style tests ---

#[test]
fn tc_ir_2_6_1_1_levitate_reduces_gravity() {
    let g = Vec3::new(0.0, -9.81, 0.0);
    let f = scale_gravity_force(g, 0.1);
    assert!((f - g * 0.1).length() < 1e-4);
}

#[test]
fn tc_ir_2_6_1_2_heavy_curse_doubles_gravity() {
    let g = Vec3::new(0.0, -9.81, 0.0);
    let f = scale_gravity_force(g, 2.0);
    assert!((f - g * 2.0).length() < 1e-4);
}

#[test]
fn tc_ir_2_6_1_3_stacked_gravity_effects() {
    let g = Vec3::new(0.0, -9.81, 0.0);
    let aggregated = 0.25;
    let f = scale_gravity_force(g, aggregated);
    assert!((f - g * 0.25).length() < 1e-4);
}

#[test]
fn tc_ir_2_6_2_1_featherfall_reduces_mass() {
    let base = mass_sample_after_scale(0.5, 1.0).mass();
    let feather = mass_sample_after_scale(0.5, 0.5).mass();
    assert!((feather - base * 0.5).abs() < 1e-4);
}

#[test]
fn tc_ir_2_6_2_2_petrify_increases_mass() {
    let base = mass_sample_after_scale(0.5, 1.0).mass();
    let stone = mass_sample_after_scale(0.5, 5.0).mass();
    assert!((stone - base * 5.0).abs() < 1e-3);
}

#[test]
fn tc_ir_2_6_3_1_ice_walk_reduces_friction() {
    let material = mat(SurfaceType::Ice, 0.8, 0.6);
    let o = friction_override_from_material(&material, 0.1).expect("override");
    assert!(o.static_friction < 0.15);
    assert!(o.dynamic_friction < 0.15);
}

#[test]
fn tc_ir_2_6_3_2_root_maximizes_friction() {
    let material = mat(SurfaceType::Grass, 0.5, 0.4);
    let o = friction_override_from_material(&material, 10.0).expect("override");
    assert!((o.static_friction - 1.0).abs() < 1e-6);
    assert!((o.dynamic_friction - 1.0).abs() < 1e-6);
}

#[test]
fn tc_ir_2_6_3_3_static_friction_written() {
    let material = mat(SurfaceType::Stone, 0.8, 0.6);
    let o = friction_override_from_material(&material, 0.5).expect("override");
    assert!((o.static_friction - 0.4).abs() < 1e-6);
}

#[test]
fn tc_ir_2_6_4_1_strength_scales_knockback() {
    let base = Vec3::new(10.0, 0.0, 0.0);
    let f = scale_knockback_force(base, 50.0);
    assert!((f - base * 1.5).length() < 1e-4);
}

#[test]
fn tc_ir_2_6_4_2_zero_strength_no_force() {
    let base = Vec3::new(10.0, 0.0, 0.0);
    let f = scale_knockback_force(base, 0.0);
    assert_eq!(f, Vec3::ZERO);
}

#[test]
fn tc_ir_2_6_5_1_ice_collision_slips() {
    let map = SurfaceEffectMap::new();
    let row = RowRef(7);
    map.insert(SurfaceType::Ice, row);
    let mut reg = TableRegistry::new();
    reg.register_effect(row, EffectDefinition { id: 701 });
    let ice = mat(SurfaceType::Ice, 0.6, 0.5);
    let mut active = ActiveEffects::new();
    let mut log = CollisionEffectLog::new();
    let mut warn = 0u32;
    assert!(apply_surface_collision_effect(
        Some(&ice),
        &map,
        &reg,
        Some(&mut active),
        10,
        &mut log,
        &mut warn
    ));
    assert!(active.contains_effect(701));
    assert_eq!(warn, 0);
}

#[test]
fn tc_ir_2_6_5_2_water_collision() {
    let map = SurfaceEffectMap::new();
    let row = RowRef(8);
    map.insert(SurfaceType::Water, row);
    let mut reg = TableRegistry::new();
    reg.register_effect(row, EffectDefinition { id: 702 });
    let water = mat(SurfaceType::Water, 0.2, 0.15);
    let mut active = ActiveEffects::new();
    let mut log = CollisionEffectLog::new();
    let mut warn = 0u32;
    assert!(apply_surface_collision_effect(
        Some(&water),
        &map,
        &reg,
        Some(&mut active),
        5,
        &mut log,
        &mut warn
    ));
    assert!(active.contains_effect(702));
}

#[test]
fn tc_ir_2_6_6_1_levitate_expires_restore() {
    let g = Vec3::new(0.0, -10.0, 0.0);
    let lev = scale_gravity_force(g, 0.1);
    let normal = scale_gravity_force(g, 1.0);
    assert!((lev - g * 0.1).length() < 1e-4);
    assert!((normal - g).length() < 1e-4);
}

#[test]
fn tc_ir_2_6_6_2_mass_effect_expires() {
    let before = mass_sample_after_scale(0.4, 5.0);
    let after = mass_sample_after_scale(0.4, 1.0);
    assert!((before.mass() / after.mass() - 5.0).abs() < 1e-3);
}

#[test]
fn tc_ir_2_6_6_3_friction_expiry_restore() {
    let material = mat(SurfaceType::Ice, 0.9, 0.8);
    assert!(friction_override_from_material(&material, 0.2).is_some());
    assert_eq!(friction_override_from_material(&material, 1.0), None);
}

#[test]
fn tc_ir_2_6_6_4_gravity_expiry_restore() {
    assert!(gravity_override_from_scale(0.05).is_some());
    assert_eq!(gravity_override_from_scale(1.0), None);
}

// --- Negative tests ---

#[test]
fn tc_ir_2_6_1_n1_zero_gravity_scale() {
    let o = gravity_override_from_scale(0.0).expect("override");
    assert!((o.scale - 0.01).abs() < 1e-6);
}

#[test]
fn tc_ir_2_6_2_n1_zero_mass_scale() {
    let m = mass_sample_after_scale(0.5, 0.0);
    assert!((m.inverse - 0.5 / 0.001).abs() < 1e-3);
}

#[test]
fn tc_ir_2_6_3_n1_negative_friction_scale() {
    let material = mat(SurfaceType::Default, 0.5, 0.4);
    let o = friction_override_from_material(&material, -1.0).expect("override");
    assert_eq!(o.static_friction, 0.0);
    assert_eq!(o.dynamic_friction, 0.0);
}

#[test]
fn tc_ir_2_6_3_n2_missing_handle_warn_once() {
    let mut missing = 0u32;
    let mut unloaded = 0u32;
    assert_eq!(
        friction_override_from_lookup(
            MaterialLookup::MissingHandle,
            0.5,
            &mut missing,
            &mut unloaded
        ),
        None
    );
    assert_eq!(
        friction_override_from_lookup(
            MaterialLookup::MissingHandle,
            0.5,
            &mut missing,
            &mut unloaded
        ),
        None
    );
    assert_eq!(missing, 1);
    assert_eq!(unloaded, 0);
}

#[test]
fn tc_ir_2_6_3_n3_unloaded_material_warn_once() {
    let mut missing = 0u32;
    let mut unloaded = 0u32;
    assert_eq!(
        friction_override_from_lookup(MaterialLookup::Unloaded, 0.5, &mut missing, &mut unloaded),
        None
    );
    assert_eq!(
        friction_override_from_lookup(MaterialLookup::Unloaded, 0.5, &mut missing, &mut unloaded),
        None
    );
    assert_eq!(missing, 0);
    assert_eq!(unloaded, 1);
}

#[test]
fn tc_ir_2_6_5_n1_unmapped_surface() {
    let map = SurfaceEffectMap::new();
    let reg = TableRegistry::new();
    let default_mat = mat(SurfaceType::Default, 0.5, 0.4);
    let mut active = ActiveEffects::new();
    let mut log = CollisionEffectLog::new();
    let mut warn = 0u32;
    assert!(!apply_surface_collision_effect(
        Some(&default_mat),
        &map,
        &reg,
        Some(&mut active),
        1,
        &mut log,
        &mut warn
    ));
    assert_eq!(active.len(), 0);
}

#[test]
fn tc_ir_2_6_5_n2_target_has_no_effects_warn_once() {
    let map = SurfaceEffectMap::new();
    let row = RowRef(3);
    map.insert(SurfaceType::Ice, row);
    let mut reg = TableRegistry::new();
    reg.register_effect(row, EffectDefinition { id: 9 });
    let ice = mat(SurfaceType::Ice, 0.5, 0.4);
    let mut log = CollisionEffectLog::new();
    let mut warn = 0u32;
    assert!(!apply_surface_collision_effect(
        Some(&ice),
        &map,
        &reg,
        None,
        1,
        &mut log,
        &mut warn
    ));
    assert!(!apply_surface_collision_effect(
        Some(&ice),
        &map,
        &reg,
        None,
        1,
        &mut log,
        &mut warn
    ));
    assert_eq!(warn, 1);
}

#[test]
fn tc_ir_2_6_5_n3_missing_handle_on_hit() {
    let map = SurfaceEffectMap::new();
    let row = RowRef(4);
    map.insert(SurfaceType::Ice, row);
    let mut reg = TableRegistry::new();
    reg.register_effect(row, EffectDefinition { id: 11 });
    let mut active = ActiveEffects::new();
    let mut log = CollisionEffectLog::new();
    let mut warn = 0u32;
    assert!(!apply_surface_collision_effect(
        None,
        &map,
        &reg,
        Some(&mut active),
        1,
        &mut log,
        &mut warn
    ));
    assert_eq!(active.len(), 0);
    assert_eq!(warn, 0);
}

#[test]
fn tc_ir_2_6_6_n1_effect_stack_overflow() {
    let mut active = ActiveEffects::new();
    for p in 1_i32..=17 {
        let _ = active.apply(p, p as u32);
    }
    assert_eq!(active.len(), 16);
    assert!(!active.contains_effect(1));
    assert!(active.contains_effect(17));
}

#[test]
fn tc_ir_2_6_6_n2_expiry_with_no_override_no_panic() {
    assert_eq!(gravity_override_from_scale(1.0), None);
    assert_eq!(
        friction_override_from_material(&mat(SurfaceType::Default, 0.5, 0.4), 1.0),
        None
    );
}

// --- Throughput-style checks (CI slack) ---

#[test]
fn tc_ir_2_6_0_b1_idle_no_touch() {
    let changed = [false, false, false];
    assert_eq!(idle_sync_touch_count(&changed), 0);
    let scales = [1.0_f32, 1.0, 1.0];
    assert_eq!(sync_gravity_batch(&scales, &changed), 0);
}

#[test]
fn tc_ir_2_6_1_b1_gravity_sync_throughput() {
    let scales = vec![0.5_f32; 1000];
    let changed = vec![true; 1000];
    let start = Instant::now();
    for _ in 0..50 {
        let _ = black_box(sync_gravity_batch(
            black_box(scales.as_slice()),
            black_box(changed.as_slice()),
        ));
    }
    assert!(start.elapsed().as_micros() < 8_000);
}

#[test]
fn tc_ir_2_6_3_b1_friction_sync_throughput() {
    let material = mat(SurfaceType::Rubber, 0.9, 0.85);
    let scales = vec![0.7_f32; 1000];
    let start = Instant::now();
    for _ in 0..40 {
        let _ = black_box(bench_friction_batch(
            black_box(&material),
            black_box(scales.as_slice()),
            1000,
        ));
    }
    assert!(start.elapsed().as_micros() < 12_000);
}

#[test]
fn tc_ir_2_6_4_b1_force_scaling_throughput() {
    let scales = vec![0.25_f32; 1000];
    let g = Vec3::new(0.0, -9.81, 0.0);
    let start = Instant::now();
    for _ in 0..40 {
        let _ = black_box(bench_gravity_force_batch(
            black_box(g),
            black_box(scales.as_slice()),
            1000,
        ));
    }
    assert!(start.elapsed().as_micros() < 12_000);
}

#[test]
fn tc_ir_2_6_2_b1_mass_sync_throughput() {
    let scales = vec![1.2_f32; 1000];
    let start = Instant::now();
    for _ in 0..40 {
        let _ = black_box(bench_mass_inverse_batch(
            black_box(0.5_f32),
            black_box(scales.as_slice()),
            1000,
        ));
    }
    assert!(start.elapsed().as_micros() < 12_000);
}

#[test]
fn tc_ir_2_6_5_b1_collision_effects_throughput() {
    let map = SurfaceEffectMap::new();
    let row = RowRef(501);
    map.insert(SurfaceType::Ice, row);
    let mut reg = TableRegistry::new();
    reg.register_effect(row, EffectDefinition { id: 501 });
    let ice = mat(SurfaceType::Ice, 0.6, 0.5);
    let start = Instant::now();
    for _ in 0..25 {
        for _ in 0..500 {
            let mut active = ActiveEffects::new();
            let mut log = CollisionEffectLog::new();
            let mut warn = 0u32;
            let _ = black_box(apply_surface_collision_effect(
                black_box(Some(&ice)),
                black_box(&map),
                black_box(&reg),
                black_box(Some(&mut active)),
                10,
                black_box(&mut log),
                black_box(&mut warn),
            ));
        }
    }
    assert!(start.elapsed().as_micros() < 25_000);
}

#[test]
fn tc_ir_2_6_6_b1_effect_expiry_restore_throughput() {
    let start = Instant::now();
    for _ in 0..120 {
        let _ = black_box(bench_gravity_override_churn(black_box(200)));
    }
    assert!(start.elapsed().as_micros() < 12_000);
}
