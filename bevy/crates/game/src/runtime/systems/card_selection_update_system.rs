use bevy::{
    prelude::*,
    window::{PrimaryWindow, Window},
};

#[cfg(all(feature = "ai-runtime", not(target_arch = "wasm32")))]
use crate::runtime::components::CardGestureView;
use crate::runtime::components::{
    CardSelectionSource, CardView, CpuHandCardView, CpuPlacedCardAnimation, CpuPlacedCardView,
    DebugSceneEntity, DeckSceneEntity, GameSceneEntity, HandCardGestureTarget, SelectableCard,
};
use crate::runtime::resources::{
    ActiveView, CARD_GESTURE_DRAG_THRESHOLD, CardFace, CardFlipState, CardGestureModel,
    CardGestureState, CardInspectionDefaults, SelectedCardModalModel,
};
#[cfg(all(feature = "ai-runtime", not(target_arch = "wasm32")))]
use crate::runtime::resources::{CardState, CardStateModel};

use super::{
    active_pointer_position, is_deck_card_hit, just_pressed_pointer_position,
    selected_inspection_transform,
};
#[cfg(all(feature = "ai-runtime", not(target_arch = "wasm32")))]
use super::{
    game_scene_card_hitboxes_for_count, game_scene_card_index_at_for_count,
    window_pointer_to_game_scene,
};

type GameCardCameraFilter = (
    With<crate::runtime::components::PrimaryViewCamera>,
    With<crate::runtime::components::GameSceneEntity>,
    With<Camera3d>,
);
type DeckCardCameraFilter = (
    With<crate::runtime::components::PrimaryViewCamera>,
    With<DeckSceneEntity>,
    With<Camera3d>,
);
type DebugCardCameraFilter = (
    With<crate::runtime::components::PrimaryViewCamera>,
    With<DebugSceneEntity>,
    With<Camera3d>,
);

/// HUMAN: Promotes click candidates on selectable passive cards into selected inspection.
/// AI: Local draggable cards remain owned by CardGestureModel; this covers CPU and screen cards.
pub fn card_selection_update_system(
    mut commands: Commands,
    primary_window_query: Query<&Window, With<PrimaryWindow>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    active_view: Res<ActiveView>,
    card_defaults: Res<CardInspectionDefaults>,
    flip_state: Res<CardFlipState>,
    mut selected_modal: ResMut<SelectedCardModalModel>,
    gesture_model: Res<CardGestureModel>,
    mut camera_queries: ParamSet<(
        Query<(&Camera, &GlobalTransform), GameCardCameraFilter>,
        Query<(&Camera, &GlobalTransform), DeckCardCameraFilter>,
        Query<(&Camera, &GlobalTransform), DebugCardCameraFilter>,
    )>,
    selectable_query: Query<
        (
            Entity,
            &SelectableCard,
            &GlobalTransform,
            &Transform,
            Option<&CpuHandCardView>,
            Option<&CpuPlacedCardView>,
            Option<&CpuPlacedCardAnimation>,
            Option<&HandCardGestureTarget>,
            Option<&GameSceneEntity>,
            Option<&DeckSceneEntity>,
            Option<&DebugSceneEntity>,
            Option<&ChildOf>,
        ),
        With<CardView>,
    >,
    parent_transform_query: Query<&GlobalTransform>,
) {
    if selected_modal.is_active() {
        selected_modal.cancel_press_candidate();
        return;
    }

    let Ok(primary_window) = primary_window_query.single() else {
        return;
    };

    if let Some(pointer_position) =
        just_pressed_pointer_position(primary_window, &mouse_buttons, &touches)
    {
        if let Some((entity, source_transform)) = top_selectable_card_at_pointer(
            pointer_position,
            *active_view,
            &card_defaults,
            &flip_state,
            &gesture_model,
            &mut camera_queries,
            &selectable_query,
        ) {
            if matches!(*active_view, ActiveView::DeckScene | ActiveView::DebugScene) {
                let Some((source_transform, target_transform)) =
                    selectable_card_selection_transforms(
                        entity,
                        selected_inspection_transform(&card_defaults),
                        &selectable_query,
                        &parent_transform_query,
                    )
                else {
                    return;
                };
                commands.entity(entity).remove::<CpuPlacedCardAnimation>();
                selected_modal.select_entity(entity, source_transform, target_transform);
                return;
            }
            selected_modal.begin_press_candidate(entity, pointer_position, source_transform);
        }
    }

    if let Some(pointer_position) = active_pointer_position(primary_window, &touches) {
        selected_modal.update_press_candidate(pointer_position, CARD_GESTURE_DRAG_THRESHOLD);
    }

    if pointer_just_released(&mouse_buttons, &touches)
        && let Some(candidate) = selected_modal.take_click_candidate()
    {
        let Some((source_transform, target_transform)) = selectable_card_selection_transforms(
            candidate.entity,
            selected_inspection_transform(&card_defaults),
            &selectable_query,
            &parent_transform_query,
        ) else {
            return;
        };
        commands
            .entity(candidate.entity)
            .remove::<CpuPlacedCardAnimation>();
        selected_modal.select_entity(candidate.entity, source_transform, target_transform);
    }
}

