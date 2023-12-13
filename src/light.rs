use glam::DVec3;

use crate::Color;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PointLight {
    position: DVec3,
    intensity: Color,
}

impl PointLight {
    pub fn new(position: DVec3, intensity: Color) -> Self {
        Self {
            position,
            intensity
        }
    }

    pub fn position(&self) -> DVec3 {
        self.position
    }

    pub fn intensity(&self) -> Color {
        self.intensity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_point_light_as_a_position_and_intensity() {
        let l = PointLight::new(DVec3::ZERO, Color::white());
        assert_eq!(l.position, DVec3::ZERO);
        assert_eq!(l.intensity, Color::white());
    }
}
