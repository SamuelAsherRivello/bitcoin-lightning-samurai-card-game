use bevy::{
    prelude::*,
    remote::BrpResult,
    remote::{RemotePlugin, http::RemoteHttpPlugin},
    render::view::screenshot::{Screenshot, save_to_disk},
};
use serde_json::Value;
use std::path::PathBuf;

use crate::runtime::systems::{
    ai_runtime_show_deck_library_system, ai_runtime_show_deck_screen_system,
};

pub const AI_RUNTIME_BRP_ENDPOINT: &str = "http://localhost:15702";
pub const AI_RUNTIME_SCREENSHOT_METHOD: &str = "bevy_debugger/screenshot";
pub const AI_RUNTIME_SHOW_DECK_SCREEN_METHOD: &str = "bevy_debugger/show_deck_screen";
pub const AI_RUNTIME_SHOW_DECK_LIBRARY_METHOD: &str = "bevy_debugger/show_deck_library";
const AI_RUNTIME_SCREENSHOT_DIR: &str = "target/ai-runtime-screenshots";
const AI_RUNTIME_SCREENSHOT_NAME_MAX_LEN: usize = 64;

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
    let (path, response_path) = match ai_runtime_screenshot_path(params) {
        Ok((path, response_path)) => (path, response_path),
        Err(message) => {
            return Ok(serde_json::json!({
                "success": false,
                "error": message
            }));
        }
    };

    let screenshot_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(AI_RUNTIME_SCREENSHOT_DIR);
    let _ = std::fs::create_dir_all(&screenshot_dir);

    commands
        .spawn(Screenshot::primary_window())
        .observe(save_to_disk(path.clone()));

    Ok(serde_json::json!({
        "path": response_path,
        "success": true
    }))
}

/// HUMAN: Resolve BRP screenshot path input to a safe target inside the game crate screenshot directory.
/// AI: Reject non-string, traversal-like, and non-PNG filenames; return relative response paths only.
fn ai_runtime_screenshot_path(params: Option<Value>) -> Result<(PathBuf, String), String> {
    let requested_path = params
        .as_ref()
        .and_then(|value| value.get("path"))
        .and_then(Value::as_str)
        .unwrap_or("screenshot.png");

    let file_name = sanitize_ai_runtime_screenshot_name(requested_path)?;
    let screenshot_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(AI_RUNTIME_SCREENSHOT_DIR);
    let destination = screenshot_dir.join(file_name.clone());

    if destination.exists()
        && destination
            .symlink_metadata()
            .is_ok_and(|metadata| metadata.file_type().is_symlink())
    {
        return Err("Invalid path: resolved destination must not be a symlink".to_string());
    }

    Ok((
        destination,
        format!("{}/{}", AI_RUNTIME_SCREENSHOT_DIR, file_name),
    ))
}

/// HUMAN: Validate screenshot filename input to block path injection and unsafe characters.
/// AI: Enforce a conservative character set and force `.png`, while still keeping defaults.
fn sanitize_ai_runtime_screenshot_name(file_name: &str) -> Result<String, String> {
    if file_name.is_empty() {
        return Err("Invalid path: screenshot filename cannot be empty".to_string());
    }

    if file_name.contains('/') || file_name.contains('\\') {
        return Err("Invalid path: path separators are not allowed".to_string());
    }

    if file_name.chars().any(|character| {
        !character.is_ascii_alphanumeric() && !matches!(character, '_' | '-' | '.')
    }) {
        return Err(
            "Invalid path: only letters, numbers, '-', '_' and '.' are allowed".to_string(),
        );
    }

    if file_name == "." || file_name == ".." {
        return Err("Invalid path: invalid filename".to_string());
    }

    if file_name.len() > AI_RUNTIME_SCREENSHOT_NAME_MAX_LEN {
        return Err(format!(
            "Invalid path: filename exceeds {AI_RUNTIME_SCREENSHOT_NAME_MAX_LEN} characters"
        ));
    }

    if !file_name.to_ascii_lowercase().ends_with(".png") {
        return Err("Invalid path: screenshot file must use .png extension".to_string());
    }

    Ok(file_name.to_string())
}

#[cfg(test)]
#[path = "../../tests/runtime/plugins/ai_runtime_plugin_tests.rs"]
mod ai_runtime_plugin_tests;
