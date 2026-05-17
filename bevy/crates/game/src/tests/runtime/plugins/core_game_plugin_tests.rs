use super::*;
use crate::runtime::shaders::materials::CardBackgroundMaskMaterial;
use bevy::app::AppExit;
use bevy::asset::AssetPlugin;
use bevy::input::{
    keyboard::{KeyboardFocusLost, KeyboardInput},
    mouse::{MouseButtonInput, MouseWheel},
};
use bevy::prelude::{
    App, Assets, ButtonInput, CursorMoved, FileDragAndDrop, Font, Image, Ime, MinimalPlugins,
    MouseButton, StandardMaterial, TouchInput, Touches, With,
};
use bevy::shader::Shader;
use bevy::window::{RequestRedraw, WindowCloseRequested};
#[cfg(feature = "desktop-hot-reload")]
use bevy_hotpatching_experiments::HotPatched;
use bevy_inspector_egui::bevy_egui::{
    EguiContext, EguiGlobalSettings, EguiMultipassSchedule, EguiPlugin, PrimaryEguiContext,
};

#[test]
fn core_game_plugin_update_schedule_initializes_without_ordering_cycle() {
    let mut app = core_game_test_app();

    app.add_plugins(CoreGamePlugin);

    app.update();
}

#[test]
fn core_game_plugin_keeps_one_primary_egui_context_for_multipass() {
    let mut app = core_game_test_app();
    app.insert_resource(EguiGlobalSettings {
        auto_create_primary_context: false,
        ..Default::default()
    })
    .add_plugins(CoreGamePlugin)
    .add_plugins(EguiPlugin::default());

    app.update();

    let primary_context_count = app
        .world_mut()
        .query_filtered::<(), With<PrimaryEguiContext>>()
        .iter(app.world())
        .count();
    let multipass_context_count = app
        .world_mut()
        .query_filtered::<(), (With<EguiContext>, With<EguiMultipassSchedule>)>()
        .iter(app.world())
        .count();
    assert_eq!(primary_context_count, 1);
    assert_eq!(multipass_context_count, 1);
}

fn core_game_test_app() -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default()))
        .add_message::<AppExit>()
        .add_message::<CursorMoved>()
        .add_message::<FileDragAndDrop>()
        .add_message::<Ime>()
        .add_message::<KeyboardInput>()
        .add_message::<KeyboardFocusLost>()
        .add_message::<MouseButtonInput>()
        .add_message::<MouseWheel>()
        .add_message::<TouchInput>()
        .add_message::<RequestRedraw>()
        .add_message::<WindowCloseRequested>()
        .init_resource::<ButtonInput<MouseButton>>()
        .init_resource::<Touches>()
        .init_resource::<Assets<StandardMaterial>>()
        .init_resource::<Assets<CardBackgroundMaskMaterial>>()
        .init_resource::<Assets<Shader>>()
        .init_asset::<Image>()
        .init_asset::<Font>();
    #[cfg(feature = "desktop-hot-reload")]
    app.add_message::<HotPatched>();
    app
}
