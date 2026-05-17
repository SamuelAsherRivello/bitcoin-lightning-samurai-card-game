use super::*;

#[test]
fn ai_runtime_endpoint_is_localhost() {
    assert_eq!(AI_RUNTIME_BRP_ENDPOINT, "http://localhost:15702");
}

#[test]
fn ai_runtime_screenshot_method_matches_mcp_contract() {
    assert_eq!(AI_RUNTIME_SCREENSHOT_METHOD, "bevy_debugger/screenshot");
}

#[test]
fn ai_runtime_show_deck_screen_method_matches_mcp_contract() {
    assert_eq!(
        AI_RUNTIME_SHOW_DECK_SCREEN_METHOD,
        "bevy_debugger/show_deck_screen"
    );
}

#[test]
fn ai_runtime_show_deck_library_method_matches_mcp_contract() {
    assert_eq!(
        AI_RUNTIME_SHOW_DECK_LIBRARY_METHOD,
        "bevy_debugger/show_deck_library"
    );
}

#[test]
fn ai_runtime_show_game_screen_method_matches_mcp_contract() {
    assert_eq!(
        AI_RUNTIME_SHOW_GAME_SCREEN_METHOD,
        "bevy_debugger/show_game_screen"
    );
}

#[test]
fn ai_runtime_show_debug_screen_method_matches_mcp_contract() {
    assert_eq!(
        AI_RUNTIME_SHOW_DEBUG_SCREEN_METHOD,
        "bevy_debugger/show_debug_screen"
    );
}

#[test]
fn ai_runtime_on_card_clicked_method_matches_mcp_contract() {
    assert_eq!(
        AI_RUNTIME_ON_CARD_CLICKED_METHOD,
        "bevy_debugger/on_card_clicked"
    );
}

#[test]
fn ai_runtime_mouse_methods_match_mcp_contract() {
    assert_eq!(AI_RUNTIME_MOUSE_PRESS_METHOD, "bevy_debugger/mouse_press");
    assert_eq!(
        AI_RUNTIME_MOUSE_RELEASE_METHOD,
        "bevy_debugger/mouse_release"
    );
}

#[test]
fn ai_runtime_screenshot_rejects_path_traversal() {
    assert!(
        ai_runtime_screenshot_path(Some(serde_json::json!({"path":"../../etc/passwd"}))).is_err()
    );
}

#[test]
fn ai_runtime_screenshot_rejects_invalid_characters() {
    assert!(ai_runtime_screenshot_path(Some(serde_json::json!({"path":"bad/name.png"}))).is_err());
    assert!(ai_runtime_screenshot_path(Some(serde_json::json!({"path":"bad\\name.png"}))).is_err());
}

#[test]
fn ai_runtime_screenshot_rejects_missing_extension() {
    assert!(ai_runtime_screenshot_path(Some(serde_json::json!({"path":"screenshot"}))).is_err());
}

#[test]
fn ai_runtime_screenshot_accepts_default_path() {
    assert!(ai_runtime_screenshot_path(None).is_ok());
}

#[test]
fn ai_runtime_pointer_position_accepts_numeric_coordinates() {
    assert_eq!(
        ai_runtime_pointer_position(Some(serde_json::json!({"x":12.5,"y":99.0}))).unwrap(),
        Vec2::new(12.5, 99.0)
    );
}

#[test]
fn ai_runtime_pointer_position_rejects_missing_or_negative_coordinates() {
    assert!(ai_runtime_pointer_position(None).is_err());
    assert!(ai_runtime_pointer_position(Some(serde_json::json!({"x":12.5}))).is_err());
    assert!(ai_runtime_pointer_position(Some(serde_json::json!({"x":-1.0,"y":99.0}))).is_err());
}
