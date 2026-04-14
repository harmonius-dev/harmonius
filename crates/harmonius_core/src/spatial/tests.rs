use std::collections::HashSet;

use glam::{UVec2, Vec2, Vec3, Vec4};

use super::{
    Aabb, BvhConfig, BvhIndex, Entity, FrustumTest, GridConfig, GridCoord, QueryConfig,
    QueryEngine, QuerySort, SpatialError, SpatialHit, SpatialLayerMask, SpatialQuery, UniformGrid,
};

// TC-1.9.4.1 — AABB intersection primitives
#[test]
fn tc_1_9_4_1_aabb_overlap_cases() {
    let a = Aabb::new(Vec3::ZERO, Vec3::ONE);
    let b = Aabb::new(Vec3::splat(0.5), Vec3::splat(1.5));
    assert!(a.intersects(&b));

    let disjoint = Aabb::new(Vec3::splat(2.0), Vec3::splat(3.0));
    assert!(!a.intersects(&disjoint));

    let inv = Vec3::new(1.0, 0.0, 0.0);
    let hit = Aabb::new(Vec3::new(5.0, -1.0, -1.0), Vec3::new(6.0, 1.0, 1.0)).intersects_ray(
        Vec3::ZERO,
        inv,
        100.0,
    );
    assert!((hit.unwrap() - 5.0).abs() < 1e-3);

    assert!(
        Aabb::new(Vec3::new(0.5, 0.0, 0.0), Vec3::new(2.0, 1.0, 1.0))
            .intersects_sphere(Vec3::ZERO, 1.0)
    );

    let planes = axis_aligned_frustum();
    let inside = Aabb::new(Vec3::new(-1.0, -1.0, 1.0), Vec3::new(1.0, 1.0, 50.0));
    assert!(matches!(
        inside.intersects_frustum(&planes),
        FrustumTest::Inside | FrustumTest::Intersecting
    ));

    let outside = Aabb::new(
        Vec3::new(200.0, 200.0, 200.0),
        Vec3::new(210.0, 210.0, 210.0),
    );
    assert!(matches!(
        outside.intersects_frustum(&planes),
        FrustumTest::Outside
    ));
}

fn axis_aligned_frustum() -> [Vec4; 6] {
    [
        Vec4::new(-1.0, 0.0, 0.0, -10.0),
        Vec4::new(1.0, 0.0, 0.0, -10.0),
        Vec4::new(0.0, -1.0, 0.0, -10.0),
        Vec4::new(0.0, 1.0, 0.0, -10.0),
        Vec4::new(0.0, 0.0, -1.0, 0.0),
        Vec4::new(0.0, 0.0, 1.0, -100.0),
    ]
}

// TC-1.9.1.1 — insert/remove + invariants
#[test]
fn tc_1_9_1_1_bvh_insert_remove_invariants() {
    let mut bvh = BvhIndex::new(BvhConfig::default());
    let entries: Vec<_> = (0..1000)
        .map(|i| (Entity(i), random_aabb(i), SpatialLayerMask::ALL))
        .collect();
    let mut handles = bvh.batch_insert(&entries);
    assert_eq!(bvh.entity_count(), 1000);
    assert!(query_all(&bvh).len() >= 1000);

    for h in handles.drain(..500) {
        bvh.remove(h).unwrap();
    }
    assert_eq!(bvh.entity_count(), 500);
    assert!(query_all(&bvh).len() >= 500);
    assert!(bvh_invariants_hold(&bvh));
}

// TC-1.9.1.2 — SAH quality smoke
#[test]
fn tc_1_9_1_2_sah_quality_finite() {
    let mut bvh = BvhIndex::new(BvhConfig::default());
    let entries: Vec<_> = (0..10_000)
        .map(|i| (Entity(i), random_aabb(i), SpatialLayerMask::ALL))
        .collect();
    bvh.batch_insert(&entries);
    let q = bvh.sah_quality();
    assert!(q.is_finite());
    assert!(q > 0.0);
}

// TC-1.9.2.1 — incremental refit touches only ancestors
#[test]
fn tc_1_9_2_1_incremental_refit_scope() {
    let mut bvh = BvhIndex::new(BvhConfig::default());
    let entries: Vec<_> = (0..10_000)
        .map(|i| (Entity(i), random_aabb(i), SpatialLayerMask::ALL))
        .collect();
    let handles = bvh.batch_insert(&entries);
    bvh.test_propagation_steps = 0;
    let updates: Vec<_> = handles
        .iter()
        .take(100)
        .enumerate()
        .map(|(j, h)| {
            let base = random_aabb(j as u32);
            (*h, shift(base, Vec3::splat(0.001)))
        })
        .collect();
    bvh.refit_moved(&updates);
    assert!(bvh.test_propagation_steps > 0);
    assert!(bvh.test_propagation_steps < 10_000);
}

