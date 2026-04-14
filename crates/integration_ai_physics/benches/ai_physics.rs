//! Criterion benches for AI ↔ physics harness hot paths (dev-dependency only; **not** CI
//! threshold gates — see `docs/design/integration/ai-physics-test-cases.md`).

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use glam::Vec3;
use integration_ai_physics::{
    enumerate_avoidance, parabola_point, AvoidanceQuery, JumpArcQuery, JumpArcScene, MaterialId,
    PhysicsBody, PlanarPatch, RigidBodyKind, WalkabilityQuery, WalkabilityScene,
};
use integration_ai_physics::{AgentMask, Entity, Plane};

fn bench_walkability_1000(c: &mut Criterion) {
    let scene = WalkabilityScene {
        bvh_ready: true,
        surfaces: vec![PlanarPatch {
            plane: Plane::from_point_normal(Vec3::ZERO, Vec3::Y),
            center: Vec3::ZERO,
            xz_extent: 50.0,
            material: MaterialId(0),
            entity: Entity(1),
        }],
    };
    let query = WalkabilityQuery {
        request_id: 0,
        foot_position: Vec3::new(0.0, 0.01, 0.0),
        max_slope_deg: 45.0,
        agent_mask: AgentMask::default().with_material(MaterialId(0)),
        excluded_entity: None,
    };
    let mut metrics = integration_ai_physics::FallbackMetrics::default();
    c.bench_function("tc_ir_2_5_1_b1_walkability_1000", |b| {
        b.iter(|| {
            for i in 0..1000 {
                let mut q = query;
                q.request_id = i;
                black_box(scene.resolve(q, &mut metrics));
            }
        });
    });
}

fn bench_jump_arc_100x16(c: &mut Criterion) {
    let scene = JumpArcScene {
        blockers: vec![],
        ground_y: Some(0.0),
    };
    let query = JumpArcQuery {
        start: Vec3::ZERO,
        initial_velocity: Vec3::new(5.0, 5.0, 0.0),
        gravity: Vec3::new(0.0, -9.81, 0.0),
        segment_count: 16,
        segment_dt: 0.08,
    };
    c.bench_function("tc_ir_2_5_2_b1_jump_arc_100x16", |b| {
        b.iter(|| {
            for _ in 0..100 {
                black_box(scene.trace(query));
            }
        });
    });
}

fn bench_climb_100(c: &mut Criterion) {
    use integration_ai_physics::{AxisAlignedBox, ClimbScene, ClimbSweepQuery};
    let ledge = AxisAlignedBox::new(Vec3::new(0.0, 1.2, -1.0), Vec3::new(1.5, 1.25, 1.0));
    let scene = ClimbScene { wall_x: 0.0, ledge };
    let query = ClimbSweepQuery {
        origin: Vec3::new(0.29, 0.0, 0.0),
        direction: Vec3::Y,
        capsule_radius: 0.3,
        max_height: 1.5,
    };
    c.bench_function("tc_ir_2_5_3_b1_climb_100", |b| {
        b.iter(|| {
            for _ in 0..100 {
                black_box(scene.sweep(query));
            }
        });
    });
}

fn bench_avoidance_500(c: &mut Criterion) {
    let mut bodies = Vec::new();
    for i in 0..64 {
        bodies.push(PhysicsBody {
            entity: Entity(i + 1),
            position: Vec3::new((i as f32).sin(), (i as f32).cos(), 0.0),
            radius: 0.3,
            body: RigidBodyKind::Dynamic {
                linvel: Vec3::new(1.0, 0.0, 0.0),
            },
        });
    }
    let query = AvoidanceQuery {
        center: Vec3::ZERO,
        radius: 5.0,
        self_entity: Entity(999),
    };
    let mut metrics = integration_ai_physics::FallbackMetrics::default();
    c.bench_function("tc_ir_2_5_4_b1_avoidance_500", |b| {
        b.iter(|| {
            for _ in 0..500 {
                black_box(enumerate_avoidance(black_box(&bodies), query, &mut metrics));
            }
        });
    });
}

fn bench_grounded_reads(c: &mut Criterion) {
    use integration_ai_physics::{read_ai_grounded_state, AiGroundedState, FootContact};
    let contacts = [FootContact {
        ground_normal: Vec3::Y,
        ground_entity: Entity(1),
    }];
    let prev = AiGroundedState {
        grounded: false,
        ground_normal: Vec3::ZERO,
        ground_entity: Entity::NONE,
    };
    let mut metrics = integration_ai_physics::FallbackMetrics::default();
    c.bench_function("tc_ir_2_5_5_b1_grounded_500", |b| {
        b.iter(|| {
            for _ in 0..500 {
                black_box(read_ai_grounded_state(Some(&contacts), prev, &mut metrics));
            }
        });
    });
}

fn bench_parabola_samples(c: &mut Criterion) {
    let start = Vec3::ZERO;
    let v0 = Vec3::new(5.0, 5.0, 0.0);
    let g = Vec3::new(0.0, -9.81, 0.0);
    c.bench_function("parabola_point_hot_loop", |b| {
        b.iter(|| {
            for i in 0..256 {
                let t = (i as f32) * 1e-3;
                black_box(parabola_point(start, v0, g, t));
            }
        });
    });
}

criterion_group!(
    benches,
    bench_walkability_1000,
    bench_jump_arc_100x16,
    bench_climb_100,
    bench_avoidance_500,
    bench_grounded_reads,
    bench_parabola_samples
);
criterion_main!(benches);
