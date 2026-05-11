use bevy::prelude::*;

use crate::runtime::resources::{CardSlotSide, CostPointModel, PowerPointModel};

/// HUMAN: Marker and data for a rendered cost point view.
/// AI: CostPointView presents play-budget cost only; never use it for scoring totals.
#[derive(Clone, Copy, Component, Debug, Eq, PartialEq)]
pub struct CostPointView {
    pub model: CostPointModel,
}

impl CostPointView {
    pub const fn new(model: CostPointModel) -> Self {
        Self { model }
    }
}

/// HUMAN: Marker and data for a rendered power point view.
/// AI: PowerPointView presents card power or location totals from PowerPointModel.
#[derive(Clone, Copy, Component, Debug, Eq, PartialEq)]
pub struct PowerPointView {
    pub model: PowerPointModel,
}

impl PowerPointView {
    pub const fn new(model: PowerPointModel) -> Self {
        Self { model }
    }
}

/// HUMAN: Identifies a location-side power total rendered in GameView.
/// AI: Systems recalculate this from runtime slot/card data; the text is only presentation.
#[derive(Clone, Copy, Component, Debug, Eq, PartialEq)]
pub struct LocationPowerPointView {
    pub location_index: usize,
    pub side: CardSlotSide,
}

impl LocationPowerPointView {
    pub const fn new(location_index: usize, side: CardSlotSide) -> Self {
        Self {
            location_index,
            side,
        }
    }
}
