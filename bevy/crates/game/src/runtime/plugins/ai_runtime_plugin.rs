use bevy::{
    prelude::*,
    remote::BrpResult,
    remote::{RemotePlugin, http::RemoteHttpPlugin},
    render::view::screenshot::{Screenshot, save_to_disk},
};
use serde_json::Value;

pub const AI_RUNTIME_BRP_ENDPOINT: &str = "http://localhost:15702";
pub const AI_RUNTIME_SCREENSHOT_METHOD: &str = "bevy_debugger/screenshot";

/// HUMAN: Development-only plugin that exposes Bevy Remote Protocol for AI runtime inspection.
/// AI: Keep this opt-in behind the ai-runtime feature and native desktop targets only.
pub struct AiRuntimePlugin;

impl Plugin for AiRuntimePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            RemotePlugin::default()
                .with_method(AI_RUNTIME_SCREENSHOT_METHOD, ai_runtime_screenshot_handler),
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
mod tests {
    use super::*;

    #[test]
    fn ai_runtime_endpoint_is_localhost() {
        assert_eq!(AI_RUNTIME_BRP_ENDPOINT, "http://localhost:15702");
    }

    #[test]
    fn ai_runtime_screenshot_method_matches_mcp_contract() {
        assert_eq!(AI_RUNTIME_SCREENSHOT_METHOD, "bevy_debugger/screenshot");
    }
}
