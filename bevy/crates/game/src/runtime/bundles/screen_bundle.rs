use bevy::prelude::*;

use crate::runtime::components::{
    DebugSceneEntity, DebugSceneRoot, DeckSceneEntity, DeckSceneRoot, GameSceneEntity,
    GameSceneRoot, MetaSceneEntity, MetaSceneRoot,
};

const SCREEN_WIDTH: f32 = 1280.0;
const SCREEN_HEIGHT: f32 = 800.0;

fn screen_node() -> Node {
    Node {
        position_type: PositionType::Absolute,
        width: Val::Px(SCREEN_WIDTH),
        height: Val::Px(SCREEN_HEIGHT),
        ..Default::default()
    }
}

fn full_node() -> Node {
    Node {
        position_type: PositionType::Absolute,
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        ..Default::default()
    }
}

fn meta_screen_background() -> BackgroundColor {
    BackgroundColor(Color::srgb(0.08, 0.09, 0.12))
}

macro_rules! meta_screen_bundle {
    ($bundle:ident, $screen_name:literal, $human:literal, $ai:literal) => {
        #[doc = concat!(" HUMAN: ", $human)]
        #[doc = concat!(" AI: ", $ai)]
        #[derive(Bundle, Debug)]
        pub struct $bundle {
            pub name: Name,
            pub root: MetaSceneRoot,
            pub scene_entity: MetaSceneEntity,
            pub node: Node,
            pub background: BackgroundColor,
        }

        impl Default for $bundle {
            fn default() -> Self {
                Self {
                    name: Name::new($screen_name),
                    root: MetaSceneRoot,
                    scene_entity: MetaSceneEntity,
                    node: screen_node(),
                    background: meta_screen_background(),
                }
            }
        }
    };
}

meta_screen_bundle!(
    MainMenuScreenBundle,
    "MainMenuScreen",
    "Root bundle for the main menu conceptual MainMenuScreen.",
    "MainMenuScreen is AppScene plus this meta UI child root."
);

meta_screen_bundle!(
    LightningScreenBundle,
    "LightningScreen",
    "Root bundle for the placeholder Lightning login conceptual screen.",
    "LightningScreen remains UI-only until real Lightning auth arrives."
);

meta_screen_bundle!(
    MatchmakingScreenBundle,
    "MatchmakingScreen",
    "Root bundle for the fake matchmaking conceptual screen.",
    "MatchmakingScreen owns only temporary search presentation state."
);

meta_screen_bundle!(
    SettingsScreenBundle,
    "SettingsScreen",
    "Root bundle for the pre-game settings conceptual screen.",
    "SettingsScreen is AppScene plus this meta UI child root."
);

/// HUMAN: Root bundle for the gameplay conceptual GameScreen.
/// AI: GameScreen is AppScene plus this GameScene child root.
#[derive(Bundle, Debug)]
pub struct GameScreenBundle {
    pub name: Name,
    pub root: GameSceneRoot,
    pub scene_entity: GameSceneEntity,
    pub node: Node,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

impl Default for GameScreenBundle {
    fn default() -> Self {
        Self {
            name: Name::new("GameScene"),
            root: GameSceneRoot,
            scene_entity: GameSceneEntity,
            node: full_node(),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
            visibility: Visibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
        }
    }
}

/// HUMAN: Root bundle for the deck conceptual DeckScreen.
/// AI: DeckScreen is AppScene plus this DeckScene child root.
#[derive(Bundle, Debug)]
pub struct DeckScreenBundle {
    pub name: Name,
    pub root: DeckSceneRoot,
    pub scene_entity: DeckSceneEntity,
    pub node: Node,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

impl Default for DeckScreenBundle {
    fn default() -> Self {
        Self {
            name: Name::new("DeckScreen"),
            root: DeckSceneRoot,
            scene_entity: DeckSceneEntity,
            node: full_node(),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
            visibility: Visibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
        }
    }
}

/// HUMAN: Root bundle for the debug conceptual DebugScreen.
/// AI: DebugScreen is AppScene plus this DebugScene child root.
#[derive(Bundle, Debug)]
pub struct DebugScreenBundle {
    pub name: Name,
    pub root: DebugSceneRoot,
    pub scene_entity: DebugSceneEntity,
    pub node: Node,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

impl Default for DebugScreenBundle {
    fn default() -> Self {
        Self {
            name: Name::new("DebugScene"),
            root: DebugSceneRoot,
            scene_entity: DebugSceneEntity,
            node: full_node(),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
            visibility: Visibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn screen_bundles_cover_all_current_conceptual_screens() {
        let names = [
            MainMenuScreenBundle::default().name.to_string(),
            LightningScreenBundle::default().name.to_string(),
            MatchmakingScreenBundle::default().name.to_string(),
            GameScreenBundle::default().name.to_string(),
            SettingsScreenBundle::default().name.to_string(),
            DeckScreenBundle::default().name.to_string(),
            DebugScreenBundle::default().name.to_string(),
        ];

        assert_eq!(
            names,
            [
                "MainMenuScreen",
                "LightningScreen",
                "MatchmakingScreen",
                "GameScene",
                "SettingsScreen",
                "DeckScreen",
                "DebugScene",
            ]
        );
    }

    #[test]
    fn meta_screen_bundles_use_safe_virtual_screen_size() {
        let main = MainMenuScreenBundle::default();

        assert_eq!(main.node.width, Val::Px(SCREEN_WIDTH));
        assert_eq!(main.node.height, Val::Px(SCREEN_HEIGHT));
    }
}
