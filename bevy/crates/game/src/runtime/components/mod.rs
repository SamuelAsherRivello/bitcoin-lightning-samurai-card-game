use bevy::prelude::*;

use crate::runtime::resources::CardFace;
use crate::runtime::resources::{CardSlotSide, MatchPlayerSide};

pub mod card_gesture_component;
pub mod card_selection_component;
pub mod card_ui_component;
pub mod debug_drawing_component;
pub mod deck_screen_component;
pub mod game_control_component;
pub mod point_view_visual_modifier_component;
pub mod top_navigation_component;

pub use card_gesture_component::*;
pub use card_selection_component::*;
pub use card_ui_component::*;
pub use debug_drawing_component::*;
pub use deck_screen_component::*;
pub use game_control_component::*;
pub use point_view_visual_modifier_component::*;
pub use top_navigation_component::*;

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

/// HUMAN: Root marker for the deck sub-screen view.
/// AI: DeckScene is loaded on top of AppScene and may be despawned/reloaded.
#[derive(Component, Debug, Default)]
pub struct DeckSceneRoot;

/// HUMAN: Marker for entities owned by DeckScene.
/// AI: Keep deck rendering/query filters on this marker.
#[derive(Component, Debug, Default)]
pub struct DeckSceneEntity;

/// HUMAN: Root marker for the debug sub-screen scene.
/// AI: DebugScene is a duplicated DeckScene-style scene for diagnostics.
#[derive(Component, Debug, Default)]
pub struct DebugSceneRoot;

/// HUMAN: Marker for entities owned by DebugScene.
/// AI: Keep debug rendering/query filters on this marker.
#[derive(Component, Debug, Default)]
pub struct DebugSceneEntity;

/// HUMAN: Root marker for the gameplay sub-screen view.
/// AI: GameScene is loaded on top of AppScene and may be despawned/reloaded.
#[derive(Component, Debug, Default)]
pub struct GameSceneRoot;

/// HUMAN: Marker for entities owned by GameScene.
/// AI: Keep gameplay rendering/query filters on this marker.
#[derive(Component, Debug, Default)]
pub struct GameSceneEntity;

/// HUMAN: Root marker for lightweight meta-game UI screens.
/// AI: Main, LightningLogin, Matchmaking, and Settings share this UI-only view root.
#[derive(Component, Debug, Default)]
pub struct MetaSceneRoot;

/// HUMAN: Marker for entities owned by a meta-game UI screen.
/// AI: Use this for despawn/reload of non-gameplay screens.
#[derive(Component, Debug, Default)]
pub struct MetaSceneEntity;

/// HUMAN: Click action for buttons owned by meta-game screens.
/// AI: Route all non-top-nav meta buttons through this enum.
#[derive(Clone, Copy, Component, Debug, Eq, PartialEq)]
pub enum MetaScreenButtonAction {
    LightningLogin,
    LearnLightning,
    StartGame,
    MatchmakingBack,
    CpuBrain,
    MatchMode,
    ToggleSfx,
    ToggleMusic,
    CycleFramerate,
    CycleQuality,
}

/// HUMAN: Marker for clickable meta-game screen buttons.
/// AI: Keep screen action routing independent from rendered text.
#[derive(Component, Debug)]
pub struct MetaScreenButton {
    pub action: MetaScreenButtonAction,
}

impl MetaScreenButton {
    pub const fn new(action: MetaScreenButtonAction) -> Self {
        Self { action }
    }
}

#[derive(Component, Debug, Default)]
pub struct WorldBackground;

#[derive(Component, Debug, Default)]
pub struct LocalPlayerHand;

#[derive(Component, Debug, Default)]
pub struct LocalPlayerHandCardPreview;

#[derive(Component, Debug, Default)]
pub struct RoundUi;

#[derive(Component, Debug, Default)]
pub struct EndRoundButton;

/// HUMAN: Status text for the active two-player match result.
/// AI: Keep final winner presentation separate from the mode button label.
#[derive(Component, Debug, Default)]
pub struct MatchStatusText;

/// HUMAN: Render marker for a passive CPU-controlled hand card.
/// AI: CPU hand cards show round setup only and never receive gesture/cursor markers.
#[derive(Component, Clone, Debug, Eq, PartialEq)]
pub struct CpuHandCardView {
    pub owner: MatchPlayerSide,
    pub instance_id: u64,
    pub hand_index: usize,
    pub card_id: String,
    pub visible_face: CardFace,
}

impl CpuHandCardView {
    pub fn new(
        owner: MatchPlayerSide,
        instance_id: u64,
        hand_index: usize,
        card_id: impl Into<String>,
        visible_face: CardFace,
    ) -> Self {
        Self {
            owner,
            instance_id,
            hand_index,
            card_id: card_id.into(),
            visible_face,
        }
    }
}

