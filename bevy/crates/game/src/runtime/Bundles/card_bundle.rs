use bevy::prelude::*;

use crate::runtime::components::CardView;
use crate::runtime::resources::CardModel;

/// HUMAN: Bundle for the root visual entity of a rendered card.
/// AI: This creates CardView roots from CardModel data; child layers are spawned by card view systems.
#[derive(Bundle, Debug)]
pub struct CardViewBundle {
    name: Name,
    card_view: CardView,
    transform: Transform,
    global_transform: GlobalTransform,
    visibility: Visibility,
}

impl CardViewBundle {
    pub fn new(card_model: &CardModel, transform: Transform) -> Self {
        Self {
            name: Name::new(format!("CardView {}", card_model.display_name)),
            card_view: CardView,
            transform,
            global_transform: GlobalTransform::default(),
            visibility: Visibility::Visible,
        }
    }
}
