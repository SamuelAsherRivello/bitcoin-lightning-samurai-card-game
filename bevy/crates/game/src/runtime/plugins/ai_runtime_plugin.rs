use bevy::{
    prelude::*,
    remote::BrpResult,
    remote::{RemotePlugin, http::RemoteHttpPlugin},
    render::view::screenshot::{Screenshot, save_to_disk},
};
use serde_json::Value;

use crate::runtime::systems::{
    ai_runtime_show_deck_library_system, ai_runtime_show_deck_screen_system,
};

pub const AI_RUNTIME_BRP_ENDPOINT: &str = "http://localhost:15702";
pub const AI_RUNTIME_SCREENSHOT_METHOD: &str = "bevy_debugger/screenshot";
pub const AI_RUNTIME_SHOW_DECK_SCREEN_METHOD: &str = "bevy_debugger/show_deck_screen";
pub const AI_RUNTIME_SHOW_DECK_LIBRARY_METHOD: &str = "bevy_debugger/show_deck_library";

/// HUMAN: Development-only plugin that exposes Bevy Remote Protocol for AI runtime inspection.
/// AI: Keep this opt-in behind the ai-runtime feature and native desktop targets only.
pub struct AiRuntimePlugin;

impl Plugin for AiRuntimePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            RemotePlugin::default()
                .with_method(AI_RUNTIME_SCREENSHOT_METHOD, ai_runtime_screenshot_handler)
                .with_method(
                    AI_RUNTIME_SHOW_DECK_SCREEN_METHOD,
                    ai_runtime_show_deck_screen_system,
                )
                .with_method(
                    AI_RUNTIME_SHOW_DECK_LIBRARY_METHOD,
                    ai_runtime_show_deck_library_system,
                ),
        )
        .add_plugins(RemoteHttpPlugin::default());
    }
}

fn ai_runtime_screenshot_handler(
    In(params): In<Option<Value>>,
    mut commands: Commands,
) -> BrpResult {
    let path = params
        .as_ref()
        .and_then(|value| value.get("path"))
        .and_then(Value::as_str)
        .unwrap_or("target/ai-runtime-screenshots/screenshot.png")
        .to_string();

    if let Some(parent) = std::path::Path::new(&path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    commands
        .spawn(Screenshot::primary_window())
        .observe(save_to_disk(path.clone()));

    Ok(serde_json::json!({
        "path": path,
        "success": true
    }))
}

#[cfg(test)]
#[path = "../../tests/runtime/plugins/ai_runtime_plugin_tests.rs"]
mod ai_runtime_plugin_tests;
