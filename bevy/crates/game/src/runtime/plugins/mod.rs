pub mod core_game_plugin;

#[cfg(all(feature = "ai-runtime", not(target_arch = "wasm32")))]
mod ai_runtime_plugin;

pub use core_game_plugin::CoreGamePlugin;
#[cfg(all(feature = "ai-runtime", not(target_arch = "wasm32")))]
pub use ai_runtime_plugin::{
    AI_RUNTIME_BRP_ENDPOINT, AI_RUNTIME_MOUSE_PRESS_METHOD, AI_RUNTIME_MOUSE_RELEASE_METHOD,
    AI_RUNTIME_ON_CARD_CLICKED_METHOD, AI_RUNTIME_SCREENSHOT_METHOD,
    AI_RUNTIME_SHOW_DEBUG_SCREEN_METHOD, AI_RUNTIME_SHOW_DECK_LIBRARY_METHOD,
    AI_RUNTIME_SHOW_DECK_SCREEN_METHOD, AI_RUNTIME_SHOW_GAME_SCREEN_METHOD, AiRuntimePlugin,
};
