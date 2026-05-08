use bevy::prelude::*;

use crate::runtime::resources::{
    CardInspectionDefaults, CardInspectionState, DebugHudState, GameTicks, PrimaryCameraDefaults,
    WindowPlacementState,
};
use crate::runtime::systems::{
    advance_ticks, load_saved_window_placement, restore_window_placement_to_current_monitors,
    save_window_placement_on_close, scale_debug_hud, setup_card_placeholder, setup_debug_hud,
    setup_game, setup_inspector, setup_primary_camera, smooth_card_rotation, toggle_inspector,
    track_card_pointer_target, track_window_placement, track_window_size, update_debug_hud,
};

pub struct CoreGamePlugin;

impl Plugin for CoreGamePlugin {
    fn build(&self, app: &mut App) {
        let camera_defaults = PrimaryCameraDefaults::default();

        app.insert_resource(ClearColor(camera_defaults.clear_color))
            .insert_resource(camera_defaults)
            .init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<StandardMaterial>>()
            .init_resource::<GameTicks>()
            .init_resource::<CardInspectionDefaults>()
            .init_resource::<CardInspectionState>()
            .init_resource::<DebugHudState>()
            .init_resource::<WindowPlacementState>()
            .init_resource::<ButtonInput<KeyCode>>()
            .add_systems(
                Startup,
                (
                    load_saved_window_placement,
                    setup_game,
                    setup_primary_camera,
                    setup_card_placeholder,
                    setup_inspector,
                    setup_debug_hud,
                ),
            )
            .add_systems(
                Update,
                (
                    advance_ticks,
                    restore_window_placement_to_current_monitors,
                    track_window_placement,
                    track_window_size,
                    save_window_placement_on_close,
                    track_card_pointer_target,
                    smooth_card_rotation.after(track_card_pointer_target),
                    toggle_inspector,
                    update_debug_hud.after(toggle_inspector),
                    scale_debug_hud,
                ),
            );
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use std::time::Duration;

    use super::CoreGamePlugin;
    use crate::runtime::components::{
        CardPlaceholder, DebugHudKeyText, DebugHudText, InspectorState, Player, PrimarySceneCamera,
    };
    use crate::runtime::resources::{
        CARD_MAX_TILT_DEGREES, CardInspectionDefaults, CardInspectionState, DebugHudState,
        GameTicks, PrimaryCameraDefaults,
    };
    use crate::runtime::systems::{target_rotation_for_pointer, update_card_target_from_pointer};

    #[test]
    fn plugin_spawns_player_camera_card_debug_hud_inspector_and_advances_ticks() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins).add_plugins(CoreGamePlugin);

        app.update();

        let mut player_query = app.world_mut().query::<&Player>();
        let player_count = player_query.iter(app.world()).count();
        let mut hud_query = app.world_mut().query::<&DebugHudText>();
        let hud_count = hud_query.iter(app.world()).count();
        let mut inspector_query = app.world_mut().query::<&InspectorState>();
        let inspector_count = inspector_query.iter(app.world()).count();
        let mut card_query = app.world_mut().query::<&CardPlaceholder>();
        let card_count = card_query.iter(app.world()).count();
        let mut camera_query = app
            .world_mut()
            .query_filtered::<&PrimarySceneCamera, With<Camera3d>>();
        let camera_count = camera_query.iter(app.world()).count();
        let ticks = app.world().resource::<GameTicks>().0;

