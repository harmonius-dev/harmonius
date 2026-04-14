//! Integration tests for composition plan (TC-16.5.x.y).

use bevy_ecs::prelude::*;
use harmonius_composition::{
    apply_quest_reward, load_crafting_snapshot, load_inventory_snapshot, load_snapshot,
    on_cell_changed_apply_effect, on_entry_appended_advance_graph, on_keyframe_fire_log,
    on_slot_changed_aggregate, peer_state_hash, save_crafting_snapshot, save_inventory_snapshot,
    save_snapshot, step_frame, AbilityCastTarget, AbilityRecipe, AbilityState, AssetId,
    AttributeSet, AttributeSetDefinition, BindError, CellChanged, CompositionEvent,
    CompositionEventQueue, CompositionRecipe, Container, ContainerDefinition, ContainerLayout,
    CraftingRecipe, CraftingState, DefinitionAsset, Destination, DeterministicRng,
    DirectedGraphDefinition, DirectedGraphInstance, Effect, EntryAppended, FrameInput,
    InventoryRecipe, InventoryState, KeyframeFired, Meter, MeterDefinition, QuestRecipe,
    QuestState, RecipeContext, ScheduleRecipe, ScheduleState, SlotChanged, StealthRecipe,
    StealthState,
};

// --- TC-16.5.1.x DefinitionAsset -------------------------------------------------

#[test]
fn test_definition_asset_bind_meter() {
    let mut world = World::new();
    let e = world.spawn_empty().id();
    let def = MeterDefinition::new(AssetId(1), 1, 0.0, 100.0, 50.0);
    let h = def.mint_handle(&mut world);
    def.bind(&mut world, e, h).unwrap();
    let m = world.entity(e).get::<Meter>().unwrap();
    assert!((m.value - 50.0).abs() < f32::EPSILON);
    assert!((m.min - 0.0).abs() < f32::EPSILON);
    assert!((m.max - 100.0).abs() < f32::EPSILON);
}

#[test]
fn test_definition_asset_bind_attrset() {
    let mut world = World::new();
    let e = world.spawn_empty().id();
    let def = AttributeSetDefinition::new(
        AssetId(2),
        1,
        vec![("strength".into(), 10.0), ("agility".into(), 8.0)],
    );
    let h = def.mint_handle(&mut world);
    def.bind(&mut world, e, h).unwrap();
    let a = world.entity(e).get::<AttributeSet>().unwrap();
    assert_eq!(a.values.get("strength").copied(), Some(10.0));
    assert_eq!(a.values.get("agility").copied(), Some(8.0));
}

#[test]
fn test_definition_asset_bind_container() {
    let mut world = World::new();
    let e = world.spawn_empty().id();
    let def = ContainerDefinition::new(AssetId(3), 1, 4, 8, 32);
    let h = def.mint_handle(&mut world);
    def.bind(&mut world, e, h).unwrap();
    let c = world.entity(e).get::<Container>().unwrap();
    assert_eq!(c.capacity, 32);
    assert_eq!(c.slots.len(), 32);
    assert!(matches!(
        c.layout,
        ContainerLayout::Grid { rows: 4, cols: 8 }
    ));
    assert!(c.slots.iter().all(|s| s.is_none()));
}

#[test]
fn test_definition_asset_bind_graph() {
    let mut world = World::new();
    let e = world.spawn_empty().id();
    let def = DirectedGraphDefinition::new(AssetId(4), 1, 5, 4);
    let h = def.mint_handle(&mut world);
    def.bind(&mut world, e, h).unwrap();
    let g = world.entity(e).get::<DirectedGraphInstance>().unwrap();
    assert_eq!(g.nodes, 5);
    assert_eq!(g.edges, 4);
}

#[test]
fn test_definition_asset_unbind_removes() {
    let mut world = World::new();
    let e = world.spawn_empty().id();
    let def = MeterDefinition::new(AssetId(5), 1, 0.0, 1.0, 0.5);
    let h = def.mint_handle(&mut world);
    def.bind(&mut world, e, h).unwrap();
    def.unbind(&mut world, e).unwrap();
    assert!(!world.entity(e).contains::<Meter>());
}

