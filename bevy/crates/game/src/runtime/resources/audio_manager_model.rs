use bevy::prelude::*;

use crate::runtime::bundles::PointType;
use crate::runtime::resources::{
    CARD_SLOT_LOCATION_COUNT, CardSlotBoardModel, CardSlotSide, CardSlotState, GameLocationModel,
    GameLocationState, MatchModel, MatchPlayerSide,
};

pub const AUDIO_SFX_CLICK_01_PATH: &str = "audio/sfx/Click01.wav";
pub const AUDIO_SFX_SLIDE_01_PATH: &str = "audio/sfx/Slide01.wav";
pub const AUDIO_SFX_TAMBORINE_01_PATH: &str = "audio/sfx/Tamborine01.wav";
pub const AUDIO_SFX_UPGRADE_01_PATH: &str = "audio/sfx/Upgrade01.wav";
pub const AUDIO_SFX_FLIP_01_PATH: &str = "audio/sfx/Flip01.wav";

/// HUMAN: User-facing audio channel controlled by SettingsScreen preferences.
/// AI: Keep channel policy centralized so callers only request named audio.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AudioChannelModel {
    Sfx,
    Music,
}

/// HUMAN: Named game audio cue mapped one-to-one to a runtime asset.
/// AI: Add enum variants instead of loading paths directly from gameplay systems.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AudioEnum {
    ButtonClick,
    CardSlide,
    CardFlip,
    LocationOpen,
    LocationLeadChange,
}

impl AudioEnum {
    pub const fn channel(self) -> AudioChannelModel {
        match self {
            Self::ButtonClick
            | Self::CardSlide
            | Self::CardFlip
            | Self::LocationOpen
            | Self::LocationLeadChange => AudioChannelModel::Sfx,
        }
    }

    pub const fn asset_path(self) -> &'static str {
        match self {
            Self::ButtonClick => AUDIO_SFX_CLICK_01_PATH,
            Self::CardSlide => AUDIO_SFX_SLIDE_01_PATH,
            Self::CardFlip => AUDIO_SFX_FLIP_01_PATH,
            Self::LocationOpen => AUDIO_SFX_TAMBORINE_01_PATH,
            Self::LocationLeadChange => AUDIO_SFX_UPGRADE_01_PATH,
        }
    }

    pub const fn volume(self) -> f32 {
        match self {
            Self::LocationOpen => 0.6,
            Self::LocationLeadChange => 0.25,
            Self::CardFlip => 0.4,
            Self::ButtonClick | Self::CardSlide => 1.0,
        }
    }
}

/// HUMAN: A queued audio cue that can be inspected by tests before playback drains it.
/// AI: Keep requests value-like so systems can record behavior without human hearing.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AudioRequestModel {
    pub cue: AudioEnum,
}

impl AudioRequestModel {
    pub const fn new(cue: AudioEnum) -> Self {
        Self { cue }
    }
}

/// HUMAN: Winning side state for a shared location.
/// AI: None is the tied state; do not treat ties as lead-change sounds.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum LocationWinningSideModel {
    #[default]
    None,
    Near,
    Far,
}

/// HUMAN: Runtime audio request manager and transition memory.
/// AI: This resource observes state changes and drains requests into Bevy audio playback.
#[derive(Clone, Debug, Resource)]
pub struct AudioManagerModel {
    pub requests: Vec<AudioRequestModel>,
    previous_hand_counts: [usize; 2],
    previous_slot_counts: [[usize; 2]; CARD_SLOT_LOCATION_COUNT],
    previous_location_states: [GameLocationState; CARD_SLOT_LOCATION_COUNT],
    previous_winning_sides: [LocationWinningSideModel; CARD_SLOT_LOCATION_COUNT],
}

impl Default for AudioManagerModel {
    fn default() -> Self {
        Self {
            requests: Vec::new(),
            previous_hand_counts: [0; 2],
            previous_slot_counts: [[0; 2]; CARD_SLOT_LOCATION_COUNT],
            previous_location_states: [GameLocationState::Closed; CARD_SLOT_LOCATION_COUNT],
            previous_winning_sides: [LocationWinningSideModel::None; CARD_SLOT_LOCATION_COUNT],
        }
    }
}

impl AudioManagerModel {
    pub fn request(&mut self, cue: AudioEnum) {
        self.requests.push(AudioRequestModel::new(cue));
    }

