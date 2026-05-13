use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

use crate::runtime::bundles::{PointLocationView, PointType, PointView};
use crate::runtime::components::{
    CpuPlacedCardView, HandCardGestureTarget, PointViewCardInstanceLink, PointViewCircle,
    PointViewOutlineTreatment, PointViewVisualModifiers, VisualModificationCondition,
    VisualModificationTarget, VisualModificationTreatment, VisualModifier,
};
use crate::runtime::resources::{
    ActiveView, CardInstanceStateCollectionModel, CardSlotBoardModel, CardSlotSide, CardZoneModel,
    GameLocationModel,
};

const LOCATION_SIDE_COUNT: usize = 2;

/// HUMAN: Queries needed to resolve VMS rules from point roots and point targets.
/// AI: Keep split query params here so root mutation and target sync stay borrow-safe.
#[derive(SystemParam)]
pub struct VisualModifierUpdateQueries<'w, 's> {
    point_roots: Query<
        'w,
        's,
        (
            Entity,
            &'static PointView,
            Option<&'static PointLocationView>,
            Option<&'static PointViewCardInstanceLink>,
            Option<&'static ChildOf>,
            Option<&'static Children>,
            &'static mut PointViewVisualModifiers,
        ),
    >,
    point_roots_read: Query<
        'w,
        's,
        (
            Entity,
            &'static PointView,
            Option<&'static PointLocationView>,
        ),
    >,
    card_roots: Query<
        'w,
        's,
        (
            Option<&'static HandCardGestureTarget>,
            Option<&'static CpuPlacedCardView>,
        ),
    >,
    circle_targets: Query<
        'w,
        's,
        (
            &'static PointViewCircle,
            Option<&'static mut Node>,
            Option<&'static mut BorderColor>,
        ),
    >,
    outline_targets: Query<'w, 's, (&'static PointViewOutlineTreatment, &'static mut Visibility)>,
}

/// HUMAN: Applies visual modification rules to point-view circle targets.
/// AI: Evaluate Conditions first, resolve Targets second, apply Treatments last.
pub fn visual_modifier_update_system(
    active_view: Option<Res<ActiveView>>,
    card_instances: Option<Res<CardInstanceStateCollectionModel>>,
    slot_board: Res<CardSlotBoardModel>,
    game_location_model: Option<Res<GameLocationModel>>,
    mut queries: VisualModifierUpdateQueries,
) {
    if !super::is_game_view_active(active_view.as_deref()) {
        return;
    }

    let location_totals = collect_location_totals(&queries.point_roots_read);

    for (_, point_view, point_location, instance_link, child_of, children, mut modifiers) in
        &mut queries.point_roots
    {
        modifiers.clear();

        let ability_outline_active = card_power_modified_by_ability(
            point_view,
            instance_link,
            card_instances.as_deref(),
            child_of,
            &queries.card_roots,
            &slot_board,
            game_location_model.as_deref(),
        );
        modifiers.set_active(VisualModifier::AbilityOutline, ability_outline_active);

        let leading_score_active = point_location.is_some_and(|location_view| {
            location_total_is_leading(point_view, location_view, &location_totals)
        });
        modifiers.set_active(VisualModifier::LeadingScoreOutline, leading_score_active);

        sync_point_view_treatments(
            children,
            &modifiers,
            &mut queries.circle_targets,
            &mut queries.outline_targets,
        );
    }
}

fn card_power_modified_by_ability(
    point_view: &PointView,
    instance_link: Option<&PointViewCardInstanceLink>,
    card_instances: Option<&CardInstanceStateCollectionModel>,
    child_of: Option<&ChildOf>,
    card_roots: &Query<(Option<&HandCardGestureTarget>, Option<&CpuPlacedCardView>)>,
    slot_board: &CardSlotBoardModel,
    game_location_model: Option<&GameLocationModel>,
) -> bool {
    if point_view.model.point_type != PointType::CardPower {
        return false;
    }

    if let Some(location_index) = instance_link.and_then(|link| {
        card_instances
            .and_then(|instances| instances.by_instance_id(link.instance_id))
            .and_then(|card| match card.zone {
                CardZoneModel::Location { location_index, .. } => Some(location_index),
                _ => None,
            })
    }) {
        let active_delta = game_location_model
            .map(|locations| locations.ability_delta_for_location(location_index))
            .unwrap_or(0);
        return VisualModificationCondition::card_power_modified_by_ability(active_delta);
    }

    let Some(parent) = child_of.map(ChildOf::parent) else {
        return false;
    };
    let Ok((hand_target, cpu_placed_view)) = card_roots.get(parent) else {
        return false;
    };
    let location_index = hand_target
        .and_then(|target| {
            slot_board
                .local_slot_for_card(target.hand_index)
                .map(|(location_index, _)| location_index)
        })
        .or_else(|| cpu_placed_view.map(|view| view.location_index));
    let Some(location_index) = location_index else {
        return false;
    };
    let active_delta = game_location_model
        .map(|locations| locations.ability_delta_for_location(location_index))
        .unwrap_or(0);

    VisualModificationCondition::card_power_modified_by_ability(active_delta)
}