#[test]
fn test_definition_asset_version_mismatch() {
    let mut world = World::new();
    let e = world.spawn_empty().id();
    let def = MeterDefinition::new(AssetId(6), 2, 0.0, 1.0, 0.5);
    let mut h = def.mint_handle(&mut world);
    h.version = 1;
    let err = def.bind(&mut world, e, h).unwrap_err();
    assert_eq!(
        err,
        BindError::VersionMismatch {
            expected: 2,
            actual: 1
        }
    );
}

#[test]
fn test_definition_asset_idempotent_bind() {
    let mut world = World::new();
    let e = world.spawn_empty().id();
    let def = MeterDefinition::new(AssetId(7), 1, 0.0, 10.0, 5.0);
    let h = def.mint_handle(&mut world);
    def.bind(&mut world, e, h).unwrap();
    def.bind(&mut world, e, h).unwrap();
    assert!(world.entity(e).contains::<Meter>());
}

#[test]
fn test_bind_error_invalid_handle() {
    let mut world = World::new();
    let e = world.spawn_empty().id();
    let def = MeterDefinition::new(AssetId(8), 1, 0.0, 1.0, 0.5);
    let h = def.mint_handle(&mut world);
    def.bind(&mut world, e, h).unwrap();
    def.unbind(&mut world, e).unwrap();
    let err = def.bind(&mut world, e, h).unwrap_err();
    assert_eq!(err, BindError::InvalidHandle);
}

#[test]
fn test_recipe_handle_tracks_definitions() {
    let mut world = World::new();
    let root = world.spawn_empty().id();
    let recipe = QuestRecipe {
        graph: DirectedGraphDefinition::new(AssetId(10), 1, 3, 2),
        buff_attrs: AttributeSetDefinition::new(AssetId(11), 1, vec![]),
    };
    let handle = recipe
        .install(&mut world, root, &RecipeContext { tick: 0 })
        .unwrap();
    assert_eq!(handle.definitions.len(), 2);
}

#[test]
fn test_recipe_uninstall_clears_entities() {
    let mut world = World::new();
    let root = world.spawn_empty().id();
    let recipe = QuestRecipe {
        graph: DirectedGraphDefinition::new(AssetId(12), 1, 2, 1),
        buff_attrs: AttributeSetDefinition::new(AssetId(13), 1, vec![]),
    };
    let handle = recipe
        .install(&mut world, root, &RecipeContext { tick: 0 })
        .unwrap();
    recipe.uninstall(&mut world, &handle).unwrap();
    assert!(!world.entity(root).contains::<DirectedGraphInstance>());
    assert!(!world.entity(root).contains::<AttributeSet>());
    assert!(!world.entity(root).contains::<QuestState>());
}

// --- TC-16.5.2.x Recipes --------------------------------------------------------

#[test]
fn test_quest_recipe_end_to_end() {
    let mut world = World::new();
    let player = world.spawn_empty().id();
    let recipe = QuestRecipe {
        graph: DirectedGraphDefinition::new(AssetId(20), 1, 3, 2),
        buff_attrs: AttributeSetDefinition::new(AssetId(21), 1, vec![]),
    };
    let rh = recipe
        .install(&mut world, player, &RecipeContext { tick: 0 })
        .unwrap();
    for _ in 0..3 {
        step_frame(
            &mut world,
            &FrameInput {
                tick: 1,
                quest_kills: 1,
                ..Default::default()
            },
        );
    }
    let g = world.entity(player).get::<DirectedGraphInstance>().unwrap();
    assert_eq!(g.current_node, 2);
    apply_quest_reward(&mut world, player);
    assert!(world.entity(player).contains::<Effect>());
    let _ = rh;
}

#[test]
fn test_ability_recipe_end_to_end() {
    let mut world = World::new();
    let caster = world.spawn_empty().id();
    let target = world.spawn_empty().id();
    let recipe = AbilityRecipe {
        graph: DirectedGraphDefinition::new(AssetId(30), 1, 2, 1),
        mana: MeterDefinition::new(AssetId(31), 1, 0.0, 200.0, 100.0),
    };
    recipe
        .install(&mut world, caster, &RecipeContext { tick: 0 })
        .unwrap();
    world
        .entity_mut(caster)
        .insert(AbilityCastTarget { target });
    step_frame(
        &mut world,
        &FrameInput {
            tick: 1,
            ability_cast: true,
            ..Default::default()
        },
    );
    let a = world.entity(caster).get::<AbilityState>().unwrap();
    assert!((a.mana - 80.0).abs() < f32::EPSILON);
    let eff = world.entity(target).get::<Effect>().unwrap();
    assert_eq!(eff.name, "damage");
    assert!((eff.magnitude - 10.0).abs() < f32::EPSILON);
}

