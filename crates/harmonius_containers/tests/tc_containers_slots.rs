//! Integration tests named after `TC-*` rows in `containers-slots-test-cases.md`.

use std::collections::{HashMap, HashSet};

use glam::Vec3;

use harmonius_containers::{
    AttributeSet, Container, ContainerDef, DragDropRequest, Entity, EquipmentSlot, EquipmentSlots,
    GridContainer, ItemStack, ItemTransferred, Rarity, SlotEntry, Socket, SortCriterion, SortOrder,
    StatModifier, TagSet, TransferError, apply_socket_modifiers, clear_socket_modifiers,
    compute_depth, process_drag_drop, read_strength, sort_container, try_equip, validate_circular,
    validate_nesting_insert, validate_transfer,
};

fn tag_set(tags: &[&str]) -> TagSet {
    TagSet::new(tags.iter().map(|tag| (*tag).to_string()))
}

// TC-16.2.1.1
#[test]
fn test_container_construct_capacity() {
    let def = ContainerDef {
        capacity: 8,
        weight_limit: 100.0,
        required_tags: TagSet::default(),
    };
    let container = Container::new(def);
    assert_eq!(container.len(), 0);
    assert_eq!(container.capacity(), 8);
    assert!(container.is_empty());
}

// TC-16.2.1.2
#[test]
fn test_container_insert_until_full() {
    let def = ContainerDef {
        capacity: 8,
        weight_limit: 100.0,
        required_tags: TagSet::default(),
    };
    let mut container = Container::new(def);
    for idx in 1..=8 {
        container
            .insert(ItemStack::Single {
                entity: Entity::from_raw(idx),
                weight: 1.0,
                name: format!("item{idx}"),
                rarity: Rarity::Common,
                tags: TagSet::default(),
            })
            .unwrap();
    }
    let err = container
        .insert(ItemStack::Single {
            entity: Entity::from_raw(9),
            weight: 1.0,
            name: "item9".to_string(),
            rarity: Rarity::Common,
            tags: TagSet::default(),
        })
        .unwrap_err();
    assert_eq!(err, TransferError::ContainerFull);
    assert_eq!(container.len(), 8);
}

// TC-16.2.1.3
#[test]
fn test_container_remove_frees_slot() {
    let def = ContainerDef {
        capacity: 8,
        weight_limit: 100.0,
        required_tags: TagSet::default(),
    };
    let mut container = Container::new(def);
    for idx in 1..=8 {
        container
            .insert(ItemStack::Single {
                entity: Entity::from_raw(idx),
                weight: 1.0,
                name: format!("item{idx}"),
                rarity: Rarity::Common,
                tags: TagSet::default(),
            })
            .unwrap();
    }
    let removed = container.remove_entity(Entity::from_raw(3)).unwrap();
    assert!(matches!(
        removed,
        SlotEntry::Item {
            entity: Entity(3),
            ..
        }
    ));
    container
        .insert(ItemStack::Single {
            entity: Entity::from_raw(9),
            weight: 1.0,
            name: "item9".to_string(),
            rarity: Rarity::Common,
            tags: TagSet::default(),
        })
        .unwrap();
    assert_eq!(container.len(), 8);
}

// TC-16.2.1.4
#[test]
fn test_container_weight_limit() {
    let def = ContainerDef {
        capacity: 8,
        weight_limit: 50.0,
        required_tags: TagSet::default(),
    };
    let mut container = Container::new(def);
    container
        .insert(ItemStack::Single {
            entity: Entity::from_raw(1),
            weight: 10.0,
            name: "a".to_string(),
            rarity: Rarity::Common,
            tags: TagSet::default(),
        })
        .unwrap();
    container
        .insert(ItemStack::Single {
            entity: Entity::from_raw(2),
            weight: 20.0,
            name: "b".to_string(),
            rarity: Rarity::Common,
            tags: TagSet::default(),
        })
        .unwrap();
    let err = container
        .insert(ItemStack::Single {
            entity: Entity::from_raw(3),
            weight: 25.0,
            name: "c".to_string(),
            rarity: Rarity::Common,
            tags: TagSet::default(),
        })
        .unwrap_err();
    assert_eq!(
        err,
        TransferError::OverWeight {
            limit: 50.0,
            attempted: 55.0,
        }
    );
    assert!((container.total_weight() - 30.0).abs() < f32::EPSILON);
}

