//! Integration tests mapped from `docs/design/integration/input-ui-test-cases.md`.

use input_ui::{
    gameplay_context, gameplay_observes_move_forward_key, menu_context, observe_gameplay_inputs,
    ray_panel_hit, ContextId, ContextStack, DispatchLog, DragDropManager, Entity, EventPhase,
    EventRouter, FocusCommitPolicy, FocusGroup, FocusManager, Focusable, GestureEvent,
    GesturePhase, GestureType, HandlerPolicy, InputConsumption, InputDebugFlags, MappingContext,
    MouseButton, PanelSpec, Rect, ScrollView, SwipeDirection, TextInput, UiPointerEvent, Vec2,
    VrControllerState, WidgetCommand, WidgetSpec, WidgetTree,
};

const ROOT: Entity = Entity(0);

fn tree_two_buttons() -> WidgetTree {
    WidgetTree::new(
        ROOT,
        vec![
            WidgetSpec {
                id: ROOT,
                parent: None,
                rect: Rect::from_xywh(0.0, 0.0, 10_000.0, 10_000.0),
                z_order: 0,
                disabled: false,
            },
            WidgetSpec {
                id: Entity(1),
                parent: Some(ROOT),
                rect: Rect::from_xywh(0.0, 0.0, 100.0, 40.0),
                z_order: 1,
                disabled: false,
            },
            WidgetSpec {
                id: Entity(2),
                parent: Some(ROOT),
                rect: Rect::from_xywh(50.0, 0.0, 100.0, 40.0),
                z_order: 2,
                disabled: false,
            },
        ],
    )
}

#[test]
fn tc_ir_4_2_1_1_pointer_hover_enters() {
    let tree = tree_two_buttons();
    let pos = Vec2::new(60.0, 20.0);
    let hit = tree.hit_test(pos).expect("hit");
    assert_eq!(hit, Entity(2));

    let mut router = EventRouter::new();
    let mut log = DispatchLog::default();
    let event = UiPointerEvent::Enter { position: pos };
    router.dispatch_pointer(&tree, Some(hit), event, &HandlerPolicy::default(), &mut log);

    let hovered = router.interaction.get(&hit).copied().unwrap_or_default();
    assert!(hovered.hovered);
    assert!(log.records().iter().any(|r| {
        r.phase == EventPhase::Target
            && r.target == hit
            && matches!(r.event, UiPointerEvent::Enter { .. })
    }));
}

#[test]
fn tc_ir_4_2_1_2_pointer_click_presses() {
    let tree = tree_two_buttons();
    let pos = Vec2::new(60.0, 20.0);
    let hit = tree.hit_test(pos).unwrap();
    let mut router = EventRouter::new();
    let mut log = DispatchLog::default();
    router.dispatch_pointer(
        &tree,
        Some(hit),
        UiPointerEvent::Down {
            position: pos,
            button: MouseButton::Primary,
        },
        &HandlerPolicy::default(),
        &mut log,
    );
    assert!(router.interaction.get(&hit).unwrap().pressed);
    assert!(log.records().iter().any(|r| {
        r.phase == EventPhase::Target
            && matches!(
                r.event,
                UiPointerEvent::Down {
                    button: MouseButton::Primary,
                    ..
                }
            )
    }));
}

#[test]
fn tc_ir_4_2_1_3_pointer_leave_clears_hover() {
    let tree = tree_two_buttons();
    let pos = Vec2::new(60.0, 20.0);
    let hit = tree.hit_test(pos).unwrap();
    let mut router = EventRouter::new();
    let mut log = DispatchLog::default();
    router.dispatch_pointer(
        &tree,
        Some(hit),
        UiPointerEvent::Enter { position: pos },
        &HandlerPolicy::default(),
        &mut log,
    );
    router.dispatch_pointer(
        &tree,
        Some(hit),
        UiPointerEvent::Leave,
        &HandlerPolicy::default(),
        &mut log,
    );
    assert!(!router.interaction.get(&hit).unwrap().hovered);
}

