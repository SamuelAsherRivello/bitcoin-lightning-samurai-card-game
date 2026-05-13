use bevy::prelude::*;

use crate::runtime::components::{
    CardSelectionSource, CpuPlacedCardAnimation, CpuPlacedCardAnimationPhase, CpuPlacedCardView,
};
use crate::runtime::resources::{
    ActiveView, CardFace, CardFlipState, CardSlotSide, MatchPlayerSide,
};

use super::{
    selectable_card_front_is_visible, selectable_card_motion_allows_selection,
    selection_source_matches_view, transform_from_global_relative_to_parent,
};

#[test]
fn screen_card_source_matches_its_active_view() {
    assert!(selection_source_matches_view(
        CardSelectionSource::ScreenCard {
            view: ActiveView::DebugScene,
        },
        ActiveView::DebugScene,
        None,
        None,
        None,
    ));
    assert!(!selection_source_matches_view(
        CardSelectionSource::ScreenCard {
            view: ActiveView::DebugScene,
        },
        ActiveView::GameScene,
        None,
        None,
        None,
    ));
}

#[test]
fn generic_card_view_bundle_source_matches_scene_markers() {
    let game_scene = crate::runtime::components::GameSceneEntity;
    let debug = crate::runtime::components::DebugSceneEntity;

    assert!(selection_source_matches_view(
        CardSelectionSource::CardViewBundle,
        ActiveView::GameScene,
        Some(&game_scene),
        None,
        None,
    ));
    assert!(selection_source_matches_view(
        CardSelectionSource::CardViewBundle,
        ActiveView::DebugScene,
        None,
        None,
        Some(&debug),
    ));
    assert!(!selection_source_matches_view(
        CardSelectionSource::CardViewBundle,
        ActiveView::DeckScene,
        Some(&game_scene),
        None,
        Some(&debug),
    ));
}

#[test]
fn screen_card_requires_front_face_and_no_flip_animation() {
    let mut flip_state = CardFlipState::default();
    flip_state.visible_face = CardFace::Front;
    assert!(selectable_card_front_is_visible(
        CardSelectionSource::ScreenCard {
            view: ActiveView::DebugScene,
        },
        None,
        None,
        None,
        &flip_state,
    ));

    flip_state.target_y_rotation = std::f32::consts::PI;
    assert!(!selectable_card_front_is_visible(
        CardSelectionSource::ScreenCard {
            view: ActiveView::DebugScene,
        },
        None,
        None,
        None,
        &flip_state,
    ));
}

#[test]
fn far_location_card_can_select_when_reveal_animation_is_showing_front() {
    let source = CardSelectionSource::OpponentLocation {
        owner: MatchPlayerSide::Far,
        side: CardSlotSide::Opponent,
        location_index: 0,
        slot_index: 1,
    };
    let view = CpuPlacedCardView::new(
        MatchPlayerSide::Far,
        CardSlotSide::Opponent,
        0,
        1,
        "test-card",
        CardFace::Back,
    );
    let mut animation = CpuPlacedCardAnimation::flip_to_front(Transform::default(), 0.0);
    animation.phase = CpuPlacedCardAnimationPhase::Revealing;
    animation.current_y_rotation = std::f32::consts::FRAC_PI_3;

    assert!(selectable_card_front_is_visible(
        source,
        None,
        Some(&view),
        Some(&animation),
        &CardFlipState::default(),
    ));
    assert!(selectable_card_motion_allows_selection(
        source,
        Some(&animation),
    ));
}

#[test]
fn far_location_card_can_select_when_revealed_front_has_no_animation() {
    let source = CardSelectionSource::OpponentLocation {
        owner: MatchPlayerSide::Far,
        side: CardSlotSide::Opponent,
        location_index: 0,
        slot_index: 1,
    };
    let view = CpuPlacedCardView::new(
        MatchPlayerSide::Far,
        CardSlotSide::Opponent,
        0,
        1,
        "test-card",
        CardFace::Front,
    );

    assert!(selectable_card_front_is_visible(
        source,
        None,
        Some(&view),
        None,
        &CardFlipState::default(),
    ));
    assert!(selectable_card_motion_allows_selection(source, None));
}

#[test]
fn far_location_card_cannot_select_while_still_showing_back() {
    let source = CardSelectionSource::OpponentLocation {
        owner: MatchPlayerSide::Far,
        side: CardSlotSide::Opponent,
        location_index: 0,
        slot_index: 1,
    };
    let view = CpuPlacedCardView::new(
        MatchPlayerSide::Far,
        CardSlotSide::Opponent,
        0,
        1,
        "test-card",
        CardFace::Back,
    );
    let animation = CpuPlacedCardAnimation::flip_to_front(Transform::default(), 0.0);

    assert!(!selectable_card_front_is_visible(
        source,
        None,
        Some(&view),
        Some(&animation),
        &CardFlipState::default(),
    ));
    assert!(!selectable_card_motion_allows_selection(
        source,
        Some(&animation),
    ));
}

#[test]
fn selected_card_target_is_converted_to_parent_local_space() {
    let parent_transform = Transform::from_translation(Vec3::new(10.0, 0.0, 0.0));
    let target_world_transform = Transform::from_translation(Vec3::new(0.0, 0.0, 0.88));
    let target = transform_from_global_relative_to_parent(
        target_world_transform,
        &GlobalTransform::from(parent_transform),
    );

    assert_eq!(target.translation, Vec3::new(-10.0, 0.0, 0.88));
}
