use bevy::prelude::*;

use crate::runtime::components::TopNavigationRoot;

const TOP_NAVIGATION_FRAME_WIDTH: f32 = 1280.0;
const TOP_NAVIGATION_TOP: f32 = 24.0;
const TOP_NAVIGATION_HEIGHT: f32 = 48.0;

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
                top: Val::Px(TOP_NAVIGATION_TOP),
                left: Val::Px(0.0),
                width: Val::Px(TOP_NAVIGATION_FRAME_WIDTH),
                height: Val::Px(TOP_NAVIGATION_HEIGHT),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                column_gap: Val::Px(12.0),
                ..Default::default()
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn top_navigation_uses_fixed_safe_area_frame() {
        let bundle = TopNavigationViewBundle::default();

        assert_eq!(bundle.node.position_type, PositionType::Absolute);
        assert_eq!(bundle.node.top, Val::Px(TOP_NAVIGATION_TOP));
        assert_eq!(bundle.node.left, Val::Px(0.0));
        assert_eq!(bundle.node.width, Val::Px(TOP_NAVIGATION_FRAME_WIDTH));
        assert_eq!(bundle.node.height, Val::Px(TOP_NAVIGATION_HEIGHT));
        assert_eq!(bundle.node.justify_content, JustifyContent::Center);
    }
}
