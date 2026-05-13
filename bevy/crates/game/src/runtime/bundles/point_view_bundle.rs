use bevy::prelude::*;

use crate::runtime::components::PointViewVisualModifiers;
use crate::runtime::resources::{CardSlotSide, GameFont, POINT_VIEW_FONT};
use crate::runtime::resources::{CostPointModel, PowerPointModel};

pub const POINT_VIEW_BASE_TEXT_FONT_SIZE: f32 = 168.0;
pub const POINT_VIEW_BUNDLE_FONT: GameFont = POINT_VIEW_FONT;

/// HUMAN: Shared semantic type for point badges rendered in GameView and deck cards.
/// AI: Every badge value and color decision now routes through this type family.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PointType {
    LocationPower,
    CardPower,
    CardEnergy,
}

impl PointType {
    pub const fn is_energy(self) -> bool {
        matches!(self, Self::CardEnergy)
    }

    pub fn background_color(self) -> Color {
        if self.is_energy() {
            Color::srgb(0.04, 0.18, 0.60)
        } else {
            Color::srgb(0.74, 0.18, 0.18)
        }
    }

    pub fn text_color(self) -> Color {
        Color::WHITE
    }
}

/// HUMAN: Point payload that unifies cost, power, and total-point badge values.
/// AI: Keep display values normalized as a single -99..99 integer contract for rendering.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PointModel {
    pub point_type: PointType,
    pub value: i32,
}

impl PointModel {
    pub const fn new(kind: PointType, value: i32) -> Self {
        Self {
            point_type: kind,
            value,
        }
    }

    pub const fn location_power(value: i32) -> Self {
        Self::new(PointType::LocationPower, value)
    }

    pub const fn card_power(value: i32) -> Self {
        Self::new(PointType::CardPower, value)
    }

    pub const fn card_energy(value: i32) -> Self {
        Self::new(PointType::CardEnergy, value)
    }

    pub const fn from_power_point(point_type: PointType, model: PowerPointModel) -> Self {
        Self::new(point_type, model.value)
    }

    pub const fn from_cost_point(model: CostPointModel) -> Self {
        Self::new(PointType::CardEnergy, model.value)
    }

    pub fn display_text(self) -> String {
        self.value.to_string()
    }

    pub fn background_color(self) -> Color {
        self.point_type.background_color()
    }

    pub fn text_color(self) -> Color {
        self.point_type.text_color()
    }
}

/// HUMAN: ECS component used to render a badge value and type in the runtime.
/// AI: Keeps rendering query/update paths type-aware with one canonical data payload.
#[derive(Clone, Copy, Component, Debug, Eq, PartialEq)]
pub struct PointView {
    pub model: PointModel,
}

impl PointView {
    pub const fn new(model: PointModel) -> Self {
        Self { model }
    }
}

/// HUMAN: Identifies a card-side location badge for total-updating behavior.
/// AI: The location side and index are UI semantics and stay separate from value logic.
#[derive(Clone, Copy, Component, Debug, Eq, PartialEq)]
pub struct PointLocationView {
    pub location_index: usize,
    pub side: CardSlotSide,
}

impl PointLocationView {
    pub const fn new(location_index: usize, side: CardSlotSide) -> Self {
        Self {
            location_index,
            side,
        }
    }
}

/// HUMAN: Base bundle for every point badge root entity.
/// AI: Use this for a consistent point-model-bearing point root.
#[derive(Bundle, Debug)]
pub struct PointViewBundle {
    pub name: Name,
    pub view: PointView,
    pub visual_modifiers: PointViewVisualModifiers,
}

impl PointViewBundle {
    pub fn new(name: impl Into<String>, model: PointModel) -> Self {
        Self {
            name: Name::new(name.into()),
            view: PointView::new(model),
            visual_modifiers: PointViewVisualModifiers::default(),
        }
    }
}

#[cfg(test)]
#[path = "../../tests/runtime/bundles/point_view_bundle_tests.rs"]
mod point_view_bundle_tests;
