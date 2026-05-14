use bevy::prelude::*;

use crate::runtime::components::DeckView;

/// HUMAN: Visual bundle for a deck tile.
/// AI: Children should render the existing card back image and the deck name.
#[derive(Bundle, Debug)]
pub struct DeckViewBundle {
    name: Name,
    deck_view: DeckView,
    button: Button,
    node: Node,
    background: BackgroundColor,
    border: BorderColor,
}

impl DeckViewBundle {
    pub fn new(deck_name: impl Into<String>) -> Self {
        let deck_name = deck_name.into();
        Self {
            name: Name::new(format!("DeckView {deck_name}")),
            deck_view: DeckView::new(deck_name),
            button: Button,
            node: Node {
                width: Val::Px(132.0),
                height: Val::Px(214.0),
                border: UiRect::all(Val::Px(2.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(8.0)),
                ..Default::default()
            },
            background: BackgroundColor(Color::srgba(0.08, 0.11, 0.16, 0.94)),
            border: BorderColor::all(Color::srgb(0.78, 0.84, 0.92)),
        }
    }
}
