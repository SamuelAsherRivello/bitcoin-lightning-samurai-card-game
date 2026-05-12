use super::*;

#[test]
fn template_component_when_new_is_called_sets_expected_values() {
    let result = TemplateComponent::new("Player", Vec3::ZERO);

    assert_eq!(result.name, "Player");
    assert_eq!(result.velocity, Vec3::ZERO);
}