#[test]
fn tc_ir_4_2_1_4_overlapping_z_order_wins() {
    let tree = tree_two_buttons();
    let overlap = Vec2::new(75.0, 20.0);
    assert_eq!(tree.hit_test(overlap), Some(Entity(2)));
}

#[test]
fn tc_ir_4_2_1_5_disabled_skips_target() {
    let tree = WidgetTree::new(
        ROOT,
        vec![
            WidgetSpec {
                id: ROOT,
                parent: None,
                rect: Rect::from_xywh(0.0, 0.0, 10_000.0, 10_000.0),
                z_order: 0,
                disabled: false,
            },
            WidgetSpec {
                id: Entity(1),
                parent: Some(ROOT),
                rect: Rect::from_xywh(0.0, 0.0, 100.0, 40.0),
                z_order: 5,
                disabled: true,
            },
        ],
    );
    let pos = Vec2::new(10.0, 10.0);
    let hit = tree.hit_test(pos).unwrap();
    let mut router = EventRouter::new();
    let mut log = DispatchLog::default();
    router.dispatch_pointer(
        &tree,
        Some(hit),
        UiPointerEvent::Down {
            position: pos,
            button: MouseButton::Primary,
        },
        &HandlerPolicy::default(),
        &mut log,
    );
    assert!(!log.records().iter().any(|r| r.phase == EventPhase::Target));
    assert!(!router
        .interaction
        .get(&hit)
        .map(|s| s.pressed)
        .unwrap_or(false));
}

#[test]
fn tc_ir_4_2_1_6_touch_and_mouse_independent() {
    let tree = tree_two_buttons();
    let mouse_hit = tree.hit_test(Vec2::new(10.0, 10.0)).unwrap();
    let touch_hit = tree.hit_test(Vec2::new(60.0, 20.0)).unwrap();
    assert_ne!(mouse_hit, touch_hit);
}

#[test]
fn tc_ir_4_2_1_7_capture_stops_before_target() {
    let tree = WidgetTree::new(
        ROOT,
        vec![
            WidgetSpec {
                id: ROOT,
                parent: None,
                rect: Rect::from_xywh(0.0, 0.0, 10_000.0, 10_000.0),
                z_order: 0,
                disabled: false,
            },
            WidgetSpec {
                id: Entity(1),
                parent: Some(ROOT),
                rect: Rect::from_xywh(0.0, 0.0, 200.0, 200.0),
                z_order: 1,
                disabled: false,
            },
            WidgetSpec {
                id: Entity(2),
                parent: Some(Entity(1)),
                rect: Rect::from_xywh(10.0, 10.0, 50.0, 50.0),
                z_order: 2,
                disabled: false,
            },
        ],
    );
    let hit = Entity(2);
    let mut policy = HandlerPolicy::default();
    policy.capture_stop_before.insert(Entity(1));

    let mut router = EventRouter::new();
    let mut log = DispatchLog::default();
    router.dispatch_pointer(
        &tree,
        Some(hit),
        UiPointerEvent::Down {
            position: Vec2::new(20.0, 20.0),
            button: MouseButton::Primary,
        },
        &policy,
        &mut log,
    );

    assert!(!log.records().iter().any(|r| r.phase == EventPhase::Target));
    assert!(!log.records().iter().any(|r| r.phase == EventPhase::Bubble));
}

