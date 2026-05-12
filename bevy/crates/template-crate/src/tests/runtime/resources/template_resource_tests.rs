use super::*;

#[test]
fn template_resource_when_default_is_called_sets_expected_values() {
    let result = TemplateResource::default();

    assert_eq!(result.move_speed, 5.0);
    assert_eq!(result.rotation_speed, 180.0);
}

#[test]
fn movement_delta_when_called_scales_velocity_by_speed_and_time() {
    let resource = TemplateResource::default();

    let result = resource.movement_delta(Vec3::X, 2.0);

    assert_eq!(result, Vec3::new(10.0, 0.0, 0.0));
}
