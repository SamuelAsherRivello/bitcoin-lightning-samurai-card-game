use bevy::prelude::*;

use crate::runtime::resources::GameTicks;
use crate::runtime::systems::{advance_ticks, setup_game};

pub struct CoreGamePlugin;

impl Plugin for CoreGamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameTicks>()
            .add_systems(Startup, setup_game)
            .add_systems(Update, advance_ticks);
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;

    use super::CoreGamePlugin;
    use crate::runtime::components::Player;
    use crate::runtime::resources::GameTicks;

    #[test]
    fn plugin_spawns_player_and_advances_ticks() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins).add_plugins(CoreGamePlugin);

        app.update();

        let mut player_query = app.world_mut().query::<&Player>();
        let player_count = player_query.iter(app.world()).count();
        let ticks = app.world().resource::<GameTicks>().0;

        assert_eq!(player_count, 1);
        assert_eq!(ticks, 1);
    }
}
