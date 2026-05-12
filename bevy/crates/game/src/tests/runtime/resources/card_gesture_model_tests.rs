use super::*;

fn source_transform() -> Transform {
    Transform::from_translation(Vec3::new(1.0, 2.0, 0.3)).with_scale(Vec3::splat(0.5))
}

#[test]
fn press_records_one_active_focus_and_source_transform() {
    let mut model = CardGestureModel::default();
    let source = source_transform();

    assert!(model.press(2, Vec2::new(10.0, 20.0), Vec2::new(12.0, 24.0), source));

    assert_eq!(model.state, CardGestureState::Pressed);
    assert_eq!(model.active_hand_index, Some(2));
    assert_eq!(model.source_transform, Some(source));
    assert!(model.is_active_for(2));
}

#[test]
fn selected_card_can_return_to_source() {
    let mut model = CardGestureModel::default();
    let source = source_transform();
    let target = Transform::from_translation(Vec3::ZERO).with_scale(Vec3::splat(1.2));

    assert!(model.press(1, Vec2::ZERO, Vec2::ZERO, source));
    model.select(target);
    assert_eq!(model.state, CardGestureState::SelectedInspecting);
    assert_eq!(model.target_transform, Some(target));

    model.return_to_source();
    assert_eq!(model.state, CardGestureState::Returning);
    assert_eq!(model.target_transform, Some(source));
    assert_eq!(
        model.resolved_destination,
        Some(CardGestureDestination::HandCardSlot { hand_index: 1 })
    );
}

#[test]
fn threshold_boundary_converts_to_drag_once() {
    let mut model = CardGestureModel::default();
    assert!(model.press(0, Vec2::ZERO, Vec2::ZERO, source_transform()));

    model.update_pointer(
        Vec2::new(CARD_GESTURE_DRAG_THRESHOLD - 0.1, 0.0),
        CARD_GESTURE_DRAG_THRESHOLD,
    );
    assert_eq!(model.state, CardGestureState::Pressed);

    model.update_pointer(
        Vec2::new(CARD_GESTURE_DRAG_THRESHOLD, 0.0),
        CARD_GESTURE_DRAG_THRESHOLD,
    );
    assert_eq!(model.state, CardGestureState::Dragging);
    assert!(
        model
            .pointer
            .expect("pointer should stay active while dragging")
            .has_crossed_drag_threshold
    );
}

#[test]
fn pointer_keeps_card_center_offset_for_dragging() {
    let pointer = PointerGestureModel::new(Vec2::new(10.0, 20.0), Vec2::new(18.0, 32.0));

    assert_eq!(pointer.card_center_offset, Vec2::new(8.0, 12.0));
    assert_eq!(pointer.current_card_center(), Vec2::new(18.0, 32.0));
}

#[test]
fn active_pointer_gesture_cannot_be_replaced() {
    let mut model = CardGestureModel::default();
    let first_source = source_transform();
    let second_source = Transform::from_translation(Vec3::new(8.0, 9.0, 0.3));

    assert!(model.press(0, Vec2::ZERO, Vec2::ZERO, first_source));
    model.update_pointer(
        Vec2::new(CARD_GESTURE_DRAG_THRESHOLD, 0.0),
        CARD_GESTURE_DRAG_THRESHOLD,
    );

    assert!(!model.press(1, Vec2::splat(5.0), Vec2::splat(5.0), second_source));
    assert_eq!(model.state, CardGestureState::Dragging);
    assert_eq!(model.active_hand_index, Some(0));
    assert_eq!(model.source_transform, Some(first_source));
}

#[test]
fn successful_drag_resolves_to_location_card_slot() {
    let mut model = CardGestureModel::default();
    let target_slot = CardGestureSlotTarget::new(2, CardSlotSide::LocalPlayer, 3);

    assert!(model.press(0, Vec2::ZERO, Vec2::ZERO, source_transform()));
    model.place(target_slot, Transform::from_translation(Vec3::X));

    assert_eq!(
        model.resolved_destination,
        Some(CardGestureDestination::LocationCardSlot {
            location_index: 2,
            slot_index: 3,
        })
    );
}
