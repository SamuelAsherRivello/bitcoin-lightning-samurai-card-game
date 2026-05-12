use bevy::prelude::*;

use crate::runtime::resources::CardSlotRect;

/// HUMAN: Bundle for a scalable area overlay anchored to a location area.
/// AI: Use it for shared location-area visualization driven by layout model rects.
#[derive(Bundle, Debug)]
pub struct LocationViewBundle {
    name: Name,
    node: Node,
    border: BorderColor,
    background: BackgroundColor,
    visibility: Visibility,
}

impl LocationViewBundle {
    pub const WIDTH_SCALE: f32 = 0.9;
    pub const HEIGHT_SCALE: f32 = 0.8;
    pub const BORDER_THICKNESS: f32 = 2.0;

    pub fn scaled_size(area: CardSlotRect) -> Vec2 {
        Vec2::new(
            area.width * Self::WIDTH_SCALE,
            area.height * Self::HEIGHT_SCALE,
        )
    }

    pub fn new(area: CardSlotRect) -> Self {
        let width = area.width * Self::WIDTH_SCALE;
        let height = area.height * Self::HEIGHT_SCALE;
        let left = area.left + (area.width - width) / 2.0;
        let top = area.top + (area.height - height) / 2.0;

        Self {
            name: Name::new("LocationViewBundle"),
            node: Node {
                position_type: PositionType::Absolute,
                left: Val::Px(left),
                top: Val::Px(top),
                width: Val::Px(width),
                height: Val::Px(height),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            border: BorderColor::all(Color::NONE),
            background: BackgroundColor(Color::NONE),
            visibility: Visibility::Visible,
        }
    }
}
