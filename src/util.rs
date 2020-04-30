//! Various utility functions.

/// Convert from radians to degrees.
///
/// # Usage
///
/// ```
/// # use weekend_tracer_rs::util::rad_to_deg;
/// assert_eq!(rad_to_deg(2.0 * std::f32::consts::PI / 3.0), 120.0);
/// ```
pub fn rad_to_deg(x: f32) -> f32 {
    x * 180.0 / std::f32::consts::PI
}

/// Convert from degrees to radians.
///
/// # Usage
///
/// ```
/// # use weekend_tracer_rs::util::deg_to_rad;
/// assert_eq!(deg_to_rad(270.0), 3.0 * std::f32::consts::PI / 2.0);
/// ```
pub fn deg_to_rad(x: f32) -> f32 {
    x * std::f32::consts::PI / 180.0
}

/// Clamps a number to a value.
///
/// # Usage
///
/// ```
/// # use weekend_tracer_rs::util::clamp;
/// assert_eq!(clamp(-3.0, -2.0, 3.0), -2.0);
/// assert_eq!(clamp(1.0, -2.0, 3.0), 1.0);
/// assert_eq!(clamp(5.0, -2.0, 3.0), 3.0);
pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}
