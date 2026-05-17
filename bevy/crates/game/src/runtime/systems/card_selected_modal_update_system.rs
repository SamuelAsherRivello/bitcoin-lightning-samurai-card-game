use bevy::{
    camera::visibility::{NoCpuCulling, NoFrustumCulling, RenderLayers},
    prelude::*,
    window::{PrimaryWindow, Window},
};

use crate::runtime::components::SelectedCardModalBackdrop;
use crate::runtime::resources::{
    CardGestureModel, CardGestureState, SELECTED_CARD_MODAL_SETTLE_EPSILON, SelectedCardModalModel,
};

use super::{
    CARD_GESTURE_SELECTED_Z, CARD_RENDER_LAYER, active_pointer_position,
    game_scene_perspective_view_size_at_z,
};

const SELECTED_MODAL_BACKDROP_Z: f32 = 0.86;
const SELECTED_MODAL_BACKDROP_DEPTH_BIAS: f32 = 64.0;
const SELECTED_MODAL_ANIMATION_RATE: f32 = 14.0;

/// HUMAN: Updates selected-card modal dimming, dismissal, and passive card transforms.
/// AI: Keep input capture here so lower systems can simply check SelectedCardModalModel.
pub fn card_selected_modal_update_system(
    mut commands: Commands,
    time: Res<Time>,
    primary_window_query: Query<&Window, With<PrimaryWindow>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut selected_modal: ResMut<SelectedCardModalModel>,
    mut gesture_model: ResMut<CardGestureModel>,
    mut selected_transform_query: Query<
        (Entity, &mut Transform, &GlobalTransform),
        Without<SelectedCardModalBackdrop>,
    >,
    mut backdrop_query: Query<
        (
            Entity,
            &mut Mesh3d,
            &MeshMaterial3d<StandardMaterial>,
            &mut Transform,
        ),
        With<SelectedCardModalBackdrop>,
    >,
) {
    if !selected_modal.is_active() {
        selected_modal.cancel_press_candidate();
        despawn_backdrops(&mut commands, &backdrop_query);
        return;
    }

    if selected_entity_is_missing(&selected_modal, &selected_transform_query) {
        selected_modal.clear();
        despawn_backdrops(&mut commands, &backdrop_query);
        return;
    }

    handle_modal_pointer_press(
        &primary_window_query,
        &mouse_buttons,
        &touches,
        &mut selected_modal,
        &mut gesture_model,
    );

    let interpolation = selected_modal_interpolation(time.delta_secs());
    selected_modal.advance_fade_with_interpolation(interpolation);
    sync_backdrop(
        &mut commands,
        &mut meshes,
        &mut materials,
        &selected_modal,
        &mut backdrop_query,
    );
    animate_selected_card(
        interpolation,
        &mut selected_modal,
        &mut selected_transform_query,
    );

    if selected_modal.dismiss_pending
        && selected_card_should_clear(&selected_modal, &mut selected_transform_query)
    {
        selected_modal.clear();
        despawn_backdrops(&mut commands, &backdrop_query);
    }
}

fn handle_modal_pointer_press(
    primary_window_query: &Query<&Window, With<PrimaryWindow>>,
    mouse_buttons: &ButtonInput<MouseButton>,
    touches: &Touches,
    selected_modal: &mut SelectedCardModalModel,
    gesture_model: &mut CardGestureModel,
) {
    if !pointer_just_pressed(mouse_buttons, touches) {
        return;
    }
    if selected_modal.take_suppressed_pointer_dismiss() {
        return;
    }
    let Ok(primary_window) = primary_window_query.single() else {
        return;
    };
    let Some(pointer_position) = active_pointer_position(primary_window, touches) else {
        return;
    };
    let _ = pointer_position;

    selected_modal.request_dismiss();
    if gesture_model.state == CardGestureState::SelectedInspecting {
        gesture_model.return_to_source();
    }
}

