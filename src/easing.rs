use std::f32::consts::PI;

#[allow(dead_code)]
pub fn cubic_in_out(x: f32) -> f32 {
    if x < 0.5 {
        4.0 * x * x * x
    } else {
        1.0 - ((-2.0 * x + 2.0).powi(3)) / 2.0
    }
}

#[allow(dead_code)]
pub fn sine_in_out(x: f32) -> f32 {
    -((PI * x).cos() - 1.0) / 2.0
}
