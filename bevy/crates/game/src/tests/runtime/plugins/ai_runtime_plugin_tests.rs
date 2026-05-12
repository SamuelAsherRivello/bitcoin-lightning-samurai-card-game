use super::*;

#[test]
fn ai_runtime_endpoint_is_localhost() {
    assert_eq!(AI_RUNTIME_BRP_ENDPOINT, "http://localhost:15702");
}

#[test]
fn ai_runtime_screenshot_method_matches_mcp_contract() {
    assert_eq!(AI_RUNTIME_SCREENSHOT_METHOD, "bevy_debugger/screenshot");
}