// TC-16.2.2.1
#[test]
fn test_grid_insert_3x2() {
    let mut grid = GridContainer::new(10, 4);
    let item = Entity::from_raw(1);
    let (x, y) = grid.insert(3, 2, item).unwrap();
    assert_eq!((x, y), (0, 0));
    for yy in 0..2 {
        for xx in 0..3 {
            assert!(grid.occupancy().is_occupied(xx, yy));
        }
    }
    assert!(!grid.occupancy().is_occupied(9, 3));
}

// TC-16.2.2.2
#[test]
fn test_grid_bin_pack_first_fit() {
    let mut grid = GridContainer::new(10, 4);
    let first = Entity::from_raw(1);
    let second = Entity::from_raw(2);
    assert_eq!(grid.insert(3, 2, first).unwrap(), (0, 0));
    assert_eq!(grid.insert(2, 4, second).unwrap(), (3, 0));
}

// TC-16.2.2.3
#[test]
fn test_grid_overlap_rejected() {
    let mut grid = GridContainer::new(10, 4);
    grid.insert(5, 4, Entity::from_raw(1)).unwrap();
    let err = grid.insert(8, 1, Entity::from_raw(2)).unwrap_err();
    assert_eq!(err, TransferError::NoFreeRegion);
}

// TC-16.2.2.4
#[test]
fn test_grid_remove_clears_cells() {
    let mut grid = GridContainer::new(10, 4);
    let item = Entity::from_raw(1);
    grid.insert(3, 2, item).unwrap();
    grid.remove(item).unwrap();
    for y in 0..4 {
        for x in 0..10 {
            assert!(!grid.occupancy().is_occupied(x, y));
        }
    }
    assert_eq!(grid.insert(3, 2, item).unwrap(), (0, 0));
}

// TC-16.2.3.1
#[test]
fn test_stack_merge_partial() {
    let def = ContainerDef {
        capacity: 8,
        weight_limit: 100.0,
        required_tags: TagSet::default(),
    };
    let mut container = Container::new(def);
    let kind = Entity::from_raw(42);
    container
        .insert(ItemStack::Stack {
            kind,
            count: 10,
            max_stack: 16,
            per_unit_weight: 0.1,
            name: "stack".to_string(),
            rarity: Rarity::Common,
            tags: TagSet::default(),
        })
        .unwrap();
    container
        .insert(ItemStack::Stack {
            kind,
            count: 5,
            max_stack: 16,
            per_unit_weight: 0.1,
            name: "stack".to_string(),
            rarity: Rarity::Common,
            tags: TagSet::default(),
        })
        .unwrap();
    assert_eq!(container.len(), 1);
    match &container.slots()[0] {
        SlotEntry::Stack { count, .. } => assert_eq!(*count, 15),
        _ => panic!("expected merged stack"),
    }
}

// TC-16.2.3.2
#[test]
fn test_stack_overflow_new_slot() {
    let def = ContainerDef {
        capacity: 8,
        weight_limit: 100.0,
        required_tags: TagSet::default(),
    };
    let mut container = Container::new(def);
    let kind = Entity::from_raw(7);
    container
        .insert(ItemStack::Stack {
            kind,
            count: 10,
            max_stack: 16,
            per_unit_weight: 0.1,
            name: "stack".to_string(),
            rarity: Rarity::Common,
            tags: TagSet::default(),
        })
        .unwrap();
    container
        .insert(ItemStack::Stack {
            kind,
            count: 10,
            max_stack: 16,
            per_unit_weight: 0.1,
            name: "stack".to_string(),
            rarity: Rarity::Common,
            tags: TagSet::default(),
        })
        .unwrap();
    assert_eq!(container.len(), 2);
    let counts: Vec<u32> = container
        .slots()
        .iter()
        .filter_map(|slot| match slot {
            SlotEntry::Stack { count, .. } => Some(*count),
            _ => None,
        })
        .collect();
    assert_eq!(counts, vec![16, 4]);
}

