use bevy::prelude::*;

pub const MODAL_UI_BACKDROP_OPACITY: f32 = 0.9;

/// HUMAN: Full-screen modal overlay that blocks the rest of the app visually.
/// AI: ModalUiBundle owns the centered layout root and 90% black backdrop.
#[derive(Bundle, Debug)]
pub struct ModalUiBundle {
    pub name: Name,
    pub node: Node,
    pub background_color: BackgroundColor,
    pub ui_target_camera: UiTargetCamera,
    pub global_z_index: GlobalZIndex,
}

impl ModalUiBundle {
    pub fn new(name: impl Into<String>, ui_camera: Entity) -> Self {
        Self {
            name: Name::new(name.into()),
            node: Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                top: Val::Px(0.0),
                bottom: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: BackgroundColor(Color::srgba(
                0.0,
                0.0,
                0.0,
                MODAL_UI_BACKDROP_OPACITY,
            )),
            ui_target_camera: UiTargetCamera(ui_camera),
            global_z_index: GlobalZIndex(900),
        }
    }
}

/// HUMAN: Centered modal prompt panel inside a modal overlay.
/// AI: Title, body, and menu children are spawned under this fixed-width panel.
#[derive(Bundle, Debug)]
pub struct ModalPromptUiBundle {
    pub name: Name,
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
}

impl ModalPromptUiBundle {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: Name::new(name.into()),
            node: Node {
                width: Val::Px(420.0),
                border: UiRect::all(Val::Px(2.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(18.0),
                padding: UiRect::all(Val::Px(24.0)),
                ..Default::default()
            },
            background_color: BackgroundColor(Color::srgb(0.06, 0.07, 0.09)),
            border_color: BorderColor::all(Color::srgb(0.42, 0.46, 0.54)),
        }
    }
}

/// HUMAN: Modal footer area that holds zero, one, or two command buttons.
/// AI: Systems can spawn no children or up to two buttons without changing prompt layout.
#[derive(Bundle, Debug)]
pub struct ModalMenuUiBundle {
    pub name: Name,
    pub node: Node,
}

impl ModalMenuUiBundle {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: Name::new(name.into()),
            node: Node {
                width: Val::Percent(100.0),
                height: Val::Px(48.0),
                column_gap: Val::Px(12.0),
                justify_content: JustifyContent::FlexEnd,
                align_items: AlignItems::Center,
                ..Default::default()
            },
        }
    }
}

/// HUMAN: Standard modal command button.
/// AI: Marker components are inserted by the caller so this bundle stays reusable.
#[derive(Bundle, Debug)]
pub struct ModalButtonUiBundle {
    pub name: Name,
    pub button: Button,
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
}

impl ModalButtonUiBundle {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: Name::new(name.into()),
            button: Button,
            node: Node {
                width: Val::Px(120.0),
                height: Val::Px(44.0),
                border: UiRect::all(Val::Px(2.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: BackgroundColor(Color::srgb(0.24, 0.28, 0.36)),
            border_color: BorderColor::all(Color::srgb(0.6, 0.65, 0.74)),
        }
    }
}
