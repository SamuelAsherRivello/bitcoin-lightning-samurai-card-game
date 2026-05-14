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