/// HUMAN: Public AI-runtime hook for the consequence of a card click at a window coordinate.
/// AI: This bypasses OS input synthesis but reuses the same selectable-card resolution and modal state.
#[cfg(all(feature = "ai-runtime", not(target_arch = "wasm32")))]
pub fn ai_runtime_on_card_clicked_system(
    In(params): In<Option<serde_json::Value>>,
    mut commands: Commands,
    mut primary_window_query: Query<&mut Window, With<PrimaryWindow>>,
    active_view: Res<ActiveView>,
    card_defaults: Res<CardInspectionDefaults>,
    flip_state: Res<CardFlipState>,
    mut selected_modal: ResMut<SelectedCardModalModel>,
    mut gesture_model: ResMut<CardGestureModel>,
    card_states: Res<CardStateModel>,
    mut camera_queries: ParamSet<(
        Query<(&Camera, &GlobalTransform), GameCardCameraFilter>,
        Query<(&Camera, &GlobalTransform), DeckCardCameraFilter>,
        Query<(&Camera, &GlobalTransform), DebugCardCameraFilter>,
    )>,
    selectable_query: Query<
        (
            Entity,
            &SelectableCard,
            &GlobalTransform,
            &Transform,
            Option<&CpuHandCardView>,
            Option<&CpuPlacedCardView>,
            Option<&CpuPlacedCardAnimation>,
            Option<&HandCardGestureTarget>,
            Option<&GameSceneEntity>,
            Option<&DeckSceneEntity>,
            Option<&DebugSceneEntity>,
            Option<&ChildOf>,
        ),
        With<CardView>,
    >,
    hand_card_query: Query<(Entity, &HandCardGestureTarget, &Transform), With<CardGestureView>>,
    parent_transform_query: Query<&GlobalTransform>,
) -> bevy::remote::BrpResult {
    let pointer_position = match ai_runtime_card_click_pointer_position(params) {
        Ok(position) => position,
        Err(message) => {
            return Ok(serde_json::json!({
                "success": false,
                "error": message
            }));
        }
    };
    let Ok(mut primary_window) = primary_window_query.single_mut() else {
        return Ok(serde_json::json!({
            "success": false,
            "error": "Primary window unavailable"
        }));
    };

    primary_window.set_cursor_position(Some(pointer_position));

    if selected_modal.is_active() {
        selected_modal.request_dismiss();
    }

    if *active_view == ActiveView::GameScene
        && let Some(entity) = ai_runtime_select_game_hand_card_at_pointer(
            pointer_position,
            primary_window.size(),
            &card_defaults,
            &card_states,
            &mut gesture_model,
            &mut selected_modal,
            &hand_card_query,
        )
    {
        return Ok(serde_json::json!({
            "active_view": format!("{:?}", *active_view),
            "entity": format!("{entity:?}"),
            "success": true,
            "x": pointer_position.x,
            "y": pointer_position.y
        }));
    }

    let Some((entity, _)) = top_selectable_card_at_pointer(
        pointer_position,
        *active_view,
        &card_defaults,
        &flip_state,
        &gesture_model,
        &mut camera_queries,
        &selectable_query,
    ) else {
        return Ok(serde_json::json!({
            "active_view": format!("{:?}", *active_view),
            "success": false,
            "error": "No selectable card at pointer",
            "x": pointer_position.x,
            "y": pointer_position.y
        }));
    };

    let Some((source_transform, target_transform)) = selectable_card_selection_transforms(
        entity,
        selected_inspection_transform(&card_defaults),
        &selectable_query,
        &parent_transform_query,
    ) else {
        return Ok(serde_json::json!({
            "active_view": format!("{:?}", *active_view),
            "entity": format!("{entity:?}"),
            "success": false,
            "error": "Selectable card transform unavailable",
            "x": pointer_position.x,
            "y": pointer_position.y
        }));
    };

    commands.entity(entity).remove::<CpuPlacedCardAnimation>();
    selected_modal.select_entity(entity, source_transform, target_transform);

    Ok(serde_json::json!({
        "active_view": format!("{:?}", *active_view),
        "entity": format!("{entity:?}"),
        "success": true,
        "x": pointer_position.x,
        "y": pointer_position.y
    }))
}

