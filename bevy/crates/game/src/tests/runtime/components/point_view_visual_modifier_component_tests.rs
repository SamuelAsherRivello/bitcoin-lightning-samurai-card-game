use super::*;

#[test]
fn initial_rules_define_condition_target_and_treatment() {
    let rules = VisualModificationRule::initial_rules();

    assert_eq!(rules.len(), 2);
    assert!(rules.iter().any(|rule| {
        rule.modifier == VisualModifier::AbilityOutline
            && rule.condition == VisualModificationCondition::CardPowerModifiedByAbility
            && rule.target == VisualModificationTarget::CardPowerPointCircle
            && rule.treatment == VisualModificationTreatment::gold_outline()
    }));
    assert!(rules.iter().any(|rule| {
        rule.modifier == VisualModifier::LeadingScoreOutline
            && rule.condition == VisualModificationCondition::LocationTotalIsLeading
            && rule.target == VisualModificationTarget::LocationTotalPointCircle
            && rule.treatment == VisualModificationTreatment::white_outline()
    }));
}

#[test]
fn card_power_modified_condition_tracks_nonzero_ability_delta() {
    assert!(VisualModificationCondition::card_power_modified_by_ability(
        2
    ));
    assert!(VisualModificationCondition::card_power_modified_by_ability(
        -2
    ));
    assert!(!VisualModificationCondition::card_power_modified_by_ability(0));
}

#[test]
fn location_total_leading_condition_is_strict() {
    assert!(VisualModificationCondition::location_total_is_leading(5, 3));
    assert!(!VisualModificationCondition::location_total_is_leading(
        3, 5
    ));
    assert!(!VisualModificationCondition::location_total_is_leading(
        4, 4
    ));
    assert!(!VisualModificationCondition::location_total_is_leading(
        0, 0
    ));
}

#[test]
fn active_modifier_state_is_idempotent_and_clearable() {
    let mut modifiers = PointViewVisualModifiers::default();

    modifiers.set_active(VisualModifier::AbilityOutline, true);
    modifiers.set_active(VisualModifier::AbilityOutline, true);

    assert!(modifiers.is_active(VisualModifier::AbilityOutline));

    modifiers.set_active(VisualModifier::AbilityOutline, false);
    assert!(!modifiers.is_active(VisualModifier::AbilityOutline));

    modifiers.set_active(VisualModifier::LeadingScoreOutline, true);
    modifiers.clear();
    assert!(!modifiers.is_active(VisualModifier::LeadingScoreOutline));
}
