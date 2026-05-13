use bevy::prelude::*;

use crate::runtime::resources::SelectedCardModalModel;

#[test]
fn selected_modal_identifies_active_point_overlay_suppression() {
    let mut model = SelectedCardModalModel::default();
    assert!(!model.blocks_lower_interactions());

    model.select_entity(
        Entity::from_bits(20),
        Transform::default(),
        Transform::from_scale(Vec3::splat(2.0)),
    );

    assert!(model.blocks_lower_interactions());
}