#[test]
fn test_inventory_recipe_end_to_end() {
    let mut world = World::new();
    let inv = world.spawn_empty().id();
    let recipe = InventoryRecipe {
        container: ContainerDefinition::new(AssetId(40), 1, 2, 4, 8),
        stats: AttributeSetDefinition::new(AssetId(41), 1, vec![("attack".into(), 10.0)]),
    };
    recipe
        .install(&mut world, inv, &RecipeContext { tick: 0 })
        .unwrap();
    on_slot_changed_aggregate(
        &mut world,
        &SlotChanged {
            owner: inv,
            slot: 0,
        },
    );
    let attrs = world.entity(inv).get::<AttributeSet>().unwrap();
    assert!((attrs.values["attack"] - 15.0).abs() < f32::EPSILON);
}

#[test]
fn test_crafting_recipe_end_to_end() {
    let mut world = World::new();
    let bench = world.spawn_empty().id();
    let recipe = CraftingRecipe {
        graph: DirectedGraphDefinition::new(AssetId(50), 1, 2, 1),
        container: ContainerDefinition::new(AssetId(51), 1, 1, 4, 8),
    };
    recipe
        .install(&mut world, bench, &RecipeContext { tick: 0 })
        .unwrap();
    step_frame(
        &mut world,
        &FrameInput {
            tick: 1,
            craft_attempt: true,
            ..Default::default()
        },
    );
    let c = world.entity(bench).get::<CraftingState>().unwrap();
    assert_eq!(c.herbs, 0);
    assert_eq!(c.vials, 0);
    assert_eq!(c.potions, 1);
}

#[test]
fn test_stealth_recipe_end_to_end() {
    let mut world = World::new();
    let player = world.spawn_empty().id();
    let recipe = StealthRecipe {
        attrs: AttributeSetDefinition::new(AssetId(60), 1, vec![]),
    };
    recipe
        .install(&mut world, player, &RecipeContext { tick: 0 })
        .unwrap();
    step_frame(
        &mut world,
        &FrameInput {
            tick: 1,
            stealth_move: true,
            ..Default::default()
        },
    );
    let s = world.entity(player).get::<StealthState>().unwrap();
    assert!(s.detected);
}

#[test]
fn test_schedule_recipe_end_to_end() {
    let mut world = World::new();
    let npc = world.spawn_empty().id();
    let recipe = ScheduleRecipe {
        role_table: AttributeSetDefinition::new(AssetId(70), 1, vec![]),
    };
    recipe
        .install(&mut world, npc, &RecipeContext { tick: 0 })
        .unwrap();
    step_frame(
        &mut world,
        &FrameInput {
            tick: 1,
            schedule_ticks: 540,
            ..Default::default()
        },
    );
    assert!(world.entity(npc).get::<ScheduleState>().unwrap().fired);
    let dest = world.entity(npc).get::<Destination>().unwrap();
    assert!(world.get_entity(dest.target).is_ok());
}

// --- TC-16.5.3.x Cross-primitive --------------------------------------------------

#[test]
fn test_event_log_to_graph_advance() {
    let mut world = World::new();
    let qe = world.spawn_empty().id();
    let graph = DirectedGraphDefinition::new(AssetId(80), 1, 3, 2);
    let gh = graph.mint_handle(&mut world);
    graph.bind(&mut world, qe, gh).unwrap();
    world.entity_mut(qe).insert(QuestState {
        active_objective: 0,
        objectives: 3,
    });
    on_entry_appended_advance_graph(
        &mut world,
        &EntryAppended {
            target: qe,
            label: "kill",
        },
    );
    assert_eq!(
        world
            .entity(qe)
            .get::<DirectedGraphInstance>()
            .unwrap()
            .current_node,
        1
    );
}

#[test]
fn test_container_to_attribute_propagation() {
    let mut world = World::new();
    let e = world.spawn_empty().id();
    let stats = AttributeSetDefinition::new(AssetId(90), 1, vec![("attack".into(), 10.0)]);
    let sh = stats.mint_handle(&mut world);
    stats.bind(&mut world, e, sh).unwrap();
    on_slot_changed_aggregate(&mut world, &SlotChanged { owner: e, slot: 0 });
    assert!(
        (world.entity(e).get::<AttributeSet>().unwrap().values["attack"] - 15.0).abs()
            < f32::EPSILON
    );
}

