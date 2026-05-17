use bevy::prelude::*;

/// HUMAN: Shared visual style for game UI buttons.
/// AI: Keep exactly one default style for this feature; add variants here later.
#[derive(Clone, Copy, Component, Debug, Default, Eq, PartialEq)]
pub enum ButtonUiStyle {
    #[default]
    Affirmative,
    Negative,
    Lightning,
}

/// HUMAN: Shared bundle used by all game UI buttons.
/// AI: Callers may override layout and colors while retaining the common button/style marker.
#[derive(Bundle, Debug)]
pub struct ButtonUiBundle {
    pub name: Name,
    pub button: Button,
    pub style: ButtonUiStyle,
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
}

impl ButtonUiBundle {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: Name::new(name.into()),
            button: Button,
            style: ButtonUiStyle::Affirmative,
            node: Node {
                width: Val::Px(120.0),
                height: Val::Px(44.0),
                border: UiRect::all(Val::Px(2.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: BackgroundColor(Color::srgb(0.20, 0.24, 0.32)),
            border_color: BorderColor::all(Color::srgb(0.60, 0.64, 0.72)),
        }
    }

    pub fn with_node(mut self, node: Node) -> Self {
        self.node = node;
        self
    }

    pub fn with_style(mut self, style: ButtonUiStyle) -> Self {
        self.style = style;
        self
    }

    pub fn with_colors(mut self, background: Color, border: Color) -> Self {
        self.background_color = BackgroundColor(background);
        self.border_color = BorderColor::all(border);
        self
    }

    pub fn affirmative_button_style(mut self) -> Self {
        self.style = ButtonUiStyle::Affirmative;
        self.with_colors(Color::srgb(0.20, 0.24, 0.32), Color::srgb(0.60, 0.64, 0.72))
    }

    pub fn negative_button_style(mut self) -> Self {
        self.style = ButtonUiStyle::Negative;
        self.with_colors(Color::srgb(0.44, 0.12, 0.14), Color::srgb(0.88, 0.36, 0.38))
    }

    pub fn lightning_button_style(mut self) -> Self {
        self.style = ButtonUiStyle::Lightning;
        self.with_colors(Color::srgb(0.86, 0.63, 0.18), Color::srgb(1.0, 0.82, 0.32))
    }
}