#[cfg(all(feature = "ai-runtime", not(target_arch = "wasm32")))]
fn ai_runtime_select_game_hand_card_at_pointer(
    pointer_position: Vec2,
    window_size: Vec2,
    card_defaults: &CardInspectionDefaults,
    card_states: &CardStateModel,
    gesture_model: &mut CardGestureModel,
    selected_modal: &mut SelectedCardModalModel,
    hand_card_query: &Query<(Entity, &HandCardGestureTarget, &Transform), With<CardGestureView>>,
) -> Option<Entity> {
    let game_scene_position = window_pointer_to_game_scene(pointer_position, window_size)?;
    let hand_indices = card_states.indices_with_state(CardState::Hand);
    let order_index =
        game_scene_card_index_at_for_count(pointer_position, window_size, hand_indices.len())?;
    let hand_index = card_states.hand_index_at_order(order_index)?;
    if !card_states.is_selectable(hand_index) {
        return None;
    }
    let (card_min, card_max) = game_scene_card_hitboxes_for_count(hand_indices.len())
        .get(order_index)
        .copied()?;
    let card_center = (card_min + card_max) * 0.5;
    let (entity, source_transform) =
        hand_card_query
            .iter()
            .find_map(|(entity, target, transform)| {
                (target.hand_index == hand_index).then_some((entity, *transform))
            })?;
    let target_transform = selected_inspection_transform(card_defaults);

    gesture_model.press(
        hand_index,
        game_scene_position,
        card_center,
        source_transform,
    );
    gesture_model.select(target_transform);
    selected_modal.select_entity(entity, source_transform, target_transform);

    Some(entity)
}

#[cfg(all(feature = "ai-runtime", not(target_arch = "wasm32")))]
fn ai_runtime_card_click_pointer_position(
    params: Option<serde_json::Value>,
) -> Result<Vec2, String> {
    let Some(params) = params else {
        return Err("Invalid pointer: expected x and y".to_string());
    };
    let x = params
        .get("x")
        .and_then(serde_json::Value::as_f64)
        .ok_or_else(|| "Invalid pointer: x must be a number".to_string())?;
    let y = params
        .get("y")
        .and_then(serde_json::Value::as_f64)
        .ok_or_else(|| "Invalid pointer: y must be a number".to_string())?;
    if !x.is_finite() || !y.is_finite() || x < 0.0 || y < 0.0 {
        return Err("Invalid pointer: x and y must be finite non-negative numbers".to_string());
    }

    Ok(Vec2::new(x as f32, y as f32))
}