// TC-1.9.2.2 — fat AABB fast path avoids rebuild
#[test]
fn tc_1_9_2_2_fat_aabb_skip_structure() {
    let mut cfg = BvhConfig::default();
    cfg.fat_aabb_margin = 1.0;
    let mut bvh = BvhIndex::new(cfg);
    let aabb = Aabb::new(Vec3::ZERO, Vec3::ONE);
    let h = bvh.insert(Entity(1), aabb, SpatialLayerMask::ALL);
    let nodes_before = bvh.node_count();
    bvh.test_rebuild_calls = 0;
    bvh.update(
        h,
        Aabb::new(Vec3::splat(0.01), Vec3::splat(0.99)),
        SpatialLayerMask::ALL,
    )
    .unwrap();
    assert_eq!(bvh.node_count(), nodes_before);
    assert_eq!(bvh.test_rebuild_calls, 0);
}

// TC-1.9.2.3 — batch insert comparable quality
#[test]
fn tc_1_9_2_3_batch_insert_quality() {
    let mut incremental = BvhIndex::new(BvhConfig::default());
    for i in 0..1000 {
        incremental.insert(Entity(i), random_aabb(i), SpatialLayerMask::ALL);
    }
    let mut batch = BvhIndex::new(BvhConfig::default());
    let entries: Vec<_> = (0..1000)
        .map(|i| (Entity(i), random_aabb(i), SpatialLayerMask::ALL))
        .collect();
    batch.batch_insert(&entries);
    let ratio = (batch.sah_quality() / incremental.sah_quality())
        .max(incremental.sah_quality() / batch.sah_quality());
    assert!(ratio < 5.0, "SAH quality ratio unexpectedly large: {ratio}");
}

// TC-1.9.1.3 — rebuild restores quality
#[test]
fn tc_1_9_1_3_rebuild_quality_recovery() {
    let mut bvh = BvhIndex::new(BvhConfig::default());
    let mut handles = Vec::new();
    for i in 0..512 {
        handles.push(bvh.insert(Entity(i), random_aabb(i), SpatialLayerMask::ALL));
    }
    let fresh = bvh.sah_quality();
    for _ in 0..200 {
        let updates: Vec<_> = handles
            .iter()
            .map(|h| {
                let idx = h.index();
                (*h, shift(random_aabb(idx), Vec3::splat(3.0)))
            })
            .collect();
        bvh.refit_moved(&updates);
    }
    let degraded = bvh.sah_quality();
    bvh.rebuild_full();
    let rebuilt = bvh.sah_quality();
    assert!(degraded >= fresh * 0.5);
    assert!(rebuilt <= fresh * 1.2 || rebuilt <= degraded);
}

// TC-1.9.1.4 — stale handle detection
#[test]
fn tc_1_9_1_4_stale_handle() {
    let mut bvh = BvhIndex::new(BvhConfig::default());
    let h = bvh.insert(Entity(1), unit_aabb(), SpatialLayerMask::ALL);
    bvh.remove(h).unwrap();
    let err = bvh
        .update(h, unit_aabb(), SpatialLayerMask::ALL)
        .unwrap_err();
    assert!(matches!(err, SpatialError::StaleHandle { .. }));
}

// TC-1.9.1.5 — memory budget (analytic upper bound)
#[test]
fn tc_1_9_1_5_memory_budget_upper_bound() {
    let mut bvh = BvhIndex::new(BvhConfig::default());
    let entries: Vec<_> = (0..10_000)
        .map(|i| (Entity(i), random_aabb(i), SpatialLayerMask::ALL))
        .collect();
    bvh.batch_insert(&entries);
    let per_entity = bvh.memory_upper_bound_bytes() / bvh.entity_count() as usize;
    assert!(
        per_entity <= 512,
        "per-entity upper bound unexpectedly large: {per_entity}"
    );
}

// TC-1.9.3.3 — grid insert + radius query (supersedes octree cases)
#[test]
fn tc_1_9_3_3_grid_matches_bruteforce() {
    let cfg = GridConfig {
        cell_size: 1.0,
        dimensions: UVec2::new(128, 128),
        origin: Vec2::ZERO,
    };
    let mut grid = UniformGrid::new(cfg);
    let mut brute = Vec::new();
    for i in 0..10_000 {
        let p = Vec2::new((i % 97) as f32, ((i * 7) % 101) as f32);
        let e = Entity(i);
        grid.insert(e, p).unwrap();
        brute.push((e, p));
    }
    let center = Vec2::new(40.0, 40.0);
    let radius = 15.0_f32;
    let mut expected: Vec<Entity> = brute
        .iter()
        .filter(|(_, p)| p.distance(center) <= radius)
        .map(|(e, _)| *e)
        .collect();
    expected.sort_by_key(|e| e.0);
    let got = grid.query_radius(center, radius);
    assert_eq!(got, expected);
}

