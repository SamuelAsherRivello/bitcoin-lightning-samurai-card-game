use super::*;

#[test]
fn template_shader_settings_when_default_is_called_sets_expected_values() {
    let result = TemplateShaderSettings::default();

    assert_eq!(result.shader_path, TEMPLATE_SHADER_PATH);
    assert_eq!(result.tint, Color::WHITE);
}