    pub fn drain_enabled_requests(
        &mut self,
        sfx_enabled: bool,
        music_enabled: bool,
    ) -> Vec<AudioRequestModel> {
        let requests = std::mem::take(&mut self.requests);
        requests
            .into_iter()
            .filter(|request| match request.cue.channel() {
                AudioChannelModel::Sfx => sfx_enabled,
                AudioChannelModel::Music => music_enabled,
            })
            .collect()
    }

    pub fn observe_match_hands(&mut self, match_model: &MatchModel) {
        let next_counts = [match_model.near.hand.len(), match_model.far.hand.len()];
        for (index, next_count) in next_counts.into_iter().enumerate() {
            if next_count > self.previous_hand_counts[index] {
                for _ in self.previous_hand_counts[index]..next_count {
                    self.request(AudioEnum::CardSlide);
                }
            }
            self.previous_hand_counts[index] = next_count;
        }
    }

    pub fn observe_slot_board(&mut self, slot_board: &CardSlotBoardModel) {
        let mut next_counts = [[0; 2]; CARD_SLOT_LOCATION_COUNT];
        for slot in slot_board.slots() {
            if matches!(slot.state, CardSlotState::Populated { .. }) {
                next_counts[slot.location_index][slot_side_index(slot.side)] += 1;
            }
        }

        for (location_index, location_counts) in next_counts.iter().enumerate() {
            for (side_index, next_count) in location_counts.iter().copied().enumerate() {
                if next_count > self.previous_slot_counts[location_index][side_index] {
                    for _ in self.previous_slot_counts[location_index][side_index]..next_count {
                        self.request(AudioEnum::CardSlide);
                    }
                }
                self.previous_slot_counts[location_index][side_index] = next_count;
            }
        }
    }

    pub fn observe_locations(&mut self, game_location_model: &GameLocationModel) {
        for location_index in 0..CARD_SLOT_LOCATION_COUNT {
            let next_state = game_location_model
                .definition(location_index)
                .map(|definition| definition.state(game_location_model.round))
                .unwrap_or(GameLocationState::Closed);
            if self.previous_location_states[location_index] == GameLocationState::Closed
                && next_state == GameLocationState::Open
            {
                self.request(AudioEnum::LocationOpen);
            }
            self.previous_location_states[location_index] = next_state;
        }
    }

    pub fn observe_location_winners(&mut self, location_totals: &[(usize, CardSlotSide, i32)]) {
        for location_index in 0..CARD_SLOT_LOCATION_COUNT {
            let near = location_totals
                .iter()
                .find(|(index, side, _)| {
                    *index == location_index && *side == CardSlotSide::LocalPlayer
                })
                .map(|(_, _, value)| *value)
                .unwrap_or(0);
            let far = location_totals
                .iter()
                .find(|(index, side, _)| {
                    *index == location_index && *side == CardSlotSide::Opponent
                })
                .map(|(_, _, value)| *value)
                .unwrap_or(0);
            let next_winner = location_winning_side(near, far);
            if next_winner != LocationWinningSideModel::None
                && next_winner != self.previous_winning_sides[location_index]
            {
                self.request(AudioEnum::LocationLeadChange);
            }
            self.previous_winning_sides[location_index] = next_winner;
        }
    }
}

pub const fn location_winning_side(near: i32, far: i32) -> LocationWinningSideModel {
    if near > far {
        LocationWinningSideModel::Near
    } else if far > near {
        LocationWinningSideModel::Far
    } else {
        LocationWinningSideModel::None
    }
}

pub const fn match_side_index(side: MatchPlayerSide) -> usize {
    match side {
        MatchPlayerSide::Near => 0,
        MatchPlayerSide::Far => 1,
    }
}

pub const fn slot_side_index(side: CardSlotSide) -> usize {
    match side {
        CardSlotSide::LocalPlayer => match_side_index(MatchPlayerSide::Near),
        CardSlotSide::Opponent => match_side_index(MatchPlayerSide::Far),
    }
}

pub fn location_total_audio_entries(
    entries: impl IntoIterator<Item = (usize, CardSlotSide, PointType, i32)>,
) -> Vec<(usize, CardSlotSide, i32)> {
    entries
        .into_iter()
        .filter_map(|(location_index, side, point_type, value)| {
            (point_type == PointType::LocationPower).then_some((location_index, side, value))
        })
        .collect()
}
