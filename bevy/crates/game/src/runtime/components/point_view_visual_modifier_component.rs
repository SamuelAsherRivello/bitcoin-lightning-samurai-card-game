use bevy::prelude::*;

use crate::runtime::resources::CardInstanceId;

pub const VISUAL_MODIFIER_OUTLINE_WIDTH_PX: f32 = 3.0;
pub const VISUAL_MODIFIER_CARD_OUTLINE_SCALE: f32 = 1.22;
pub const INITIAL_VISUAL_MODIFICATION_RULES: [VisualModificationRule; 2] = [
    VisualModificationRule::abilityoutline(),
    VisualModificationRule::leadingscoreoutline(),
];

/// HUMAN: Named visual modifications emitted by VMS rule evaluation.
/// AI: Keep these semantic; concrete colors and surfaces live in Treatments and Targets.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum VisualModifier {
    AbilityOutline,
    LeadingScoreOutline,
}

/// HUMAN: Predicate name for deciding when a visual modification should be active.
/// AI: Conditions read model state only; do not mutate render entities here.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum VisualModificationCondition {
    CardPowerModifiedByAbility,
    LocationTotalIsLeading,
}

impl VisualModificationCondition {
    pub const fn card_power_modified_by_ability(active_ability_delta: i32) -> bool {
        active_ability_delta != 0
    }

    pub const fn location_total_is_leading(value: i32, paired_value: i32) -> bool {
        value > paired_value
    }
}

/// HUMAN: Render element selector for applying a visual modification.
/// AI: Targets identify the smallest intended visual child, not the whole card or location.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum VisualModificationTarget {
    CardPowerPointCircle,
    LocationTotalPointCircle,
}

/// HUMAN: Named treatment color for VMS presentation.
/// AI: Keep colors centralized so future effects avoid hard-coded material values.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum VisualModificationColor {
    Gold,
    White,
}

impl VisualModificationColor {
    pub fn color(self) -> Color {
        match self {
            Self::Gold => Color::srgb(1.0, 0.74, 0.18),
            Self::White => Color::WHITE,
        }
    }
}

/// HUMAN: Presentation operation applied to a resolved VMS target.
/// AI: Treatments never recalculate point values or gameplay state.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VisualModificationTreatment {
    Outline {
        color: VisualModificationColor,
        width: f32,
    },
}

impl VisualModificationTreatment {
    pub const fn gold_outline() -> Self {
        Self::Outline {
            color: VisualModificationColor::Gold,
            width: VISUAL_MODIFIER_OUTLINE_WIDTH_PX,
        }
    }

    pub const fn white_outline() -> Self {
        Self::Outline {
            color: VisualModificationColor::White,
            width: VISUAL_MODIFIER_OUTLINE_WIDTH_PX,
        }
    }
}

/// HUMAN: Declarative Condition/Target/Treatment rule for one visual modification.
/// AI: Add new effects by extending this rule list instead of scattering color changes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VisualModificationRule {
    pub modifier: VisualModifier,
    pub condition: VisualModificationCondition,
    pub target: VisualModificationTarget,
    pub treatment: VisualModificationTreatment,
}

impl VisualModificationRule {
    pub const fn abilityoutline() -> Self {
        Self {
            modifier: VisualModifier::AbilityOutline,
            condition: VisualModificationCondition::CardPowerModifiedByAbility,
            target: VisualModificationTarget::CardPowerPointCircle,
            treatment: VisualModificationTreatment::gold_outline(),
        }
    }

    pub const fn leadingscoreoutline() -> Self {
        Self {
            modifier: VisualModifier::LeadingScoreOutline,
            condition: VisualModificationCondition::LocationTotalIsLeading,
            target: VisualModificationTarget::LocationTotalPointCircle,
            treatment: VisualModificationTreatment::white_outline(),
        }
    }

    pub const fn initial_rules() -> &'static [Self] {
        &INITIAL_VISUAL_MODIFICATION_RULES
    }
}

/// HUMAN: Active VMS modifier state for one PointView root.
/// AI: This stores rule outputs; renderer sync decides the concrete Bevy components.
#[derive(Component, Clone, Debug, Default, Eq, PartialEq)]
pub struct PointViewVisualModifiers {
    active: Vec<VisualModifier>,
}

impl PointViewVisualModifiers {
    pub fn is_active(&self, modifier: VisualModifier) -> bool {
        self.active.contains(&modifier)
    }

    pub fn set_active(&mut self, modifier: VisualModifier, is_active: bool) {
        if is_active {
            if !self.active.contains(&modifier) {
                self.active.push(modifier);
            }
        } else {
            self.active.retain(|active| *active != modifier);
        }
    }

    pub fn clear(&mut self) {
        self.active.clear();
    }
}

/// HUMAN: Marks the circle/background child that VMS treatments are allowed to alter.
/// AI: Use this instead of display-name matching when resolving Targets.
#[derive(Component, Clone, Copy, Debug, Eq, PartialEq)]
pub struct PointViewCircle {
    pub target: VisualModificationTarget,
}

impl PointViewCircle {
    pub const fn new(target: VisualModificationTarget) -> Self {
        Self { target }
    }
}

/// HUMAN: Optional direct link from a card-owned point view to the 015 card instance model.
/// AI: Current systems may use adapter fallback until rendered views carry stable instance IDs.
#[derive(Component, Clone, Copy, Debug, Eq, PartialEq)]
pub struct PointViewCardInstanceLink {
    pub instance_id: CardInstanceId,
}

impl PointViewCardInstanceLink {
    pub const fn new(instance_id: CardInstanceId) -> Self {
        Self { instance_id }
    }
}

/// HUMAN: World-space treatment child that can be toggled for point-view outlines.
/// AI: UI point views use BorderColor instead; mesh point views use this helper child.
#[derive(Component, Clone, Copy, Debug, Eq, PartialEq)]
pub struct PointViewOutlineTreatment {
    pub modifier: VisualModifier,
}

impl PointViewOutlineTreatment {
    pub const fn new(modifier: VisualModifier) -> Self {
        Self { modifier }
    }
}

#[cfg(test)]
#[path = "../../tests/runtime/components/point_view_visual_modifier_component_tests.rs"]
mod point_view_visual_modifier_component_tests;
