use bevy::prelude::*;

use crate::runtime::resources::{DebugLogState, safe_debug_log_message};

/// HUMAN: Emits one scoped startup log for developer QA.
/// AI: Keep logs sparse and redact-free; tests should inspect behavior instead of terminal noise.
pub fn debug_log_update_system(mut log_state: ResMut<DebugLogState>) {
    if log_state.startup_logged {
        return;
    }

    info!(
        "{}",
        safe_debug_log_message(
            "startup",
            "DebugHUD, inspector, Card UI separation, and debug drawing checks are available"
        )
    );
    log_state.startup_logged = true;
}
