use glam::DVec3;

use crate::{Color, World};

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

    pub fn intensity_at(&self, world_point: DVec3, world: &World) -> f64 {
        match world.is_shadowed(world_point, self.position) {
            true => 0.0,
            false => 1.0,
        } 
    }
}

#[cfg(test)]
mod tests {
    use glam::dvec3;

    use crate::world::tests::default_world;

    use super::*;

    #[test]
    fn a_point_light_as_a_position_and_intensity() {
        let l = PointLight::new(DVec3::ZERO, Color::white());
        assert_eq!(l.position, DVec3::ZERO);
        assert_eq!(l.intensity, Color::white());
    }

    #[test]
    fn point_lights_evaluate_the_light_intensity_at_a_given_point() {
        let w = default_world();
        let light = w.light(0).unwrap();
        let datas = vec![
            (dvec3(0.0, 1.0001, 0.0), 1.0),
            (dvec3(-1.0001, 0.0, 0.0), 1.0),
            (dvec3(0.0, 0.0, -1.0001), 1.0),
            (dvec3(0.0, 0.0, 1.0001), 0.0),
            (dvec3(1.0001, 0.0, 0.0), 0.0),
            (dvec3(0.0, -1.0001, 0.0), 0.0),
            (dvec3(0.0, 0.0, 0.0), 0.0),
        ];
        for data in datas {
            assert_eq!(light.intensity_at(data.0, &w), data.1);
        }
    }   
}
