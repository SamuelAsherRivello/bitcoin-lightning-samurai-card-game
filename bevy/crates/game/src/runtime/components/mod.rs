use bevy::prelude::*;
use bevy::{
    core_pipeline::{core_3d::graph::Node3d, fullscreen_material::FullscreenMaterial},
    render::{
        extract_component::ExtractComponent,
        render_graph::{InternedRenderLabel, RenderLabel},
        render_resource::ShaderType,
    },
    shader::ShaderRef,
};

use crate::runtime::resources::CardFace;
use crate::runtime::resources::{CardSlotSide, MatchPlayerSide};
use crate::runtime::tweens::{LocationIntroTweenSample, sample_location_intro_tween};

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

const DECK_PROMPT_BACKDROP_BLUR_SHADER_PATH: &str = "shaders/deck_prompt_backdrop_blur.wgsl";

/// HUMAN: Player marker for the local game participant.
/// AI: Keep player state separate from card, scene, and view markers.
#[derive(Component, Debug, Default)]
pub struct Player;

/// HUMAN: Legacy marker retained only for tests that assert old camera removal.
/// AI: Do not add this to runtime entities; use AppSceneCamera for camera queries.
#[derive(Component, Debug, Default)]
pub struct PrimaryViewCamera;

/// HUMAN: Marker for the single shared 3D camera owned by AppScene.
/// AI: New camera-dependent systems should query this instead of view-owned cameras.
#[derive(Component, Debug, Default)]
pub struct AppSceneCamera;

/// HUMAN: Ordered render bands for content drawn through the shared AppScene camera.
/// AI: Use these semantic layers when replacing camera-order-based stacking.
#[derive(Clone, Copy, Component, Debug, Eq, PartialEq)]
pub enum SharedCameraRenderLayering {
    WorldBackground,
    LocationSurface,
    CardSurface,
    CardPointText,
    UiOverlay,
    TransitionFade,
}

/// HUMAN: Marker for overlay views that must follow the shared AppScene camera.
/// AI: Transition, modal, HUD, and debug overlays migrate through this marker over time.
#[derive(Component, Debug, Default)]
pub struct SharedCameraOverlayView;

/// HUMAN: Root marker for the always-present AppScene UI tree.
/// AI: AppScene owns persistent overlays and hosts one active sub-screen view.
#[derive(Component, Debug, Default)]
pub struct AppSceneRoot;

/// HUMAN: Marker for entities that belong to the always-present AppScene.
/// AI: Use only for persistent app-level entities, not child scene entities.
#[derive(Component, Debug, Default)]
pub struct AppSceneEntity;

/// HUMAN: Marker for the always-on fullscreen screen-transition overlay node.
/// AI: transition_update_system owns alpha updates and keeps this above active view content.
#[derive(Component, Debug, Default)]
pub struct ScreenTransitionOverlay;

/// HUMAN: Legacy marker retained only for tests that assert old camera removal.
/// AI: Do not add this to runtime entities; transition overlays target AppSceneCamera.
#[derive(Component, Debug, Default)]
pub struct ScreenTransitionCamera;

/// HUMAN: Fullscreen blur post-process settings for the DeckScreen prompt backdrop.
/// AI: Attach this to DeckScene 3D cameras only while prompt overlays are active.
#[derive(Component, ExtractComponent, Clone, Copy, ShaderType, Debug)]
pub struct DeckPromptBackdropBlur {
    pub texel_radius: f32,
}

impl Default for DeckPromptBackdropBlur {
    fn default() -> Self {
        Self { texel_radius: 1.35 }
    }
}

impl FullscreenMaterial for DeckPromptBackdropBlur {
    fn fragment_shader() -> ShaderRef {
        DECK_PROMPT_BACKDROP_BLUR_SHADER_PATH.into()
    }

    fn node_edges() -> Vec<InternedRenderLabel> {
        vec![
            Node3d::Tonemapping.intern(),
            Self::node_label().intern(),
            Node3d::EndMainPassPostProcessing.intern(),
        ]
    }
}

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