#[test]
fn test_grid_to_effect_trigger() {
    let mut world = World::new();
    let cell = world.spawn_empty().id();
    on_cell_changed_apply_effect(
        &mut world,
        &CellChanged {
            owner: cell,
            value: 1.0,
        },
    );
    let eff = world.entity(cell).get::<Effect>().unwrap();
    assert_eq!(eff.name, "grid_pulse");
}

#[test]
fn test_timeline_to_event_log_fire() {
    let mut world = World::new();
    world.insert_resource(CompositionEventQueue::default());
    let owner = world.spawn_empty().id();
    let fired = KeyframeFired { owner, tick: 540 };
    on_keyframe_fire_log(&mut world, &fired);
    let evt = world
        .resource::<CompositionEventQueue>()
        .events
        .front()
        .cloned()
        .unwrap();
    match evt {
        CompositionEvent::EntryAppended(e) => {
            assert_eq!(e.target, owner);
            assert_eq!(e.label, "keyframe");
        }
        other => panic!("expected EntryAppended, got {other:?}"),
    }
}

// --- TC-16.5.4.x Determinism ----------------------------------------------------

fn run_peer_pair(
    frames: u64,
    mut build: impl FnMut() -> (World, Entity),
    input_fn: impl Fn(u64) -> FrameInput,
) {
    let (mut w1, e1) = build();
    let (mut w2, e2) = build();
    for t in 0..frames {
        let input = input_fn(t);
        step_frame(&mut w1, &input);
        step_frame(&mut w2, &input);
        assert_eq!(peer_state_hash(&mut w1), peer_state_hash(&mut w2));
    }
    let _ = (e1, e2);
}

#[test]
fn test_two_peer_determinism_quest() {
    run_peer_pair(
        600,
        || {
            let mut world = World::new();
            let e = world.spawn_empty().id();
            let recipe = QuestRecipe {
                graph: DirectedGraphDefinition::new(AssetId(100), 1, 8, 7),
                buff_attrs: AttributeSetDefinition::new(AssetId(101), 1, vec![]),
            };
            recipe
                .install(&mut world, e, &RecipeContext { tick: 0 })
                .unwrap();
            (world, e)
        },
        |t| FrameInput {
            tick: t,
            quest_kills: (t % 11 == 0) as u8,
            ..Default::default()
        },
    );
}

#[test]
fn test_two_peer_determinism_ability() {
    run_peer_pair(
        600,
        || {
            let mut world = World::new();
            let e = world.spawn_empty().id();
            let recipe = AbilityRecipe {
                graph: DirectedGraphDefinition::new(AssetId(110), 1, 3, 2),
                mana: MeterDefinition::new(AssetId(111), 1, 0.0, 100.0, 100.0),
            };
            recipe
                .install(&mut world, e, &RecipeContext { tick: 0 })
                .unwrap();
            (world, e)
        },
        |t| FrameInput {
            tick: t,
            ability_cast: (t % 5 == 0),
            ..Default::default()
        },
    );
}