// TC-16.2.3.3
#[test]
fn test_stack_incompatible_no_merge() {
    let def = ContainerDef {
        capacity: 8,
        weight_limit: 100.0,
        required_tags: TagSet::default(),
    };
    let mut container = Container::new(def);
    let item_a = Entity::from_raw(1);
    let item_b = Entity::from_raw(2);
    container
        .insert(ItemStack::Stack {
            kind: item_a,
            count: 5,
            max_stack: 99,
            per_unit_weight: 0.1,
            name: "a".to_string(),
            rarity: Rarity::Common,
            tags: TagSet::default(),
        })
        .unwrap();
    container
        .insert(ItemStack::Stack {
            kind: item_b,
            count: 5,
            max_stack: 99,
            per_unit_weight: 0.1,
            name: "b".to_string(),
            rarity: Rarity::Common,
            tags: TagSet::default(),
        })
        .unwrap();
    container
        .insert(ItemStack::Stack {
            kind: item_a,
            count: 5,
            max_stack: 99,
            per_unit_weight: 0.1,
            name: "a".to_string(),
            rarity: Rarity::Common,
            tags: TagSet::default(),
        })
        .unwrap();
    match &container.slots()[0] {
        SlotEntry::Stack { kind, count, .. } => {
            assert_eq!(*kind, item_a);
            assert_eq!(*count, 10);
        }
        _ => panic!("slot0 stack"),
    }
    match &container.slots()[1] {
        SlotEntry::Stack { kind, count, .. } => {
            assert_eq!(*kind, item_b);
            assert_eq!(*count, 5);
        }
        _ => panic!("slot1 stack"),
    }
}

// TC-16.2.4.1
#[test]
fn test_nesting_depth_within_limit() {
    let mut parent = HashMap::new();
    let a = Entity::from_raw(1);
    let b = Entity::from_raw(2);
    let c = Entity::from_raw(3);
    validate_nesting_insert(b, a, &parent, 3).unwrap();
    parent.insert(b, a);
    validate_nesting_insert(c, b, &parent, 3).unwrap();
    parent.insert(c, b);
    assert_eq!(compute_depth(c, &parent), 3);
}

// TC-16.2.4.2
#[test]
fn test_nesting_depth_exceeds() {
    let mut parent = HashMap::new();
    let a = Entity::from_raw(1);
    let b = Entity::from_raw(2);
    let c = Entity::from_raw(3);
    let d = Entity::from_raw(4);
    parent.insert(b, a);
    parent.insert(c, b);
    let err = validate_nesting_insert(d, c, &parent, 3).unwrap_err();
    assert_eq!(
        err,
        TransferError::NestingTooDeep {
            max: 3,
            attempted: 4,
        }
    );
}

// TC-16.2.4.3
#[test]
fn test_nesting_circular_rejected() {
    let mut parent = HashMap::new();
    let a = Entity::from_raw(1);
    let b = Entity::from_raw(2);
    parent.insert(b, a);
    let err = validate_circular(a, b, &parent).unwrap_err();
    assert_eq!(err, TransferError::CircularNesting);
}

// TC-16.2.5.1
#[test]
fn test_sort_by_weight_ascending() {
    let mut slots = vec![
        SlotEntry::Item {
            entity: Entity::from_raw(1),
            weight: 5.0,
            name: "a".to_string(),
            rarity: Rarity::Common,
            tags: TagSet::default(),
        },
        SlotEntry::Item {
            entity: Entity::from_raw(2),
            weight: 1.0,
            name: "b".to_string(),
            rarity: Rarity::Common,
            tags: TagSet::default(),
        },
        SlotEntry::Item {
            entity: Entity::from_raw(3),
            weight: 3.0,
            name: "c".to_string(),
            rarity: Rarity::Common,
            tags: TagSet::default(),
        },
    ];
    sort_container(&mut slots, SortCriterion::Weight, SortOrder::Asc);
    let weights: Vec<f32> = slots
        .iter()
        .map(|slot| match slot {
            SlotEntry::Item { weight, .. } => *weight,
            _ => 0.0,
        })
        .collect();
    assert_eq!(weights, vec![1.0, 3.0, 5.0]);
}

// TC-16.2.5.2
#[test]
fn test_sort_by_name_lexicographic() {
    let mut slots = vec![
        SlotEntry::Item {
            entity: Entity::from_raw(1),
            weight: 1.0,
            name: "sword".to_string(),
            rarity: Rarity::Common,
            tags: TagSet::default(),
        },
        SlotEntry::Item {
            entity: Entity::from_raw(2),
            weight: 1.0,
            name: "apple".to_string(),
            rarity: Rarity::Common,
            tags: TagSet::default(),
        },
        SlotEntry::Item {
            entity: Entity::from_raw(3),
            weight: 1.0,
            name: "potion".to_string(),
            rarity: Rarity::Common,
            tags: TagSet::default(),
        },
    ];
    sort_container(&mut slots, SortCriterion::Name, SortOrder::Asc);
    let names: Vec<&str> = slots
        .iter()
        .map(|slot| match slot {
            SlotEntry::Item { name, .. } => name.as_str(),
            _ => "",
        })
        .collect();
    assert_eq!(names, vec!["apple", "potion", "sword"]);
}