fn sync_backdrop(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    selected_modal: &SelectedCardModalModel,
    backdrop_query: &mut Query<
        (
            Entity,
            &mut Mesh3d,
            &MeshMaterial3d<StandardMaterial>,
            &mut Transform,
        ),
        With<SelectedCardModalBackdrop>,
    >,
) {
    let opacity = selected_modal.opacity();
    let view_size = game_scene_perspective_view_size_at_z(SELECTED_MODAL_BACKDROP_Z);
    let transform = Transform::from_translation(Vec3::new(0.0, 0.0, SELECTED_MODAL_BACKDROP_Z));

    if backdrop_query.is_empty() {
        let material = materials.add(StandardMaterial {
            base_color: Color::srgba(0.0, 0.0, 0.0, opacity),
            alpha_mode: AlphaMode::Blend,
            depth_bias: SELECTED_MODAL_BACKDROP_DEPTH_BIAS,
            unlit: true,
            ..Default::default()
        });
        commands.spawn((
            Name::new("Selected Card Modal Backdrop"),
            SelectedCardModalBackdrop,
            Mesh3d(meshes.add(Rectangle::new(view_size.x, view_size.y))),
            MeshMaterial3d(material),
            transform,
            RenderLayers::layer(CARD_RENDER_LAYER),
            Visibility::Visible,
            NoCpuCulling,
            NoFrustumCulling,
        ));
        return;
    }

    for (_, mut mesh, material, mut backdrop_transform) in backdrop_query {
        *mesh = Mesh3d(meshes.add(Rectangle::new(view_size.x, view_size.y)));
        if let Some(material) = materials.get_mut(&material.0) {
            material.base_color = Color::srgba(0.0, 0.0, 0.0, opacity);
        }
        *backdrop_transform = transform;
    }
}

fn animate_selected_card(
    interpolation: f32,
    selected_modal: &mut SelectedCardModalModel,
    selected_transform_query: &mut Query<
        (Entity, &mut Transform, &GlobalTransform),
        Without<SelectedCardModalBackdrop>,
    >,
) {
    let Some(selected_entity) = selected_modal.selected_entity else {
        return;
    };
    let target_transform = if selected_modal.dismiss_pending {
        selected_modal.source_transform
    } else {
        selected_modal.target_transform
    };
    let Some(target_transform) = target_transform else {
        return;
    };
    let Ok((_, mut transform, _)) = selected_transform_query.get_mut(selected_entity) else {
        return;
    };
    transform.translation = transform
        .translation
        .lerp(target_transform.translation, interpolation);
    if !selected_modal.dismiss_pending {
        transform.translation.z = transform.translation.z.max(CARD_GESTURE_SELECTED_Z);
    }
    transform.scale = transform.scale.lerp(target_transform.scale, interpolation);
    transform.rotation = transform
        .rotation
        .slerp(target_transform.rotation, interpolation);
}

fn selected_card_should_clear(
    selected_modal: &SelectedCardModalModel,
    selected_transform_query: &mut Query<
        (Entity, &mut Transform, &GlobalTransform),
        Without<SelectedCardModalBackdrop>,
    >,
) -> bool {
    let (Some(selected_entity), Some(source_transform)) = (
        selected_modal.selected_entity,
        selected_modal.source_transform,
    ) else {
        return true;
    };
    let Ok((_, mut transform, _)) = selected_transform_query.get_mut(selected_entity) else {
        return true;
    };
    let is_at_source = transform.translation.distance(source_transform.translation)
        <= SELECTED_CARD_MODAL_SETTLE_EPSILON
        && transform.scale.distance(source_transform.scale) <= SELECTED_CARD_MODAL_SETTLE_EPSILON
        && transform.rotation.angle_between(source_transform.rotation)
            <= SELECTED_CARD_MODAL_SETTLE_EPSILON;
    if is_at_source || selected_modal.fade_progress() <= SELECTED_CARD_MODAL_SETTLE_EPSILON {
        *transform = source_transform;
        return true;
    }
    false
}

fn selected_entity_is_missing(
    selected_modal: &SelectedCardModalModel,
    selected_transform_query: &Query<
        (Entity, &mut Transform, &GlobalTransform),
        Without<SelectedCardModalBackdrop>,
    >,
) -> bool {
    selected_modal
        .selected_entity
        .is_some_and(|entity| selected_transform_query.get(entity).is_err())
}

fn despawn_backdrops(
    commands: &mut Commands,
    backdrop_query: &Query<
        (
            Entity,
            &mut Mesh3d,
            &MeshMaterial3d<StandardMaterial>,
            &mut Transform,
        ),
        With<SelectedCardModalBackdrop>,
    >,
) {
    for (entity, _, _, _) in backdrop_query {
        commands.entity(entity).despawn();
    }
}

fn pointer_just_pressed(mouse_buttons: &ButtonInput<MouseButton>, touches: &Touches) -> bool {
    mouse_buttons.just_pressed(MouseButton::Left) || touches.iter_just_pressed().next().is_some()
}

fn selected_modal_interpolation(delta_seconds: f32) -> f32 {
    (delta_seconds * SELECTED_MODAL_ANIMATION_RATE).clamp(0.0, 1.0)
}

#[cfg(test)]
#[path = "../../tests/runtime/systems/card_selected_modal_update_system_tests.rs"]
mod card_selected_modal_update_system_tests;
