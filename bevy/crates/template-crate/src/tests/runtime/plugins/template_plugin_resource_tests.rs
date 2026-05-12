use super::*;

#[test]
fn template_plugin_resource_when_default_is_called_sets_expected_values() {
    let result = TemplatePluginResource::default();

    assert_eq!(result.move_speed, 5.0);
}
