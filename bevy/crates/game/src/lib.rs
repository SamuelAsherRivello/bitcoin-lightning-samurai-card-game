use bevy::prelude::*;
use bevy_aspect_ratio_mask::{AspectRatioMask, AspectRatioPlugin, Resolution};
use bevy_card_game_shared::runtime::plugins::DebugToolingPlugin;
use bevy_inspector_egui::{
    DefaultInspectorConfigPlugin,
    bevy_egui::{EguiGlobalSettings, EguiPlugin, EguiPrimaryContextPass},
};
use bevy_tweening::TweeningPlugin;

pub mod runtime;

use runtime::plugins::CoreGamePlugin;
use runtime::shaders::materials::CardBackgroundMaskMaterial;
use runtime::systems::{card_ui, inspector_ui};

const GAME_SCENE_WIDTH: f32 = 1280.0;
const GAME_SCENE_HEIGHT: f32 = 800.0;
const APP_BACKGROUND_COLOR: Color = Color::srgb(0.08, 0.08, 0.08);

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AspectRatioPlugin {
            resolution: Resolution {
                width: GAME_SCENE_WIDTH,
                height: GAME_SCENE_HEIGHT,
            },
            mask: AspectRatioMask {
                color: APP_BACKGROUND_COLOR,
            },
            ..Default::default()
        })
        .add_plugins(MeshPickingPlugin)
        .add_plugins(TweeningPlugin)
        .add_plugins(MaterialPlugin::<CardBackgroundMaskMaterial>::default())
        .insert_resource(EguiGlobalSettings {
            auto_create_primary_context: false,
            ..Default::default()
        })
        .add_plugins(DebugToolingPlugin)
        .add_plugins(CoreGamePlugin)
        .add_plugins(EguiPlugin::default())
        .add_plugins(DefaultInspectorConfigPlugin)
        .add_systems(EguiPrimaryContextPass, (inspector_ui, card_ui));
    }
}
