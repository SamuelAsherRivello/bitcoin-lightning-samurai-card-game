use bevy::prelude::*;

pub mod runtime;

use runtime::plugins::CoreGamePlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CoreGamePlugin);
    }
}