fn top_selectable_card_at_pointer(
    pointer_position: Vec2,
    active_view: ActiveView,
    card_defaults: &CardInspectionDefaults,
    flip_state: &CardFlipState,
    gesture_model: &CardGestureModel,
    camera_queries: &mut ParamSet<(
        Query<(&Camera, &GlobalTransform), GameCardCameraFilter>,
        Query<(&Camera, &GlobalTransform), DeckCardCameraFilter>,
        Query<(&Camera, &GlobalTransform), DebugCardCameraFilter>,
    )>,
    selectable_query: &Query<
        (
            Entity,
            &SelectableCard,
            &GlobalTransform,
            &Transform,
            Option<&CpuHandCardView>,
            Option<&CpuPlacedCardView>,
            Option<&CpuPlacedCardAnimation>,
            Option<&HandCardGestureTarget>,
            Option<&GameSceneEntity>,
            Option<&DeckSceneEntity>,
            Option<&DebugSceneEntity>,
            Option<&ChildOf>,
        ),
        With<CardView>,
    >,
) -> Option<(Entity, Transform)> {
    selectable_query
        .iter()
        .filter(
            |(
                _,
                selectable,
                _,
                _,
                cpu_hand,
                cpu_placed,
                animation,
                _local_target,
                game_scene,
                deck,
                debug,
                _child_of,
            )| {
                selection_source_matches_view(
                    selectable.source,
                    active_view,
                    *game_scene,
                    *deck,
                    *debug,
                ) && selectable.is_stationary()
                    && selectable_card_motion_allows_selection(selectable.source, *animation)
                    && selectable_card_front_is_visible(
                        selectable.source,
                        *cpu_hand,
                        *cpu_placed,
                        *animation,
                        flip_state,
                    )
                    && !matches!(
                        gesture_model.state,
                        CardGestureState::Dragging | CardGestureState::Returning
                    )
            },
        )
        .filter(|(_, _, global_transform, _, _, _, _, _, _, _, _, _)| {
            selectable_card_contains_pointer(
                pointer_position,
                active_view,
                camera_queries,
                global_transform,
                card_defaults,
            )
        })
        .max_by(|left, right| {
            left.2
                .translation()
                .z
                .partial_cmp(&right.2.translation().z)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|(entity, _, _, transform, _, _, _, _, _, _, _, _)| (entity, *transform))
}

fn selectable_card_selection_transforms(
    entity: Entity,
    target_world_transform: Transform,
    selectable_query: &Query<
        (
            Entity,
            &SelectableCard,
            &GlobalTransform,
            &Transform,
            Option<&CpuHandCardView>,
            Option<&CpuPlacedCardView>,
            Option<&CpuPlacedCardAnimation>,
            Option<&HandCardGestureTarget>,
            Option<&GameSceneEntity>,
            Option<&DeckSceneEntity>,
            Option<&DebugSceneEntity>,
            Option<&ChildOf>,
        ),
        With<CardView>,
    >,
    parent_transform_query: &Query<&GlobalTransform>,
) -> Option<(Transform, Transform)> {
    let (_, _, _, source_transform, _, _, _, _, _, _, _, child_of) =
        selectable_query.get(entity).ok()?;
    let target_transform = child_of
        .and_then(|child_of| parent_transform_query.get(child_of.parent()).ok())
        .map(|parent_global_transform| {
            transform_from_global_relative_to_parent(
                target_world_transform,
                parent_global_transform,
            )
        })
        .unwrap_or(target_world_transform);

    Some((*source_transform, target_transform))
}

fn transform_from_global_relative_to_parent(
    global_transform: Transform,
    parent_global_transform: &GlobalTransform,
) -> Transform {
    let local_matrix = parent_global_transform.affine().inverse()
        * GlobalTransform::from(global_transform).affine();
    Transform::from_matrix(Mat4::from(local_matrix))
}

fn selectable_card_contains_pointer(
    pointer_position: Vec2,
    active_view: ActiveView,
    camera_queries: &mut ParamSet<(
        Query<(&Camera, &GlobalTransform), GameCardCameraFilter>,
        Query<(&Camera, &GlobalTransform), DeckCardCameraFilter>,
        Query<(&Camera, &GlobalTransform), DebugCardCameraFilter>,
    )>,
    global_transform: &GlobalTransform,
    card_defaults: &CardInspectionDefaults,
) -> bool {
    match active_view {
        ActiveView::GameScene => {
            let camera_query = camera_queries.p0();
            is_deck_card_hit(
                pointer_position,
                camera_query.iter().next(),
                Some(global_transform),
                card_defaults,
            )
        }
        ActiveView::DeckScene => {
            let camera_query = camera_queries.p1();
            is_deck_card_hit(
                pointer_position,
                camera_query.iter().next(),
                Some(global_transform),
                card_defaults,
            )
        }
        ActiveView::DebugScene => {
            let camera_query = camera_queries.p2();
            is_deck_card_hit(
                pointer_position,
                camera_query.iter().next(),
                Some(global_transform),
                card_defaults,
            )
        }
        ActiveView::MainMenuScene
        | ActiveView::LightningScene
        | ActiveView::MatchmakingScene
        | ActiveView::SettingsScene => false,
    }
}

fn selection_source_matches_view(
    source: CardSelectionSource,
    active_view: ActiveView,
    game_scene: Option<&GameSceneEntity>,
    deck: Option<&DeckSceneEntity>,
    debug: Option<&DebugSceneEntity>,
) -> bool {
    match source {
        CardSelectionSource::CardViewBundle => match active_view {
            ActiveView::GameScene => game_scene.is_some(),
            ActiveView::DeckScene => deck.is_some(),
            ActiveView::DebugScene => debug.is_some(),
            ActiveView::MainMenuScene
            | ActiveView::LightningScene
            | ActiveView::MatchmakingScene
            | ActiveView::SettingsScene => false,
        },
        CardSelectionSource::LocalHand { .. }
        | CardSelectionSource::LocalLocation { .. }
        | CardSelectionSource::OpponentHand { .. }
        | CardSelectionSource::OpponentLocation { .. } => active_view == ActiveView::GameScene,
        CardSelectionSource::ScreenCard { view } => active_view == view,
    }
}

fn selectable_card_front_is_visible(
    source: CardSelectionSource,
    cpu_hand: Option<&CpuHandCardView>,
    cpu_placed: Option<&CpuPlacedCardView>,
    animation: Option<&CpuPlacedCardAnimation>,
    flip_state: &CardFlipState,
) -> bool {
    match source {
        CardSelectionSource::CardViewBundle => {
            if let Some(cpu_hand) = cpu_hand {
                cpu_hand.visible_face == CardFace::Front
            } else if let Some(cpu_placed) = cpu_placed {
                cpu_placed.visible_face == CardFace::Front
                    || animation
                        .is_some_and(|animation| animation.current_face() == CardFace::Front)
            } else {
                true
            }
        }
        CardSelectionSource::OpponentHand { .. } => {
            cpu_hand.is_some_and(|view| view.visible_face == CardFace::Front)
        }
        CardSelectionSource::OpponentLocation { .. } => {
            cpu_placed.is_some_and(|view| view.visible_face == CardFace::Front)
                || animation.is_some_and(|animation| animation.current_face() == CardFace::Front)
        }
        CardSelectionSource::ScreenCard { .. } => {
            flip_state.visible_face == CardFace::Front && !flip_state.is_animating()
        }
        CardSelectionSource::LocalHand { .. } | CardSelectionSource::LocalLocation { .. } => true,
    }
}

fn selectable_card_motion_allows_selection(
    source: CardSelectionSource,
    animation: Option<&CpuPlacedCardAnimation>,
) -> bool {
    match (source, animation) {
        (_, None) => true,
        (CardSelectionSource::OpponentLocation { .. }, Some(animation)) => {
            animation.phase == crate::runtime::components::CpuPlacedCardAnimationPhase::Revealing
                && animation.current_face() == CardFace::Front
        }
        _ => false,
    }
}

fn pointer_just_released(mouse_buttons: &ButtonInput<MouseButton>, touches: &Touches) -> bool {
    mouse_buttons.just_released(MouseButton::Left) || touches.iter_just_released().next().is_some()
}

#[cfg(test)]
#[path = "../../tests/runtime/systems/card_selection_update_system_tests.rs"]
mod card_selection_update_system_tests;
