use bevy::prelude::*;

use crate::runtime::resources::CardSlotSide;
use crate::runtime::resources::{CostPointModel, PowerPointModel};

/// HUMAN: Shared semantic type for point badges rendered in GameView and deck cards.
/// AI: Every badge value and color decision now routes through this type family.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PointType {
    LocationPower,
    CardPower,
    CardEnergy,
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
}

impl PointViewBundle {
    pub fn new(name: impl Into<String>, model: PointModel) -> Self {
        Self {
            name: Name::new(name.into()),
            view: PointView::new(model),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::resources::{CostPointModel, PowerPointModel};

    #[test]
    fn point_model_constructs_expected_type_and_payload() {
        let location_power = PointModel::location_power(7);
        let card_power = PointModel::card_power(-8);
        let card_energy = PointModel::card_energy(99);

        assert_eq!(location_power.point_type, PointType::LocationPower);
        assert_eq!(location_power.value, 7);
        assert_eq!(card_power.point_type, PointType::CardPower);
        assert_eq!(card_power.value, -8);
        assert_eq!(card_energy.point_type, PointType::CardEnergy);
        assert_eq!(card_energy.value, 99);
    }

    #[test]
    fn point_model_formats_numeric_display_with_negatives() {
        let point = PointModel::new(PointType::CardPower, -99);

        assert_eq!(point.display_text(), "-99");
    }

    #[test]
    fn point_model_from_cost_and_power_inputs_preserves_type_and_value() {
        let location_from_power = PointModel::from_power_point(
            PointType::LocationPower,
            PowerPointModel::new(-42),
        );
        let card_energy = PointModel::from_cost_point(CostPointModel::new(42));
        let card_power = PointModel::from_power_point(PointType::CardPower, PowerPointModel::new(11));

        assert_eq!(location_from_power, PointModel::new(PointType::LocationPower, -42));
        assert_eq!(card_energy, PointModel::new(PointType::CardEnergy, 42));
        assert_eq!(card_power, PointModel::new(PointType::CardPower, 11));
    }

    #[test]
    fn point_view_bundle_contains_name_and_view_payload() {
        let model = PointModel::card_power(4);
        let bundle = PointViewBundle::new("Test Point Bundle", model);

        assert_eq!(bundle.name.as_str(), "Test Point Bundle");
        assert_eq!(bundle.view, PointView::new(model));
    }
}
