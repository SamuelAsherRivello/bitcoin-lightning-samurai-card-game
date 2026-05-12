use bevy::prelude::*;

pub const GAME_FONT_COUNT: usize = 4;
pub const GAME_BUTTON_FONT: GameFont = GameFont::Kamikaze;
pub const POINT_VIEW_FONT: GameFont = GameFont::BlastDragon;

/// HUMAN: Master list of font assets available to the game runtime.
/// AI: Keep font paths centralized so UI systems choose semantic font constants.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GameFont {
    BlastDragon,
    Kamikaze,
    Scarfire,
    TheLastShuriken,
}

impl GameFont {
    pub const fn all() -> [Self; GAME_FONT_COUNT] {
        [
            Self::BlastDragon,
            Self::Kamikaze,
            Self::Scarfire,
            Self::TheLastShuriken,
        ]
    }

    pub const fn asset_path(self) -> &'static str {
        match self {
            Self::BlastDragon => "fonts/blast-dragon/Blast Dragon D.otf",
            Self::Kamikaze => "fonts/kamikaze/Kamikaze.ttf",
            Self::Scarfire => "fonts/scarfire/Scarfire-Regular.otf",
            Self::TheLastShuriken => "fonts/the-last-shuriken/The Last Shuriken.otf",
        }
    }

    pub fn handle(self, asset_server: &AssetServer) -> Handle<Font> {
        asset_server.load(self.asset_path())
    }
}
