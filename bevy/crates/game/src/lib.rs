use bevy::prelude::*;
use bevy_aspect_ratio_mask::{AspectRatioPlugin, Resolution};
use bevy_inspector_egui::{
    DefaultInspectorConfigPlugin,
    bevy_egui::{EguiGlobalSettings, EguiPlugin, EguiPrimaryContextPass},
};

pub mod runtime;

use runtime::plugins::CoreGamePlugin;
use runtime::systems::{card_ui, inspector_ui};

const GAME_VIEW_WIDTH: f32 = 1280.0;
const GAME_VIEW_HEIGHT: f32 = 800.0;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AspectRatioPlugin {
            resolution: Resolution {
                width: GAME_VIEW_WIDTH,
                height: GAME_VIEW_HEIGHT,
            },
            ..Default::default()
        })
        .add_plugins(MeshPickingPlugin)
        .insert_resource(EguiGlobalSettings {
            auto_create_primary_context: false,
            ..Default::default()
        })
        .add_plugins(CoreGamePlugin)
        .add_plugins(EguiPlugin::default())
        .add_plugins(DefaultInspectorConfigPlugin)
        .add_systems(EguiPrimaryContextPass, (inspector_ui, card_ui));
    }
}