fn location_total_is_leading(
    point_view: &PointView,
    location_view: &PointLocationView,
    location_totals: &[[Option<i32>; LOCATION_SIDE_COUNT]],
) -> bool {
    if point_view.model.point_type != PointType::LocationPower {
        return false;
    }
    let Some(totals) = location_totals.get(location_view.location_index) else {
        return false;
    };
    let side_index = location_side_index(location_view.side);
    let paired_index = 1 - side_index;
    let Some(paired_value) = totals[paired_index] else {
        return false;
    };

    VisualModificationCondition::location_total_is_leading(point_view.model.value, paired_value)
}

fn collect_location_totals(
    point_roots: &Query<(Entity, &PointView, Option<&PointLocationView>)>,
) -> Vec<[Option<i32>; LOCATION_SIDE_COUNT]> {
    let mut totals = Vec::new();
    for (_, point_view, point_location) in point_roots {
        let Some(location_view) = point_location else {
            continue;
        };
        if point_view.model.point_type != PointType::LocationPower {
            continue;
        }
        if totals.len() <= location_view.location_index {
            totals.resize(
                location_view.location_index + 1,
                [None; LOCATION_SIDE_COUNT],
            );
        }
        totals[location_view.location_index][location_side_index(location_view.side)] =
            Some(point_view.model.value);
    }
    totals
}

fn location_side_index(side: CardSlotSide) -> usize {
    match side {
        CardSlotSide::Opponent => 0,
        CardSlotSide::LocalPlayer => 1,
    }
}

fn sync_point_view_treatments(
    children: Option<&Children>,
    modifiers: &PointViewVisualModifiers,
    circle_targets: &mut Query<(
        &PointViewCircle,
        Option<&mut Node>,
        Option<&mut BorderColor>,
    )>,
    outline_targets: &mut Query<(&PointViewOutlineTreatment, &mut Visibility)>,
) {
    let Some(children) = children else {
        return;
    };

    for child in children.iter() {
        if let Ok((circle, node, border)) = circle_targets.get_mut(child) {
            sync_circle_treatment(circle, node, border, modifiers);
        }
        if let Ok((outline, mut visibility)) = outline_targets.get_mut(child) {
            *visibility = if modifiers.is_active(outline.modifier) {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
    }
}

fn sync_circle_treatment(
    circle: &PointViewCircle,
    node: Option<Mut<Node>>,
    border: Option<Mut<BorderColor>>,
    modifiers: &PointViewVisualModifiers,
) {
    let active_treatment = match circle.target {
        VisualModificationTarget::CardPowerPointCircle
            if modifiers.is_active(VisualModifier::AbilityOutline) =>
        {
            Some(VisualModificationTreatment::gold_outline())
        }
        VisualModificationTarget::LocationTotalPointCircle
            if modifiers.is_active(VisualModifier::LeadingScoreOutline) =>
        {
            Some(VisualModificationTreatment::white_outline())
        }
        _ => None,
    };

    if let (Some(mut node), Some(mut border)) = (node, border) {
        match active_treatment {
            Some(VisualModificationTreatment::Outline { color, width }) => {
                node.border = UiRect::all(Val::Px(width));
                *border = BorderColor::all(color.color());
            }
            None => {
                node.border = UiRect::ZERO;
                *border = BorderColor::all(Color::NONE);
            }
        }
    }
}

#[cfg(test)]
#[path = "../../tests/runtime/systems/visual_modifier_update_system_tests.rs"]
mod visual_modifier_update_system_tests;
