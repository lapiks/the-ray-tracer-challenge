use glam::Vec3;

use crate::Color;

#[derive(Debug, Clone, Copy, PartialEq)]
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

    pub fn position(&self) -> &Vec3 {
        &self.position
    }

    pub fn intensity(&self) -> &Color {
        &self.intensity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_point_light_as_a_position_and_intensity() {
        let l = PointLight::new(&Vec3::ZERO, &Color::white());
        assert_eq!(l.position, Vec3::ZERO);
        assert_eq!(l.intensity, Color::white());
    }
}
