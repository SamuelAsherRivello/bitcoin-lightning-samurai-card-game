use bevy::{
    audio::{PlaybackSettings, Volume},
    prelude::*,
};

use crate::runtime::{
    bundles::{PointLocationView, PointType},
    resources::{
        ActiveView, AudioManagerModel, GameLocationModel, MatchModel, MetaGameSettingsModel,
    },
};

use super::is_game_scene_active;

/// HUMAN: Converts accepted gameplay state transitions into audio requests.
/// AI: Run after gameplay/UI point state updates so repeated redraws do not replay sounds.
pub fn audio_game_state_update_system(
    active_view: Option<Res<ActiveView>>,
    mut audio_manager: ResMut<AudioManagerModel>,
    match_model: Option<Res<MatchModel>>,
    slot_board: Res<crate::runtime::resources::CardSlotBoardModel>,
    game_location_model: Option<Res<GameLocationModel>>,
    location_points: Query<(&PointLocationView, &crate::runtime::bundles::PointView)>,
) {
    if !is_game_scene_active(active_view.as_deref()) {
        return;
    }

    if let Some(match_model) = match_model.as_deref() {
        audio_manager.observe_match_hands(match_model);
    }
    audio_manager.observe_slot_board(&slot_board);
    if let Some(game_location_model) = game_location_model.as_deref() {
        audio_manager.observe_locations(game_location_model);
    }

    let totals = crate::runtime::resources::location_total_audio_entries(
        location_points
            .iter()
            .map(|(location, point)| {
                (
                    location.location_index,
                    location.side,
                    point.model.point_type,
                    point.model.value,
                )
            })
            .filter(|(_, _, point_type, _)| *point_type == PointType::LocationPower),
    );
    audio_manager.observe_location_winners(&totals);
}

/// HUMAN: Drains enabled audio requests into Bevy audio playback entities.
/// AI: Tests can inspect AudioManagerModel before this system drains requests.
pub fn audio_playback_update_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    settings: Res<MetaGameSettingsModel>,
    mut audio_manager: ResMut<AudioManagerModel>,
) {
    for request in
        audio_manager.drain_enabled_requests(settings.sfx_enabled, settings.music_enabled)
    {
        commands.spawn((
            AudioPlayer::new(asset_server.load(request.cue.asset_path())),
            PlaybackSettings::DESPAWN.with_volume(Volume::Linear(request.cue.volume())),
        ));
    }
}
