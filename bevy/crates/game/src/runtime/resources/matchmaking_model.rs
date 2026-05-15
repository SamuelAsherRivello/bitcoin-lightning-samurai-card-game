use bevy::{asset::UntypedHandle, prelude::*};

use super::{
    ActiveLocations, ActiveWorldModel, CARD_BACK_TEXTURE_PATH, CARD_SAFE_AREA_TEXTURE_PATH,
    CardModelRegistry, GameFont, LocationModelRegistry, WorldModelRegistry,
};

pub const MATCHMAKING_SEARCH_SECONDS: f32 = 0.5;
pub const MATCHMAKING_FOUND_SECONDS: f32 = 0.5;
pub const MATCHMAKING_PREPARING_SECONDS: f32 = 0.5;
pub const MATCH_ASSETS_PRELOAD_ENABLED: bool = true;

/// HUMAN: Fake matchmaking presentation phase before entering GameScreen.
/// AI: Keep this deterministic; Loading is gated by Bevy asset readiness, not a fake timer.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum MatchmakingPhaseModel {
    #[default]
    Searching,
    Found,
    Loading,
    Preparing,
}

impl MatchmakingPhaseModel {
    pub const fn opponent_label(self) -> &'static str {
        match self {
            Self::Searching => "Searching",
            Self::Found | Self::Loading | Self::Preparing => "Player 02",
        }
    }

    pub const fn status_label(self) -> &'static str {
        match self {
            Self::Searching => "Searching...",
            Self::Found => "Matching...",
            Self::Loading => "Loading...",
            Self::Preparing => "Preparing...",
        }
    }
}

/// HUMAN: Runtime state for the temporary matchmaking screen.
/// AI: Reset this each time MatchmakingScreen is entered; handles keep preload assets alive.
#[derive(Resource, Clone, Debug, Default, PartialEq)]
pub struct MatchmakingModel {
    pub phase: MatchmakingPhaseModel,
    pub elapsed_seconds: f32,
    match_prepared: bool,
    preload_handles: Vec<UntypedHandle>,
}

impl MatchmakingModel {
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub const fn match_is_prepared(&self) -> bool {
        self.match_prepared
    }

    pub fn mark_match_prepared(&mut self) {
        self.match_prepared = true;
    }

    pub fn tick(&mut self, delta_seconds: f32, loading_complete: bool) -> bool {
        self.elapsed_seconds += delta_seconds.max(0.0);
        match self.phase {
            MatchmakingPhaseModel::Searching
                if self.elapsed_seconds >= MATCHMAKING_SEARCH_SECONDS =>
            {
                self.phase = MatchmakingPhaseModel::Found;
                self.elapsed_seconds = 0.0;
                false
            }
            MatchmakingPhaseModel::Found if self.elapsed_seconds >= MATCHMAKING_FOUND_SECONDS => {
                self.phase = if MATCH_ASSETS_PRELOAD_ENABLED {
                    MatchmakingPhaseModel::Loading
                } else {
                    MatchmakingPhaseModel::Preparing
                };
                self.elapsed_seconds = 0.0;
                false
            }
            MatchmakingPhaseModel::Loading if loading_complete => {
                self.phase = MatchmakingPhaseModel::Preparing;
                self.elapsed_seconds = 0.0;
                false
            }
            MatchmakingPhaseModel::Preparing
                if self.elapsed_seconds >= MATCHMAKING_PREPARING_SECONDS =>
            {
                true
            }
            _ => false,
        }
    }

    pub fn begin_preload(
        &mut self,
        asset_server: &AssetServer,
        card_model_registry: &CardModelRegistry,
        world_model_registry: &WorldModelRegistry,
        active_world_model: &ActiveWorldModel,
        location_model_registry: &LocationModelRegistry,
        active_locations: &ActiveLocations,
    ) {
        if !self.preload_handles.is_empty() {
            return;
        }

        let mut handles = Vec::new();
        for card_model in card_model_registry.card_models() {
            handles.push(
                asset_server
                    .load::<Image>(card_model.background_texture)
                    .untyped(),
            );
            handles.push(
                asset_server
                    .load::<Image>(card_model.frame_texture)
                    .untyped(),
            );
            handles.push(
                asset_server
                    .load::<Image>(card_model.foreground_texture)
                    .untyped(),
            );
            handles.push(
                asset_server
                    .load::<Image>(card_model.title_texture)
                    .untyped(),
            );
        }
        handles.push(asset_server.load::<Image>(CARD_BACK_TEXTURE_PATH).untyped());
        handles.push(
            asset_server
                .load::<Image>(CARD_SAFE_AREA_TEXTURE_PATH)
                .untyped(),
        );
        handles.push(
            asset_server
                .load::<Image>(
                    world_model_registry
                        .active_world_model(active_world_model)
                        .background_texture,
                )
                .untyped(),
        );
        for location in location_model_registry.selected_locations(active_locations) {
            handles.push(asset_server.load::<Image>(location.texture).untyped());
        }
        for font in GameFont::all() {
            handles.push(asset_server.load::<Font>(font.asset_path()).untyped());
        }

        self.preload_handles = handles;
    }

    pub fn preload_handle_count(&self) -> usize {
        self.preload_handles.len()
    }

    pub fn preload_is_complete(&self, asset_server: &AssetServer) -> bool {
        !self.preload_handles.is_empty()
            && self
                .preload_handles
                .iter()
                .all(|handle| asset_server.is_loaded_with_dependencies(handle))
    }

    pub fn opponent_label(&self) -> &'static str {
        self.phase.opponent_label()
    }

    pub fn status_label(&self) -> &'static str {
        self.phase.status_label()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matchmaking_phases_expose_requested_labels() {
        assert_eq!(
            MatchmakingPhaseModel::Searching.opponent_label(),
            "Searching"
        );
        assert_eq!(
            MatchmakingPhaseModel::Searching.status_label(),
            "Searching..."
        );
        assert_eq!(MatchmakingPhaseModel::Found.opponent_label(), "Player 02");
        assert_eq!(MatchmakingPhaseModel::Found.status_label(), "Matching...");
        assert_eq!(MatchmakingPhaseModel::Loading.opponent_label(), "Player 02");
        assert_eq!(MatchmakingPhaseModel::Loading.status_label(), "Loading...");
        assert_eq!(
            MatchmakingPhaseModel::Preparing.opponent_label(),
            "Player 02"
        );
        assert_eq!(
            MatchmakingPhaseModel::Preparing.status_label(),
            "Preparing..."
        );
    }

    #[test]
    fn matchmaking_spends_half_second_searching_then_half_second_found() {
        let mut model = MatchmakingModel::default();

        assert!(!model.tick(0.49, false));
        assert_eq!(model.phase, MatchmakingPhaseModel::Searching);
        assert!(!model.tick(0.01, false));
        assert_eq!(model.phase, MatchmakingPhaseModel::Found);
        assert_eq!(model.elapsed_seconds, 0.0);
        assert!(!model.tick(0.49, false));
        assert!(!model.tick(0.01, false));
        assert_eq!(model.phase, MatchmakingPhaseModel::Loading);
    }

    #[test]
    fn matchmaking_preparing_holds_for_half_second_after_loading_completes() {
        let mut model = MatchmakingModel {
            phase: MatchmakingPhaseModel::Loading,
            elapsed_seconds: 0.0,
            match_prepared: true,
            preload_handles: Vec::new(),
        };

        assert!(!model.tick(0.0, true));
        assert_eq!(model.phase, MatchmakingPhaseModel::Preparing);
        assert!(!model.tick(0.49, false));
        assert!(model.tick(0.01, false));
    }
}