/// HUMAN: Shared identity for one GameScene location bundle presentation.
/// AI: Attach this to both 3D surface and safe-area overlay entities for sync.
#[derive(Component, Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocationBundle {
    pub location_index: usize,
}

impl LocationBundle {
    pub const fn new(location_index: usize) -> Self {
        Self { location_index }
    }
}

/// HUMAN: Marker for a location bundle's 3D textured surface.
/// AI: Surface opacity and depth stay separate from readable UI overlay children.
#[derive(Component, Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocationBundleSurface {
    pub location_index: usize,
}

impl LocationBundleSurface {
    pub const fn new(location_index: usize) -> Self {
        Self { location_index }
    }
}

/// HUMAN: Marker for the safe-area UI overlay of a location bundle.
/// AI: Overlay scale follows the same intro sample as the 3D surface.
#[derive(Component, Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocationBundleOverlay {
    pub location_index: usize,
}

impl LocationBundleOverlay {
    pub const fn new(location_index: usize) -> Self {
        Self { location_index }
    }
}

/// HUMAN: Intro timing state for one GameScene location bundle.
/// AI: Progress is derived from elapsed time plus per-location sequence delay.
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct LocationBundleIntro {
    pub location_index: usize,
    pub elapsed_seconds: f32,
}

impl LocationBundleIntro {
    pub const fn new(location_index: usize) -> Self {
        Self {
            location_index,
            elapsed_seconds: 0.0,
        }
    }

    pub fn sample(&self) -> LocationBundleIntroSample {
        LocationBundleIntroSample::at(self.location_index, self.elapsed_seconds)
    }
}

/// HUMAN: Child visual that fades with its parent location bundle intro.
/// AI: Use this for border/title/body/point overlays that need color alpha sync.
#[derive(Component, Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocationBundleIntroVisual {
    pub location_index: usize,
}

impl LocationBundleIntroVisual {
    pub const fn new(location_index: usize) -> Self {
        Self { location_index }
    }
}

/// HUMAN: Sampled visual state for the location bundle intro animation.
/// AI: Keeps scale and opacity calculations deterministic for surfaces and overlays.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LocationBundleIntroSample {
    pub scale: f32,
    pub opacity: f32,
}

impl LocationBundleIntroSample {
    pub fn at(location_index: usize, elapsed_seconds: f32) -> Self {
        sample_location_intro_tween(location_index, elapsed_seconds).into()
    }
}

impl From<LocationIntroTweenSample> for LocationBundleIntroSample {
    fn from(sample: LocationIntroTweenSample) -> Self {
        Self {
            scale: sample.scale,
            opacity: sample.opacity,
        }
    }
}

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

/// HUMAN: Marker for text nodes that show current SettingsScreen option values.
/// AI: This decouples dynamic labels from button interaction state and supports in-place refresh.
#[derive(Component, Clone, Copy, Debug, Eq, PartialEq)]
pub struct SettingsButtonLabel {
    pub action: MetaScreenButtonAction,
}

impl SettingsButtonLabel {
    pub const fn new(action: MetaScreenButtonAction) -> Self {
        Self { action }
    }
}

pub const WORLD_THEME_FADE_SECONDS: f32 = 0.125;
pub const WORLD_THEME_BLACK_HOLD_SECONDS: f32 = 0.125;

/// HUMAN: Marker for the GameScene world background image surface.
/// AI: Keep the textured world plane separate from its black transition overlay.
#[derive(Component, Debug, Default)]
pub struct WorldBackground;

/// HUMAN: Marker for the GameScene world background fade-to-black layer.
/// AI: Transition systems own this material alpha and leave gameplay UI unaffected.
#[derive(Component, Debug, Default)]
pub struct WorldFadeOverlay;

/// HUMAN: Ordered phase state for world-background theme fades.
/// AI: Gate theme texture swaps at full black so T-key cycling does not pop.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorldThemeTransitionPhase {
    StartupFadeIn,
    Idle,
    FadeOutToBlack,
    HoldAtBlack,
    FadeInFromBlack,
}