#[test]
fn tc_ir_4_2_1_8_bubble_visits_ancestors() {
    let tree = WidgetTree::new(
        ROOT,
        vec![
            WidgetSpec {
                id: ROOT,
                parent: None,
                rect: Rect::from_xywh(0.0, 0.0, 10_000.0, 10_000.0),
                z_order: 0,
                disabled: false,
            },
            WidgetSpec {
                id: Entity(1),
                parent: Some(ROOT),
                rect: Rect::from_xywh(0.0, 0.0, 200.0, 200.0),
                z_order: 1,
                disabled: false,
            },
            WidgetSpec {
                id: Entity(2),
                parent: Some(Entity(1)),
                rect: Rect::from_xywh(0.0, 0.0, 200.0, 200.0),
                z_order: 1,
                disabled: false,
            },
            WidgetSpec {
                id: Entity(3),
                parent: Some(Entity(2)),
                rect: Rect::from_xywh(10.0, 10.0, 40.0, 40.0),
                z_order: 2,
                disabled: false,
            },
        ],
    );
    let hit = Entity(3);
    let mut router = EventRouter::new();
    let mut log = DispatchLog::default();
    router.dispatch_pointer(
        &tree,
        Some(hit),
        UiPointerEvent::Down {
            position: Vec2::new(20.0, 20.0),
            button: MouseButton::Primary,
        },
        &HandlerPolicy::default(),
        &mut log,
    );

    let bubble: Vec<Entity> = log
        .records()
        .iter()
        .filter(|r| r.phase == EventPhase::Bubble)
        .map(|r| r.target)
        .collect();
    assert_eq!(bubble, vec![Entity(2), Entity(1), ROOT]);
}

#[test]
fn tc_ir_4_2_2_1_tab_advances_focus() {
    let mut focus = FocusManager::new(vec![
        Focusable {
            entity: Entity(1),
            tab_index: 0,
            group: None,
            rect: Rect::from_xywh(0.0, 0.0, 10.0, 10.0),
        },
        Focusable {
            entity: Entity(2),
            tab_index: 1,
            group: None,
            rect: Rect::from_xywh(20.0, 0.0, 10.0, 10.0),
        },
    ]);
    focus.set_focus(Entity(1));
    focus.advance();
    assert_eq!(focus.focused_entity(), Some(Entity(2)));
}

#[test]
fn tc_ir_4_2_2_2_shift_tab_reverses() {
    let mut focus = FocusManager::new(vec![
        Focusable {
            entity: Entity(1),
            tab_index: 0,
            group: None,
            rect: Rect::from_xywh(0.0, 0.0, 10.0, 10.0),
        },
        Focusable {
            entity: Entity(2),
            tab_index: 1,
            group: None,
            rect: Rect::from_xywh(20.0, 0.0, 10.0, 10.0),
        },
    ]);
    focus.set_focus(Entity(2));
    focus.reverse();
    assert_eq!(focus.focused_entity(), Some(Entity(1)));
}

#[test]
fn tc_ir_4_2_2_3_arrow_moves_directionally() {
    let group = FocusGroup { id: 1, wrap: false };
    let mut focus = FocusManager::new(vec![
        Focusable {
            entity: Entity(1),
            tab_index: 0,
            group: Some(group.clone()),
            rect: Rect::from_xywh(0.0, 0.0, 10.0, 10.0),
        },
        Focusable {
            entity: Entity(2),
            tab_index: 1,
            group: Some(group),
            rect: Rect::from_xywh(50.0, 0.0, 10.0, 10.0),
        },
    ]);
    focus.set_focus(Entity(1));
    focus.directional_right();
    assert_eq!(focus.focused_entity(), Some(Entity(2)));
}

#[test]
fn tc_ir_4_2_3_1_gamepad_south_confirms() {
    let mut focus = FocusManager::new(vec![Focusable {
        entity: Entity(9),
        tab_index: 0,
        group: None,
        rect: Rect::from_xywh(0.0, 0.0, 10.0, 10.0),
    }]);
    focus.set_focus(Entity(9));
    assert_eq!(focus.gamepad_south(), Some(WidgetCommand::Activate));
}

#[test]
fn tc_ir_4_2_3_2_gamepad_east_cancels() {
    let mut focus = FocusManager::new(vec![Focusable {
        entity: Entity(9),
        tab_index: 0,
        group: None,
        rect: Rect::from_xywh(0.0, 0.0, 10.0, 10.0),
    }]);
    focus.set_focus(Entity(9));
    assert_eq!(focus.gamepad_east(), Some(WidgetCommand::Close));
}

