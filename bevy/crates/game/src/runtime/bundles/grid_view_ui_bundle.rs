use bevy::prelude::*;

use crate::runtime::components::CardGrid;
use crate::runtime::resources::DeckEditableZoneModel;

/// HUMAN: 2D UI bundle for a titled deck-editor style panel.
/// AI: GridViewUiBundle owns the shared frame/title geometry while content stays in systems.
#[derive(Bundle, Debug)]
pub struct GridViewUiBundle {
    pub name: Name,
    pub card_grid: CardGrid,
    pub node: Node,
    pub pickable: Pickable,
}

impl GridViewUiBundle {
    pub fn new(
        title: impl Into<String>,
        zone: DeckEditableZoneModel,
        left: f32,
        top: f32,
        width: f32,
        row_gap: Val,
    ) -> Self {
        let title = title.into();
        Self {
            name: Name::new(format!("DeckScreen {title} Panel")),
            card_grid: CardGrid::new(title, zone),
            node: Node {
                position_type: PositionType::Absolute,
                left: Val::Px(left),
                top: Val::Px(top),
                width: Val::Px(width),
                flex_direction: FlexDirection::Column,
                row_gap,
                ..Default::default()
            },
            pickable: Pickable::IGNORE,
        }
    }
}
