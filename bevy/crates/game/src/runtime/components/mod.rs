use bevy::prelude::*;

use crate::runtime::resources::CardFace;

pub mod card_gesture_component;
pub mod card_ui_component;
pub mod debug_drawing_component;
pub mod point_view_component;

pub use card_gesture_component::*;
pub use card_ui_component::*;
pub use debug_drawing_component::*;
pub use point_view_component::*;

/// HUMAN: Player marker for the local game participant.
/// AI: Keep player state separate from card, scene, and view markers.
#[derive(Component, Debug, Default)]
pub struct Player;

/// HUMAN: Primary camera marker for view-owned cameras.
/// AI: This is view scoped; avoid Scene wording except for AppScene.
#[derive(Component, Debug, Default)]
pub struct PrimaryViewCamera;

/// HUMAN: Root marker for the always-present AppScene UI tree.
/// AI: AppScene owns persistent overlays and hosts one active sub-screen view.
#[derive(Component, Debug, Default)]
pub struct AppSceneRoot;

/// HUMAN: Marker for entities that belong to the always-present AppScene.
/// AI: Use only for persistent app-level entities, not child scene entities.
#[derive(Component, Debug, Default)]
pub struct AppSceneEntity;

/// HUMAN: Root marker for the deck builder sub-screen view.
/// AI: DeckBuilderScene is loaded on top of AppScene and may be despawned/reloaded.
#[derive(Component, Debug, Default)]
pub struct DeckBuilderSceneRoot;

/// HUMAN: Marker for entities owned by DeckBuilderScene.
/// AI: Keep deck builder rendering/query filters on this marker.
#[derive(Component, Debug, Default)]
pub struct DeckBuilderSceneEntity;

/// HUMAN: Root marker for the debug settings sub-screen scene.
/// AI: DebugSettingsScene is a duplicated DeckBuilderScene-style scene for diagnostics.
#[derive(Component, Debug, Default)]
pub struct DebugSettingsSceneRoot;

/// HUMAN: Marker for entities owned by DebugSettingsScene.
/// AI: Keep debug settings rendering/query filters on this marker.
#[derive(Component, Debug, Default)]
pub struct DebugSettingsSceneEntity;

/// HUMAN: Root marker for the gameplay sub-screen view.
/// AI: GameView is loaded on top of AppScene and may be despawned/reloaded.
#[derive(Component, Debug, Default)]
pub struct GameViewRoot;

/// HUMAN: Marker for entities owned by GameView.
/// AI: Keep gameplay rendering/query filters on this marker.
#[derive(Component, Debug, Default)]
pub struct GameViewEntity;

#[derive(Component, Debug, Default)]
pub struct WorldBackground;

#[derive(Component, Debug, Default)]
pub struct LocalPlayerHand;

#[derive(Component, Debug, Default)]
pub struct LocalPlayerHandCardPreview;

#[derive(Component, Debug, Default)]
pub struct TurnUi;

#[derive(Component, Debug, Default)]
pub struct EndTurnButton;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LocationRevealState {
    Unrevealed,
    Revealed,
}

#[derive(Component, Debug)]
pub struct GameLocation {
    pub index: usize,
    pub reveal_state: LocationRevealState,
}

impl GameLocation {
    pub const fn new(index: usize, reveal_state: LocationRevealState) -> Self {
        Self {
            index,
            reveal_state,
        }
    }
}

/// HUMAN: Root marker for a rendered card view.
/// AI: Pair with CardViewBundle when spawning the root visual entity.
#[derive(Component, Debug, Default)]
pub struct CardView;

/// HUMAN: Marker for a visible face layer on a CardView.
/// AI: Used by flip visibility systems; keep face semantics here, not in CardModel.
#[derive(Component, Debug)]
pub struct CardFaceLayer {
    pub face: CardFace,
}

impl CardFaceLayer {
    pub const fn new(face: CardFace) -> Self {
        Self { face }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CardLayerRole {
    Background,
    Frame,
    SafeArea,
    Foreground,
    Title,
}

#[derive(Component, Debug)]
pub struct CardParallaxLayer {
    pub role: CardLayerRole,
    pub apparent_depth: f32,
    pub neutral_translation: Vec3,
}

impl CardParallaxLayer {
    pub const fn new(role: CardLayerRole, apparent_depth: f32, neutral_translation: Vec3) -> Self {
        Self {
            role,
            apparent_depth,
            neutral_translation,
        }
    }
}

#[derive(Component, Debug)]
pub struct CardBackgroundLayer {
    pub uses_frame_mask: bool,
}

impl CardBackgroundLayer {
    pub const fn new(uses_frame_mask: bool) -> Self {
        Self { uses_frame_mask }
    }
}

#[derive(Component, Debug)]
pub struct CardFrameLayer;

#[derive(Component, Debug)]
pub struct DebugHudText;

#[derive(Component, Debug)]
pub struct DebugHudFpsText;

#[derive(Component, Clone, Copy, Debug)]
pub struct DebugHudKeyText {
    pub key_code: KeyCode,
    pub is_toggle: bool,
}

impl DebugHudKeyText {
    pub const fn new(key_code: KeyCode, is_toggle: bool) -> Self {
        Self {
            key_code,
            is_toggle,
        }
    }
}

#[derive(Component, Debug)]
pub struct InspectorState {
    pub is_visible: bool,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Default for InspectorState {
    fn default() -> Self {
        Self {
            is_visible: false,
            x: 24.0,
            y: 132.0,
            width: 338.0,
            height: 310.0,
        }
    }
}