// TC-1.9.3.4 — boundary assignment is deterministic
#[test]
fn tc_1_9_3_4_grid_boundary_assignment() {
    let cfg = GridConfig {
        cell_size: 2.0,
        dimensions: UVec2::new(16, 16),
        origin: Vec2::ZERO,
    };
    let mut grid = UniformGrid::new(cfg);
    let coord = grid.insert(Entity(1), Vec2::new(2.0, 0.0)).unwrap();
    assert_eq!(coord, GridCoord { x: 1, y: 0 });
}

// TC-1.9.4.2 — ray cast vs brute force
#[test]
fn tc_1_9_4_2_ray_cast_accuracy() {
    let mut bvh = BvhIndex::new(BvhConfig::default());
    for i in 0..2000 {
        bvh.insert(Entity(i), random_aabb(i), SpatialLayerMask::ALL);
    }
    let engine = QueryEngine::new(&bvh, None);
    for seed in 0..200 {
        let origin = Vec3::new(seed as f32 * 0.13, 1.0, 2.0);
        let dir = Vec3::new(0.2, -0.4, 0.6).normalize();
        let max_dist = 500.0_f32;
        let cfg = QueryConfig::default();
        let hits = engine.ray_cast(origin, dir, max_dist, &cfg);
        let brute = brute_ray_leaves(&bvh, origin, dir, max_dist);
        let a: HashSet<_> = hits.iter().map(|h| h.entity).collect();
        let b: HashSet<_> = brute.into_iter().collect();
        assert_eq!(a, b, "seed {seed}");
    }
}

// TC-1.9.4.3 — frustum vs brute
#[test]
fn tc_1_9_4_3_frustum_accuracy() {
    let mut bvh = BvhIndex::new(BvhConfig::default());
    for i in 0..2000 {
        bvh.insert(Entity(i), random_aabb(i), SpatialLayerMask::ALL);
    }
    let engine = QueryEngine::new(&bvh, None);
    let planes = axis_aligned_frustum();
    let got = engine.frustum_query(&planes, SpatialLayerMask::ALL);
    let mut brute = Vec::new();
    brute_frustum(&bvh, &planes, &mut brute);
    assert_eq!(sorted_entities(got), sorted_entities(brute));
}

// TC-1.9.4.4 — kNN vs brute
#[test]
fn tc_1_9_4_4_knn_accuracy() {
    let mut bvh = BvhIndex::new(BvhConfig::default());
    for i in 0..2000 {
        bvh.insert(Entity(i), random_aabb(i), SpatialLayerMask::ALL);
    }
    let engine = QueryEngine::new(&bvh, None);
    let origin = Vec3::new(3.0, 4.0, 5.0);
    let cfg = QueryConfig::default();
    let hits = engine.k_nearest(origin, 10, 200.0, &cfg);
    let brute = brute_knn(&bvh, origin, 10, 200.0);
    assert_eq!(hits.len(), brute.len());
    for (a, b) in hits.iter().zip(brute.iter()) {
        assert_eq!(a.entity, b.entity);
    }
}

// TC-1.9.4.5 — layer filtering
#[test]
fn tc_1_9_4_5_layer_filtering() {
    let mut bvh = BvhIndex::new(BvhConfig::default());
    for i in 0..100 {
        bvh.insert(
            Entity(i),
            shift(unit_aabb(), Vec3::new(i as f32, 0.0, 0.0)),
            SpatialLayerMask::custom(8),
        );
    }
    for i in 100..200 {
        bvh.insert(
            Entity(i),
            shift(unit_aabb(), Vec3::new(i as f32, 0.0, 0.0)),
            SpatialLayerMask::custom(9),
        );
    }
    let engine = QueryEngine::new(&bvh, None);
    let mut cfg = QueryConfig::default();
    cfg.layer_mask = SpatialLayerMask::custom(8);
    let hits = engine.overlap_aabb(&Aabb::new(Vec3::splat(-5.0), Vec3::splat(500.0)), &cfg);
    assert_eq!(hits.len(), 100);
}

// TC-1.9.4.6 — empty index queries
#[test]
fn tc_1_9_4_6_empty_queries() {
    let bvh = BvhIndex::new(BvhConfig::default());
    let engine = QueryEngine::new(&bvh, None);
    let cfg = QueryConfig::default();
    assert!(engine.ray_cast(Vec3::ZERO, Vec3::X, 10.0, &cfg).is_empty());
    assert!(engine
        .frustum_query(&axis_aligned_frustum(), SpatialLayerMask::ALL)
        .is_empty());
    assert!(engine.k_nearest(Vec3::ZERO, 5, 10.0, &cfg).is_empty());
    assert!(engine.overlap_aabb(&unit_aabb(), &cfg).is_empty());
}