        assert_eq!(player_count, 1);
        assert_eq!(camera_count, 1);
        assert_eq!(card_count, 1);
        assert_eq!(hud_count, 1);
        assert_eq!(inspector_count, 1);
        assert_eq!(ticks, 1);
    }

    #[test]
    fn primary_camera_uses_documented_defaults() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins).add_plugins(CoreGamePlugin);

        app.update();

        let defaults = app.world().resource::<PrimaryCameraDefaults>().clone();
        let clear_color = app.world().resource::<ClearColor>().0;
        let mut camera_query = app.world_mut().query_filtered::<
            (&Name, &Transform, &Projection),
            (With<PrimarySceneCamera>, With<Camera3d>),
        >();
        let (name, transform, projection) = camera_query.single(app.world()).unwrap();

        assert_eq!(name.as_str(), "Primary 3D Camera");
        assert_eq!(transform.translation, defaults.position);
        assert_eq!(
            transform.rotation,
            defaults.transform().rotation,
            "camera should look at the configured target"
        );
        assert_eq!(clear_color, defaults.clear_color);

        let Projection::Perspective(perspective) = projection else {
            panic!("primary camera should use perspective projection");
        };

        assert_eq!(perspective.fov, defaults.fov_radians);
        assert_eq!(perspective.near, defaults.near);
        assert_eq!(perspective.far, defaults.far);
    }

    #[test]
    fn keyboard_input_does_not_move_primary_camera() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins).add_plugins(CoreGamePlugin);

        app.update();

        let mut camera_query = app
            .world_mut()
            .query_filtered::<&Transform, (With<PrimarySceneCamera>, With<Camera3d>)>();
        let initial_transform = *camera_query.single(app.world()).unwrap();

        {
            let mut keys = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
            keys.press(KeyCode::KeyW);
            keys.press(KeyCode::KeyA);
            keys.press(KeyCode::KeyS);
            keys.press(KeyCode::KeyD);
            keys.press(KeyCode::Space);
        }
        app.update();

        let current_transform = *camera_query.single(app.world()).unwrap();
        assert_eq!(current_transform, initial_transform);
    }

    #[test]
    fn card_placeholder_uses_plain_white_untextured_material_and_center_transform() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins).add_plugins(CoreGamePlugin);

        app.update();

        let mut card_query = app.world_mut().query_filtered::<(
            &Name,
            &Transform,
            &Mesh3d,
            &MeshMaterial3d<StandardMaterial>,
        ), With<CardPlaceholder>>();
        let (name, transform, _mesh, material_handle) = card_query.single(app.world()).unwrap();
        let materials = app.world().resource::<Assets<StandardMaterial>>();
        let material = materials.get(&material_handle.0).unwrap();

        assert_eq!(name.as_str(), "Poker Card Placeholder");
        assert_eq!(transform.translation, Vec3::ZERO);
        assert_eq!(transform.rotation, Quat::IDENTITY);
        assert_eq!(transform.scale, Vec3::ONE);
        assert_eq!(material.base_color, Color::WHITE);
        assert!(material.base_color_texture.is_none());
        assert!(material.unlit);
    }

    #[test]
    fn pointer_target_maps_corners_to_matching_card_normal_direction() {
        let defaults = CardInspectionDefaults::default();
        let rotation = target_rotation_for_pointer(Vec2::new(1.0, 1.0), &defaults);
        let normal = rotation * Vec3::Z;
        let (yaw, pitch, roll) = rotation.to_euler(EulerRot::YXZ);

        assert!(normal.x > 0.0);
        assert!(normal.y < 0.0);
        assert!(yaw.abs() <= CARD_MAX_TILT_DEGREES.to_radians());
        assert!(pitch.abs() <= CARD_MAX_TILT_DEGREES.to_radians());
        assert!(roll.abs() < 0.0001);
    }

    #[test]
    fn pointer_target_clamps_to_twenty_degrees_per_axis() {
        let defaults = CardInspectionDefaults::default();
        let rotation = target_rotation_for_pointer(Vec2::new(8.0, -8.0), &defaults);
        let (yaw, pitch, _roll) = rotation.to_euler(EulerRot::YXZ);

        assert!((yaw - CARD_MAX_TILT_DEGREES.to_radians()).abs() < 0.0001);
        assert!((pitch + CARD_MAX_TILT_DEGREES.to_radians()).abs() < 0.0001);
    }

    #[test]
    fn pointer_position_updates_target_from_window_normalized_coordinates() {
        let defaults = CardInspectionDefaults::default();
        let mut state = CardInspectionState::default();

        update_card_target_from_pointer(
            Vec2::new(800.0, 600.0),
            Vec2::new(800.0, 600.0),
            &defaults,
            &mut state,
        );

        assert_eq!(state.last_pointer_normalized, Vec2::ONE);
        assert_ne!(state.target_rotation, Quat::IDENTITY);
    }

    #[test]
    fn card_rotation_smooths_toward_target_without_moving_camera() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins).add_plugins(CoreGamePlugin);

        app.update();
        let camera_transform = *app
            .world_mut()
            .query_filtered::<&Transform, (With<PrimarySceneCamera>, With<Camera3d>)>()
            .single(app.world())
            .unwrap();
        let target_rotation = target_rotation_for_pointer(
            Vec2::new(1.0, 1.0),
            app.world().resource::<CardInspectionDefaults>(),
        );
        {
            let mut state = app.world_mut().resource_mut::<CardInspectionState>();
            state.target_rotation = target_rotation;
        }
        app.world_mut()
            .resource_mut::<Time>()
            .advance_by(Duration::from_millis(16));
        app.update();

        let mut card_query = app
            .world_mut()
            .query_filtered::<&Transform, With<CardPlaceholder>>();
        let card_transform = *card_query.single(app.world()).unwrap();
        let current_camera_transform = *app
            .world_mut()
            .query_filtered::<&Transform, (With<PrimarySceneCamera>, With<Camera3d>)>()
            .single(app.world())
            .unwrap();

        assert_ne!(card_transform.rotation, Quat::IDENTITY);
        assert_ne!(card_transform.rotation, target_rotation);
        assert_eq!(card_transform.translation, Vec3::ZERO);
        assert_eq!(current_camera_transform, camera_transform);
    }

    #[test]
    fn hud_contains_wasd_f_and_i_key_labels() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins).add_plugins(CoreGamePlugin);

        app.update();

        let mut key_query = app.world_mut().query::<&DebugHudKeyText>();
        let keys: Vec<KeyCode> = key_query
            .iter(app.world())
            .map(|key_text| key_text.key_code)
            .collect();

        assert!(keys.contains(&KeyCode::KeyW));
        assert!(keys.contains(&KeyCode::KeyA));
        assert!(keys.contains(&KeyCode::KeyS));
        assert!(keys.contains(&KeyCode::KeyD));
        assert!(keys.contains(&KeyCode::KeyF));
        assert!(keys.contains(&KeyCode::KeyI));
    }

    #[test]
    fn f_toggles_fps_visibility() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins).add_plugins(CoreGamePlugin);

        app.update();
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyF);
        app.update();

        assert!(app.world().resource::<DebugHudState>().is_fps_visible);
    }

    #[test]
    fn i_toggles_inspector_visibility() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins).add_plugins(CoreGamePlugin);

        app.update();
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyI);
        app.update();

        let mut inspector_query = app.world_mut().query::<&InspectorState>();
        let inspector = inspector_query.single(app.world()).unwrap();
        assert!(inspector.is_visible);
        assert_eq!(inspector.width, 676.0);
    }

    #[test]
    fn wasd_keys_do_not_toggle_debug_features() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins).add_plugins(CoreGamePlugin);

        app.update();
        {
            let mut keys = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
            keys.press(KeyCode::KeyW);
            keys.press(KeyCode::KeyA);
            keys.press(KeyCode::KeyS);
            keys.press(KeyCode::KeyD);
        }
        app.update();

        let mut inspector_query = app.world_mut().query::<&InspectorState>();
        let inspector = inspector_query.single(app.world()).unwrap();
        assert!(!app.world().resource::<DebugHudState>().is_fps_visible);
        assert!(!inspector.is_visible);
    }
}
