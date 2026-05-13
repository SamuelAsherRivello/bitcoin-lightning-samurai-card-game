use bevy::prelude::*;

use crate::runtime::bundles::PointView;
use crate::runtime::resources::SelectedCardModalModel;

use super::CardPointTextView;

/// HUMAN: Hides lower card point text while one selected card is inspected.
/// AI: Point Text2d uses a separate camera, so visibility must be selected-card aware.
pub fn card_point_overlay_selection_update_system(
    selected_modal: Res<SelectedCardModalModel>,
    point_parent_query: Query<
        (&ChildOf, &Visibility),
        (With<PointView>, Without<CardPointTextView>),
    >,
    mut text_query: Query<
        (&ChildOf, &mut Visibility),
        (With<CardPointTextView>, Without<PointView>),
    >,
) {
    let selected_entity = selected_modal.selected_entity;
    for (child_of, mut visibility) in &mut text_query {
        let visible = point_parent_query.get(child_of.parent()).is_ok_and(
            |(point_parent, point_visibility)| {
                *point_visibility != Visibility::Hidden
                    && selected_entity
                        .is_none_or(|selected_entity| point_parent.parent() == selected_entity)
            },
        );
        *visibility = if visible {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

#[cfg(test)]
#[path = "../../tests/runtime/systems/card_point_overlay_selection_update_system_tests.rs"]
mod card_point_overlay_selection_update_system_tests;