#[test]
fn test_two_peer_determinism_combined() {
    run_peer_pair(
        1800,
        || {
            let mut world = World::new();
            let ctx = RecipeContext { tick: 0 };
            let quest_e = world.spawn_empty().id();
            QuestRecipe {
                graph: DirectedGraphDefinition::new(AssetId(120), 1, 4, 3),
                buff_attrs: AttributeSetDefinition::new(AssetId(121), 1, vec![]),
            }
            .install(&mut world, quest_e, &ctx)
            .unwrap();
            let ability_e = world.spawn_empty().id();
            AbilityRecipe {
                graph: DirectedGraphDefinition::new(AssetId(122), 1, 3, 2),
                mana: MeterDefinition::new(AssetId(123), 1, 0.0, 200.0, 100.0),
            }
            .install(&mut world, ability_e, &ctx)
            .unwrap();
            let inv_e = world.spawn_empty().id();
            InventoryRecipe {
                container: ContainerDefinition::new(AssetId(124), 1, 2, 4, 8),
                stats: AttributeSetDefinition::new(AssetId(125), 1, vec![("attack".into(), 10.0)]),
            }
            .install(&mut world, inv_e, &ctx)
            .unwrap();
            let craft_e = world.spawn_empty().id();
            CraftingRecipe {
                graph: DirectedGraphDefinition::new(AssetId(126), 1, 2, 1),
                container: ContainerDefinition::new(AssetId(127), 1, 1, 4, 8),
            }
            .install(&mut world, craft_e, &ctx)
            .unwrap();
            let stealth_e = world.spawn_empty().id();
            StealthRecipe {
                attrs: AttributeSetDefinition::new(AssetId(128), 1, vec![]),
            }
            .install(&mut world, stealth_e, &ctx)
            .unwrap();
            let sched_e = world.spawn_empty().id();
            ScheduleRecipe {
                role_table: AttributeSetDefinition::new(AssetId(129), 1, vec![]),
            }
            .install(&mut world, sched_e, &ctx)
            .unwrap();
            (world, quest_e)
        },
        |t| {
            let mut r = DeterministicRng::from_tick(t, 0xC0_57_16_15);
            FrameInput {
                tick: t,
                quest_kills: (r.next_u32() % 3) as u8,
                ability_cast: r.next_u32() % 2 == 0,
                inventory_equip: r.next_u32() % 2 == 0,
                craft_attempt: r.next_u32() % 2 == 0,
                stealth_move: r.next_u32() % 2 == 0,
                schedule_ticks: u64::from(r.next_u32() % 4),
            }
        },
    );
}

// --- TC-16.5.7.x Save / load ------------------------------------------------------

#[test]
fn test_save_load_quest_state() {
    let mut world = World::new();
    let e = world.spawn_empty().id();
    let graph = DirectedGraphDefinition::new(AssetId(200), 1, 5, 4);
    let gh = graph.mint_handle(&mut world);
    graph.bind(&mut world, e, gh).unwrap();
    world.entity_mut(e).insert(QuestState {
        active_objective: 2,
        objectives: 5,
    });
    let bytes = save_snapshot(&world, e).unwrap();
    world.entity_mut(e).insert(QuestState {
        active_objective: 0,
        objectives: 5,
    });
    load_snapshot(&mut world, e, &bytes).unwrap();
    assert_eq!(
        world
            .entity(e)
            .get::<QuestState>()
            .unwrap()
            .active_objective,
        2
    );
}

#[test]
fn test_save_load_inventory_with_effects() {
    let mut world = World::new();
    let e = world.spawn_empty().id();
    let stats = AttributeSetDefinition::new(AssetId(210), 1, vec![("attack".into(), 12.0)]);
    let sh = stats.mint_handle(&mut world);
    stats.bind(&mut world, e, sh).unwrap();
    world.entity_mut(e).insert(InventoryState { attack: 7.0 });
    world.entity_mut(e).insert(Effect {
        name: "equip_buff".into(),
        magnitude: 3.0,
    });
    let bytes = save_inventory_snapshot(&world, e).unwrap();
    world.entity_mut(e).remove::<Effect>();
    world.entity_mut(e).insert(InventoryState { attack: 0.0 });
    load_inventory_snapshot(&mut world, e, &bytes).unwrap();
    assert!((world.entity(e).get::<InventoryState>().unwrap().attack - 7.0).abs() < f32::EPSILON);
    let eff = world.entity(e).get::<Effect>().unwrap();
    assert_eq!(eff.name, "equip_buff");
}

#[test]
fn test_save_load_crafting_progress() {
    let mut world = World::new();
    let e = world.spawn_empty().id();
    world.entity_mut(e).insert(CraftingState {
        herbs: 1,
        vials: 0,
        potions: 2,
    });
    world.entity_mut(e).insert(ScheduleState {
        elapsed: 400,
        fire_at: 540,
        fired: false,
    });
    let bytes = save_crafting_snapshot(&world, e).unwrap();
    world.entity_mut(e).insert(CraftingState {
        herbs: 9,
        vials: 9,
        potions: 9,
    });
    load_crafting_snapshot(&mut world, e, &bytes).unwrap();
    let c = world.entity(e).get::<CraftingState>().unwrap();
    assert_eq!(c.herbs, 1);
    assert_eq!(c.potions, 2);
    assert_eq!(world.entity(e).get::<ScheduleState>().unwrap().elapsed, 400);
}