// TC-16.2.5.3
#[test]
fn test_sort_by_rarity_desc() {
    let mut slots = vec![
        SlotEntry::Item {
            entity: Entity::from_raw(1),
            weight: 1.0,
            name: "a".to_string(),
            rarity: Rarity::Common,
            tags: TagSet::default(),
        },
        SlotEntry::Item {
            entity: Entity::from_raw(2),
            weight: 1.0,
            name: "b".to_string(),
            rarity: Rarity::Legendary,
            tags: TagSet::default(),
        },
        SlotEntry::Item {
            entity: Entity::from_raw(3),
            weight: 1.0,
            name: "c".to_string(),
            rarity: Rarity::Rare,
            tags: TagSet::default(),
        },
    ];
    sort_container(&mut slots, SortCriterion::Rarity, SortOrder::Desc);
    let rarities: Vec<Rarity> = slots
        .iter()
        .map(|slot| match slot {
            SlotEntry::Item { rarity, .. } => *rarity,
            _ => Rarity::Common,
        })
        .collect();
    assert_eq!(
        rarities,
        vec![Rarity::Legendary, Rarity::Rare, Rarity::Common]
    );
}

// TC-16.2.5.4
#[test]
fn test_sort_preserves_entities() {
    let mut slots: Vec<SlotEntry> = (1..=5)
        .map(|idx| SlotEntry::Item {
            entity: Entity::from_raw(idx),
            weight: idx as f32,
            name: format!("n{idx}"),
            rarity: Rarity::Common,
            tags: TagSet::default(),
        })
        .collect();
    let before: HashSet<u64> = slots
        .iter()
        .filter_map(|slot| match slot {
            SlotEntry::Item { entity, .. } => Some(entity.raw()),
            _ => None,
        })
        .collect();
    sort_container(&mut slots, SortCriterion::Weight, SortOrder::Asc);
    let after: HashSet<u64> = slots
        .iter()
        .filter_map(|slot| match slot {
            SlotEntry::Item { entity, .. } => Some(entity.raw()),
            _ => None,
        })
        .collect();
    assert_eq!(before, after);
}

// TC-16.2.6.1
#[test]
fn test_socket_tag_match_attach() {
    let mut socket = Socket::new(tag_set(&["gem"]), Vec3::ZERO);
    let item = Entity::from_raw(9);
    socket.attach(item, &tag_set(&["gem"])).unwrap();
    assert_eq!(socket.occupant, Some(item));
}

// TC-16.2.6.2
#[test]
fn test_socket_tag_mismatch_reject() {
    let mut socket = Socket::new(tag_set(&["gem"]), Vec3::ZERO);
    let item = Entity::from_raw(9);
    let err = socket.attach(item, &tag_set(&["weapon"])).unwrap_err();
    assert_eq!(
        err,
        TransferError::TagMismatch {
            required: vec!["gem".to_string()],
            provided: vec!["weapon".to_string()],
        }
    );
    assert_eq!(socket.occupant, None);
}

// TC-16.2.6.3
#[test]
fn test_socket_offset_stored() {
    let offset = Vec3::new(0.0, 1.0, 0.0);
    let mut socket = Socket::new(TagSet::default(), offset);
    let item = Entity::from_raw(1);
    socket.attach(item, &TagSet::default()).unwrap();
    assert_eq!(socket.transform_offset, offset);
    socket.detach();
    assert_eq!(socket.transform_offset, offset);
}

// TC-16.2.9.1
#[test]
fn test_transfer_capacity_check() {
    let def = ContainerDef {
        capacity: 4,
        weight_limit: 100.0,
        required_tags: TagSet::default(),
    };
    let mut container = Container::new(def);
    for idx in 1..=4 {
        container
            .insert(ItemStack::Single {
                entity: Entity::from_raw(idx),
                weight: 1.0,
                name: format!("i{idx}"),
                rarity: Rarity::Common,
                tags: TagSet::default(),
            })
            .unwrap();
    }
    let snapshot = container.clone();
    let err = validate_transfer(&container, 1.0, &TagSet::default()).unwrap_err();
    assert_eq!(err, TransferError::ContainerFull);
    assert_eq!(container.len(), snapshot.len());
    assert_eq!(container.total_weight(), snapshot.total_weight());
}

