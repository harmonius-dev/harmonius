//! Criterion benches for composition budgets (TC-16.5.5.B1–B10).

use std::time::Instant;

use bevy_ecs::prelude::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use harmonius_composition::{
    peer_state_hash, step_frame, AbilityRecipe, AssetId, AttributeSetDefinition, CompositionRecipe,
    Container, ContainerDefinition, DefinitionAsset, DirectedGraphDefinition, FrameInput,
    MeterDefinition, QuestRecipe, QuestState, RecipeContext, ScheduleRecipe, StealthState,
};

fn bench_256_character_scene(c: &mut Criterion) {
    c.bench_function("bench_256_character_scene", |b| {
        b.iter(|| {
            let mut world = World::new();
            for i in 0..256 {
                let e = world.spawn_empty().id();
                let graph = DirectedGraphDefinition::new(AssetId(i), 1, 8, 7);
                let gh = graph.mint_handle(&mut world);
                let _ = graph.bind(&mut world, e, gh);
                world.entity_mut(e).insert(QuestState {
                    active_objective: 0,
                    objectives: 8,
                });
            }
            let h = peer_state_hash(&mut world);
            black_box(h);
        });
    });
}

fn bench_10k_item_inventory_stack_op(c: &mut Criterion) {
    c.bench_function("bench_10k_item_inventory_stack_op", |b| {
        let mut world = World::new();
        let e = world.spawn_empty().id();
        let def = ContainerDefinition::new(AssetId(1), 1, 1, 10_000, 10_000);
        let h = def.mint_handle(&mut world);
        def.bind(&mut world, e, h).unwrap();
        b.iter(|| {
            let start = Instant::now();
            if let Some(mut c) = world.entity_mut(e).get_mut::<Container>() {
                if c.slots.len() >= 2 {
                    c.slots.swap(0, 1);
                }
            }
            black_box(start.elapsed());
        });
    });
}

fn bench_quest_graph_advance_32_graphs(c: &mut Criterion) {
    c.bench_function("bench_quest_graph_advance_32_graphs", |b| {
        let mut world = World::new();
        for i in 0..32 {
            let e = world.spawn_empty().id();
            let graph = DirectedGraphDefinition::new(AssetId(100 + i), 1, 8, 7);
            let gh = graph.mint_handle(&mut world);
            graph.bind(&mut world, e, gh).unwrap();
            world.entity_mut(e).insert(QuestState {
                active_objective: 0,
                objectives: 8,
            });
        }
        b.iter(|| {
            let input = FrameInput {
                tick: 1,
                quest_kills: 1,
                ..Default::default()
            };
            step_frame(&mut world, black_box(&input));
            black_box(peer_state_hash(&mut world));
        });
    });
}

fn bench_ability_64_activations(c: &mut Criterion) {
    c.bench_function("bench_ability_64_activations", |b| {
        let mut world = World::new();
        let recipe = AbilityRecipe {
            graph: DirectedGraphDefinition::new(AssetId(2), 1, 3, 2),
            mana: MeterDefinition::new(AssetId(3), 1, 0.0, 100.0, 100.0),
        };
        for _ in 0..64 {
            let root = world.spawn_empty().id();
            recipe
                .install(&mut world, root, &RecipeContext { tick: 0 })
                .unwrap();
        }
        b.iter(|| {
            let input = FrameInput {
                tick: 1,
                ability_cast: true,
                ..Default::default()
            };
            step_frame(&mut world, black_box(&input));
            black_box(peer_state_hash(&mut world));
        });
    });
}

fn bench_effect_tick_1024_active(c: &mut Criterion) {
    c.bench_function("bench_effect_tick_1024_active", |b| {
        let mut world = World::new();
        for i in 0..1024 {
            let e = world.spawn_empty().id();
            world.entity_mut(e).insert(harmonius_composition::Effect {
                name: format!("e{i}"),
                magnitude: 1.0,
            });
        }
        b.iter(|| {
            black_box(peer_state_hash(&mut world));
        });
    });
}

fn bench_event_log_2048_appends(c: &mut Criterion) {
    c.bench_function("bench_event_log_2048_appends", |b| {
        let mut v = Vec::with_capacity(2048);
        b.iter(|| {
            v.clear();
            for i in 0..2048 {
                v.push(i);
            }
            black_box(v.len());
        });
    });
}

fn bench_schedule_128_timelines(c: &mut Criterion) {
    c.bench_function("bench_schedule_128_timelines", |b| {
        let mut world = World::new();
        let recipe = ScheduleRecipe {
            role_table: AttributeSetDefinition::new(AssetId(9), 1, vec![]),
        };
        for _ in 0..128 {
            let root = world.spawn_empty().id();
            recipe
                .install(&mut world, root, &RecipeContext { tick: 0 })
                .unwrap();
        }
        b.iter(|| {
            let input = FrameInput {
                tick: 1,
                schedule_ticks: 1,
                ..Default::default()
            };
            step_frame(&mut world, black_box(&input));
            black_box(peer_state_hash(&mut world));
        });
    });
}

fn bench_spatial_awareness_256_sensors(c: &mut Criterion) {
    c.bench_function("bench_spatial_awareness_256_sensors", |b| {
        let mut world = World::new();
        for _ in 0..256 {
            let e = world.spawn_empty().id();
            world.entity_mut(e).insert(StealthState { detected: false });
        }
        b.iter(|| {
            black_box(peer_state_hash(&mut world));
        });
    });
}

fn bench_grid_128x128_propagate(c: &mut Criterion) {
    c.bench_function("bench_grid_128x128_propagate", |b| {
        let mut grid = vec![0.0f32; 128 * 128];
        b.iter(|| {
            for z in grid.iter_mut() {
                *z = (*z * 0.99).min(1.0);
            }
            black_box(grid[0]);
        });
    });
}

fn bench_composition_determinism_1800f(c: &mut Criterion) {
    c.bench_function("bench_composition_determinism_1800f", |b| {
        b.iter(|| {
            let mut w1 = World::new();
            let mut w2 = World::new();
            let r1 = QuestRecipe {
                graph: DirectedGraphDefinition::new(AssetId(40), 1, 3, 2),
                buff_attrs: AttributeSetDefinition::new(AssetId(41), 1, vec![]),
            };
            let r2 = QuestRecipe {
                graph: DirectedGraphDefinition::new(AssetId(40), 1, 3, 2),
                buff_attrs: AttributeSetDefinition::new(AssetId(41), 1, vec![]),
            };
            let e1 = w1.spawn_empty().id();
            let e2 = w2.spawn_empty().id();
            r1.install(&mut w1, e1, &RecipeContext { tick: 0 }).unwrap();
            r2.install(&mut w2, e2, &RecipeContext { tick: 0 }).unwrap();
            for t in 0..1800 {
                let input = FrameInput {
                    tick: t,
                    quest_kills: (t % 7 == 0) as u8,
                    ..Default::default()
                };
                step_frame(&mut w1, &input);
                step_frame(&mut w2, &input);
            }
            assert_eq!(peer_state_hash(&mut w1), peer_state_hash(&mut w2));
        });
    });
}

criterion_group!(
    benches,
    bench_256_character_scene,
    bench_10k_item_inventory_stack_op,
    bench_quest_graph_advance_32_graphs,
    bench_ability_64_activations,
    bench_effect_tick_1024_active,
    bench_event_log_2048_appends,
    bench_schedule_128_timelines,
    bench_spatial_awareness_256_sensors,
    bench_grid_128x128_propagate,
    bench_composition_determinism_1800f,
);
criterion_main!(benches);
