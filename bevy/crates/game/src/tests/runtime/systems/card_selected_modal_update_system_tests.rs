use bevy::prelude::*;

use crate::runtime::resources::SelectedCardModalModel;

#[test]
fn selected_card_click_requests_modal_dismissal() {
    let mut model = SelectedCardModalModel::default();
    let entity = Entity::from_bits(10);
    model.select_entity(
        entity,
        Transform::default(),
        Transform::from_scale(Vec3::splat(2.0)),
    );

    model.request_dismiss();

    assert_eq!(model.selected_entity, Some(entity));
    assert!(model.dismiss_pending);
}

#[test]
fn backdrop_click_requests_modal_dismissal() {
    let mut model = SelectedCardModalModel::default();
    model.select_entity(
        Entity::from_bits(11),
        Transform::default(),
        Transform::from_scale(Vec3::splat(2.0)),
    );

    model.request_dismiss();

    assert!(model.dismiss_pending);
}