/// HUMAN: Runtime transition state attached to the active world background.
/// AI: Track applied and pending model indices locally so visual changes stay background-scoped.
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct WorldThemeTransition {
    pub phase: WorldThemeTransitionPhase,
    pub elapsed_seconds: f32,
    pub overlay_alpha: f32,
    pub applied_world_index: usize,
    pub pending_world_index: Option<usize>,
}

impl WorldThemeTransition {
    pub const fn startup(applied_world_index: usize) -> Self {
        Self {
            phase: WorldThemeTransitionPhase::StartupFadeIn,
            elapsed_seconds: 0.0,
            overlay_alpha: 1.0,
            applied_world_index,
            pending_world_index: None,
        }
    }

    pub fn request_world_index(&mut self, world_index: usize) {
        if world_index == self.applied_world_index && self.phase == WorldThemeTransitionPhase::Idle
        {
            return;
        }

        self.pending_world_index = Some(world_index);
        if self.phase == WorldThemeTransitionPhase::Idle {
            self.phase = WorldThemeTransitionPhase::FadeOutToBlack;
            self.elapsed_seconds = 0.0;
            self.overlay_alpha = 0.0;
        }
    }
}

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

/// HUMAN: Render-only tween state for any card root animation.
/// AI: Keeps shared card movement and reveal animation separate from gameplay slot state.
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct CardAnimation {
    pub phase_start_transform: Transform,
    pub target_transform: Transform,
    pub slot_transform: Transform,
    pub current_y_rotation: f32,
    pub target_y_rotation: f32,
    pub phase_elapsed_seconds: f32,
    pub start_delay_seconds: f32,
    pub phase: CardAnimationPhase,
    pub flip_style: CardAnimationFlipStyle,
    pub swan_peak_sfx_played: bool,
    pub swan_land_sfx_played: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CardAnimationPhase {
    MovingToHand,
    MovingToSlot,
    Revealing,
}

/// HUMAN: Reveal style selector for card front flips.
/// AI: Standard keeps rotation-only; Swan adds temporary scale bloom.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CardAnimationFlipStyle {
    Standard,
    Swan,
}

/// HUMAN: Runtime marker for a card root with an active animation owner.
/// AI: Sync systems use this lifecycle marker so animation commands can follow through.
#[derive(Component, Clone, Copy, Debug, Eq, PartialEq)]
pub struct CardAnimationMarker {
    pub phase: CardAnimationPhase,
    pub flip_style: CardAnimationFlipStyle,
}

impl From<CardAnimation> for CardAnimationMarker {
    fn from(animation: CardAnimation) -> Self {
        Self {
            phase: animation.phase,
            flip_style: animation.flip_style,
        }
    }
}

impl CardAnimation {
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
            phase: CardAnimationPhase::MovingToHand,
            flip_style: CardAnimationFlipStyle::Standard,
            swan_peak_sfx_played: false,
            swan_land_sfx_played: false,
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
            phase: CardAnimationPhase::MovingToSlot,
            flip_style: CardAnimationFlipStyle::Standard,
            swan_peak_sfx_played: false,
            swan_land_sfx_played: false,
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
            phase: CardAnimationPhase::Revealing,
            flip_style: CardAnimationFlipStyle::Standard,
            swan_peak_sfx_played: false,
            swan_land_sfx_played: false,
        }
    }

    pub fn swan_flip_to_front(slot_transform: Transform, start_delay_seconds: f32) -> Self {
        Self {
            phase_start_transform: slot_transform,
            target_transform: slot_transform,
            slot_transform,
            current_y_rotation: std::f32::consts::PI,
            target_y_rotation: 0.0,
            phase_elapsed_seconds: 0.0,
            start_delay_seconds,
            phase: CardAnimationPhase::Revealing,
            flip_style: CardAnimationFlipStyle::Swan,
            swan_peak_sfx_played: false,
            swan_land_sfx_played: false,
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

/// HUMAN: Marker for card face layers controlled by per-card placement animation.
/// AI: Excludes animated card layers from the global card flip visibility resource.
#[derive(Component, Debug, Default)]
pub struct CardAnimationFaceLayer;

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
