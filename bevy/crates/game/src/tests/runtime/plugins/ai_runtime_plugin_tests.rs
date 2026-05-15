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
