use super::*;

#[test]
fn calculate_forward_vector_when_rotation_is_zero_returns_positive_x() {
    let result = calculate_forward_vector(0.0);

    assert_eq!(result, Vec3::new(1.0, 0.0, 0.0));
}

#[test]
fn calculate_forward_vector_when_rotation_is_pi_returns_negative_x() {
    let result = calculate_forward_vector(std::f32::consts::PI);

    assert_eq!(result.round(), Vec3::new(-1.0, 0.0, 0.0));
}