// TC-1.9.4.7 — nearest sort order
#[test]
fn tc_1_9_4_7_ray_sort_nearest() {
    let mut bvh = BvhIndex::new(BvhConfig::default());
    for i in 0..100 {
        bvh.insert(
            Entity(i),
            shift(unit_aabb(), Vec3::new(0.0, 0.0, i as f32)),
            SpatialLayerMask::ALL,
        );
    }
    let engine = QueryEngine::new(&bvh, None);
    let mut cfg = QueryConfig::default();
    cfg.sort = QuerySort::Nearest;
    let hits = engine.ray_cast(Vec3::new(0.5, 0.5, -10.0), Vec3::Z, 200.0, &cfg);
    for w in hits.windows(2) {
        assert!(w[0].distance <= w[1].distance);
    }
}

fn shift(aabb: Aabb, v: Vec3) -> Aabb {
    Aabb::new(aabb.min + v, aabb.max + v)
}

fn unit_aabb() -> Aabb {
    Aabb::new(Vec3::ZERO, Vec3::ONE)
}

fn random_aabb(seed: u32) -> Aabb {
    let base = Vec3::new(
        (seed % 50) as f32,
        ((seed / 7) % 50) as f32,
        ((seed / 13) % 50) as f32,
    );
    Aabb::new(base, base + Vec3::splat(1.0))
}

fn query_all(bvh: &BvhIndex) -> Vec<Entity> {
    let engine = QueryEngine::new(bvh, None);
    let cfg = QueryConfig {
        layer_mask: SpatialLayerMask::ALL,
        max_results: 0,
        sort: QuerySort::Unsorted,
    };
    engine
        .overlap_aabb(
            &Aabb::new(Vec3::splat(-10_000.0), Vec3::splat(10_000.0)),
            &cfg,
        )
        .into_iter()
        .map(|h| h.entity)
        .collect()
}

fn bvh_invariants_hold(bvh: &BvhIndex) -> bool {
    bvh.verify_structure_invariants().is_ok()
}

fn brute_ray_leaves(bvh: &BvhIndex, origin: Vec3, dir: Vec3, max_dist: f32) -> Vec<Entity> {
    let dir_len = dir.length();
    if dir_len <= f32::EPSILON {
        return Vec::new();
    }
    let d = dir / dir_len;
    let inv = Vec3::new(
        super::bvh::safe_inv(d.x),
        super::bvh::safe_inv(d.y),
        super::bvh::safe_inv(d.z),
    );
    let mut hits = Vec::new();
    bvh.for_each_overlapping_leaf(
        &Aabb::new(Vec3::splat(-1e6), Vec3::splat(1e6)),
        SpatialLayerMask::ALL,
        |leaf| {
            if let Some(_t) = leaf.aabb.intersects_ray(origin, inv, max_dist) {
                hits.push(leaf.entity);
            }
            true
        },
    );
    hits.sort_by_key(|e| e.0);
    hits.dedup();
    hits
}

fn brute_frustum(bvh: &BvhIndex, planes: &[Vec4; 6], out: &mut Vec<Entity>) {
    bvh.for_each_overlapping_leaf(
        &Aabb::new(Vec3::splat(-1e6), Vec3::splat(1e6)),
        SpatialLayerMask::ALL,
        |leaf| {
            if !matches!(leaf.aabb.intersects_frustum(planes), FrustumTest::Outside) {
                out.push(leaf.entity);
            }
            true
        },
    );
}

fn brute_knn(bvh: &BvhIndex, origin: Vec3, k: u32, max_radius: f32) -> Vec<SpatialHit> {
    let mut hits = Vec::new();
    bvh.for_each_overlapping_leaf(
        &Aabb::from_center_extents(origin, Vec3::splat(max_radius)),
        SpatialLayerMask::ALL,
        |leaf| {
            let d = leaf.aabb.center().distance(origin);
            if d <= max_radius {
                hits.push(SpatialHit {
                    entity: leaf.entity,
                    point: leaf.aabb.center(),
                    normal: Vec3::ZERO,
                    distance: d,
                });
            }
            true
        },
    );
    hits.sort_by(|a, b| {
        a.distance
            .partial_cmp(&b.distance)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    hits.truncate(k as usize);
    hits
}

fn sorted_entities(mut v: Vec<Entity>) -> Vec<Entity> {
    v.sort_by_key(|e| e.0);
    v
}
