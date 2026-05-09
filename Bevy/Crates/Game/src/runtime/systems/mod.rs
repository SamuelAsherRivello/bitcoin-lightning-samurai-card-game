use bevy::prelude::*;
use bevy_card_game_shared::GameTitle;

use crate::runtime::components::Player;
use crate::runtime::resources::GameTicks;

pub fn setup_game(mut commands: Commands) {
    commands.spawn((Player, Name::new(GameTitle::DISPLAY)));
}

pub fn advance_ticks(mut ticks: ResMut<GameTicks>) {
    ticks.0 += 1;
}