/// HUMAN: Render marker for a card placed by a CPU-controlled player.
/// AI: CPU-owned cards are passive and never receive gesture/cursor rotation markers.
#[derive(Component, Clone, Debug, Eq, PartialEq)]
pub struct CpuPlacedCardView {
    pub owner: MatchPlayerSide,
    pub side: CardSlotSide,
    pub location_index: usize,
    pub slot_index: usize,
    pub card_id: String,
    pub visible_face: CardFace,
}

/// HUMAN: Render-only tween state for a CPU-controlled placed card.
/// AI: Keeps CPU move and reveal animation separate from gameplay slot state.
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct CpuPlacedCardAnimation {
    pub phase_start_transform: Transform,
    pub target_transform: Transform,
    pub slot_transform: Transform,
    pub current_y_rotation: f32,
    pub target_y_rotation: f32,
    pub phase_elapsed_seconds: f32,
    pub start_delay_seconds: f32,
    pub phase: CpuPlacedCardAnimationPhase,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CpuPlacedCardAnimationPhase {
    MovingToHand,
    MovingToSlot,
    Revealing,
}

impl CpuPlacedCardAnimation {
    pub fn move_to_hand(
        source_transform: Transform,
        hand_transform: Transform,
        visible_face: CardFace,
    ) -> Self {
        let y_rotation = match visible_face {
            CardFace::Front => 0.0,
            CardFace::Back => std::f32::consts::PI,
        };
        Self {
            phase_start_transform: source_transform,
            target_transform: hand_transform,
            slot_transform: hand_transform,
            current_y_rotation: y_rotation,
            target_y_rotation: y_rotation,
            phase_elapsed_seconds: 0.0,
            start_delay_seconds: 0.0,
            phase: CpuPlacedCardAnimationPhase::MovingToHand,
        }
    }

    pub fn move_hand_to_slot(
        hand_transform: Transform,
        slot_transform: Transform,
        visible_face: CardFace,
    ) -> Self {
        let y_rotation = match visible_face {
            CardFace::Front => 0.0,
            CardFace::Back => std::f32::consts::PI,
        };
        Self {
            phase_start_transform: hand_transform,
            target_transform: slot_transform,
            slot_transform,
            current_y_rotation: y_rotation,
            target_y_rotation: y_rotation,
            phase_elapsed_seconds: 0.0,
            start_delay_seconds: 0.0,
            phase: CpuPlacedCardAnimationPhase::MovingToSlot,
        }
    }

    pub fn move_deck_to_hand_to_slot(
        source_transform: Transform,
        hand_transform: Transform,
        slot_transform: Transform,
        visible_face: CardFace,
    ) -> Self {
        let mut animation = Self::move_to_hand(source_transform, hand_transform, visible_face);
        animation.slot_transform = slot_transform;
        animation
    }

    pub fn flip_to_front(slot_transform: Transform, start_delay_seconds: f32) -> Self {
        Self {
            phase_start_transform: slot_transform,
            target_transform: slot_transform,
            slot_transform,
            current_y_rotation: std::f32::consts::PI,
            target_y_rotation: 0.0,
            phase_elapsed_seconds: 0.0,
            start_delay_seconds,
            phase: CpuPlacedCardAnimationPhase::Revealing,
        }
    }

    pub fn current_face(self) -> CardFace {
        if self.current_y_rotation.cos() >= 0.0 {
            CardFace::Front
        } else {
            CardFace::Back
        }
    }
}

/// HUMAN: Marker for card face layers controlled by CPU placement animation.
/// AI: Excludes CPU layers from the global card flip visibility resource.
#[derive(Component, Debug, Default)]
pub struct CpuPlacedCardFaceLayer;

impl CpuPlacedCardView {
    pub fn new(
        owner: MatchPlayerSide,
        side: CardSlotSide,
        location_index: usize,
        slot_index: usize,
        card_id: impl Into<String>,
        visible_face: CardFace,
    ) -> Self {
        Self {
            owner,
            side,
            location_index,
            slot_index,
            card_id: card_id.into(),
            visible_face,
        }
    }
}

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

/// HUMAN: Centered title text for a GameScene location.
/// AI: Sync this from GameLocationModel when round reveal state changes.
#[derive(Component, Debug)]
pub struct GameLocationTitleText {
    pub location_index: usize,
}

impl GameLocationTitleText {
    pub const fn new(location_index: usize) -> Self {
        Self { location_index }
    }
}

/// HUMAN: Centered ability body text for a GameScene location.
/// AI: Keep the entity alive while closed so later rounds can reveal text in place.
#[derive(Component, Debug)]
pub struct GameLocationBodyText {
    pub location_index: usize,
}

impl GameLocationBodyText {
    pub const fn new(location_index: usize) -> Self {
        Self { location_index }
    }
}

/// HUMAN: Border view for a GameScene location card.
/// AI: Sync color from open/closed state without respawning the location.
#[derive(Component, Debug)]
pub struct GameLocationBorder {
    pub location_index: usize,
}

impl GameLocationBorder {
    pub const fn new(location_index: usize) -> Self {
        Self { location_index }
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
