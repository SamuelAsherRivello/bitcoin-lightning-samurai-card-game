use bevy::prelude::*;

use crate::runtime::components::{
    DebugSceneEntity, DebugSceneRoot, DeckSceneEntity, DeckSceneRoot, GameSceneEntity,
    GameSceneRoot,
};

/// HUMAN: Root bundle for the gameplay conceptual GameScreen.
/// AI: GameScreen is AppScene plus this GameScene child root.
#[derive(Bundle, Debug)]
pub struct GameScreenBundle {
    pub name: Name,
    pub root: GameSceneRoot,
    pub scene_entity: GameSceneEntity,
}

impl Default for GameScreenBundle {
    fn default() -> Self {
        Self {
            name: Name::new("GameScreen"),
            root: GameSceneRoot,
            scene_entity: GameSceneEntity,
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
}

impl Default for DeckScreenBundle {
    fn default() -> Self {
        Self {
            name: Name::new("DeckScreen"),
            root: DeckSceneRoot,
            scene_entity: DeckSceneEntity,
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
}

impl Default for DebugScreenBundle {
    fn default() -> Self {
        Self {
            name: Name::new("DebugScreen"),
            root: DebugSceneRoot,
            scene_entity: DebugSceneEntity,
        }
    }
}
