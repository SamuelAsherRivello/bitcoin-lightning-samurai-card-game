use super::*;

#[test]
fn template_bundle_when_new_is_called_sets_expected_values() {
    let result = TemplateBundle::new("Player", Vec3::X);

    assert_eq!(result.template_component.name, "Player");
    assert_eq!(result.template_component.velocity, Vec3::X);
}