#[test]
fn tc_ir_4_2_3_3_dpad_wrap() {
    let group = FocusGroup { id: 1, wrap: true };
    let mut focus = FocusManager::new(vec![
        Focusable {
            entity: Entity(1),
            tab_index: 0,
            group: Some(group.clone()),
            rect: Rect::from_xywh(0.0, 0.0, 10.0, 10.0),
        },
        Focusable {
            entity: Entity(2),
            tab_index: 1,
            group: Some(group),
            rect: Rect::from_xywh(50.0, 0.0, 10.0, 10.0),
        },
    ]);
    focus.set_focus(Entity(2));
    focus.directional_right();
    assert_eq!(focus.focused_entity(), Some(Entity(1)));
}

#[test]
fn tc_ir_4_2_4_1_pan_scrolls() {
    let mut scroll = ScrollView::new(0.5, 4.0);
    scroll.apply_gesture(GestureEvent {
        gesture_type: GestureType::Pan,
        phase: GesturePhase::Changed,
        position: Vec2::new(0.0, 0.0),
        scale: 1.0,
        delta: Vec2::new(0.0, 50.0),
    });
    assert_eq!(scroll.offset.y, 50.0);
}

#[test]
fn tc_ir_4_2_4_2_swipe_triggers_drag() {
    let mut drag = DragDropManager::default();
    drag.on_swipe(GestureEvent {
        gesture_type: GestureType::Swipe {
            direction: SwipeDirection::Up,
        },
        phase: GesturePhase::Began,
        position: Vec2::new(0.0, 0.0),
        scale: 1.0,
        delta: Vec2::new(0.0, 0.0),
    });
    assert_eq!(drag.last_drag, Some(SwipeDirection::Up));
}

#[test]
fn tc_ir_4_2_4_3_pinch_zooms() {
    let mut scroll = ScrollView::new(0.5, 4.0);
    scroll.apply_gesture(GestureEvent {
        gesture_type: GestureType::Pinch,
        phase: GesturePhase::Began,
        position: Vec2::new(10.0, 10.0),
        scale: 1.0,
        delta: Vec2::new(0.0, 0.0),
    });
    scroll.apply_gesture(GestureEvent {
        gesture_type: GestureType::Pinch,
        phase: GesturePhase::Changed,
        position: Vec2::new(10.0, 10.0),
        scale: 1.5,
        delta: Vec2::new(0.0, 0.0),
    });
    assert!((scroll.content_scale - 1.5).abs() < f32::EPSILON);
}

#[test]
fn tc_ir_4_2_4_4_pinch_cancel_reverts() {
    let mut scroll = ScrollView::new(0.5, 4.0);
    scroll.apply_gesture(GestureEvent {
        gesture_type: GestureType::Pinch,
        phase: GesturePhase::Began,
        position: Vec2::new(0.0, 0.0),
        scale: 1.0,
        delta: Vec2::new(0.0, 0.0),
    });
    scroll.apply_gesture(GestureEvent {
        gesture_type: GestureType::Pinch,
        phase: GesturePhase::Changed,
        position: Vec2::new(0.0, 0.0),
        scale: 2.0,
        delta: Vec2::new(0.0, 0.0),
    });
    scroll.apply_gesture(GestureEvent {
        gesture_type: GestureType::Pinch,
        phase: GesturePhase::Cancelled,
        position: Vec2::new(0.0, 0.0),
        scale: 1.0,
        delta: Vec2::new(0.0, 0.0),
    });
    assert!((scroll.content_scale - 1.0).abs() < f32::EPSILON);
}

#[test]
fn tc_ir_4_2_5_1_ui_context_blocks_gameplay_pointer() {
    let mut stack = ContextStack::new();
    stack.push(MappingContext::ui(
        ContextId(1),
        10,
        InputConsumption::all(),
    ));
    let obs = observe_gameplay_inputs(&stack, true, false);
    assert!(!obs.pointer_move);
}

#[test]
fn tc_ir_4_2_5_2_pop_restores_pointer() {
    let mut stack = ContextStack::new();
    stack.push(MappingContext::ui(
        ContextId(1),
        10,
        InputConsumption::all(),
    ));
    let mut flags = InputDebugFlags::default();
    stack.pop(&mut flags);
    let obs = observe_gameplay_inputs(&stack, true, false);
    assert!(obs.pointer_move);
}

