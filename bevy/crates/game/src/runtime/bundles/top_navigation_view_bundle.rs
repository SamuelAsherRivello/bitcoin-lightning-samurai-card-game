use bevy::prelude::*;

use crate::runtime::components::TopNavigationRoot;

/// HUMAN: Root bundle for the reusable top navigation bar.
/// AI: Position this inside the aspect-ratio-safe content frame.
#[derive(Bundle, Debug)]
pub struct TopNavigationViewBundle {
    name: Name,
    root: TopNavigationRoot,
    node: Node,
}

impl Default for TopNavigationViewBundle {
    fn default() -> Self {
        Self {
            name: Name::new("TopNavigation"),
            root: TopNavigationRoot,
            node: Node {
                position_type: PositionType::Absolute,
                top: Val::Px(24.0),
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Px(48.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                column_gap: Val::Px(12.0),
                ..Default::default()
            },
        }
    }
}
