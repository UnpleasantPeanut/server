/// Assuming tha the origin is always 0, 0
pub fn parametric_equation_circle(radius: f64, angle: f64) -> (f64, f64) {
    (radius * angle.cos(), radius * angle.sin())
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::parametric_equation_circle;

    #[test]
    fn should_calculate_circle_coordinates_with_angle() {
        let (x, y) = parametric_equation_circle(1.0, PI);
        assert_eq!(x, -1.0);
        assert_eq!(y, 1.2246467991473532e-16);
    }
}