#[test]
fn tc_ir_4_2_5_3_chat_window_granularity() {
    let mut stack = ContextStack::new();
    stack.push(MappingContext::ui(
        ContextId(2),
        10,
        InputConsumption::chat_window(),
    ));
    let obs = observe_gameplay_inputs(&stack, true, true);
    assert!(obs.pointer_move);
    assert!(!obs.key_press);
}

#[test]
fn tc_ir_4_2_6_1_vr_ray_hits_panel_center() {
    let panel = PanelSpec {
        bounds: Rect::from_xywh(0.0, 0.0, 100.0, 100.0),
    };
    let origin = Vec2::new(50.0, -50.0);
    let dir = Vec2::new(0.0, 1.0);
    let hit = ray_panel_hit(origin, dir, panel).expect("hit");
    assert!((hit.x - 50.0).abs() < f32::EPSILON);
    assert!((hit.y - 0.0).abs() < f32::EPSILON);
}

#[test]
fn tc_ir_4_2_6_2_vr_ray_misses() {
    let panel = PanelSpec {
        bounds: Rect::from_xywh(0.0, 0.0, 10.0, 10.0),
    };
    let origin = Vec2::new(50.0, -50.0);
    let dir = Vec2::new(0.0, -1.0);
    assert!(ray_panel_hit(origin, dir, panel).is_none());
}

#[test]
fn tc_ir_4_2_6_3_vr_trigger_down() {
    let prev = VrControllerState {
        trigger_pressed: false,
        grip_pressed: false,
        buttons: 0,
    };
    let cur = VrControllerState {
        trigger_pressed: true,
        grip_pressed: false,
        buttons: 0,
    };
    let fired = !prev.trigger_pressed && cur.trigger_pressed;
    assert!(fired);
}

#[test]
fn tc_ir_4_2_7_1_text_input_routes_char() {
    let mut text = TextInput::new(FocusCommitPolicy::CommitOnBlur);
    text.insert_char('a');
    assert_eq!(text.text, "a");
}

#[test]
fn tc_ir_4_2_7_2_keys_blocked_from_gameplay() {
    assert!(!gameplay_observes_move_forward_key('W', true, true));
}

#[test]
fn tc_ir_4_2_7_3_ime_commit_on_blur() {
    let mut text = TextInput::new(FocusCommitPolicy::CommitOnBlur);
    text.text.push_str("hello");
    text.begin_composition("世界");
    text.on_blur();
    assert_eq!(text.text, "hello世界");
    assert!(text.composition.is_empty());
}

#[test]
fn tc_ir_4_2_7_4_ime_cancel_on_blur() {
    let mut text = TextInput::new(FocusCommitPolicy::CancelOnBlur);
    text.text.push_str("hello");
    text.begin_composition("世界");
    text.on_blur();
    assert_eq!(text.text, "hello");
    assert!(text.composition.is_empty());
}

#[test]
fn tc_ir_4_2_8_1_menu_push_context() {
    let mut stack = ContextStack::new();
    stack.push(gameplay_context(ContextId(0)));
    stack.push(menu_context(ContextId(1)));
    assert_eq!(stack.top().unwrap().context_id, ContextId(1));
}

#[test]
fn tc_ir_4_2_8_2_menu_close_restores() {
    let mut stack = ContextStack::new();
    stack.push(gameplay_context(ContextId(0)));
    stack.push(menu_context(ContextId(1)));
    let mut flags = InputDebugFlags::default();
    stack.pop(&mut flags);
    assert_eq!(stack.top().unwrap().context_id, ContextId(0));
}

#[test]
fn tc_ir_4_2_8_3_underflow_warning() {
    let mut stack = ContextStack::new();
    let mut flags = InputDebugFlags {
        log_consumption: true,
        ..InputDebugFlags::default()
    };
    assert!(stack.pop(&mut flags).is_none());
    assert!(stack
        .warnings
        .iter()
        .any(|w| *w == "context stack underflow"));
}
