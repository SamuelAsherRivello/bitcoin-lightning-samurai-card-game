use bevy::prelude::*;

use crate::runtime::resources::{
    SELECTED_CARD_MODAL_FADE_SECONDS, SELECTED_CARD_MODAL_MAX_OPACITY, SelectedCardModalModel,
};

#[test]
fn selected_modal_fades_to_max_opacity() {
    let mut model = SelectedCardModalModel::default();
    let entity = Entity::from_bits(1);
    model.select_entity(
        entity,
        Transform::default(),
        Transform::from_scale(Vec3::splat(2.0)),
    );

    model.advance_fade(SELECTED_CARD_MODAL_FADE_SECONDS * 0.5);
    assert_eq!(model.opacity(), SELECTED_CARD_MODAL_MAX_OPACITY * 0.5);

    model.advance_fade(SELECTED_CARD_MODAL_FADE_SECONDS);
    assert_eq!(model.opacity(), SELECTED_CARD_MODAL_MAX_OPACITY);
}

#[test]
fn selected_modal_eased_fade_uses_max_opacity() {
    let mut model = SelectedCardModalModel::default();
    model.select_entity(
        Entity::from_bits(12),
        Transform::default(),
        Transform::from_scale(Vec3::splat(2.0)),
    );

    model.advance_fade_with_interpolation(0.5);

    assert_eq!(SELECTED_CARD_MODAL_MAX_OPACITY, 0.9);
    assert_eq!(model.opacity(), SELECTED_CARD_MODAL_MAX_OPACITY * 0.5);
}

#[test]
fn selected_modal_backdrop_dismiss_sets_flag_without_clearing_selection() {
    let mut model = SelectedCardModalModel::default();
    let entity = Entity::from_bits(2);
    model.select_entity(
        entity,
        Transform::default(),
        Transform::from_scale(Vec3::splat(2.0)),
    );

    model.request_dismiss();

    assert_eq!(model.selected_entity, Some(entity));
    assert!(model.dismiss_pending);
    assert!(model.blocks_lower_interactions());
}

#[test]
fn selected_modal_suppresses_the_opening_click_dismiss_once() {
    let mut model = SelectedCardModalModel::default();
    model.select_entity(
        Entity::from_bits(4),
        Transform::default(),
        Transform::from_scale(Vec3::splat(2.0)),
    );

    assert!(model.take_suppressed_pointer_dismiss());
    assert!(!model.take_suppressed_pointer_dismiss());
}

#[test]
fn selected_modal_press_candidate_rejects_dragged_clicks() {
    let mut model = SelectedCardModalModel::default();
    let entity = Entity::from_bits(3);
    model.begin_press_candidate(entity, Vec2::ZERO, Transform::default());

    model.update_press_candidate(Vec2::new(9.0, 0.0), 8.0);

    assert!(model.take_click_candidate().is_none());
}
