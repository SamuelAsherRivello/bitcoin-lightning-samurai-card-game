use bevy::prelude::*;

use crate::runtime::resources::{CostPointModel, PowerPointModel};

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