// TC-16.2.9.2
#[test]
fn test_transfer_weight_check() {
    let def = ContainerDef {
        capacity: 8,
        weight_limit: 50.0,
        required_tags: TagSet::default(),
    };
    let mut container = Container::new(def);
    for idx in 1..=3 {
        container
            .insert(ItemStack::Single {
                entity: Entity::from_raw(idx),
                weight: 15.0,
                name: "x".to_string(),
                rarity: Rarity::Common,
                tags: TagSet::default(),
            })
            .unwrap();
    }
    let err = validate_transfer(&container, 10.0, &TagSet::default()).unwrap_err();
    assert_eq!(
        err,
        TransferError::OverWeight {
            limit: 50.0,
            attempted: 55.0,
        }
    );
}

// TC-16.2.9.3
#[test]
fn test_transfer_tag_check() {
    let def = ContainerDef {
        capacity: 4,
        weight_limit: 50.0,
        required_tags: tag_set(&["consumable"]),
    };
    let container = Container::new(def);
    let err = validate_transfer(&container, 1.0, &tag_set(&["weapon"])).unwrap_err();
    assert_eq!(
        err,
        TransferError::TagMismatch {
            required: vec!["consumable".to_string()],
            provided: vec!["weapon".to_string()],
        }
    );
}

// TC-13.8.1.1
#[test]
fn test_inventory_grid_layout() {
    let mut grid = GridContainer::new(10, 4);
    grid.insert(2, 2, Entity::from_raw(1)).unwrap();
    grid.insert(3, 1, Entity::from_raw(2)).unwrap();
    grid.insert(1, 4, Entity::from_raw(3)).unwrap();
    // Assert no overlaps by brute force occupancy uniqueness per cell.
    let mut seen = HashSet::new();
    for y in 0..4 {
        for x in 0..10 {
            if grid.occupancy().is_occupied(x, y) {
                assert!(seen.insert((x, y)));
            }
        }
    }
    assert_eq!(seen.len(), 11);
}

// TC-13.8.2.1
#[test]
fn test_stackable_resource_merge() {
    let def = ContainerDef {
        capacity: 4,
        weight_limit: 100.0,
        required_tags: TagSet::default(),
    };
    let mut container = Container::new(def);
    let ammo = Entity::from_raw(100);
    container
        .insert(ItemStack::Stack {
            kind: ammo,
            count: 10,
            max_stack: 30,
            per_unit_weight: 0.05,
            name: "ammo".to_string(),
            rarity: Rarity::Common,
            tags: TagSet::default(),
        })
        .unwrap();
    container
        .insert(ItemStack::Stack {
            kind: ammo,
            count: 15,
            max_stack: 30,
            per_unit_weight: 0.05,
            name: "ammo".to_string(),
            rarity: Rarity::Common,
            tags: TagSet::default(),
        })
        .unwrap();
    assert_eq!(container.len(), 1);
    match &container.slots()[0] {
        SlotEntry::Stack { count, .. } => assert_eq!(*count, 25),
        _ => panic!("expected merged ammo stack"),
    }
}

// TC-13.8.3.1
#[test]
fn test_gem_socket_attach() {
    let mut socket = Socket::new(tag_set(&["gem"]), Vec3::ZERO);
    let gem = Entity::from_raw(500);
    socket.attach(gem, &tag_set(&["gem"])).unwrap();
    assert_eq!(socket.occupant, Some(gem));
}

// TC-13.8.4.1
#[test]
fn test_equipment_slot_constraint() {
    let mut slots = EquipmentSlots {
        helmet: EquipmentSlot::new(tag_set(&["helmet"])),
        chest: EquipmentSlot::new(TagSet::default()),
        legs: EquipmentSlot::new(TagSet::default()),
        weapon: EquipmentSlot::new(TagSet::default()),
    };
    let weapon_item = Entity::from_raw(10);
    let err = try_equip(
        &mut slots.helmet,
        "helmet",
        weapon_item,
        &tag_set(&["weapon"]),
    )
    .unwrap_err();
    assert!(matches!(err, TransferError::SlotConstraintMismatch(_)));
    assert_eq!(slots.helmet.occupant, None);
}

