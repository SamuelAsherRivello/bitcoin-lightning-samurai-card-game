use super::*;

use crate::runtime::bundles::{PointLocationView, PointModel, PointViewBundle};
use crate::runtime::components::{
    CardGestureView, CpuPlacedCardView, HandCardGestureTarget, PointViewCircle,
    PointViewOutlineTreatment, PointViewVisualModifiers, VisualModificationTarget, VisualModifier,
};
use crate::runtime::resources::{
    ActiveView, CardFace, CardSlotBoardModel, CardSlotSide, GameLocationModel, MatchPlayerSide,
};
use bevy::ecs::system::RunSystemOnce;

fn app_with_vms_resources() -> App {
    let mut app = App::new();
    app.insert_resource(ActiveView::GameScene)
        .insert_resource(CardSlotBoardModel::default())
        .insert_resource(GameLocationModel::default());
    app
}

#[test]
fn abilityoutline_activates_for_local_card_power_modified_by_ability() {
    let mut app = app_with_vms_resources();
    app.world_mut()
        .resource_mut::<CardSlotBoardModel>()
        .place_next_local(0, 0);
    let card = app
        .world_mut()
        .spawn((CardGestureView, HandCardGestureTarget::new(0)))
        .id();
    let point = app
        .world_mut()
        .spawn((
            PointViewBundle::new("PowerPointView", PointModel::card_power(3)),
            ChildOf(card),
        ))
        .with_children(|parent| {
            parent.spawn((
                PointViewOutlineTreatment::new(VisualModifier::AbilityOutline),
                Visibility::Hidden,
            ));
        })
        .id();

    app.world_mut()
        .run_system_once(visual_modifier_update_system)
        .unwrap();

    assert!(
        app.world()
            .get::<PointViewVisualModifiers>(point)
            .unwrap()
            .is_active(VisualModifier::AbilityOutline)
    );
    let child = app.world().get::<Children>(point).unwrap()[0];
    assert_eq!(
        app.world().get::<Visibility>(child),
        Some(&Visibility::Visible)
    );
}

#[test]
fn abilityoutline_stays_hidden_for_facedown_card_power_modified_by_ability() {
    let mut app = app_with_vms_resources();
    let card = app
        .world_mut()
        .spawn(CpuPlacedCardView::new(
            MatchPlayerSide::Near,
            CardSlotSide::LocalPlayer,
            0,
            0,
            "test-card",
            CardFace::Back,
        ))
        .id();
    let point = app
        .world_mut()
        .spawn((
            PointViewBundle::new("PowerPointView", PointModel::card_power(3)),
            ChildOf(card),
        ))
        .with_children(|parent| {
            parent.spawn((
                PointViewOutlineTreatment::new(VisualModifier::AbilityOutline),
                Visibility::Hidden,
            ));
        })
        .id();

    app.world_mut()
        .run_system_once(visual_modifier_update_system)
        .unwrap();

    assert!(
        !app.world()
            .get::<PointViewVisualModifiers>(point)
            .unwrap()
            .is_active(VisualModifier::AbilityOutline)
    );
    let child = app.world().get::<Children>(point).unwrap()[0];
    assert_eq!(
        app.world().get::<Visibility>(child),
        Some(&Visibility::Hidden)
    );
}

#[test]
fn abilityoutline_clears_when_card_power_is_not_modified_by_ability() {
    let mut app = app_with_vms_resources();
    let card = app
        .world_mut()
        .spawn((CardGestureView, HandCardGestureTarget::new(0)))
        .id();
    let point = app
        .world_mut()
        .spawn((
            PointViewBundle::new("PowerPointView", PointModel::card_power(3)),
            ChildOf(card),
        ))
        .id();

    app.world_mut()
        .run_system_once(visual_modifier_update_system)
        .unwrap();

    assert!(
        !app.world()
            .get::<PointViewVisualModifiers>(point)
            .unwrap()
            .is_active(VisualModifier::AbilityOutline)
    );
}

#[test]
fn leadingscoreoutline_activates_only_for_higher_location_total() {
    let mut app = app_with_vms_resources();
    let local = app
        .world_mut()
        .spawn((
            PointViewBundle::new("PowerPointView", PointModel::location_power(5)),
            PointLocationView::new(0, CardSlotSide::LocalPlayer),
        ))
        .with_children(|parent| {
            parent.spawn((
                PointViewCircle::new(VisualModificationTarget::LocationTotalPointCircle),
                Node::default(),
                BorderColor::all(Color::NONE),
            ));
        })
        .id();
    let opponent = app
        .world_mut()
        .spawn((
            PointViewBundle::new("PowerPointView", PointModel::location_power(3)),
            PointLocationView::new(0, CardSlotSide::Opponent),
        ))
        .id();

    app.world_mut()
        .run_system_once(visual_modifier_update_system)
        .unwrap();

    assert!(
        app.world()
            .get::<PointViewVisualModifiers>(local)
            .unwrap()
            .is_active(VisualModifier::LeadingScoreOutline)
    );
    let child = app.world().get::<Children>(local).unwrap()[0];
    assert_eq!(
        *app.world().get::<BorderColor>(child).unwrap(),
        BorderColor::all(Color::WHITE)
    );
    assert!(
        !app.world()
            .get::<PointViewVisualModifiers>(opponent)
            .unwrap()
            .is_active(VisualModifier::LeadingScoreOutline)
    );
}

#[test]
fn leadingscoreoutline_clears_for_tied_location_total() {
    let mut app = app_with_vms_resources();
    let local = app
        .world_mut()
        .spawn((
            PointViewBundle::new("PowerPointView", PointModel::location_power(4)),
            PointLocationView::new(0, CardSlotSide::LocalPlayer),
        ))
        .id();
    app.world_mut().spawn((
        PointViewBundle::new("PowerPointView", PointModel::location_power(4)),
        PointLocationView::new(0, CardSlotSide::Opponent),
    ));

    app.world_mut()
        .run_system_once(visual_modifier_update_system)
        .unwrap();

    assert!(
        !app.world()
            .get::<PointViewVisualModifiers>(local)
            .unwrap()
            .is_active(VisualModifier::LeadingScoreOutline)
    );
}
