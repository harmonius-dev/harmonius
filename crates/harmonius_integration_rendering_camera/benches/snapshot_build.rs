use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};

use harmonius_integration_rendering_camera::{
    build_render_view_from_camera, drs_pi_step, extract_sorted_render_views, resolve_post_process_stack,
    CameraOutput, Entity, PostProcessBlend, PostProcessParams, PostProcessVolume, Projection,
    Viewport,
};

fn bench_snapshot_build(c: &mut Criterion) {
    let projection = Projection::Perspective {
        fov_y_radians: 1.047,
        aspect: 16.0 / 9.0,
        near: 0.1,
        far: 1000.0,
    };
    let output = CameraOutput {
        brain: Entity(1),
        stable_index: 0,
        active: true,
        position: glam::Vec3::new(0.0, 5.0, -10.0),
        rotation: glam::Quat::IDENTITY,
        projection,
        render_layers: 1,
        render_order: 0,
        viewport: Viewport {
            x: 0,
            y: 0,
            width: 1920,
            height: 1080,
        },
    };
    c.bench_function("render_view_from_camera_build", |b| {
        b.iter(|| {
            black_box(build_render_view_from_camera(
                black_box(&output),
                black_box(1.0),
                black_box(0.5),
            ))
        });
    });
}

fn bench_post_process_resolve(c: &mut Criterion) {
    let mut volumes = Vec::new();
    for idx in 0..8 {
        let base = idx as f32;
        volumes.push(PostProcessVolume {
            entity: Entity(idx + 1),
            min: glam::Vec3::new(-1.0 + base * 0.01, -1.0, -1.0),
            max: glam::Vec3::new(1.0 + base * 0.01, 1.0, 1.0),
            priority: 0,
            params: PostProcessParams {
                exposure: 1.0 + base * 0.05,
                ..PostProcessParams::default()
            },
        });
    }
    let blends: Vec<PostProcessBlend> = volumes
        .iter()
        .map(|v| PostProcessBlend {
            volume: v.entity,
            weight: 1.0 / 8.0,
            priority: 0,
        })
        .collect();
    c.bench_function("post_process_resolve_eight_volumes", |b| {
        b.iter(|| {
            black_box(resolve_post_process_stack(
                black_box(glam::Vec3::ZERO),
                black_box(&blends),
                black_box(&volumes),
                |_| true,
            ))
        });
    });
}

fn bench_multi_view_extract(c: &mut Criterion) {
    let projection = Projection::Perspective {
        fov_y_radians: 1.0,
        aspect: 1.0,
        near: 0.1,
        far: 100.0,
    };
    let mut outputs = Vec::new();
    for idx in 0..4 {
        outputs.push(CameraOutput {
            brain: Entity(idx + 1),
            stable_index: idx,
            active: true,
            position: glam::Vec3::new(idx as f32, 0.0, 0.0),
            rotation: glam::Quat::IDENTITY,
            projection,
            render_layers: 1,
            render_order: 0,
            viewport: Viewport::default(),
        });
    }
    c.bench_function("extract_four_cameras", |b| {
        b.iter(|| {
            black_box(extract_sorted_render_views(
                black_box(&outputs),
                black_box(1.0),
                black_box(0.5),
            ))
        });
    });
}

fn bench_drs_pi(c: &mut Criterion) {
    c.bench_function("drs_pi_step", |b| {
        b.iter(|| black_box(drs_pi_step(black_box(1.0), black_box(20.0), black_box(16.6), black_box(0.5))))
    });
}

criterion_group!(
    benches,
    bench_snapshot_build,
    bench_post_process_resolve,
    bench_multi_view_extract,
    bench_drs_pi
);
criterion_main!(benches);