// TC-13.8.5.1
#[test]
fn test_drag_drop_validator_routed() {
    let mut source = Container::new(ContainerDef {
        capacity: 8,
        weight_limit: 100.0,
        required_tags: TagSet::default(),
    });
    let mut target = Container::new(ContainerDef {
        capacity: 8,
        weight_limit: 100.0,
        required_tags: TagSet::default(),
    });
    for (idx, name) in [(1u64, "A"), (2, "B"), (3, "C")] {
        source
            .insert(ItemStack::Single {
                entity: Entity::from_raw(idx),
                weight: 1.0,
                name: name.to_string(),
                rarity: Rarity::Common,
                tags: TagSet::default(),
            })
            .unwrap();
    }
    let event = process_drag_drop(DragDropRequest {
        source: &mut source,
        target: &mut target,
        item: Entity::from_raw(2),
    })
    .unwrap();
    assert_eq!(
        event,
        ItemTransferred {
            item: Entity::from_raw(2),
        }
    );
    let source_ids: Vec<u64> = source
        .slots()
        .iter()
        .filter_map(|slot| match slot {
            SlotEntry::Item { entity, .. } => Some(entity.raw()),
            _ => None,
        })
        .collect();
    assert_eq!(source_ids, vec![1, 3]);
    let target_ids: Vec<u64> = target
        .slots()
        .iter()
        .filter_map(|slot| match slot {
            SlotEntry::Item { entity, .. } => Some(entity.raw()),
            _ => None,
        })
        .collect();
    assert_eq!(target_ids, vec![2]);
}

#[test]
fn test_drag_drop_moves_stack_by_kind() {
    let ammo = Entity::from_raw(700);
    let mut source = Container::new(ContainerDef {
        capacity: 8,
        weight_limit: 100.0,
        required_tags: TagSet::default(),
    });
    let mut target = Container::new(ContainerDef {
        capacity: 8,
        weight_limit: 100.0,
        required_tags: TagSet::default(),
    });
    source
        .insert(ItemStack::Stack {
            kind: ammo,
            count: 10,
            max_stack: 30,
            per_unit_weight: 0.05,
            name: "ammo".to_string(),
            rarity: Rarity::Common,
            tags: TagSet::default(),
        })
        .unwrap();
    let event = process_drag_drop(DragDropRequest {
        source: &mut source,
        target: &mut target,
        item: ammo,
    })
    .unwrap();
    assert_eq!(event, ItemTransferred { item: ammo });
    assert!(source.is_empty());
    match &target.slots()[0] {
        SlotEntry::Stack { count, .. } => assert_eq!(*count, 10),
        _ => panic!("expected stack in target"),
    }
}

#[test]
fn test_equipment_slot_rejects_second_equip() {
    let mut slot = EquipmentSlot::new(tag_set(&["helmet"]));
    let helm = Entity::from_raw(1);
    try_equip(&mut slot, "helmet", helm, &tag_set(&["helmet"])).unwrap();
    let err = try_equip(
        &mut slot,
        "helmet",
        Entity::from_raw(2),
        &tag_set(&["helmet"]),
    )
    .unwrap_err();
    assert_eq!(
        err,
        TransferError::EquipmentOccupied {
            slot: "helmet".to_string(),
        }
    );
    assert_eq!(slot.occupant, Some(helm));
}

#[test]
fn test_socket_rejects_double_attach() {
    let mut socket = Socket::new(tag_set(&["gem"]), Vec3::ZERO);
    let g1 = Entity::from_raw(501);
    let g2 = Entity::from_raw(502);
    socket.attach(g1, &tag_set(&["gem"])).unwrap();
    let err = socket.attach(g2, &tag_set(&["gem"])).unwrap_err();
    assert_eq!(err, TransferError::SocketOccupied);
    assert_eq!(socket.occupant, Some(g1));
}

// TC-13.10.3.1
#[test]
fn test_stat_propagate_flat_add() {
    let mut attrs = AttributeSet {
        strength_base: 50,
        strength_bonus: 0,
    };
    let mods = [StatModifier {
        stat: "strength",
        flat_add: 10,
    }];
    apply_socket_modifiers(&mut attrs, &mods);
    assert_eq!(read_strength(&attrs), 60);
    clear_socket_modifiers(&mut attrs, &mods);
    assert_eq!(read_strength(&attrs), 50);
}
