use bevy::prelude::*;

/// HUMAN: Tracks one-shot scoped diagnostic logs for developer QA.
/// AI: Avoid per-frame logs and never include secrets or local credentials.
#[derive(Resource, Debug, Default)]
pub struct DebugLogState {
    pub startup_logged: bool,
}

pub fn safe_debug_log_message(scope: &str, message: &str) -> String {
    format!("debug-tooling::{scope}: {message}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scoped_log_message_has_no_secret_like_payload() {
        let message = safe_debug_log_message("qa", "tests can verify DebugHUD and Card UI");

        assert!(message.contains("debug-tooling::qa"));
        assert!(!message.to_lowercase().contains("password"));
        assert!(!message.to_lowercase().contains("token"));
    }
}
