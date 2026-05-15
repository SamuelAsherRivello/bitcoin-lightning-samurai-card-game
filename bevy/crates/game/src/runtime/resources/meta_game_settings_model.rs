use bevy::prelude::Resource;
use bevy_persistent::{error::PersistenceError, prelude::*};
use serde::{Deserialize, Serialize};

use crate::runtime::resources::{MatchModeModel, workspace_root_path_for_game};

/// HUMAN: Persisted CPU Brain difficulty choice for pre-game settings.
/// AI: Level1 is the only valid value until future CPU brains are authored.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum CpuBrainLevelSettingModel {
    #[default]
    Level1,
}

impl CpuBrainLevelSettingModel {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Level1 => "1",
        }
    }

    pub const fn next(self) -> Self {
        match self {
            Self::Level1 => Self::Level1,
        }
    }
}

/// HUMAN: Pre-game settings that drive future matches and audio preferences.
/// AI: Keep this local and explicit; audio flags are stored before audio systems exist.
#[derive(Clone, Debug, Deserialize, PartialEq, Resource, Serialize)]
pub struct MetaGameSettingsModel {
    pub cpu_brain_level: CpuBrainLevelSettingModel,
    pub selected_mode: MatchModeModel,
    pub sfx_enabled: bool,
    pub music_enabled: bool,
    pub framerate: u8,
}

impl Default for MetaGameSettingsModel {
    fn default() -> Self {
        Self {
            cpu_brain_level: CpuBrainLevelSettingModel::Level1,
            selected_mode: MatchModeModel::HumanVersusCpu,
            sfx_enabled: true,
            music_enabled: true,
            framerate: 120,
        }
    }
}

impl MetaGameSettingsModel {
    const ALLOWED_FRAMERATES: [u8; 3] = [30, 60, 120];

    pub fn cycle_cpu_brain_level(&mut self) {
        self.cpu_brain_level = self.cpu_brain_level.next();
    }

    pub fn toggle_mode(&mut self) {
        self.selected_mode = self.selected_mode.next();
    }

    pub fn toggle_sfx(&mut self) {
        self.sfx_enabled = !self.sfx_enabled;
    }

    pub fn toggle_music(&mut self) {
        self.music_enabled = !self.music_enabled;
    }

    pub fn toggle_framerate(&mut self) {
        self.framerate = match self.framerate {
            30 => 60,
            60 => 120,
            120 => 30,
            _ => 30,
        };
    }

    pub const fn framerate_label(&self) -> &'static str {
        match self.framerate {
            30 => "30",
            60 => "60",
            120 => "120",
            _ => "120",
        }
    }

    pub fn normalize_framerate(&mut self) {
        if !Self::ALLOWED_FRAMERATES.contains(&self.framerate) {
            self.framerate = 120;
        }
    }

    pub const fn audio_label(enabled: bool) -> &'static str {
        if enabled { "On" } else { "Off" }
    }
}

pub fn meta_game_settings_path() -> std::path::PathBuf {
    workspace_root_path_for_game()
        .join("data")
        .join("local_storage")
        .join("meta-game-settings.json")
}

pub fn create_meta_game_settings_store()
-> Result<Persistent<MetaGameSettingsModel>, PersistenceError> {
    Persistent::<MetaGameSettingsModel>::builder()
        .name("meta game settings")
        .format(StorageFormat::JsonPretty)
        .path(meta_game_settings_path())
        .default(MetaGameSettingsModel::default())
        .revertible(true)
        .revert_to_default_on_deserialization_errors(true)
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cpu_brain_click_stays_on_level_one() {
        let mut model = MetaGameSettingsModel::default();

        model.cycle_cpu_brain_level();

        assert_eq!(model.cpu_brain_level, CpuBrainLevelSettingModel::Level1);
        assert_eq!(model.cpu_brain_level.label(), "1");
    }

    #[test]
    fn framerate_cycled_and_normalized() {
        let mut model = MetaGameSettingsModel::default();

        model.toggle_framerate();
        assert_eq!(model.framerate, 30);

        model.toggle_framerate();
        assert_eq!(model.framerate, 60);

        model.toggle_framerate();
        assert_eq!(model.framerate, 120);

        model.framerate = 75;
        model.normalize_framerate();
        assert_eq!(model.framerate, 120);
    }

    #[test]
    fn mode_and_audio_settings_toggle() {
        let mut model = MetaGameSettingsModel::default();

        model.toggle_mode();
        model.toggle_sfx();
        model.toggle_music();

        assert_eq!(model.selected_mode, MatchModeModel::CpuVersusCpu);
        assert!(!model.sfx_enabled);
        assert!(!model.music_enabled);
    }
}
