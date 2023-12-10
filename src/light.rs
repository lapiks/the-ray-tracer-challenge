use glam::Vec3;

use crate::Color;

pub struct PointLight {
    position: Vec3,
    intensity: Color,
}

impl PointLight {
    pub fn new(position: &Vec3, intensity: &Color) -> Self {
        Self {
            position: *position,
            intensity: *intensity
        }
    }
}

#[cfg(test)]
mod tests {
    use glam::vec3;
    use super::*;

    #[test]
    fn a_point_light_as_a_position_and_intensity() {
        let l = PointLight::new(&vec3(0.0, 0.0, 0.0), &Color::new(1.0, 1.0, 1.0));
        assert_eq!(l.position, vec3(0.0, 0.0, 0.0));
        assert_eq!(l.intensity, Color::new(1.0, 1.0, 1.0));
    }
}
